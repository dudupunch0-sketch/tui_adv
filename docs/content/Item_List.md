# 아이템 목록

## 목적

이 문서는 1차 수직 슬라이스에서 사용할 아이템 10개를 정의한다.
아이템은 생존, 정보 접근, 루트 해금 중 하나 이상의 역할을 가진다.

## 아이템 목록

| id | 이름 | 유형 | 핵심 효과 |
|---|---|---|---|
| bottled_water | 생수 | consumable | 갈증 감소 |
| coffee | 커피 | consumable | 정신력 소폭 회복, 갈증 소폭 감소 또는 부작용 |
| snack | 과자 | consumable | 허기 감소 |
| cup_noodle | 컵라면 | consumable | 허기 크게 감소, 물 조건 가능 |
| first_aid_kit | 구급상자 | consumable | 체력 회복 |
| power_bank | 보조배터리 | consumable/tool | 배터리 회복 |
| flashlight | 손전등 | tool | 어두운 구역 선택지 해금, 배터리 소모 |
| employee_badge | 사원증 | key | 기본 출입 권한 |
| security_override_badge | 보안실 우회권한 | key | 서버실 우회/보안 정복 루트 |
| parking_key_fob | 지하주차장 키태그 | key | 지하주차장 차단기 탈출 루트 |
| crumpled_printout | 구겨진 출력물 | clue | 현실 연결/진실 루트 시작 |
| ex_employee_note | 퇴사자의 메모 | clue | 진실 루트 단서 |

## 상세 목록

### bottled_water: 생수

유형:

- consumable
- survival
- drink

설명:

```text
아직 밀봉된 생수병.
지금은 복지보다 생존에 가깝다.
```

효과 초안:

- 갈증 -30
- 사용 후 제거

발견 위치:

- 탕비실
- 회의실
- 개발팀 사무실 책상

### coffee: 커피

유형:

- consumable
- survival
- office

설명:

```text
식어가는 커피.
회사에서 가장 흔한 각성제이자 가장 약한 위로.
```

효과 초안:

- 정신력 +5
- 갈증 -5
- 낮은 확률로 불안 로그 또는 정신력 -2

발견 위치:

- 탕비실
- 내 자리
- 회의실

### snack: 과자

유형:

- consumable
- food

설명:

```text
누군가 비상용으로 숨겨둔 과자.
이제 정말 비상용이 되었다.
```

효과 초안:

- 허기 -15
- 사용 후 제거

발견 위치:

- 탕비실
- 개발팀 사무실

### cup_noodle: 컵라면

유형:

- consumable
- food

설명:

```text
야근의 오래된 동료.
물을 넣을 수 있다면 아직 희망이 있다.
```

효과 초안:

- 허기 -35
- 조건부: 생수 또는 정수기 접근이 있으면 효과 증가
- 물 없이 먹으면 갈증 +10 가능

발견 위치:

- 탕비실

### first_aid_kit: 구급상자

유형:

- consumable
- medical

설명:

```text
비상용 구급상자.
누군가 한 번 열어본 흔적이 있다.
```

효과 초안:

- 체력 +30
- 사용 후 제거

발견 위치:

- 복도
- 비상계단 근처
- 개발팀 사무실

### power_bank: 보조배터리

유형:

- consumable
- battery
- tool

설명:

```text
충전량이 애매하게 남은 보조배터리.
그래도 어둠 속에서는 애매한 희망도 희망이다.
```

효과 초안:

- 배터리 +30
- 사용 후 제거 또는 빈 보조배터리로 전환

발견 위치:

- 내 자리
- 개발팀 사무실

### flashlight: 손전등

유형:

- tool
- light

설명:

```text
작은 손전등.
회사 지급품은 아니고 누군가의 개인 물건 같다.
```

효과 초안:

- 어두운 구역 선택지 해금
- 사용할 때마다 배터리 -2 또는 별도 내구도 감소

발견 위치:

- 책상 아래
- 비상계단 근처

### employee_badge: 사원증

유형:

- key
- access

설명:

```text
당신의 사원증.
사진 속 당신은 아직 이 상황을 모른다.
```

효과 초안:

- 기본 출입 선택지 해금
- 서버실 앞/보안 패널에서 조건으로 사용

시작 보유:

- yes

### security_override_badge: 보안실 우회권한

유형:

- key
- security
- access
- conquest

설명:

```text
보안실 콘솔에서 임시로 발급한 출입 권한.
층수 기록이 맞지 않는 문에만 통한다.
```

효과 초안:

- 서버실 앞의 우회 진입 선택지 해금
- 보안실-서버실 정복 루트의 후속 선택지 조건

발견 위치:

- 보안실 `security_room_floor_mismatch_console`

### parking_key_fob: 지하주차장 키태그

유형:

- key
- parking
- access

설명:

```text
시동이 켜진 차 안쪽에서 발견한 작은 키태그.
차종 로고 대신 사내 자산번호가 붙어 있다.
```

효과 초안:

- 지하주차장 차단기 개방 선택지 해금
- `escape_parking_lot` 탈출 조건 일부

발견 위치:

- 지하주차장 `parking_ignition`

### crumpled_printout: 구겨진 출력물

유형:

- clue
- printer
- reality_link

설명:

```text
복합기에서 나온 구겨진 출력물.
문서 양식은 사내 표준인데, 내용은 아직 일어나지 않은 일을 말한다.
```

효과 초안:

- 현실 연결 루트 시작 플래그
- 회의실/탕비실 후속 선택지 해금

발견 위치:

- 복합기 구역

### ex_employee_note: 퇴사자의 메모

유형:

- clue
- truth
- messenger

설명:

```text
이미 퇴사한 사람이 남긴 듯한 메모.
그런데 날짜는 내일이다.
```

효과 초안:

- 진실 루트 단서
- 사내망 메시지 후속 이벤트 해금

발견 위치:

- 내 자리
- 개발팀 사무실
- 사내망 이벤트 보상
