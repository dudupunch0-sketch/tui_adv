# 인카운터 목록

이 문서는 런타임 기본 인카운터 데이터 `src/tui_adv/data/encounters.yaml`의 공개 문서판이다.
실제 구현 기준 현재 인카운터 수는 21개다.

## 목록 요약

| id | 이름 | 조건 요약 | 선택지 수 |
|---|---|---|---:|
| `ex_employee_messenger` | 퇴사자의 메신저 | 위치: `dev_desk` | 4 |
| `printer_prints_alone` | 복합기가 혼자 출력한다 | 위치: `printer_area` | 3 |
| `pantry_coffee_machine` | 탕비실 커피머신 | 위치: `pantry` | 5 |
| `strange_water_dispenser` | 정수기의 이상한 물 | 위치: `pantry`; 금지 플래그: `thirst_hallucination_seen`; 최소 자원: `thirst>=60` | 3 |
| `supply_closet_cache` | 물품창고 비상 보급함 | 위치: `supply_closet` | 4 |
| `supply_closet_auto_brawl` | 물품창고 자동 난투 | 위치: `supply_closet`; 필수 플래그: `supply_scuffle_started`; 금지 플래그: `supply_scuffle_resolved` | 3 |
| `meeting_room_whiteboard_marker` | 회의실 화이트보드 모서리 | 위치: `meeting_room`; 필수 단서: `future_choice_printout` | 2 |
| `meeting_room_all_hands` | 존재하지 않는 부서의 전체회의 | 위치: `meeting_room`; 필수 플래그: `truth_route_started` | 2 |
| `security_room_delayed_cctv` | 지연된 CCTV 화면 | 위치: `security_room`; 필수 단서: `meeting_pattern_noticed`; 필수 플래그: `impossible_meeting_saved` | 2 |
| `security_room_floor_mismatch_console` | 어긋난 층수의 보안 콘솔 | 위치: `security_room`; 필수 단서: `security_floor_misalignment`; 필수 플래그: `elevator_returned_wrong_floor` | 3 |
| `emergency_stairs_exit_sign` | 비상계단 문틈 표식 | 위치: `emergency_stairs`; 금지 플래그: `escape_puzzle_ready` | 2 |
| `spatial_exit_puzzle` | 비상계단 공간 왜곡 | 위치: `emergency_stairs`; 필수 플래그: `escape_puzzle_ready` | 2 |
| `server_room_radio` | 서버실 문 앞 무전기 | 위치: `server_room_front` | 4 |
| `server_room_console` | 관리자 콘솔 | 위치: `server_room` | 3 |
| `elevator_nonexistent_floor` | 존재하지 않는 층의 엘리베이터 | 위치: `elevator_hall` | 2 |
| `rooftop_signal` | 옥상의 제한된 외부 신호 | 위치: `rooftop`; 필수 플래그: `rooftop_accessed` | 2 |
| `parking_ignition` | 지하주차장의 시동음 | 위치: `parking_lot` | 2 |
| `parking_exit_ramp` | 지하주차장 차단기 | 위치: `parking_lot`; 필수 아이템: `parking_key_fob`; 필수 플래그: `parking_key_found` | 2 |
| `lobby_reception_kiosk` | 무인 로비 안내 키오스크 | 위치: `lobby` | 3 |
| `lobby_exit_gate` | 로비 출구 게이트 | 위치: `lobby`; 필수 아이템: `visitor_badge`; 필수 플래그: `visitor_badge_printed` | 2 |
| `executive_approval_console` | 대표실 결재 콘솔 | 위치: `executive_office`; 필수 플래그: `executive_route_started` | 2 |

## 상세

### ex_employee_messenger: 퇴사자의 메신저

퇴사한 전임자에게서 사내 메신저가 도착했다.

- 조건: 위치: `dev_desk`

선택지:
1. `check_message` — 메시지를 확인한다
   - 비용: `battery: -3`, `sanity: -2`
   - 결과: 단서+ `ex_employee_contacted`; 로그: 퇴사자의 메시지를 확인했다.
2. `ignore_phone` — 무시하고 휴대폰을 엎어둔다
   - 결과: 자원 `sanity: +2`; 로그: 휴대폰을 엎어두자 알림음이 한 박자 늦게 멈췄다.
3. `search_ex_employee` — 전임자의 이름을 사내망에서 검색한다
   - 비용: `battery: -8`
   - 결과: 아이템+ `ex_employee_memo`; 플래그+ `truth_route_started`; 로그: 사내망 캐시에 남은 전임자의 흔적을 찾았다.
4. `trace_packet_delay` — [인터페이스] 알림 지연 시간을 역추적한다
   - 조건: 최소 능력치: `interface>=4`
   - 비용: `battery: -2`
   - 결과: 로그: 알림 패킷을 조심스럽게 붙잡았다.
   - 판정: `2d6 + interface >= 10`
   - 성공: 단서+ `delayed_packet_route`; 플래그+ `network_truth_hint`; 로그: 지연 시간 사이에서 숨은 라우팅을 찾았다.
   - 실패: 자원 `sanity: -4`; 위험도 `+1`; 로그: 패킷이 역으로 당신의 단말을 훑고 지나갔다.

### printer_prints_alone: 복합기가 혼자 출력한다

꺼져 있던 복합기가 아직 하지 않은 선택을 출력한다.

- 조건: 위치: `printer_area`

선택지:
1. `read_printout` — 출력물을 읽는다
   - 비용: `sanity: -3`
   - 결과: 단서+ `future_choice_printout`; 로그: 출력물에는 아직 고르지 않은 선택지가 적혀 있었다.
2. `take_printout` — 출력물을 챙긴다
   - 결과: 아이템+ `crumpled_printout`; 플래그+ `printer_secret_started`; 로그: 따뜻한 출력물을 접어 주머니에 넣었다.
3. `check_toner` — 토너 카트리지를 확인한다
   - 조건: 최소 자원: `sanity>=40`
   - 결과: 단서+ `reality_link_hint_1`; 플래그+ `reality_link_started`; 로그: 토너 카트리지 안쪽에서 이상한 표식을 봤다.

### pantry_coffee_machine: 탕비실 커피머신

커피머신 화면에 '물을 보충하십시오'가 반복된다. 물통은 가득 차 있다.

- 조건: 위치: `pantry`

선택지:
1. `brew_coffee` — 커피를 뽑는다
   - 결과: 자원 `sanity: +4`, `hunger: -3`, `thirst: +5`; 로그: 커피는 아직 따뜻했고, 컵 바닥에는 작은 검은 점이 남았다.
2. `inspect_water_tank` — 물통을 확인한다
   - 결과: 자원 `sanity: -2`; 단서+ `full_water_tank_warning`; 로그: 물통은 가득 차 있는데 센서는 계속 빈 상태를 보고했다.
3. `trace_toner_symbol` — 점검 라벨의 표식을 토너 표식과 맞춰 본다
   - 조건: 필수 플래그: `reality_link_started`
   - 결과: 단서+ `reality_link_hint_2`; 플래그+ `reality_link_second_seen`, `pantry_hint_seen`; 로그: 커피머신 점검 라벨의 작은 숫자들이 토너 안쪽 표식과 같은 방향으로 접혔다.
4. `look_behind_machine` — 커피머신 뒤를 본다
   - 조건: 필수 플래그: `printer_secret_started`
   - 결과: 단서+ `reality_link_hint_2`; 플래그+ `coffee_machine_back_panel`, `pantry_hint_seen`; 로그: 커피머신 뒤쪽 패널에 복합기 출력물과 같은 표식이 있었다.
5. `take_bottled_water` — 밀봉된 생수 한 병을 챙긴다
   - 결과: 아이템+ `bottled_water`; 로그: 탕비실 냉장고 문 안쪽에서 밀봉된 생수 한 병을 꺼냈다.

### strange_water_dispenser: 정수기의 이상한 물

정수기에서 물소리가 나지만 컵에는 물이 차지 않는다. 목이 마를수록 빈 컵 안쪽이 출렁이는 것처럼 보인다.

- 조건: 위치: `pantry`; 금지 플래그: `thirst_hallucination_seen`; 최소 자원: `thirst>=60`

선택지:
1. `drink_false_water` — 물을 마신다
   - 결과: 자원 `thirst: -25`, `sanity: -8`; 플래그+ `thirst_hallucination_seen`; 로그: 빈 컵을 삼키자 목은 잠깐 식었지만 정수기 표시등이 당신의 이름을 깜빡였다.
2. `search_for_bottled_water` — 생수병을 찾는다
   - 비용: `sanity: -2`
   - 결과: 아이템+ `bottled_water`; 플래그+ `thirst_hallucination_seen`; 로그: 정수기 아래 수납칸에서 실제로 밀봉된 생수 한 병을 찾았다.
3. `unplug_dispenser` — 정수기 전원을 뽑는다
   - 결과: 단서+ `water_dispenser_false_sound`; 플래그+ `thirst_hallucination_seen`; 위험도 `-1`; 로그: 플러그를 뽑자 물소리는 멈췄지만 컵 안쪽의 물결은 한 박자 늦게 사라졌다.

### supply_closet_cache: 물품창고 비상 보급함

물품창고 안쪽 선반에 '재난 대응 키트'라고 적힌 박스가 열려 있다. 누군가 필요한 것만 가져가라는 듯 세 칸을 비워 두었다.

- 조건: 위치: `supply_closet`

선택지:
1. `take_first_aid_kit` — 구급상자를 챙긴다
   - 결과: 아이템+ `first_aid_kit`; 로그: 비상 보급함에서 구급상자를 꺼냈다.
2. `take_power_bank` — 보조배터리를 챙긴다
   - 결과: 아이템+ `power_bank`; 로그: 비상 보급함 아래칸에서 충전 케이블이 묶인 보조배터리를 챙겼다.
3. `take_emergency_snack` — 비상 간식 봉지를 챙긴다
   - 결과: 아이템+ `snack`; 로그: 유통기한이 오늘로 끝나는 과자 봉지를 챙겼다.
4. `brace_for_supply_scuffle` — 잠긴 물품 카트를 끌어 복도 쪽으로 세운다
   - 결과: 플래그+ `supply_scuffle_started`; 로그: 잠긴 바퀴가 바닥을 긁는 동안, 선반 뒤의 무언가가 움직임을 멈췄다.

### supply_closet_auto_brawl: 물품창고 자동 난투

물품창고의 좁은 통로에서 뒤틀린 야간 당직자가 선반을 붙잡고 돌진한다. 당신이 모든 타격을 직접 조작할 시간은 없다. 몸싸움은 이미 시작됐고, 지금 바꿀 수 있는 것은 거리와 균형뿐이다.

- 조건: 위치: `supply_closet`; 필수 플래그: `supply_scuffle_started`; 금지 플래그: `supply_scuffle_resolved`
- presentation: `visual_id=supply_closet_scuffle`, `layout=combat_intervention`, stable terms `거리 / 균형 / 소화기 핀`

선택지:
1. `keep_distance_between_shelves` — 선반 사이 거리를 벌려 숨을 고른다
   - 결과: 자원 `sanity: +2`; 플래그+ `supply_scuffle_resolved`; 로그: 당신은 선반 끝을 따라 물러섰고, 싸움은 더 커지기 전에 통로 반대편으로 흘러갔다.
2. `hook_cart_to_cabinet` — 캐비닛 손잡이에 카트를 걸어 거리를 만든다
   - 비용: `health: -4`
   - 결과: 단서+ `improvised_distance_control`; 플래그+ `supply_scuffle_resolved`, `combat_intervention_success`; 위험도 `-1`; 로그: 상대의 균형이 선반 쪽으로 밀렸다. 공격이 아니라 거리 조절이 난투의 흐름을 바꿨다.
3. `pull_extinguisher_pin` — 소화기 핀을 뽑아 시야를 끊는다
   - 비용: `sanity: -4`
   - 결과: 단서+ `improvised_visibility_break`; 플래그+ `supply_scuffle_resolved`, `combat_intervention_success`; 위험도 `-1`; 로그: 분말이 통로를 덮자 손과 그림자만 남았다. 당신은 보이는 공격 대신 보이지 않는 틈으로 빠져나왔다.

### meeting_room_whiteboard_marker: 회의실 화이트보드 모서리

회의실 화이트보드 모서리에 지워지지 않는 작은 사각형이 남아 있다. 방금 본 출력물의 빈 선택지와 같은 모양이다.

- 조건: 위치: `meeting_room`; 필수 단서: `future_choice_printout`

선택지:
1. `decode_whiteboard_marker` — 출력물의 빈 선택지를 화이트보드 표식에 겹친다
   - 비용: `sanity: -3`
   - 결과: 단서+ `reality_link_hint_3`; 플래그+ `reality_link_third_seen`, `meeting_marker_seen`; 로그: 화이트보드 모서리의 지워지지 않는 사각형이 출력물의 빈 선택지와 겹치며 세 번째 현실 표식이 되었다.
2. `erase_marker_corner` — 표식을 지우개로 문질러 본다
   - 결과: 자원 `sanity: -2`; 플래그+ `meeting_marker_smudged`; 로그: 표식은 지워지지 않고 지우개 가루만 작은 화살표처럼 남겼다.

### meeting_room_all_hands: 존재하지 않는 부서의 전체회의

회의실 화면에는 방금 저장된 회의록이 떠 있다. 참석자는 전 직원, 발언자는 당신 하나다.

- 조건: 위치: `meeting_room`; 필수 플래그: `truth_route_started`

선택지:
1. `save_impossible_minutes` — 회의록을 저장하고 패턴을 표시한다
   - 비용: `battery: -4`, `sanity: -5`
   - 결과: 단서+ `meeting_pattern_noticed`; 플래그+ `impossible_meeting_saved`; 로그: 회의록의 발언 시간이 모두 같은 초 단위로 반복된다는 것을 표시했다.
2. `leave_without_saving` — 저장하지 않고 회의실을 나간다
   - 결과: 자원 `sanity: +1`; 로그: 회의실 문이 닫히자 방금 전 회의 알림이 취소되었다.

### security_room_delayed_cctv: 지연된 CCTV 화면

보안실 모니터들은 복도보다 한 박자 늦은 장면을 반복한다. 조금 전 당신이 한 선택도 녹화되어 있다.

- 조건: 위치: `security_room`; 필수 단서: `meeting_pattern_noticed`; 필수 플래그: `impossible_meeting_saved`

선택지:
1. `replay_delayed_cctv` — 지연된 CCTV 화면을 되감는다
   - 비용: `battery: -6`, `sanity: -4`
   - 결과: 단서+ `server_log_fragment`; 플래그+ `security_camera_mapped`, `isolation_protocol_revealed`; 로그: CCTV 지연 프레임 사이에서 격리 프로토콜 로그 조각을 읽었다.
2. `cover_cameras` — 모니터를 덮고 보안실을 떠난다
   - 결과: 자원 `sanity: +2`; 로그: 꺼진 모니터들이 천천히 당신을 놓아주었다.

### security_room_floor_mismatch_console: 어긋난 층수의 보안 콘솔

엘리베이터가 토해낸 보안실의 출입 기록은 현재 층을 서로 다른 세 숫자로 적고 있다. 틀린 층수 하나가 서버실 문과 같은 색으로 깜빡인다.

- 조건: 위치: `security_room`; 필수 단서: `security_floor_misalignment`; 필수 플래그: `elevator_returned_wrong_floor`

선택지:
1. `extract_security_override` — 보안실 층수 로그에서 서버실 우회권한을 뽑는다
   - 비용: `battery: -6`, `sanity: -3`
   - 결과: 아이템+ `security_override_badge`; 단서+ `security_override_route`; 플래그+ `security_override_unlocked`; 로그: 어긋난 층수 로그를 하나로 접자 서버실 우회권한이 임시 배지처럼 발급되었다.
2. `loop_cctv_delay` — 지연된 CCTV를 서버실 앞에 고정한다
   - 비용: `battery: -3`
   - 결과: 단서+ `server_room_cctv_blindspot`; 플래그+ `cctv_delay_looped`; 로그: 서버실 앞 CCTV가 당신보다 한 박자 늦게 감시하도록 루프를 걸었다.
3. `leave_misaligned_room` — 층수 기록을 닫고 보안실을 나간다
   - 결과: 자원 `sanity: +1`; 로그: 보안실 문패가 다시 현재 층을 모르는 척했다.

### emergency_stairs_exit_sign: 비상계단 문틈 표식

비상계단 문틈의 초록색 표식이 층수 표시와 같은 박자로 숨을 쉰다.

- 조건: 위치: `emergency_stairs`; 금지 플래그: `escape_puzzle_ready`

선택지:
1. `align_breathing_floor` — 계단문 틈의 숨소리와 층수 표시를 맞춘다
   - 비용: `sanity: -3`
   - 결과: 플래그+ `escape_puzzle_ready`; 로그: 초록 표식이 한 번 꺼졌다 켜지자 반복되는 층수의 규칙이 드러났다.
2. `ignore_exit_sign` — 표식과 눈을 마주치지 않고 계단을 살핀다
   - 결과: 자원 `sanity: +1`; 로그: 표식은 시야 가장자리에 초록빛 잔상을 남겼지만 더는 따라오지 않았다.

### spatial_exit_puzzle: 비상계단 공간 왜곡

층수 표시는 4, 4, 4, 4를 반복한다. 한 칸만 진짜 계단처럼 숨을 쉰다.

- 조건: 위치: `emergency_stairs`; 필수 플래그: `escape_puzzle_ready`

선택지:
1. `solve_distorted_floor` — 반복되는 층수의 비밀을 풀고 문을 통과한다
   - 결과: 플래그+ `escape_route_completed`; 로그: 층수 표시가 한 번 거꾸로 흐른 뒤 비상문이 열렸다.
2. `walk_down_wrong_stairs` — 아래라고 믿고 계속 내려간다
   - 결과: 자원 `sanity: -25`; 플래그+ `spatial_exit_failed`; 로그: 내려갈수록 계단은 회사 안쪽으로 더 깊게 접혔다.

### server_room_radio: 서버실 문 앞 무전기

닫힌 서버실 문틈에서 사내 방송이 거꾸로 새어 나온다.

- 조건: 위치: `server_room_front`

선택지:
1. `tune_internal_channel` — 제한된 주파수를 맞춘다
   - 비용: `battery: -5`, `sanity: -3`
   - 결과: 단서+ `internal_channel_key`; 플래그+ `server_room_broadcast_controlled`; 로그: 사내 방송의 잡음 뒤에서 통제 채널을 붙잡았다.
2. `back_away_from_signal` — 신호에서 물러난다
   - 결과: 자원 `sanity: +1`; 로그: 무전기 소리가 잠깐 당신의 목소리를 흉내 내다가 멈췄다.
3. `follow_cold_air` — 문틈의 찬 공기를 따라 안쪽으로 들어간다
   - 비용: `sanity: -2`
   - 결과: 플래그+ `server_room_entered`; 이동 `server_room`; 로그: 서버실 문은 열리지 않았지만, 당신은 이미 문 안쪽에 서 있었다.
4. `enter_with_security_override` — 보안실 우회권한으로 서버실 문을 연다
   - 조건: 필수 아이템: `security_override_badge`; 필수 플래그: `security_override_unlocked`
   - 비용: `battery: -2`
   - 결과: 플래그+ `server_room_entered`, `security_override_used`; 이동 `server_room`; 로그: 우회권한을 대자 서버실 문은 잠금 해제가 아니라 예외 처리로 당신을 통과시켰다.

### server_room_console: 관리자 콘솔

랙 사이 KVM 콘솔에는 관리자 세션이 잠들지 않은 채 남아 있다.

- 조건: 위치: `server_room`

선택지:
1. `assume_admin_console` — 관리자 콘솔에 격리 규칙을 덮어쓴다
   - 비용: `battery: -10`, `sanity: -6`
   - 결과: 단서+ `admin_console_signature`; 플래그+ `network_admin_claimed`, `internal_network_access`; 로그: 관리자 콘솔의 격리 규칙을 당신의 퇴근 규칙으로 덮어썼다.
2. `pull_network_cable` — 가장 두꺼운 케이블을 뽑는다
   - 결과: 자원 `sanity: -8`; 플래그+ `network_backlash`; 로그: 케이블을 뽑자 서버실 전체가 당신의 심장 박동으로 재부팅했다.
3. `lock_isolation_with_security_override` — 출입 로그와 격리 규칙을 함께 잠근다
   - 조건: 필수 아이템: `security_override_badge`; 필수 플래그: `security_override_used`
   - 비용: `battery: -8`, `sanity: -5`
   - 결과: 단서+ `security_lockdown_signature`; 플래그+ `network_admin_claimed`, `internal_network_access`, `security_lockdown_claimed`; 로그: 출입 로그와 격리 규칙이 같은 해시로 묶이자 보안실과 서버실이 당신을 예외 관리자라고 불렀다.

### elevator_nonexistent_floor: 존재하지 않는 층의 엘리베이터

엘리베이터 버튼 패널에는 없던 R층이 켜져 있다. 외부 인터넷 아이콘은 한 칸만 떠 있지만, 층수 표시는 계속 옥상을 가리킨다.

- 조건: 위치: `elevator_hall`

선택지:
1. `press_rooftop_button` — 존재하지 않는 R층 버튼을 누른다
   - 비용: `battery: -4`, `sanity: -3`
   - 결과: 단서+ `nonexistent_rooftop_button`; 플래그+ `rooftop_accessed`; 이동 `rooftop`; 로그: 버튼을 누르자 엘리베이터는 움직이지 않았지만 문 밖이 옥상으로 바뀌었다.
2. `force_elevator_doors` — 문틈을 벌려 현재 층으로 돌아온다
   - 결과: 자원 `health: -4`; 단서+ `security_floor_misalignment`; 플래그+ `elevator_returned_wrong_floor`; 이동 `security_room`; 로그: 엘리베이터 문틈을 억지로 벌리자 보안실 모니터들이 방금 전의 현재 층을 되감아 보여 주었다.

### rooftop_signal: 옥상의 제한된 외부 신호

옥상 난간 너머로 도시의 불빛이 보인다. 휴대폰은 외부 인터넷을 한 번만 보낼 수 있을 만큼의 신호를 붙잡았다.

- 조건: 위치: `rooftop`; 필수 플래그: `rooftop_accessed`

선택지:
1. `send_limited_signal` — 제한된 외부 신호를 짧게 송신한다
   - 비용: `battery: -12`, `sanity: -5`
   - 결과: 단서+ `outside_signal_ack`; 플래그+ `rooftop_signal_sent`; 로그: 짧은 구조 신호가 전송되자 옥상 비상등이 실제 밤하늘과 같은 박자로 깜빡였다.
2. `record_wrong_skyline` — 어긋난 도시 야경을 녹화한다
   - 비용: `battery: -5`
   - 결과: 자원 `sanity: -4`; 단서+ `wrong_skyline_recording`; 로그: 녹화된 야경에는 회사 건물이 바깥에서 보이지 않았다.

### parking_ignition: 지하주차장의 시동음

지하주차장 어딘가에서 시동이 걸린 차가 낮게 떨고 있다. 운전석에는 아무도 없고, 대시보드에는 사내망 출입 안내가 떠 있다.

- 조건: 위치: `parking_lot`

선택지:
1. `follow_idling_engine` — 켜져 있는 차의 키태그를 찾는다
   - 비용: `battery: -4`, `sanity: -3`
   - 결과: 아이템+ `parking_key_fob`; 단서+ `idling_engine_trace`; 플래그+ `parking_key_found`; 로그: 시동음이 가장 크게 울리는 차의 컵홀더에서 작은 키태그를 찾았다.
2. `silence_engine` — 시동음을 무시하고 차 사이를 지나간다
   - 결과: 자원 `sanity: +1`; 로그: 시동음은 등 뒤에서 계속 따라왔지만, 잠깐은 모른 척할 수 있었다.

### parking_exit_ramp: 지하주차장 차단기

출구 차단기는 올라가다 만 상태로 멈췄다. 키태그를 가까이 대자 외부 도로의 습기가 아주 얇게 흘러든다.

- 조건: 위치: `parking_lot`; 필수 아이템: `parking_key_fob`; 필수 플래그: `parking_key_found`

선택지:
1. `open_exit_ramp` — 주차장 차단기를 임시 개방한다
   - 비용: `battery: -5`, `sanity: -4`
   - 결과: 단서+ `parking_exit_route`; 플래그+ `parking_ramp_opened`; 로그: 차단기가 정상 근무 종료 알림처럼 짧게 울리고, 지하주차장 출구가 실제 도로와 맞물렸다.
2. `wedge_ramp_with_cart` — 청소 카트로 차단기를 받쳐 둔다
   - 결과: 자원 `health: -3`; 플래그+ `parking_ramp_jammed`; 로그: 카트는 차단기를 붙잡았지만, 출구 밖 풍경은 아직 한 프레임씩 밀려 있다.

### lobby_reception_kiosk: 무인 로비 안내 키오스크

로비 안내 키오스크가 꺼진 화면으로 당신의 얼굴을 인식한다. 방문 목적 입력란에는 이미 '퇴근 승인'이 떠 있다.

- 조건: 위치: `lobby`

선택지:
1. `print_visitor_badge` — 방문증 프린터를 깨운다
   - 비용: `battery: -3`, `sanity: -2`
   - 결과: 아이템+ `visitor_badge`; 단서+ `lobby_reception_log`; 플래그+ `visitor_badge_printed`; 로그: 프린터가 열을 올리더니 당신을 방문객으로 분류한 임시 방문증을 뱉었다.
2. `press_executive_call` — 대표실 호출 버튼을 길게 누른다
   - 비용: `battery: -5`, `sanity: -4`
   - 결과: 단서+ `executive_call_route`; 플래그+ `executive_route_started`; 이동 `executive_office`; 로그: 호출 버튼을 누르자 로비 천장의 스피커가 결재 대기음을 내고, 다음 문은 대표실로 바뀌었다.
3. `wipe_guestbook` — 방문자 명부에서 내 이름을 지운다
   - 결과: 자원 `sanity: +1`; 플래그+ `lobby_guestbook_wiped`; 로그: 명부에서 이름을 지우자 회전문 유리에 비친 당신도 한 박자 늦게 사라졌다.

### lobby_exit_gate: 로비 출구 게이트

로비 출구 게이트는 바깥 도로를 비추지만, 바코드 리더는 사내 방문증만 읽겠다는 듯 붉게 깜빡인다.

- 조건: 위치: `lobby`; 필수 아이템: `visitor_badge`; 필수 플래그: `visitor_badge_printed`

선택지:
1. `scan_visitor_badge` — 방문증 바코드를 출구 게이트에 읽힌다
   - 비용: `battery: -2`, `sanity: -3`
   - 결과: 단서+ `outside_lobby_reflection`; 플래그+ `lobby_exit_opened`; 로그: 게이트가 방문객 퇴실 절차를 승인하자 로비 회전문 바깥의 도로가 더는 반복되지 않았다.
2. `brace_revolving_door` — 회전문을 몸으로 밀어 멈춘다
   - 결과: 자원 `health: -4`; 플래그+ `lobby_door_jammed`; 로그: 회전문은 잠깐 멈췄지만 바깥 풍경은 여전히 같은 택시를 세 번씩 지나가게 했다.

### executive_approval_console: 대표실 결재 콘솔

대표실 결재 콘솔은 회사의 생존 규칙을 문서번호 없이 열어 둔 채, 마지막 승인자 칸만 비워 두었다.

- 조건: 위치: `executive_office`; 필수 플래그: `executive_route_started`

선택지:
1. `claim_executive_approval` — 대표 승인란에 내 이름을 입력한다
   - 비용: `battery: -8`, `sanity: -6`
   - 결과: 단서+ `executive_signature_loop`; 플래그+ `executive_approval_claimed`, `company_policy_overwritten`; 로그: 결재 콘솔이 당신의 이름을 대표 승인권자로 복제하자 회사 규칙의 결재선이 거꾸로 접혔다.
2. `reject_survival_policy` — 생존 규칙 문서를 반려한다
   - 결과: 자원 `sanity: -5`; 플래그+ `executive_policy_rejected`; 로그: 반려 버튼을 누르자 문서는 반려 사유에 당신의 다음 생각을 자동으로 적기 시작했다.
