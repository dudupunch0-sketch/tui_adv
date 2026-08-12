# Combat contract handoff — WP-L18 + intervention canonical sync

상태: canonical design / runtime handoff_required

## 종료 조건

GameCore primitive owner가 사실을 수집하고, authoring objective mapping이 결과를 매핑한다. 우선순위(낮은 숫자가 먼저)는 forced_stop > captured > surrendered > fled > objective_completed > both_sides_defeated > one_side_defeated > max_ticks다. 같은 tick에 같은 priority가 둘 이상이면 validator error다. mutual defeat primitive은 보존한다. 알 수 없는 fact/result, terminal objective 누락은 거부한다.

## simulation version

현재 코드 capability에서 관찰된 실제 값은 `v3`이다. code registry가 지원 버전을 소유하고, authoring은 required version을 선언하며 validator가 exact compatibility를 거부/허용한다. unknown, missing, unsupported version은 fallback하지 않는다. selector/formula registry의 `v1`은 독립된 의미 버전이며 simulation version과 맞춰 올리지 않는다.

## 전투 개입 응답

코드 담당자는 다음 순서로 읽는다.

1. `contracts/intervention.yml`: 승인된 machine-readable 정본
2. `schema/combat_intervention.schema.json`: `intervention.yml` 전체 문서 검증
3. `schema/combat_intervention_response.schema.json`: 개별 authoring response payload 검증
4. 이 문서의 구현 WP와 acceptance criteria
5. `docs/design/Combat_Intervention_Response_Model_Handoff.md`: 역사적 제안과 current runtime gap
6. `crates/escape-core/src/combat_opportunity.rs`, `combat_runtime.rs`, `save.rs`: 구현 touchpoint

Response는 optional `strategy_modifier`와 optional `special_effect`의 composite이며 둘 중 하나 이상이 필요하다. effect-only/strategy-only/both는 payload 존재로 파생한다. `intervention_kind`, `resolution_kind`, hybrid enum을 추가하지 않는다. 복합형의 strategy는 special effect 성공·실패와 무관하게 항상 적용한다.

Special effect의 success/failure는 `set_flag`, `create_loot_entitlement`, `grant_item` typed action plan을 만든다. Combat core가 deterministic `action_id`/`entitlement_id`를 만들고 GameCore가 response application transaction에서 exactly-once 적용한다. Retry는 receipt/checkpoint ledger의 ID가 있으면 `already_applied` 또는 기존 entitlement를 반환하며 mutation을 반복하지 않는다. Terminal loot claim은 별도 claim action/transaction ID와 terminal receipt를 사용한다. Loot entitlement는 기본적으로 victory/objective에서 claim 가능하고 escape/defeat/surrender/capture/forced_stop/both-sides-defeated에서는 미획득이다. 직접 지급은 response transaction에서 즉시 획득한다.

Selector/formula는 namespaced canonical ID만 runtime authoring에 허용한다. `resolution_kind`, `self`, `target`, `observer`, `opponent`, `any`는 offline/load-time migration 입력에서만 처리하며 runtime에서는 거부한다. 문맥으로 1:1 증명되지 않는 `choice`/`target`/`opponent`/`any`는 자동 변환하지 않고 `designer_review_required`로 보낸다. Response selector는 pause snapshot에서 한 번 resolve하고 stable combatant ID 오름차순으로 tie-break한다. Strategy targeting rule은 매 tick canonical snapshot에서 재평가한다.

현재 runtime에는 pause preflight의 canonical selector/formula 해석과 formula receipt 및 decision receipt 구조가 이미 구현되어 있다. 남은 구현 gap은 receipt 이전의 resolved inputs를 실제 state에 반영하는 atomic application, lifecycle/terminal integration, selected-target provenance 정책 delta, outcome-action source-selector resolve/provenance다.

### WP-I2b 승인 정본 — formula/selector/receipt 경계

이 절은 v1의 의미를 바꾸는 것이 아니라 WP-I2b 후속 canonical semantics를 고정한다. `combat.formula.v1.fixed_chance`는 `chance_percent` 정수 하나만 허용하며 0..=100 밖, 누락, unknown key, wrong type은 preflight 오류다. roll은 0..99 정수이고 `roll < chance_percent`일 때만 성공한다. 0/100은 항상 실패/성공이며 draw와 roll은 없지만 fingerprint와 sub-seed는 계산한다. 그 외 확률은 정확히 1회 draw를 사용한다.

확률 판정의 canonical tuple은 `[combat.formula.v1.fixed_chance, actual_combat, effective_segment_seed, simulation_version, manifest_fingerprint, segment_index, pause_tick, pause_id, evaluation_fingerprint, authored_response_id, formula_id, normalized_formula_parameters, resolved_executor_id, canonical_ordered_target_ids]`다. UTF-8 compact canonical JSON array, BTreeMap lexicographic object keys, decimal integer encoding을 사용한다. 현재 v3 hash는 FNV-1a 64이며 input fingerprint는 lowercase 16-hex FNV 결과, sub-seed는 같은 64-bit 값, draw 0 roll은 `sub_seed % 100`이다. 알고리즘 변경은 simulation version bump가 필요하다. wall clock/frame/renderer/forecast는 금지하고 draw index는 0이다.

Response selector는 pause snapshot에서 1회만 해석한다. executor `observer`는 exact opportunity observer이고 can_act·required capabilities·hp>0인 active combatant 하나여야 하며, `any_capable`의 각 후보도 같은 조건을 만족하는 exactly one active combatant with hp>0로 매핑되어야 한다. `any_capable`은 같은 조건의 pause observers를 stable ID 오름차순으로 골라야 한다. 조건 불충족/zero candidate는 preflight 실패다. vector/insertion order는 금지한다. `selected_target`은 정확히 하나의 immutable bound target만 사용한다. pause에서 active였거나 triggering tick 시작에 active였다가 같은 tick에 KO가 된 경우만 허용하고 previously-KO/fled/departed/captured는 거부한다. `departed`는 fled가 아닌 비도주 이탈이다. bound ID, bound tick, tick 시작 상태, trigger tick provenance를 보존하며 atomic transaction 안에서 later state로 재평가하지 않는다. lowest health는 max_hp>0 invariant를 먼저 검사하고 integer cross-multiplication으로 비교한다. surrounded는 anchor six-neighborhood의 distinct occupied hexes를 세고 전체 후보 footprint를 ally count에서 제외하며 active footprints만 사용한다. all active allies의 resolved IDs는 stable ID 오름차순, authored unique effect_ids 순서는 semantic이며 effect-major, then target stable ascending으로 exactly-once 적용한다. success/failure 양 branch compatibility, action source selector resolution, registry membership은 RNG 전에 preflight한다.

Effect-target compatibility는 pause snapshot에 결박된 `CombatEffectCatalog.effects`를 source registry로 사용한다. success/failure 양 branch의 모든 authored `effect_id`를 조회하고 unknown ID는 preflight error다. 각 `CombatEffectDefinition.target_selector`는 canonical target selector ID여야 하며, response의 `special_effect.target_selector_id`와 exact match할 때만 compatible하다. 하나라도 mismatch면 RNG 전에 preflight error이며 state/cost/RNG/history는 모두 0이다. static authoring validator는 catalog 입력이 없으면 조합을 추측하지 않고, 이 runtime preflight를 mandatory machine contract로 검증한다.

Preflight 실패는 mutation/cost/RNG/history와 receipt를 0으로 유지하고 pause를 보존한다. 판정 실패는 정상적인 resolved outcome이며 failure branch만 적용하고, composite response의 strategy는 성공·실패와 무관하게 적용한다. `special_effect`가 present이면 `formula_receipt`가 required이고, receipt는 `formula_id`, `normalized_formula_parameters`, `input_fingerprint`, `rng_namespace`, nullable `rng_draw_index`, nullable `roll`, `outcome`을 보존한다. deterministic 0/100도 namespace는 actual_combat이지만 draw index와 roll은 null이다. strategy-only response에는 formula receipt 자체가 없다.

Strategy는 immutable baseline 위 typed overlay다. Authoring scope는 combatants/role/all_allies이며 `all_allies`는 executor-side `all_allies_side` overlay로 해석한다. unsupported `side` authoring scope는 거부한다. 기본 duration은 until_replaced이며 next_segment도 허용한다. 같은 scope+field는 최신 patch가 교체하고 서로 다른 필드는 공존한다. precedence는 combatant > role > all_allies_side > baseline이다. additive numeric stacking과 임의 JSON patch는 금지하고 `clear_override`는 baseline을 복원한다.

Lifecycle은 running/paused_for_intervention/terminal이며 terminal reason/result는 별도다. Pause는 termination 후보가 아니다. Stable pause ID와 evaluation fingerprint가 일치하지 않는 stale response는 거부하고 nested pause는 금지한다. Host timeout은 명시적 no_intervention 입력이다. forced_stop은 pause를 무효화하고 terminal priority 1위로 처리한다. KO tick은 결과 반영 → 잠정 terminal facts → 개입 감지 → 개입 적용 → facts 재계산 → 결착 순서다.

저장은 resolved decision receipt + `GameState.combat_intervention_ledger`와 optional active-session `SaveEnvelope.combat_checkpoint`로 분리한다. I7b의 SaveEnvelope target은 v2, checkpoint target은 별도 v2, receipt schema는 v1이다. Checkpoint는 paused session restart를 위해 serialize할 수 있지만 terminal/forced stop 후 폐기하며 transaction scratch/partial working copy/renderer state는 직렬화하지 않는다. Intent/decision/checkpoint fingerprint를 분리하고 candidate swap 시 transaction cache와 deterministic next-segment seed를 함께 확정한다. Legacy v1 selection은 `legacy_no_effect`로 보존하고 새 효과를 소급 적용하지 않는다.

### WP-I2b-S1 승인 정본 — occurrence target provenance와 identity mapping

`selected_target`의 유일한 canonical source는 opportunity occurrence다. Public `CombatOpportunityInstance`는 occurrence lifetime 동안 불변인 `bound_target_ids`와 `bound_target_tick`을 제공하고, `CombatOpportunityCandidate`는 두 값을 exact copy하며, pause도 candidate를 통해 같은 값을 보존한다. ID는 surrounding whitespace를 제거하거나 별칭 변환하지 않는다. 빈 문자열/whitespace-only ID와 duplicate는 reject하고 silent dedupe하지 않으며, 유효한 unique ID는 stable combatant ID 오름차순으로 한 번 canonicalize한다. Candidate evaluation fingerprint와 pause evaluation fingerprint에는 canonical ordered `bound_target_ids`와 `bound_target_tick`이 모두 들어가므로, 입력 vector 순서는 fingerprint를 바꾸지 않지만 occurrence target이나 bind tick 교체는 fingerprint를 바꾼다.

`combat.selector.target.v1.selected_target`은 `bound_target_ids` cardinality가 정확히 1일 때만 그 exact ID를 resolve한다. 0개, 2개 이상, unknown combatant, provenance 누락은 RNG 전에 preflight reject이며 pause/state/cost/RNG/history는 0 변화다. AI current target, most recent move/attack target, insertion order, current enemy heuristic는 source도 fallback도 아니다. 다른 다섯 target selector는 `bound_target_ids`를 union/filter/fallback에 사용하지 않고 pause snapshot만으로 기존 canonical 규칙에 따라 resolve한다. 따라서 다른 selector는 occurrence bound list가 비어 있거나 여러 개인 것만으로 실패하지 않지만, candidate/pause fingerprint에는 그 occurrence provenance를 그대로 보존한다.

Observer identity와 combatant identity는 분리한다. Public `CombatObserver`에 explicit `combatant_id`를 추가한다. `observer.id`는 detection/opportunity provenance ID이고 executor/target ID가 아니며, 두 문자열이 우연히 같아도 equality를 mapping으로 사용하지 않는다. Opportunity context validation은 observer ID와 mapped combatant ID의 blank, unknown combatant, duplicate observer ID, 둘 이상의 observer가 같은 combatant로 매핑되는 경우를 reject한다. `observer` executor는 exact opportunity observer의 mapped combatant다. `any_capable`은 observer의 `can_act`와 required capabilities, mapped combatant의 pause-active와 hp>0을 모두 만족하는 후보만 남기고 mapped combatant ID 오름차순으로 exactly one을 고른다. Capability는 observer record, active/health는 combat runtime snapshot이 owner다. 새 entity registry나 authoring identity 의미는 만들지 않는다.

`CombatRuntime`은 각 trigger tick을 resolve하기 **직전**의 combatant health/active snapshot을 캡처하고 그 tick 뒤 opportunity를 평가할 때 사용한다. Snapshot은 stable combatant ID 오름차순 record이며 각 record는 `combatant_id`, `current_health_hundredths`, `maximum_health_hundredths`, `participant_active`를 보존한다. Runtime이 `trigger_tick`, stable `pause_id`, 이 snapshot을 소유하고 occurrence가 `bound_target_tick`/`bound_target_ids`를 소유한다. `selected_target`은 pause snapshot에서 active+hp>0이거나, trigger-tick 시작 snapshot에서 participant_active+hp>0였고 같은 tick 결과로 hp=0이 된 exact bound combatant만 허용한다. Trigger tick 시작부터 hp=0/inactive였거나 unknown인 대상은 prior-KO로 reject한다. fled/departed/captured lifecycle가 도입되면 active로 간주하지 않는 기존 lifecycle 계약을 그대로 따른다.

이 provenance가 들어가는 순간 `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION`은 2다. Paused checkpoint v2는 `pause_id`, `trigger_tick`, `bound_target_ids`, `bound_target_tick`, trigger-tick 시작 snapshot, evaluation fingerprint를 모두 요구하며 malformed/blank/duplicate/cardinality-inconsistent provenance를 restore에서 reject한다. Provenance가 없는 paused checkpoint v1은 추론하거나 current state로 보충하지 않고 explicit reject한다. 이 checkpoint bump는 active combat session 내부 버전이며 `SaveEnvelope`는 계속 v1이다. `SaveEnvelope` v2와 durable ledger 추가는 I7b 전에는 금지한다.

## 표시 이름

신규 authoring은 canonical character identity registry를 요구한다. legacy는 migration warning으로 통과시킬 수 있다. 표시 순서는 encounter alias → canonical name → 선언된 generic role label → unknown combatant이며 internal ID를 사용자 문장이나 접근성 label에 출력하지 않는다.

## 로그

GameCore는 raw event를 tick/sequence 순서로 보존한다. decision receipt는 raw event를 복제하지 않고 sequence range/fingerprint만 참조한다. Strategy/effect/action/entitlement/claim은 `logs.yml`의 각 intervention raw event type으로 남기며 모두 non-groupable이다. Receipt에는 canonical entity/zone ID 또는 selector provenance만 저장하고 display label은 identity/tactical contract를 따라 render-time resolve한다. 재현에 불필요한 hidden state 전체 저장과 internal ID 사용자 노출은 금지한다. Renderer만 presentation aggregation을 수행하며 debug raw view는 허용한다.

## 전술 구역

author가 normalized 0..1 bounds와 purpose/accessibility label을 선언하고 GameCore가 범위·overlap·coverage를 검증한다. 기본 depth는 0, .33, .66, 1, lane은 0, .5, 1; encounter-local grid projection은 선택 사항이며 raw integer 좌표는 debug-only다.

## 코드 touchpoint candidates

- crates/escape-core/src/combat_conclusion.rs: termination reason/outcome primitive
- crates/escape-core/src/combat_contract.rs: simulation version/manifest validation
- crates/escape-core/src/combat_spectator.rs: raw log event and renderer-neutral view
- crates/escape-core/src/content.rs: encounter combat authoring boundary
- crates/escape-core/src/combat_opportunity.rs: response schema, selector/formula registry references
- crates/escape-core/src/combat_runtime.rs: lifecycle, response planning/preflight, receipt/checkpoint orchestration
- crates/escape-core/src/combat_intervention_transaction.rs: new_required_module; candidate GameState atomic commit/swap and transaction retry cache
- crates/escape-core/src/save.rs: save envelope v2 migration and active combat checkpoint
- crates/escape-core/tests/combat_conclusion_wave2.rs, combat_execution_wave2.rs, combat_spectator_wave3.rs: acceptance fixture locations

## Intervention implementation work packages

1. **WP-I1 schema/registry**: composite payload와 namespaced selector/formula registry를 추가하고 legacy alias/unknown ID를 index-time 거부한다.
2. **WP-I2 atomic resolution**: pause snapshot에서 executor/target/formula input을 먼저 확정하고 preflight 실패 시 state/cost/RNG/history를 0 변화로 유지한다. strategy-only와 deterministic effect는 RNG 0회, probabilistic effect는 actual-combat deterministic sub-seed에서 정확히 1회 소비한다.
3. **WP-I3 strategy/outcome application**: typed strategy overlay와 typed outcome action plan을 구현하고 GameCore transaction으로 exactly-once 적용한다. Delayed/multi-tick workflow는 만들지 않는다.
4. **WP-I4 lifecycle/termination**: running/paused/terminal과 stable pause ID를 구현하고 KO tick intervention-before-settlement, stale response, forced-stop override를 검증한다.
5. **WP-I5 receipt/checkpoint/save**: decision receipt + compact snapshot, 세 fingerprint, save/checkpoint v2 migration, legacy_no_effect 변환을 구현한다.
6. **WP-I6 consumers**: ScenePage/WASM/terminal/Web은 core marker와 결과만 표시하며 성공·결착·timeout을 재판정하지 않는다.

## Acceptance tests

- termination: each priority, simultaneous objective+defeat, tie, unknown result, mutual defeat preservation
- version: code capability `v3` pass, missing/unknown/unsupported fail, provenance includes resolved version, same input deterministic
- identity: alias/canonical/generic/unknown fallback, internal ID leak rejected, duplicate labels rejected
- logs: five move intents aggregate to one presentation item while raw five remain, three damage events sum and preserve count, tick/actor/target boundary splits, terminal/status/objective remain separate
- zones: normalized bounds pass, resize preserves meaning, gap/overlap fail, author override respects invariants, accessibility output has no raw coordinates
- response: effect-only/strategy-only/both pass, empty payload와 explicit hybrid/resolution kind fail
- registry: canonical v1 selector/formula pass, legacy alias와 unknown ID fail, stable target tie-break
- transaction: validation failure 0 mutation, composite failure에서도 strategy 적용, RNG 0/0/1회, partial checkpoint 금지
- actions: flag/loot/direct grant typed validation, action receipt exactly-once, 기본 terminal claim policy
- strategy: baseline immutable, same key latest replacement, disjoint coexist, scope precedence, next-segment expiry, clear override
- lifecycle: pause restore parity, stale/nested pause rejection, forced-stop override, KO intervention 후 terminal facts 재평가
- receipt/save: canonical resolved IDs/roll/outcome/action receipts, three fingerprints, v1 legacy_no_effect migration, uninterrupted parity

## Non-goals

## 현재 runtime drift (runtime은 이 handoff에서 변경하지 않음)

현재 mismatch는 네 가지다: opportunity instance/candidate에 bound target source가 없고, observer→combatant explicit mapping 없이 ID equality를 암묵 가정하며, `any_capable`이 observer input order에 의존하고, 새 explicit RNG formula semantic/domain tags가 없다. Current target/last move에서 selected target을 복원하는 fallback은 이 gap의 해법이 아니며 금지한다.

미구현은 atomic application(다중 target effect application 포함), lifecycle/terminal integration, selected-target provenance 정책 delta, outcome-action source-selector resolve/provenance다. 현재 runtime effect definitions의 target_selector는 legacy/noncanonical일 수 있어 이 canonical compatibility로 수렴하는 구현 delta가 남아 있다. 이 문서는 runtime merge/completion을 주장하지 않는다.

This slice does not modify Rust/TS/Web/runtime/generated files, add combat balance or weapon numbers, implement AI, create entity registry, or declare runtime completion. 정본 02 자동전투/상황 트리거, 05 무기 세부, 08 전투 예시는 후속 read order이며 이 PR의 구현 완료 항목이 아니다.

## WP-I7 transaction contract — implementation handoff

이 절은 WP-I7의 착수 질문을 모두 닫은 canonical contract다. response/preflight planning은 `crates/escape-core/src/combat_runtime.rs`가 소유하고, 새 `crates/escape-core/src/combat_intervention_transaction.rs`(`new_required_module`)가 candidate `GameState`의 atomic commit/swap을 소유한다. `crates/escape-core/src/lib.rs`는 public export/entry facade이며 atomic owner가 아니다. `save.rs`는 현재 `SaveEnvelope`/`SAVE_SCHEMA_VERSION = 1`의 load-save 경계다. 다음 DTO 이름은 모두 `new required type`이다: `CombatInterventionResponseInput`, `CombatInterventionResponsePlan`, `CombatInterventionCommitResult`, `CombatLootClaimInput`, `CombatLootClaimResult`, `CombatLootEntitlement`.

`CombatInterventionResponsePlan`은 fully resolved DTO다. 필드는 정확히 `response_application_transaction_id`, `pause_id`, `evaluation_fingerprint`, `precondition_game_state_fingerprint`, `resolved_executor_id`, `resolved_target_ids`, `resolved_outcome`, `strategy_overlay_plan`, `effect_application_plan`, `outcome_action_plan`, `formula_receipt`, `decision_receipt_draft`, `deterministic_next_segment_seed`, `provenance`다. `decision_receipt_draft`는 plan input-only이며 candidate에서 최종 `decision_receipt`를 만드는 데만 쓰고 `GameState.combat_intervention_ledger`나 `SaveEnvelope.combat_checkpoint`에 절대 저장하지 않는다. GameCore transaction은 이 plan을 소비만 하며 selector/formula/branch/target을 재평가하지 않는다. precondition fingerprint가 stale이면 response result `rejected`, candidate swap/ledger/history/cost는 0, pause retained다.

`set_flag`는 authored action order로 한 번 append하는 idempotent set, `grant_item`은 전역 item dedupe 없이 action ID별 exact-once direct grant, `create_loot_entitlement`는 inventory mutation 없는 unique-ID entitlement 생성이다. Combat core는 typed plan과 deterministic ID를 만들고 transaction module은 clone/candidate 전체 적용 후 성공 시에만 swap한다. terminal loot claim은 별도의 claim action/application transaction이다. response result는 `applied|already_applied|rejected`, action receipt는 `applied|already_applied|pending_claim`, claim result는 `applied|already_applied|rejected`다.

Preflight 성공 직후 receipt는 active session context에서 GameState mutation 전에 만든다. SaveEnvelope의 optional `combat_checkpoint`/`CombatRuntimeCheckpoint`는 paused session restart를 위해 serialize할 수 있지만 terminal/forced stop 뒤 폐기하며 GameState ledger로 승격하지 않는다. transaction scratch, partial working copy, renderer state만 절대 serialize하지 않는다. preflight/selector/formula/branch compatibility 또는 candidate operation 실패는 original state/ledger/history/cost unchanged, pause retained다. candidate에는 최종 `decision_receipt`, action receipts, transaction result cache, deterministic seed만 넣고 swap한 뒤 next-segment seed는 cache의 값을 그대로 retry에 반환한다.

I7b에서 `GameState.combat_intervention_ledger`를 추가할 때만 `SaveEnvelope` schema를 v1에서 v2로 올린다. ledger는 transaction result cache, applied action IDs, entitlement map, claim receipt map, applied claim IDs를 단일 field로 가진다. v1 load는 빈 `CombatInterventionLedger`로 default한다. checkpoint schema v1→v2는 별도 gate이며 I2b/I7a에서 checkpoint 필드를 바꾸면 `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION`도 v2가 된다. provenance 없는 v1 paused checkpoint는 explicit reject한다.

### 별도 work package와 acceptance

1. **I2b-S1a — occurrence/identity carrier**: 선행 조건은 PR #219의 I2 prerequisite authoring contract merge다. Owned files는 `crates/escape-core/src/combat_opportunity.rs`, `crates/escape-core/tests/combat_opportunity_wave1.rs`, 그리고 mandatory field 추가로 깨지는 struct literal만 고치는 `crates/escape-core/src/combat_runtime.rs`의 compile fixture/test 영역이다. `CombatOpportunityInstance.bound_target_ids`/`bound_target_tick`, candidate exact copy/fingerprint, `CombatObserver.combatant_id`, stable mapped-combatant selection을 구현한다. Acceptance는 blank/duplicate/unknown mapping reject, bound vector order-independent canonical fingerprint, bound value/tick change fingerprint delta, observer vector reversal parity, capability/can_act filtering이다. Stop은 content authoring schema 변경, participant/entity registry 생성, runtime checkpoint/lifecycle/transaction mutation이 필요할 때다. Forbidden files는 `save.rs`, `combat_intervention_transaction.rs`, TS/Web/generated/storypack, `docs/content/design_source/schema/combat_intervention_response.schema.json`이다.
2. **I2b-S1b — pause provenance/checkpoint v2**: I2b-S1a merge와 독립 review가 선행 조건이다. Owned file은 `crates/escape-core/src/combat_runtime.rs`와 그 module-local/focused runtime tests다. Tick resolution 직전 snapshot, stable pause ID, selected-target exact-one preflight, same-tick KO boundary, paused checkpoint v2 roundtrip/restore validation을 구현한다. Acceptance는 no source·multi source·unknown·prior-KO reject, same-tick lethal accept, AI current target/last move fallback 부재, malformed provenance reject, v2 JSON roundtrip, provenance-less paused v1 explicit reject, preflight 실패 전후 state/cost/RNG/history equality다. Stop은 GameState mutation, terminal/lifecycle settlement, response transaction, `SaveEnvelope` bump, authoring schema 변경이 필요할 때다. Forbidden files는 `save.rs`, `combat_intervention_transaction.rs`, `lib.rs` public facade, TS/Web/generated/storypack, 모든 authoring schema다.
3. **I2b-S2 — formula tuple/receipt**: I2b-S1b review 뒤 PR #219의 S2 범위를 수행한다. Owned files는 `crates/escape-core/src/combat_intervention.rs`, `combat_runtime.rs`, focused tests이며 canonical tuple/0·100·1..99 규칙만 다룬다. Save/GameState durable schema와 새 RNG source는 금지한다.
4. **I2b-S3 — target compatibility/source provenance**: I2b-S2 review 뒤 PR #219의 S3 범위를 수행한다. Owned files는 `crates/escape-core/src/combat_intervention.rs`, `combat_runtime.rs`, focused tests이며 여섯 selector와 both-branch compatibility/source provenance만 다룬다. Renderer/ScenePage/save/GameState mutation과 I7a transaction은 금지한다.
5. **I7a — transaction adapter**: 새 transaction module이 `CombatInterventionResponsePlan`을 candidate에 적용하고 GameCore entry facade를 통해 atomic swap한다. acceptance는 preflight zero mutation, action collision rollback, same item distinct actions, transaction retry equal result, rejected zero commit이다.
6. **I7b — persistence/claim ledger**: v2 SaveEnvelope, empty-ledger v1 load, restart exactly-once, entitlement survival, separate terminal claim transaction을 구현한다. acceptance는 claim denial retry, v1 checkpoint policy, entitlement collision rollback, claim idempotency다.
7. **I7c — lifecycle/terminal E2E**: KO tick의 `apply → provisional facts → detect → intervene → recompute → settle`, stale/nested pause, forced-stop invalidation, terminal claim policy를 연결한다. acceptance는 same-tick intervention-before-settlement, stale response rejection, nested pause rejection, terminal claim result, uninterrupted/restart parity다.

구현 순서는 **I2b-S1a → I2b-S1b → I2b-S2 → I2b-S3 → I7a → I7b → I7c**이며 각 slice는 merge와 독립 review 뒤 다음 slice로 간다. 바로 다음 action은 PR #219 merge 후 one-Luna 범위의 I2b-S1a다. 이 design slice에서는 Rust/TS/Web 파일을 수정하지 않는다.
