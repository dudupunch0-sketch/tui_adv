window.TUI_ADV_IMPLEMENTATION = window.TUI_ADV_IMPLEMENTATION || {};
window.TUI_ADV_IMPLEMENTATION.routes = {
  "nodes": [
    {
      "id": "start",
      "label": "내 자리",
      "kind": "escape",
      "x": 11,
      "y": 45,
      "summary": "퇴사자의 메신저에서 첫 선택이 시작된다.",
      "files": [
        "locations.yaml",
        "encounters.yaml"
      ],
      "links": [
        "dev_office"
      ],
      "scene": {
        "background": "당신의 모니터는 아직 켜져 있다.",
        "dialogue": "퇴사한 전임자에게서 사내 메신저가 도착했다.",
        "choices": [
          {
            "label": "개발팀 사무실(으)로 이동한다",
            "line": "내 자리에서 개발팀 사무실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_office",
            "next": "dev_office",
            "nextLabel": "개발팀 사무실",
            "source": "locations.yaml:dev_desk.connections"
          },
          {
            "label": "메시지를 확인한다",
            "line": "퇴사자의 메시지를 확인했다.",
            "requirements": "cost battery +3, sanity +2",
            "result": "clues ex_employee_contacted",
            "next": "start",
            "nextLabel": "현재 상황 유지",
            "source": "ex_employee_messenger:check_message"
          },
          {
            "label": "무시하고 휴대폰을 엎어둔다",
            "line": "휴대폰을 엎어두자 알림음이 한 박자 늦게 멈췄다.",
            "requirements": "없음",
            "result": "상태 변화 없음",
            "next": "start",
            "nextLabel": "현재 상황 유지",
            "source": "ex_employee_messenger:ignore_phone"
          },
          {
            "label": "전임자의 이름을 사내망에서 검색한다",
            "line": "사내망 캐시에 남은 전임자의 흔적을 찾았다.",
            "requirements": "cost battery +8",
            "result": "items ex_employee_memo; flags truth_route_started",
            "next": "start",
            "nextLabel": "현재 상황 유지",
            "source": "ex_employee_messenger:search_ex_employee"
          },
          {
            "label": "[인터페이스] 알림 지연 시간을 역추적한다",
            "line": "알림 패킷을 조심스럽게 붙잡았다.",
            "requirements": "interface 4 이상; cost battery +2; interface check 10",
            "result": "check interface 난이도 10",
            "next": "start",
            "nextLabel": "현재 상황 유지",
            "source": "ex_employee_messenger:trace_packet_delay"
          }
        ]
      }
    },
    {
      "id": "dev_office",
      "label": "개발팀 사무실",
      "kind": "escape",
      "x": 25,
      "y": 45,
      "summary": "복도, 회의실, 복합기, 물품창고로 이어지는 허브.",
      "files": [
        "locations.yaml"
      ],
      "links": [
        "hallway",
        "meeting_room",
        "printer_area",
        "supply_closet"
      ],
      "scene": {
        "background": "개발팀 사무실은 정상적으로 켜져 있지만 사람만 없다.",
        "dialogue": "개발팀 사무실에서 이동 가능한 경로를 고른다.",
        "choices": [
          {
            "label": "내 자리(으)로 이동한다",
            "line": "개발팀 사무실에서 내 자리(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_desk",
            "next": "start",
            "nextLabel": "내 자리",
            "source": "locations.yaml:dev_office.connections"
          },
          {
            "label": "복도(으)로 이동한다",
            "line": "개발팀 사무실에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:dev_office.connections"
          },
          {
            "label": "회의실(으)로 이동한다",
            "line": "개발팀 사무실에서 회의실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location meeting_room",
            "next": "meeting_room",
            "nextLabel": "회의실",
            "source": "locations.yaml:dev_office.connections"
          },
          {
            "label": "복합기 구역(으)로 이동한다",
            "line": "개발팀 사무실에서 복합기 구역(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location printer_area",
            "next": "printer_area",
            "nextLabel": "복합기 구역",
            "source": "locations.yaml:dev_office.connections"
          },
          {
            "label": "물품창고(으)로 이동한다",
            "line": "개발팀 사무실에서 물품창고(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location supply_closet",
            "next": "supply_closet",
            "nextLabel": "물품창고",
            "source": "locations.yaml:dev_office.connections"
          }
        ]
      }
    },
    {
      "id": "supply_closet",
      "label": "물품창고",
      "kind": "escape",
      "x": 25,
      "y": 24,
      "summary": "초반 생존 자원을 챙기는 보급 노드.",
      "files": [
        "locations.yaml",
        "encounters.yaml",
        "items.yaml"
      ],
      "links": [
        "dev_office"
      ],
      "scene": {
        "background": "물품창고 선반에는 라벨이 붙은 비상 보급품 박스들이 남아 있다.",
        "dialogue": "물품창고 안쪽 선반에 '재난 대응 키트'라고 적힌 박스가 열려 있다. 누군가 필요한 것만 가져가라는 듯 세 칸을 비워 두었다.",
        "choices": [
          {
            "label": "개발팀 사무실(으)로 이동한다",
            "line": "물품창고에서 개발팀 사무실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_office",
            "next": "dev_office",
            "nextLabel": "개발팀 사무실",
            "source": "locations.yaml:supply_closet.connections"
          },
          {
            "label": "구급상자를 챙긴다",
            "line": "비상 보급함에서 구급상자를 꺼냈다.",
            "requirements": "없음",
            "result": "items first_aid_kit",
            "next": "supply_closet",
            "nextLabel": "현재 상황 유지",
            "source": "supply_closet_cache:take_first_aid_kit"
          },
          {
            "label": "보조배터리를 챙긴다",
            "line": "비상 보급함 아래칸에서 충전 케이블이 묶인 보조배터리를 챙겼다.",
            "requirements": "없음",
            "result": "items power_bank",
            "next": "supply_closet",
            "nextLabel": "현재 상황 유지",
            "source": "supply_closet_cache:take_power_bank"
          },
          {
            "label": "비상 간식 봉지를 챙긴다",
            "line": "유통기한이 오늘로 끝나는 과자 봉지를 챙겼다.",
            "requirements": "없음",
            "result": "items snack",
            "next": "supply_closet",
            "nextLabel": "현재 상황 유지",
            "source": "supply_closet_cache:take_emergency_snack"
          }
        ]
      }
    },
    {
      "id": "printer_area",
      "label": "복합기 구역",
      "kind": "reality",
      "x": 25,
      "y": 70,
      "summary": "현실 연결 루트의 첫 단서가 출력되는 장소.",
      "files": [
        "encounters.yaml",
        "secrets.example.yaml"
      ],
      "links": [
        "pantry",
        "meeting_room",
        "dev_office"
      ],
      "scene": {
        "background": "복합기는 절전 모드지만 출력 트레이에는 따뜻한 종이가 있다.",
        "dialogue": "꺼져 있던 복합기가 아직 하지 않은 선택을 출력한다.",
        "choices": [
          {
            "label": "개발팀 사무실(으)로 이동한다",
            "line": "복합기 구역에서 개발팀 사무실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_office",
            "next": "dev_office",
            "nextLabel": "개발팀 사무실",
            "source": "locations.yaml:printer_area.connections"
          },
          {
            "label": "탕비실(으)로 이동한다",
            "line": "복합기 구역에서 탕비실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location pantry",
            "next": "pantry",
            "nextLabel": "탕비실",
            "source": "locations.yaml:printer_area.connections"
          },
          {
            "label": "출력물을 읽는다",
            "line": "출력물에는 아직 고르지 않은 선택지가 적혀 있었다.",
            "requirements": "cost sanity +3",
            "result": "clues future_choice_printout",
            "next": "printer_area",
            "nextLabel": "현재 상황 유지",
            "source": "printer_prints_alone:read_printout"
          },
          {
            "label": "출력물을 챙긴다",
            "line": "따뜻한 출력물을 접어 주머니에 넣었다.",
            "requirements": "없음",
            "result": "items crumpled_printout; flags printer_secret_started",
            "next": "printer_area",
            "nextLabel": "현재 상황 유지",
            "source": "printer_prints_alone:take_printout"
          },
          {
            "label": "토너 카트리지를 확인한다",
            "line": "토너 카트리지 안쪽에서 이상한 표식을 봤다.",
            "requirements": "sanity 40 이상",
            "result": "clues reality_link_hint_1; flags reality_link_started",
            "next": "printer_area",
            "nextLabel": "현재 상황 유지",
            "source": "printer_prints_alone:check_toner"
          }
        ]
      }
    },
    {
      "id": "pantry",
      "label": "탕비실",
      "kind": "reality",
      "x": 42,
      "y": 76,
      "summary": "커피머신과 정수기 이벤트, 현실 힌트 1과 2의 종착지.",
      "files": [
        "encounters.yaml",
        "endings.yaml"
      ],
      "links": [
        "printer_area",
        "reality_001",
        "reality_002"
      ],
      "scene": {
        "background": "탕비실에는 커피 냄새가 남아 있다.",
        "dialogue": "커피머신 화면에 '물을 보충하십시오'가 반복된다. 물통은 가득 차 있다. / 정수기에서 물소리가 나지만 컵에는 물이 차지 않는다. 목이 마를수록 빈 컵 안쪽이 출렁이는 것처럼 보인다.",
        "choices": [
          {
            "label": "복합기 구역(으)로 이동한다",
            "line": "탕비실에서 복합기 구역(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location printer_area",
            "next": "printer_area",
            "nextLabel": "복합기 구역",
            "source": "locations.yaml:pantry.connections"
          },
          {
            "label": "커피를 뽑는다",
            "line": "커피는 아직 따뜻했고, 컵 바닥에는 작은 검은 점이 남았다.",
            "requirements": "없음",
            "result": "resources sanity +4, hunger -3, thirst +5",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "pantry_coffee_machine:brew_coffee"
          },
          {
            "label": "물통을 확인한다",
            "line": "물통은 가득 차 있는데 센서는 계속 빈 상태를 보고했다.",
            "requirements": "없음",
            "result": "clues full_water_tank_warning; resources sanity -2",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "pantry_coffee_machine:inspect_water_tank"
          },
          {
            "label": "점검 라벨의 표식을 토너 표식과 맞춰 본다",
            "line": "커피머신 점검 라벨의 작은 숫자들이 토너 안쪽 표식과 같은 방향으로 접혔다.",
            "requirements": "flags reality_link_started",
            "result": "clues reality_link_hint_2; flags reality_link_second_seen, pantry_hint_seen",
            "next": "reality_002",
            "nextLabel": "두 번째 현실 연결 힌트",
            "source": "pantry_coffee_machine:trace_toner_symbol"
          },
          {
            "label": "커피머신 뒤를 본다",
            "line": "커피머신 뒤쪽 패널에 복합기 출력물과 같은 표식이 있었다.",
            "requirements": "flags printer_secret_started",
            "result": "clues reality_link_hint_2; flags coffee_machine_back_panel, pantry_hint_seen",
            "next": "reality_002",
            "nextLabel": "두 번째 현실 연결 힌트",
            "source": "pantry_coffee_machine:look_behind_machine"
          },
          {
            "label": "밀봉된 생수 한 병을 챙긴다",
            "line": "탕비실 냉장고 문 안쪽에서 밀봉된 생수 한 병을 꺼냈다.",
            "requirements": "없음",
            "result": "items bottled_water",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "pantry_coffee_machine:take_bottled_water"
          },
          {
            "label": "물을 마신다",
            "line": "빈 컵을 삼키자 목은 잠깐 식었지만 정수기 표시등이 당신의 이름을 깜빡였다.",
            "requirements": "forbidden flags thirst_hallucination_seen; thirst 60 이상",
            "result": "flags thirst_hallucination_seen; resources thirst -25, sanity -8",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "strange_water_dispenser:drink_false_water"
          },
          {
            "label": "생수병을 찾는다",
            "line": "정수기 아래 수납칸에서 실제로 밀봉된 생수 한 병을 찾았다.",
            "requirements": "forbidden flags thirst_hallucination_seen; thirst 60 이상; cost sanity +2",
            "result": "items bottled_water; flags thirst_hallucination_seen",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "strange_water_dispenser:search_for_bottled_water"
          },
          {
            "label": "정수기 전원을 뽑는다",
            "line": "플러그를 뽑자 물소리는 멈췄지만 컵 안쪽의 물결은 한 박자 늦게 사라졌다.",
            "requirements": "forbidden flags thirst_hallucination_seen; thirst 60 이상",
            "result": "clues water_dispenser_false_sound; flags thirst_hallucination_seen; danger -1",
            "next": "pantry",
            "nextLabel": "현재 상황 유지",
            "source": "strange_water_dispenser:unplug_dispenser"
          }
        ]
      }
    },
    {
      "id": "meeting_room",
      "label": "회의실",
      "kind": "truth",
      "x": 42,
      "y": 56,
      "summary": "회의 패턴, 화이트보드, 세 번째 현실 연결 힌트가 만나는 지점.",
      "files": [
        "encounters.yaml",
        "secrets.example.yaml"
      ],
      "links": [
        "dev_office",
        "security_room",
        "reality_003",
        "truth"
      ],
      "scene": {
        "background": "회의실 예약 패널에는 방금 생성된 일정이 떠 있다.",
        "dialogue": "회의실 화이트보드 모서리에 지워지지 않는 작은 사각형이 남아 있다. 방금 본 출력물의 빈 선택지와 같은 모양이다. / 회의실 화면에는 방금 저장된 회의록이 떠 있다. 참석자는 전 직원, 발언자는 당신 하나다.",
        "choices": [
          {
            "label": "개발팀 사무실(으)로 이동한다",
            "line": "회의실에서 개발팀 사무실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_office",
            "next": "dev_office",
            "nextLabel": "개발팀 사무실",
            "source": "locations.yaml:meeting_room.connections"
          },
          {
            "label": "출력물의 빈 선택지를 화이트보드 표식에 겹친다",
            "line": "화이트보드 모서리의 지워지지 않는 사각형이 출력물의 빈 선택지와 겹치며 세 번째 현실 표식이 되었다.",
            "requirements": "clues future_choice_printout; cost sanity +3",
            "result": "clues reality_link_hint_3; flags reality_link_third_seen, meeting_marker_seen",
            "next": "reality_003",
            "nextLabel": "세 번째 현실 연결 힌트",
            "source": "meeting_room_whiteboard_marker:decode_whiteboard_marker"
          },
          {
            "label": "표식을 지우개로 문질러 본다",
            "line": "표식은 지워지지 않고 지우개 가루만 작은 화살표처럼 남겼다.",
            "requirements": "clues future_choice_printout",
            "result": "flags meeting_marker_smudged; resources sanity -2",
            "next": "meeting_room",
            "nextLabel": "현재 상황 유지",
            "source": "meeting_room_whiteboard_marker:erase_marker_corner"
          },
          {
            "label": "회의록을 저장하고 패턴을 표시한다",
            "line": "회의록의 발언 시간이 모두 같은 초 단위로 반복된다는 것을 표시했다.",
            "requirements": "flags truth_route_started; cost battery +4, sanity +5",
            "result": "clues meeting_pattern_noticed; flags impossible_meeting_saved",
            "next": "meeting_room",
            "nextLabel": "현재 상황 유지",
            "source": "meeting_room_all_hands:save_impossible_minutes"
          },
          {
            "label": "저장하지 않고 회의실을 나간다",
            "line": "회의실 문이 닫히자 방금 전 회의 알림이 취소되었다.",
            "requirements": "flags truth_route_started",
            "result": "resources sanity +1",
            "next": "meeting_room",
            "nextLabel": "현재 상황 유지",
            "source": "meeting_room_all_hands:leave_without_saving"
          }
        ]
      }
    },
    {
      "id": "hallway",
      "label": "복도",
      "kind": "escape",
      "x": 42,
      "y": 34,
      "summary": "로비, 주차장, 엘리베이터, 서버실 앞 경로로 분기한다.",
      "files": [
        "locations.yaml"
      ],
      "links": [
        "emergency_stairs",
        "elevator_hall",
        "server_room_front",
        "parking_lot",
        "lobby",
        "security_room"
      ],
      "scene": {
        "background": "복도 비상등이 일정하지 않은 간격으로 깜빡인다.",
        "dialogue": "복도에서 이동 가능한 경로를 고른다.",
        "choices": [
          {
            "label": "개발팀 사무실(으)로 이동한다",
            "line": "복도에서 개발팀 사무실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location dev_office",
            "next": "dev_office",
            "nextLabel": "개발팀 사무실",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "서버실 앞(으)로 이동한다",
            "line": "복도에서 서버실 앞(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location server_room_front",
            "next": "server_room_front",
            "nextLabel": "서버실 앞",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "비상계단(으)로 이동한다",
            "line": "복도에서 비상계단(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location emergency_stairs",
            "next": "emergency_stairs",
            "nextLabel": "비상계단",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "보안실(으)로 이동한다",
            "line": "복도에서 보안실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location security_room",
            "next": "security_room",
            "nextLabel": "보안실",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "엘리베이터 홀(으)로 이동한다",
            "line": "복도에서 엘리베이터 홀(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location elevator_hall",
            "next": "elevator_hall",
            "nextLabel": "엘리베이터 홀",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "지하주차장(으)로 이동한다",
            "line": "복도에서 지하주차장(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location parking_lot",
            "next": "parking_lot",
            "nextLabel": "지하주차장",
            "source": "locations.yaml:hallway.connections"
          },
          {
            "label": "로비(으)로 이동한다",
            "line": "복도에서 로비(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location lobby",
            "next": "lobby",
            "nextLabel": "로비",
            "source": "locations.yaml:hallway.connections"
          }
        ]
      }
    },
    {
      "id": "emergency_stairs",
      "label": "비상계단",
      "kind": "escape",
      "x": 59,
      "y": 18,
      "summary": "층수 반복 규칙을 맞추면 퇴근 성공으로 이어진다.",
      "files": [
        "encounters.yaml",
        "endings.yaml"
      ],
      "links": [
        "hallway"
      ],
      "scene": {
        "background": "계단 아래에서는 위층에서 들려야 할 발소리가 올라온다.",
        "dialogue": "비상계단 문틈의 초록색 표식이 층수 표시와 같은 박자로 숨을 쉰다. / 층수 표시는 4, 4, 4, 4를 반복한다. 한 칸만 진짜 계단처럼 숨을 쉰다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "비상계단에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:emergency_stairs.connections"
          },
          {
            "label": "계단문 틈의 숨소리와 층수 표시를 맞춘다",
            "line": "초록 표식이 한 번 꺼졌다 켜지자 반복되는 층수의 규칙이 드러났다.",
            "requirements": "forbidden flags escape_puzzle_ready; cost sanity +3",
            "result": "flags escape_puzzle_ready",
            "next": "emergency_stairs",
            "nextLabel": "현재 상황 유지",
            "source": "emergency_stairs_exit_sign:align_breathing_floor"
          },
          {
            "label": "표식과 눈을 마주치지 않고 계단을 살핀다",
            "line": "표식은 시야 가장자리에 초록빛 잔상을 남겼지만 더는 따라오지 않았다.",
            "requirements": "forbidden flags escape_puzzle_ready",
            "result": "상태 변화 없음",
            "next": "emergency_stairs",
            "nextLabel": "현재 상황 유지",
            "source": "emergency_stairs_exit_sign:ignore_exit_sign"
          },
          {
            "label": "반복되는 층수의 비밀을 풀고 문을 통과한다",
            "line": "층수 표시가 한 번 거꾸로 흐른 뒤 비상문이 열렸다.",
            "requirements": "flags escape_puzzle_ready",
            "result": "flags escape_route_completed",
            "next": "escape_commute",
            "nextLabel": "퇴근 성공",
            "source": "spatial_exit_puzzle:solve_distorted_floor"
          },
          {
            "label": "아래라고 믿고 계속 내려간다",
            "line": "내려갈수록 계단은 회사 안쪽으로 더 깊게 접혔다.",
            "requirements": "flags escape_puzzle_ready",
            "result": "flags spatial_exit_failed; resources sanity -25",
            "next": "game_over_spatial_collapse",
            "nextLabel": "게임오버: 계단이 접혔다",
            "source": "spatial_exit_puzzle:walk_down_wrong_stairs"
          }
        ]
      }
    },
    {
      "id": "elevator_hall",
      "label": "엘리베이터 홀",
      "kind": "escape",
      "x": 59,
      "y": 34,
      "summary": "옥상 신호 루트와 보안실 우회 루트의 관문.",
      "files": [
        "encounters.yaml"
      ],
      "links": [
        "hallway",
        "rooftop",
        "security_room"
      ],
      "scene": {
        "background": "엘리베이터 층수 표시가 존재하지 않는 R층과 4층 사이에서 떨린다.",
        "dialogue": "엘리베이터 버튼 패널에는 없던 R층이 켜져 있다. 외부 인터넷 아이콘은 한 칸만 떠 있지만, 층수 표시는 계속 옥상을 가리킨다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "엘리베이터 홀에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:elevator_hall.connections"
          },
          {
            "label": "옥상(으)로 이동한다",
            "line": "엘리베이터 홀에서 옥상(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location rooftop",
            "next": "rooftop",
            "nextLabel": "옥상",
            "source": "locations.yaml:elevator_hall.connections"
          },
          {
            "label": "존재하지 않는 R층 버튼을 누른다",
            "line": "버튼을 누르자 엘리베이터는 움직이지 않았지만 문 밖이 옥상으로 바뀌었다.",
            "requirements": "cost battery +4, sanity +3",
            "result": "이동 rooftop; clues nonexistent_rooftop_button; flags rooftop_accessed",
            "next": "rooftop",
            "nextLabel": "옥상",
            "source": "elevator_nonexistent_floor:press_rooftop_button"
          },
          {
            "label": "문틈을 벌려 현재 층으로 돌아온다",
            "line": "엘리베이터 문틈을 억지로 벌리자 보안실 모니터들이 방금 전의 현재 층을 되감아 보여 주었다.",
            "requirements": "없음",
            "result": "이동 security_room; clues security_floor_misalignment; flags elevator_returned_wrong_floor; resources health -4",
            "next": "security_room",
            "nextLabel": "보안실",
            "source": "elevator_nonexistent_floor:force_elevator_doors"
          }
        ]
      }
    },
    {
      "id": "rooftop",
      "label": "옥상",
      "kind": "escape",
      "x": 76,
      "y": 18,
      "summary": "제한된 외부 신호를 보내 탈출한다.",
      "files": [
        "endings.yaml",
        "encounters.yaml"
      ],
      "links": [
        "elevator_hall"
      ],
      "scene": {
        "background": "옥상 문 너머의 밤공기는 실제 바깥보다 한 프레임 늦게 움직인다.",
        "dialogue": "옥상 난간 너머로 도시의 불빛이 보인다. 휴대폰은 외부 인터넷을 한 번만 보낼 수 있을 만큼의 신호를 붙잡았다.",
        "choices": [
          {
            "label": "엘리베이터 홀(으)로 이동한다",
            "line": "옥상에서 엘리베이터 홀(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location elevator_hall",
            "next": "elevator_hall",
            "nextLabel": "엘리베이터 홀",
            "source": "locations.yaml:rooftop.connections"
          },
          {
            "label": "제한된 외부 신호를 짧게 송신한다",
            "line": "짧은 구조 신호가 전송되자 옥상 비상등이 실제 밤하늘과 같은 박자로 깜빡였다.",
            "requirements": "flags rooftop_accessed; cost battery +12, sanity +5",
            "result": "clues outside_signal_ack; flags rooftop_signal_sent",
            "next": "escape_rooftop_signal",
            "nextLabel": "옥상 외부 신호",
            "source": "rooftop_signal:send_limited_signal"
          },
          {
            "label": "어긋난 도시 야경을 녹화한다",
            "line": "녹화된 야경에는 회사 건물이 바깥에서 보이지 않았다.",
            "requirements": "flags rooftop_accessed; cost battery +5",
            "result": "clues wrong_skyline_recording; resources sanity -4",
            "next": "rooftop",
            "nextLabel": "현재 상황 유지",
            "source": "rooftop_signal:record_wrong_skyline"
          }
        ]
      }
    },
    {
      "id": "security_room",
      "label": "보안실",
      "kind": "truth",
      "x": 76,
      "y": 44,
      "summary": "지연 CCTV와 층수 불일치 콘솔로 진실 루트를 연다.",
      "files": [
        "encounters.yaml"
      ],
      "links": [
        "hallway",
        "server_room_front",
        "truth"
      ],
      "scene": {
        "background": "꺼진 CCTV 모니터들이 복도보다 한 박자 늦은 장면을 보여준다.",
        "dialogue": "보안실 모니터들은 복도보다 한 박자 늦은 장면을 반복한다. 조금 전 당신이 한 선택도 녹화되어 있다. / 엘리베이터가 토해낸 보안실의 출입 기록은 현재 층을 서로 다른 세 숫자로 적고 있다. 틀린 층수 하나가 서버실 문과 같은 색으로 깜빡인다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "보안실에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:security_room.connections"
          },
          {
            "label": "지연된 CCTV 화면을 되감는다",
            "line": "CCTV 지연 프레임 사이에서 격리 프로토콜 로그 조각을 읽었다.",
            "requirements": "flags impossible_meeting_saved; clues meeting_pattern_noticed; cost battery +6, sanity +4",
            "result": "clues server_log_fragment; flags security_camera_mapped, isolation_protocol_revealed",
            "next": "truth",
            "nextLabel": "격리 프로토콜의 진실",
            "source": "security_room_delayed_cctv:replay_delayed_cctv"
          },
          {
            "label": "모니터를 덮고 보안실을 떠난다",
            "line": "꺼진 모니터들이 천천히 당신을 놓아주었다.",
            "requirements": "flags impossible_meeting_saved; clues meeting_pattern_noticed",
            "result": "resources sanity +2",
            "next": "security_room",
            "nextLabel": "현재 상황 유지",
            "source": "security_room_delayed_cctv:cover_cameras"
          },
          {
            "label": "보안실 층수 로그에서 서버실 우회권한을 뽑는다",
            "line": "어긋난 층수 로그를 하나로 접자 서버실 우회권한이 임시 배지처럼 발급되었다.",
            "requirements": "flags elevator_returned_wrong_floor; clues security_floor_misalignment; cost battery +6, sanity +3",
            "result": "items security_override_badge; clues security_override_route; flags security_override_unlocked",
            "next": "security_room",
            "nextLabel": "현재 상황 유지",
            "source": "security_room_floor_mismatch_console:extract_security_override"
          },
          {
            "label": "지연된 CCTV를 서버실 앞에 고정한다",
            "line": "서버실 앞 CCTV가 당신보다 한 박자 늦게 감시하도록 루프를 걸었다.",
            "requirements": "flags elevator_returned_wrong_floor; clues security_floor_misalignment; cost battery +3",
            "result": "clues server_room_cctv_blindspot; flags cctv_delay_looped",
            "next": "security_room",
            "nextLabel": "현재 상황 유지",
            "source": "security_room_floor_mismatch_console:loop_cctv_delay"
          },
          {
            "label": "층수 기록을 닫고 보안실을 나간다",
            "line": "보안실 문패가 다시 현재 층을 모르는 척했다.",
            "requirements": "flags elevator_returned_wrong_floor; clues security_floor_misalignment",
            "result": "resources sanity +1",
            "next": "security_room",
            "nextLabel": "현재 상황 유지",
            "source": "security_room_floor_mismatch_console:leave_misaligned_room"
          }
        ]
      }
    },
    {
      "id": "server_room_front",
      "label": "서버실 앞",
      "kind": "conquest",
      "x": 59,
      "y": 56,
      "summary": "서버실 내부로 들어가는 제한 구역 문턱.",
      "files": [
        "locations.yaml",
        "encounters.yaml"
      ],
      "links": [
        "hallway",
        "server_room"
      ],
      "scene": {
        "background": "닫힌 문틈에서 차가운 바람이 나온다.",
        "dialogue": "닫힌 서버실 문틈에서 사내 방송이 거꾸로 새어 나온다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "서버실 앞에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:server_room_front.connections"
          },
          {
            "label": "서버실 내부(으)로 이동한다",
            "line": "서버실 앞에서 서버실 내부(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location server_room",
            "next": "server_room",
            "nextLabel": "서버실 내부",
            "source": "locations.yaml:server_room_front.connections"
          },
          {
            "label": "제한된 주파수를 맞춘다",
            "line": "사내 방송의 잡음 뒤에서 통제 채널을 붙잡았다.",
            "requirements": "cost battery +5, sanity +3",
            "result": "clues internal_channel_key; flags server_room_broadcast_controlled",
            "next": "conquest_broadcast_channel",
            "nextLabel": "사내 방송 장악",
            "source": "server_room_radio:tune_internal_channel"
          },
          {
            "label": "신호에서 물러난다",
            "line": "무전기 소리가 잠깐 당신의 목소리를 흉내 내다가 멈췄다.",
            "requirements": "없음",
            "result": "상태 변화 없음",
            "next": "server_room_front",
            "nextLabel": "현재 상황 유지",
            "source": "server_room_radio:back_away_from_signal"
          },
          {
            "label": "문틈의 찬 공기를 따라 안쪽으로 들어간다",
            "line": "서버실 문은 열리지 않았지만, 당신은 이미 문 안쪽에 서 있었다.",
            "requirements": "cost sanity +2",
            "result": "이동 server_room; flags server_room_entered",
            "next": "server_room",
            "nextLabel": "서버실 내부",
            "source": "server_room_radio:follow_cold_air"
          },
          {
            "label": "보안실 우회권한으로 서버실 문을 연다",
            "line": "우회권한을 대자 서버실 문은 잠금 해제가 아니라 예외 처리로 당신을 통과시켰다.",
            "requirements": "flags security_override_unlocked; items security_override_badge; cost battery +2",
            "result": "이동 server_room; flags server_room_entered, security_override_used",
            "next": "server_room",
            "nextLabel": "서버실 내부",
            "source": "server_room_radio:enter_with_security_override"
          }
        ]
      }
    },
    {
      "id": "server_room",
      "label": "서버실 내부",
      "kind": "conquest",
      "x": 76,
      "y": 62,
      "summary": "사내망 관리자와 보안 격리 정복 엔딩으로 이어진다.",
      "files": [
        "endings.yaml",
        "encounters.yaml"
      ],
      "links": [
        "server_room_front",
        "truth"
      ],
      "scene": {
        "background": "랙 LED가 별자리처럼 깜빡이고, 냉기는 일정한 박자로 숨을 쉰다.",
        "dialogue": "랙 사이 KVM 콘솔에는 관리자 세션이 잠들지 않은 채 남아 있다.",
        "choices": [
          {
            "label": "서버실 앞(으)로 이동한다",
            "line": "서버실 내부에서 서버실 앞(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location server_room_front",
            "next": "server_room_front",
            "nextLabel": "서버실 앞",
            "source": "locations.yaml:server_room.connections"
          },
          {
            "label": "관리자 콘솔에 격리 규칙을 덮어쓴다",
            "line": "관리자 콘솔의 격리 규칙을 당신의 퇴근 규칙으로 덮어썼다.",
            "requirements": "cost battery +10, sanity +6",
            "result": "clues admin_console_signature; flags network_admin_claimed, internal_network_access",
            "next": "conquest_network_admin",
            "nextLabel": "사내망 관리자 권한",
            "source": "server_room_console:assume_admin_console"
          },
          {
            "label": "가장 두꺼운 케이블을 뽑는다",
            "line": "케이블을 뽑자 서버실 전체가 당신의 심장 박동으로 재부팅했다.",
            "requirements": "없음",
            "result": "flags network_backlash; resources sanity -8",
            "next": "server_room",
            "nextLabel": "현재 상황 유지",
            "source": "server_room_console:pull_network_cable"
          },
          {
            "label": "출입 로그와 격리 규칙을 함께 잠근다",
            "line": "출입 로그와 격리 규칙이 같은 해시로 묶이자 보안실과 서버실이 당신을 예외 관리자라고 불렀다.",
            "requirements": "flags security_override_used; items security_override_badge; cost battery +8, sanity +5",
            "result": "clues security_lockdown_signature; flags network_admin_claimed, internal_network_access, security_lockdown_claimed",
            "next": "conquest_network_admin",
            "nextLabel": "사내망 관리자 권한",
            "source": "server_room_console:lock_isolation_with_security_override"
          }
        ]
      }
    },
    {
      "id": "truth",
      "label": "격리 프로토콜",
      "kind": "truth",
      "x": 89,
      "y": 48,
      "summary": "회의실과 보안실 단서를 모아 재난의 구조를 확인한다.",
      "files": [
        "endings.yaml"
      ],
      "links": [],
      "scene": {
        "background": "삭제된 출입 기록과 회의록은 같은 결론을 가리켰다. 사라진 것은 직원들이 아니라 당신이 접속한 업무공간이었다. 회사는 이 현상을 격리 프로토콜이라고 불렀다.",
        "dialogue": "격리 프로토콜의 진실",
        "choices": [
          {
            "label": "격리 프로토콜의 진실 확인",
            "line": "삭제된 출입 기록과 회의록은 같은 결론을 가리켰다. 사라진 것은 직원들이 아니라 당신이 접속한 업무공간이었다. 회사는 이 현상을 격리 프로토콜이라고 불렀다.",
            "requirements": "flags impossible_meeting_saved, isolation_protocol_revealed; clues meeting_pattern_noticed, server_log_fragment; items ex_employee_memo; health 1 이상; sanity 1 이상",
            "result": "truth_isolation_protocol 엔딩",
            "next": "truth_isolation_protocol",
            "nextLabel": "격리 프로토콜의 진실",
            "source": "endings.yaml:truth_isolation_protocol"
          }
        ]
      }
    },
    {
      "id": "parking_lot",
      "label": "지하주차장",
      "kind": "escape",
      "x": 59,
      "y": 84,
      "summary": "키태그와 차단기를 이용해 외부로 빠져나간다.",
      "files": [
        "endings.yaml",
        "encounters.yaml"
      ],
      "links": [
        "hallway"
      ],
      "scene": {
        "background": "지하주차장의 형광등은 시동음과 다른 박자로 깜빡인다.",
        "dialogue": "지하주차장 어딘가에서 시동이 걸린 차가 낮게 떨고 있다. 운전석에는 아무도 없고, 대시보드에는 사내망 출입 안내가 떠 있다. / 출구 차단기는 올라가다 만 상태로 멈췄다. 키태그를 가까이 대자 외부 도로의 습기가 아주 얇게 흘러든다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "지하주차장에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:parking_lot.connections"
          },
          {
            "label": "켜져 있는 차의 키태그를 찾는다",
            "line": "시동음이 가장 크게 울리는 차의 컵홀더에서 작은 키태그를 찾았다.",
            "requirements": "cost battery +4, sanity +3",
            "result": "items parking_key_fob; clues idling_engine_trace; flags parking_key_found",
            "next": "parking_lot",
            "nextLabel": "현재 상황 유지",
            "source": "parking_ignition:follow_idling_engine"
          },
          {
            "label": "시동음을 무시하고 차 사이를 지나간다",
            "line": "시동음은 등 뒤에서 계속 따라왔지만, 잠깐은 모른 척할 수 있었다.",
            "requirements": "없음",
            "result": "resources sanity +1",
            "next": "parking_lot",
            "nextLabel": "현재 상황 유지",
            "source": "parking_ignition:silence_engine"
          },
          {
            "label": "주차장 차단기를 임시 개방한다",
            "line": "차단기가 정상 근무 종료 알림처럼 짧게 울리고, 지하주차장 출구가 실제 도로와 맞물렸다.",
            "requirements": "flags parking_key_found; items parking_key_fob; cost battery +5, sanity +4",
            "result": "clues parking_exit_route; flags parking_ramp_opened",
            "next": "escape_parking_lot",
            "nextLabel": "지하주차장 탈출",
            "source": "parking_exit_ramp:open_exit_ramp"
          },
          {
            "label": "청소 카트로 차단기를 받쳐 둔다",
            "line": "카트는 차단기를 붙잡았지만, 출구 밖 풍경은 아직 한 프레임씩 밀려 있다.",
            "requirements": "flags parking_key_found; items parking_key_fob",
            "result": "flags parking_ramp_jammed; resources health -3",
            "next": "parking_lot",
            "nextLabel": "현재 상황 유지",
            "source": "parking_exit_ramp:wedge_ramp_with_cart"
          }
        ]
      }
    },
    {
      "id": "lobby",
      "label": "로비",
      "kind": "escape",
      "x": 76,
      "y": 84,
      "summary": "방문증과 회전문 게이트를 이용한 탈출 루트.",
      "files": [
        "endings.yaml",
        "encounters.yaml"
      ],
      "links": [
        "hallway",
        "executive_office"
      ],
      "scene": {
        "background": "로비 회전문은 바깥을 보여주지만 같은 장면을 세 번씩 반복한다.",
        "dialogue": "로비 안내 키오스크가 꺼진 화면으로 당신의 얼굴을 인식한다. 방문 목적 입력란에는 이미 '퇴근 승인'이 떠 있다. / 로비 출구 게이트는 바깥 도로를 비추지만, 바코드 리더는 사내 방문증만 읽겠다는 듯 붉게 깜빡인다.",
        "choices": [
          {
            "label": "복도(으)로 이동한다",
            "line": "로비에서 복도(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location hallway",
            "next": "hallway",
            "nextLabel": "복도",
            "source": "locations.yaml:lobby.connections"
          },
          {
            "label": "대표실(으)로 이동한다",
            "line": "로비에서 대표실(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location executive_office",
            "next": "executive_office",
            "nextLabel": "대표실",
            "source": "locations.yaml:lobby.connections"
          },
          {
            "label": "방문증 프린터를 깨운다",
            "line": "프린터가 열을 올리더니 당신을 방문객으로 분류한 임시 방문증을 뱉었다.",
            "requirements": "cost battery +3, sanity +2",
            "result": "items visitor_badge; clues lobby_reception_log; flags visitor_badge_printed",
            "next": "lobby",
            "nextLabel": "현재 상황 유지",
            "source": "lobby_reception_kiosk:print_visitor_badge"
          },
          {
            "label": "대표실 호출 버튼을 길게 누른다",
            "line": "호출 버튼을 누르자 로비 천장의 스피커가 결재 대기음을 내고, 다음 문은 대표실로 바뀌었다.",
            "requirements": "cost battery +5, sanity +4",
            "result": "이동 executive_office; clues executive_call_route; flags executive_route_started",
            "next": "executive_office",
            "nextLabel": "대표실",
            "source": "lobby_reception_kiosk:press_executive_call"
          },
          {
            "label": "방문자 명부에서 내 이름을 지운다",
            "line": "명부에서 이름을 지우자 회전문 유리에 비친 당신도 한 박자 늦게 사라졌다.",
            "requirements": "없음",
            "result": "flags lobby_guestbook_wiped; resources sanity +1",
            "next": "lobby",
            "nextLabel": "현재 상황 유지",
            "source": "lobby_reception_kiosk:wipe_guestbook"
          },
          {
            "label": "방문증 바코드를 출구 게이트에 읽힌다",
            "line": "게이트가 방문객 퇴실 절차를 승인하자 로비 회전문 바깥의 도로가 더는 반복되지 않았다.",
            "requirements": "flags visitor_badge_printed; items visitor_badge; cost battery +2, sanity +3",
            "result": "clues outside_lobby_reflection; flags lobby_exit_opened",
            "next": "escape_lobby_revolving_door",
            "nextLabel": "로비 회전문 탈출",
            "source": "lobby_exit_gate:scan_visitor_badge"
          },
          {
            "label": "회전문을 몸으로 밀어 멈춘다",
            "line": "회전문은 잠깐 멈췄지만 바깥 풍경은 여전히 같은 택시를 세 번씩 지나가게 했다.",
            "requirements": "flags visitor_badge_printed; items visitor_badge",
            "result": "flags lobby_door_jammed; resources health -4",
            "next": "lobby",
            "nextLabel": "현재 상황 유지",
            "source": "lobby_exit_gate:brace_revolving_door"
          }
        ]
      }
    },
    {
      "id": "executive_office",
      "label": "대표실",
      "kind": "conquest",
      "x": 89,
      "y": 78,
      "summary": "대표 승인권 장악 엔딩의 핵심 노드.",
      "files": [
        "endings.yaml",
        "encounters.yaml"
      ],
      "links": [
        "lobby"
      ],
      "scene": {
        "background": "대표실 책상 위 결재판에는 아직 작성하지 않은 당신의 퇴근 승인서가 놓여 있다.",
        "dialogue": "대표실 결재 콘솔은 회사의 생존 규칙을 문서번호 없이 열어 둔 채, 마지막 승인자 칸만 비워 두었다.",
        "choices": [
          {
            "label": "로비(으)로 이동한다",
            "line": "대표실에서 로비(으)로 이어지는 동선을 탄다.",
            "requirements": "이동 가능",
            "result": "location lobby",
            "next": "lobby",
            "nextLabel": "로비",
            "source": "locations.yaml:executive_office.connections"
          },
          {
            "label": "대표 승인란에 내 이름을 입력한다",
            "line": "결재 콘솔이 당신의 이름을 대표 승인권자로 복제하자 회사 규칙의 결재선이 거꾸로 접혔다.",
            "requirements": "flags executive_route_started; cost battery +8, sanity +6",
            "result": "clues executive_signature_loop; flags executive_approval_claimed, company_policy_overwritten",
            "next": "conquest_executive_approval",
            "nextLabel": "대표 승인권 장악",
            "source": "executive_approval_console:claim_executive_approval"
          },
          {
            "label": "생존 규칙 문서를 반려한다",
            "line": "반려 버튼을 누르자 문서는 반려 사유에 당신의 다음 생각을 자동으로 적기 시작했다.",
            "requirements": "flags executive_route_started",
            "result": "flags executive_policy_rejected; resources sanity -5",
            "next": "executive_office",
            "nextLabel": "현재 상황 유지",
            "source": "executive_approval_console:reject_survival_policy"
          }
        ]
      }
    },
    {
      "id": "reality_001",
      "label": "현실 힌트 1",
      "kind": "reality",
      "x": 42,
      "y": 91,
      "summary": "복합기 IP 숫자 합계에서 시작하는 첫 히든 루트.",
      "files": [
        "endings.yaml",
        "secrets.example.yaml"
      ],
      "links": [],
      "scene": {
        "background": "출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다. 커피 냄새와 복합기 표식이 하나의 계산으로 이어졌다.",
        "dialogue": "첫 번째 현실 연결 힌트",
        "choices": [
          {
            "label": "첫 번째 현실 연결 힌트 확인",
            "line": "출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다. 커피 냄새와 복합기 표식이 하나의 계산으로 이어졌다.",
            "requirements": "flags printer_secret_started, pantry_hint_seen; items crumpled_printout; health 1 이상; sanity 1 이상",
            "result": "hidden_reality_hint_001 엔딩; local_hint_id real_note_001",
            "next": "hidden_reality_hint_001",
            "nextLabel": "첫 번째 현실 연결 힌트",
            "source": "endings.yaml:hidden_reality_hint_001"
          }
        ]
      }
    },
    {
      "id": "reality_002",
      "label": "현실 힌트 2",
      "kind": "reality",
      "x": 59,
      "y": 90,
      "summary": "복합기 토너 표식과 커피머신 점검 라벨 루트.",
      "files": [
        "endings.yaml",
        "secrets.example.yaml"
      ],
      "links": [],
      "scene": {
        "background": "토너 안쪽 표식과 커피머신 점검 라벨의 숫자가 같은 현실 좌표를 향했다. 이번 단서는 출력물 없이도 사무실 어딘가에 남은 두 번째 메모를 가리킨다.",
        "dialogue": "두 번째 현실 연결 힌트",
        "choices": [
          {
            "label": "두 번째 현실 연결 힌트 확인",
            "line": "토너 안쪽 표식과 커피머신 점검 라벨의 숫자가 같은 현실 좌표를 향했다. 이번 단서는 출력물 없이도 사무실 어딘가에 남은 두 번째 메모를 가리킨다.",
            "requirements": "flags reality_link_started, reality_link_second_seen, pantry_hint_seen; clues reality_link_hint_1, reality_link_hint_2; health 1 이상; sanity 1 이상",
            "result": "hidden_reality_hint_002 엔딩; local_hint_id real_note_002",
            "next": "hidden_reality_hint_002",
            "nextLabel": "두 번째 현실 연결 힌트",
            "source": "endings.yaml:hidden_reality_hint_002"
          }
        ]
      }
    },
    {
      "id": "reality_003",
      "label": "현실 힌트 3",
      "kind": "reality",
      "x": 89,
      "y": 62,
      "summary": "화이트보드 더미 숫자 합계 퍼즐 루트.",
      "files": [
        "endings.yaml",
        "secrets.example.yaml"
      ],
      "links": [],
      "scene": {
        "background": "출력물이 예고한 빈 선택지가 회의실 화이트보드 모서리와 정확히 겹쳤다. 세 번째 단서는 회의 기록이 아니라 현실 사무실에 남겨진 작은 표식을 향한다.",
        "dialogue": "세 번째 현실 연결 힌트",
        "choices": [
          {
            "label": "세 번째 현실 연결 힌트 확인",
            "line": "출력물이 예고한 빈 선택지가 회의실 화이트보드 모서리와 정확히 겹쳤다. 세 번째 단서는 회의 기록이 아니라 현실 사무실에 남겨진 작은 표식을 향한다.",
            "requirements": "flags reality_link_third_seen, meeting_marker_seen; clues future_choice_printout, reality_link_hint_3; health 1 이상; sanity 1 이상",
            "result": "hidden_reality_hint_003 엔딩; local_hint_id real_note_003",
            "next": "hidden_reality_hint_003",
            "nextLabel": "세 번째 현실 연결 힌트",
            "source": "endings.yaml:hidden_reality_hint_003"
          }
        ]
      }
    }
  ],
  "routes": [
    {
      "title": "비상계단 탈출",
      "type": "escape",
      "status": "smoke 통과",
      "outcome": "escape_commute 엔딩",
      "steps": [
        "emergency_stairs에서 escape_puzzle_ready 플래그 확보",
        "공간 왜곡 퍼즐 선택",
        "퇴근 성공 엔딩 도달"
      ],
      "command": "--location emergency_stairs --flag escape_puzzle_ready --choice 1"
    },
    {
      "title": "옥상 외부 신호",
      "type": "escape",
      "status": "smoke 통과",
      "outcome": "escape_rooftop_signal 엔딩",
      "steps": [
        "개발팀 사무실에서 복도 이동",
        "엘리베이터 홀 이벤트 선택",
        "옥상에서 제한된 외부 신호 송신"
      ],
      "command": "--action choice:1 --action move:dev_office --action move:hallway --action move:elevator_hall --action choice:1 --action choice:1"
    },
    {
      "title": "지하주차장 탈출",
      "type": "escape",
      "status": "smoke 통과",
      "outcome": "escape_parking_lot 엔딩",
      "steps": [
        "복도에서 지하주차장 이동",
        "시동음 이벤트로 키태그 확보",
        "차단기 이벤트로 탈출"
      ],
      "command": "--action move:parking_lot 흐름"
    },
    {
      "title": "로비 회전문 탈출",
      "type": "escape",
      "status": "smoke 통과",
      "outcome": "escape_lobby_revolving_door 엔딩",
      "steps": [
        "로비 키오스크에서 방문증 확보",
        "로비 출구 게이트 선택",
        "회전문 탈출"
      ],
      "command": "--action move:lobby 흐름"
    },
    {
      "title": "대표 승인권 장악",
      "type": "conquest",
      "status": "smoke 통과",
      "outcome": "conquest_executive_approval 엔딩",
      "steps": [
        "로비에서 대표실 경로 확보",
        "대표실 결재 콘솔 접근",
        "승인권 장악"
      ],
      "command": "--action move:lobby --action choice:2 --action choice:1"
    },
    {
      "title": "보안 격리 정복",
      "type": "conquest",
      "status": "smoke 통과",
      "outcome": "conquest_security_lockdown 엔딩",
      "steps": [
        "엘리베이터 홀에서 보안실 우회",
        "서버실 앞에서 관리자 콘솔 접근",
        "보안 격리 권한 장악"
      ],
      "command": "--location elevator_hall --action choice:2 --action choice:1"
    },
    {
      "title": "격리 프로토콜의 진실",
      "type": "truth",
      "status": "smoke 통과",
      "outcome": "truth_isolation_protocol 엔딩",
      "steps": [
        "회의실 전체회의 패턴 확인",
        "보안실 지연 CCTV 확인",
        "격리 프로토콜 진실 엔딩"
      ],
      "command": "--action move:meeting_room ... --action move:security_room --action choice:1"
    },
    {
      "title": "현실 연결 힌트 1",
      "type": "reality",
      "status": "smoke 통과",
      "outcome": "hidden_reality_hint_001 엔딩",
      "steps": [
        "복합기에서 IP 숫자 단서 확보",
        "탕비실 커피머신 조사",
        "local secret 있으면 최종 힌트 표시"
      ],
      "command": "--location printer_area --action choice:2 --action move:pantry --action choice:3"
    },
    {
      "title": "현실 연결 힌트 2",
      "type": "reality",
      "status": "smoke 통과",
      "outcome": "hidden_reality_hint_002 엔딩",
      "steps": [
        "복합기 토너 표식 확인",
        "탕비실 커피머신 점검 라벨 확인",
        "두 번째 현실 연결 엔딩"
      ],
      "command": "--location printer_area --action choice:3 --action move:pantry --action choice:3"
    },
    {
      "title": "현실 연결 힌트 3",
      "type": "reality",
      "status": "smoke 통과",
      "outcome": "hidden_reality_hint_003 엔딩",
      "steps": [
        "복합기 화이트보드 더미 숫자 확보",
        "회의실 모서리 단서 확인",
        "숫자 합계 30과 세 번째 현실 연결 엔딩"
      ],
      "command": "--location printer_area --action choice:1 --action move:dev_office --action move:meeting_room --action choice:1"
    }
  ]
};
