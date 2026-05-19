# 인카운터 목록

## 목적

이 문서는 1차 수직 슬라이스와 1차 확장 콘텐츠 팩에 들어갈 인카운터 초안을 정의한다.
각 인카운터는 나중에 `encounters.yaml`로 옮기기 쉽도록 id, 위치, 핵심 선택, 주요 결과를 가진다.

## 목록 요약

| id | 이름 | 주요 위치 | 핵심 역할 |
|---|---|---|---|
| ex_employee_messenger | 퇴사자의 메신저 | 내 자리 | 진실 루트 시작 |
| printer_prints_alone | 복합기가 혼자 출력한다 | 복합기 구역 | 현실 연결 시작 |
| meeting_room_booking | 회의실 예약 패널 | 회의실 | 코스믹/진실 단서 |
| pantry_coffee_machine | 탕비실 커피머신 | 탕비실 | 회복과 괴현상 |
| supply_closet_cache | 물품창고 비상 보급함 | 물품창고 | 보급품 획득 |
| strange_water_dispenser | 정수기의 이상한 물 | 탕비실 | 갈증 회복 위험 |
| cold_server_door | 서버실 앞의 차가운 바람 | 서버실 앞 | 정보/정복 관문 |
| stairwell_footsteps | 비상계단의 발소리 | 비상계단 | 탈출 위험 |
| elevator_nonexistent_floor | 존재하지 않는 층의 엘리베이터 | 엘리베이터 홀 | 옥상 루트 진입 |
| office_broadcast | 사내 방송 | 복도/사무실 | 위험도/회사 괴담 |
| minutes_with_my_name | 회의록에 적힌 내 이름 | 회의실 | 정신력/진실 단서 |
| fridge_note | 냉장고 안의 쪽지 | 탕비실 | 현실 연결 후속 |
| flashlight_under_desk | 책상 아래 손전등 | 개발팀 사무실 | 도구 획득 |
| camera_watches | 보안 카메라의 시선 | 복도 | 정복/보안 떡밥 |
| rooftop_signal | 옥상의 제한된 외부 신호 | 옥상 | 외부 신호 탈출 |
| parking_ignition | 지하주차장의 시동음 | 비상계단/복도 | 확장 탈출 떡밥 |

## 상세 초안

### ex_employee_messenger: 퇴사자의 메신저

위치:

- dev_desk

설명 요약:

퇴사한 전임자에게서 사내 메신저가 온다.
외부 인터넷은 끊겼지만 사내망 알림은 도착한다.

선택지:

1. 메시지를 확인한다
   - 비용: 배터리 -3, 정신력 -2
   - 결과: 단서 `ex_employee_contacted`

2. 무시하고 휴대폰을 엎어둔다
   - 결과: 정신력 +2, 단서 놓침

3. 전임자의 이름을 사내망에서 검색한다
   - 조건: 배터리 >= 8
   - 결과: 퇴사자의 메모 또는 진실 루트 플래그

4. [인터페이스] 알림 지연 시간을 역추적한다
   - 조건: interface >= 4, 배터리 >= 2
   - 판정: 2d6 + interface >= 10
   - 성공: 단서 `delayed_packet_route`, 플래그 `network_truth_hint`
   - 실패: 정신력 피해, 위험도 증가

### printer_prints_alone: 복합기가 혼자 출력한다

위치:

- printer_area

설명 요약:

꺼져 있던 복합기가 출력물을 뱉는다.
출력물에는 플레이어가 아직 하지 않은 선택이 적혀 있다.

선택지:

1. 출력물을 읽는다
   - 비용: 정신력 -3
   - 결과: 단서 획득

2. 출력물을 챙긴다
   - 결과: 아이템 `crumpled_printout`, 플래그 `printer_secret_started`

3. 토너 카트리지를 확인한다
   - 조건: 정신력 >= 40
   - 결과: 현실 연결 힌트 1단계

### meeting_room_booking: 회의실 예약 패널

위치:

- meeting_room

설명 요약:

예약 패널에 “전 직원 참석” 회의가 표시된다.
주최 부서는 존재하지 않는다.

선택지:

1. 회의에 참석한다
   - 비용: 정신력 -10
   - 결과: 진실 단서, 위험도 +1

2. 예약 내용을 촬영한다
   - 비용: 배터리 -5
   - 결과: 단서 `impossible_meeting_saved`

3. 회의실을 나간다
   - 결과: 안전하지만 단서 없음

### pantry_coffee_machine: 탕비실 커피머신

위치:

- pantry

설명 요약:

커피머신 화면에 “물을 보충하십시오”가 반복된다.
물통은 가득 차 있다.

선택지:

1. 커피를 뽑는다
   - 결과: 아이템 또는 즉시 효과 `coffee`

2. 물통을 확인한다
   - 결과: 갈증 관련 단서 또는 정신력 -2

3. 커피머신 뒤를 본다
   - 조건: `printer_secret_started`
   - 결과: 현실 연결 힌트 후속

### supply_closet_cache: 물품창고 비상 보급함

위치:

- supply_closet

설명 요약:

물품창고 비상 보급함에 구급상자, 보조배터리, 과자 봉지가 남아 있다.
소모품 사용 루프를 자연스럽게 경험시키는 안전한 보급 인카운터다.

선택지:

1. 구급상자를 챙긴다
   - 결과: 아이템 `first_aid_kit`

2. 보조배터리를 챙긴다
   - 결과: 아이템 `power_bank`

3. 비상 간식 봉지를 챙긴다
   - 결과: 아이템 `snack`

### strange_water_dispenser: 정수기의 이상한 물

위치:

- pantry

설명 요약:

정수기에서 물소리가 나지만 컵에는 물이 차지 않는다.
갈증이 높을수록 유혹적으로 보인다.

조건:

- 위치: pantry
- 갈증 60 이상
- 플래그 `thirst_hallucination_seen` 없음

선택지:

1. 물을 마신다
   - 결과: 갈증 -25, 정신력 -8, 플래그 `thirst_hallucination_seen`

2. 생수병을 찾는다
   - 비용: 정신력 -2
   - 결과: 아이템 `bottled_water`, 플래그 `thirst_hallucination_seen`

3. 정수기 전원을 뽑는다
   - 결과: 위험도 -1, 단서 `water_dispenser_false_sound`, 플래그 `thirst_hallucination_seen`

### cold_server_door: 서버실 앞의 차가운 바람

위치:

- server_room_front

설명 요약:

문틈에서 냉기가 흘러나오고, 안쪽에서 키보드 소리가 들린다.

선택지:

1. 사원증을 찍는다
   - 조건: `employee_badge`
   - 결과: 접근 실패 로그 또는 플래그 `server_access_attempted`

2. 패널을 촬영한다
   - 비용: 배터리 -4
   - 결과: 단서 `server_panel_symbols`

3. 문에서 물러난다
   - 결과: 안전

### stairwell_footsteps: 비상계단의 발소리

위치:

- emergency_stairs

설명 요약:

아래층에서 위로 올라오는 발소리가 들린다.
층수 표시는 변하지 않는다.

선택지:

1. 아래로 내려간다
   - 조건: 배터리 또는 손전등이 있으면 유리
   - 결과: 탈출 루트 진행 또는 체력/정신력 피해

2. 문을 닫고 막는다
   - 결과: 위험도 -1, 턴 경과

3. 발소리를 녹음한다
   - 비용: 배터리 -4
   - 결과: 진실 단서

### elevator_nonexistent_floor: 존재하지 않는 층의 엘리베이터

위치:

- elevator_hall

설명 요약:

엘리베이터 버튼 패널에 없던 R층이 켜져 있다.
외부 인터넷 아이콘은 한 칸만 떠 있고, 버튼을 누르면 옥상 루트로 이어진다.

선택지:

1. 존재하지 않는 R층 버튼을 누른다
   - 비용: 배터리 -4, 정신력 -3
   - 결과: 위치 `rooftop`, 플래그 `rooftop_accessed`, 단서 `nonexistent_rooftop_button`

2. 문틈을 벌려 현재 층으로 돌아온다
   - 결과: 보안실로 이동, 체력 -4, 단서 `security_floor_misalignment`, 플래그 `elevator_returned_wrong_floor`

### office_broadcast: 사내 방송

위치:

- hallway
- dev_office

설명 요약:

아무도 없는 사무실에 정상 근무 안내 방송이 나온다.

선택지:

1. 방송을 끝까지 듣는다
   - 비용: 정신력 -5
   - 결과: 단서 또는 위험도 +1

2. 스피커를 찾는다
   - 결과: 위치/보안 단서

3. 귀를 막고 지나간다
   - 결과: 정신력 피해 감소

### minutes_with_my_name: 회의록에 적힌 내 이름

위치:

- meeting_room

설명 요약:

회의록에는 플레이어가 참석했고 발언했다는 기록이 있다.
아직 회의에 들어간 적이 없다.

선택지:

1. 회의록을 읽는다
   - 비용: 정신력 -8
   - 결과: 진실 단서

2. 사진을 찍는다
   - 비용: 배터리 -3
   - 결과: 단서 보존

3. 회의록을 찢는다
   - 결과: 위험도 +1, 정신력 +2 가능

### fridge_note: 냉장고 안의 쪽지

위치:

- pantry

설명 요약:

냉장고 안쪽에 얼어붙은 쪽지가 있다.

선택지:

1. 쪽지를 꺼낸다
   - 결과: 단서 또는 갈증 -/정신력 변화

2. 냉장고 문을 닫는다
   - 결과: 안전

3. 출력물과 대조한다
   - 조건: `crumpled_printout`
   - 결과: 현실 연결 힌트 2단계

### flashlight_under_desk: 책상 아래 손전등

위치:

- dev_office

설명 요약:

책상 아래에 손전등이 굴러와 있다.
방금 누군가 떨어뜨린 것 같다.

선택지:

1. 손전등을 줍는다
   - 결과: 아이템 `flashlight`

2. 책상 아래를 더 본다
   - 비용: 정신력 -2
   - 결과: 추가 단서 또는 위험 로그

### camera_watches: 보안 카메라의 시선

위치:

- hallway
- dev_office

설명 요약:

보안 카메라가 플레이어를 따라 움직인다.
녹화등은 꺼져 있다.

선택지:

1. 카메라를 향해 손을 흔든다
   - 결과: 사내망 메시지 트리거 가능

2. 카메라를 피한다
   - 결과: 위험도 감소 가능

3. 카메라 위치를 기록한다
   - 비용: 배터리 -2
   - 결과: 정복 루트 단서

### rooftop_signal: 옥상의 제한된 외부 신호

위치:

- rooftop

조건:

- 플래그 `rooftop_accessed`

설명 요약:

옥상 난간 너머로 도시의 불빛이 보이고, 휴대폰은 외부 인터넷을 한 번만 보낼 수 있을 만큼의 신호를 붙잡는다.
제한된 신호를 보내면 옥상 외부 신호 탈출 엔딩으로 이어진다.

선택지:

1. 제한된 외부 신호를 짧게 송신한다
   - 비용: 배터리 -12, 정신력 -5
   - 결과: 플래그 `rooftop_signal_sent`, 단서 `outside_signal_ack`

2. 어긋난 도시 야경을 녹화한다
   - 비용: 배터리 -5
   - 결과: 단서 `wrong_skyline_recording`, 정신력 피해

### parking_ignition: 지하주차장의 시동음

위치:

- emergency_stairs
- hallway

설명 요약:

아래쪽 어딘가에서 자동차 시동음이 들린다.
건물 안에는 아무도 없어야 한다.

선택지:

1. 소리를 따라간다
   - 결과: 위험도 +1, 확장 루트 단서

2. 돌아간다
   - 결과: 안전
