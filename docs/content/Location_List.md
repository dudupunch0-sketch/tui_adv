# 위치 목록

이 문서는 런타임 기본 위치 데이터 `src/tui_adv/data/locations.yaml`의 공개 문서판이다.
실제 구현 기준 현재 위치 수는 16개다.

## 목록 요약

| id | 이름 | 연결 | 위험도 | 태그 |
|---|---|---|---:|---|
| `dev_desk` | 내 자리 | `dev_office` | 0 | `office`, `start`, `messenger`, `personal` |
| `dev_office` | 개발팀 사무실 | `dev_desk`, `hallway`, `meeting_room`, `printer_area`, `supply_closet` | 0 | `office`, `hub`, `workstations` |
| `hallway` | 복도 | `dev_office`, `server_room_front`, `emergency_stairs`, `security_room`, `elevator_hall`, `parking_lot`, `lobby` | 1 | `transit`, `exposed`, `cctv` |
| `supply_closet` | 물품창고 | `dev_office` | 1 | `survival`, `storage`, `supplies` |
| `pantry` | 탕비실 | `printer_area` | 0 | `survival`, `food`, `drink`, `reality_link` |
| `meeting_room` | 회의실 | `dev_office` | 1 | `meeting`, `sanity_risk`, `truth` |
| `printer_area` | 복합기 구역 | `dev_office`, `pantry` | 0 | `printer`, `clue`, `reality_link` |
| `server_room_front` | 서버실 앞 | `hallway`, `server_room` | 2 | `restricted`, `network`, `truth`, `conquest` |
| `server_room` | 서버실 내부 | `server_room_front` | 3 | `restricted`, `network`, `conquest`, `cold` |
| `emergency_stairs` | 비상계단 | `hallway` | 2 | `escape`, `dark`, `loop`, `danger` |
| `security_room` | 보안실 | `hallway` | 2 | `security`, `surveillance`, `truth` |
| `elevator_hall` | 엘리베이터 홀 | `hallway`, `rooftop` | 2 | `transit`, `elevator`, `anomaly` |
| `rooftop` | 옥상 | `elevator_hall` | 3 | `escape`, `signal`, `open_air` |
| `parking_lot` | 지하주차장 | `hallway` | 3 | `escape`, `parking`, `dark` |
| `lobby` | 로비 | `hallway`, `executive_office` | 2 | `escape`, `reception`, `public_border` |
| `executive_office` | 대표실 | `lobby` | 3 | `conquest`, `approval`, `restricted` |

## 상세

### dev_desk: 내 자리

당신의 모니터는 아직 켜져 있다.

- 위험도: `0`
- 연결: `dev_office`
- 태그: `office`, `start`, `messenger`, `personal`

### dev_office: 개발팀 사무실

개발팀 사무실은 정상적으로 켜져 있지만 사람만 없다.

- 위험도: `0`
- 연결: `dev_desk`, `hallway`, `meeting_room`, `printer_area`, `supply_closet`
- 태그: `office`, `hub`, `workstations`

### hallway: 복도

복도 비상등이 일정하지 않은 간격으로 깜빡인다.

- 위험도: `1`
- 연결: `dev_office`, `server_room_front`, `emergency_stairs`, `security_room`, `elevator_hall`, `parking_lot`, `lobby`
- 태그: `transit`, `exposed`, `cctv`

### supply_closet: 물품창고

물품창고 선반에는 라벨이 붙은 비상 보급품 박스들이 남아 있다.

- 위험도: `1`
- 연결: `dev_office`
- 태그: `survival`, `storage`, `supplies`

### pantry: 탕비실

탕비실에는 커피 냄새가 남아 있다.

- 위험도: `0`
- 연결: `printer_area`
- 태그: `survival`, `food`, `drink`, `reality_link`

### meeting_room: 회의실

회의실 예약 패널에는 방금 생성된 일정이 떠 있다.

- 위험도: `1`
- 연결: `dev_office`
- 태그: `meeting`, `sanity_risk`, `truth`

### printer_area: 복합기 구역

복합기는 절전 모드지만 출력 트레이에는 따뜻한 종이가 있다.

- 위험도: `0`
- 연결: `dev_office`, `pantry`
- 태그: `printer`, `clue`, `reality_link`

### server_room_front: 서버실 앞

닫힌 문틈에서 차가운 바람이 나온다.

- 위험도: `2`
- 연결: `hallway`, `server_room`
- 태그: `restricted`, `network`, `truth`, `conquest`

### server_room: 서버실 내부

랙 LED가 별자리처럼 깜빡이고, 냉기는 일정한 박자로 숨을 쉰다.

- 위험도: `3`
- 연결: `server_room_front`
- 태그: `restricted`, `network`, `conquest`, `cold`

### emergency_stairs: 비상계단

계단 아래에서는 위층에서 들려야 할 발소리가 올라온다.

- 위험도: `2`
- 연결: `hallway`
- 태그: `escape`, `dark`, `loop`, `danger`

### security_room: 보안실

꺼진 CCTV 모니터들이 복도보다 한 박자 늦은 장면을 보여준다.

- 위험도: `2`
- 연결: `hallway`
- 태그: `security`, `surveillance`, `truth`

### elevator_hall: 엘리베이터 홀

엘리베이터 층수 표시가 존재하지 않는 R층과 4층 사이에서 떨린다.

- 위험도: `2`
- 연결: `hallway`, `rooftop`
- 태그: `transit`, `elevator`, `anomaly`

### rooftop: 옥상

옥상 문 너머의 밤공기는 실제 바깥보다 한 프레임 늦게 움직인다.

- 위험도: `3`
- 연결: `elevator_hall`
- 태그: `escape`, `signal`, `open_air`

### parking_lot: 지하주차장

지하주차장의 형광등은 시동음과 다른 박자로 깜빡인다.

- 위험도: `3`
- 연결: `hallway`
- 태그: `escape`, `parking`, `dark`

### lobby: 로비

로비 회전문은 바깥을 보여주지만 같은 장면을 세 번씩 반복한다.

- 위험도: `2`
- 연결: `hallway`, `executive_office`
- 태그: `escape`, `reception`, `public_border`

### executive_office: 대표실

대표실 책상 위 결재판에는 아직 작성하지 않은 당신의 퇴근 승인서가 놓여 있다.

- 위험도: `3`
- 연결: `lobby`
- 태그: `conquest`, `approval`, `restricted`
