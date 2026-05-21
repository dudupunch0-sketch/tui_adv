window.TUI_ADV_IMPLEMENTATION = window.TUI_ADV_IMPLEMENTATION || {};
window.TUI_ADV_IMPLEMENTATION.systems = {
  features: [
    {
      category: "engine",
      title: "순수 상태 모델과 턴 진행",
      description: "GameState, PlayerState, 위치, 플래그, 단서, 로그를 불변 데이터 중심으로 다루고 이동과 선택지 실행 때 턴을 진행한다.",
      tags: ["GameState", "resources", "turn"],
      files: ["src/tui_adv/game/state.py", "src/tui_adv/game/loop.py"]
    },
    {
      category: "engine",
      title: "인카운터와 선택지 엔진",
      description: "조건, 비용, 효과, 2d6 능력치 판정, 성공/실패 분기를 YAML 콘텐츠와 연결한다.",
      tags: ["encounter", "choice", "2d6"],
      files: ["src/tui_adv/game/encounters.py", "src/tui_adv/data/encounters.yaml"]
    },
    {
      category: "content",
      title: "YAML 콘텐츠 런타임",
      description: "위치, 인카운터, 엔딩, 아이템, 업적을 공개 YAML에서 읽어 기본 런타임 데이터로 사용한다.",
      tags: ["YAML", "loader", "content"],
      files: ["src/tui_adv/game/content.py", "src/tui_adv/data/*.yaml"]
    },
    {
      category: "tui",
      title: "Textual TUI와 smoke snapshot",
      description: "실제 Textual 화면과 테스트 가능한 snapshot 렌더러를 분리해 TUI 레이아웃을 헤드리스로 검증한다.",
      tags: ["Textual", "snapshot", "smoke"],
      files: ["src/tui_adv/tui/app.py", "tests/test_tui_app.py"]
    },
    {
      category: "tui",
      title: "도움말, 인벤토리, 로그 상세 패널",
      description: "?는 상세 도움말, i는 소지품과 단서 상세, l은 전체 로그 상세를 보여준다. 이동 단축키는 예약 키를 건너뛰어도 행동을 잃지 않는다.",
      tags: ["?", "i", "l", "detail"],
      files: ["src/tui_adv/tui/app.py", "tests/test_tui_app.py"]
    },
    {
      category: "save",
      title: "CLI와 TUI 저장, 불러오기",
      description: "JSON 저장 파일을 만들고, CLI와 TUI 모두에서 load/save 조합을 쓸 수 있다. TUI에서는 s 저장, q 종료를 지원한다.",
      tags: ["save", "load", "json"],
      files: ["src/tui_adv/game/save.py", "src/tui_adv/main.py", "src/tui_adv/tui/app.py"]
    },
    {
      category: "save",
      title: "저장 슬롯 목록과 삭제 모드",
      description: "TUI 시작 화면에서 최근 저장 파일을 번호로 고르고, d로 삭제 모드에 들어가 번호로 슬롯을 삭제한다. smoke CLI도 --delete-save-slot을 지원한다.",
      tags: ["slot", "delete", "start screen"],
      files: ["src/tui_adv/tui/app.py", "src/tui_adv/main.py", "tests/test_main.py"]
    },
    {
      category: "tui",
      title: "압박 경고 패널",
      description: "저정신력, 저전원, 고갈증, 고허기 상태에서 선택지 왜곡과 인터넷 단절 위험 같은 경고를 따로 표시한다.",
      tags: ["pressure", "sanity", "battery"],
      files: ["src/tui_adv/tui/status.py", "src/tui_adv/tui/app.py"]
    },
    {
      category: "safety",
      title: "현실 연결 안전 경계",
      description: "공개 저장소에는 중간 힌트와 placeholder만 둔다. 실제 최종 위치는 private 또는 local secret에만 둔다.",
      tags: ["reality link", "local secret", "safety"],
      files: ["src/tui_adv/game/secrets.py", "docs/dev/Reality_Secret_Safety_Checklist.md", "docs/templates/local-secrets.template.yaml"]
    },
    {
      category: "web",
      title: "브라우저 fake-TUI parity shell",
      description: "Vite/TypeScript 앱이 Python YAML에서 생성된 JSON을 읽어 대표 terminal 루트, 소모품, 업적, 능력치 판정, 압박 상태를 mirror core로 실행하고 숫자 키/버튼 입력과 localStorage 저장을 지원한다.",
      tags: ["Vite", "TypeScript", "fake-TUI", "localStorage"],
      files: ["web/src/main.ts", "web/src/game/actions.ts", "web/src/ui/render.ts", "web/src/styles/fake-tui.css"]
    },
    {
      category: "web",
      title: "복합기 pretext/Canvas anomaly panel",
      description: "현실 연결 복합기 장면에서 @chenglou/pretext를 CDN ESM으로 불러오고, 실패 시 안전한 텍스트 fallback을 보여준다. 공개 secret guard가 final_hint 같은 private-only 필드를 차단한다.",
      tags: ["pretext", "Canvas", "reality link", "secret guard"],
      files: ["web/src/effects/printerFlow.ts", "web/src/security/publicSecretGuard.ts", "web/src/data/generated/secrets.example.json"]
    },
    {
      category: "content",
      title: "YAML→브라우저 JSON export",
      description: "scripts/export_web_data.py가 공개 YAML 콘텐츠를 web/src/data/generated/*.json으로 쓰고 --check로 stale 여부와 public secret 안전성을 검증한다.",
      tags: ["export", "JSON", "generated data"],
      files: ["scripts/export_web_data.py", "tests/test_web_data_export.py", "web/src/data/generated/*.json"]
    },
    {
      category: "test",
      title: "대표 루트 smoke와 회귀 테스트",
      description: "탈출, 정복, 진실, 현실 연결 루트를 CLI 경로로 검증하고, 저장 슬롯과 TUI 패널은 단위 테스트로 고정한다. 브라우저 fake-TUI는 Vitest로 terminal route parity, item/achievement/check/pressure mechanics, secret guard, renderer, pretext fallback을 검증한다.",
      tags: ["pytest", "vitest", "route smoke", "regression"],
      files: ["tests/test_cli.py", "tests/test_tui_app.py", "tests/test_secrets.py", "web/src/**/*.test.ts"]
    }
  ]
};
