# 메인 챕터 간격 cadence contract

## 기본 리듬

메인 챕터와 다음 메인 챕터 사이에는 서브퀘스트 1~2개와 랜덤 인카운터 약 7개가 삽입될 수 있다. 이 간격은 선택 보상·관계 회복·잡역·훈련·조사로 채우며, 다음 메인 사건을 즉시 호출하는 강제 큐가 아니다.

## 장 필수 구조

모든 장은 재진입 recap, 안정적인 opening, 장 안에서 해결 가능한 local urgent conflict, 상세 beats/dialogue, 4개 이상 선택지와 성공/실패 변주, 관계 방향, 보상 이름·종류·컨셉, 퀘스트 단서, 후속 flags, local closure, interlude-safe state, low-pressure hook, next reentry trigger를 가진다.

## 경계 규칙

- 긴박함은 장면 안에서 완결한다. 종료 뒤 즉시 추격·전투·붕괴·카운트다운을 시작하지 않는다.
- 다음 필수 사건은 시간 경과·준비·정보 축적 뒤 재진입한다. 짧은 시간창은 사용하지 않는다.
- 장 종료 상태에는 수면, 식사, 잡역, 회복, 훈련, 조사 중 하나 이상을 삽입할 여지가 있어야 한다.
- 1/08은 이상 징후 기록과 경계 강화·다음 조사 일정만 남긴다. 2/09는 시간 경과 뒤 누적 전조로 재개한다.
- 수치, 희귀도, runtime 효과는 authoring review 전까지 미정이다.

## 6축 감사 체크리스트

각 장의 `cadence_audit`에서 시간(`time`), 장소(`place`), 동행(`companions`), 부상(`injury`), 정보(`information`), 적대 압력(`hostile_pressure`)을 각각 확인한다. 압력은 장 밖으로 자동 전이하지 않고 다음 재진입 조건의 누적 단서로만 남긴다.

## 상태 표기

모든 companion은 `status: authoring_draft`, `review_status: authoring_review_required`, `runtime_status: not_implemented`다. 이는 저작 정본 후보이며 구현 완료나 검수 승인 선언이 아니다.

Literal validation: every choice carries immediate_result, success, failure, relationship_directions, reward_candidate, quest_clue, and flags with always/success/failure; every chapter carries scene_context and convergence.
