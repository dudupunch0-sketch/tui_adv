# Combat contract index

status: canonical
effective_date: 2026-08-02
runtime_status: handoff_required
owner: GameCore/runtime handoff + design validator
source_basis: docs/content/design_source/reports/combat_contract_gap_audit.md

이 디렉터리는 전투 계약의 설계 정본이다. Rust GameCore가 이미 제공하는 사실과 아직 구현하지 않은 handoff 계약을 분리한다. 이 문서와 5개 YAML은 runtime API를 대체하지 않으며, 구현 완료를 선언하지 않는다.

## 결정 요약

| 계약 | 정본 파일 | owner | 현재 상태 |
|---|---|---|---|
| 종료 조건 | termination.yml | GameCore primitive / authoring objective mapping | handoff_required |
| simulation version | simulation_version.yml | code registry / author requirement / validator | handoff_required |
| 표시 이름 | identity.yml | canonical character identity registry | handoff_required |
| 로그 | logs.yml | immutable raw events / renderer aggregation | handoff_required |
| 전술 구역 | tactical_zones.yml | author semantics / GameCore validation / renderer labels | handoff_required |

## 근거와 범위

정본 01/02/03/05/06/08/09/13은 audit에 기록된 local design-source arc/event records와 docs/dev/Data_Schema.md, docs/dev/Combat_System_Operating_Guide.md를 근거로 한다. 02 자동전투·상황 트리거, 05 무기 세부, 08 전투 예시는 독립 normalized record로 아직 승격되지 않았다. 이 계약 작업에서는 구현 완료로 표시하지 않고 후속 read order로 남긴다.

현재 runtime 사실은 crates/escape-core/src/combat_conclusion.rs, combat_contract.rs, combat_spectator.rs와 해당 wave 테스트에 있다. 현재 코드가 사용하는 실제 version은 v1이지만 code-supported registry는 아직 없으므로 새 authoring은 validator가 v1만 허용하며 runtime registry 구현을 후속 handoff로 남긴다.

검증 명령:

~~~bash
python tools/story_design/validate_combat_contracts.py --root docs/content/design_source
pytest -q tools/story_design/tests/test_validate_combat_contracts.py
~~~

Development Plan의 WP-L18 handoff 항목은 이 index와 handoffs/combat_contract_handoff.md를 함께 참조한다.
