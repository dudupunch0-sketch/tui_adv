# T4 S2d2 — paused response selection + segment transition

status: implemented
date: 2026-08-08
baseline_commit: `3bd3fc0`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step5_2608081415.md`

구현자는 `/caveman lite`를 적용한다. 소유 파일은
`crates/escape-core/src/combat_runtime.rs`와 그 모듈의 `#[cfg(test)]`뿐이다. S2d1의
순수 `derive_segment_seed`를 paused candidate 선택에 연결한다. 실제 response effect 적용,
새 simulation/stepper 재구성, save/serde schema는 다음 slice로 넘긴다.

## 2. 목표

paused runtime이 candidate에 대해 유효한 response를 하나만 받아 canonical selection
history를 기록하고, 다음 segment용 seed를 결정론적으로 계산한다. 같은 pause + 같은
response + 같은 입력이면 같은 history/fingerprint/seed를 만들며, 다른 유효 response는
seed가 달라야 한다. no-intervention도 명시적인 response id로 기록한다.

## 3. 내부 계약

추상 계약은 다음 동작을 만족해야 한다. 정확한 보조 타입명은 기존 코드 스타일에 맞춰도 되지만
public export/serde boundary는 열지 않는다.

- runtime은 `segment_index`, `selection_history`, `base_effective_seed`,
  `simulation_version`, `manifest_fingerprint`, `namespace`를 내부 상태로 가진다.
- `resume_with_response(response_id: &str)` 또는 동등한 crate-private API를 추가한다.
- paused candidate가 없으면 `InvalidInput`이다.
- response id가 현재 pause evaluation의 options에 없으면 `InvalidInput`이다.
- 선택 시 현재 pause의 candidate `instance_id`, `opportunity_id`, tick, 현재 segment index를
  `CombatRuntimeSelectionHistoryEntry`로 기록한다.
- 선택 후 `segment_index += 1`하고 S2d1 `derive_segment_seed`를 호출한다. 반환값 또는 내부
  provenance는 테스트에서 검증 가능해야 한다.
- 선택 이력은 canonical sort 규칙을 유지하고, 같은 pause에서 두 번 선택할 수 없다.
- `resume_no_intervention()`은 기존 호출 호환성을 유지하되 내부적으로 response id
  `no_intervention`을 기록한다. evaluation에 해당 option이 실제로 없으면 `InvalidInput`이다.
- 이 slice에서 선택은 state/effect를 변경하지 않는다. 다음 slice가 seed를 사용해 runtime을
  재구성·재개한다.

## 4. 구현 순서

### WP-0 — drift

WSL HEAD/status와 baseline test를 기록한다. 기존 staged `cli_smoke`와 untracked handoff는
건드리지 않는다.

### WP-1 — selection state

`CombatRuntime` 내부에 segment/history/provenance 상태를 추가한다. 기존 `new` 및
`with_opportunities` 입력에서 effective seed, version, manifest fingerprint, namespace를
재사용한다. 새 RNG/hash helper를 만들지 않는다.

### WP-2 — response validation + transition

paused evaluation option을 stable response id로 검증하고, candidate metadata를 포함한
history entry를 저장한다. S2d1 `derive_segment_seed`로 다음 seed를 계산한다. 선택 후 pause를
해제하되, 실제 전투 state 변경은 하지 않는다.

### WP-3 — deterministic tests

1. actionable response 선택이 정확한 tick/instance/opportunity/segment history를 만든다.
2. 동일 입력·동일 response 반복은 동일 seed/fingerprint를 만든다.
3. 다른 유효 response와 `no_intervention`은 다른 history/seed를 만든다.
4. unknown response, unpaused 호출, duplicate selection은 `InvalidInput`이다.
5. 기존 `resume_no_intervention` dedupe/pause semantics와 S2a~S2d1 테스트가 유지된다.

## 5. 소유 파일·금지 범위

허용: `combat_runtime.rs` 및 그 unit tests.

금지: combat_execution/resolution/simulation/opportunity/conclusion, public serde/schema/save,
renderer/terminal/Web, content/generated artifacts, 실제 effect/role/policy 변경.

## 6. acceptance

- pause response가 canonical history에 정확히 한 번 기록된다.
- response 선택 결과가 기존 `derive_segment_seed` 규칙으로 검증 가능한 deterministic seed를
  만든다.
- `no_intervention`은 명시적으로 기록되며, option이 없으면 거부된다.
- runtime이 아직 state를 재구성하지 않는 범위를 보고서에 명시한다.
- `cargo fmt --all -- --check`, `git diff --check`, targeted/runtime/opportunity/full workspace
  tests 통과.

## 7. 정지 조건

- 기존 opportunity response option의 no-intervention semantics를 바꿔야 하는 경우.
- seed를 적용하려면 새 RNG source, public schema/version bump, 또는 다른 core 모듈 변경이
  필요한 경우.
- pause candidate와 response id를 stable하게 매칭할 수 없는 경우.
- 기존 fingerprint/결정론 테스트가 설명 없이 바뀌는 경우.

## 8. 구현 보고

- implementation commit: `f848afb`
- `CombatRuntime`이 `segment_index`, selection history, next segment seed를 내부 보유한다.
- `resume_with_response`가 현재 pause candidate의 option을 검증하고
  `(segment, tick, instance, opportunity, response)` entry를 한 번 기록한다.
- `resume_no_intervention`은 기존 API를 유지하면서 `no_intervention` option을 같은 경로로
  기록한다. 실제 response effect/state 재구성은 의도적으로 다음 slice 범위다.
- 테스트: runtime unit 8/8, opportunity integration 12/12, workspace 0 failures.
- `cargo fmt --all -- --check`, `git diff --check` 통과.
