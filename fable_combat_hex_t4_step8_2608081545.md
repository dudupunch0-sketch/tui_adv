# T4 S3b — additive public combat save boundary

status: implemented
date: 2026-08-08
baseline_commit: `f5aa11c`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step7_2608081505.md`

구현자는 `/caveman lite`를 적용한다. 이 slice는 S3a 내부 checkpoint를 **additive public
save boundary**로 연결한다. full-frame 저장은 유지하고 delta/압축, response effect/state mutation,
renderer 연결은 다음 slice다.

## 2. 목표

기존 `SaveEnvelope`가 combat checkpoint를 선택적으로 보관하도록 한다. combat 필드가 없는
기존 JSON은 그대로 읽고, `None`일 때 기존 save JSON bytes가 변하지 않아야 한다. checkpoint
payload에는 schema version, simulation version, manifest fingerprint, effective seed/namespace,
선택 이력과 frame 진행 상태가 보존된다.

## 3. 경계 계약

- `CombatRuntimeCheckpoint`를 public serde DTO로 승격하고
  `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION`을 둔다.
- `CombatRuntimeCheckpoint`는 S3a 필드를 유지한다. public visibility가 필요한 필드만
  노출하고 내부 opportunity 구현 세부는 private field로 숨겨도 된다.
- 확정 결정: `CombatRuntimeCheckpoint` 자체는 `lib.rs`에서 re-export하지만
  `opportunities` 필드와 `CombatRuntimeOpportunityState` 타입은 private으로 유지한다.
  serde DTO 경계에는 포함되지만 Rust public API에 opportunity 구현 세부를 누출하지 않는다.
- `SaveEnvelope`에 다음 additive field를 추가한다.

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub combat_checkpoint: Option<CombatRuntimeCheckpoint>,
```

- `save_state`는 기존 동작대로 `combat_checkpoint: None`을 만든다. 기존 `load_state`는
  missing combat key를 `None`으로 읽는다.
- `lib.rs`에서 checkpoint DTO와 schema constant를 export한다. `CombatRuntime` 자체를 public
  API로 열거나 renderer에 배선하지 않는다.
- 새 schema version은 additive optional field만으로 유지한다. 기존 `SAVE_SCHEMA_VERSION` 또는
  simulation version을 임의 bump하지 않는다. bump가 필요하다고 판단되면 정지 보고한다.

## 4. 구현 순서

### WP-0 — drift + constructor inventory

WSL HEAD/status와 baseline test 확인. `SaveEnvelope {` 및 `save_state` 명시적 생성 지점을 모두
센다. staged `cli_smoke`와 untracked handoff는 건드리지 않는다.

### WP-1 — public DTO/schema

S3a checkpoint에 schema version과 provenance 검증을 추가한다. `restore`는 지원하지 않는
checkpoint schema, simulation version/manifest mismatch를 `InvalidInput`으로 거부한다.
기존 internal runtime tests를 유지한다.

### WP-2 — SaveEnvelope additive field

`save.rs`의 envelope/constructor/load 경로에 optional combat field를 연결한다. `save_state`의
기존 JSON이 byte-equivalent인지 확인하고, combat checkpoint를 넣은 envelope가 JSON
round-trip되는지 테스트한다.

### WP-3 — public export + tests

1. old save JSON(combat key 없음) load 성공, `combat_checkpoint == None`.
2. `save_state` output에서 combat key가 생략됨.
3. checkpoint 포함 envelope JSON round-trip 후 schema/provenance/history/frame 보존.
4. unsupported checkpoint schema와 altered manifest/version은 `InvalidInput`.
5. existing execution/resolution/opportunity/runtime/workspace tests 0 failures.

## 5. 소유 파일·금지 범위

허용: `crates/escape-core/src/combat_runtime.rs`, `crates/escape-core/src/save.rs`,
`crates/escape-core/src/lib.rs`, 해당 unit/integration tests.

금지: combat_execution/resolution/simulation/opportunity/conclusion, renderer/terminal/Web,
delta encoding/compression, response effect/state mutation, content/generated artifacts.

## 6. acceptance

- 기존 SaveEnvelope JSON compatibility 유지.
- checkpoint 포함 save round-trip이 deterministic provenance/history/frame을 보존.
- public DTO가 schema/version/manifest mismatch를 조용히 수용하지 않음.
- full-frame payload 크기를 측정해 보고서에 남기고, delta는 별도 S3c로 명시한다.
- `cargo fmt --all -- --check`, `git diff --check`, targeted/save/runtime/full workspace tests 통과.

## 7. 정지 조건

- SaveEnvelope additive field만으로 old JSON compatibility를 보장할 수 없는 경우.
- public export가 private opportunity type을 누출하거나 API 이름이 두 가지로 해석되는 경우.
- schema/simulation version bump가 필요한데 canonical owner가 불명확한 경우.
- frame payload 크기 측정 없이 임의 delta/상한을 추가해야 하는 경우.

## 8. 구현 보고

- implementation commits: `be77d67`, `0cd7fab`
- `CombatRuntimeCheckpoint`와 schema constant를 public export하고, `SaveEnvelope`에
  `combat_checkpoint: Option<_>` additive field를 연결했다. 기존 `save_state`는 `None`을
  유지해 combat key를 생략한다.
- old SaveEnvelope JSON missing combat key round-trips to `None`; checkpoint 포함 envelope와
  schema/provenance mismatch rejection을 테스트했다.
- 테스트: runtime lib 17/17, save compatibility 1/1, event-stage 11/11, workspace 0 failures.
- S3a response-selected checkpoint fixture의 full-frame JSON payload은 4,429 bytes였다.
  delta/compression은 수치 측정 후 S3c에서 별도 결정한다.
- `cargo fmt --all -- --check`, `git diff --check` 통과.
