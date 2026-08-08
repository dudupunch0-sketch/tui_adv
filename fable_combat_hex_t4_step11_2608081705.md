# T4 S3e — compact checkpoint storage selection

status: implemented
date: 2026-08-08
baseline_commit: `52c3a78`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)
implementation_commits: `5e1d3d5`, `99d1361`

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step10_2608081635.md`

구현자는 `/caveman lite`를 적용한다. S3d sparse delta를 checkpoint 저장 경로에 선택적으로
연결한다. full-frame checkpoint JSON과 old SaveEnvelope compatibility는 유지한다. response
effect/state mutation, renderer wiring, new schema version은 범위 밖이다.

## 2. 목표

같은 runtime에 대해 full checkpoint와 compact checkpoint를 각각 만들 수 있게 한다.
compact checkpoint는 S3d delta만 저장하고 restore 시 동일 execution/resolution frames를
복원한다. 기존 full checkpoint와 old JSON은 계속 읽힌다.

## 3. 내부 계약

- `CombatRuntimeCheckpoint`에 additive optional `frame_deltas`를 둔다.
- `checkpoint()`는 기존 full-frame payload를 계속 만든다.
- `checkpoint_compact()` 또는 동등한 crate-private API는 full frame arrays를 비우고
  `frame_deltas: Some(...)`을 채운다.
- restore 입력은 다음 중 정확히 하나만 허용한다.
  1. full execution/resolution frames
  2. non-empty `frame_deltas`
- 둘 다 비어 있거나 둘 다 채워지면 `InvalidInput`이다.
- restore는 delta decode 후 기존 frame tick/provenance/seed/pause 검증을 동일 적용한다.
- public SaveEnvelope field와 schema version은 변경하지 않는다. compact checkpoint는 기존
  `combat_checkpoint` field에 그대로 담긴다.

## 4. 구현 순서

### WP-0 — storage shape inventory

현재 checkpoint/restore 필드와 S3d encode/decode를 재확인한다. serde default/skip 규칙을
임의로 바꾸지 않는다.

### WP-1 — compact capture

`checkpoint_compact()`를 추가하고 full/compact metadata가 동일한지 검증한다. compact payload는
frame delta만 바뀌어야 한다.

### WP-2 — restore dual path

restore가 full 또는 delta path를 안전하게 선택하도록 한다. malformed both/none, corrupt
delta, tick gap은 `InvalidInput`으로 반환한다. panic 금지.

### WP-3 — tests

1. full checkpoint restore 결과와 compact checkpoint restore 결과 fingerprint 동일.
2. compact checkpoint JSON round-trip 후 finish parity.
3. 12 participant·1,200 tick fixture에서 compact JSON bytes와 full JSON bytes를 stable label로
   비교한다.
4. both/none frame storage, corrupt delta 입력은 `InvalidInput`.
5. old SaveEnvelope, runtime/opportunity/full workspace tests 유지.

## 5. 소유 파일·금지 범위

허용: `crates/escape-core/src/combat_runtime.rs` 및 해당 `#[cfg(test)]`.

금지: save.rs/lib.rs/public API/schema/version bump, combat_execution/resolution/simulation,
renderer/terminal/Web, response effect/state mutation, generated content.

## 6. acceptance

- full/compact checkpoint가 같은 결과·fingerprint를 복원한다.
- compact JSON이 S3d 측정값과 일관된 크기 감소를 보인다. 숫자는 보고서에 기록한다.
- malformed storage representation이 panic 없이 거부된다.
- fmt/check/targeted/full workspace tests 통과.

## 7. 정지 조건

- full checkpoint compatibility를 깨지 않고 dual storage를 표현할 수 없는 경우.
- compact restore가 새 RNG source 또는 public schema bump를 요구하는 경우.
- delta decode 결과가 기존 frame/result fingerprint와 달라지는 경우.

## 8. 구현·검수 결과

- `CombatRuntimeCheckpoint.frame_deltas`는 additive optional 필드이며 `serde(default)`로
  기존 full checkpoint JSON(필드 없음)을 계속 읽는다. `checkpoint()`는 full frame을 유지하고,
  `checkpoint_compact()`는 frame 배열을 비우고 S3d delta만 저장한다.
- restore는 full frame 또는 non-empty delta 중 정확히 하나만 허용한다. both/none/empty delta,
  tick gap 및 corrupt delta는 `InvalidInput`으로 거부하며 panic 경로가 없다.
- full/compact JSON roundtrip과 finish fingerprint parity, 구버전 checkpoint 필드 누락 호환,
  malformed storage 테스트를 추가했다.
- 12 participant·1,200 tick 측정값:
  `checkpoint_json_bytes=3,800,404`, `compact_checkpoint_json_bytes=220,138`,
  `delta_json_bytes=211,902`, `save_envelope_json_bytes=3,801,007`.
  compact payload는 full 대비 약 94.2% 작다.
- 검증: runtime 19/0, workspace 0 failures, `cargo fmt --all`, `cargo check -p escape-core`,
  `git diff --check` 통과. 사용자 변경 `cli_smoke` 및 `HANDOFF_combat_wave3_2608021400.md`는
  커밋하지 않았다.
