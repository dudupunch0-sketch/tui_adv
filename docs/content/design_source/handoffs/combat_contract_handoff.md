# Combat contract handoff — WP-L18

상태: canonical design / runtime handoff_required

## 종료 조건

GameCore primitive owner가 사실을 수집하고, authoring objective mapping이 결과를 매핑한다. 우선순위(낮은 숫자가 먼저)는 forced_stop > captured > surrendered > fled > objective_completed > both_sides_defeated > one_side_defeated > max_ticks다. 같은 tick에 같은 priority가 둘 이상이면 validator error다. mutual defeat primitive은 보존한다. 알 수 없는 fact/result, terminal objective 누락은 거부한다.

## simulation version

현재 코드와 테스트에서 관찰된 실제 값은 v1이다. code registry가 지원 버전을 소유하고, authoring은 required version을 선언하며 validator가 exact compatibility를 거부/허용한다. unknown, missing, unsupported version은 fallback하지 않는다.

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
- crates/escape-core/tests/combat_conclusion_wave2.rs, combat_execution_wave2.rs, combat_spectator_wave3.rs: acceptance fixture locations

## Acceptance tests

- termination: each priority, simultaneous objective+defeat, tie, unknown result, mutual defeat preservation
- version: v1 pass, missing/unknown/unsupported fail, provenance includes resolved version, same input deterministic
- identity: alias/canonical/generic/unknown fallback, internal ID leak rejected, duplicate labels rejected
- logs: five move intents aggregate to one presentation item while raw five remain, three damage events sum and preserve count, tick/actor/target boundary splits, terminal/status/objective remain separate
- zones: normalized bounds pass, resize preserves meaning, gap/overlap fail, author override respects invariants, accessibility output has no raw coordinates

## Non-goals

This slice does not modify Rust/TS/Web/runtime/generated files, add combat balance or weapon numbers, implement AI, create entity registry, or declare runtime completion. 정본 02 자동전투/상황 트리거, 05 무기 세부, 08 전투 예시는 후속 read order이며 이 PR의 구현 완료 항목이 아니다.
