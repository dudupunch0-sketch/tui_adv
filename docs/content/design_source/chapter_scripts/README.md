# 객패귀 1막 챕터 대본 companion

이 디렉터리는 기존 event 정본을 수정하지 않고, 48개 메인 슬롯의 장면·선택·cadence를 상세화하는 저작용 companion source다.

## 형식과 canonical 관계

- `script_id`는 `guestpass_act1_chapter_01`처럼 companion만의 고유 ID다.
- `event_ref`는 `manifest.yml`의 event record ID이며, 챕터의 사건 정체성·슬롯·arc를 결정하는 canonical reference다.
- companion은 runtime event가 아니므로 기존 `events/imported/`와 `events/authored/`를 대체하거나 중복 등록하지 않는다.
- 기존 authored 1/08 `wuxia_guestpass_burnt_token_last_route_record.yml`는 원형을 보존한다. 1/08의 cadence 보강은 이 companion에서만 한다.
- 모든 수치·희귀도·실제 runtime 효과는 `deferred`; 모든 레코드는 `authoring_review_required` / `not_implemented`다.

## 필수 필드 계약

각 챕터는 `reentry_recap`, `stable_opening`, `local_urgent_conflict`, `beats_and_dialogue`, `choices`, `relationship_directions`, `rewards`, `quest_clues`, `flags`, `local_closure`, `interlude_safe_state`, `low_pressure_hook`, `next_reentry_trigger`를 둔다. `choices`는 4개 이상이며 각 선택은 성공·실패 결과를 모두 가진다.

`cadence_audit`는 시간·장소·동행·부상·정보·적대 압력의 6축을 기록한다. 장 종료 시점은 일상·잡역·회복·훈련·조사가 끼어들 수 있는 safe state여야 한다.

금지되는 것은 장 밖으로 즉시 이어지는 추격·전투·붕괴·카운트다운, 곧바로 다음 필수 사건을 강제하는 전달, 짧은 시간창 의존이다. terminal ending만 예외다.

검증 기대사항: 선택지별 literal 검증에서는 32개 선택지의 immediate_result, success, failure, relationship_directions, reward_candidate, quest_clue, flags(always/success/failure)를 확인하고, 8개 챕터 모두의 scene_context와 convergence도 확인한다. 이 문서는 generated validation 결과를 선언하지 않는다.
