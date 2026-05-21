window.TUI_ADV_IMPLEMENTATION = window.TUI_ADV_IMPLEMENTATION || {};
window.TUI_ADV_IMPLEMENTATION.content = {
  catalog: [
    {
      title: "위치",
      items: [
        { id: "dev_desk", name: "내 자리" },
        { id: "dev_office", name: "개발팀 사무실" },
        { id: "hallway", name: "복도" },
        { id: "supply_closet", name: "물품창고" },
        { id: "pantry", name: "탕비실" },
        { id: "meeting_room", name: "회의실" },
        { id: "printer_area", name: "복합기 구역" },
        { id: "server_room_front", name: "서버실 앞" },
        { id: "server_room", name: "서버실 내부" },
        { id: "emergency_stairs", name: "비상계단" },
        { id: "security_room", name: "보안실" },
        { id: "elevator_hall", name: "엘리베이터 홀" },
        { id: "rooftop", name: "옥상" },
        { id: "parking_lot", name: "지하주차장" },
        { id: "lobby", name: "로비" },
        { id: "executive_office", name: "대표실" }
      ]
    },
    {
      title: "인카운터",
      items: [
        { id: "ex_employee_messenger", name: "퇴사자의 메신저" },
        { id: "printer_prints_alone", name: "복합기가 혼자 출력한다" },
        { id: "pantry_coffee_machine", name: "탕비실 커피머신" },
        { id: "strange_water_dispenser", name: "정수기의 이상한 물" },
        { id: "supply_closet_cache", name: "물품창고 비상 보급함" },
        { id: "meeting_room_whiteboard_marker", name: "회의실 화이트보드 모서리" },
        { id: "meeting_room_all_hands", name: "존재하지 않는 부서의 전체회의" },
        { id: "security_room_delayed_cctv", name: "지연된 CCTV 화면" },
        { id: "security_room_floor_mismatch_console", name: "어긋난 층수의 보안 콘솔" },
        { id: "emergency_stairs_exit_sign", name: "비상계단 문틈 표식" },
        { id: "spatial_exit_puzzle", name: "비상계단 공간 왜곡" },
        { id: "server_room_radio", name: "서버실 문 앞 무전기" },
        { id: "server_room_console", name: "관리자 콘솔" },
        { id: "elevator_nonexistent_floor", name: "존재하지 않는 층의 엘리베이터" },
        { id: "rooftop_signal", name: "옥상의 제한된 외부 신호" },
        { id: "parking_ignition", name: "지하주차장의 시동음" },
        { id: "parking_exit_ramp", name: "지하주차장 차단기" },
        { id: "lobby_reception_kiosk", name: "무인 로비 안내 키오스크" },
        { id: "lobby_exit_gate", name: "로비 출구 게이트" },
        { id: "executive_approval_console", name: "대표실 결재 콘솔" }
      ]
    },
    {
      title: "엔딩",
      items: [
        { id: "game_over_spatial_collapse", name: "게임오버: 계단이 접혔다" },
        { id: "escape_commute", name: "퇴근 성공" },
        { id: "hidden_reality_hint_001", name: "첫 번째 현실 연결 힌트" },
        { id: "hidden_reality_hint_002", name: "두 번째 현실 연결 힌트" },
        { id: "hidden_reality_hint_003", name: "세 번째 현실 연결 힌트" },
        { id: "truth_isolation_protocol", name: "격리 프로토콜의 진실" },
        { id: "conquest_network_admin", name: "사내망 관리자 권한" },
        { id: "conquest_security_lockdown", name: "보안 격리 권한 장악" },
        { id: "conquest_broadcast_channel", name: "사내 방송 장악" },
        { id: "escape_rooftop_signal", name: "옥상 외부 신호" },
        { id: "escape_parking_lot", name: "지하주차장 탈출" },
        { id: "escape_lobby_revolving_door", name: "로비 회전문 탈출" },
        { id: "conquest_executive_approval", name: "대표 승인권 장악" }
      ]
    },
    {
      title: "아이템",
      items: [
        { id: "bottled_water", name: "생수" },
        { id: "coffee", name: "커피" },
        { id: "snack", name: "과자" },
        { id: "cup_noodle", name: "컵라면" },
        { id: "first_aid_kit", name: "구급상자" },
        { id: "power_bank", name: "보조배터리" },
        { id: "flashlight", name: "손전등" },
        { id: "employee_badge", name: "사원증" },
        { id: "security_override_badge", name: "보안실 우회권한" },
        { id: "crumpled_printout", name: "구겨진 출력물" },
        { id: "ex_employee_memo", name: "퇴사자의 메모" },
        { id: "parking_key_fob", name: "지하주차장 키태그" },
        { id: "visitor_badge", name: "임시 방문증" }
      ]
    },
    {
      title: "업적",
      items: [
        { id: "first_signal_received", name: "첫 신호 확인" },
        { id: "reality_link_discovered", name: "현실과 접속한 사람" },
        { id: "reality_link_second_marker", name: "두 번째 현실 표식" },
        { id: "reality_link_third_marker", name: "세 번째 현실 표식" },
        { id: "broadcast_channel_captured", name: "사내 방송 장악자" },
        { id: "truth_protocol_understood", name: "격리 프로토콜 독해" },
        { id: "network_admin_claimed", name: "사내망 관리자" },
        { id: "rooftop_signal_sent", name: "외부 신호 송신" },
        { id: "parking_lot_escape_driver", name: "지하주차장 탈출자" },
        { id: "lobby_exit_commuter", name: "로비 게이트 통과자" },
        { id: "executive_approval_holder", name: "대표 승인권자" }
      ]
    },
    {
      title: "브라우저 산출물",
      items: [
        { id: "web_app", name: "Vite fake-TUI 브라우저 앱" },
        { id: "web_terminal_parity", name: "브라우저 terminal 루트 parity mirror core" },
        { id: "web_inventory_achievement_panels", name: "브라우저 인벤토리·업적·컨트롤·압박 패널" },
        { id: "web_data_export", name: "YAML→JSON export/check 스크립트" },
        { id: "printer_pretext_effect", name: "복합기 pretext/Canvas anomaly panel" },
        { id: "public_secret_guard", name: "공개 secret private-only 필드 차단" }
      ]
    }
  ]
};
