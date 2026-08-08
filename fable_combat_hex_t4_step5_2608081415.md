# T4 S2d1 — canonical selection-history segment seed primitive

status: ready-for-implementation
date: 2026-08-08
baseline_commit: `e0e0060`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step4_2608081407.md` §9

구현자는 `/caveman lite`를 적용한다. 소유 파일은 `crates/escape-core/src/combat_runtime.rs`와
그 모듈의 `#[cfg(test)]`뿐이다. 이 slice는 seed 계산 primitive만 만들고, response effect 적용,
runtime resume, save/serde schema는 다음 slice로 넘긴다.

## 2. 목표

같은 manifest·base effective seed·simulation version·RNG namespace·segment index·선택 이력이
항상 같은 segment seed를 만들도록 canonical selection history를 만든다. history 입력 순서가
달라도 동일한 segment 순서/심화 tick이면 같은 seed를 만든다. 유효한 response id가 달라지면
seed가 달라져야 한다.

새 RNG source를 만들지 않는다. 기존 deterministic FNV/stable fingerprint 해시와
`CombatRngNamespace`만 사용한다. 계산 결과는 아직 public serde boundary에 노출하지 않는다.

## 3. 고정 내부 계약

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct CombatRuntimeSelectionHistoryEntry {
    pub(crate) segment_index: u32,
    pub(crate) tick: u32,
    pub(crate) instance_id: String,
    pub(crate) opportunity_id: String,
    pub(crate) response_id: String,
}

pub(crate) fn derive_segment_seed(
    base_effective_seed: u64,
    namespace: CombatRngNamespace,
    simulation_version: &CombatSimulationVersion,
    manifest_fingerprint: &str,
    segment_index: u32,
    history: &[CombatRuntimeSelectionHistoryEntry],
) -> Result<u64, CombatRuntimeError>;
```

검증 규칙:

- manifest/version/id 문자열은 trim-empty면 `InvalidInput`이다.
- history의 각 id는 비어 있지 않아야 한다.
- canonical history는 `(segment_index, tick, instance_id, opportunity_id, response_id)`로 정렬한다.
- 동일 `segment_index`가 두 번 있거나 entry segment가 요청 segment보다 크면 `InvalidInput`이다.
- payload는 `(base_effective_seed, namespace.as_str(), simulation_version.as_str(),
  manifest_fingerprint, segment_index, canonical_history)`를 직렬화해 기존 stable FNV로 해시한다.
- forecast/actual namespace가 다르면 seed도 달라진다.

## 4. 구현 순서

### WP-0 — drift

WSL HEAD/status와 workspace baseline을 기록한다. 기존 staged cli_smoke/untracked handoff는
건드리지 않는다.

### WP-1 — canonical history + hash

`combat_runtime.rs`에 위 타입/함수를 추가한다. existing `CombatRuntimeError::InvalidInput`과
`stable_fingerprint` helper를 재사용하고, 별도 RNG/seed source를 추가하지 않는다.

### WP-2 — deterministic tests

1. 같은 history의 input order를 섞어도 같은 seed/fingerprint.
2. response id가 다르면 seed가 다름.
3. actual vs forecast namespace가 분리됨.
4. duplicate segment, empty id, future segment 입력은 `InvalidInput`.

## 5. 소유 파일·금지 범위

허용: `combat_runtime.rs` 및 그 unit tests.

금지: combat_execution/resolution/simulation/opportunity/conclusion, public serde/schema/save,
renderer/terminal/Web, content/generated artifacts.

## 6. acceptance

- pure function이 동일 입력을 반복해 동일 u64 seed를 낸다.
- history representation 순서와 response 선택 변화가 명시된 규칙대로 seed에 반영된다.
- 기존 S2a/S2b/S2c runtime behavior와 workspace tests가 유지된다.
- fmt/check/targeted/full workspace tests 통과.

## 7. 범위 밖·정지

response를 실제 combat state에 적용하거나 paused runtime을 새 seed로 재구성하는 것은 다음
slice다. public provenance/schema/version bump, 기존 hash algorithm 변경, RNG namespace 추가가
필요하면 정지 보고한다.
