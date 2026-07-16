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
- **배치**: IllustrationBlock은 StoryStage/ResultStage의 위·중간·끝 어디에도
  배치할 수 있다. 일반 Event는 1개가 기본이며 특별/보스 Event는 최대 3개를
  권장한다. 고정 visual slot 하나를 전제로 하지 않는다.
- **미완성 에셋**: stable `visual_id`와 Event 이름 기반 alt를 유지하고 safe
  placeholder를 표시한다. `이벤트 이름.png`를 실제 경로처럼 만들지 않는다.
- **에셋 매니페스트**: `artManifest.ts`에 등록된 `visual_id`는 생성된 `.webp` 애니메이션풍 일러스트 에셋으로 표시되며, 등록되지 않은 `visual_id`는 generic SVG 또는 location-based SVG 폴백으로 렌더링된다.
- **규격 및 포맷**: 규격은 5:3 비율(1120x672, `title_hero`만 3:5 세로 1120x1867)을 준수하며, WEBP 포맷으로 변환 및 최적화하여 각 파일은 150KB 이하여야 한다.
- **삽지 프레임**: 일러스트는 한지 백그라운드 톤과 자연스럽게 결합되도록 얇은 먹 테두리(1px `--line-hard`), 한지 매트(5px 패딩, `--paper-lit`), 아주 약한 세피아 톤 필터 (`filter: saturate(0.92) sepia(0.06)`) 보정이 적용된다.
- **인장 유지**: 일러스트가 활성화되어 있을 때도 우하단의 수묵 인장(seal)은 그림 위에 오버레이로 유지된다.
- **에셋 라이선스**: 디렉터리의 모든 이미지는 프로젝트가 AI 생성으로 직접 제작한 자산이며, 타사 IP 및 외부 게임/작품의 아트를 침해하지 않는다.

### 밀도 계약 (Density Agreement)
- **한 화면 목표**: 390x844 모바일 뷰포트 해상도에서 "본문 제목 + 서사 4~6줄 + 삽화 + 선택지 2개 + 하단 스트립"이 스크롤 없이 또는 단 한 번의 짧은 스크롤로 한눈에 들어와야 한다.
- **상한 제약**: 삽화 영역(`figure[data-region="visual"]`)은 최대 `36dvh` 높이 상한을 준수하며 `object-fit: cover` 및 `preserveAspectRatio="xMidYMid slice"`로 크롭된다.
- **요약 로그 및 칩**: 결과 로그(`.story-result-log`)는 최근 1행 및 필수 업적/아이템 행만 노출하도록 압축하며, GlyphFX stable-terms 칩은 figcaption 옆 한 줄 인라인으로 축소한다.
- **모바일 간격**: 560px 이하 해상도에서 본문 폰트 `1rem/1.7`, 문단 간격 `0.7em`, 선택지 `min-height: 48px` 및 padding 축소로 조밀한 밀도를 확보한다.

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
