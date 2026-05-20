window.TUI_ADV_IMPLEMENTATION = window.TUI_ADV_IMPLEMENTATION || {};
window.TUI_ADV_IMPLEMENTATION.routes = {
  nodes: [
    { id: "start", label: "내 자리", kind: "escape", summary: "퇴사자의 메신저에서 첫 선택이 시작된다.", files: ["locations.yaml", "encounters.yaml"], links: ["dev_office", "printer_area"] },
    { id: "dev_office", label: "개발팀 사무실", kind: "escape", summary: "복도, 회의실, 물품창고로 이어지는 허브.", files: ["locations.yaml"], links: ["hallway", "meeting_room"] },
    { id: "hallway", label: "복도", kind: "escape", summary: "로비, 주차장, 엘리베이터, 서버실 앞 경로로 분기한다.", files: ["locations.yaml"], links: ["lobby", "parking_lot", "elevator_hall"] },
    { id: "elevator_hall", label: "엘리베이터 홀", kind: "escape", summary: "옥상 신호 루트와 보안실 우회 루트의 관문.", files: ["encounters.yaml"], links: ["rooftop", "security_room"] },
    { id: "server_room", label: "서버실 내부", kind: "conquest", summary: "사내망 관리자와 보안 격리 정복 엔딩으로 이어진다.", files: ["endings.yaml", "encounters.yaml"], links: ["security_room"] },
    { id: "executive_office", label: "대표실", kind: "conquest", summary: "대표 승인권 장악 엔딩의 핵심 노드.", files: ["endings.yaml", "encounters.yaml"], links: ["lobby"] },
    { id: "meeting_room", label: "회의실", kind: "truth", summary: "회의 패턴, 화이트보드, 세 번째 현실 연결 힌트가 만나는 지점.", files: ["encounters.yaml", "secrets.example.yaml"], links: ["truth", "reality_003"] },
    { id: "security_room", label: "보안실", kind: "truth", summary: "지연 CCTV와 층수 불일치 콘솔로 진실 루트를 연다.", files: ["encounters.yaml"], links: ["truth", "server_room"] },
    { id: "truth", label: "격리 프로토콜", kind: "truth", summary: "회의실과 보안실 단서를 모아 재난의 구조를 확인한다.", files: ["endings.yaml"], links: [] },
    { id: "printer_area", label: "복합기 구역", kind: "reality", summary: "세 현실 연결 루트의 첫 단서가 출력되는 장소.", files: ["encounters.yaml", "secrets.example.yaml"], links: ["pantry", "meeting_room"] },
    { id: "pantry", label: "탕비실", kind: "reality", summary: "커피머신과 정수기 이벤트, 현실 힌트 1과 2의 종착지.", files: ["encounters.yaml"], links: ["reality_001", "reality_002"] },
    { id: "reality_001", label: "현실 힌트 1", kind: "reality", summary: "복합기 IP 숫자 합계에서 시작하는 첫 히든 루트.", files: ["endings.yaml", "secrets.example.yaml"], links: [] },
    { id: "reality_002", label: "현실 힌트 2", kind: "reality", summary: "복합기 토너 표식과 커피머신 점검 라벨 루트.", files: ["endings.yaml", "secrets.example.yaml"], links: [] },
    { id: "reality_003", label: "현실 힌트 3", kind: "reality", summary: "화이트보드 더미 숫자 합계 퍼즐 루트.", files: ["endings.yaml", "secrets.example.yaml"], links: [] },
    { id: "rooftop", label: "옥상", kind: "escape", summary: "제한된 외부 신호를 보내 탈출한다.", files: ["endings.yaml", "encounters.yaml"], links: [] },
    { id: "parking_lot", label: "지하주차장", kind: "escape", summary: "키태그와 차단기를 이용해 외부로 빠져나간다.", files: ["endings.yaml", "encounters.yaml"], links: [] },
    { id: "lobby", label: "로비", kind: "escape", summary: "방문증과 회전문 게이트를 이용한 탈출 루트.", files: ["endings.yaml", "encounters.yaml"], links: ["executive_office"] }
  ],
  routes: [
    { title: "비상계단 탈출", type: "escape", status: "smoke 통과", outcome: "escape_commute 엔딩", steps: ["emergency_stairs에서 escape_puzzle_ready 플래그 확보", "공간 왜곡 퍼즐 선택", "퇴근 성공 엔딩 도달"], command: "--location emergency_stairs --flag escape_puzzle_ready --choice 1" },
    { title: "옥상 외부 신호", type: "escape", status: "smoke 통과", outcome: "escape_rooftop_signal 엔딩", steps: ["개발팀 사무실에서 복도 이동", "엘리베이터 홀 이벤트 선택", "옥상에서 제한된 외부 신호 송신"], command: "--action choice:1 --action move:dev_office --action move:hallway --action move:elevator_hall --action choice:1 --action choice:1" },
    { title: "지하주차장 탈출", type: "escape", status: "smoke 통과", outcome: "escape_parking_lot 엔딩", steps: ["복도에서 지하주차장 이동", "시동음 이벤트로 키태그 확보", "차단기 이벤트로 탈출"], command: "--action move:parking_lot 흐름" },
    { title: "로비 회전문 탈출", type: "escape", status: "smoke 통과", outcome: "escape_lobby_revolving_door 엔딩", steps: ["로비 키오스크에서 방문증 확보", "로비 출구 게이트 선택", "회전문 탈출"], command: "--action move:lobby 흐름" },
    { title: "대표 승인권 장악", type: "conquest", status: "smoke 통과", outcome: "conquest_executive_approval 엔딩", steps: ["로비에서 대표실 경로 확보", "대표실 결재 콘솔 접근", "승인권 장악"], command: "--action move:lobby --action choice:2 --action choice:1" },
    { title: "보안 격리 정복", type: "conquest", status: "smoke 통과", outcome: "conquest_security_lockdown 엔딩", steps: ["엘리베이터 홀에서 보안실 우회", "서버실 앞에서 관리자 콘솔 접근", "보안 격리 권한 장악"], command: "--location elevator_hall --action choice:2 --action choice:1" },
    { title: "격리 프로토콜의 진실", type: "truth", status: "smoke 통과", outcome: "truth_isolation_protocol 엔딩", steps: ["회의실 전체회의 패턴 확인", "보안실 지연 CCTV 확인", "격리 프로토콜 진실 엔딩"], command: "--action move:meeting_room ... --action move:security_room --action choice:1" },
    { title: "현실 연결 힌트 1", type: "reality", status: "smoke 통과", outcome: "hidden_reality_hint_001 엔딩", steps: ["복합기에서 IP 숫자 단서 확보", "탕비실 커피머신 조사", "local secret 있으면 최종 힌트 표시"], command: "--location printer_area --action choice:2 --action move:pantry --action choice:3" },
    { title: "현실 연결 힌트 2", type: "reality", status: "smoke 통과", outcome: "hidden_reality_hint_002 엔딩", steps: ["복합기 토너 표식 확인", "탕비실 커피머신 점검 라벨 확인", "두 번째 현실 연결 엔딩"], command: "--location printer_area --action choice:3 --action move:pantry --action choice:3" },
    { title: "현실 연결 힌트 3", type: "reality", status: "smoke 통과", outcome: "hidden_reality_hint_003 엔딩", steps: ["복합기 화이트보드 더미 숫자 확보", "회의실 모서리 단서 확인", "숫자 합계 30과 세 번째 현실 연결 엔딩"], command: "--location printer_area --action choice:1 --action move:dev_office --action move:meeting_room --action choice:1" }
  ]
};
