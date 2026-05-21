window.TUI_ADV_IMPLEMENTATION = window.TUI_ADV_IMPLEMENTATION || {};
window.TUI_ADV_IMPLEMENTATION.overview = {
  title: "escape from the office 구현 지도",
  summary: "TUI 기반 회사 아포칼립스 선택지 게임의 현재 구현 범위, 루트, 콘텐츠, 수정 파일을 한 화면에서 추적한다. 개발자인 내가 follow-up 요청을 만들거나 대사와 루트를 고칠 때 쓰는 내부 위키다.",
  metrics: [
    { label: "위치", value: "16" },
    { label: "인카운터", value: "20" },
    { label: "엔딩", value: "13" },
    { label: "Python 테스트", value: "149" },
    { label: "Web 테스트", value: "32" }
  ],
  highlights: [
    "체력, 정신력, 배터리, 허기, 갈증 기반 GameState와 턴 루프",
    "YAML 기반 위치, 인카운터, 엔딩, 아이템, 업적 로더",
    "Textual TUI smoke, 저장/불러오기, 저장 슬롯 선택과 삭제",
    "현실 연결 힌트 3종과 local secret 안전 경계",
    "Vite 브라우저 fake-TUI shell, terminal 루트 parity mirror core, localStorage 저장, 복합기 pretext/Canvas anomaly panel",
    "YAML 공개 콘텐츠를 브라우저 JSON으로 export/check하는 스크립트",
    "탈출, 정복, 진실, 히든 현실 연결 루트 smoke 검증"
  ],
  nextWork: [
    "실제 Textual 화면에서 저장 슬롯 UX 수동 QA",
    "브라우저 fake-TUI에서 선택 불가 이유, 이동 단축키, 수동 플레이 감각 QA",
    "재난 원인 문서와 생존자, 시스템 제압, 설득 루트 추가",
    "선택 불가 선택지 이유 표시와 색상 테마 위젯 스타일 연결",
    "새 slice가 끝날 때 이 구현 지도 data 파일 갱신"
  ],
  maintenance: [
    "개요와 지표는 data/overview.js에서 갱신",
    "TUI와 엔진 기능은 data/systems.js에 기능 카드 추가",
    "엔딩과 경로는 data/routes.js의 node와 route를 함께 갱신",
    "위치, 인카운터, 아이템, 업적은 data/content.js에서 목록 갱신",
    "시각 구조를 바꿀 때만 index.html 또는 assets/styles.css 수정"
  ],
  sources: [
    { title: "진입 HTML", path: "docs/implementation-map/index.html", description: "단일 브라우저 진입점. 카테고리 데이터 파일을 script로 연결한다." },
    { title: "스타일 토큰", path: "docs/implementation-map/assets/styles.css", description: "Notion식 위키 방향의 vanilla CSS 토큰, 레이아웃, 반응형 규칙." },
    { title: "상호작용", path: "docs/implementation-map/assets/app.js", description: "검색, 필터, 노드 클릭, 섹션 하이라이트를 담당한다." },
    { title: "개요 데이터", path: "docs/implementation-map/data/overview.js", description: "프로젝트 요약, 지표, 다음 작업, 갱신 규칙." },
    { title: "시스템 데이터", path: "docs/implementation-map/data/systems.js", description: "엔진, TUI, 저장, 안전, 테스트 기능 카드." },
    { title: "루트 데이터", path: "docs/implementation-map/data/routes.js", description: "탈출, 정복, 진실, 현실 연결 루트와 연결선 노드, 원본 YAML 기준 배경/대사/전체 선택지/다음 상황." },
    { title: "콘텐츠 데이터", path: "docs/implementation-map/data/content.js", description: "위치, 인카운터, 엔딩, 아이템, 업적 목록." },
    { title: "브라우저 앱", path: "web/", description: "Vite 기반 fake-TUI. YAML 생성 JSON을 읽는 TypeScript mirror core가 대표 terminal 루트, 소모품, 업적, 능력치 판정, 압박 UI를 검증하고 localStorage 저장과 pretext/Canvas anomaly panel을 제공한다." },
    { title: "웹 데이터 export", path: "scripts/export_web_data.py", description: "src/tui_adv/data/*.yaml을 web/src/data/generated/*.json으로 쓰거나 stale 여부를 검사한다." }
  ]
};
