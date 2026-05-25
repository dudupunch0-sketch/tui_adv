# escape from the office 전체 개발 계획

> **Canonical main plan:** 이 repo의 현재 개발 우선순위, 다음 작업 순서, active direction은 이 파일이 기준이다. 다른 LLM/agent에게 작업을 맡길 때는 “`docs/dev/Development_Plan.md`를 메인 플랜으로 보고 다음 작업을 진행해”라고 지시한다.
>
> 이 문서는 처음 작성된 구현 전 기준점도 포함하므로, 충돌이 있으면 상단의 최신 방향/다음 액션을 우선한다. `README.md`는 요약/실행법, `docs/dev/Checklist.md`는 완료 여부 추적, 아키텍처/스키마 문서는 계약 참조, `idea_box/`는 active plan이 없을 때 보는 backlog다. `.hermes/plans/`는 세션용 작업 계획이며 canonical source가 아니다.

## 0.0 계획 문서 우선순위

1. `docs/dev/Development_Plan.md`: 단일 메인 플랜. 현재 방향, 다음 작업, 우선순위, phase 순서를 여기서 판단한다.
2. `docs/dev/Checklist.md`: 완료 여부 추적용 체크리스트. 독립적인 다음 계획을 두지 않는다.
3. `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/dev/Data_Schema.md`, `docs/design/UI_Rules.md`, `docs/dev/TUI_Layout.md`: 설계 계약/참조 문서. 작업 순서의 source of truth가 아니다.
4. `README.md`: 사람용 빠른 안내와 실행법. 긴 다음 작업 목록은 이 파일로 복제하지 않는다.
5. `idea_box/`: active plan/todo가 없거나 사용자가 명시적으로 요청했을 때 처리하는 backlog.
6. `.hermes/plans/`: 일회성 세션 artifact. 완료되었거나 이 파일에 흡수된 계획은 정리한다.

## 0. 2026-05-22 방향 갱신

현재 개발 방향은 다음과 같이 고정한다.

```text
Rust GameCore
  ├─ Web Storybook + GlyphFX renderer
  │   └─ primary player UX
  └─ SuperLightTUI terminal renderer
      └─ terminal-native fallback / horror edition
```

- Web Storybook + GlyphFX가 플레이어용 메인 UX 후보다. 이미지/장면 컷, 대화 내역, 읽기 중심 선택지, Canvas/GlyphFX는 이 경로에서 먼저 구현한다.
- Rust terminal 경로는 SuperLightTUI 기반 renderer로 유지한다. fallback은 우선순위/환경 호환성의 의미이며, 단순 debug dump를 뜻하지 않는다.
- Python/Textual과 TypeScript mirror core는 전환기 legacy/parity oracle이다. 새 게임 규칙을 그쪽에 계속 복제하지 않는다.
- 세부 아키텍처는 `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`를 따른다.
- wire schema와 구현 계약은 `docs/dev/Data_Schema.md`의 renderer-neutral content bundle, `ScenePage`, action id, `EffectCue`, WASM JSON boundary 설계를 따른다.

## 0.1 2026-05-23 idea_box backlog 반영

`idea_box/BACKLOG_ORDER.md` 기준 open backlog 3개를 처리했다.

1. `TUI Storybook + GlyphFX Concept v2`는 이미 Web Storybook/GlyphFX primary UX와 SuperLightTUI terminal-native horror/fallback edition으로 채택/구현 방향에 반영되어 있어 adopted/merged로 닫았다.
2. 꿈 엔딩 분기 아이디어는 `docs/story/Dream_Ending_Branching.md`로 승격했다. 이는 후속 엔딩/스토리팩 작성용 설계 후보이며, 현재 런타임 YAML/schema/code에 `dream` 또는 `epilogue` 타입을 추가했다는 뜻은 아니다.
3. 현실 탈출 엔딩 분기 아이디어는 `docs/story/Real_Escape_Ending_Branching.md`로 승격했고, 아래 `0.2 active main plan`에서 `escape_commute` text-backed 후일담 첫 runtime slice로 promote했다. 이는 `real_escape`, `post_escape`, `aftermath` 같은 새 타입을 추가한다는 뜻은 아니다.

active/후속 구현에서 새 게임 규칙과 엔딩 판정은 Rust GameCore/content YAML 쪽에서 별도 slice로 진행하고, Web Storybook과 SuperLightTUI는 `ScenePage`/`ActionResult`를 표시하는 renderer로 유지한다.

## 0.2 2026-05-23 active main plan: 현실 탈출 후일담 첫 런타임 slice

이 섹션은 `.hermes/plans/2026-05-23_021437-real-escape-aftermath-implementation.md`의 내용을 canonical main plan으로 흡수한 runtime slice 기록이다.
`.hermes/plans/` 파일은 세션 artifact이며, 이 섹션과 아래 “현재 최우선 남은 작업” / “다음 액션”이 작업 순서의 기준이다.
현재 상태: 구현 완료.

목표:

- `escape_commute` 현실 탈출 엔딩에 공개-safe 후일담 정산을 실제 런타임 콘텐츠로 노출한다.

아키텍처:

- 첫 slice는 새 `kind`, 새 schema field, renderer별 분기를 만들지 않는다.
- 기존 공개 YAML `src/tui_adv/data/endings.yaml`의 `escape_commute.text`에 구조화된 `[POST-ESCAPE REPORT]` 후일담 블록을 붙인다.
- Rust GameCore의 기존 `ScenePage.mode: ending` 경로가 Web Storybook/WASM과 SuperLightTUI terminal까지 그대로 전달하는지 테스트로 증명한다.
- Web Storybook과 SuperLightTUI는 후일담을 재판정하지 않고 core가 제공한 `ScenePage.body_blocks`를 표시한다.

첫 후일담 톤:

```text
[POST-ESCAPE REPORT]
survivor_count: 1
evidence_level: 0
company_response: denial
employee_status: access_revoked
risk_level: ongoing

ENDING: 정문 밖
```

명시적 비목표:

- `real_escape`, `post_escape`, `aftermath` 같은 새 엔딩 종류 추가.
- `EndingDef`에 `aftermath`, `post_escape_report`, `report_blocks` 같은 새 필드 추가.
- rescued NPC, evidence level, witness count 같은 새 runtime state 추가.
- Python/Textual 또는 legacy TypeScript mirror에 새 gameplay rule 복제.
- private 현실 단서, 실제 회사명, 실제 위치, 실제 직원/조직/내부망 정보 추가.

구현 순서:

1. 최신 `origin/main`에서 `feature/real-escape-aftermath` 브랜치를 만든다.
2. RED 테스트를 먼저 추가한다.
   - `tests/test_content_data.py`: `escape_commute`에 `[POST-ESCAPE REPORT]`, `survivor_count: 1`, `evidence_level: 0`, `company_response: denial`, `risk_level: ongoing`, `ENDING: 정문 밖`이 있고 private-only field 이름이 없음을 검증한다.
   - `crates/escape-core/tests/route_parity.rs`: `scene_page_from_content()`의 ending page `body_blocks`에 후일담이 들어오는지 검증한다.
   - `crates/escape-terminal/tests/cli_smoke.rs`: scripted action으로 `escape_commute`에 도달한 SuperLightTUI snapshot이 후일담을 표시하는지 검증한다.
   - `web/src/game/parity.test.ts`: generated ending data에서 Web parity route가 후일담 텍스트를 볼 수 있는지 검증한다.
3. `src/tui_adv/data/endings.yaml`의 `escape_commute.text`만 최소 수정한다.
4. `scripts/export_web_data.py --write`로 Rust/Web content bundle과 Web generated JSON을 갱신한다.
5. targeted GREEN 테스트를 실행한다.
6. `docs/story/Real_Escape_Ending_Branching.md`, `docs/content/Ending_List.md`, 이 문서를 현재 구현 상태에 맞게 동기화한다.
7. 전체 검증 matrix를 실행한 뒤 focused PR로 올린다.

핵심 검증 명령:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
python3 scripts/export_web_data.py \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json \
  --check
python3 -m pytest tests -q
python3 -m compileall -q src tests scripts
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check

cd web
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
npm test
```

## 0.3 2026-05-23 active main plan: 모바일 픽셀 스토리북 UI redesign

이 섹션은 `.hermes/plans/2026-05-23_102222-mobile-pixel-storybook-redesign-plan.md`와 `.hermes/plans/2026-05-23_102222-mobile-pixel-storybook-reference-analysis.md`를 canonical main plan으로 승격해 흡수한 UI slice 기록이다.
`.hermes/plans/` 파일은 세션 artifact이며, 실제 작업 순서와 우선순위는 이 섹션과 아래 “현재 최우선 남은 작업” / “다음 액션”을 기준으로 판단한다.
현재 상태: 모바일 픽셀 스토리북 UI redesign 완료.

목표:

- 현재 Web Storybook 화면을 “웹 대시보드/카드 페이지” 느낌에서 벗어나, `idea_box/플레이화면*.bmp` 레퍼런스의 모바일 세로형 픽셀 RPG/게임북 화면 문법을 반영한 `escape from the office` 전용 Web Storybook primary UX로 재설계한다.
- Web에서 돌아가더라도 웹사이트처럼 보이지 않고, 고정 HUD, 진행 rail, 양피지/결재서류형 본문, 중앙 픽셀 일러스트, 문장형 선택지, 하단 아이콘 dock이 있는 모바일 게임북 화면으로 보이게 한다.

레퍼런스 확인 결과:

- `origin/main`의 `idea_box/`에는 다음 파일이 있다.
  - `idea_box/플레이화면0.bmp`: 실제 포맷 PNG, 810 x 1440.
  - `idea_box/플레이화면1.bmp`: 실제 포맷 PNG, 810 x 1644.
  - `idea_box/플레이화면2.bmp`: 실제 포맷 PNG, 800 x 1644.
- 파일 확장자는 `.bmp`지만 magic number 기준 실제 포맷은 PNG다.
- 이 이미지들은 production asset이 아니라 UI grammar reference다. 그대로 asset으로 쓰기 전에는 출처/라이선스/저작권을 확인해야 한다.

레퍼런스에서 추출한 화면 문법:

```text
[고정 상단 HUD]
  캐릭터/격리자 초상, 이름표, health/sanity slot, 우측 2x3 상태칸, 문서/설정 ornament

[상단 진행도/위험도 rail]
  단순 divider가 아니라 turn/danger/route 압박을 암시하는 장식 bar

[본문 스토리 영역]
  웹 card가 아니라 종이 위에 직접 흐르는 서사 텍스트
  중앙 픽셀 일러스트 또는 사건 visual
  결과/보상/단서 로그는 modal이 아니라 본문 흐름 안의 강조 문장

[선택지]
  사각 웹 버튼이 아니라 ✥ bullet이 붙은 큰 문장형 선택지

[하단 dock]
  균등분할 web tab bar가 아니라 기록/단서/업적/행동/가방 pixel object dock
```

`escape from the office`식 변환 규칙:

| 레퍼런스 요소 | 우리 게임 적용 |
|---|---|
| 캐릭터 초상 | 사원증 사진, CCTV 얼굴, 격리 대상 avatar |
| 하트 row | `health` / 신체 반응 slot |
| 보석 row | `sanity` / 집중도 slot |
| 우측 2x3 스탯 | `health`, `sanity`, `battery`, `hunger`, `thirst`, `danger` 또는 core가 제공한 resource summary |
| 두루마리 | 결재 문서, 사내 공지, 격리 티켓, `page.chapter_label` |
| 보물상자 | 단서함, 분실물함, 사내 보관함 |
| 트로피 | 업적, 엔딩 기록, 감사패/도장 |
| 숲/유적 | 복도, 복합기, 회의실, 서버실, 비상계단, 사내망/문서 장면 |

아키텍처 경계:

- Rust GameCore / `ScenePage` / WASM JSON boundary는 game truth와 semantic contract로 유지한다.
- 변경은 Web renderer layer에 집중한다.
- Renderer는 action eligibility, outcome, ending, achievement를 재계산하지 않는다.
- `ScenePage`에는 CSS class, pixel coordinate, DOM selector, Canvas command, terminal color object, image file path 같은 renderer-specific data를 넣지 않는다.
- 새 gameplay rule은 Web UI 때문에 Python/Textual이나 legacy TypeScript mirror에 복제하지 않는다.

주요 비목표:

- Rust GameCore gameplay rule 변경.
- 새 ending/action/resource 판정 추가.
- reference image를 production asset으로 직접 사용.
- SuperLightTUI terminal renderer redesign. 단, Web 기준과 semantic parity 주의점은 문서에 남길 수 있다.
- Tauri/Electron/desktop wrapper 추가.
- 대규모 asset pipeline 추가.
- legacy fake-TUI shell 삭제.

주요 수정 대상:

- `web/src/ui/storybook/render.ts`
  - `renderStorybookPage()`를 portrait board 구조로 재편한다.
  - `renderHud()`, `renderProgressRail()`, `renderStoryFlow()`, `renderBottomDock()`을 추가한다.
  - 기존 status aside card를 HUD로 올린다.
  - choices는 접근 가능한 `<button>`을 유지하되 문장형 row로 바꾼다.
- `web/src/styles/storybook.css`
  - dark card dashboard / 2-column grid를 제거하고 parchment portrait board CSS로 대체한다.
  - desktop에서도 centered portrait board를 유지한다.
  - HUD, rail, story body, illustration, sentence choices, bottom dock 스타일을 추가한다.
- `web/src/ui/storybook/visualCatalog.ts`
  - visible figcaption/card chrome을 줄이고 중앙 픽셀 삽화 역할로 조정한다.
  - printer/messenger/corridor placeholder를 office-pixel vignette 방향으로 조정한다.
  - GlyphFX stable terms/fallback text는 유지한다.
- `web/src/ui/storybook/history.ts`
  - history를 독립 dashboard card가 아니라 drawer/result-log/dock-compatible secondary area로 낮춘다.
  - `data-region="history"`는 유지한다.
- `web/src/ui/storybook/render.test.ts`
  - HUD/rail/dock/sentence-choice contract test를 먼저 추가한다.
  - 기존 `data-renderer`, `data-region`, `data-action-id`, GlyphFX fallback 보존 test를 유지한다.
- 선택 사항: `web/src/core/scenePageFromTurn.ts`
  - legacy fallback path에서 이미 존재하는 `PlayerState.hunger`/`thirst`를 `status_summary.resources`에 display-only로 포함할 수 있다.
  - 이는 gameplay rule 변경이 아니어야 하며, WASM/Rust `ScenePage`가 이미 충분한 resource summary를 제공하면 생략한다.

구현 순서:

1. 최신 `origin/main`에서 `feature/mobile-pixel-storybook-ui` 브랜치를 만든다.
2. `web/src/ui/storybook/render.test.ts`에 RED contract test를 추가한다.
   - `storybook-hud`, `story-progress-rail`, `storybook-dock`, `choice-row`, `choice-bullet` 존재.
   - `data-renderer="web-storybook"`, `data-region="status|visual|body|choices|history"`, `data-action-id`, `data-action-kind` 보존.
   - legacy fake-TUI/dashboard 용어와 `class="fake-tui"`가 없는지 확인.
   - GlyphFX stable terms와 fallback text가 계속 읽히는지 확인.
3. `web/src/ui/storybook/render.ts`의 top-level DOM skeleton을 portrait board 구조로 변경한다.
   - `renderHud(page)`
   - `renderProgressRail(page)`
   - `renderStoryFlow(page)`
   - `renderChoices(actions, blockedActions, turn)`
   - `renderBottomDock(page)`
4. status 표시를 HUD로 변환한다.
   - nameplate는 기본적으로 `page.location.name`을 사용한다.
   - document ornament는 `page.chapter_label`을 사용한다.
   - `health`/`sanity`는 slot row로 표시한다.
   - resource stat grid는 core가 제공한 `status_summary.resources`와 `danger`만 사용한다.
5. progress/danger rail을 추가한다.
   - `turn`과 `danger`를 aria label에 포함한다.
   - danger band는 display-only CSS attribute로 둔다.
   - rail progress는 실제 엔딩 진행도가 아니라 시각적 압박 marker다.
6. story flow를 재배치한다.
   - `movement`/location 성격은 visual-first.
   - `encounter`는 text-first.
   - `ending`은 visual/body 중심.
   - schema 변경 없이 renderer-local layout policy만 사용한다.
7. choices를 문장형 row로 바꾼다.
   - 버튼 semantics, click, number-key action id 매핑은 유지한다.
   - 시각적으로는 card button이 아니라 `✥` bullet + 큰 문장 + optional cost/reason으로 보이게 한다.
   - blocked action reason은 계속 보여준다.
8. bottom dock과 history drawer를 추가한다.
   - 기록/단서/업적/현재 목표/소지품을 pixel object dock으로 배치한다.
   - first slice에서는 history만 실제 `<details>` drawer로 연결해도 된다.
   - 아직 구현되지 않은 dock 기능을 과장하지 않는다.
9. `web/src/styles/storybook.css`를 parchment portrait board 기준으로 재작성한다.
   - 800~810px reference width를 기준으로 center board를 만든다.
   - desktop 2-column dashboard media query를 제거하거나 portrait-preserving 방식으로 바꾼다.
   - Korean text readability, focus-visible, reduced-motion을 확인한다.
10. 필요 시 `visualCatalog.ts`와 `history.ts`를 새 구조에 맞춘다.
11. 구현 후 docs를 동기화한다.
   - 필요하면 `docs/design/Mobile_Pixel_Storybook_UI.md`를 새 canonical design doc으로 만들고 reference analysis를 흡수한다.
   - `docs/design/TUI_Storybook_GlyphFX_Concept.md`, `docs/design/UI_Rules.md`, `docs/00_Index.md`, `README.md`는 중복 없이 링크/요약만 갱신한다.
   - 이 `docs/dev/Development_Plan.md`의 상태를 구현 진행/완료에 맞게 갱신한다.

핵심 검증 명령:

```bash
source ~/.config/tui_adv/tmp-installs.sh
cd web
npm test -- --run web/src/ui/storybook/render.test.ts
npm test
npm run build

cd ..
python3 scripts/export_web_data.py \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json \
  --check
python3 -m pytest tests/test_docs_contract.py tests/test_web_packaging_decision.py -q
git diff --check
```

수동 visual QA:

- 브라우저 preview를 390x844, 414x896, 800x1440, 810x1644, wide desktop에서 확인한다.
- desktop에서도 2-column dashboard가 아니라 centered portrait board인지 확인한다.
- HUD/rail/story text/central visual/sentence choices/bottom dock이 reference grammar와 맞는지 확인한다.
- choices click과 number-key action이 유지되는지 확인한다.
- printer anomaly GlyphFX canvas 또는 fallback text가 계속 보이는지 확인한다.
- unknown visual id placeholder가 action을 drop하지 않는지 확인한다.

완료 기준:

- `web/src/ui/storybook/render.test.ts` 통과.
- `cd web && npm test` 통과.
- `cd web && npm run build` 통과.
- 문서 변경 시 docs contract/export checks 통과.
- `git diff --check` 통과.
- manual browser QA에서 “인터넷 페이지”가 아니라 “모바일 픽셀 게임북 board”로 보임.
- office-horror 정체성이 fantasy RPG asset 복붙처럼 보이지 않음.
- Renderer contract boundary가 유지됨.

## 0.4 2026-05-23 active main plan: Web Storybook visual regression 자동화

이 섹션은 `.hermes/plans/2026-05-23_150444-web-storybook-visual-regression-qa-plan.md`의 내용을 canonical main plan으로 승격해 흡수한 QA automation slice 기록이다.
`.hermes/plans/` 파일은 세션 artifact이며, 실제 작업 순서와 우선순위는 이 섹션과 아래 “현재 최우선 남은 작업” / “다음 액션”을 기준으로 판단한다.
현재 상태: 구현 완료.

구현 결과:

- `web/scripts/storybook-reference-qa.mjs`가 Playwright Chromium으로 reference viewport structural/layout visual QA를 실행한다.
- `web/package.json`에 `qa:storybook:visual` script와 `playwright-chromium` devDependency를 추가했다.
- `tests/test_web_visual_qa_contract.py`가 QA script/package script/viewport set/scratch output/report schema/docs sync를 고정한다.
- `docs/design/Mobile_Pixel_Storybook_UI.md`와 `docs/dev/Checklist.md`에 automated visual QA command와 scratch artifact policy를 반영했다.

목표:

- PR #71에서 완성한 Web Storybook 모바일 픽셀 board가 회귀하지 않도록 reference-size visual QA를 반복 가능한 자동화 경로로 만든다.
- 기존 수동/일회성 Playwright 확인을 repo-local QA script와 문서화된 command로 고정한다.
- Web Storybook primary UX가 desktop에서 2-column dashboard로 돌아가거나 legacy fake-TUI marker를 노출하는 regressions를 빠르게 잡는다.

아키텍처:

- Rust GameCore, `ScenePage`, WASM JSON boundary는 변경하지 않는다.
- QA harness는 Web-only 검증 계층이다. build/serve된 Web Storybook artifact를 Playwright로 열고 DOM/layout/interaction contract를 확인한다.
- 첫 slice는 구조적 visual regression이다. screenshot JSON/report를 scratch output에 남기되, golden pixel baseline은 아직 repo에 커밋하지 않는다.
- screenshots, Playwright browser cache, generated wasm package, `node_modules`는 모두 local/scratch artifact로 유지한다.

주요 비목표:

- gameplay rule, route, ending, eligibility, resource 계산 변경.
- `ScenePage` schema 또는 renderer-neutral content bundle 변경.
- production asset pipeline 추가.
- scene composition schema 추가.
- screenshot/golden image를 Git에 커밋.
- Python/Textual 또는 legacy TypeScript mirror에 새 Web QA logic 복제.

구현 대상:

- `web/scripts/storybook-reference-qa.mjs`
  - Playwright Chromium viewport runner.
  - `--base-url`, `--out-dir`, optional `--require-wasm`를 받는다.
  - URL serving은 담당하지 않는다. QA 대상 server는 기존 `vite preview`, `preview:player`, `preview:wasm`, 또는 검증된 static server가 맡는다.
  - 각 viewport는 fresh browser context에서 실행하고 localStorage를 clear한다.
  - `reducedMotion: "reduce"`, `deviceScaleFactor: 1`, `document.fonts.ready` 대기를 사용해 screenshot/layout 흔들림을 줄인다.
  - JSON report와 screenshots를 `--out-dir` 아래에 저장한다.
- `web/package.json`
  - `qa:storybook:visual` 또는 비슷한 discoverable script를 추가한다.
  - first implementation은 reproducibility를 위해 `playwright-chromium` devDependency 추가를 우선 검토한다.
  - Playwright 설치/브라우저 cache가 `/home`을 채우지 않도록 `PLAYWRIGHT_BROWSERS_PATH` tmp policy를 문서화한다.
- `tests/test_web_visual_qa_contract.py` 또는 기존 Web packaging test 확장
  - QA script 존재, viewport set, scratch output policy, hardcoded local path 금지, script option, report schema를 검증한다.
- `docs/design/Mobile_Pixel_Storybook_UI.md`
  - automated visual QA section을 추가한다.
- `docs/dev/Checklist.md`
  - 진행 추적 checkbox를 추가한다.

구현 전 고정 결정:

1. Dependency: script는 Chromium만 필요로 하며, 구현 PR에서 `playwright-chromium` devDependency 추가를 우선한다. 브라우저 binary/cache는 `PLAYWRIGHT_BROWSERS_PATH=/tmp/...`로 돌리는 명령을 docs에 남긴다.
2. Serving: first slice의 QA script는 server를 띄우지 않고 `--base-url`만 검사한다. custom Node static server는 이번 slice에서 만들지 않는다. MIME/path traversal 리스크를 피하기 위해 기존 Vite preview 또는 검증된 static server를 사용한다.
3. Layout thresholds:
   - 모든 viewport에서 `documentElement.scrollWidth <= viewportWidth`와 `body.scrollWidth <= viewportWidth`.
   - mobile/reference viewport에서 shell width는 viewport width를 넘지 않는다.
   - wide desktop에서 shell width는 760~850px 범위이고 좌우 margin 차이는 4px 이하.
4. Interaction 판정: fresh context에서 first enabled choice click 전후의 shell text/hash 또는 turn/title/body hash가 바뀌어야 한다. number key `1`도 별도 fresh context에서 같은 기준으로 검증한다.
5. WASM mode: `--require-wasm`는 `assets/wasm-pkg/escape_wasm.js`, `escape_wasm_bg.wasm` resource load, `.storybook-runtime-warning` 부재, WASM bootstrap 완료 대기를 확인한다.
6. Report schema: `visual-qa-report.json`은 `baseUrl`, `requireWasm`, `viewports[]`를 포함하고, 각 viewport entry에는 `name`, `width`, `height`, `passed`, `checks[]`, `screenshot`, `shellRect`, `scrollWidth`를 기록한다.

Viewport set:

```text
390x844
414x896
800x1440
810x1644
1440x1000  # wide desktop에서도 centered portrait board 유지
```

필수 검증 contract:

- `[data-renderer="web-storybook"]` 존재.
- `.storybook-shell`이 viewport 폭을 넘지 않고 wide desktop에서 약 810px centered board로 유지.
- `.storybook-hud[data-region="status"]`, `.story-progress-rail`, `[data-region="visual"]`, `[data-region="body"]`, `[data-region="choices"]`, `[data-region="history"]`, `.storybook-dock` 존재.
- `button.choice-row[data-action-id]`가 1개 이상 존재하고 `.choice-bullet`이 보존됨.
- `.fake-tui`, `.storybook-topline`, legacy `CURRENT ENCOUNTER` / `LOCAL STATUS` 같은 dashboard marker가 user-visible하지 않음.
- `documentElement.scrollWidth`와 `body.scrollWidth`가 viewport width를 넘지 않음.
- 첫 choice click이 rendered page를 바꿈.
- fresh page에서 number key `1`이 rendered page를 바꿈.
- browser title에 legacy `fake TUI` wording이 없음.
- `--require-wasm` mode에서는 `assets/wasm-pkg/escape_wasm.js`, `escape_wasm_bg.wasm` resource load와 `.storybook-runtime-warning` 부재를 확인한다.

구현 순서:

1. 최신 `origin/main`에서 `feature/web-storybook-visual-regression-qa` 브랜치를 만든다.
2. RED 테스트를 먼저 추가한다.
   - QA script/package script 존재.
   - viewport set이 script에 포함됨.
   - script가 scratch `--out-dir`를 요구하거나 지원함.
   - hardcoded `/home/...` output path가 없음.
   - static server를 추가한다면 `.js`와 `.wasm` MIME handling을 테스트한다.
3. `web/scripts/storybook-reference-qa.mjs`를 최소 구현한다.
   - script는 URL을 직접 serve하지 않고 `--base-url` 대상으로 QA만 수행한다.
   - build/serve orchestration은 기존 Vite preview/player preview 또는 별도 runner가 담당한다.
   - fresh context, localStorage clear, reduced motion, deviceScaleFactor 1, `document.fonts.ready` 대기를 기본값으로 둔다.
4. safe serving path를 결정한다.
   - first slice에서는 custom Node static server를 만들지 않는다.
   - 우선 후보: `vite preview`, `preview:player`, `preview:wasm`, 또는 `python3 -m http.server`로 `web/dist`를 serve.
   - 나중에 custom Node server를 만들 필요가 생기면 별도 PR에서 path traversal 방지와 explicit MIME map을 테스트한다.
5. package scripts를 추가한다.
   - first implementation은 `playwright-chromium` devDependency 추가를 우선하고, `PLAYWRIGHT_BROWSERS_PATH=/tmp/...` 문서화를 함께 넣는다.
   - browser install/cache가 repo 또는 `/home`에 남지 않도록 docs와 test wording을 맞춘다.
6. `--require-wasm` smoke를 추가한다.
   - Rust/WASM-primary artifact가 runtime warning 없이 load되는지 확인한다.
   - hardened `copy-wasm-pkg.mjs`가 absolute output을 거부하므로 scratch `/tmp` preview copy에는 해당 helper를 억지로 쓰지 않는다. 필요 시 controlled one-off copy를 QA command 안에서 사용한다.
7. `docs/design/Mobile_Pixel_Storybook_UI.md`, `docs/dev/Checklist.md`, 이 문서를 구현 상태에 맞게 동기화한다.
8. 전체 검증 matrix를 실행한 뒤 focused PR로 올린다.

핵심 검증 명령:

```bash
source ~/.config/tui_adv/tmp-installs.sh
export PLAYWRIGHT_BROWSERS_PATH=/tmp/dudupunch0-tui-adv/ms-playwright
cd web
npm test
npm run build
npm run qa:storybook:visual -- \
  --base-url <verified-local-preview-url> \
  --out-dir /tmp/dudupunch0-tui-adv/storybook-visual-qa
# Rust/WASM-primary preview smoke가 필요하면 같은 command에 --require-wasm 추가

cd ..
python3 -m pytest \
  tests/test_docs_contract.py \
  tests/test_web_packaging_decision.py \
  tests/test_web_wasm_build_standardization.py \
  tests/test_web_visual_qa_contract.py \
  -q
python3 scripts/export_web_data.py \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json \
  --check
git diff --check
```

완료 기준:

- reference viewport QA command가 repo에서 discoverable하다.
- viewport 390x844, 414x896, 800x1440, 810x1644, wide desktop가 통과한다.
- click과 number-key interaction contract가 통과한다.
- `--require-wasm` mode에서 Rust/WASM-primary artifact가 runtime warning 없이 load된다.
- screenshots와 JSON report는 scratch output에만 생성되고 Git에 커밋되지 않는다.
- docs/checklist/main plan이 구현 상태와 일치한다.
- Renderer contract boundary가 유지된다.

## 1. 목표

국내 최고 대기업 IT/반도체 회사의 연구개발동 같은 사무실을 배경으로 한 TUI 기반 랜덤 인카운터 선택지 생존 게임을 만든다.
플레이어는 제한된 인터넷/사내망, 불안정한 사무실 공간, 알 수 없는 재난 속에서 체력, 정신력, 배터리, 허기, 갈증을 관리하며 다음 목표 중 하나를 달성한다.

- 탈출: 건물 밖으로 살아서 나간다.
- 정복: 보안실/서버실/사내 시스템을 장악해 회사를 지배한다.
- 진실 발견: 재난의 원인과 회사의 비밀을 밝혀낸다.
- 히든 현실 연결: 게임 속 단서를 따라 실제 사무실에 숨겨진 메모 또는 보물 위치를 알아낸다.

확정된 제목/코드명은 `escape from the office`다.
1차 재난 타입은 “불명 재난”으로 시작한다. 갑자기 사람들이 전부 보이지 않고, 공간 또는 차원이 회사 연구개발동 단위로 격리된 듯하며, 외부 인터넷은 제한되지만 사내망을 통해 간헐적으로 연락과 로그가 들어온다.
톤은 블랙코미디 회사 괴담에서 시작해 점차 코스믹 호러로 기울어진다.

## 2. 핵심 플레이 경험

플레이어가 매 턴 하는 일은 단순해야 한다.

1. 현재 위치와 상태를 확인한다.
2. 랜덤 또는 조건 기반 인카운터를 읽는다.
3. 선택지를 고른다.
4. 자원, 아이템, 플래그, 위치가 변화한다.
5. 새로운 단서 또는 위험이 열린다.
6. 탈출/정복/진실/히든 루트 중 하나로 수렴한다.

게임의 재미는 다음 네 가지에서 나온다.

- 같은 회사 공간이 재난 타입에 따라 다르게 작동한다.
- 선택지는 현재 자원, 아이템, 플래그에 따라 달라진다.
- 정신력 저하와 갈증 상승이 안정성을 흔들어 텍스트와 선택지를 왜곡할 수 있다.
- 터미널, 로그, 사내망, 시스템 메시지 자체가 믿을 수 없는 공간처럼 느껴진다.
- 현실 사무실과 연결된 히든 힌트가 게임 바깥의 탐험으로 이어진다.

## 3. 기본 가정

현재 활성 기술 방향은 다음과 같다.

- 원본 콘텐츠: YAML
- 공통 런타임: Rust GameCore(`escape-core`) + renderer-neutral content bundle
- Primary UX: Web Storybook + GlyphFX
- Terminal UX: SuperLightTUI 기반 terminal renderer/fallback
- Legacy/parity: Python/Textual, TypeScript mirror core, 기존 fake-TUI browser shell
- 테스트: pytest + Vitest + Cargo tests
- 저장 데이터: JSON/schema-versioned envelope로 수렴

게임 규칙은 renderer에 의존하지 않는 Rust core로 수렴한다. Web과 terminal은 core가 제공한 `ScenePage`/`TurnView`/`ActionResult`/`EffectCue`를 표시하고 입력을 전달하는 어댑터로 둔다.

## 4. 주요 시스템

### 4.1 플레이어 상태

기본 상태는 다음 5개다.

| 상태 | 방향 | 의미 |
|---|---|---|
| 체력 | 높을수록 좋음 | 물리적 생존력 |
| 정신력 | 높을수록 좋음 | 괴현상, 공포, 환각 저항력 |
| 배터리 | 높을수록 좋음 | 휴대폰, 손전등, 사내망, 녹음/촬영에 필요한 전력 |
| 허기 | 낮을수록 좋음 | 시간이 지날수록 쌓이는 굶주림 압박 |
| 갈증 | 낮을수록 좋음 | 허기보다 빠르게 위험해지는 탈수 압박 |

초기 규칙 초안:

- 매 턴 허기 +1
- 매 턴 갈증 +2
- 배터리는 기본 감소 없거나, 어두운 구역/휴대폰 사용 시 감소
- 허기 70 이상이면 체력 회복 제한 또는 행동 성공률 감소
- 갈증 60 이상이면 정신력 회복 제한, 환각 이벤트 가능
- 정신력 30 이하이면 일부 이벤트 설명/선택지 왜곡 가능

### 4.2 위치와 맵

1차 수직 슬라이스 위치:

- 내 자리
- 개발팀 사무실
- 복도
- 탕비실
- 회의실
- 복합기 구역
- 서버실 앞
- 비상계단

확장 위치:

- 서버실
- 보안실
- 대표실/임원실
- 엘리베이터
- 옥상
- 지하주차장
- 로비
- 화장실
- 물품창고

### 4.3 인카운터

인카운터는 다음 요소를 가진다.

- id
- 이름
- 발생 위치
- 발생 가능한 재난 타입
- 발생 조건
- 설명 텍스트
- 선택지 목록
- 성공/실패/확정 결과
- 상태 변화
- 아이템 변화
- 플래그 변화
- 후속 이벤트 또는 엔딩 연결

1차 목표는 인카운터 15개다.

예시 후보:

- 퇴사자의 메신저
- 복합기가 혼자 출력한다
- 회의실 예약 패널
- 탕비실 커피머신
- 정수기의 이상한 물
- 서버실 앞의 차가운 바람
- 비상계단의 발소리
- 엘리베이터의 존재하지 않는 층
- 사내 방송
- 회의록에 적힌 내 이름
- 냉장고 안의 쪽지
- 책상 아래 손전등
- 보안 카메라의 시선
- 옥상의 신호
- 지하주차장의 시동음

### 4.4 재난 타입

재난 타입은 같은 맵과 콘텐츠에 다른 규칙을 덮는 레이어다.

확정된 1차 재난 타입:

- 불명 재난: 사람 실종, 연구개발동 규모의 공간/차원 격리, 외부 인터넷 제한, 사내망 간헐 연락

확장 후보:

- 좀비 사태: 소음, 체력, 허기/갈증 압박 중심
- 외계인 침공: 전파, 배터리, 창문/옥상/서버실 중심
- 코스믹 호러: 정신력, 회의실, 문서, 진실 루트 중심
- 백룸 침식: 위치 왜곡, 반복 공간, 잘못된 지도 중심
- 사내 AI 폭주: 사내망, CCTV, 보안실, 서버실 중심

1차 구현은 `unknown_isolation` 하나로 시작하고, 구조만 재난 타입 확장을 고려한다.

### 4.5 아이템과 단서

아이템 분류:

- 생존 아이템: 생수, 과자, 컵라면, 커피, 구급상자
- 전력/도구: 보조배터리, 손전등, 멀티탭, 사원증
- 정보 아이템: 구겨진 출력물, 퇴사자의 메모, 서버 로그 조각, 회의실 예약 내역
- 루트 아이템: 관리자 카드, 차량 키, 옥상 출입키, 마스터 카드

단서는 아이템과 별도로 관리할 수 있다.

- 아이템: 사용하거나 소지하는 물건
- 단서: 루트 조건을 여는 정보
- 플래그: 엔진이 판단하는 내부 상태

### 4.6 엔딩

1차 엔딩:

- 실패: 체력 0, 정신력 0, 탈수/굶주림 한계
- 탈출: 비상계단 루트로 건물 탈출
- 히든 현실 연결: 첫 번째 현실 메모 힌트 획득

확장 엔딩:

- 정복: 보안실/서버실/사내 방송 장악
- 진실: 재난의 원인과 회사의 비밀 발견
- 재난 타입별 특수 엔딩
- 현실 연결 히든 엔딩 여러 개

### 4.7 현실 연결

현실 연결은 이 프로젝트의 핵심 차별점이다.

원칙:

- 실제 위치는 공개 문서나 공개 데이터 파일에 넣지 않는다.
- 위험한 위치, 개인 물건, 잠긴 구역, 전기설비 근처는 사용하지 않는다.
- 공용 공간과 안전한 사물만 사용한다.
- 게임 안에서는 힌트를 단계적으로 공개한다.
- 실제 최종 위치는 `private/` 또는 로컬 전용 데이터 파일로 관리한다.

힌트 단계 예시:

1. 분위기 힌트: “커피 냄새가 남아 있는 곳.”
2. 구역 힌트: “차가운 문이 있는 방.”
3. 사물 힌트: “오른쪽 작은 자석.”
4. 최종 힌트: 실제 사무실 위치. 로컬 비공개 파일에만 둔다.

## 5. 개발 단계

### Phase 0: 문서 기반 정렬

목표:

- 게임 컨셉과 문서 계층을 고정한다.
- 개발 전 변경 비용이 낮은 상태에서 핵심 결정을 기록한다.

산출물:

- `docs/00_Index.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- 이후 추가할 `story/`, `design/`, `content/`, `dev/` 문서 목록

완료 기준:

- 전체 계획과 체크리스트가 존재한다.
- 핵심 자원 5개가 문서에 반영되어 있다.
- 현실 연결 정보의 공개/비공개 경계가 정의되어 있다.

### Phase 1: 프로젝트 스캐폴딩과 기술 확정

목표:

- 실제 실행 가능한 최소 프로젝트 구조를 만든다.
- TUI 라이브러리와 데이터 포맷을 확정한다.

주요 작업:

1. Python 패키지 구조 생성
2. 의존성 관리 방식 결정
3. Textual/Rich 중 TUI 기준 결정
4. pytest 기반 테스트 환경 구성
5. 데이터 디렉터리 생성
6. 기본 실행 명령 정의

예상 파일:

```text
pyproject.toml
src/tui_adv/__init__.py
src/tui_adv/main.py
src/tui_adv/game/state.py
src/tui_adv/game/engine.py
src/tui_adv/data/
tests/
```

검증:

- `python -m tui_adv` 또는 프로젝트에서 정한 명령으로 실행 가능
- `pytest` 실행 가능

### Phase 2: 도메인 모델과 상태 시스템

목표:

- 체력/정신력/배터리/허기/갈증을 가진 `GameState`를 구현한다.
- 상태 변화와 임계치 효과를 테스트한다.

주요 작업:

1. `PlayerState` 또는 `Resources` 모델 구현
2. 수치 범위 clamp 규칙 구현
3. 턴 경과에 따른 허기/갈증 증가 구현
4. 체력/정신력/배터리/허기/갈증 임계치 상태 계산
5. 상태 변화 로그 생성

검증:

- 상태 수치가 0-100 범위를 벗어나지 않는다.
- 허기/갈증은 턴 경과로 증가한다.
- 허기/갈증 임계치가 올바르게 계산된다.

### Phase 3: 위치, 이동, 기본 게임 루프

목표:

- 플레이어가 위치를 가지고, 선택을 통해 이동하고, 턴이 진행되게 한다.

주요 작업:

1. 위치 모델 구현
2. 위치 연결 그래프 구현
3. 이동 선택지 생성
4. 턴 진행 함수 구현
5. 위험도/시간 필드 추가
6. 기본 로그 시스템 구현

검증:

- 시작 위치에서 인접 위치로 이동 가능
- 이동 시 턴이 증가
- 턴 증가에 따라 허기/갈증 변화
- 로그에 이동 결과 기록

### Phase 4: 인카운터와 선택지 엔진

목표:

- 위치/조건에 맞는 인카운터를 뽑고, 선택지 결과를 적용한다.

주요 작업:

1. Encounter 모델 정의
2. Choice 모델 정의
3. 조건 검사 시스템 구현
4. 비용 적용 시스템 구현
5. 성공/실패/확정 결과 적용
6. 아이템/플래그/단서 변경 처리
7. 테스트용 하드코딩 인카운터 3개 구현

검증:

- 조건을 만족하지 않는 선택지는 숨기거나 비활성화된다.
- 비용이 부족한 선택지는 실패하거나 선택 불가 처리된다.
- 선택 결과가 상태/아이템/플래그에 반영된다.

### Phase 5: 데이터 파일 분리

목표:

- 위치, 아이템, 인카운터, 엔딩을 데이터 파일로 관리한다.

주요 작업:

1. 데이터 포맷 확정: YAML 또는 JSON
2. 스키마 문서 작성
3. 데이터 로더 구현
4. 데이터 검증 함수 구현
5. 샘플 위치/아이템/인카운터 데이터 작성
6. 잘못된 데이터에 대한 오류 메시지 작성

예상 데이터:

```text
src/tui_adv/data/locations.yaml
src/tui_adv/data/items.yaml
src/tui_adv/data/encounters.yaml
src/tui_adv/data/endings.yaml
src/tui_adv/data/secrets.example.yaml
```

검증:

- 모든 데이터 파일 로드 성공
- 필수 필드 누락 시 명확한 오류 발생
- 테스트에서 샘플 데이터 기반 게임 루프 실행

### Phase 6: TUI 수직 슬라이스

목표:

- 터미널에서 실제로 플레이 가능한 최소 화면을 만든다.

화면 구성:

- 상단: 시간, 위치, 사내망 상태, 위험도
- 왼쪽: 위치/미니맵
- 중앙: 현재 이벤트와 선택지
- 오른쪽: 사내 진단형 상태, 아이템, 단서
- 하단: 최근 로그

주요 작업:

1. TUI 앱 진입점 구현
2. 상태 패널 구현. 단, 기본 화면은 노골적인 HP bar보다 사내 진단/LOCAL STATUS 형식을 우선한다.
3. 이벤트 패널 구현
4. 선택지 입력 구현
5. 로그 패널 구현
6. 게임 엔진과 TUI 연결

검증:

- 키보드로 선택지를 고를 수 있다.
- 선택 결과가 즉시 화면에 반영된다.
- 체력/정신력/배터리/허기/갈증이 내부 상태와 일치하게 표시된다.
- 기본 화면의 상태 표시는 게임 HUD보다 사내 시스템/진단 로그처럼 보인다.
- 최소 5턴 이상 플레이 가능하다.

### Phase 7: 1차 콘텐츠 작성

목표:

- 1차 수직 슬라이스에 필요한 콘텐츠를 채운다.

범위:

- 위치 8개
- 아이템 10개
- 인카운터 15개
- 엔딩 3개
- 현실 연결 힌트 1개. 단 실제 위치는 비공개 로컬 파일

검증:

- 인카운터가 위치별로 고르게 나온다.
- 생존 아이템으로 허기/갈증/체력 회복 가능
- 첫 번째 히든 루트 플래그 체인이 작동한다.

### Phase 8: 저장/불러오기와 랜덤 시드

목표:

- 플레이 중단/재개와 재현 가능한 테스트를 지원한다.

주요 작업:

1. 저장 데이터 구조 정의
2. JSON 저장 구현
3. JSON 로드 구현
4. 랜덤 시드 저장
5. 저장 파일 버전 필드 추가

검증:

- 저장 후 로드하면 상태/위치/아이템/플래그가 유지된다.
- 같은 시드로 같은 테스트 시나리오를 재현할 수 있다.

### Phase 9: 엔딩 루트 확장

목표:

- 탈출, 정복, 진실, 현실 연결 루트를 분리해서 확장한다.

주요 작업:

1. 탈출 루트 강화
2. 정복 루트 추가
3. 진실 루트 추가
4. 히든 현실 연결 루트 확장
5. 엔딩 조건 우선순위 정의

검증:

- 각 루트별 최소 1개 엔딩 도달 가능
- 여러 엔딩 조건이 동시에 만족될 때 우선순위가 명확함

### Phase 10: 밸런싱, QA, 패키징

목표:

- 반복 플레이가 가능한 수준으로 난이도와 UX를 정리한다.

주요 작업:

1. 자원 증가/감소량 조정
2. 인카운터 발생률 조정
3. 선택지 비용 조정
4. 실패 상태 UX 개선
5. README 작성
6. 실행/설치 방법 정리
7. 릴리즈 체크리스트 작성

검증:

- 새 게임에서 10분 이내 주요 루프를 경험 가능
- 평균 플레이 길이가 의도 범위 안에 들어옴
- 최소 한 개 엔딩에 안정적으로 도달 가능
- README만 보고 실행 가능

## 6. 테스트 전략

테스트는 세 층으로 나눈다.

### 6.1 순수 로직 테스트

대상:

- 상태 변화
- 조건 검사
- 선택지 비용
- 결과 적용
- 엔딩 판정
- 저장/로드 직렬화

### 6.2 데이터 검증 테스트

대상:

- 위치 id 참조 무결성
- 아이템 id 참조 무결성
- 인카운터 필수 필드
- 선택지 결과 필드
- 엔딩 조건 필드
- 히든 루트 플래그 체인

### 6.3 TUI 스모크 테스트

대상:

- 앱 시작
- 기본 화면 렌더링
- 선택지 입력
- 상태 반영
- 종료 처리

가능하면 TUI 로직과 게임 엔진을 분리해서 대부분의 테스트는 TUI 없이 실행되게 한다.

## 7. 현실 연결 안전 기준

현실 연결 기능은 반드시 별도 체크를 통과해야 한다.

- 공용 공간인가?
- 플레이어가 허가 없이 들어가면 안 되는 공간이 아닌가?
- 높은 곳, 전기설비, 위험 물질 근처가 아닌가?
- 개인 물건이나 민감한 회사 자료와 무관한가?
- 힌트가 실제 직원, 고객, 회사 기밀을 노출하지 않는가?
- 최종 위치가 공개 저장소나 배포 파일에 들어가지 않는가?

## 8. 우선순위

현재 완료된 기반:

1. 문서와 계획에 Web Storybook primary + SuperLightTUI terminal renderer 방향을 명확히 고정했다.
2. `escape-core`에 renderer-safe `ScenePage` contract를 추가했다.
3. Web Storybook renderer skeleton을 세워 visual/body/history/choice/status region과 GlyphFX reduced-motion fallback을 검증했다.
4. `escape-wasm` JSON-string boundary를 추가해 Rust GameCore의 `GameState`, `ScenePage`, `ActionResult`, save envelope를 JSON으로 노출했다.
5. `escape-terminal` content TUI snapshot/play loop를 SuperLightTUI renderer로 전환했다.
6. Web/terminal action id parity smoke를 추가했다.
7. Rust GameCore가 movement pages, usable items, ability checks, major endings, achievement unlock metadata, hunger/thirst pressure cues, public-safe reality-link reward metadata를 처리한다.
8. Web Storybook runtime이 `escape-wasm` JSON boundary와 generated content bundle을 통해 Rust `ScenePage`를 소비한다. generated wasm package가 없는 개발 환경에서는 legacy TS mirror fallback만 사용한다.
9. Web WASM build/preview 표준화 완료: `web/package.json`의 `wasm:build`가 `wasm-pack build ../crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg`를 실행하고, `build:wasm` / `preview:wasm`이 Rust/WASM-primary Web 검증 경로를 제공한다. `build:wasm`은 `wasm:copy`로 generated package를 `web/dist/assets/wasm-pkg/`에 복사해 production dynamic import 경로를 맞춘다.
10. legacy TypeScript mirror는 fallback/parity oracle로 freeze했고, Python/Textual도 legacy smoke/parity oracle로만 유지한다.
11. SuperLightTUI terminal visual card/GlyphFX/input polish 완료: `escape-terminal`은 `ScenePage.visual`을 ASCII/Unicode card로 표시하고, `glyph_anomaly`의 intensity meter/stable terms/fallback text와 현재 턴 입력 범위를 노출한다.
12. Web/Tauri/Electron 패키징 결정 완료: 현재 플레이어 배포 표면은 Web-only이며, `web/package.json`의 `build:player` / `preview:player` alias가 Rust/WASM-primary Web artifact(`web/dist/`)를 기준으로 한다. Tauri/Electron은 desktop wrapper 고유 가치가 생길 때까지 deferred다.
13. terminal full-screen app loop/tick/raw-draw GlyphFX 완료: `escape-terminal --app`은 SuperLightTUI `run_with` full-screen loop를 사용하고, `--app-smoke --tick`은 같은 app-frame renderer를 headless로 검증한다. `glyph_anomaly`는 raw-draw layer에서 tick마다 cell wave를 바꾸되 stable terms/fallback text를 유지한다.
14. idea_box 엔딩 분기 backlog 승격 완료: 꿈 엔딩은 `docs/story/Dream_Ending_Branching.md`에 story/design 후보로 정리했고, 현실 탈출 후일담은 `escape_commute` text-backed runtime slice로 승격했다. 새 런타임 엔딩 타입/schema는 열지 않았다.
15. 현실 탈출 후일담 첫 runtime slice 완료: `escape_commute.text`에 public-safe `[POST-ESCAPE REPORT]`를 추가했고 Python content, Rust `ScenePage`, SuperLightTUI snapshot, Web generated parity 테스트로 같은 후일담 노출을 검증했다.
16. 모바일 픽셀 스토리북 UI redesign 완료: Web Storybook을 centered portrait board, HUD, progress rail, paper story flow, 문장형 선택지, bottom dock 구조로 재설계했고 `docs/design/Mobile_Pixel_Storybook_UI.md`에 canonical visual contract를 남겼다.
17. Web Storybook visual regression 자동화 완료: `web/scripts/storybook-reference-qa.mjs`와 `qa:storybook:visual` script로 reference viewport DOM/layout/interaction QA, optional `--require-wasm` smoke, scratch screenshot/`visual-qa-report.json` output을 고정했다.

현재 최우선 남은 작업:

1. 현재 active main plan 기준 즉시 진행할 구현 작업은 없다.
   - 방금 완료한 최우선 slice: Web Storybook visual regression 자동화 first slice.
   - 유지 범위: Rust GameCore / `ScenePage` / WASM JSON boundary는 renderer QA 때문에 변경하지 않는다.
   - 금지 범위: golden screenshot baseline commit, production asset pipeline, scene composition schema, gameplay rule 변경.
   - 다음 구현 slice는 아래 “나중” backlog나 `idea_box`를 별도 검토해 새 active plan으로 승격한 뒤 시작한다.

전환 중 유지:

1. Python/Textual 직접 플레이와 smoke는 legacy/parity oracle로 유지하되 새 gameplay rule을 추가하지 않는다.
2. TypeScript mirror core와 fake-TUI browser shell은 generated wasm package가 없는 개발 환경의 fallback/parity oracle로 유지한다.
3. 새 게임 규칙, route truth, eligibility, outcome, ending, achievement는 renderer가 아니라 Rust core에 추가한다.
4. 현실 탈출 후일담 slice에서도 renderer가 후일담을 재판정하지 않는다. Web Storybook과 SuperLightTUI는 core `ScenePage.body_blocks`를 표시한다.
5. 모바일 픽셀 스토리북 UI redesign에서도 Web renderer는 `ScenePage` semantic field와 action id를 표시/전달만 하며, gameplay truth와 renderer-neutral schema를 변경하지 않는다.

나중:

1. 대표 Web/Rust route smoke가 legacy coverage를 대체하면 Python/Textual과 TypeScript mirror retire 여부를 다시 결정한다.
2. 정복/진실/재난 타입별 변형 콘텐츠 확대
3. 현실 탈출 후일담 다중 변형: `escape_rooftop_signal`, `escape_parking_lot`, `escape_lobby_revolving_door` 같은 다른 escape 엔딩으로 확장할지 결정한다.
4. 후일담 변형이 2개 이상이 되고 단순 text blob이 부족해지면 별도 `aftermath` schema/field slice를 검토한다.
5. 꿈 엔딩을 실제 콘텐츠로 구현할지 결정한다.
6. Tauri/Electron desktop wrapper 재검토: native file dialog, offline file import/export, OS-level 알림/업데이트 같은 Web-only 한계를 실제 요구로 확인한 뒤 별도 slice로 연다.
7. optional inline image는 terminal cell/GlyphFX baseline 밖 future backlog로 둔다. Kitty/Sixel/iTerm2 capability 요구가 실제로 생길 때 별도 slice로 연다.
8. 포켓로그식 URL 즉시 플레이 Web player 공개 배포 계획: `docs/dev/Web_Player_PokeRogue_Style_Plan.md`를 기준으로 GitHub Pages/WASM-required/static deploy slice를 검토한다.
9. 여러 히든 현실 보물

## 9. 주요 리스크

### 범위 과대

재난 타입, 엔딩, 현실 연결이 모두 커질 수 있다.

대응:

- 1차는 불명 재난(`unknown_isolation`)만 구현한다.
- 엔딩은 탈출과 첫 히든 힌트만 우선 구현한다.
- 정복/진실/재난별 특수 규칙은 구조만 열어 둔다.

### 콘텐츠와 코드 결합

인카운터가 코드에 박히면 확장이 어려워진다.

대응:

- 가능한 빨리 데이터 파일로 분리한다.
- 데이터 스키마와 검증 테스트를 둔다.

### 현실 위치 정보 노출

실제 사무실 위치가 Git에 올라갈 수 있다.

대응:

- 실제 위치는 `private/` 또는 `.local` 파일에만 둔다.
- 공개 예시는 `secrets.example.yaml`로 따로 둔다.
- 릴리즈 체크리스트에 비밀 정보 검사 항목을 넣는다.

### Renderer와 core 결합

Web 또는 terminal renderer가 게임 규칙을 다시 구현하면 Rust GameCore 공통화가 깨진다.

대응:

- `escape-core`가 action eligibility, outcome, ending, achievement의 truth를 소유한다.
- Web Storybook과 SuperLightTUI terminal은 `ScenePage`/`ActionResult`를 표시하고 action id만 전달한다.
- SuperLightTUI는 `escape-terminal`에만 추가하고 `escape-core`에는 절대 넣지 않는다.

## 10. 다음 액션

1. 이 slice의 PR에서는 `web/scripts/storybook-reference-qa.mjs`, `qa:storybook:visual`, docs/checklist/main plan 동기화, 검증 결과만 포함한다.
2. 새 active implementation slice는 아직 열지 않는다.
3. 다음 작업을 시작할 때는 이 파일의 “나중” backlog와 `idea_box`를 별도 검토해 새 active main plan으로 승격한다.
4. 그 전에는 asset pipeline, scene composition schema, 또는 새 콘텐츠 확장을 열지 않는다.
