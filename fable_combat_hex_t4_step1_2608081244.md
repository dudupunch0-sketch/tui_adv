# T4 S1 — tick 단위 실행·판정 인터리빙

status: implemented
date: 2026-08-08
baseline_commit: `2ae781fdb427b94f10353529aaddb1341d986b1b`
baseline_test: `cargo test --workspace --no-fail-fast` = **434 passed / 0 failed**
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 구현자에게 먼저 읽힐 문서

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `docs/design/Combat_Hex_Rework_Development_Plan.md` §5, §6 T4, §9, §10
4. `fable_combat_hex_t4_slice_plan_2608081232.md`

구현자는 시작 시 `/caveman lite`를 적용한다. 보고는 짧게 쓰되 public API 서명, 테스트 수,
fingerprint, 실패 원인은 생략하지 않는다. 위 문서와 아래 소유 파일만 읽고 작업한다. S1은
**tick resolver primitive만** 만든다. 현재 `CombatSimulation::new`가 inactive 입력을 내부 map에서
제외하는 roster semantics를 바꾸지 않는다. 실제 simulation↔resolution interleaving, KO roster
동기화, terminal runtime, S2 개입·seed schema는 미리 만들지 않는다.

## 2. 실측된 현재 경계

현재 코드는 다음 순서다.

```text
CombatExecution::execute
  └─ CombatSimulation::run_ticks(ticks)  # 이동 frame을 상한까지 먼저 생성
CombatResolution::resolve
  └─ execute_combat를 다시 호출
  └─ execution.frames 전체를 순회하며 공격·효과·조기 결착 판정
```

실측한 위험 지점:

- `CombatSimulation::new`는 현재 inactive 입력 participant를 내부 roster에서 제외한다.
  따라서 mid-run KO를 다음 tick active 상태로 반영할 runtime 경계가 아직 없다.
- `CombatResolution::resolve`는 tick별 health snapshot을 만들지만 그 결과를 simulation에
  되먹임하지 않는다. 따라서 이동과 판정이 분리되어 있다.
- `CombatResolutionResult.execution.frames`는 resolution이 결착 tick에서 끝나도 원래
  execution 상한까지 남는다.
- `ScenePage` producer, terminal/Web adapter는 이번 슬라이스 소유가 아니다.

## 3. 목표와 고정 계약

### 목표

기존 monolithic `resolve`에서 판정 한 tick을 독립적으로 실행할 수 있는 stepper를 추출한다.
현재 batch `resolve_combat`는 이 stepper를 사용해 기존 결과를 유지한다. simulation과 stepper를
실제로 한 tick씩 묶고 KO를 다음 tick에 반영하는 runtime은 S2 소유다.

### S1에서 고정하는 동작 순서

1. tick-start health snapshot을 기준으로 한 tick의 공격을 동시에 판정한다.
2. 공격·피해·효과를 적용하고 resolution frame을 만든다.
3. stepper는 누적 state/log/effect를 보존하며 다음 frame을 받을 수 있다.

S2는 이 stepper 뒤에 simulation 한 tick과 결착·개입 순서를 배선한다.

### S1 tick stepper 계약

구현자는 아래 의미를 만족하는 non-serde `pub(crate)` core API를 제공한다. 내부 모듈명과
참조 lifetime은 바꿀 수 있지만, 변경 시 보고서에 최종 서명을 그대로 적는다.

```rust
pub(crate) struct CombatResolutionStepper { /* validated state + gauges + logs */ }

impl CombatResolutionStepper {
    pub(crate) fn new(/* resolution request + execution metadata */) -> Result<Self, CombatResolutionError>;
    pub(crate) fn step(
        &mut self,
        frame: &CombatTickFrame,
    ) -> Result<CombatResolutionFrame, CombatResolutionError>;
    pub(crate) fn finish(self) -> CombatResolutionState;
}
```

`step()`은 전달받은 한 frame만 판정하고, 기존 health snapshot·attack gauge·effect·log
semantics를 유지한다. `finish()`는 기존 `CombatResolutionState`를 만든다. terminal 판정이나
simulation active 동기화는 이 타입의 책임이 아니며 S2에서 runtime이 수행한다.

S1은 `CombatRuntime`을 만들지 않는다. presentation speed를 판정 입력으로 사용하지 않으며,
forecast namespace와 actual namespace를 섞지 않는다.

## 4. 소유 파일

### 수정 가능

- `crates/escape-core/src/combat_resolution.rs`
- `crates/escape-core/src/combat_execution.rs` (execution metadata helper가 정말 필요할 때만)
- `crates/escape-core/src/lib.rs`는 export가 정말 필요할 때만 수정한다.
- 신규 `crates/escape-core/tests/combat_hex_t4_runtime_step1.rs`
- 기존 `combat_resolution_wave2.rs`는 기대값 수정 없이 회귀만 실행한다.
- 동작과 모순되는 기존 전투 테스트 주석/설명. 고정 기대값은 이유 보고 후에만 수정한다.

### 수정 금지

- `crates/escape-core/src/combat_hex.rs`
- `crates/escape-core/src/combat_conclusion.rs`
- `crates/escape-core/src/combat_opportunity.rs`
- `crates/escape-core/src/combat_state.rs`
- `crates/escape-core/src/combat_spectator.rs`
- `crates/escape-core/src/scene_page.rs`
- `crates/escape-terminal/**`, `web/**`
- `docs/**` (보고서 제외), content bundle/generated artifact

`CombatConclusion` 로직을 복제하거나 `CombatOpportunityCatalog` 필드를 추가하지 않는다.

## 5. 작업 순서(WP)

각 WP는 별도 커밋으로 남긴다. 시작 전에 WP-0의 생성 지점 목록을 보고서에 붙인다.

### WP-0 — baseline·생성 지점 확인

- WSL에서 `git status --short --branch -uall`, `git rev-parse HEAD`를 실행한다.
- `CombatExecutionRequest`, `CombatResolutionRequest`, `CombatSimulationInput` 명시적 생성
  지점을 다시 세어 누락을 막는다.
- baseline이 434/0이 아니거나 fingerprint/fixture가 이미 다르면 구현하지 말고 정지 보고한다.
- main worktree의 기존 staged `crates/escape-terminal/tests/cli_smoke.rs`와
  untracked `HANDOFF_combat_wave3_2608021400.md`는 소유 범위 밖이므로 건드리지 않는다.

### WP-1 — resolver state 추출

- 현재 `resolve`의 입력 검증, combatant 초기화, defense/attack/effect catalog map, 누적
  health/effect/log state를 `CombatResolutionStepper` 내부 상태로 옮긴다.
- `CombatSimulation` participant roster/active semantics는 건드리지 않는다. KO roster 유지와
  active toggle은 S2 runtime 소유다.

### WP-2 — per-tick step 추출

- 기존 `resolve`의 validation, attack gauge, tick-start health snapshot, outcome/log/effect
  적용을 한 tick 단위로 호출할 수 있게 추출한다.
- 공격 정의 순서와 participant 입력 순서가 결과에 영향을 주지 않게 기존 BTree 정렬·snapshot
  규칙을 유지한다.
- 별도 runtime 오류 타입을 만들지 않는다. 잘못된 frame·입력은 기존
  `CombatResolutionError`를 `Result`로 반환한다.
- stepper가 `CombatResolutionFrame`의 combatant snapshot, full/core log, effect state를 기존
  semantics 그대로 누적하게 한다.

### WP-3 — batch wrapper 유지

- `resolve_combat`이 `execute_combat`으로 만든 execution frames를 새 stepper에 순서대로
  공급하도록 바꾼다. `CombatExecutionResult`와 `CombatResolutionResult` 조립은 기존 JSON
  필드·provenance·namespace를 유지한다.
- `execute_combat` 단독 호출의 기존 batch semantics는 유지한다. S1에서 terminal 조기 종료,
  presentation speed, save schema를 옮기지 않는다.
- S1 결과 fingerprint/fixture가 바뀌면 추출 오류로 보고하고 기대값을 고치지 않는다.

### WP-4 — 회귀·계약 테스트

신규 테스트 최소 목록:

1. `stepper_reproduces_single_tick_resolution`
2. `stepper_repeats_identically_for_same_input`
3. `stepper_is_input_order_invariant`
4. `stepper_accumulates_effects_and_logs_across_ticks`

기존 `combat_resolution_wave2`, `combat_simulation_wave2`, `combat_spectator_wave3`,
`combat_cadence_t3` 회귀를 함께 실행한다. 고정 fingerprint가 바뀌면 판정 변경인지
표시/범위 변경인지 각각 구분해 보고한다.

## 6. acceptance criteria

- `resolve_combat` production path가 추출된 tick stepper를 사용한다.
- stepper가 기존 single/batch resolution 결과·로그·fingerprint를 유지한다.
- 동일 입력 반복·participant/attack 입력 순서 변경 모두 같은 stepper/result fingerprint를 만든다.
- 기존 batch API와 additive JSON 필드는 유지된다. save/progress schema는 추가되지 않는다.
- simulation version bump가 필요하면 이유와 영향 파일을 정지 보고한다. 임의 bump 금지.
- `cargo fmt --all -- --check`, targeted tests, full workspace test, `git diff --check`가 통과한다.

## 7. 범위 밖

- S2 simulation↔resolution interleaving·결착 tick 중단·개입 후보·pause marker·selection history·segment seed
- S3 save/progress schema·delta encoding·render cache
- S4 즉발/failure effect 적용, role/policy 변경, 긴급 구조
- `CombatPresentationSpeed` 재생 layer 이동
- T5 치명타·회복·부활, T8 cue 확장, ScenePage/WASM/terminal/Web adapter

## 8. 정지 조건

다음 중 하나면 기대값을 고치지 말고 즉시 보고한다.

- 시작 baseline이 434/0이 아님
- 기존 판정값·simulation/resolution fingerprint 변화 원인을 설명할 수 없음
- tick 순서가 두 가지로 해석됨
- `CombatTickFrame`/`CombatResolutionFrame` 필드 변경 또는 simulation version bump가 필요함
- inactive 처리와 기존 frame shape 사이 계약이 모호함
- `combat_hex.rs`·금지 파일 수정이 필요함
- panic, `unwrap`으로 실제 입력 실패를 삼키는 구현이 필요함

## 9. 검증 명령과 보고 형식

구현자는 실제 출력 숫자를 보고한다. PASS라는 한 단어만 쓰지 않는다.

```bash
cd /home/dudu/work/tui-adv
cargo fmt --all -- --check
cargo test -p escape-core --test combat_hex_t4_runtime_step1
cargo test -p escape-core --test combat_simulation_wave2
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test combat_spectator_wave3
cargo test -p escape-core --test combat_cadence_t3
cargo test --workspace --no-fail-fast
git diff --check
```

보고서 필수 항목:

- baseline commit/수치와 최종 commit/수치
- WP별 변경 파일·커밋
- 최종 `CombatResolutionStepper`의 전체 public/public(crate) 서명
- stepper single-tick·누적 테스트의 실제 관측값
- 변경된 fingerprint/fixture 기대값 목록과 각각의 이유
- 소유 범위 밖 diff 여부
- 다음 S2 plan이 소비해야 할 확정 `CombatResolutionStepper` 서명과 남은 질문

## 10. 구현 결과 (2026-08-08)

- Production `resolve_combat` now uses `CombatResolutionStepper::new → step → finish`.
- Step preserves tick-start health snapshot, active/KO guards, attack gauges including multi-fire,
  deterministic collision/roll/damage, effect stacking, per-outcome effect IDs, and full logs.
- Batch fingerprint assembly remains `(execution.fingerprint, frames, state, full_log)`.
- Commits: `abe63b3` (wiring/parity), `e696f0b` (direct stepper contract tests).
- Verification: `cargo fmt --all -- --check`; direct unit contract 2/2; resolution integration 25/25;
  workspace `cargo test --workspace --no-fail-fast --quiet` 0 failures; `git diff --check` clean.
- Direct tests cover deterministic repeated step/finish and missing-position `Result` handling.
- `resolve_legacy` remains as a temporary parity oracle and should be removed after S2 confirms the
  new runtime does not need it.
