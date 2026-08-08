# T4 S2b — KO roster synchronization

status: implemented
date: 2026-08-08
baseline_commit: `cf6dbef`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step1_2608081244.md` §10
5. `fable_combat_hex_t4_step2_2608081343.md` §10

구현자는 `/caveman lite`를 적용하고 WSL에서 작업한다. 이 slice의 code owner는
`combat_simulation.rs`, `combat_runtime.rs`, 그리고 해당 모듈의 `#[cfg(test)]` 테스트다.
사용자 staged `crates/escape-terminal/tests/cli_smoke.rs`와 untracked handoff 파일은 건드리지
않는다.

## 2. 목표와 불변식

S2a runtime은 resolution frame을 만든 뒤 다음 simulation tick을 호출한다. 이번 slice는 그
resolution frame의 health를 다음 tick의 **runtime active overlay**에 반영한다.

```text
tick N: simulation → resolution step
  → resolution combatant health <= 0 인 내부 participant를 inactive 처리
tick N+1: inactive actor는 move/target/occupancy에서 제외
```

- tick N의 frame은 마지막 공격과 모든 동시 outcome을 그대로 보존한다.
- inactive participant의 static input/manifest JSON을 수정하지 않는다.
- resolution stepper의 participant state를 임의로 mutate하지 않는다. 다음 frame의 target/move
  입력은 simulation overlay가 결정하고, stepper는 tick-start health snapshot으로 공격을 한 번 더
  방어한다.
- inactive를 roster에서 삭제하지 않는다. 상태·결론·관전용 id와 frame shape를 보존한다.

## 3. 고정 API

`CombatSimulation`에 다음 crate-private method를 추가한다.

```rust
pub(crate) fn sync_active_from_health(
    &mut self,
    health_hundredths: &BTreeMap<String, i64>,
) -> Result<(), CombatSimulationError>;
```

method는 내부 participant가 health map에 존재하고 값이 `<= 0`이면 `active=false`로 바꾼다.
map에 없는 id는 조용히 새 participant를 만들거나 제거하지 않는다. `CombatSimulation`의
public input/fingerprint는 변경하지 않는다.

`advance_tick`의 active 경계는 다음처럼 고정한다.

- active participant만 move gauge를 누적하고 move intent를 만든다.
- active participant만 occupancy snapshot과 contention 입력에 참여한다.
- target fallback은 기존 `p.active && p.side != actor.side` 규칙을 유지한다.
- frame `positions`/state id는 roster 보존을 위해 기존처럼 유지할 수 있지만, inactive actor의
  move intent는 생성하지 않는다.

`CombatRuntime::advance_tick`은 `stepper.step` 직후 resolution frame에서 health map을 만들고
`simulation.sync_active_from_health`를 호출한다. 첫 tick은 1이며 sync는 다음 호출에만 영향을
준다. S2b는 pause/conclusion/selection seed를 추가하지 않는다.

## 4. 구현 순서

### WP-0 — drift 확인

- WSL `git status --short --branch -uall`, `git rev-parse HEAD`.
- workspace baseline 0 failures가 아니면 정지 보고.
- `CombatSimulation::new`가 초기 inactive 입력을 이미 roster에서 제외하는 사실을 유지한다.

### WP-1 — simulation active boundary

`combat_simulation.rs`의 `advance_tick` loops와 occupancy helper에 active filter를 적용하고,
crate-private `sync_active_from_health`를 추가한다. 이동 gauge map은 roster 보존 때문에
삭제하지 않되 inactive actor에 대해 누적/사용하지 않는다. 현재 T1-c/T1-d occupancy와
participant/target 입력 순서를 바꾸지 않는다.

### WP-2 — runtime sync

`combat_runtime.rs`에서 resolution frame combatant snapshot을 id→health map으로 정렬해 만든 뒤
다음 tick 전에 simulation sync를 호출한다. sync 오류는 `CombatRuntimeError::Simulation`으로
전달한다. `finish`/execution fingerprint 조립은 S2a와 동일하게 유지한다.

### WP-3 — regression tests

runtime unit test에 다음 fixture를 추가한다.

1. ally의 lethal attack가 tick 1에 enemy health를 0으로 만든다.
2. tick 1 frame에는 lethal attack outcome이 있다.
3. tick 2 frame에는 enemy move intent/target이 없고, enemy가 occupancy를 막지 않으며,
   ally의 target selection도 inactive enemy를 선택하지 않는다.
4. 두 공격자가 같은 tick에 서로를 KO해도 tick 1 양쪽 outcome은 유지되고, tick 2에는 양쪽
   move/attack가 없다. attack definition 입력 순서를 뒤집어도 결과가 같다.

existing simulation/resolution/cadence tests는 기대값을 임의로 고치지 않는다.

## 5. 소유 파일

수정 허용:

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/src/combat_runtime.rs`
- `crates/escape-core/src/lib.rs`는 module 등록이 이미 되어 있으면 수정하지 않는다.
- `combat_runtime.rs` `#[cfg(test)]` 내부 테스트

수정 금지:

- `combat_resolution.rs`, `combat_execution.rs` (S2a 계약 동결)
- `combat_opportunity.rs`, `combat_conclusion.rs`, `combat_spectator.rs`
- `combat_hex.rs`, `scene_page.rs`, terminal/Web, save/serde/generated artifacts

## 6. acceptance criteria

- 마지막 hit tick의 resolution outcomes/frame은 그대로 남는다.
- health `<=0` participant는 다음 tick move/target/occupancy에서 제외되고, roster id/state는
  삭제되지 않는다.
- 동시 KO는 attack definition/input 순서와 무관하며 같은 tick 양쪽 outcome을 보존한다.
- active overlay가 static manifest/input fingerprint와 presentation speed/seed를 바꾸지 않는다.
- runtime errors는 Result로 전달되고 panic/implicit participant 생성이 없다.
- `cargo fmt --all -- --check`, runtime/simulation/resolution/cadence targeted tests,
  workspace test, `git diff --check` 통과.

## 7. 범위 밖·정지 조건

- opportunity marker, 결착 우선순위, selection history/segment seed, save/renderer는 S2c 이후.
- active roster sync 때문에 public schema/version bump가 필요하면 정지.
- inactive frame shape와 spectator contract가 충돌하면 정지.
- 기존 fixed fingerprint/value 변화 원인을 설명할 수 없으면 기대값을 수정하지 말고 정지.

## 8. 보고 형식

- baseline/final commit와 전체 테스트 수치
- sync method 최종 서명과 active filtering 위치
- lethal tick/next tick 관측값 및 동시 KO 순서 invariance 결과
- static input/manifest fingerprint 변화 여부
- S2c pause marker가 소비할 tick/health/active roster 경계

## 9. 구현 결과 (2026-08-08)

- 구현 커밋: `270f178`
- `sync_active_from_health(&BTreeMap<String, i64>)`를 추가하고, simulation의 move gauge,
  move intent, contention occupancy, occupancy snapshot을 active participant로 제한했다.
- runtime은 매 resolution frame 직후 health map을 sync한다. lethal tick의 frame/outcome은
  유지되고, 다음 tick에는 KO participant의 move/target이 생성되지 않는다.
- 직접 검증: runtime unit 3/3, simulation integration 14/14, workspace 0 failures,
  `cargo fmt --all -- --check`, `git diff --check` 통과.
- static input/manifest fingerprint·public schema는 변경하지 않았다. S2c는 이 runtime의
  tick/health/active overlay 경계를 소비한다.
