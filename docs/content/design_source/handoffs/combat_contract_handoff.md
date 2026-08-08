# Combat contract handoff — WP-L18 + intervention canonical sync

상태: canonical design / runtime handoff_required

## 종료 조건

GameCore primitive owner가 사실을 수집하고, authoring objective mapping이 결과를 매핑한다. 우선순위(낮은 숫자가 먼저)는 forced_stop > captured > surrendered > fled > objective_completed > both_sides_defeated > one_side_defeated > max_ticks다. 같은 tick에 같은 priority가 둘 이상이면 validator error다. mutual defeat primitive은 보존한다. 알 수 없는 fact/result, terminal objective 누락은 거부한다.

## simulation version

현재 코드 capability에서 관찰된 실제 값은 `v3`이다. code registry가 지원 버전을 소유하고, authoring은 required version을 선언하며 validator가 exact compatibility를 거부/허용한다. unknown, missing, unsupported version은 fallback하지 않는다. selector/formula registry의 `v1`은 독립된 의미 버전이며 simulation version과 맞춰 올리지 않는다.

## 전투 개입 응답

코드 담당자는 다음 순서로 읽는다.

1. `contracts/intervention.yml`: 승인된 machine-readable 정본
2. `schema/combat_intervention.schema.json`: response/contract 구조
3. 이 문서의 구현 WP와 acceptance criteria
4. `docs/design/Combat_Intervention_Response_Model_Handoff.md`: 역사적 제안과 current runtime gap
5. `crates/escape-core/src/combat_opportunity.rs`, `combat_runtime.rs`, `save.rs`: 구현 touchpoint

Response는 optional `strategy_modifier`와 optional `special_effect`의 composite이며 둘 중 하나 이상이 필요하다. effect-only/strategy-only/both는 payload 존재로 파생한다. `intervention_kind`, `resolution_kind`, hybrid enum을 추가하지 않는다. 복합형의 strategy는 special effect 성공·실패와 무관하게 항상 적용한다.

Special effect의 success/failure는 `set_flag`, `create_loot_entitlement`, `grant_item` typed action plan을 만든다. Combat core가 plan을 계산하고 GameCore가 `action_id`와 `application_transaction_id`로 exactly-once 원자 적용한다. Loot entitlement는 기본적으로 victory/objective에서 claim 가능하고 escape/defeat/surrender/capture/forced_stop/both-sides-defeated에서는 미획득이다. 직접 지급은 같은 response transaction에서 즉시 획득한다.

Selector/formula는 namespaced canonical ID만 runtime authoring에 허용한다. `self`, `target`, `observer`, `opponent`, `any`는 migration 입력에서만 정규화하며 runtime에서는 거부한다. Response selector는 pause snapshot에서 한 번 resolve하고 stable combatant ID 오름차순으로 tie-break한다. Strategy targeting rule은 매 tick canonical snapshot에서 재평가한다.

Strategy는 immutable baseline 위 typed overlay다. Scope는 all_allies/role/combatants, 기본 duration은 until_replaced이며 next_segment도 허용한다. 같은 scope+field는 최신 patch가 교체하고 서로 다른 필드는 공존한다. precedence는 combatant > role > side > baseline이다. additive numeric stacking과 임의 JSON patch는 금지하고 `clear_override`는 baseline을 복원한다.

Lifecycle은 running/paused_for_intervention/terminal이며 terminal reason/result는 별도다. Pause는 termination 후보가 아니다. Stable pause ID와 evaluation fingerprint가 일치하지 않는 stale response는 거부하고 nested pause는 금지한다. Host timeout은 명시적 no_intervention 입력이다. forced_stop은 pause를 무효화하고 terminal priority 1위로 처리한다. KO tick은 결과 반영 → 잠정 terminal facts → 개입 감지 → 개입 적용 → facts 재계산 → 결착 순서다.

저장은 resolved decision receipt + compact state snapshot이다. Save/checkpoint schema는 v2, receipt schema는 v1이다. Checkpoint는 pre/post stable state만 허용하며 partial apply를 직렬화하지 않는다. Intent/decision/checkpoint fingerprint를 분리하고 decision receipt fingerprint를 확정한 뒤 다음 segment seed를 파생한다. Legacy v1 selection은 `legacy_no_effect`로 보존하고 새 효과를 소급 적용하지 않는다.

## 표시 이름

신규 authoring은 canonical character identity registry를 요구한다. legacy는 migration warning으로 통과시킬 수 있다. 표시 순서는 encounter alias → canonical name → 선언된 generic role label → unknown combatant이며 internal ID를 사용자 문장이나 접근성 label에 출력하지 않는다.

## 로그

GameCore는 raw event를 tick/sequence 순서로 보존한다. renderer만 presentation aggregation을 수행한다. 현재 runtime event type(move_intent, target_selection, collision, damage_applied, effect_applied, hidden effect)을 기준으로, move intent는 동일 tick/template family/actor/target 그룹만 합치고 damage는 값을 합산하되 hit count와 sequence 범위를 보존한다. terminal/status/objective/effect hidden은 합치지 않는다. debug raw view는 허용한다.

## 전술 구역

author가 normalized 0..1 bounds와 purpose/accessibility label을 선언하고 GameCore가 범위·overlap·coverage를 검증한다. 기본 depth는 0, .33, .66, 1, lane은 0, .5, 1; encounter-local grid projection은 선택 사항이며 raw integer 좌표는 debug-only다.

## 코드 touchpoint candidates

- crates/escape-core/src/combat_conclusion.rs: termination reason/outcome primitive
- crates/escape-core/src/combat_contract.rs: simulation version/manifest validation
- crates/escape-core/src/combat_spectator.rs: raw log event and renderer-neutral view
- crates/escape-core/src/content.rs: encounter combat authoring boundary
- crates/escape-core/src/combat_opportunity.rs: response schema, selector/formula registry references
- crates/escape-core/src/combat_runtime.rs: lifecycle, atomic response transaction, receipt/checkpoint
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

This slice does not modify Rust/TS/Web/runtime/generated files, add combat balance or weapon numbers, implement AI, create entity registry, or declare runtime completion. 정본 02 자동전투/상황 트리거, 05 무기 세부, 08 전투 예시는 후속 read order이며 이 PR의 구현 완료 항목이 아니다.
