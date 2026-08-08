# T4 S3a — deterministic runtime checkpoint restore

status: implemented
date: 2026-08-08
baseline_commit: `1cc1f83`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step6_2608081430.md`

구현자는 `/caveman lite`를 적용한다. 소유 파일은
`crates/escape-core/src/combat_runtime.rs`와 해당 모듈의 `#[cfg(test)]`뿐이다.

이 slice는 full-frame 내부 checkpoint를 serde round-trip하고, 결정론적으로 runtime을
복구하는 기반만 만든다. public lib export, `SaveEnvelope` 연결, delta encoding, response effect
적용/state 재구성은 다음 slice다.

## 2. 목표

paused 또는 진행 중인 `CombatRuntime`을 checkpoint로 캡처한 뒤 새 runtime으로 복원한다.
복원 시 동일 request를 처음부터 deterministic replay해 simulation/stepper 내부 상태를
재구성하고, 저장된 frame과 byte-equivalent인지 검증한다. 같은 checkpoint에서 계속 실행한
결과는 원본 runtime과 같아야 한다.

## 3. 내부 계약

보조 타입명은 기존 스타일에 맞춰도 되지만 다음 정보는 보존해야 한다.

- `request`
- `execution_frames`, `resolution_frames`
- opportunity config/context와 `paused` marker
- `segment_index`, `selection_history`, `next_segment_seed`

권장 API:

```rust
pub(crate) fn checkpoint(&self) -> Result<CombatRuntimeCheckpoint, CombatRuntimeError>;
pub(crate) fn restore(checkpoint: CombatRuntimeCheckpoint) -> Result<Self, CombatRuntimeError>;
```

`CombatRuntimeCheckpoint`는 내부 `Serialize + Deserialize` 타입이다. serde JSON은 테스트에서만
사용한다. 기존 public result/save schema와 simulation version을 변경하지 않는다.

검증 규칙:

- request ticks는 0이 아니고 `max_ticks` 이하.
- execution/resolution frame 길이는 같고, tick은 1부터 연속한다.
- frame은 원본 request로 deterministic replay한 frame과 완전히 같아야 한다. 다르면
  `InvalidInput`이다.
- paused marker가 있으면 마지막 resolution tick과 일치하고 candidate가 존재해야 한다.
- selection history는 S2d1 canonical 규칙을 다시 통과해야 한다.
- checkpoint 복원은 RNG를 새로 만들지 않는다. 기존 execution namespace/effective seed와
  `derive_segment_seed`만 재사용한다.

## 4. 구현 순서

### WP-0 — drift

WSL HEAD/status와 baseline test를 확인한다. staged `cli_smoke`와 untracked handoff는 건드리지
않는다. 명시적 checkpoint 생성 지점을 먼저 센다.

### WP-1 — checkpoint serde model

`CombatRuntimePause`, selection history entry, opportunity config에 필요한 serde derive를
추가하고 내부 `CombatRuntimeCheckpoint`를 만든다. 필드 누락은 `#[serde(default)]`로 숨기지
말고 validation으로 거부한다. full frames를 저장하는 이유와 delta가 다음 slice인 사실을
주석/보고서에 남긴다.

### WP-2 — capture/restore

`checkpoint`가 현재 runtime을 값 복사로 캡처한다. `restore`는 `CombatRuntime::new` 후 저장된
frame 수만큼 `advance_tick`을 호출하고 execution/resolution frame과 대조한다. 대조 후
opportunity/pause/history/segment metadata를 복원한다.

### WP-3 — tests

1. pause 직전 checkpoint → JSON round-trip → 같은 pause marker/fingerprint.
2. response 선택 후 checkpoint → restore → 남은 ticks 실행 결과가 원본과 같다.
3. frame tick 누락/순서 변경, frame 내용 변경, paused tick 불일치, history invalid 입력은
   `InvalidInput`이다.
4. old public `CombatExecutionResult`/`CombatResolutionResult` serde와 S2a~S2d2 tests는
   그대로 통과한다.

## 5. 소유 파일·금지 범위

허용: `combat_runtime.rs` 및 해당 unit tests.

금지: `save.rs`, `lib.rs` public export, combat_execution/resolution/simulation/opportunity,
public result/schema, renderer/terminal/Web, delta compression, response effect/state mutation.

## 6. acceptance

- 내부 checkpoint serde round-trip 성공.
- restore 후 동일 입력·동일 선택 이력에서 동일 pause/finish fingerprint.
- 변조된 frame/metadata를 조용히 수용하지 않고 `InvalidInput` 반환.
- full-frame 저장의 한계와 다음 S3b public save/delta 범위를 보고서에 명시.
- `cargo fmt --all -- --check`, `git diff --check`, targeted/runtime/opportunity/full workspace
  tests 통과.

## 7. 정지 조건

- 재구성에 새 RNG source 또는 public simulation version bump가 필요해지는 경우.
- 현재 runtime private state만으로 deterministic replay가 불가능한 경우.
- checkpoint 필드 추가가 public serde boundary를 암묵적으로 바꾸는 경우.
- 기존 fingerprint/test 값이 설명 없이 변경되는 경우.

## 8. 구현 보고

- implementation commits: `505d5ff`, `1623379`, `3838557`, `25a6b32`, `2e0e0af`,
  `17fdd94`, `ead0ec4`
- 내부 `CombatRuntimeCheckpoint`가 request, full execution/resolution frames, opportunity
  state, pause marker, segment/history/seed를 serde round-trip한다.
- restore는 partial progress를 허용하고, 원본 request로 deterministic replay한 frame과 exact
  equality를 확인한다. frame length/tick, pause/candidate, opportunity budget/context,
  canonical history, derived seed를 검증한다.
- 테스트: runtime unit 12/12, opportunity integration 12/12, workspace 0 failures.
- `cargo fmt --all -- --check`, `git diff --check` 통과.
- full-frame checkpoint는 내부 기반만 제공한다. `SaveEnvelope`/public export/delta encoding과
  실제 response effect/state mutation은 다음 slice 범위다.
