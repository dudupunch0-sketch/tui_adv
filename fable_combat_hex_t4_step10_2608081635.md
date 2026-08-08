# T4 S3d — deterministic sparse frame delta primitive

status: implemented
date: 2026-08-08
baseline_commit: `68501d4`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step9_2608081605.md`

구현자는 `/caveman lite`를 적용한다. 현재 full checkpoint는 12 participant·1,200 tick에서
3,800,404 bytes다. 이 slice는 이를 근거로 **내부 deterministic sparse delta primitive**만
만든다. public SaveEnvelope 교체, schema 변경, 압축 codec 선택은 다음 slice다.

## 2. 목표

execution/resolution frame 배열을 이전 frame 기준 sparse delta로 encode/decode한다. decode 결과는
원본 frame과 구조적으로 완전히 같아야 한다. 입력 순서, HashMap 순서, wall-clock에 의존하지
않는다.

## 3. 내부 계약

권장 crate-private 타입:

```rust
struct CombatRuntimeFrameDelta {
    tick: u32,
    moves: Option<Vec<CombatMoveIntent>>,
    position_updates: BTreeMap<String, HexCoord>,
    execution_fingerprint: String,
    outcomes: Option<Vec<CombatAttackOutcome>>,
    combatant_updates: Vec<CombatResolutionCombatant>,
    resolution_fingerprint: String,
}
```

- 첫 delta는 moves/position/combatants를 전부 포함한다. 이후 frame에서 이전 값과 동일한
  moves/outcomes는 `None`으로 생략한다.
- positions는 이전 map과 다른 id만 BTreeMap으로 저장한다. combatants는 id 정렬 Vec로
  저장하며 duplicate id는 InvalidInput이다.
- decode는 tick 1부터 연속성을 검증하고, update를 이전 snapshot에 적용한다. 첫 delta의
  `None`, tick gap, duplicate combatant id, empty fingerprint는 InvalidInput이다.
- 기존 frame fingerprint 문자열은 재계산하지 않고 보존한다. delta는 새 RNG/hash source를
  만들지 않는다.

## 4. 구현 순서

### WP-0 — shape inventory

`CombatTickFrame`/`CombatResolutionFrame` 필드와 serde order를 직접 확인한다. public DTO/save
field는 추가하지 않는다.

### WP-1 — encode/decode

`encode_frame_deltas(execution, resolution)`와 `decode_frame_deltas(deltas)`를 추가한다.
길이 불일치, tick 불연속, missing map entry는 Result error로 반환한다.

### WP-2 — tests + measurement

1. 기존 2 participant fixture: encode→decode가 execution/resolution frames와 exact equality.
2. 12 participant·1,200 tick fixture: delta JSON bytes를 full-frame JSON bytes와 함께 출력.
3. delta가 실제로 더 작아지는지는 fixture 측정값으로만 보고한다. 작아지지 않으면 목표를
   조용히 고치지 말고 S3e에서 표현을 재설계한다.
4. malformed delta(tick gap, first None, duplicate id)는 InvalidInput.
5. 기존 checkpoint restore/fingerprint/workspace tests 유지.

## 5. 소유 파일·금지 범위

허용: `crates/escape-core/src/combat_runtime.rs` 및 해당 `#[cfg(test)]`.

금지: save.rs/lib.rs/public SaveEnvelope/schema, combat_execution/resolution/simulation,
renderer/terminal/Web, response effect/state mutation, generated content.

## 6. acceptance

- encode/decode exact equality 및 deterministic order.
- malformed delta panic 없음.
- full vs delta JSON bytes를 stable label로 측정·보고.
- `cargo fmt --all -- --check`, `git diff --check`, targeted/full workspace tests 통과.

## 7. 정지 조건

- frame field를 보존하려면 public schema/fingerprint 계산식을 변경해야 하는 경우.
- sparse delta가 현재 fixture에서 더 작지 않아도 임의 압축 목표를 추가하지 않는다.
- 기존 frame/result fingerprint가 설명 없이 변하는 경우.

## 8. 구현 보고

- implementation commits: `28f2110`, `0ce15b7`
- crate-private sparse delta encode/decode를 추가했다. moves/outcomes는 unchanged면 생략하고,
  positions/combatants는 BTreeMap 기반 changed update만 저장한다. decode는 이전 snapshot을
  carry-forward하며 duplicate id/tick gap/first None/fingerprint 오류를 거부한다.
- 2 participant fixture exact equality 및 malformed delta 테스트 추가.
- WSL 측정값(12 participant·1,200 tick): full checkpoint JSON `3,800,404` bytes,
  sparse delta JSON `211,902` bytes. 두 payload는 metadata 범위가 다르므로 절대 비교값으로
  기록하고, public 저장 교체는 다음 slice에서 결정한다.
- runtime unit 17/17, workspace 0 failures, fmt/diff check 통과.
