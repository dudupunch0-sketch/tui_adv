# Fable Combat I2b runtime S1 — stable executor and pause provenance

## 선행 조건

`fable_combat_i2_prerequisite_step1_2608122325.md` acceptance 및 독립 review 완료.

## 소유 파일

- `crates/escape-core/src/combat_opportunity.rs`
- `crates/escape-core/src/combat_runtime.rs`
- 각 파일의 기존 test 또는 `combat_opportunity_wave1.rs`

## 구현

- opportunity evaluation과 runtime preflight의 `any_capable` 후보를 capability/can_act/active/hp 조건으로 필터링한 뒤 stable combatant ID 오름차순으로 선택한다.
- pause 생성 직전 triggering tick 시작 combatant 상태를 immutable provenance로 캡처한다.
- `pause_id`, `bound_target_ids`, `bound_target_tick`, `trigger_tick`, tick-start active/health 상태를 checkpoint에 보존한다.
- `selected_target`은 정확히 하나의 bound ID만 허용한다. triggering tick 시작에는 active였으나 같은 tick resolution으로 KO된 대상은 허용하고, 이전 tick부터 KO/fled/departed/captured인 대상은 거부한다.
- 기존 simulation/result fingerprint는 변경하지 않는다.

## Acceptance

- observer vector 순서를 뒤집어도 같은 `any_capable` ID.
- capability 누락, inactive, hp=0 후보 제외.
- same-tick lethal target preflight 허용; prior-KO target 거부.
- pause checkpoint JSON roundtrip과 malformed provenance 명시적 거부.
- 기존 opportunity/runtime targeted tests와 workspace tests 통과.

## 중단 조건

- static participant authoring flag를 mutation해야 한다.
- checkpoint version bump 없이 provenance 호환성을 보장할 수 없다.
- terminal/lifecycle 또는 GameState mutation이 필요하다.
