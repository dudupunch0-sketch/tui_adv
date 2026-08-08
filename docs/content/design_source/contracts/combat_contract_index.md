# Combat contract index

status: canonical
effective_date: 2026-08-02
runtime_status: handoff_required
owner: GameCore/runtime handoff + design validator
source_basis: docs/content/design_source/reports/combat_contract_gap_audit.md + approved intervention decisions

이 디렉터리는 전투 계약의 설계 정본이다. Rust GameCore가 이미 제공하는 사실과 아직 구현하지 않은 handoff 계약을 분리한다. 이 문서와 6개 YAML은 runtime API를 대체하지 않으며, 구현 완료를 선언하지 않는다.

## 결정 요약

| 계약 | 정본 파일 | owner | 현재 상태 |
|---|---|---|---|
| 종료 조건 | termination.yml | GameCore primitive / authoring objective mapping | handoff_required |
| simulation version | simulation_version.yml | code registry / author requirement / validator | handoff_required |
| 표시 이름 | identity.yml | canonical character identity registry | handoff_required |
| 로그 | logs.yml | immutable raw events / renderer aggregation | handoff_required |
| 전술 구역 | tactical_zones.yml | author semantics / GameCore validation / renderer labels | handoff_required |
| 전투 개입 응답 | intervention.yml | Combat core plan / GameCore atomic apply / renderer display | handoff_required |

## 근거와 범위

정본 01/02/03/05/06/08/09/13은 audit에 기록된 local design-source arc/event records와 docs/dev/Data_Schema.md, docs/dev/Combat_System_Operating_Guide.md를 근거로 한다. 02 자동전투·상황 트리거, 05 무기 세부, 08 전투 예시는 독립 normalized record로 아직 승격되지 않았다. 이 계약 작업에서는 구현 완료로 표시하지 않고 후속 read order로 남긴다.

현재 runtime 사실은 crates/escape-core/src/combat_conclusion.rs, combat_contract.rs, combat_runtime.rs, combat_spectator.rs와 해당 wave/T4 테스트에 있다. 코드 capability의 현재 simulation version은 `v3`이며 design validator는 Rust 상수를 읽어 계약의 current observed 값과 대조한다. selector/formula registry의 `v1`은 독립된 의미 버전이며 simulation `v3`과 같은 축이 아니다.

`intervention.yml`은 composite response, typed outcome action, versioned selector/formula registry, typed strategy overlay, pause/terminal lifecycle, resolved decision receipt와 checkpoint v2를 정본화한다. 현재 Rust의 response 적용은 이 계약을 아직 구현하지 않았으므로 `runtime_status: handoff_required`를 유지한다.

검증 명령:

~~~bash
python tools/story_design/validate_combat_contracts.py --root docs/content/design_source
pytest -q tools/story_design/tests/test_validate_combat_contracts.py
~~~

Development Plan의 WP-L18 handoff 항목은 이 index와 handoffs/combat_contract_handoff.md를 함께 참조한다.
