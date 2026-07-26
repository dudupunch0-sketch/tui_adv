---
status: implementation-verified
phase: combat-wave2
step: 3
owner: coding_implementer
date: 2026-07-26
---

# Wave 2 Step 3 — 실제 공격·충돌·피해·상태 effect resolver

## 목적

Wave 2 Step 1~2의 고정 tick·연속 좌표·역할/목표·실행 mode parity·이중 로그 위에, 실제 공격 접촉/사거리, 결정론 명중, 고정소수점 피해·균형 피해, 등록된 상태 effect 적용을 하는 renderer-neutral GameCore 경계를 추가한다.

이번 단계의 결과는 Web/terminal renderer가 재판정하지 않고 소비할 수 있는 CombatResolutionResult다.

@@text
CombatExecutionResult(frames)
  -> target/position snapshot
  -> collision + attack-range gate
  -> seeded accuracy roll (actual/forecast namespace)
  -> fixed-point damage/defense resolution
  -> registered effect catalog stacking/application
  -> deterministic resolution state + full/core resolution log
@@

## 기준 정본

- 허브: <https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449>
- 01. 전투 루프와 개입 예산: 같은 fixed tick/seed/version은 같은 전투 결과와 로그를 만들며 mode/speed는 판정 속도만 바꾼다.
- 03. 핵심 상태 시스템: 생명력은 월드와 공유하는 신체 수치이고 전투 effect는 결착 시 정리한다. manifest/실제 seed와 forecast seed는 분리한다. 수치 계산은 data-driven이며 effect catalog ID만 허용한다.
- 07. UI·템포·리스크: 전투 상단은 시뮬레이션, 하단은 등록 사건 로그다. renderer는 판정을 재계산하지 않는다.
- 09. 다수전 전투 시스템: 활성 전투는 아군 4명/적 8명, 연속 좌표·방향·공격/지원/충돌 범위를 사용한다.
- 11. 능력치·숙련·전투 스킬 시스템: 명중/피해/방어 공식과 최소 설계 정밀도 0.01 고정소수점 계약을 따른다. 정확한 기술 비용·회복률·콘텐츠 수치는 밸런스 TODO로 남긴다.
- 12. 기술 기반 선택지·전투 기회 시스템: 결과는 등록된 effect catalog bundle만 적용하며, seed 밖 난수·미등록 상태 변경은 금지한다.
- 13. 감독형 관전·전략 피드백 시스템: 전체 사건은 stable tick/sequence로 보존하고 core log는 중요도 필터로 파생한다.

## 소유 파일

coding agent는 아래 파일만 추가·수정한다.

- crates/escape-core/src/combat_resolution.rs (신규 resolver와 public data contract)
- crates/escape-core/src/lib.rs (module/public export만)
- crates/escape-core/tests/combat_resolution_wave2.rs (신규 회귀 테스트)

기존 combat_contract.rs, combat_state.rs, combat_simulation.rs, combat_execution.rs의 public 필드/동작은 깨지지 않게 사용한다. plan/index/운영 문서와 기존 테스트 수정은 main orchestrator가 검증 후 처리한다. .claude/worktrees/는 절대 건드리지 않는다.

## 구현 계약

### 1. Sidecar resolution request

기존 CombatExecutionRequest 작동 경로의 struct literal 호환을 보존한다. 새 CombatResolutionRequest가 다음을 가진다.

- 기존 CombatExecutionRequest
- actor별 CombatAttackDefinition 목록
- target별 CombatDefenseProfile 목록
- 초기 CombatState에서 읽은 combatant health/balance와 기존 active effects
- CombatEffectCatalog

실행은 기존 execute_combat을 먼저 호출해 같은 frames/mode/namespace/effective seed를 사용하고, 그 결과를 resolver에 전달한다. 기존 이동 전용 실행 결과의 parity 계약은 바꾸지 않는다.

### 2. Data-driven attack/defense

CombatAttackDefinition은 최소 다음을 가진다.

- stable attack ID, actor ID
- power_hundredths (0.01 고정소수점 입력)
- ability_multiplier_hundredths (100 = 1.00; 능력/숙련 파생값을 호출자가 전달)
- accuracy_percent (0~100)
- attack_range, penetration_hundredths
- 접촉 시 collision_balance_hundredths
- 적중 시 balance_power_hundredths
- CombatAttackEffect 목록(effect ID + effect chance 0~100)

CombatDefenseProfile은 combatant ID와 data-owned defense_hundredths, balance_resistance_hundredths를 가진다. 누락된 공격/방어 profile, 음수 수치, 중복 ID, 존재하지 않는 actor/target/effect는 hard error다. 기본 위력·방어·비용·회복률을 코드 상수로 만들지 않는다.

### 3. Tick resolution 순서

각 execution frame에서 move intent의 target을 사용한다. actor/target이 active가 아니거나 반대 진영이 아니면 공격하지 않는다.

각 공격은 stable attack ID → actor ID 순으로 처리한다.

1. actor/target 위치와 collision radius로 접촉 여부를 계산한다.
2. attack range 안인지 판정한다. 사거리 밖은 hit/damage/effect가 없다.
3. 접촉 결과와 사거리 결과를 모두 결과에 남긴다.
4. 명중 roll은 CombatRngNamespace::ActualCombat 또는 ForecastEnsemble을 통해 stable seed/tick/attack ID/actor/target에서 파생한다. 100%는 항상 적중하되 판정/로그는 남긴다.
5. 적중하면 정본의 data-driven 공식으로 계산한다.
   - pre-defense: power × 5/12 × ability_multiplier
   - effective defense: max(0, defense - penetration)
   - defense reduction: effective_defense / (effective_defense + 20)
   - final damage: pre-defense × (1 - reduction)
   - 내부는 최소 0.01 정밀도의 정수 fixed-point로 계산하고 overflow를 hard error로 반환한다.
6. 접촉/적중에 따라 data-owned balance damage를 적용한다. 생명력/균형은 0 아래로 내려가지 않는다.
7. 적중한 effect는 effect chance roll을 별도 stable namespace stream에서 수행하고, 성공한 ID만 catalog 정책에 따라 active effect로 반영한다.

정본에 아직 없는 시동/회복/쿨타임/호흡 비용/명명 기술 스키마는 이번 단계에서 추정하지 않는다.

### 4. Runtime state와 effect stacking

기존 whole-health CombatState 저장 계약을 깨지 않도록 resolver 결과에는 CombatResolutionState sidecar를 둔다.

- combatant별 current/max health와 balance를 hundredths로 보존한다.
- 초기값은 기존 CombatantState 정수값을 ×100으로 변환한다.
- active effect instance 목록과 이번 tick의 applied/suppressed effect ID를 보존한다.
- Unique, Replace, Strongest, AdditiveWithCap, StackCount, DurationRefresh, Independent 정책을 stable stacking group + target 기준으로 결정한다.
- catalog에 없는 effect나 잘못된 stacking cap은 조용히 무시하지 않는다.
- 전투 결착 시 combat-only effect를 제거하는 기존 CombatState::conclude 계약과 충돌하지 않는 pure result로 남긴다. world persistent status로 자동 승격하지 않는다.

### 5. Resolution result/log

CombatResolutionResult는 최소 다음을 가진다.

- 원본 CombatExecutionResult
- tick별 CombatResolutionFrame
- CombatResolutionState
- stable resolution fingerprint
- 전체 resolution log와 중요도 필터 core log

CombatAttackOutcome은 attack/actor/target, collision, in-range, roll, hit/miss, damage_hundredths, balance delta, applied/suppressed effect IDs를 포함한다.

resolution log는 자유 문장이 아니라 enum tag(예: Collision, AttackRoll, DamageApplied, EffectApplied, EffectSuppressed)와 stable tick/sequence를 사용한다. renderer가 표시할 수 있는 데이터만 제공하고 원인 분석/전략 조언은 생성하지 않는다.

## Acceptance criteria

1. 같은 resolution request를 반복하거나 input 배열 순서를 바꿔도 frame/outcome/state/log/fingerprint가 동일하다.
2. Actual/Retry/Auto/Fast와 OneX/TwoX/Instant는 기존 execution frames와 resolution 결과가 동일하다.
3. Forecast는 ForecastEnsemble namespace를 사용하고 actual seed/roll을 재사용하지 않으며 반복 forecast는 동일하다.
4. collision radius와 attack range 경계(접촉, 사거리 밖, 사거리 안)가 테스트된다.
5. 0%, 100%, 중간 accuracy의 seeded hit/miss가 테스트되며 seed/tick/attack ID가 roll identity에 포함된다.
6. fixed-point damage/defense 공식, penetration, health/balance clamp와 overflow/invalid input이 테스트된다. 임의 balance 상수는 없다.
7. effect catalog unknown ID, chance, stacking 정책(최소 unique/replace/cap/independent)이 deterministic하게 검증된다.
8. result full/core log가 사건 누락 없이 stable sequence이며 core filter가 중요도 규칙으로 파생된다.
9. cargo fmt --all -- --check, targeted combat tests, cargo test --workspace --no-fail-fast, git diff --check가 main에서 통과한다.

## Non-goal

- ScenePage/WASM/Web Storybook/SuperLightTUI 전투 renderer
- 전투 종료 보고서/승률 100회 ensemble/전략 조언/MVP 평가
- 배경 전투·증원·대형/결속·패주·전투 종료 조건
- full named-skill/loadout/cooldown/breath-cost schema
- 기술별 밸런스 표와 실제 콘텐츠 수치 확정
- renderer-local physics 또는 UI 재판정
- existing CombatSimulation movement/target semantics 재작성

## 검증 명령

    cd /home/dudu/work/tui-adv
    cargo fmt --all -- --check
    cargo test -p escape-core --test combat_resolution_wave2 --test combat_execution_wave2 --test combat_simulation_wave2 --test combat_state_wave1 --test combat_contract_wave1 --test combat_opportunity_wave1
    cargo test --workspace --no-fail-fast
    git diff --check

## 구현 보고 형식

- 변경 파일과 public API
- execution frame과 resolution frame 연결 방식
- fixed-point damage/defense/accuracy/effect stacking 처리
- actual vs forecast namespace evidence
- targeted/workspace/fmt/diff 검증 결과
- non-goal과 다음 단계(다수전 AI 행동·결착/renderer adapter)


## 구현 보고

- 구현 파일: crates/escape-core/src/combat_resolution.rs, crates/escape-core/src/lib.rs, crates/escape-core/tests/combat_resolution_wave2.rs
- public API: CombatResolutionRequest/Result/State/Frame, CombatAttackDefinition/Effect, CombatDefenseProfile, resolution log tag/event/error, resolve_combat
- 연결: 기존 execute_combat을 호출해 동일한 frame/mode/namespace/effective seed를 사용하고, resolver 결과를 sidecar fixed-point 상태로 반환한다.
- 판정: collision radius + attack range gate, ActualCombat/ForecastEnsemble namespace roll, 0.01 fixed-point damage/defense/penetration, health/balance clamp, catalog stacking을 구현했다.
- effect: unknown reference는 거부하고 chance/Unique/Replace/Strongest/AdditiveWithCap/StackCount/DurationRefresh/Independent를 deterministic하게 처리한다. persistent lifetime은 combat-only로 승격하지 않는다.
- 로그: Collision/AttackRoll/DamageApplied/EffectApplied/EffectSuppressed를 stable tick/sequence로 full log에 저장하고 중요도 필터로 core log를 만든다.
- main 검증: cargo fmt --all -- --check, targeted 6개 combat suite(총 52개: contract 5, execution 6, opportunity 12, resolution 11, simulation 10, state 8), cargo test --workspace --no-fail-fast, git diff --check 통과.
- 추가 회귀: collision miss에도 balance 적용, lethal health clamp, range/penetration/accuracy/overflow, mode/presentation parity, forecast namespace repeatability, input order invariance, effect stacking/log filter를 고정했다.
- non-goal: renderer adapter, 전투 종료 보고서/결착, 배경 전투·증원·대형/결속/패주, full skill/loadout/cooldown/breath schema, 밸런스 표 확정.
- 다음: 다수전 행동/결착 조건 또는 ScenePage/WASM 관전 adapter를 별도 승인 plan으로 나눈다.
