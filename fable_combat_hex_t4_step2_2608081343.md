# T4 S2a — interleaved runtime primitive

status: ready-for-implementation
date: 2026-08-08
baseline_commit: `d2c5e19`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기 순서와 운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step1_2608081244.md` §10

구현자는 `/caveman lite`를 적용한다. 구현은 WSL에서 수행한다. 이 plan은 한 subagent가
완료할 수 있는 S2a 한 단위다. 작업자는 기존 staged `crates/escape-terminal/tests/cli_smoke.rs`
와 untracked `HANDOFF_combat_wave3_2608021400.md`를 건드리지 않는다.

## 2. 목표

현재 `execute_combat`가 `CombatSimulation::run_ticks`로 모든 이동 frame을 먼저 만든 뒤
`resolve_combat`가 전체 frame을 판정한다. 이를 대체할 내부 runtime primitive를 만든다.
runtime은 매 호출마다 다음 순서를 정확히 수행한다.

```text
CombatSimulation::advance_tick()  # 1회
  → CombatResolutionStepper::step(frame)  # 같은 frame.tick
  → CombatRuntimeFrame 반환
```

첫 frame의 `frame.tick`은 simulation의 실제 값인 **1**이다. 배열 index를 tick으로 재해석하지
않는다. runtime은 `request.execution.ticks`만큼만 소비하며, 상한을 넘긴 호출은 `None`을
반환한다. 아직 결착·개입·KO roster 변경은 이 slice의 책임이 아니다.

## 3. 고정 내부 계약

새 `crates/escape-core/src/combat_runtime.rs`에 다음 crate-private 타입을 만든다. 이름·필드는
이 계약을 벗어나지 않는다.

```rust
pub(crate) struct CombatRuntimeFrame {
    pub execution: CombatTickFrame,
    pub resolution: CombatResolutionFrame,
}

pub(crate) struct CombatRuntime {
    // simulation, stepper, exact execution metadata, accumulated frames/logs
}

impl CombatRuntime {
    pub(crate) fn new(
        request: CombatResolutionRequest,
    ) -> Result<Self, CombatRuntimeError>;
    pub(crate) fn advance_tick(
        &mut self,
    ) -> Result<Option<CombatRuntimeFrame>, CombatRuntimeError>;
    pub(crate) fn finish(
        self,
    ) -> Result<CombatResolutionResult, CombatRuntimeError>;
}
```

`CombatRuntimeError`는 새 runtime 모듈 내부 crate-private enum으로 두고
`Execution(CombatExecutionError)`, `Simulation(CombatSimulationError)`,
`Resolution(CombatResolutionError)`, `InvalidInput`을 명시적으로 보존한다. panic, `unwrap`로
입력 오류를 삼키지 않는다.

`finish()`가 조립하는 `CombatExecutionResult`/`CombatResolutionResult`는 기존 serde 필드와
fingerprint 계산식을 그대로 사용한다. `CombatPresentationSpeed`는 metadata로 보존하되
simulation·resolution seed와 frame 결과에 영향을 주지 않는다. forecast는 기존
`ForecastEnsemble` namespace와 `manifest.derived_seed`를 그대로 사용한다.

## 4. 구현 순서

### WP-0 — drift 확인

- WSL에서 `git status --short --branch -uall`, `git rev-parse HEAD` 실행.
- `cargo test --workspace --no-fail-fast --quiet`가 baseline과 다르면 정지 보고.
- `CombatExecutionRequest`/`CombatResolutionRequest` 생성 지점을 확인하고, 새 public serde
  필드나 simulation version bump를 만들지 않는다.

### WP-1 — execution metadata helper

`crates/escape-core/src/combat_execution.rs`의 기존 `execute`에서 namespace, effective seed,
setup fingerprint, provenance, execution log/fingerprint 계산을 재사용 가능한 crate-private
helper로 분리한다. 기존 `execute` 결과는 byte-for-byte/fixture parity를 유지한다.

### WP-2 — runtime primitive

`combat_runtime.rs`에서 effective-seed input으로 `CombatSimulation::new`를 한 번 만들고,
S1의 `CombatResolutionStepper::new`를 execution metadata와 함께 만든다. `advance_tick`은
simulation을 정확히 한 번 호출하고 같은 tick frame을 stepper에 넘긴 뒤 두 frame을 누적한다.
`request.execution.ticks == 0`, max tick 초과, simulation/stepper 입력 오류는 각각 `Result`로
반환한다. 여기서는 `CombatSimulation` 내부의 inactive roster semantics를 바꾸지 않는다.

### WP-3 — batch 조립·회귀 테스트

`finish()`는 누적 execution frames와 move logs를 기존 순서로 조립하고, stepper state/log와
기존 fingerprint 튜플을 사용해 `CombatResolutionResult`를 만든다. `resolve_combat`의 기존
batch path는 이 runtime을 아직 production default로 교체하지 않아도 되지만, runtime 결과를
기존 `resolve_combat`와 비교하는 테스트를 추가한다. fixture에 결착이 발생하면 S2b 이후
정책이 달라질 수 있으므로, S2a parity fixture는 모든 tick에서 양 진영이 살아 있는 값으로
고정한다.

## 5. 소유 파일

수정/신규 허용:

- `crates/escape-core/src/combat_runtime.rs` (신규)
- `crates/escape-core/src/combat_execution.rs`
- `crates/escape-core/src/lib.rs` (crate 내부 모듈 등록만)
- `crates/escape-core/tests/combat_hex_t4_runtime_step2.rs` (신규)

수정 금지:

- `combat_simulation.rs` (KO/active overlay는 S2b)
- `combat_resolution.rs` (S1 contract 동결)
- `combat_opportunity.rs`, `combat_conclusion.rs`, `combat_state.rs`
- `combat_hex.rs`, `combat_spectator.rs`, `scene_page.rs`, terminal/Web, docs/dev canonical 문서
- save/progress/serde schema 및 generated artifact

## 6. acceptance criteria

- `advance_tick()` 한 번이 simulation tick 1회와 stepper tick 1회를 정확히 발생시킨다.
- 반환된 `execution.tick == resolution.tick`이며 첫 tick은 1이다.
- runtime을 동일 입력으로 두 번 실행하면 frame/state/log/fingerprint가 같다.
- runtime batch fixture와 기존 `resolve_combat`의 resolution frames/state/full log/fingerprint가
  같다(결착 없는 fixture).
- `ticks == 0`/상한 초과/잘못된 frame metadata가 panic 없이 `Err` 또는 `None`으로 처리된다.
- forecast와 actual namespace/seed가 기존 execution contract와 동일하다.
- `cargo fmt --all -- --check`, targeted runtime/execution/resolution tests,
  `cargo test --workspace --no-fail-fast --quiet`, `git diff --check` 통과.

## 7. 범위 밖·다음 plan

- S2b: resolution frame의 health를 다음 simulation tick active overlay에 반영하는 KO roster sync.
- S2c: `CombatOpportunityCatalog::evaluate`를 tick 후 결착 전 marker로 연결하는 pause contract.
- S2d: selection history와 segment index를 canonicalize해 resumed seed/provenance를 만드는 contract.
- 결착 우선순위, response effect 적용, save/JSON, renderer replay, presentation speed 이동은 이후
  plan에서만 다룬다.

## 8. 정지 조건

- execution fingerprint/기존 fixture가 바뀌는데 원인을 순수 metadata 추출로 설명할 수 없음.
- `CombatTickFrame.tick`을 index로 바꾸거나 runtime이 `run_ticks`를 호출해야만 동작함.
- runtime이 static participant `active` 필드를 변경해야 함(이는 S2b overlay 설계로 이동).
- 새 RNG source, public serde 필드, simulation version bump, 금지 파일 수정이 필요함.
- `resolve_combat`와 runtime parity가 결착 없는 fixture에서도 달라짐.

## 9. 구현 보고 형식

- baseline/final commit 및 workspace 수치
- 최종 `CombatRuntime`/`CombatRuntimeFrame`/`CombatRuntimeError` 서명
- 첫 tick 번호, 호출별 frame 수, runtime-vs-batch fingerprint 비교값
- 변경 파일과 범위 밖 diff 여부
- S2b가 소비할 runtime state/active roster 질문
