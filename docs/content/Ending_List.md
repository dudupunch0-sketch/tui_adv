# 엔딩 목록

## 목적

이 문서는 1차 수직 슬라이스에서 도달 가능한 엔딩과, 이후 확장할 엔딩 방향을 정의한다.

## 엔딩 우선순위

여러 엔딩 조건이 동시에 만족되면 높은 priority가 우선한다.

| priority | 종류 |
|---:|---|
| 100 | 즉시 실패 |
| 80 | 히든/현실 연결 |
| 75 | 보안 정복 |
| 70 | 진실/정복 |
| 65 | 옥상 신호 탈출 |
| 60 | 비상계단 탈출 |
| 10 | 중간/메타 |

## 1차 엔딩

### failure_health: 사망

유형:

- failure

priority:

- 100

조건:

- 체력 <= 0

본문 톤:

```text
마지막으로 본 것은 비상등의 붉은 깜빡임이었다.
회사는 당신의 부재를 결근으로 처리할 것이다.
```

### failure_sanity: 정신 붕괴

유형:

- failure

priority:

- 100

조건:

- 정신력 <= 0

본문 톤:

```text
회의는 끝나지 않았다.
당신은 언제부터인가 참석자가 아니라 안건이 되어 있었다.
```

### failure_dehydration_or_starvation: 탈진

유형:

- failure

priority:

- 100

조건 초안:

- 갈증 >= 100
- 또는 허기 >= 100 상태에서 추가 위험 턴 경과

본문 톤:

```text
탕비실은 한 걸음 거리였을지도 모른다.
하지만 이곳에서 거리는 더 이상 믿을 수 있는 단위가 아니었다.
```

### escape_emergency_stairs: 비상계단 탈출

유형:

- escape

priority:

- 60

조건 초안:

- 위치: emergency_stairs
- 플래그: `emergency_stairs_route_open`
- 체력 > 0
- 정신력 > 0

본문 톤:

```text
아래층 표시가 마지막으로 한 번 깜빡였다.
문을 밀자, 차갑고 평범한 밤공기가 들어왔다.

처음으로 사내망이 완전히 끊겼다.
그 침묵이 이렇게 안심되는 것인 줄 몰랐다.
```

### escape_rooftop_signal: 옥상 외부 신호

유형:

- escape

priority:

- 65

조건:

- 위치: rooftop
- 플래그: `rooftop_signal_sent`
- 단서: `outside_signal_ack`
- 체력 > 0
- 정신력 > 0

본문 톤:

```text
제한된 외부 신호가 옥상 비상등을 실제 밤하늘과 동기화했다.
누군가 회사 바깥에서 이 건물이 아직 있다는 사실을 확인했다.
```

### hidden_reality_hint_001: 첫 번째 현실 연결 힌트

유형:

- hidden
- reality_link

priority:

- 80

조건:

- 플래그: `printer_secret_started`
- 플래그: `pantry_hint_seen`
- 아이템: `crumpled_printout`
- 체력 > 0
- 정신력 > 0
- `local_hint_id`: `real_note_001`

본문 톤:

```text
출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다.
커피 냄새와 복합기 표식이 하나의 계산으로 이어졌다.

커피 냄새가 남아 있는 방.
차가운 문과 종이 냄새 사이의 반복 표식.
복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.

IP 주소: 192.168.0.42
숫자 합계: 33
```

공개 배포판:

- 중간 힌트와 공개 더미 IP/숫자 합계까지만 표시한다.

로컬 private 데이터 활성화 시:

- 더 구체적인 최종 힌트를 표시할 수 있다.
- 실제 최종 위치는 공개 저장소에 포함하지 않는다.

## 확장 엔딩

### conquest_network_admin: 사내망 장악

유형:

- conquest

방향:

서버실/전산실/보안 시스템을 장악해 회사의 규칙을 역이용한다.

핵심 조건 후보:

- server_panel_symbols
- security_camera_mapped
- internal_network_access
- broadcast_control

### conquest_security_lockdown: 보안 격리 권한 장악

유형:

- conquest

priority:

- 75

조건:

- 위치: server_room
- 플래그: `security_lockdown_claimed`
- 체력 > 0
- 정신력 > 0

본문 톤:

```text
어긋난 층수의 보안 기록과 서버실 격리 규칙이 하나의 예외 권한으로 묶였다.
이제 회사는 당신을 감시 대상이 아니라 잠금 절차의 소유자로 취급한다.
```

루트 요약:

1. 엘리베이터 문틈을 벌려 보안실로 잘못 돌아온다.
2. 보안실 층수 로그에서 `security_override_badge`와 `security_override_unlocked`를 얻는다.
3. 서버실 앞에서 우회권한을 사용해 `security_override_used`를 얻는다.
4. 서버실 관리자 콘솔에서 출입 로그와 격리 규칙을 함께 잠근다.

### truth_isolation_protocol: 격리 프로토콜의 진실

유형:

- truth

방향:

사람들이 사라진 것이 아니라, 플레이어가 다른 업무공간/차원/프로토콜에 격리되었음을 밝힌다.

핵심 조건 후보:

- ex_employee_contacted
- impossible_meeting_saved
- meeting_pattern_noticed
- server_log_fragment

### escape_parking_lot: 지하주차장 탈출

유형:

- escape

방향:

지하주차장의 차량 또는 출입 램프를 통해 탈출한다.

## 엔딩 작성 원칙

- 실패 엔딩도 정보를 조금 남긴다.
- 탈출 엔딩은 완전한 진실을 설명하지 않아도 된다.
- 히든 현실 연결 엔딩은 공개 문서에서 중간 힌트까지만 제공한다.
- 정복 엔딩은 회사 블랙코미디의 쾌감을 살린다.
- 진실 엔딩은 코스믹 호러의 대가를 남긴다.
