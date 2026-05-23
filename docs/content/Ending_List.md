# 엔딩 목록

이 문서는 런타임 기본 엔딩 데이터 `src/tui_adv/data/endings.yaml`과 코드 내 즉시 실패 엔딩의 공개 문서판이다.
실제 구현 기준 YAML 엔딩은 13개이고, 체력/정신력 0 실패 엔딩 2개는 `src/tui_adv/game/endings.py`에서 즉시 판정한다.

## 코드 내 즉시 실패 엔딩

| id | 이름 | 조건 |
|---|---|---|
| `game_over_health_depleted` | 게임오버: 신체 반응 없음 | 체력 `0` 이하 |
| `game_over_sanity_depleted` | 게임오버: 집중도 붕괴 | 정신력 `0` 이하 |

## YAML 엔딩 요약

| id | 이름 | 유형 | 우선순위 | 조건 요약 |
|---|---|---|---:|---|
| `game_over_spatial_collapse` | 게임오버: 계단이 접혔다 | `failure` | 100 | 필수 플래그: `spatial_exit_failed` |
| `escape_commute` | 퇴근 성공 | `escape` | 60 | 위치: `emergency_stairs`; 필수 플래그: `escape_route_completed`; 최소 자원: `health>=1`, `sanity>=1` |
| `hidden_reality_hint_001` | 첫 번째 현실 연결 힌트 | `hidden` | 80 | 필수 아이템: `crumpled_printout`; 필수 플래그: `printer_secret_started`, `pantry_hint_seen`; 최소 자원: `health>=1`, `sanity>=1` |
| `hidden_reality_hint_002` | 두 번째 현실 연결 힌트 | `hidden` | 82 | 필수 단서: `reality_link_hint_1`, `reality_link_hint_2`; 필수 플래그: `reality_link_started`, `reality_link_second_seen`, `pantry_hint_seen`; 최소 자원: `health>=1`, `sanity>=1` |
| `hidden_reality_hint_003` | 세 번째 현실 연결 힌트 | `hidden` | 84 | 필수 단서: `future_choice_printout`, `reality_link_hint_3`; 필수 플래그: `reality_link_third_seen`, `meeting_marker_seen`; 최소 자원: `health>=1`, `sanity>=1` |
| `truth_isolation_protocol` | 격리 프로토콜의 진실 | `truth` | 70 | 필수 아이템: `ex_employee_memo`; 필수 단서: `meeting_pattern_noticed`, `server_log_fragment`; 필수 플래그: `impossible_meeting_saved`, `isolation_protocol_revealed`; 최소 자원: `health>=1`, `sanity>=1` |
| `conquest_network_admin` | 사내망 관리자 권한 | `conquest` | 70 | 위치: `server_room`; 필수 플래그: `network_admin_claimed`; 최소 자원: `health>=1`, `sanity>=1` |
| `conquest_security_lockdown` | 보안 격리 권한 장악 | `conquest` | 75 | 위치: `server_room`; 필수 플래그: `security_lockdown_claimed`; 최소 자원: `health>=1`, `sanity>=1` |
| `conquest_broadcast_channel` | 사내 방송 장악 | `conquest` | 50 | 필수 플래그: `server_room_broadcast_controlled`; 최소 자원: `health>=1`, `sanity>=1` |
| `escape_rooftop_signal` | 옥상 외부 신호 | `escape` | 65 | 위치: `rooftop`; 필수 단서: `outside_signal_ack`; 필수 플래그: `rooftop_signal_sent`; 최소 자원: `health>=1`, `sanity>=1` |
| `escape_parking_lot` | 지하주차장 탈출 | `escape` | 64 | 위치: `parking_lot`; 필수 아이템: `parking_key_fob`; 필수 단서: `parking_exit_route`; 필수 플래그: `parking_ramp_opened`; 최소 자원: `health>=1`, `sanity>=1` |
| `escape_lobby_revolving_door` | 로비 회전문 탈출 | `escape` | 66 | 위치: `lobby`; 필수 아이템: `visitor_badge`; 필수 단서: `outside_lobby_reflection`; 필수 플래그: `lobby_exit_opened`; 최소 자원: `health>=1`, `sanity>=1` |
| `conquest_executive_approval` | 대표 승인권 장악 | `conquest` | 76 | 위치: `executive_office`; 필수 단서: `executive_signature_loop`; 필수 플래그: `executive_approval_claimed`, `company_policy_overwritten`; 최소 자원: `health>=1`, `sanity>=1` |

## 상세

### game_over_spatial_collapse: 게임오버: 계단이 접혔다

- 유형: `failure`
- 우선순위: `100`
- 조건: 필수 플래그: `spatial_exit_failed`

공간 왜곡의 규칙을 틀렸다. 계단은 아래가 아니라 당신 안쪽으로 접혔다.

### escape_commute: 퇴근 성공

- 유형: `escape`
- 우선순위: `60`
- 조건: 위치: `emergency_stairs`; 필수 플래그: `escape_route_completed`; 최소 자원: `health>=1`, `sanity>=1`

공간 왜곡 속에서 반복되는 층수를 맞춰 풀었다. 비상문 너머의 평범한 밤공기가 당신을 퇴근시켰다.

```text
[POST-ESCAPE REPORT]
survivor_count: 1
evidence_level: 0
company_response: denial
employee_status: access_revoked
risk_level: ongoing

ENDING: 정문 밖
```

구현 메모: 이 후일담은 기존 `text`에 포함된 public-safe block이다. 새 `kind`나 새 `EndingDef` field는 없고, Web Storybook/SuperLightTUI는 Rust core `ScenePage.body_blocks`를 표시한다.

### hidden_reality_hint_001: 첫 번째 현실 연결 힌트

- 유형: `hidden`
- 우선순위: `80`
- 조건: 필수 아이템: `crumpled_printout`; 필수 플래그: `printer_secret_started`, `pantry_hint_seen`; 최소 자원: `health>=1`, `sanity>=1`
- 로컬 현실 힌트 id: `real_note_001`

출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다. 커피 냄새와 복합기 표식이 하나의 계산으로 이어졌다.

### hidden_reality_hint_002: 두 번째 현실 연결 힌트

- 유형: `hidden`
- 우선순위: `82`
- 조건: 필수 단서: `reality_link_hint_1`, `reality_link_hint_2`; 필수 플래그: `reality_link_started`, `reality_link_second_seen`, `pantry_hint_seen`; 최소 자원: `health>=1`, `sanity>=1`
- 로컬 현실 힌트 id: `real_note_002`

토너 안쪽 표식과 커피머신 점검 라벨의 숫자가 같은 현실 좌표를 향했다. 이번 단서는 출력물 없이도 사무실 어딘가에 남은 두 번째 메모를 가리킨다.

### hidden_reality_hint_003: 세 번째 현실 연결 힌트

- 유형: `hidden`
- 우선순위: `84`
- 조건: 필수 단서: `future_choice_printout`, `reality_link_hint_3`; 필수 플래그: `reality_link_third_seen`, `meeting_marker_seen`; 최소 자원: `health>=1`, `sanity>=1`
- 로컬 현실 힌트 id: `real_note_003`

출력물이 예고한 빈 선택지가 회의실 화이트보드 모서리와 정확히 겹쳤다. 세 번째 단서는 회의 기록이 아니라 현실 사무실에 남겨진 작은 표식을 향한다.

### truth_isolation_protocol: 격리 프로토콜의 진실

- 유형: `truth`
- 우선순위: `70`
- 조건: 필수 아이템: `ex_employee_memo`; 필수 단서: `meeting_pattern_noticed`, `server_log_fragment`; 필수 플래그: `impossible_meeting_saved`, `isolation_protocol_revealed`; 최소 자원: `health>=1`, `sanity>=1`

삭제된 출입 기록과 회의록은 같은 결론을 가리켰다. 사라진 것은 직원들이 아니라 당신이 접속한 업무공간이었다. 회사는 이 현상을 격리 프로토콜이라고 불렀다.

### conquest_network_admin: 사내망 관리자 권한

- 유형: `conquest`
- 우선순위: `70`
- 조건: 위치: `server_room`; 필수 플래그: `network_admin_claimed`; 최소 자원: `health>=1`, `sanity>=1`

관리자 콘솔은 당신을 임시 예외가 아니라 규칙 작성자로 받아들였다. 이제 회사의 격리 공간은 당신의 퇴근 절차를 승인해야 한다.

### conquest_security_lockdown: 보안 격리 권한 장악

- 유형: `conquest`
- 우선순위: `75`
- 조건: 위치: `server_room`; 필수 플래그: `security_lockdown_claimed`; 최소 자원: `health>=1`, `sanity>=1`

어긋난 층수의 보안 기록과 서버실 격리 규칙이 하나의 예외 권한으로 묶였다. 이제 회사는 당신을 감시 대상이 아니라 잠금 절차의 소유자로 취급한다.

### conquest_broadcast_channel: 사내 방송 장악

- 유형: `conquest`
- 우선순위: `50`
- 조건: 필수 플래그: `server_room_broadcast_controlled`; 최소 자원: `health>=1`, `sanity>=1`

당신은 제한된 사내 방송 채널을 붙잡았다. 이제 회사는 더 이상 알 수 없는 목소리만으로 명령하지 못한다.

### escape_rooftop_signal: 옥상 외부 신호

- 유형: `escape`
- 우선순위: `65`
- 조건: 위치: `rooftop`; 필수 단서: `outside_signal_ack`; 필수 플래그: `rooftop_signal_sent`; 최소 자원: `health>=1`, `sanity>=1`

제한된 외부 신호가 옥상 비상등을 실제 밤하늘과 동기화했다. 누군가 회사 바깥에서 이 건물이 아직 있다는 사실을 확인했다.

### escape_parking_lot: 지하주차장 탈출

- 유형: `escape`
- 우선순위: `64`
- 조건: 위치: `parking_lot`; 필수 아이템: `parking_key_fob`; 필수 단서: `parking_exit_route`; 필수 플래그: `parking_ramp_opened`; 최소 자원: `health>=1`, `sanity>=1`

지하주차장 차단기가 한 번 더 흔들리더니 회사의 격리 경계가 출구 램프를 놓쳤다. 헤드라이트가 켜진 빈 차들 사이로, 실제 도로의 습한 공기가 당신을 밀어냈다.

### escape_lobby_revolving_door: 로비 회전문 탈출

- 유형: `escape`
- 우선순위: `66`
- 조건: 위치: `lobby`; 필수 아이템: `visitor_badge`; 필수 단서: `outside_lobby_reflection`; 필수 플래그: `lobby_exit_opened`; 최소 자원: `health>=1`, `sanity>=1`

방문객 퇴실 절차가 회사의 격리 규칙보다 먼저 처리되었다. 로비 회전문은 같은 풍경을 반복하는 일을 멈추고, 당신을 실제 도로 쪽으로 밀어냈다.

### conquest_executive_approval: 대표 승인권 장악

- 유형: `conquest`
- 우선순위: `76`
- 조건: 위치: `executive_office`; 필수 단서: `executive_signature_loop`; 필수 플래그: `executive_approval_claimed`, `company_policy_overwritten`; 최소 자원: `health>=1`, `sanity>=1`

대표 승인란에 입력된 이름이 회사의 생존 규칙 위로 번졌다. 이제 모든 결재선은 당신의 퇴근 여부를 묻기 전에 먼저 당신에게 승인받아야 한다.
