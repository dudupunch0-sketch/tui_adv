# T4 S4a — response application contract gate

status: design-gate
date: 2026-08-08
baseline_commit: `b15f7fb`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/design/Combat_Hex_Rework_Development_Plan.md` §T4(d)~(e)
3. `docs/dev/Implementation_Slice_Discipline.md`
4. `fable_combat_hex_t4_slice_plan_2608081232.md`
5. `fable_combat_hex_t4_step11_2608081705.md`

코드 구현은 설계 승인 뒤 `/caveman lite`를 적용한 `gpt-5.6-luna` medium subagent에 한정
위임한다. 구현자는 WSL clone만 사용하며, main 오케스트레이터가 직접 diff·테스트를 재검증한다.

## 2. 현재 코드와 정본의 차이

- `CombatOpportunityCandidate`/`CombatResponseOption`은 response id, executor id,
  success/failure effect id 목록만 제공한다. 성공·실패를 결정하는 입력/판정과 target id는
  없다.
- `CombatResponseDefinition`에는 `executor_selector`/`target_selector`/`resolution_kind`가
  있지만, runtime이 이를 실제 전투원·효과 대상에 연결하는 규칙은 아직 없다.
- `CombatRuntime::resume_with_response`는 response id를 선택 이력·segment seed에만 넣고,
  `CombatResolutionStepper`의 `active_effects`, roles, policies를 변경하지 않는다.
- 현재 runtime은 response 선택의 성공을 임의로 `true` 또는 `no-op`로 간주할 수 없다. 이는
  정본의 “실패도 no-op가 아닌 새 상태·결과”와 데이터 기반 밸런스 원칙을 위반한다.

## 3. 이번 설계 게이트의 목표

실제 효과 적용 전에 다음 세 가지를 정본 계약으로 확정한다.

1. response outcome: `success`/`failure`를 누가, 어떤 결정론적 입력으로 산출하는가.
2. target resolution: executor/target selector가 어떤 stable id를 선택하는가.
3. history provenance: outcome·executor·target을 선택 이력과 checkpoint에 어떻게 포함해
   같은 선택 이력의 resume fingerprint를 보장하는가.

## 4. 승인 후 구현할 최소 slice

승인된 계약만을 대상으로 다음 파일을 소유한다.

- `crates/escape-core/src/combat_runtime.rs`
- `crates/escape-core/src/combat_resolution.rs`
- 해당 두 모듈의 targeted tests

예상 API(이름은 승인 후 최종 확정):

- 내부 `CombatRuntimeIntervention` 입력: response id, 명시된 outcome, executor id,
  target id, effect ids 및 provenance.
- `CombatResolutionStepper::apply_intervention(...)`: 기존 effect catalog/stacking/log
  경로를 재사용해 재개 직전에 정확히 한 번 적용.
- runtime resume가 intervention을 canonical history에 기록하고 다음 segment seed를
  파생한 뒤 다음 tick에서 계속 실행.

새 RNG source, 임의 성공률·피해·회복 수치, renderer 판단, public SaveEnvelope/schema 변경은
범위 밖이다. roles/policies 교체는 target/outcome 계약과 별도 S4b slice로 분리한다.

## 5. 승인 후 acceptance

- 같은 intervention 입력은 effect instance/log/state/fingerprint가 동일하다.
- success와 failure는 서로 다른 명시적 상태를 만들며 failure를 no-op로 삼지 않는다.
- executor/target id가 없거나 active roster와 불일치하면 `InvalidInput`이고 panic이 없다.
- pause → intervention → checkpoint JSON → restore → finish가 uninterrupted 실행과 parity를
  갖는다.
- 같은 canonical history는 같은 다음 segment seed를 만들고, outcome/target을 바꾸면 seed와
  후속 결과가 달라진다.
- 기존 434/0 계열 회귀, full workspace, fmt/diff 검사를 통과한다.

## 6. 정지 조건

- 성공·실패 산출 규칙 또는 target selector tie-break가 정본에서 확정되지 않은 상태.
- 기존 `CombatEffectInstance`의 target/lifetime/stacking semantics를 재사용할 수 없는 상태.
- response 선택을 기존 `resume_with_response(response_id)`에 임의 기본값으로 끼워 넣어야 하는
  상태.

이 문서는 위 계약이 승인되기 전에는 코드 구현을 시작하지 않는 설계 게이트다. 계약 승인 뒤
S4a 구현 plan으로 승격하고, 그때 한 subagent가 수행할 파일·테스트 단위로 다시 고정한다.
