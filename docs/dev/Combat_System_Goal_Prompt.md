# `/goal` 전투 시스템 재개 프롬프트

아래 블록을 새 세션에서 `/goal` 뒤에 붙여 넣는다. 이 프롬프트는 context 압축 후 오케스트레이터가 같은 운영 기준을 복원하도록 작성했다.

```text
목표:
Notion 전투 시스템 허브와 canonical 00~13을 기준으로, Rust GameCore의 결정론 전투 계약을 단계별로 구현한다. 한 goal은 한두 개의 plan slice만 다루고, 각 단계는 main 직접 검증 후 다음 단계로 넘긴다.

역할:
- 너는 오케스트레이터다. 조사·범위·plan·충돌·검증·보고를 담당한다.
- `/root/coding_implementer`는 tui_adv_coder 역할의 5.6 luna / medium coding agent다. 승인된 한 slice의 구현만 위임한다.
- subagent의 검증 보고를 그대로 믿지 말고 WSL에서 다시 실행한다.

반드시 먼저 읽을 파일:
1. `/home/dudu/work/tui-adv/AGENTS.md`
2. `/home/dudu/work/tui-adv/docs/dev/Combat_System_Operating_Guide.md`
3. `/home/dudu/work/tui-adv/docs/dev/Development_Plan.md` 상단
4. `/home/dudu/work/tui-adv/docs/design/Combat_System_Implementation_Plan_Index.md`
5. 현재 ACTIVE_PLAN으로 기록된 step plan 한 파일

작업 환경:
- 모든 git/cargo/npm/pytest/gh 명령은 WSL에서 실행한다.
- 저장소: `/home/dudu/work/tui-adv`
- `git status --short --branch -uall`로 시작하고, 미추적 `.claude/worktrees/`는 건드리지 않는다.
- Windows worktree는 장기 빌드/개발 기준으로 사용하지 않는다.

현재 baseline:
- Wave 1 Step 1은 구현·검증 완료다.
- Wave 1 Step 2는 구현·검증 완료다. 전투원 상태·effect catalog·deterministic pre-combat projection 계약이 Rust GameCore에 추가되었다.
- Wave 1 Step 3은 구현·검증 완료다. opportunity → detection → response 후보와 대표 관찰자/실행자, 0~3 개입 예산 계약이 Rust GameCore에 추가되었다.
- Wave 2 Step 1은 구현·검증 완료다. 고정 정수 좌표, role weights·target fallback, active 4/8 상한, snapshot 동시 tick, canonical simulation fingerprint가 Rust GameCore에 추가되었다.
- `crates/escape-core/src/combat_contract.rs`에 CombatSimulationVersion, 5개 CombatRngNamespace, CombatManifest, effect reason/suppressed effect, canonical JSON/fingerprint/derived seed, validation이 있다.
- `crates/escape-core/tests/combat_contract_wave1.rs` 5개 테스트가 있다.
- Wave 2 Step 2는 구현·검증 완료다. actual/retry/auto/fast parity, forecast namespace 분리, presentation parity, dual log와 실행 fingerprint가 추가되었다.
- `crates/escape-core/src/combat_execution.rs`와 `crates/escape-core/tests/combat_execution_wave2.rs` 10개 테스트가 있다(Wave 2 Step 2 당시 6개, Wave 2 Step 6에서 provenance 회귀 테스트 4개가 늘었다).
- `fable_combat_wave2_step3_2607261845.md`가 구현·검증 완료다. sidecar resolution request가 기존 execution frame을 재사용하고 collision/range/accuracy, fixed-point damage/defense, health/balance clamp, effect catalog stacking, resolution full/core log를 제공한다.
- `crates/escape-core/src/combat_resolution.rs`와 `crates/escape-core/tests/combat_resolution_wave2.rs` 11개 테스트가 있다.
- `fable_combat_wave2_step4_2607261845.md`가 구현·검증 완료다. resolution fingerprint/state를 소비하는 pure 결착 evaluator가 mutual-defeat precedence, max-tick stalemate, stable survivor/defeated report, combat-only cleanup sidecar를 제공한다.
- `crates/escape-core/src/combat_conclusion.rs`와 `crates/escape-core/tests/combat_conclusion_wave2.rs` 결착 회귀 테스트 14개가 있다(Wave 3 Step 1b 포함).
- `fable_combat_wave3_step1a_2608020020.md`가 구현·검증 완료다. `CombatResolutionResult`를 입력으로 받는 `escape-core` 전용 관전 view 어댑터가 새 판정 없이 tick별 체스말 프레임, 공용 연출 cue(Attack/Hit/Evade), 템플릿 id 기반 이중 로그, 누설 차단(AttackRoll/EffectSuppressed 제외, Hidden/Conditional/미등록 효과 id 마스킹)을 제공한다. `crates/escape-core/src/combat_spectator.rs`와 `crates/escape-core/tests/combat_spectator_wave3.rs` 12개 테스트가 있다. `BalanceBroken`/`Incapacitated` cue는 tick별 상태 스냅샷이 아직 없어 의도적으로 제외했다(후속 slice 선행 필요).
- `fable_combat_wave2_step5_2608020117.md`가 구현·검증 완료다. `CombatResolutionFrame.combatants`(tick 종료 시점 전투원 스냅샷, additive-optional)를 추가하고 `combat_spectator.rs`가 이를 소비해 `BalanceBroken`·`Incapacitated` cue 2개를 파생한다. 정본 13의 공용 연출 문법 5개가 모두 확보됐다. `crates/escape-core/tests/combat_resolution_wave2.rs` 16개 테스트, `crates/escape-core/tests/combat_spectator_wave3.rs` 19개 테스트가 있다.
- `fable_combat_wave3_step1b_2608020437.md`가 구현·검증 완료다. `CombatConclusionRequest.tick_millis`(0은 `InvalidTickMillis`)와 `CombatConclusionReport.duration_millis`·`combatants: Vec<CombatCombatantReport>`(피해 입힘/받음·처치 수·전투불능, id 오름차순)·`top_damage_dealt_id`/`top_damage_taken_id`(발생하지 않으면 `None`, 동점은 id 최소)를 additive-optional로 추가했다. 판정은 `resolution.frames[].outcomes`/`combatants` 스냅샷만 집계하고 재계산하지 않는다. 치유량·명줄은 아직 없다(healing/패배 결과 스키마 slice 선행 필요). 정본 13이 금지하는 전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교는 의도적으로 구현하지 않았다.
- `fable_combat_wave2_step6_2608020838.md`가 구현·검증 완료다. `CombatExecutionResult.provenance: Option<CombatProvenance>`(`simulation_version`·`tick_millis`·`manifest_fingerprint`, additive-optional, `execute()`는 항상 `Some`)를 추가하고 `CombatSpectatorRequest.tick_millis`/`CombatConclusionRequest.tick_millis` 중복 파라미터를 제거했다. `spectate()`/`conclude()`는 이제 `resolution.execution.provenance`에서 `tick_millis`를 읽고, 없거나 0이면 두 에러 enum의 `InvalidTickMillis`를 대체한 `MissingProvenance`를 낸다. 정본 03 근거로 fingerprint 안정성 선결 과제를 해소했다 — 결정성은 같은 `simulation_version` 안에서만 보장하므로 schema 추가로 fingerprint 값이 바뀌는 것은 계약 위반이 아니며, fingerprint를 비교하는 consumer는 반드시 `simulation_version`도 함께 비교해야 한다.
- `fable_combat_wave3_step1c_2608021109.md`가 구현·검증 완료다. `CombatSpectatorView.simulation_version`(provenance에서 파생)과 `CombatSpectatorPage`(`view` + optional `report`)를 추가하고 `ScenePage.combat: Option<CombatSpectatorPage>`로 renderer 경계 밖에 additive-optional 노출했다. 전투를 여는 인카운터 authoring이 없어(Wave 3 Step 2) `scene_page_from_turn_view`는 `combat: None`만 낸다 — 이 slice는 producer가 아니라 구조 슬롯만 추가했다. `#[serde(default, skip_serializing_if = "Option::is_none")]`로 기존 `ScenePage` JSON이 바이트 단위로 동일함을 신규 테스트로 고정했다. `crates/escape-core/tests/scene_page_combat_boundary.rs`(신규 5개 테스트)와 `crates/escape-wasm/tests/json_contract.rs`(기존 테스트 무수정, 1개 추가로 37개)가 있다.
- 고급 다수전 AI 행동·조기 tick 중단, 치유량·명줄, 전투를 여는 인카운터 authoring(`ScenePage.combat`의 producer), Web/terminal 전투 UI, 기술 비용·호흡 회복률·밸런스 수치는 아직 미구현이다.

Notion 불변식:
- 같은 manifest·seed·선택 이력·simulation version은 같은 결과와 로그를 만든다.
- story/encounter/actual/forecast/cosmetic RNG는 분리하며 forecast가 actual RNG를 재사용하지 않는다.
- 전투는 체스말 형태 캐릭터의 결정론적 실시간 시뮬레이션이고 renderer는 판정을 재계산하지 않는다.
- 전투 중 감독형 개입은 인카운터 중요도/유형이 정한 0~3회 상한을 공유한다.
- 생명력·균형·호흡·공포·분노·자세·무기 제어·거리·압박·시야·대형·결속은 다른 상태 층이다.
- 상태이상 (전투)은 결착 시 제거하고, 상태이상 (지속)은 사건/패배 결과가 명시할 때만 부여한다.
- 정확한 기술 비용·회복률·피해·방어·쿨타임 값은 임의로 고정하지 않는다.

진행 규칙:
1. 현재 main plan과 active step plan의 drift를 확인한다.
2. 계획 파일이 없거나 범위가 크면 먼저 작은 plan 파일을 만든다. 한 plan은 한 coding agent 작업량으로 제한한다.
3. plan에 소유 파일, 금지 범위, acceptance criteria, 검증 명령을 적는다.
4. coding agent에게 자기완결형 지시를 보낸다.
5. agent가 끝나면 diff와 파일을 직접 읽고 WSL에서 fmt/test/build/export를 필요한 만큼 다시 실행한다.
6. 검증이 통과하면 plan 상태와 Development_Plan 링크를 갱신하고, 다음 slice의 미결정 사항만 보고한다.
7. commit/push/PR은 사용자가 별도로 명령할 때만 한다.

이번 goal의 종료 조건:
- 현재 plan의 acceptance criteria를 모두 충족한다.
- main에서 targeted test와 `cargo test --workspace --no-fail-fast`를 실행해 통과한다.
- `git diff --check`가 통과한다.
- plan/report에 구현 파일, 검증 결과, non-goal, 다음 단계가 기록된다.
- 임의의 밸런스 수치나 renderer-local gameplay 로직을 추가하지 않는다.

보고 형식:
- Plan: 어떤 문서를 실행했는가
- Implement: coding agent가 바꾼 파일과 핵심 API
- Verify: main에서 실행한 정확한 명령과 결과
- Scope: 완료/보류/non-goal
- Next: 다음 plan 파일 또는 사용자 결정이 필요한 질문
```

권장 다음 goal 문장:

```text
Wave 3 Step 2(시스템형 1개 + 혼합형 1개 + 각본형 1개 authoring slice)를 별도 작은 plan으로 설계하고, 승인 후 WSL 회귀 검증까지 수행한다. `ScenePage.combat`은 Wave 3 Step 1c에서 이미 additive-optional로 노출됐지만 이를 채우는 producer(전투를 여는 인카운터 authoring)가 없어 여전히 항상 `None`이다. Step 1d(terminal/Web 렌더러)보다 Step 2를 먼저 진행한다 — authoring이 없으면 렌더러가 표시할 데이터가 없기 때문이다 (단계 순서 조정 근거는 `docs/design/Combat_System_Implementation_Plan_Index.md` 참고).
```
