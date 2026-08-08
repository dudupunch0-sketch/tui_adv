# T4 S2c — opportunity pause marker

status: implemented
date: 2026-08-08
baseline_commit: `5e8899f`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step2_2608081343.md` §10
5. `fable_combat_hex_t4_step3_2608081402.md` §9

구현자는 `/caveman lite`를 사용한다. 소유 범위는 `combat_runtime.rs`와 그 파일의
`#[cfg(test)]` contract test다. 기존 opportunity catalog semantics를 재작성하지 않는다.

## 2. 목표

S2b runtime의 각 resolution frame 뒤에 기존 `CombatOpportunityCatalog::evaluate`를 연결한다.
candidate가 생긴 **정확한 resolution tick**에서 runtime을 멈추고, renderer가 아니라 core가 만든
pause marker를 반환한다.

```text
simulation tick N → resolution step N
  → opportunity evaluate(N)
  → candidate 없음: 다음 tick
  → candidate 있음: Pause marker 반환, 다음 tick 생성 금지
```

이 slice는 response effect 적용·selection-history seed·결착 우선순위를 구현하지 않는다.
재개는 `no_intervention`만 지원해 presented-id/budget semantics를 고정한다. 실제 response는
S2d에서 소비한다.

## 3. 고정 내부 계약

`combat_runtime.rs`에 다음 crate-private 타입과 메서드를 추가한다. 기존 S2a `new`/`advance_tick`
호출은 깨지지 않게 유지하고, opportunity-aware 경로를 별도 메서드로 둔다.

```rust
pub(crate) struct CombatRuntimeOpportunityConfig {
    pub(crate) catalog: CombatOpportunityCatalog,
    pub(crate) instances: Vec<CombatOpportunityInstance>,
    pub(crate) context: CombatOpportunityContext,
}

pub(crate) struct CombatRuntimePause {
    pub(crate) tick: u32,
    pub(crate) evaluation: CombatOpportunityEvaluation,
    pub(crate) evaluation_fingerprint: String,
}

pub(crate) enum CombatRuntimeAdvance {
    Frame(CombatRuntimeFrame),
    Paused(CombatRuntimePause),
    Complete,
}

impl CombatRuntime {
    pub(crate) fn with_opportunities(
        request: CombatResolutionRequest,
        config: CombatRuntimeOpportunityConfig,
    ) -> Result<Self, CombatRuntimeError>;
    pub(crate) fn advance_with_opportunities(
        &mut self,
    ) -> Result<CombatRuntimeAdvance, CombatRuntimeError>;
    pub(crate) fn resume_no_intervention(&mut self) -> Result<(), CombatRuntimeError>;
}
```

`with_opportunities` validates catalog/context once. Before each evaluation it clones the configured
context, sets `current_tick` to the just-resolved `frame.tick`, and passes existing
`presented_instance_ids`/budget through unchanged. When `evaluation.candidate.is_some()`, runtime
stores the candidate instance id in `presented_instance_ids`, adopts `evaluation.budget`, stores the
marker, and returns `Paused` without constructing another simulation frame. `resume_no_intervention`
clears the pause only; it does not add an RNG source or mutate combat state.

The marker fingerprint is exactly `CombatOpportunityEvaluation::fingerprint()`. Tick is the frame's
1-based `tick`, never a vector index. Free alerts do not pause and remain in the returned evaluation
only when a candidate is present; S2d may add a non-pausing alert stream if needed.

## 4. 구현 순서

### WP-0 — drift check

- WSL `git status --short --branch -uall`, HEAD, workspace tests.
- Existing staged cli_smoke/untracked handoff are outside scope.
- Stop on opportunity fixture/value/fingerprint drift that cannot be explained by wiring.

### WP-1 — opportunity-aware runtime state

Add optional config, pause state, presented-id set, and budget state to `CombatRuntime`. Preserve the
S2a no-op runtime path when no config exists. Do not change `CombatOpportunityCatalog::evaluate`.

### WP-2 — evaluate/pause/resume order

After `stepper.step` and S2b health sync, evaluate at the same frame tick. Candidate pause precedes
any future conclusion check. A paused runtime returns the same marker until
`resume_no_intervention`; advancing while paused must not call `advance_tick` again.

### WP-3 — tests

Use deterministic unit fixtures:

1. candidate trigger at tick 1 yields `Paused.tick == 1`, exact evaluation fingerprint, and no tick 2
   frame before resume;
2. no-intervention resume records presented instance and next evaluation does not repeat a deduped
   candidate; budget is consumed exactly once;
3. expiry/no-trigger yields `Frame` for every tick and never pauses;
4. repeated identical runtime/config yields equal marker/fingerprint/budget; input order changes do
   not change candidate ordering.

## 5. 소유 파일

수정 허용:

- `crates/escape-core/src/combat_runtime.rs`
- its `#[cfg(test)]` unit module

수정 금지:

- `combat_opportunity.rs` (catalog/evaluate contract frozen)
- `combat_simulation.rs`, `combat_resolution.rs`, `combat_execution.rs`
- `combat_conclusion.rs`, `combat_spectator.rs`, `scene_page.rs`, terminal/Web, save/serde/schema

## 6. acceptance criteria

- candidate가 생긴 정확한 1-based tick에서 core pause가 발생한다.
- paused 상태에서 simulation/resolution tick이 추가 생성되지 않는다.
- `resume_no_intervention` 뒤 presented-id dedupe와 budget 소비가 기존 catalog semantics와 같다.
- 동일 입력/기회 config가 동일 marker/evaluation fingerprint를 만들고, presentation speed가
  결과에 섞이지 않는다.
- 기존 S2a/S2b no-op runtime path와 workspace tests가 유지된다.
- `cargo fmt --all -- --check`, targeted runtime/opportunity tests, workspace test, diff check 통과.

## 7. 범위 밖·정지 조건

- response effect 적용, role/policy 변경, 긴급 구조, segment seed/history, save/renderer는 S2d+.
- opportunity catalog semantics 변경이나 budget/no_intervention 재해석이 필요하면 정지.
- marker tick이 frame index 또는 wall-clock에 의존하면 정지.
- public schema/version bump 또는 금지 파일 수정이 필요하면 정지.

## 8. 보고 형식

- final commit/test counts
- pause/advance/resume 최종 signatures
- trigger tick, marker fingerprint, budget/presented-id observations
- no-op path regression and out-of-scope diff report
- S2d가 소비할 pause/response boundary

## 9. 구현 결과 (2026-08-08)

- 구현 커밋: pending (runtime patch + plan update)
- `CombatRuntimeOpportunityConfig`, `CombatRuntimePause`, `CombatRuntimeAdvance`를 추가하고,
  candidate가 발생한 resolution tick에서 pause/동일 marker 재반환/no-intervention resume을
  구현했다.
- 기존 `CombatOpportunityCatalog::evaluate`를 변경하지 않고 presented id와 budget을 runtime이
  보존한다. test fixture에서 tick 1 pause, budget 1회 소비, resume 후 tick 2 진행과 dedupe를
  확인했다.
- 직접 검증: runtime unit 4/4, opportunity integration 12/12, workspace 0 failures,
  `cargo fmt --all -- --check`, `git diff --check` 통과.
- S2d는 이 pause marker를 response selection/history와 segment seed에 연결한다.
