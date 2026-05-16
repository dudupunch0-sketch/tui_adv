# 위치 목록

## 목적

이 문서는 1차 수직 슬라이스에서 사용할 실제 위치 목록을 정의한다.
실제 회사 배치를 복제하지 않고, 연구개발동 사무실의 기능적 구역을 게임용으로 추상화한다.

## 위치 id 목록

| id | 이름 | 역할 |
|---|---|---|
| dev_desk | 내 자리 | 시작 지점, 개인 단서 |
| dev_office | 개발팀 사무실 | 허브, 사무실 부재감 |
| hallway | 복도 | 이동 허브, 위험도 상승 |
| pantry | 탕비실 | 허기/갈증 회복, 현실 연결 힌트 |
| meeting_room | 회의실 | 정신력 위험, 진실 단서 |
| printer_area | 복합기 구역 | 현실 연결 루트 시작 |
| server_room_front | 서버실 앞 | 정보/정복/진실 관문 |
| emergency_stairs | 비상계단 | 탈출 루트, 반복 공간 위험 |

## 연결 구조

```text
             [meeting_room]
                   |
[dev_desk] - [dev_office] - [hallway] - [emergency_stairs]
                   |           |
              [printer_area]  [server_room_front]
                   |
                [pantry]
```

## 상세 목록

### dev_desk: 내 자리

설명:

```text
당신의 모니터는 아직 켜져 있다.
커서가 사내 메신저 입력창에서 깜빡인다.
방금 전까지 누군가 옆자리에 있었던 것처럼 공기가 미묘하게 따뜻하다.
```

태그:

- office
- start
- messenger
- personal

연결:

- dev_office

주요 인카운터:

- 퇴사자의 메신저
- 책상 아래 손전등
- 자동 빌드 실패 알림

### dev_office: 개발팀 사무실

설명:

```text
개발팀 사무실은 정상적으로 켜져 있다.
모니터, 키보드, 의자, 커피컵은 모두 사람의 흔적을 남기고 있지만 사람만 없다.
어딘가에서 키보드 치는 소리가 한 번 들린다.
```

태그:

- office
- hub
- workstations

연결:

- dev_desk
- hallway
- meeting_room
- printer_area

주요 인카운터:

- 사내 방송
- 보안 카메라의 시선
- 책상 아래 손전등

### hallway: 복도

설명:

```text
복도 비상등이 일정하지 않은 간격으로 깜빡인다.
출입 게이트 쪽에서는 아무도 찍지 않은 사원증 인식음이 들린다.
```

태그:

- transit
- exposed
- cctv

연결:

- dev_office
- server_room_front
- emergency_stairs

주요 인카운터:

- 보안 카메라의 시선
- 사내 방송
- 비상계단의 발소리

### pantry: 탕비실

설명:

```text
탕비실에는 커피 냄새가 남아 있다.
정수기 표시등은 정상인데, 물 흐르는 소리는 회의실 쪽에서 들린다.
```

태그:

- survival
- food
- drink
- reality_link

연결:

- printer_area

주요 인카운터:

- 탕비실 커피머신
- 정수기의 이상한 물
- 냉장고 안의 쪽지

### meeting_room: 회의실

설명:

```text
회의실 예약 패널에는 현재 시간이 표시되어 있다.
그 아래에는 방금 생성된 일정이 떠 있다.
참석자: 전 직원.
```

태그:

- meeting
- sanity_risk
- truth

연결:

- dev_office

주요 인카운터:

- 회의실 예약 패널
- 회의록에 적힌 내 이름
- 존재하지 않는 부서의 전체회의

### printer_area: 복합기 구역

설명:

```text
복합기는 절전 모드다.
하지만 출력 트레이에는 아직 따뜻한 종이가 한 장 놓여 있다.
```

태그:

- printer
- clue
- reality_link

연결:

- dev_office
- pantry

주요 인카운터:

- 복합기가 혼자 출력한다
- 구겨진 출력물
- 프린트 대기열의 알 수 없는 문서

### server_room_front: 서버실 앞

설명:

```text
문은 닫혀 있다.
서버실인지 전산실인지 장비실인지 알 수 없지만, 문틈에서 차가운 바람이 나온다.
패널은 사원증을 요구한다.
```

태그:

- restricted
- network
- truth
- conquest

연결:

- hallway

주요 인카운터:

- 서버실 앞의 차가운 바람
- 보안 패널의 알 수 없는 안내문
- 사내망 로그 조각

### emergency_stairs: 비상계단

설명:

```text
비상계단 문은 열려 있다.
아래층을 가리키는 표지판은 정상인데, 계단 아래에서는 위층에서 들려야 할 발소리가 올라온다.
```

태그:

- escape
- dark
- loop
- danger

연결:

- hallway

주요 인카운터:

- 비상계단의 발소리
- 같은 층으로 되돌아옴
- 비상구 표시등의 꺼짐

## 확장 후보

2차 이후 추가할 수 있는 위치:

- security_room: 보안실
- server_room: 서버실/전산실 내부
- elevator: 엘리베이터
- rooftop: 옥상
- lobby: 로비
- parking_lot: 지하주차장
- executive_office: 임원실
