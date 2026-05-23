# Mobile Pixel Storybook UI Contract

Status: active Web Storybook visual contract for the mobile portrait redesign.

## Goal

Web Storybook은 웹에서 실행되지만 웹사이트처럼 보이면 안 된다. 기본 화면은 `escape from the office`의 회사-아포칼립스 정체성을 가진 모바일 세로형 픽셀 게임북 board여야 한다.

이 문서는 `idea_box/플레이화면0.bmp`, `idea_box/플레이화면1.bmp`, `idea_box/플레이화면2.bmp`에서 추출한 UI 문법을 현재 Web Storybook renderer에 적용하기 위한 canonical design contract다.

## Reference files

`origin/main`의 `idea_box/` 기준:

| 경로 | 실제 포맷 | 크기 | 용도 |
|---|---|---:|---|
| `idea_box/플레이화면0.bmp` | PNG | 810 x 1440 | 장소 발견/도입형 visual-first page reference |
| `idea_box/플레이화면1.bmp` | PNG | 810 x 1644 | 사건 결과/보상 로그 page reference |
| `idea_box/플레이화면2.bmp` | PNG | 800 x 1644 | 인카운터 + 문장형 선택지 page reference |

주의:

- 파일 확장자는 `.bmp`지만 magic number 기준 실제 포맷은 PNG다.
- 이 파일들은 production asset이 아니라 UI grammar reference다.
- 그대로 asset으로 쓰기 전에는 출처/라이선스/저작권을 확인해야 한다.

## Core visual grammar

기본 화면 구조:

```text
[고정 상단 HUD]
  격리 대상 초상 / 현재 장소·장면 nameplate / health·sanity slot / 2x3 상태칸 / 문서·설정 ornament

[진행도·위험도 rail]
  단순 divider가 아니라 turn, danger, route 압박을 암시하는 장식 bar

[본문 스토리 영역]
  card 안이 아니라 오래된 종이/결재서류 위에 직접 놓이는 문장
  중앙 픽셀 일러스트 또는 Canvas/GlyphFX visual
  결과/보상/단서 로그는 modal이 아니라 본문 흐름 안의 강조 문장

[선택지]
  사각 웹 button card가 아니라 ✥ bullet이 붙은 큰 문장형 row

[하단 dock]
  균등분할 web tab bar가 아니라 기록/단서/업적/현재 목표/가방 pixel object dock
```

## Layout contract

- 전체 shell은 폭 800~810px 전후의 centered portrait board를 기준으로 한다.
- desktop에서도 좌측 sidebar + 우측 content dashboard로 바꾸지 않는다.
- board 배경은 오래된 종이, 결재서류, 양피지 계열의 밝은 톤을 사용한다.
- 본문, visual, choices, history는 각각 독립된 dark card로 보이면 안 된다.
- text readability가 최우선이다. GlyphFX는 가독성을 해치지 않고 stable terms/fallback text를 보존해야 한다.
- Linux/Cloud preview처럼 한국어 system font가 없는 환경에서도 tofu 박스(□)가 나오지 않도록 Web bundle은 `@fontsource/noto-serif-kr`의 Korean weight를 포함한다.

## HUD contract

상단 HUD는 reference의 게임 오브젝트 문법을 회사-격리 톤으로 변환한다.

| Reference element | Local meaning |
|---|---|
| 캐릭터 초상 | 사원증 사진, CCTV 얼굴, 격리 대상 avatar |
| 이름표 | `page.location.name` 기본, 필요 시 page title 보조 |
| 하트 row | `health` / 신체 반응 slot |
| 보석 row | `sanity` / 집중도 slot |
| 우측 2x3 stat grid | core가 제공한 resource summary와 danger display |
| 두루마리 | 결재 문서, 사내 공지, 격리 티켓, `page.chapter_label` |
| 톱니바퀴 | 설정/system ornament |

Renderer는 status를 계산하지 않는다. `ScenePage.status_summary.resources`, `turn`, `danger`, `warnings`, `pressure_cues`를 표시만 한다.

## Progress rail contract

- HUD 아래에는 `story-progress-rail`이 있어야 한다.
- rail은 `page.status_summary.turn`과 `danger`를 aria label로 노출한다.
- visual marker는 실제 엔딩 진행률이 아니라 압박감/격리도 표현이다.
- danger가 높으면 red/black 계열로 바꿀 수 있다.

## Story flow contract

Current first-slice policy:

| Page kind | Layout |
|---|---|
| `movement` / location visual | visual-first |
| `encounter` | text-first |
| `ending` | visual/body 중심 |

현재 `ScenePage`에는 body 중간 visual insertion point가 없으므로 schema를 열지 않는다. renderer-local layout policy만 사용한다. 나중에 exact story composition이 콘텐츠 계약이 되면 `docs/dev/Data_Schema.md`에서 additive semantic field를 먼저 설계한다.

## Choices contract

- 선택지는 `<button>` semantics와 `data-action-id`를 유지한다.
- renderer DOM은 `choice-row` button과 `choice-bullet` marker를 사용한다.
- 화면에서는 web card button이 아니라 `✥` bullet + index + 큰 문장 + optional cost/reason으로 보인다.
- `data-action-kind`를 유지한다.
- blocked action은 숨기지 않고 reason을 보여준다.
- 번호키/클릭 실행은 기존 `SceneAction.id` 계약을 유지한다.

## Bottom dock contract

하단 dock 기본 mapping:

| Dock | Meaning |
|---|---|
| 기록 | history drawer |
| 단서 | clue/reference placeholder |
| 업적 | achievement summary placeholder |
| 현재 목표 | action/objective placeholder |
| 가방 | inventory summary placeholder |

First slice에서는 history만 실제 drawer로 연결해도 된다. 나머지 dock item은 기능을 과장하지 말고 visual/accessible label 수준으로 둔다.

## Renderer boundary

반드시 유지한다.

- Rust GameCore가 action eligibility, outcome, ending, achievement의 truth다.
- Web Storybook은 `ScenePage` semantic field를 DOM/CSS/Canvas로 해석하고 action id를 다시 core에 전달한다.
- `ScenePage`에는 CSS class, pixel coordinate, DOM selector, Canvas command, terminal color object, image path를 넣지 않는다.
- reference image는 asset으로 직접 import하지 않는다.
- public UI/docs/generated data에는 실제 회사명, 실제 내부망 주소, 실제 좌석, 개인명, private final hint를 넣지 않는다.

## Acceptance checklist

- [ ] desktop과 mobile 모두 centered portrait board를 유지한다.
- [ ] Korean text가 OS font 유무와 무관하게 읽히고 tofu 박스로 깨지지 않는다.
- [ ] 상단 HUD에 portrait/nameplate/vital slots/stat grid/document/settings 영역이 있다.
- [ ] HUD 아래 progress/danger rail이 있다.
- [ ] story text는 paper background 위에 직접 놓이고 dark card dashboard처럼 보이지 않는다.
- [ ] visual은 중앙 pixel illustration/vignette처럼 보이고 visible card caption chrome이 없다.
- [ ] choices는 decorated sentence rows이며 `data-action-id`/`data-action-kind`를 유지한다.
- [ ] bottom dock은 standard web tab bar가 아니라 pixel object dock처럼 보인다.
- [ ] GlyphFX stable terms와 fallback text가 reduced-motion/no-canvas에서도 읽힌다.
- [ ] unknown visual id는 safe placeholder를 보여주며 action을 drop하지 않는다.
- [ ] renderer가 gameplay truth를 재계산하지 않는다.

## Validation commands

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
cd web
npm test -- --run src/ui/storybook/render.test.ts
npm test
npm run build

cd /home/dudupunch0/tui_adv
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
python3 scripts/export_web_data.py \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json \
  --check
python3 -m pytest tests/test_docs_contract.py tests/test_web_packaging_decision.py -q
git diff --check
```

Manual visual QA sizes:

- 390 x 844
- 414 x 896
- 800 x 1440
- 810 x 1644
- wide desktop
