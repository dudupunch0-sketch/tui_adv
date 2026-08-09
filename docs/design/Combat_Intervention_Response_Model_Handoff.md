# 전투 개입 응답 모델 구현 인계서

status: canonical-synced / runtime-handoff-required
date: 2026-08-08
canonical_contract: `docs/content/design_source/contracts/intervention.yml`
canonical_schema: `docs/content/design_source/schema/combat_intervention.schema.json`
authoring_payload_schema: `docs/content/design_source/schema/combat_intervention_response.schema.json`
implementation_handoff: `docs/content/design_source/handoffs/combat_contract_handoff.md`
related_plan: `fable_combat_hex_t4_step12_2608081745.md`

## 1. 문서 상태와 목적

이 문서는 처음에는 T4 response application의 미결정을 정본 담당자에게 전달하는 proposal이었다. 해당 결정은 이제 local design source의 `intervention.yml`에 승인·정본화됐다. 이 문서는 canonical contract를 대체하지 않으며, 현재 Rust 구현과 승인 계약 사이의 gap 및 구현 read order를 설명한다.

현재 runtime은 pause preflight에서 canonical selector/formula를 해석하고 formula receipt 및 decision receipt 구조를 이미 생성·보존한다. `CombatRuntime::resume_with_response(response_id)`의 남은 gap은 resolved inputs를 실제 state에 반영하는 atomic application, lifecycle/terminal integration, selected-target provenance 정책 delta, outcome-action source-selector resolve/provenance다. 따라서 정본 상태는 canonical design이지만 runtime은 `handoff_required`다.

## 2. 확정된 response 모델

Response는 두 optional payload를 가진 composite다.

```yaml
strategy_modifier: <optional typed overlay>
special_effect: <optional formula + success/failure payload>
```

- 둘 중 하나 이상이 반드시 존재한다.
- effect-only/strategy-only/both는 payload 존재 여부로 파생한다.
- 별도 `intervention_kind`, `resolution_kind`, hybrid/composite kind를 추가하지 않는다.
- 복합형에서 strategy modifier는 special effect 성공·실패와 무관하게 항상 적용한다.
- 전체 transaction은 같은 pause snapshot에서 시작해 다음 tick 전에 원자적으로 끝난다.
- tick을 넘는 delayed workflow나 multi-tick response는 현재 범위에서 금지한다.

처리 순서는 다음과 같다.

```text
response 제출
→ pause id/evaluation/response/cost/registry 참조 preflight
→ mutation 전에 executor/target/scope/formula input 확정
→ probabilistic special effect라면 actual-combat sub-seed로 정확히 1회 판정
→ strategy patch와 success/failure effect·outcome action plan 작성
→ GameCore 원자 commit
→ resolved decision receipt 기록
→ decision receipt fingerprint로 다음 segment seed 파생
```

Preflight 오류는 state, cost, RNG, history를 전혀 바꾸지 않고 pause를 유지한다. Strategy-only와 deterministic special effect는 RNG를 소비하지 않는다.

## 3. Selector와 formula registry

Runtime authoring은 namespaced versioned canonical ID만 허용한다.

- executor 예: `combat.selector.executor.v1.observer`
- target 예: `combat.selector.target.v1.executor_self`
- formula 예: `combat.formula.v1.fixed_chance`
- strategy targeting 예: `combat.strategy.targeting.v1.attackers_of`

`self`, `target`, `observer`, `opponent`, `any` 같은 legacy 문자열은 migration 입력에서만 canonical ID로 변환한다. Runtime canonical authoring에서는 unknown/legacy ID를 hard error로 거부한다.

Response selector는 pause snapshot에서 한 번 resolve하고 stable combatant ID 오름차순으로 tie-break한다. Strategy의 dynamic targeting rule은 매 tick canonical snapshot에서 재평가하며, 무효 대상은 canonical fallback, 그마저 없으면 baseline targeting으로 돌아간다.

Formula ID와 selector ID의 v1은 의미 registry version이다. 현재 runtime simulation version `v3`과 독립된 축이다. 동일 canonical ID의 의미를 제자리 수정하지 않는다. 전투 전체 판정 순서·RNG·좌표 의미가 바뀔 때만 simulation version을 올린다.

### WP-I2b 승인 정본 보충

이 보충은 v1의 의미를 변경하지 않고 WP-I2b 후속 canonical semantics를 완성한다. `combat.formula.v1.fixed_chance`는 authored `chance_percent` 정수 하나만 받으며 0..=100만 유효하다. unknown/missing/type/range 오류는 preflight 실패이고, 보정·clamp·round는 없다. roll은 0..99이며 `roll < chance_percent`가 성공이다. 0/100은 항상 실패/성공으로 draw와 roll은 없지만 fingerprint/sub-seed는 계산한다. 1..99는 정확히 1회 draw를 사용한다. 새 formula 추가 시 schema도 갱신한다.

Canonical tuple은 `[combat.formula.v1.fixed_chance, actual_combat, effective_segment_seed, simulation_version, manifest_fingerprint, segment_index, pause_tick, pause_id, evaluation_fingerprint, authored_response_id, formula_id, normalized_formula_parameters, resolved_executor_id, canonical_ordered_target_ids]`다. UTF-8 compact canonical JSON array, BTreeMap lexicographic object keys, decimal integer encoding을 사용한다. 현재 v3 hash는 FNV-1a 64이며 input fingerprint는 lowercase 16-hex FNV 결과, sub-seed는 같은 64-bit 값, draw 0 roll은 `sub_seed % 100`이다. 알고리즘 변경은 simulation version bump가 필요하다. wall clock/frame/renderer/forecast는 금지하며 draw index는 0이다. Receipt는 formula receipt가 필요한 경우에만 `formula_id`, `normalized_formula_parameters`, `input_fingerprint`, `rng_namespace`, nullable `rng_draw_index`, nullable `roll`, `outcome`을 저장하고, strategy-only에는 formula receipt 자체가 없다.

Selectors는 pause snapshot에서 1회 resolve한다. exact `observer`는 opportunity observer이며 can_act·required capabilities·hp>0인 active combatant 하나로 매핑되어야 하고, `any_capable`의 각 후보도 같은 조건을 만족하는 exactly one active combatant with hp>0로 매핑되어야 한다. `any_capable`은 pause-snapshot observers 중 같은 조건을 만족하는 전원을 stable combatant ID 오름차순으로 정렬해 첫 항목을 고른다. zero/불일치면 preflight다. vector/insertion order는 사용하지 않는다. selected_target은 정확히 하나의 immutable bound target만 받으며 pause에서 active였거나 triggering tick 시작에 active였다가 같은 tick에 KO가 된 경우만 허용한다. previously-KO/fled/departed/captured는 거부하고, `departed`는 fled가 아닌 비도주 이탈이다. bound ID·bound tick·tick 시작 상태·trigger tick provenance를 보존하며 atomic transaction 안에서 later state로 재평가하지 않는다. lowest health는 active same-side/self 포함 전원의 max_hp>0을 먼저 검증하고 integer cross-multiplication으로 비교하며 tie는 stable ID다. surrounded는 candidate anchor six-neighborhood와 교차하는 distinct occupied hexes를 세고 전체 candidate footprint를 ally count에서 제외하며 active footprints만 사용한다. all active allies의 target IDs는 stable ID 오름차순이다. authored unique effect_ids order가 semantic이고 각 effect마다 각 target을 effect-major, then target stable ascending으로 exactly once 적용한다. success/failure 양 branch compatibility, action source selector resolution/provenance, registry membership은 RNG 전에 preflight한다.

Effect-target compatibility는 pause snapshot에 결박된 `CombatEffectCatalog.effects`를 source registry로 사용한다. success/failure 양 branch의 모든 authored `effect_id`를 조회하고 unknown ID는 preflight error다. 각 `CombatEffectDefinition.target_selector`는 canonical target selector ID여야 하며, response의 `special_effect.target_selector_id`와 exact match할 때만 compatible하다. 하나라도 mismatch면 RNG 전에 preflight error이며 state/cost/RNG/history는 모두 0이다. static authoring validator는 catalog 입력이 없으면 조합을 추측하지 않고, 이 runtime preflight를 mandatory machine contract로 검증한다.

Preflight 실패는 mutation/cost/RNG/history/receipt 없이 pause를 유지한다. resolved failure는 정상 outcome이며 failure branch만 적용하고 composite strategy는 결과와 무관하게 적용한다. `special_effect`가 present이면 `formula_receipt`가 required이고, strategy-only에는 formula receipt 자체가 없다. 이 결정은 runtime 구현을 완료로 표시하지 않으며 다음 runtime handoff에서 소비한다.

## 4. Strategy modifier

전투 시작 전 역할·정책은 immutable baseline이다. 개입은 baseline을 직접 고치지 않고 typed overlay를 적용한다.

- scope: `all_allies`, `role`, `combatants`; `all_allies` resolves to the executor-side `all_allies_side` overlay; unsupported authoring scope `side` is rejected
- operation: `set_role_weight`, `set_targeting_rule`, `set_target_policy`, `clear_override`
- duration: 기본 `until_replaced`, 선택적으로 `next_segment`
- 같은 scope+field: 최신 patch가 교체
- 서로 다른 field: 공존
- precedence: combatant > role > all_allies_side > baseline
- 전투 종료: overlay 전부 폐기
- additive numeric stacking과 임의 JSON patch: 금지
- `clear_override`: 해당 key를 baseline으로 복원

Special effect가 실패해도 같은 response의 strategy patch는 적용한다.

## 5. Special effect와 typed outcome action

Special effect는 canonical formula, parameters, executor/target selector와 success/failure branch를 가진다. 각 branch는 combat effect IDs와 다음 typed action을 가질 수 있다.

- `set_flag`: run-level event flag 설정
- `create_loot_entitlement`: 전리품 획득 권리 생성
- `grant_item`: 즉시 아이템 지급

Combat core는 action plan과 deterministic `action_id`/`entitlement_id`를 만들고 GameCore가 response application transaction으로 exactly-once 적용한다. Terminal-time loot claim은 별도 claim action/transaction과 terminal receipt를 사용하며 retry는 ledger에 존재하는 ID를 근거로 `already_applied` 처리한다. 직접 지급은 response transaction에서 즉시 획득한다. Loot entitlement는 기본적으로 승리·목표 달성에서 claim 가능하고 도주·패배·항복·포획·forced stop·상호 전멸에서는 미획득이다. 명시적 authoring override는 validator를 통과한 경우만 허용한다.

도감·기억 파편은 별도 account-meta 보상 계약이며 현재 runtime에 있다고 가정하지 않는다. Action receipt는 향후 최초 획득 consumer가 사용할 acquisition provenance만 보존한다.

## 6. Lifecycle과 termination

Lifecycle은 `running | paused_for_intervention | terminal`이고 terminal reason/result는 별도다. Pause는 termination priority 후보가 아니다.

- pause는 stable `pause_id`, tick, segment, evaluation fingerprint, response 목록을 가진다.
- response의 pause ID/evaluation/state revision이 현재 pause와 다르면 stale response로 거부한다.
- nested pause는 금지한다.
- 사용자 미응답은 pause 유지, host timeout은 명시적 `no_intervention` 입력이다.
- `forced_stop`은 pause를 무효화하고 terminal priority 1위로 처리한다.

마지막 전투원 전투불능 tick의 순서는 결과 반영 → 잠정 terminal facts → 개입 감지 → 개입 적용 → facts 재계산 → 결착이다. 이 순서로만 개입형 구조·부활이 결착 전에 성립한다.

## 7. Decision receipt와 checkpoint

저장 모델은 resolved decision receipt + compact state snapshot이다.

Receipt는 canonical selector/formula IDs, resolved executor/targets/scope, formula parameters/input fingerprint/RNG draw/outcome, strategy 전후 fingerprint, effect/action/entitlement와 individual action receipt, lifecycle·terminal facts, raw log sequence reference를 기록한다. Raw event 자체는 immutable ordered stream에 한 번만 저장한다.

Fingerprint는 세 층이다.

1. intent fingerprint: 사용자가 제출한 pause/response 입력
2. decision receipt fingerprint: core가 확정한 selector/formula/strategy/effect/action 결과
3. checkpoint fingerprint: compact state + ordered receipt + raw log cursor

다음 segment seed는 decision receipt fingerprint가 확정된 뒤 파생한다. Checkpoint는 transaction 전·후 stable state만 직렬화하고 partial apply 상태를 저장하지 않는다.

Save schema와 combat checkpoint는 v2로 올리고 receipt schema는 v1로 시작한다. 기존 v1 selection history는 당시 effect application이 없었으므로 `legacy_no_effect`로 보존하고 새 효과를 소급 적용하지 않는다.

## 8. 현재 runtime drift

현재 mismatch는 세 가지다: selected_target의 same-tick KO 및 current-target binding, any_capable의 observer input-order 의존, 새 explicit RNG formula semantic/domain tags 부재다.

미구현은 atomic application(다중 target effect application 포함), lifecycle/terminal integration, selected-target provenance 정책 delta, outcome-action source-selector resolve/provenance다. 현재 runtime effect definitions의 target_selector는 legacy/noncanonical일 수 있어 이 canonical compatibility로 수렴하는 구현 delta가 남아 있다.

## 9. 구현 read order와 acceptance

구현자는 다음 순서로 읽는다.

1. `docs/content/design_source/contracts/intervention.yml`
2. `docs/content/design_source/handoffs/combat_contract_handoff.md`
3. `crates/escape-core/src/combat_opportunity.rs`
4. `crates/escape-core/src/combat_runtime.rs`
5. `crates/escape-core/src/save.rs`

필수 acceptance:

- effect-only/strategy-only/both와 empty payload 거부
- canonical registry 통과, legacy/unknown ID 거부, deterministic tie-break
- validation failure 0 mutation과 pause 유지
- 복합 success/failure 모두 strategy 적용
- RNG 소비 0/0/1회와 restore parity
- same-key strategy 교체, disjoint 공존, scope precedence, clear override
- typed action plan과 exactly-once GameCore transaction
- pause restore, stale/nested pause 거부, forced-stop override
- KO tick intervention 후 terminal facts 재평가
- receipt/checkpoint v2와 legacy_no_effect migration
- consumer가 success/terminal/timeout을 재판정하지 않음

이 acceptance가 모두 runtime 테스트로 고정되기 전에는 구현 완료로 표시하지 않는다.

## WP-I7 결정 완료 handoff

실제 owner는 `combat_runtime.rs`의 response/preflight planning, 새 `combat_intervention_transaction.rs`(`new_required_module`)의 candidate GameState atomic commit/swap으로 나눈다. `lib.rs`는 public export/entry facade이고, `save.rs`는 현재 `SaveEnvelope`와 `SAVE_SCHEMA_VERSION = 1`을 제공한다. 아래 타입은 모두 구현자가 추가해야 하는 `new required type`이다: `CombatInterventionResponseInput`, `CombatInterventionResponsePlan`, `CombatInterventionCommitResult`, `CombatLootClaimInput`, `CombatLootClaimResult`, `CombatLootEntitlement`.

`CombatInterventionResponsePlan`은 fully resolved plan이며 정확한 필드는 `response_application_transaction_id`, `pause_id`, `evaluation_fingerprint`, `precondition_game_state_fingerprint`, `resolved_executor_id`, `resolved_target_ids`, `resolved_outcome`, `strategy_overlay_plan`, `effect_application_plan`, `outcome_action_plan`, `formula_receipt`, `decision_receipt_draft`, `deterministic_next_segment_seed`, `provenance`다. `decision_receipt_draft`는 plan input-only이며 최종 `decision_receipt` 생성에만 사용하고 `GameState.combat_intervention_ledger`/`SaveEnvelope.combat_checkpoint`에 저장하지 않는다. GameCore transaction은 selector/formula/branch/target을 재판정하지 않는다. stale precondition fingerprint는 response `rejected`, zero candidate swap/ledger/history/cost mutation, pause retained다.

Action semantics는 `set_flag`=authored order append-if-absent, `grant_item`=item 전역 dedupe 없는 action-ID exact-once grant, `create_loot_entitlement`=inventory mutation 없는 unique-ID map insert로 확정한다. response commit과 terminal claim은 서로 다른 transaction ID/ledger를 쓴다. response result는 `applied|already_applied|rejected`, action receipt는 `applied|already_applied|pending_claim`, claim result는 `applied|already_applied|rejected`이며 partial apply는 없다. 동일 transaction/action/claim ID retry는 equal result를 반환한다.

Preflight receipt는 성공 직후 GameState mutation 전에 만든다. `SaveEnvelope.combat_checkpoint`/`CombatRuntimeCheckpoint`는 paused session restart를 위해 serialize할 수 있고 terminal/forced stop 뒤 폐기한다. cross-session durable source는 단일 `GameState.combat_intervention_ledger`이며 checkpoint를 ledger로 승격하지 않는다. transaction scratch, partial working copy, renderer state는 절대 serialize하지 않는다. validation/selector/formula/branch compatibility와 candidate 전체 적용이 모두 성공한 뒤 receipt/action receipts/transaction cache/seed를 candidate에 넣고 swap한다.

현재 SaveEnvelope는 v1이다. I7b가 `CombatInterventionLedger`를 GameState에 추가할 때만 v2로 올리고 v1 load는 빈 ledger로 default한다. checkpoint v1→v2는 별도 gate이며 checkpoint 필드 변경 시 `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION`도 v2로 올린다. provenance 없는 v1 paused checkpoint는 explicit reject한다.

착수 순서는 I2b(runtime delta: same-tick KO provenance, stable `any_capable`, RNG semantic/domain tuple, effect target selector/source provenance) → I7a(transaction adapter) → I7b(persistence/claim ledger) → I7c(lifecycle/terminal E2E)다. 각 WP의 input/output/test acceptance는 `contracts/intervention.yml:i7_contract.work_packages`와 `combat_contract_handoff.md`를 따른다.
