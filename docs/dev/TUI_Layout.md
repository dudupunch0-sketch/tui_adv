# TUI 레이아웃 설계

## 목적

TUI는 플레이어가 현재 상황, 자원 압박, 선택지를 한눈에 읽고 빠르게 판단하게 해야 한다.
동시에 `escape from the office`의 화면은 일반 게임 HUD가 아니라 이상한 사내 격리 대응 유틸리티처럼 보여야 한다.

## 현재 위치

이 문서는 기존 Python/Textual 및 legacy 패널형 TUI의 레이아웃 기록이다. 앞으로 terminal renderer 작업은 이 패널 구성을 그대로 늘리는 것이 아니라 `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`의 **SuperLightTUI terminal renderer** 방향을 따른다. 즉, terminal은 fallback이지만 단순 debug dump가 아니라 `ScenePage`를 SuperLightTUI layout, ASCII/Unicode visual card, terminal-native GlyphFX로 표시하는 별도 renderer다.

따라서 화면은 명확한 패널과 로그 중심의 장점을 유지하되, 새 terminal 구현은 Web Storybook과 같은 semantic page contract를 공유해야 한다. 상태/선택지/알림은 사내 시스템, 진단 로그, 내부망 메시지처럼 표현한다.
구체적인 글리치, 선택지 오염, 입력 왜곡 규칙은 `docs/design/UI_Rules.md`와 `docs/design/TUI_Storybook_GlyphFX_Concept.md`를 따른다.

## SuperLightTUI terminal renderer target

새 terminal renderer는 이 문서 아래쪽의 legacy Textual 3-column layout을 그대로 복제하지 않는다. 목표는 `ScenePage`를 SuperLightTUI cell UI로 번역하는 것이다.

```text
ScenePage
  -> SuperLightTUI app loop
  -> status/location/chapter line
  -> ASCII/Unicode visual card
  -> body/dialogue page
  -> choices and blocked reasons
  -> recent history / inventory / achievement summary
  -> terminal-native GlyphFX when EffectCue exists
```

### ScenePage region mapping

| `ScenePage` field | Terminal region | 표시 원칙 |
|---|---|---|
| `mode` | title/status line | encounter/movement/ending을 명확히 구분 |
| `location` | top line or location chip | 현재 위치와 위험도는 짧게 |
| `chapter_label` | top line | 턴/격리 단계 label |
| `status_summary` | compact status strip | 숫자보다 진단형 band 우선 |
| `visual` | visual card | `visual.id` -> ASCII/Unicode/ANSI card |
| `body_blocks` | main story panel | 가장 넓은 영역, 가독성 우선 |
| `dialogue_entries` | speaker/body lines | speaker prefix는 짧게 |
| `actions` | choices panel | 번호는 고정, id는 내부적으로 보존 |
| `blocked_actions` | disabled choices or footnote | 이유를 짧게 표시 |
| `history_entries` | recent history panel | 최근 3-5개, 상세는 후속 |
| `pressure_cues` | warning strip | 저정신력/고갈증/저전원 cue |
| `effect_cues` | GlyphFX layer | stable terms를 절대 잃지 않음 |

### Target page skeleton

```text
┌ escape from the office ─ 격리 3턴 ─ 복합기 구역 ─ 집중도: 불안정 ┐
│ ╭──────── visual: printer_anomaly ────────╮                    │
│ │        ________                           │                    │
│ │  ____ / PRINT /__                         │                    │
│ │ |복합기|  ▒▒▒  |   비상계단               │                    │
│ ╰──────────────────────────────────────────╯                    │
│ 시스템 복합기: 아직 하지 않은 선택이 출력되고 있습니다.          │
│ 종이는 따뜻하다. 일부 글자는 흔들리지만 세 단어만 남는다.        │
│                                                                  │
│ 1. 출력물을 챙긴다                       choice:take_printout    │
│ 2. 토너 덮개를 연다                     choice:open_toner       │
│ 3. 아무것도 하지 않고 물러난다           move:hallway            │
├ 최근 기록 ───────────────────────────────────────────────────────┤
│ [warning] 출력 큐가 현재 시각보다 3분 늦게 도착했습니다.         │
└──────────────────────────────────────────────────────────────────┘
```

### SuperLightTUI primitive usage

Implementation slice에서 확인할 사용 지점:

- 완료: `escape-terminal --app`(`--app`)이 `slt::run_with(...)` 기반 full-screen app loop를 연다.
- 완료: `RunConfig::tick_rate(...)` / `max_fps(60)`로 frame control을 설정한다.
- 완료: flexbox-style layout으로 page region을 배치한다.
- 완료: normal widget tree로 부족한 GlyphFX는 raw draw / cell-grid access 사용으로 보완한다.
- 완료: `ui.tick()` 또는 `--app-smoke --tick`의 tick 값을 animation signal로 사용한다.
- 완료: `--app-smoke --tick` headless/test backend로 app-frame snapshot smoke를 작성했다.

### Visual card catalog baseline

| `visual.id` | Terminal card |
|---|---|
| `opening_messenger` | 메신저 창/말풍선형 text card |
| `printer_anomaly` | ASCII 복합기 + 흔들리는 출력지 + stable terms |
| `office_corridor_static` | 복도/비상등/문 표식 Unicode card |
| unknown | title/alt text만 있는 safe placeholder card |

Inline image protocol은 baseline이 아니다. Kitty/Sixel/iTerm2 이미지는 나중에 capability detection 후 optional로만 검토한다.

### Terminal GlyphFX rules

- `EffectCue::GlyphAnomaly`는 terminal cell에서만 표현한다. Core에는 SuperLightTUI나 ANSI style object를 넣지 않는다.
- `stable_terms`는 animation 중에도 사라지지 않거나, animation 후 반드시 같은 위치/명도에서 복구된다.
- CJK width를 고려해 한글 단어를 cell 단위로 찢는 효과는 제한한다.
- reduced-motion 또는 dumb terminal fallback에서는 `fallback_text`와 stable terms를 정적 텍스트로 보여준다.
- Animation tick은 입력 처리를 막지 않는다. 선택 번호, `?`, `s`, `q`는 항상 읽힌다.

### Action parity

- Terminal 번호키는 현재 `ScenePage.actions` 배열의 순서에만 매핑한다.
- 이동 단축키가 추가되어도 내부 실행은 `move:<location_id>` action id여야 한다.
- Terminal-only shortcut은 save/help/quit 같은 renderer control에만 허용한다.
- Web과 terminal smoke는 같은 seed/content/action sequence에서 같은 action id 목록을 보여야 한다.

## 기본 화면 구성

권장 최소 터미널 크기:

```text
100 columns x 32 rows
```

작은 화면에서는 미니맵과 긴 인벤토리 표시를 접고, 이벤트/선택지/상태를 우선한다.

## 전체 레이아웃

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ escape from the office | 23:17 | 개발팀 사무실 | 사내망: 불안정 | 위험도: 2 │
├──────────────────────┬───────────────────────────────────────┬───────────────┤
│ 위치                 │ 이벤트                                │ LOCAL STATUS  │
│                      │                                       │ 신체: 정상    │
│ > 개발팀 사무실      │ 빌드는 아직 끝나지 않았다.             │ 집중: 불안정  │
│   복도               │ 사내 메신저에는 아무도 답하지 않는다.  │ 전원: 34%     │
│   탕비실             │                                       │ 식수: 없음    │
│   회의실             │ 선택                                  │ 영양: 부족    │
│                      │ 1. 메시지를 확인한다                  │               │
│                      │ 2. 복도로 나간다                      │ 소지품        │
│                      │ 3. 주변 책상을 조사한다               │ - 사원증      │
│                      │                                       │ - 출력물      │
├──────────────────────┴───────────────────────────────────────┴───────────────┤
│ 로그                                                                         │
│ 23:15 사무실이 너무 조용하다.                                                │
│ 23:16 사내망 알림이 한 번 깜빡였다.                                         │
└──────────────────────────────────────────────────────────────────────────────┘
```

## 패널 역할

### 상단 상태바

표시:

- 게임 제목
- 게임 내 시간 또는 턴
- 현재 위치
- 사내망 상태
- 위험도

사내망 상태 후보:

- 차단
- 불안정
- 내부망만
- 알 수 없음
- 비정상 연결

### 왼쪽 위치 패널

1차 구현:

- 현재 위치 표시
- 이동 가능한 위치 목록 표시

확장:

- ASCII 미니맵
- 잠긴 위치 표시
- 위험한 위치 색상 표시

### 중앙 이벤트 패널

가장 중요한 패널이다.

표시:

- 현재 인카운터 제목
- 이벤트 본문
- 선택지 목록
- 선택 불가 이유

선택지 표시 원칙:

```text
1. 메시지를 확인한다        [배터리 -3]
2. 휴대폰을 끈다            [정신력 +2]
3. 사내망 로그를 추적한다   [필요: 배터리 8 이상]
```

조건 미충족 선택지는 두 가지 방식 중 하나로 처리한다.

1차 기본값:

- 표시하되 비활성 처리하고 이유를 보여준다.

이유:

- 플레이어가 “무엇이 부족한지” 배울 수 있다.
- 히든 루트의 존재를 과하게 숨기지 않는다.

히든 선택지는 조건 만족 전까지 숨길 수 있다.

### 오른쪽 상태 패널

엔진 내부 상태는 숫자로 유지하지만, 기본 플레이 화면에서는 노골적인 HP bar처럼 보이지 않게 한다.
상태는 사내 진단/격리 대응 유틸리티처럼 표현한다.

기본 표시 예:

```text
[LOCAL STATUS]
신체 반응: 정상 범위
집중도: 불안정
단말기 전원: 34%
최근 식수 기록: 없음
영양 상태: 부족
```

상태 방향성:

- 체력/정신력/배터리는 높을수록 좋다.
- 허기/갈증은 높을수록 나쁘다.
- 허기/갈증은 숫자보다 “식수 기록 없음”, “탈수 의심”, “영양 부족” 같은 진단 문구로 먼저 보여준다.
- 정확한 숫자는 debug mode, 상세 상태 화면, 테스트 로그에서만 기본 표시한다.

압박 경고 패널:

- 저체력, 저정신력, 저전원, 고갈증, 고허기가 활성화되면 `[압박 경고]` 패널을 표시한다.
- 경고 문구는 숫자보다 게임 세계 안의 사내 시스템 진단처럼 작성한다.
- 저정신력은 선택지 왜곡, 고갈증은 정수기 환청, 저전원은 제한된 인터넷 단절 위험을 직접 알려준다.

색상 초안:

- 좋음: green
- 주의: yellow
- 위험: red
- 괴현상/왜곡: magenta 또는 purple
- 배터리/사내망: cyan

현재 Textual 구현:

- `#game-grid` 컨테이너가 2열 grid를 구성한다.
- `panel-status`, `panel-controls`, `panel-inventory`, `panel-main`, `panel-log`는 분리된 Textual `Static` 위젯으로 mount된다.
- 모든 패널은 `office-panel` 클래스를 공유하고, 핵심 이벤트/로그 패널은 `office-panel--wide`로 2열을 가로지른다.
- 터미널 테마 CSS는 어두운 배경, 녹색 테두리, cyan 강조선을 사용해 사내 격리 대응 유틸리티 느낌을 낸다.
- Textual CSS 호환성을 위해 `@media` 규칙은 쓰지 않고, 작은 화면 검증은 별도 수동 QA 항목으로 남긴다.

### 아이템/단서 영역

오른쪽 상태 패널 아래 또는 별도 탭에 표시한다.

1차 표시:

- 아이템 최대 5개까지 표시
- 단서 최대 3개까지 표시
- 더 많으면 `+N more` 표시

확장:

- 인벤토리 화면
- 단서 로그 화면
- 히든 루트 진행도 화면

### 하단 로그 패널

최근 N개의 로그를 보여준다.

1차 구현:

- 최근 5줄
- 스크롤 없음

확장:

- 로그 스크롤
- 중요 로그 고정
- 단서 로그 필터

## 입력 설계

1차 기본 키:

| 키 | 기능 |
|---|---|
| 1-9 | 현재 선택지/행동 실행, 시작 화면에서는 저장 슬롯 불러오기 |
| a/d/f/... | 이동 가능한 위치 단축키 |
| ? | 도움말 패널 새로고침 |
| s | 현재 상태를 지정된 저장 파일에 저장 |
| q | 종료 |

현재 구현에서는 `--tui --save <path>`로 저장 파일을 지정하면 숫자 행동 또는 이동 단축키 실행 후 자동 저장하고, `s` 키로 즉시 저장한다.
`--tui --load <path>`는 기존 JSON 저장 파일에서 이어 시작한다.
`--tui-smoke --save <path>`와 `--tui --save <path>`는 저장 경로의 같은 디렉터리에서 최근 JSON 저장 파일을 읽어 `[저장 파일 목록]` 패널에 표시한다. 손상된 JSON은 목록에 “읽기 실패”로 남긴다.
저장 슬롯이 있는 `--tui --save <path>` 시작 화면에서는 숫자로 슬롯을 불러오고, `n`으로 새 게임을 시작한다.

도움말 패널:

- 항상 현재 입력 모드를 짧게 표시한다.
- 플레이 중에는 `숫자: 현재 선택/행동 실행`, `?: 도움말 / s: 저장 / q: 종료`를 표시한다.
- 이동 행동이 열려 있으면 `a=개발팀 사무실`처럼 이동 단축키를 함께 표시한다.

## 화면 상태

### 시작 화면

표시:

- 제목
- 짧은 설명
- 새 게임 시작
- 저장 슬롯 목록과 숫자 불러오기
- `n` 새 게임, `q` 종료

### 플레이 화면

기본 화면.

### 아이템 사용 화면

아이템 목록과 사용 가능한 아이템을 보여준다.

### 로그 상세 화면

최근 로그보다 긴 히스토리를 보여준다.

### 엔딩 화면

엔딩 제목, 본문, 도달 조건 요약, 다시 시작/종료를 보여준다.

## 정신력 낮음 연출

정신력 40 미만부터 약한 왜곡 가능.
정신력 30 미만부터 명확한 왜곡 가능.

왜곡 예:

- 선택지 텍스트 반복
- 일부 로그가 사라졌다 다시 나타남
- 현재 위치명이 잠시 다른 이름으로 표시됨
- 사내망 상태가 의미 없는 문자열로 표시됨

주의:

- 조작을 방해할 정도로 과하면 안 된다.
- 중요한 선택의 결과는 최소한 사후 로그로 이해 가능해야 한다.
- 히든 루트 단서와 단순 장식 왜곡은 구분 가능한 패턴이 있어야 한다.

## 갈증 높음 연출

갈증 60 이상:

- 상태 라벨에 “탈수” 표시
- 일부 설명에 환각성 문장 추가

갈증 80 이상:

- 로그에 짧은 환청/착각 문구 가능
- 위험 선택지 성공률 페널티 표시

예:

```text
갈증이 심하다. 회의실 유리벽 너머로 정수기 물 흐르는 소리가 들린다.
이 층에는 정수기가 없다.
```

## 현실 연결 힌트 표시

현실 연결 힌트는 일반 보상과 다르게 강조한다.

표시 원칙:

- “현실”이라는 단어를 너무 빨리 쓰지 않는다.
- 처음에는 게임 속 장소처럼 보이게 한다.
- 힌트가 중간 강도임을 유지한다.
- 실제 최종 위치는 private/local 데이터가 있을 때만 표시한다.

공개 데이터만 있을 때:

```text
이 힌트는 게임 안의 장소만을 말하는 것 같지 않다.
```

private/local 데이터가 있을 때:

```text
로컬 비밀 데이터가 활성화되어 더 구체적인 힌트를 표시한다.
```

## 접근성/가독성

- 색상만으로 상태를 전달하지 않는다. 라벨을 함께 표시한다.
- 긴 문장은 자동 줄바꿈한다.
- 선택지 번호는 항상 고정한다.
- 실패/비활성 선택지는 이유를 텍스트로 표시한다.
- 한글 폭 처리 문제가 생길 수 있으므로 TUI 테스트에서 한글 정렬을 확인한다.

## 상세 패널과 저장 슬롯 관리

- 기본 화면은 `[소지품]`, `[단서]`, `[최근 로그]`를 짧게 보여준다.
- `i`는 소지품/단서 상세 패널을 열어 모든 항목과 아이템 설명을 보여준다.
- `l`은 최근 로그 상세 패널을 열어 축약되지 않은 로그를 번호와 함께 보여준다.
- `?`는 상세 도움말 패널을 열어 숫자/이동/저장/종료/상세 키를 설명한다.
- 저장 슬롯 시작 화면에서 `d`는 삭제 모드로 전환하고, 다음 숫자 입력은 해당 저장 파일을 삭제한다.
- 삭제 모드에서도 실제 게임 선택지 숫자와 혼동되지 않도록 `[시작]` 패널의 문구를 `숫자: 저장 파일 삭제`로 바꾼다.
- 저장 슬롯 이름 변경은 아직 구현하지 않고 `docs/dev/Save_Slot_UX.md`의 후보안을 기준으로 다음 실시간 UI/UX 점검에서 문구, 키, 패널 위치를 확정한다.

## 구현 순서

1. 상태바 + 중앙 이벤트/선택지 + 오른쪽 상태 패널만 구현
2. 로그 패널 추가
3. 위치 패널 추가
4. 아이템/단서 표시 추가
5. 시작/엔딩 화면 추가
6. 정신력/갈증 왜곡 연출 추가
7. 인벤토리/로그 상세 화면 추가
