# 비밀 목록

## 목적

이 문서는 게임 내에서 추적할 비밀과 히든 루트 조건을 정의한다.
실제 현실 사무실의 최종 위치는 절대 기록하지 않는다.

## 공개 가능 범위

이 문서에 포함할 수 있는 것:

- secret id
- 게임 내 unlock 조건
- 공개 가능한 중간 힌트
- 관련 인카운터
- 관련 플래그
- 안전 원칙

이 문서에 포함하지 않는 것:

- 실제 최종 위치
- 실제 회사명, 층, 좌석, 보안구역
- 실제 직원명 또는 팀명
- 실제 보상 배치 사진

## Secret 001: 첫 번째 현실 연결 힌트

id:

- real_note_001

상태:

- 1차 수직 슬라이스 대상

목적:

복합기와 탕비실을 잇는 히든 루트를 통해 플레이어가 “게임 속 장소와 현실 사무실이 연결될 수 있다”는 사실을 처음 깨닫게 한다.

관련 위치:

- printer_area
- pantry
- meeting_room, 선택적 후속

관련 아이템:

- crumpled_printout

관련 인카운터:

- printer_prints_alone
- pantry_coffee_machine
- fridge_note

관련 플래그:

```text
printer_secret_started
printer_page_read
pantry_hint_seen
reality_link_001_unlocked
```

unlock 조건 초안:

1. 복합기 구역에서 `printer_prints_alone` 인카운터를 본다.
2. 출력물을 챙겨 `crumpled_printout`을 얻는다.
3. 탕비실에서 커피머신 뒤 또는 냉장고 안쪽 단서를 확인한다.
4. `printer_secret_started`와 `pantry_hint_seen`이 모두 있으면 히든 힌트 엔딩이 열린다.

공개 힌트 단계:

```text
1. 커피 냄새가 남아 있는 방.
2. 차가운 문과 종이 냄새 사이의 반복 표식.
3. 복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.
```

현재 공개 샘플 IP 주소는 더미 값 `192.168.0.42`이다.
실제 복합기 IP 주소는 내부망 정보일 수 있으므로 `private/secrets.local.yaml`의 `actual_ip_address`에만 둔다.

게임 내 표현:

```text
출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다.

커피 냄새가 남아 있는 방.
차가운 문과 종이 냄새 사이의 반복 표식.
복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.

당신은 이 힌트가 화면 밖을 향하고 있다는 사실을 깨닫는다.
```

private/local 데이터가 없을 때:

- 위 중간 힌트까지만 표시한다.
- 엔딩 또는 특수 로그로 “현실 연결이 열린 것 같다”는 감각을 준다.

private/local 데이터가 있을 때:

- 로컬 파일의 `final_hint`를 추가로 표시할 수 있다.
- 로컬 데이터는 `private/secrets.local.yaml`에 둔다.
- 이 파일은 `.gitignore`로 커밋 차단한다.

안전 체크:

- 공용 공간만 사용한다.
- 개인 물건을 건드리지 않는다.
- 잠긴 공간이나 위험 설비 근처를 쓰지 않는다.
- 공개 저장소에는 최종 위치를 쓰지 않는다.

## Secret 002: 두 번째 현실 연결 힌트

id:

- real_note_002

상태:

- 구현됨

목적:

토너 카트리지 안쪽 표식과 커피머신 점검 라벨을 이어, 첫 번째 출력물 루트와 다른 방식으로 현실 사무실의 두 번째 안전한 메모 후보를 암시한다.

관련 위치:

- printer_area
- pantry

관련 인카운터:

- printer_prints_alone
- pantry_coffee_machine

관련 플래그:

```text
reality_link_started
reality_link_second_seen
pantry_hint_seen
```

unlock 조건:

1. 복합기 구역에서 토너 카트리지를 확인해 `reality_link_hint_1`과 `reality_link_started`를 얻는다.
2. 탕비실 커피머신 점검 라벨의 표식을 확인해 `reality_link_hint_2`와 `reality_link_second_seen`을 얻는다.
3. 두 단서와 플래그가 모두 있으면 `hidden_reality_hint_002`가 열린다.

공개 힌트 단계:

```text
1. 토너 카트리지 안쪽 표식과 커피머신 점검 라벨을 같은 방향으로 본다.
2. 점검 라벨에 적힌 사내망 IP 주소의 숫자들을 모두 더한다.
3. 첫 번째 힌트와 다른 숫자라면 두 번째 메모를 찾는 좌표가 된다.
```

현재 공개 샘플 IP 주소는 더미 값 `10.30.0.45`이다.
실제 점검 라벨 또는 사내망 주소는 내부 정보일 수 있으므로 `private/secrets.local.yaml`의 `actual_ip_address`에만 둔다.

안전 체크:

- 공용 물품 또는 개발자가 설치한 메모만 사용한다.
- 개인 물건, 잠긴 공간, 위험 설비 근처를 쓰지 않는다.
- 공개 저장소에는 최종 위치를 쓰지 않는다.

## Secret 003 후보: 사내망의 반대편

id:

- internal_network_other_side

상태:

- 2차 이후 후보

목적:

사내망 메시지가 현재 시간의 사람이 아니라, 다른 격리 공간 또는 이전 루프의 플레이어에게서 온 것임을 암시한다.

관련 위치:

- dev_desk
- server_room_front
- hallway

관련 인카운터:

- ex_employee_messenger
- cold_server_door
- camera_watches
- office_broadcast

관련 플래그 후보:

```text
ex_employee_contacted
server_panel_symbols
internal_log_fragment_seen
camera_response_received
```

공개 힌트 방향:

- 메시지 timestamp 불일치
- 보낸 사람의 상태 불일치
- 퇴사자의 미래 날짜 메모
- 서버실 앞 패널의 알 수 없는 문자열

## Secret 004 후보: 비상계단은 출구가 아니다

id:

- emergency_stairs_loop

상태:

- 탈출 루트 확장 후보

목적:

비상계단을 단순 탈출구가 아니라 공간 격리의 규칙을 보여주는 장소로 만든다.

관련 위치:

- emergency_stairs
- hallway

관련 인카운터:

- stairwell_footsteps
- parking_ignition

관련 플래그 후보:

```text
stairs_loop_seen
footsteps_recorded
emergency_stairs_route_open
```

공개 힌트 방향:

- 내려갔는데 같은 층
- 위에서 들려야 할 소리가 아래서 들림
- 비상구 표지의 층수 변화

## Secret 작성 규칙

새 secret을 추가할 때는 다음 정보를 기록한다.

```text
id:
상태:
목적:
관련 위치:
관련 아이템:
관련 인카운터:
관련 플래그:
unlock 조건:
공개 힌트 단계:
private/local 연동 여부:
안전 체크:
```

현실 연결 secret이면 반드시 다음을 확인한다.

- 실제 최종 위치가 이 문서에 없는가?
- 공개 힌트가 중간 강도를 넘지 않는가?
- 개인/보안/위험 공간을 암시하지 않는가?
- private/local 파일 없이도 게임이 자연스럽게 동작하는가?
