# Mobile Ink Storybook UI Contract (수묵 서책)

Status: active Web Storybook visual contract. 2026-07-11부터
`docs/design/Mobile_Pixel_Storybook_UI.md`(픽셀 게임북 board contract)를 대체한다.

## Goal

Web Storybook은 게임 HUD가 달린 앱처럼 보이면 안 된다. 기본 화면은
**ordered story flow가 놓이는 오래된 수묵 서책의 한 쪽**이며, `천기록`은 실제
세계관 장치가 개입하는 특수 block/surface에서만 표시한다. 모든 그림은
본문 흐름에 놓이는 수묵 삽화다. 화면 문법은 특정 상용 게임의 template을 따라 하지
않는다 — 이 문서가 유일한 기준이다. 미감/토큰/삽화 규칙의 상세는
`fable_ui_step1_2607111330.md` §2(수묵 천기록 디자인 언어)를 따른다.

시각 정체성 한 줄: **모바일 세로형 수묵 서책 board**.

## Core screen grammar — 서책 몰입형 + 정보 드로어

기본 화면(책 쪽):

```text
[folio 헤더]                     — 쪽 표시(예: 天記錄 · 三) + 장면 인장. 장식 최소.
[본문 흐름]                      — 장면 제목(h1) → 서사 텍스트 → 수묵 삽화(본문 사이 삽입)
                                   → 결과/보상/단서는 본문 흐름 안의 강조 문장
[선택지]                         — ✥ bullet + 번호 + 큰 문장형 row (책의 연속으로 보이게)
[각주 스트립 (하단 고정)]         — 몸/마음 요약 먹점 + 위험 실(붉은 선) + 드로어 열기
```

정보 드로어(바텀시트, 각주 스트립 탭으로 개폐):

```text
[상태 상세]   core가 준 resource band text (숫자는 보조 표기)
[소지품]      inventory_summary → 한국어 라벨
[업적]        achievement_summary → 한국어 라벨, 신규는 금박 표식
[기록]        history_entries (data-region="history")
[메뉴]        처음 화면 / 포기하기(확인 단계 필수) / 소리 / 연출
```

핵심 원칙:

- 기본 화면에는 게임 크롬을 두지 않는다. 고정 상단 HUD, 초상, 스탯 그리드,
  아이콘 dock을 기본 화면에 노출하지 않는다.
- 상태 정보는 (1) 항상 보이는 한 줄 각주 요약, (2) 원할 때 여는 드로어 상세의
  2단계로 제공한다. 위험(danger)이 높으면 각주 스트립의 붉은 실이 굵어지고
  경고 문구(core `pressure_cues`/`warnings`)가 본문에 붉은 각주로 삽입된다.
- 세로쓰기(vertical writing-mode)는 장식 한자 1~2자를 제외하고 금지한다.
  영어 등 다국어 본문에서도 성립해야 하는 layout이다.
- `docs/design/Event_Stage_Content_Model.md`의 Stage/ContentBlock 순서가 화면
  순서다. 제목/천기록/결과/일러스트/선택지의 고정 5영역으로 재배치하지 않는다.
- StoryStage와 ResultStage의 block을 연속해서 읽다가 ChoiceStage에서만 입력을
  기다린다. 한 Event 안에 여러 ChoiceStage가 올 수 있다.

## Layout contract

- 모바일 세로형/narrow browser가 1차 기준. desktop에서도 centered portrait
  board(최대 폭 ~720px 내외의 한 쪽)를 유지하고 사이드바 레이아웃으로 바꾸지 않는다.
- 본문 서체는 `@fontsource/noto-serif-kr` (400/700/900) — OS 폰트가 없어도
  한국어가 tofu 없이 렌더되어야 한다.
- 종이 배경 위에 텍스트가 직접 놓인다. dark card, 웹 대시보드 그리드 금지.
- text readability 최우선. 삽화·장식·전환이 본문 가독성을 해치면 안 된다.

## DOM region contract (QA 계약 — 클래스/데이터 속성은 형태가 바뀌어도 유지)

기존 셀렉터는 의미 컨테이너로 유지하고 역할만 재배치한다:

| 셀렉터 | 새 역할 |
|---|---|
| `.storybook-shell[data-renderer="web-storybook"]` | 책 쪽 전체 |
| `.storybook-hud[data-region="status"]` | 하단 각주 스트립 (상태 요약 + 드로어 트리거) |
| `.story-progress-rail` | 각주 스트립 안의 위험 실 (`data-danger-band` 유지) |
| `[data-region="visual"]` | ordered content 안의 각 수묵 삽화 `<figure>` (복수 가능) |
| `[data-region="body"]` | Stage/ContentBlock 순서를 보존하는 서사 흐름 |
| `[data-region="choices"]` + `button.choice-row[data-action-id]` + `.choice-bullet` | 현재 ChoiceStage의 문장형 선택지 |
| `.storybook-dock` | 정보 드로어(바텀시트) 컨테이너 |
| `[data-region="history"]` | 드로어 안의 기록 목록 |

- 선택지는 `<button>` semantics, `data-action-id`, `data-action-kind`, 숫자키
  실행 계약을 유지한다. blocked action은 숨기지 않고 사유를 보여준다.
- Renderer는 status를 계산하지 않는다. `ScenePage.status_summary`,
  `pressure_cues`, `inventory_summary`, `achievement_summary`를 표시만 한다.
- unknown visual id는 safe placeholder(수묵 generic 구도 + alt 각주)를 보여주고
  action을 drop하지 않는다.
- GlyphFX `stable_terms`/`fallback_text`는 reduced-motion/no-canvas에서도 읽힌다.

## Renderer boundary

- Rust GameCore가 action eligibility, outcome, ending, achievement의 truth다.
- `ScenePage`에는 CSS class, pixel coordinate, DOM selector, Canvas command,
  image path를 넣지 않는다.
- 외부 이미지/타사 게임 asset을 참조하거나 도입하지 않는다. 삽화는 전부
  코드(inline SVG)로 저작하며, 인물은 이목구비 없는 먹 실루엣으로 그린다.
  변주는 visual id 해시 시드로만 만든다 (`Math.random()` 금지).
- public UI/docs/generated data에는 실제 회사명/개인정보/private hint를 넣지 않는다.

## 일러스트 에셋과 밀도 계약

### 일러스트 에셋 (Illustration Assets)
- **배치**: IllustrationBlock은 StoryStage/ResultStage의 위·중간·끝 어디에도 배치할 수 있다. 일반 Event는 1개가 기본이며 특별/보스 Event는 최대 3개를 권장한다. 고정 visual slot 하나를 전제로 하지 않는다.
- **미완성 에셋**: stable `visual_id`와 Event 이름 기반 alt를 유지하고 safe placeholder를 표시한다. `이벤트 이름.png`를 실제 경로처럼 만들지 않는다.
- **에셋 매니페스트**: `artManifest.ts`에 등록된 `visual_id`는 생성된 `.webp` 애니메이션풍 일러스트 에셋으로 표시되며, 등록되지 않은 `visual_id`는 generic SVG 또는 location-based SVG 폴백으로 렌더링된다.
- **주인공 묘사 최소화 정책 (No Protagonist Appearance)**: 주인공(현대 회사원)의 구체적인 모습(셔츠, 넥타이, 사원증, 얼굴 등)이 무협 세계관 내 일러스트에서 직접적으로 노출되는 것을 철저히 배제한다. 주인공이 관여된 장면은 주로 1인칭 시점(First-person POV)으로 표현하거나, 인물이 원경에 꼭 필요할 경우 세부 묘사가 없는 미세한 형태의 먹색 실루엣(삿갓/도포 차림 등)으로만 묘사한다.
- **스타일 레퍼런스 (Style References)**: 화풍 및 인물 묘사 축소 기준에 대해 다음 레퍼런스 이미지를 참조한다.
  - [수묵 용 일러스트 레퍼런스](file:///C:/Users/82105/.gemini/antigravity/worktrees/tui-adv/enable-wsl-worktree-support/docs/design/style_ref_dragon.png)
  - [수묵 영물 일러스트 레퍼런스](file:///C:/Users/82105/.gemini/antigravity/worktrees/tui-adv/enable-wsl-worktree-support/docs/design/style_ref_beasts.png)
  - [회사원 펜선 뒷모습 레퍼런스](file:///C:/Users/82105/.gemini/antigravity/worktrees/tui-adv/enable-wsl-worktree-support/docs/design/style_ref_sketch_back.png)
  - [회사원 펜선 독서 옆모습 레퍼런스](file:///C:/Users/82105/.gemini/antigravity/worktrees/tui-adv/enable-wsl-worktree-support/docs/design/style_ref_sketch_read.png)
- **규격 및 포맷**: 규격은 5:3 비율(1120x672, `title_hero`만 3:5 세로 1120x1867)을 준수하며, WEBP 포맷으로 변환 및 최적화하여 각 파일은 150KB 이하여야 한다.
- **삽지 프레임**: 일러스트는 한지 백그라운드 톤과 자연스럽게 결합되도록 얇은 먹 테두리(1px `--line-hard`), 한지 매트(5px 패딩, `--paper-lit`), 아주 약한 세피아 톤 필터 (`filter: saturate(0.92) sepia(0.06)`) 보정이 적용된다.
- **인장 유지**: 일러스트가 활성화되어 있을 때도 우하단의 수묵 인장(seal)은 그림 위에 오버레이로 유지된다.
- **에셋 라이선스**: 디렉터리의 모든 이미지는 프로젝트가 AI 생성으로 직접 제작한 자산이며, 타사 IP 및 외부 게임/작품의 아트를 침해하지 않는다.

### 밀도 계약 (Density Agreement)
- **한 화면 목표**: 390x844 모바일 뷰포트 해상도에서 "본문 제목 + 서사 4~6줄 + 삽화 + 선택지 2개 + 하단 스트립"이 스크롤 없이 또는 단 한 번의 짧은 스크롤로 한눈에 들어와야 한다.
- **상한 제약**: 삽화 영역(`figure[data-region="visual"]`)은 최대 `36dvh` 높이 상한을 준수하며 `object-fit: cover` 및 `preserveAspectRatio="xMidYMid slice"`로 크롭된다.
- **요약 로그 및 칩**: 결과 로그(`.story-result-log`)는 최근 1행 및 필수 업적/아이템 행만 노출하도록 압축하며, GlyphFX stable-terms 칩은 figcaption 옆 한 줄 인라인으로 축소한다.
- **모바일 간격**: 560px 이하 해상도에서 본문 폰트 `1rem/1.7`, 문단 간격 `0.7em`, 선택지 `min-height: 48px` 및 padding 축소로 조밀한 밀도를 확보한다.

## 전투 관전 표면 (Combat Spectator Surface, Wave 3 Step 1d-2·1d-3)

`page.combat`(`CombatSpectatorPage`)이 있을 때만 `.storybook-page` 안에
`renderCombatStage()`가 조립하는 별도 block이다. 없으면(기존 52개 인카운터)
빈 문자열이며 래퍼 요소·클래스·`data-*` 속성도 전혀 추가되지 않는다
(`web/src/ui/storybook/render.test.ts`의 `I5: emits no combat markup at all
when page.combat is absent`).

DOM은 **마지막 `view.frames` 항목**(정지 프레임)이다 — Wave 3 Step 1d-3부터는
그 위에 `@media (prefers-reduced-motion: no-preference)` 안에서만 재생 연출
(모션)이 얹힌다. `prefers-reduced-motion: reduce`에서는 이 media block 전체가
적용되지 않으므로 1d-2의 정지 프레임이 그대로 최종 상태다 — 재생 연출 계약은
아래 별도 절에 있다.

### 70:30 레이아웃

`.combat-stage`는 `grid-template-rows: minmax(0, 70fr) minmax(0, 30fr)`로
보드(상단 65~75% 범위의 중앙값)와 핵심 로그(25~35% 범위의 중앙값)를
70:30으로 고정한다. 전투 종료 보고서(`.combat-report`)는 이 비율 밖,
표면 **아래**에 일반 흐름으로 놓인다. 기존 `.storybook-shell`의
`grid-template-rows: auto minmax(0, 1fr) auto`, `.game-viewport`,
`.game-topbar`, `.storybook-hud`, `.storybook-dock` 규칙은 무수정이다.

### cue 5종 대응표 (terminal과 짝 — 한쪽만 고치지 말 것)

`docs/dev/TUI_Layout.md`의 "전투 관전(Combat Spectator) 표시 계약" 절이
쓰는 terminal 표식(`>`/`<`/`~`/`!`/`x`)과 다음 표가 짝을 이룬다:

| `CombatSpectatorCue` | 정본 연출 의미 | terminal 표식 | web 글리프 | web 색 토큰 |
|---|---|---|---|---|
| `attack` | 짧은 전진/복귀 | `>` | `攻` | `--seal-red` |
| `hit` | 밀림/진동 | `<` | `打` | `--seal-red-lit` |
| `evade` | 측면 이동 | `~` | `避` | `--jade` |
| `balance_broken` | 흔들림/기울어짐 | `!` | `傾` | `--gold-leaf` |
| `incapacitated` | 흐려짐/표식 | `x` | `倒` | `--ink-faded` |

색·아이콘 동기화는 core에서 대응이 유도되는 짝만 연결한다:
`combat.log.damage_applied` 로그 줄만 Hit cue와 같은 색·글리프를 쓴다
(`DamageApplied` 사건 하나에서 로그와 cue가 함께 나오기 때문에 대응이
증명 가능하다). 나머지 5개 template id는 대응하는 cue가 core에 없으므로
중립 잉크색으로 두고 대응을 발명하지 않는다.

### 접근성 대체

- 보드(`[data-region="combat-board"]`, `role="img"`)는 그래픽 투영이므로
  동등한 semantic `<table class="combat-board__table sr-only">`(말 id /
  진영 / 좌표 / **참전** / cue)를 항상 함께 렌더한다.
- **`piece.active`를 "생존"으로 표시하지 않는다.** 정본 09의 "활성
  전투"(완전 시뮬레이션) 참가 여부이며 authoring 정적 값에서 온다 — 실측에서
  체력이 0이 된 뒤에도 계속 `true`였고 전투불능은 `Incapacitated` cue로만
  나타났다. 생존·전투불능은 보고서의 `survivor_ids`/`defeated_ids`가 소유한다.
- 진영은 색만으로 구분하지 않는다 — 아군/적군 각각 고유 글리프(`我`/`敵`),
  채움 대비(먹 채움 vs 짙은 종이 채움), 윤곽선 종류(실선 vs 2px 파선)를
  함께 쓴다. 적 말을 종이색으로 채우면 종이 배경과 대비가 거의 없어
  글리프가 읽히지 않는다.
- `@media (forced-colors: active)`에서 `CanvasText`/`Canvas`/`Highlight`/
  `GrayText` 시스템 색 키워드로 대체해 진영·cue 구분이 고대비 모드에서도
  사라지지 않는다.
- 로그 영역(`[data-region="combat-log"]`)은 `aria-label`만 갖는다 —
  Wave 3 Step 1d-3이 재생 노출(tick 시각 opacity 등장)을 얹은 뒤에도
  `aria-live`는 쓰지 않는다: 초당 여러 줄이 붙으면 스크린리더가 로그에
  도배당한다(정본 13의 "로그 도배를 막는다"와 같은 취지). 노출 전에도
  DOM에서 제거하지 않으므로(아래 재생 연출 계약 참고) 전체 로그는 항상
  읽을 수 있다.

### 로그·보고서 계약

- `core_log`만 문장화한다(`combatLogTemplates.ts`, 6개 template id, terminal
  `combat_log_template_line`과 글자 단위 일치). 표시 상한(`WEB_CORE_LOG_LIMIT
  = 40`)을 넘으면 `…(생략 N줄)`을 반드시 출력한다. 로그 영역 메타 줄
  (`.combat-log__meta`)은 `combat.report`가 있을 때만 "아래 전체 로그
  열람에서 확인할 수 있다"고 말한다 — 전투 진행 중에는 열람 가능하다고
  주장하지 않는다(아래 "전체 로그 열람" 절, I2).
- 알 수 없는 template_id는 버리지 않고 `data-log-unknown="true"` + id를
  노출하는 fallback 문장을 만든다.
- `combat.report`가 없으면(전투 진행 중) `.combat-report` 섹션 자체를
  만들지 않는다. `top_damage_dealt_id`/`top_damage_taken_id`/
  `decisive_tick`이 `null`이면 그 줄 자체를 생략한다("없음" 대체 문구도
  없음). 금지 항목(전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합
  MVP·이전 전투 비교)은 문구도 계산도 만들지 않는다.
- fingerprint를 표시하는 요소에는 `simulation_version`을 같은 요소 안에
  둔다(정본 03: 결정성은 같은 `simulation_version` 안에서만 보장).

### 전체 로그(`full_log`) 열람 (Wave 3 Step 1d-4 continue)

정본 07/13: "전체 로그와 상세 수치는 일시정지 또는 전투 종료 뒤 별도로
열람한다." 이 슬라이스는 일시정지 흐름을 만들지 않으므로, 열람 진입점은
**전투 종료 뒤(`combat.report`가 `Some`)에만** 존재한다(`renderCombatStage.ts`의
`renderCombatFullLog`, I2). 전투 진행 중에는 이 섹션 자체가 출력에 없다 —
"나중에 열람 가능"이라는 미래형 서술은 로그 메타 줄이 대신 한다(위 절 참고).

- `view.full_log`만 읽는다(I1) — core가 이미 누설 차단
  (`AttackRoll`/`EffectSuppressed` 제외, Hidden/Conditional 효과 id 마스킹)을
  마친 배열이라 resolution·execution 레벨에는 접근하지 않는다.
- 상한이 없다(I4) — `full_log`의 모든 줄을 `<ol class="combat-full-log__list">`
  에 낸다. 넘치는 길이는 `storybook.css`의 내부 스크롤
  (`max-block-size` + `overflow-y: auto`)이 처리하며 DOM에서 행을 빼지
  않는다. 핵심 로그의 `WEB_CORE_LOG_LIMIT = 40` 상한과는 별개이며 그대로
  둔다.
- `entry.importance`(`routine`/`important`/`decisive`)를 그대로 쓴다(I5) —
  renderer가 중요도를 다시 판단하지 않는다. 각 줄은 `data-importance`와
  정본 13의 한국어 라벨(`일반`/`중요`/`결정적`)을 함께 낸다(I9: 색만으로
  전달하지 않는다).
- `core_log`는 `full_log`의 `importance >= important` 부분집합이므로(정본
  13), 그 조건을 그대로 판정에 써서 `data-in-core-log="true"`를 붙인다(I6).
  다만 **대응 관계는 목록 앞에서 한 번만 문장으로 밝힌다**
  (`.combat-full-log__legend`). 줄마다 "핵심 로그에도 있음" 배지를 붙였을
  때 실측에서 64줄 중 32줄이 두 줄로 늘어나 목록을 훑을 수 없었고, 중요도
  칩(`중요`/`결정적`)이 이미 같은 것을 말하고 있었다 — core_log가 정확히
  `importance >= 중요`이기 때문이다. 배지를 빼자 390px에서 줄 높이가 43px로
  균일해지고 목록 전체 높이가 3551→3039px로 줄었다.
  `states the core-log correspondence once, not on every row`가 배지
  재도입을 잡는다.
- 문장은 `combatLogTemplateLine`을 그대로 쓴다(I3) — 새 문장 형식을 만들지
  않는다. 알 수 없는 template_id도 버리지 않고 core_log와 같은
  `data-log-unknown="true"` fallback을 낸다.
- tick·sequence를 `t{tick}·{sequence}` 형식으로 함께 보여 순서를 읽을 수
  있게 한다.
- 네이티브 `<details>`/`<summary>`를 쓴다(I9) — 커스텀 토글을 만들지 않는다.
  `<summary>`는 건수를 포함한다("전체 로그 N건 열람"). 로그 목록은 `<ol>`로
  순서를 의미로 표현한다.
- 이 섹션은 `.combat-stage`의 board:log 70:30 그리드 **밖**, 보고서와 같은
  층(표면 아래 일반 흐름)에 둔다(I8) — 그리드 행을 건드리지 않는다.
- 애니메이션·트랜지션을 추가하지 않는다(I10) — `<details>` 열림 자체도
  네이티브 동작이며 트랜지션을 걸지 않는다.
- **terminal(SuperLightTUI)은 여전히 개수만 표시한다** — terminal 쪽 열람
  UI는 이 슬라이스의 범위가 아니고 별도 슬라이스다. 이 web/terminal 비대칭은
  의도적이다.
- **일시정지 중 열람은 아직 없다** — 일시정지 흐름 자체가 별도 슬라이스
  (Step 2c 개입 흐름과 얽혀 있다). 전투 종료 뒤 열람만 이 슬라이스의
  범위다.
- 실화면 QA 메모: 이 인카운터는 여전히 `combat_spectator_preview_unlocked`
  게이트 뒤에 있어 `npm run qa:storybook:visual`(시작 화면에서 실제 플레이로
  진입)이 도달하지 못한다. 대신 `renderCombatStage()` 출력을 실제 빌드 CSS와
  함께 감싼 임시 하네스를 `web/dist`(gitignore됨)에 만들고 **WSL 안에서
  Playwright로 320/390/1280 폭을 계측**했다(하네스·스크립트 모두 커밋하지
  않는다). 확인한 것: 64줄 전부 렌더, legend 1회, 줄 단위 배지 0개, 세 폭
  모두 가로 스크롤 0(`listScrollW == listClientW`), 목록 내부 스크롤 동작,
  보드:로그 0.700 유지, `<summary>` 높이 38px(터치 타겟 24px 이상).
  정식 5뷰포트 QA는 게이트를 푸는 Step 1d-4 소관이다.

### 실화면에서만 잡히는 함정 (Step 1d-2 실측에서 나온 것)

단위 테스트는 HTML 문자열만 본다. 아래 세 가지는 테스트가 전부 통과한 채로
화면이 틀려 있던 항목이며, 320/390/1280 실측에서 발견했다.

- **cue 표식 겹침**: 한 말이 cue를 여러 개 가질 수 있다(피격 + 균형 붕괴 +
  전투불능). 표식을 각각 절대 배치하면 같은 자리에 겹쳐 마지막 하나만 보인다.
  `.combat-board__cues` flex 컨테이너에 담아 나란히 놓는다.
- **말이 보드 경계에서 절반 잘림**: 말은 `translate: -50% -50%`로 중심을
  좌표에 맞추므로 투영 범위를 0~100%로 잡으면 최소·최대 좌표의 말이 잘린다.
  전투원 2명이면 두 말이 항상 극단에 놓이므로 예외가 아니라 기본 경우다.
  여백을 둔 띠(14~86%)로 투영한다.
- **grid item이 높이 상한 때문에 너비가 줄어듦**: `.combat-stage`가 두 축
  모두 stretch인 grid item일 때 `aspect-ratio`와 `max-block-size`를 함께 걸면
  브라우저가 높이 상한을 만족시키려고 **너비**를 줄인다(764px 칸에서 420px로
  축소됐다). `inline-size: 100%`로 너비를 확정해야 한다.

### 재생 연출 계약 (Playback Contract, Wave 3 Step 1d-3)

`renderCombatBoard`/`renderCombatLog`(`renderCombatStage.ts`)가 매 호출마다
결정론적으로 만드는 CSS `@keyframes`/`animation`을 데이터 그대로 `<style>`
요소로 방출한다(실제 keyframe 텍스트 생성은 `combatMotion.ts`가 전담). 마운트
훅이 없는 문자열 렌더러(`renderStorybookPage`) 구조를 그대로 쓰기 위한
선택이다 — `element.animate()`(WAAPI)를 쓰려면 `web/src/main.ts` 배선이
필요하고 `prefers-reduced-motion`을 JS로 다시 확인해야 하며 결정론적 단위
테스트로 검증하기 어렵다. **트레이드오프**: `<style>` 요소가 `<body>` 안에
들어간다 — HTML 스펙상 `style`은 metadata content이므로 body 안에서는
엄격히는 비적합이다. 모든 브라우저가 그래도 적용하며 기능 문제는 없다.

- **총 재생 길이는 정확히 `(view.frames.length - 1) × view.tick_millis`
  ms다.** 연출을 위해 늘이거나 줄이지 않는다(정본 13: "시뮬레이션 시간과
  화면 시간은 항상 일치"). `frames.length <= 1`이면 애니메이션을 만들지
  않고 정지 프레임만 남긴다.
- 위치 모션은 `translate`만 쓴다(`left`/`top`은 1d-2가 이미 고정했다).
  `translate`의 백분율은 컨테이너가 아니라 요소 자기 크기 기준이므로,
  보드-상대 오프셋은 `%` 대신 컨테이너 쿼리 단위(`cqw`/`cqh`)로 쓴다 —
  `.combat-stage__board`에 `container-type: size`를 준다(그리드 행이라
  높이가 내용과 무관하게 정해지므로 size containment가 안전하다). 부수
  효과(의도됨): 말 크기(`12cqi`/`15cqi`)의 가장 가까운 컨테이너가
  `.combat-stage`에서 이 보드로 바뀌어 말이 1d-2보다 살짝 작아진다.
- 투영 범위(`projectAxis`의 min/max)는 1d-2의 "마지막 프레임만"에서
  **전체 프레임**으로 확장했다 — 마지막 프레임만으로 범위를 잡으면 재생
  중 이동하는 말이 그 범위 밖 좌표를 지날 때 보드를 벗어나 보인다. 전투원이
  매 프레임 같은 좌표에 머무는 저작 시나리오에서는 min/max가 그대로이므로
  1d-2 테스트 기대값은 바뀌지 않았다.
- 각 말의 keyframe은 **마지막 프레임 대비 상대 오프셋**으로 표현한다(마지막
  프레임 자신의 오프셋은 항상 0). 그래서 애니메이션 종료 위치와
  `prefers-reduced-motion: reduce`의 정지 위치가 `animation-fill-mode`
  의존 없이 정확히 같다.
- **cue 5종 연출 문법**(정본 13): `attack`은 `piece.facing` 방향으로 짧게
  전진 후 복귀(`facing`이 `(0, 0)`이거나 없으면 생략 — 방향을 지어내지
  않는다), `hit`은 방향 없는 감쇠 2단 진동, `evade`는 보드 Y축(정본 09:
  "측면: 화면 위·아래") 방향 짧은 이동 후 복귀, `incapacitated`는 1d-2가
  이미 쓰는 `[data-active="false"]`의 dimming 값(`opacity: 0.55`,
  `filter: saturate(0.4)`)을 그대로 재사용해 그 tick 자신의 `cues` 배열에
  있을 때만 적용한다(다른 tick으로 지속을 추론하지 않는다).
  - `balance_broken`은 `translate` 좌우 흔들림 **+ `rotate` 기울어짐**이다
    (정본 13이 "흔들림/기울어짐" 둘 다 요구한다). 흔들림만 쓰면 `hit`의 진동과
    구별되지 않아 공용 문법이 무너진다. `rotate`는 `translate`와 같은 개별
    transform 속성이라 컴포지터 스레드에서 처리되고 레이아웃을 만들지 않는다.
    기울지 않은 stop에도 `rotate: 0deg`를 명시한다 — 빼면 보간 대상에서 빠져
    한쪽으로 기운 채 남는다.
- 전투불능 감광(`opacity: 0.55` / `filter: saturate(0.4)`)은 **정적 층**에도
  둔다(`[data-cue-incapacitated="true"]`). 애니메이션 안에만 두면 `reduce`에서
  마지막 프레임이 전투불능인데도 말이 멀쩡하게 보여 재생 경로와 정지 경로의
  그림이 달라진다. 반대로 감광 속성을 **모든** keyframe stop에 내보내면
  `opacity: 1`이 재생 내내 `[data-active="false"]`의 정적 감광을 덮어쓰므로,
  `incapacitated` cue가 한 번이라도 있는 트랙에서만 내보낸다.
- 핵심 로그 각 줄은 `(entry.tick − frames[0].tick) × view.tick_millis`ms에
  opacity로 나타난다
  (`animation-delay` 인라인 + `storybook.css`의 정적 `.combat-log__row`
  규칙). **원점은 `frames[0].tick`이다** — 보드가 프레임 인덱스 k를
  `k × tick_millis`에 놓고 실측 데이터의 첫 tick이 0이 아니라 1이므로, 원점을
  빼지 않으면 같은 사건이 보드보다 한 tick 늦게 나타나 정본 13의 색·아이콘
  동기화가 깨진다. `.combat-log__row`에 `opacity: 0` 기본값을 두지 않는다 —
  `animation-fill-mode: both`가 delay 구간에 이미 `from` 상태를 적용하고,
  기본값으로 두면 애니메이션이 돌지 않을 때 로그가 영구히 보이지 않는다. `core_log` 배열 순서(`sequence`)는 그대로 유지하며, 노출 전에도
  DOM에서 제거하지 않는다.
- 모든 `animation`/`transition`/`@keyframes` 선언(생성된 `<style>` 포함)은
  `@media (prefers-reduced-motion: no-preference)` 안에만 있다. `reduce`
  에서는 이 media block 전체가 무효이므로 1d-2가 만든 출력이 그대로다.
- 신규 색상 리터럴 0개 — 기존 CSS 커스텀 프로퍼티/토큰만 쓴다. 새 이미지·
  SVG 파일도 만들지 않았다(정본 13: 공용 좌표 이동·진동으로 자산 비용
  절감).

### 정본 09 축 계약 위반 (재생 중에만 드러남 — 알려진 core 결함, WP5)

정본 09: **"화면 왼쪽: 아군 영역 / 화면 오른쪽: 적 영역"**. 저작 시점
좌표(아군 x=0 / 도전자 x=5)는 이 계약을 지킨다. 그런데 실제 프레임을
덤프해 보면 아군 x가 1, 2, 3, 2, 3, … 도전자 x가 4, 3, 2, 3, 2, … 로
진동하며 **tick 3부터 아군이 도전자보다 오른쪽에 놓인다** — 역할 가중치
`preferred_distance: 0`이 두 말을 서로 통과시키기 때문이다. 저작 시점
좌표만 검사하는 테스트로는 잡히지 않는다(저작 값 자체는 계약을 지킨다).

이 슬라이스(web)는 **이 사실을 문서에 남기는 것 이상을 하지 않는다** —
렌더러가 좌표를 왜곡해 축 계약을 "고치면" 거리 읽기 자체가 망가진다. 고칠
곳은 escape-core의 AI·충돌 규칙이며 별도 슬라이스다. `crates/`는 이
슬라이스에서 무수정이다(`combat_resolution.rs`의 역할 가중치 로직 변경
필요). 게이트 플래그(`combat_spectator_preview_unlocked`)를 유지하는 또
하나의 근거다.

### 알려진 갭

전투원 표시 이름이 없다. 관전 화면과 보고서는 core가 주는 내부
id(`wuxia_spectator_bout_ally` 등)를 그대로 보여준다.
`CombatSpectatorPiece`/`CombatCombatantReport`에도, 인카운터 combat authoring
에도 표시 이름 필드가 없어 renderer가 유도할 수 없다 — 이름을 발명하지 않고
남겨 두었다. 게이트를 푸는 슬라이스 전에 authoring에 이름 필드를 추가할지
결정해야 한다.

Wave 3 Step 1d-4(남은 범위): 게이트 플래그 제거, wasm 재빌드, 5뷰포트
실화면 QA, 전투원 표시 이름 — `docs/design/Combat_System_Implementation_Plan_Index.md`의
단계 순서 표를 본다.

## Acceptance checklist

- [ ] 기본 화면에 상단 HUD/스탯 그리드/아이콘 dock이 없다 — folio, 본문, 삽화,
      선택지, 하단 각주 스트립만 보인다.
- [ ] 각주 스트립 탭으로 드로어가 열리고 상태 상세/소지품/업적/기록/메뉴가 있다.
- [ ] 몸/마음/위험 요약이 각주 스트립에 항상 보인다 (색+형태 이중 부호화).
- [ ] 본문 세로쓰기가 없다 (장식 한자 제외) — 영어 본문으로 바꿔도 layout이 성립한다.
- [ ] Korean text가 OS 폰트 유무와 무관하게 읽힌다.
- [ ] choices가 문장형 row이고 `data-action-id`/`data-action-kind`/숫자키가 동작한다.
- [ ] unknown visual id가 safe placeholder로 표시되고 action이 유지된다.
- [ ] 복수 Stage/illustration/choice가 source 순서대로 나타나고 고정 5영역으로 재정렬되지 않는다.
- [ ] `천기록` 라벨은 실제 천기록 특수 block에서만 나타난다.
- [ ] GlyphFX stable terms/fallback text가 reduced-motion에서 읽힌다.
- [ ] renderer가 gameplay truth를 재계산하지 않는다.
- [ ] 엔딩/막다른 페이지에 다음 행동(처음 화면 등)이 항상 있다.

## Automated visual QA

`web/scripts/storybook-reference-qa.mjs`가 실행 중인 preview URL을 Playwright
Chromium으로 열고, viewport마다 DOM region contract, centered portrait layout,
horizontal overflow, click/number-key interaction을 확인한다. pixel-perfect
golden baseline이 아니라 structural/layout visual QA다.

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
export PLAYWRIGHT_BROWSERS_PATH=/tmp/dudupunch0-tui-adv/ms-playwright
cd web
npm run qa:storybook:visual -- \
  --base-url http://127.0.0.1:4173/ \
  --out-dir /tmp/dudupunch0-tui-adv/storybook-visual-qa
```

Rust/WASM-primary preview 검증 시 `--require-wasm`을 추가한다.

Output policy:

- screenshots와 `visual-qa-report.json`은 `--out-dir` 아래 scratch artifact로만 남긴다.
- 기본 예시 경로는 `/tmp/dudupunch0-tui-adv/storybook-visual-qa`.
- golden screenshot/image baseline은 Git에 커밋하지 않는다.

## Validation commands

```bash
cd web
npm test
npx tsc --noEmit
npm run build
```

Manual visual QA sizes: 390x844, 414x896, 800x1440, wide desktop.
