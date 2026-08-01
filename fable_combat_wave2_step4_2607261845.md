---
status: implementation-verified
phase: combat-wave2
step: 4
owner: coding_implementer
date: 2026-07-26
---

# Wave 2 Step 4 — 다수전 결착·전투 종료 조건 sidecar

## 목적

Wave 2 Step 1~3의 결정론적 target/move frame과 실제 resolution state 위에, 다수전의 현재 생존자 집합을 판정하고 전투가 결착되었는지 보고하는 renderer-neutral GameCore sidecar를 추가한다.

이번 단계는 기존 simulation/resolution의 판정 순서를 바꾸지 않는다. `CombatResolutionResult`를 입력으로 받아 승패·상호 전멸·시간 초과·진행 중 상태를 데이터로 보고하며, renderer가 생존자 수나 종료 조건을 다시 계산하지 않게 하는 것이 목표다.

```text
CombatResolutionResult
  -> active participant side validation
  -> health terminal-set evaluation
  -> deterministic conclusion precedence
  -> combat-only effect cleanup report
  -> conclusion fingerprint
```

## 기준 정본

- 허브: <https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449>
- 01. 전투 루프와 개입 예산: 같은 seed/version/input은 같은 결과·로그를 만들고 mode는 판정 속도만 바꾼다.
- 03. 핵심 상태 시스템: 생명력은 전투 결착의 terminal truth이고 combat-only effect는 결착 시 제거한다. persistent status로 자동 승격하지 않는다.
- 09. 다수전 전투 시스템: 아군 최대 4명/적 최대 8명, side와 active participant를 명시한다.
- 13. 감독형 관전·전략 피드백: renderer는 core의 결과·로그를 표시만 하며 원인/승률을 재판정하지 않는다.

## 소유 파일

coding agent는 아래 파일만 추가·수정한다.

- `crates/escape-core/src/combat_conclusion.rs` (신규 public sidecar contract와 pure evaluator)
- `crates/escape-core/src/lib.rs` (module/public export만)
- `crates/escape-core/tests/combat_conclusion_wave2.rs` (신규 회귀 테스트)

기존 `combat_contract.rs`, `combat_state.rs`, `combat_simulation.rs`, `combat_execution.rs`, `combat_resolution.rs`의 public 필드와 동작은 깨지지 않게 사용한다. 계획 문서·Development_Plan·기존 테스트 수정은 main orchestrator가 검증 후 처리한다. `.claude/worktrees/`는 절대 건드리지 않는다.

## 구현 계약

### 1. 결착 입력

`CombatConclusionRequest`는 최소 다음을 가진다.

- `resolution: CombatResolutionResult`
- `participants: Vec<CombatSimulationParticipant>` — side/active/id의 authoritative input
- `policy: CombatTerminationPolicy`

`CombatTerminationPolicy`는 다음을 가진다.

- `max_ticks: u32` — 호출자가 데이터로 전달하는 시간 제한. 코드에 전투별 숫자를 추가하지 않는다.
- `conclude_on_max_ticks: bool` — 제한에 도달했을 때 `Stalemate`로 닫을지 여부

입력 검증은 duplicate participant ID, resolution state와 participant ID 불일치, 빈 active ally/enemy side, `max_ticks == 0`, resolution frame tick이 정책을 초과하는 경우를 조용히 허용하지 않고 명시적 error로 반환한다. inactive participant는 결착 집합에서 제외하되, active participant와 동일 ID를 중복 선언할 수 없다.

### 2. 종료 outcome과 precedence

다음 enum과 고정 precedence를 사용한다.

```rust
CombatConclusionOutcome::{InProgress, AllyVictory, EnemyVictory, MutualDefeat, Stalemate}
CombatConclusionReason::{NoTerminalCondition, AllEnemiesDefeated, AllAlliesDefeated,
                         BothSidesDefeated, MaxTicksReached}
```

평가는 항상 다음 순서를 따른다.

1. 양 side의 active participant가 모두 health `0`이면 `MutualDefeat / BothSidesDefeated`.
2. 적 side만 모두 health `0`이면 `AllyVictory / AllEnemiesDefeated`.
3. 아군 side만 모두 health `0`이면 `EnemyVictory / AllAlliesDefeated`.
4. 위 terminal 조건이 없고 마지막 resolution tick이 `max_ticks`에 도달했으며 `conclude_on_max_ticks`가 true이면 `Stalemate / MaxTicksReached`.
5. 그 외에는 `InProgress / NoTerminalCondition`.

Health가 0보다 큰 생존자와 0인 defeated ID는 모두 stable ID 오름차순으로 보고한다. side가 모두 terminal인 경우에도 1번이 승패보다 우선한다. retreat, surrender, morale break, reinforcement, background battle은 이번 단계에서 추정하지 않는다.

### 3. Report와 cleanup sidecar

`CombatConclusionReport`는 최소 다음을 가진다.

- 원본 `resolution_fingerprint`
- `outcome`, `reason`, `decisive_tick: Option<u32>` (terminal이면 마지막 resolution frame tick)
- `active_allies`, `active_enemies`, `survivor_ids`, `defeated_ids`
- `removed_combat_effect_ids` — resolution state의 `combat_only == true` effect definition ID
- `retained_effect_ids` — combat-only가 아닌 effect definition ID. persistent status로 자동 승격하지 않는다.
- `fingerprint`

effect ID 목록과 participant ID 목록은 모두 stable 정렬·중복 제거한다. report는 입력 state를 mutate하지 않고, 결착 시 적용할 cleanup의 명시적 sidecar만 반환한다. `CombatState::conclude`와 의미가 충돌하지 않도록 persistent status/resource 승격 API를 추가하지 않는다.

### 4. Determinism와 다수전 semantics

- participant/attack input 순서를 바꾸어도 outcome, reason, ID 목록, cleanup, fingerprint가 동일해야 한다.
- Actual/Retry/Auto/Fast와 OneX/TwoX/Instant 결과는 기존 resolution과 같은 report를 만든다.
- Forecast report는 forecast resolution fingerprint를 보존하지만 actual seed/roll을 다시 계산하지 않는다.
- active 4/8 제한과 side 정보는 기존 `CombatSimulation::new` 계약을 재사용한다. 이번 slice에서 새 role weight, damage, morale, retreat threshold를 만들지 않는다.

## Acceptance criteria

1. 단일 적 전멸, 단일 아군 전멸, 양측 전멸, 진행 중, max tick stalemate의 다섯 outcome/reason이 각각 고정된다.
2. 양측 전멸이 victory보다 우선하고, ID/입력 순서를 바꿔도 report/fingerprint가 동일하다.
3. inactive participant, duplicate/missing ID, 빈 side, `max_ticks == 0`, frame tick 초과가 deterministic error가 된다.
4. survivor/defeated 목록과 `decisive_tick`이 stable하게 보고된다.
5. combat-only effect는 제거 목록으로, non-combat effect는 retained 목록으로 분리되며 persistent status 승격이 없다.
6. Actual/Retry/Auto/Fast, presentation speed, Forecast 결과가 기존 resolution fingerprint를 보존하면서 동일 conclusion contract를 사용한다.
7. renderer/UI/WASM/콘텐츠 authoring 변경 없이 pure sidecar로 동작한다.
8. `cargo fmt --all -- --check`, targeted combat suites, `cargo test --workspace --no-fail-fast`, `git diff --check`가 main에서 통과한다.

## Non-goal

- 새로운 전투 행동/스킬/쿨다운/호흡 비용/밸런스 값
- 고급 AI utility, morale, 패주, 항복, 증원, 배경 전투
- 실제 tick을 조기 중단하는 통합 simulation loop 또는 renderer adapter
- ScenePage/WASM/Web Storybook/SuperLightTUI 전투 UI
- 승률 ensemble, 전략 조언, 전투 종료 narrative/result authoring
- CombatState public field의 breaking change 또는 persistent status 자동 승격

## 검증 명령

```bash
cd /home/dudu/work/tui-adv
cargo fmt --all -- --check
cargo test -p escape-core --test combat_conclusion_wave2 --test combat_resolution_wave2 --test combat_execution_wave2 --test combat_simulation_wave2 --test combat_state_wave1 --test combat_contract_wave1 --test combat_opportunity_wave1
cargo test --workspace --no-fail-fast
git diff --check
```

## 구현 보고 형식

- 변경 파일과 public API
- outcome precedence와 active side validation
- survivor/defeated/decisive tick 계산
- combat-only cleanup sidecar와 persistent non-goal
- targeted/workspace/fmt/diff 검증 결과
- non-goal과 다음 승인 후보(고급 AI/renderer adapter 중 하나)

## 구현 보고

- 구현 파일: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/src/lib.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`
- public API: `CombatConclusionRequest`, `CombatTerminationPolicy`, `CombatConclusionOutcome`, `CombatConclusionReason`, `CombatConclusionReport`, `conclude_combat`
- 결착: active ally/enemy side와 resolution state ID를 검증하고, 상호 전멸 → 단일 side 전멸 → max tick stalemate → 진행 중 순서로 고정한다.
- 보고: stable survivor/defeated ID, terminal decisive tick, resolution fingerprint, combat-only removed/non-combat retained effect ID를 pure report로 반환하며 persistent status 승격은 하지 않는다.
- main WSL 검증: `cargo fmt --all -- --check`, targeted 7개 combat suite, `cargo test --workspace --no-fail-fast`, `git diff --check` 통과.
- non-goal: 고급 AI utility, 조기 tick 중단, 패주/항복/증원/배경 전투, renderer adapter, 밸런스·skill schema.
- 다음 승인 후보: `fable_combat_wave3_step1_2607261845.md`로 ScenePage/WASM/terminal/Web 관전 adapter를 별도 설계하거나, 고급 AI 행동을 먼저 별도 plan으로 분리한다.
