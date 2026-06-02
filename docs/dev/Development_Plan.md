# tui_adv 전체 개발 계획

> **Canonical main plan:** 이 repo의 현재 개발 우선순위, 다음 작업 순서, active direction은 이 파일이 기준이다. 다른 LLM/agent에게 작업을 맡길 때는 “`docs/dev/Development_Plan.md`를 메인 플랜으로 보고 다음 작업을 진행해”라고 지시한다.
>
> 이 문서는 처음 작성된 구현 전 기준점도 포함하므로, 충돌이 있으면 상단의 최신 방향/다음 액션을 우선한다. `README.md`는 요약/실행법, `docs/dev/Checklist.md`는 완료 여부 추적, 아키텍처/스키마 문서는 계약 참조, `idea_box/`는 active plan이 없을 때 보는 backlog다. `.hermes/plans/`는 세션용 작업 계획이며 canonical source가 아니다.
>
> 아이디어-설계 흐름은 Notion-first다. 원본 reference는 Notion이고, repo 문서는 Notion reference를 설계 후보로 변환하고 main plan으로 격상한 뒤 설계 결과를 다시 Notion과 대조하는 방식으로 운영한다.

## 0.0 계획 문서 우선순위

1. `docs/dev/Development_Plan.md`: 단일 메인 플랜. 현재 방향, 다음 작업, 우선순위, phase 순서를 여기서 판단한다.
2. `docs/dev/Checklist.md`: 완료 여부 추적용 체크리스트. 독립적인 다음 계획을 두지 않는다.
3. `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/dev/Data_Schema.md`, `docs/design/UI_Rules.md`, `docs/dev/TUI_Layout.md`: 설계 계약/참조 문서. 작업 순서의 source of truth가 아니다.
4. `README.md`: 사람용 빠른 안내와 실행법. 긴 다음 작업 목록은 이 파일로 복제하지 않는다.
5. `idea_box/`: active plan/todo가 없거나 사용자가 명시적으로 요청했을 때 처리하는 backlog. Notion-origin entry는 Notion reference를 원본으로 삼는다.
6. `.hermes/plans/`: 일회성 세션 artifact. 완료되었거나 이 파일에 흡수된 계획은 정리한다.

## 0.0a 2026-05-31 Notion-first 아이디어-설계 운영 규칙

앞으로 새 설계 아이디어는 다음 흐름으로 처리한다.

1. 사용자가 Notion에 아이디어를 정리한다. 이 Notion 문서가 원본 reference다.
2. agent는 Notion reference를 읽고 repo 안의 설계 아이디어 문서로 변환한다. 보통 `docs/design/`, `docs/content/`, `docs/story/` 아래에 candidate 문서를 만들고, `idea_box/inbox/*.md`에는 Notion page id/title/url, `related_docs`, 처리 순서를 남긴다.
3. 다음에 실제 설계할 항목은 설계 아이디어 문서 중 하나를 이 파일의 active main plan / “현재 최우선 남은 작업”으로 격상시킨 뒤 진행한다.
4. 설계가 끝나면 Notion reference와 결과 설계 문서를 비교해 방향, 톤, 핵심 제약, non-goals가 일치하는지 확인한다.
5. Notion reference 대조까지 완료했거나 명시적으로 폐기/병합한 경우에만 해당 idea entry를 `done` 처리한다. 단순 import나 설계 아이디어 문서 작성만으로는 `done` 처리하지 않는다.

2026-06-01 추가 규칙: `이구학지 — 천기록` Notion source는 상위 문서가 초기 기획/시놉시스 역할을 하고, 최신 세부 운영 기준은 `00`~`08` 하위 관리 문서와 `09. 이구학지 사건 카드 DB` / `10. 이구학지 후일담 카드 DB`가 우선한다. repo 구현은 여전히 이 파일과 `docs/content/*`, `docs/design/*` canonical docs를 통과해야 하며, Notion DB row를 읽었다는 이유만으로 runtime 구현 완료나 기본 bundle 반영으로 표시하지 않는다. 최신 source ledger와 coverage는 `docs/dev/Notion_Design_Coverage.md` 및 `idea_box/notion_sources.yml`를 본다.

## 0.0b 2026-06-01 default storypack 전환

현재 메인/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 전환한다. 새 Web player 기본 UX, UI/UX QA, 이후 runtime slice는 이구학지를 우선한다.

첫 비-office 기준팩은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.

- Web player의 새 게임과 terminal `--scene content` 기본 실행은 이구학지 bundle을 기본으로 로드한다.
- 이구학지 기본 run은 `igu-hakji.rust.save.v1` / `igu-hakji.last-run-summary.v1` 계열 localStorage key를 사용해 기존 office save와 섞이지 않게 한다.
- 기존 `escape from the office` content와 `escape-office.*` save key는 legacy/parity/cleanup 대상으로 남긴다.
- Rust/WASM GameCore가 이구학지 Web player의 필수 경로다. generated wasm package가 없을 때 office TypeScript mirror로 조용히 fallback하지 않는다.
- 아래 과거 섹션의 “기본 office bundle 유지”, “preview only” 문구는 당시 slice 기록이다. 최신 작업 판단은 이 섹션과 상단 우선순위를 우선한다.

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
source ~/.config/tui_adv/tmp-installs.sh
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
source ~/.config/tui_adv/tmp-installs.sh
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

## 0.5 2026-05-24 active main plan: Web player deployment readiness

이 섹션은 `docs/dev/Web_Player_PokeRogue_Style_Plan.md`의 “PR 1 — Web player deployment readiness”를 canonical main plan으로 승격해 구현한 static deploy slice 기록이다.
현재 상태: 구현 완료.

목표:

- 포켓로그처럼 URL만 열면 바로 플레이할 수 있는 Web player 배포 기반을 고정한다.
- GitHub Pages project site(`/tui_adv/`)에서 JS/CSS/WASM asset path가 깨지지 않도록 Vite base path와 WASM import path를 하드닝한다.
- production player에서는 Rust/WASM GameCore 로드 실패를 legacy fallback으로 숨기지 않고 fatal player error로 표시한다.
- GitHub Pages workflow에서 build/test/WASM-required visual QA/deploy artifact upload를 수행한다.

구현 결과:

- `web/vite.config.ts`에 `base: process.env.VITE_BASE_PATH ?? '/'`를 추가했다.
- `web/src/core/wasmRuntime.ts`의 default WASM module path를 `new URL('./wasm-pkg/escape_wasm.js', import.meta.url).toString()`으로 바꿨다.
- `web/src/main.ts`에 `VITE_REQUIRE_WASM=true` fatal policy를 추가했다. 이 mode에서는 WASM bootstrap 실패 시 `storybook-runtime-error` 화면을 표시하고 legacy mirror fallback을 실행하지 않는다.
- `.github/workflows/pages.yml`을 추가해 main push/manual dispatch에서 content export check, Rust workspace test, web test, `VITE_BASE_PATH=/tui_adv/ VITE_REQUIRE_WASM=true npm run build:player`, `qa:storybook:visual --require-wasm`, Pages artifact upload/deploy를 수행한다.
- `tests/test_web_player_deployment_contract.py`가 deploy readiness contract를 고정한다.
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`와 `docs/dev/Checklist.md`를 구현 상태에 맞게 동기화했다.

비목표:

- gameplay rule, route truth, eligibility, outcome, ending, achievement 변경.
- `ScenePage` schema 또는 renderer-neutral content bundle 변경.
- start/save UX 화면 추가. 이는 `Web_Player_PokeRogue_Style_Plan.md`의 PR 2 후보로 남긴다.
- PWA/service worker/backend/계정/랭킹 추가.
- golden screenshot baseline commit.

핵심 검증 명령:

```bash
source ~/.config/tui_adv/tmp-installs.sh
python3 scripts/export_web_data.py \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json \
  --check
python3 -m pytest tests/test_web_player_deployment_contract.py tests/test_web_visual_qa_contract.py tests/test_docs_contract.py -q

cd web
npm test
VITE_BASE_PATH=/tui_adv/ npm run build
VITE_BASE_PATH=/tui_adv/ VITE_REQUIRE_WASM=true npm run build:player
npm run qa:storybook:visual -- \
  --base-url <verified-local-preview-url> \
  --out-dir /tmp/dudupunch0-tui-adv/storybook-visual-qa-pages \
  --require-wasm
git diff --check
```

완료 기준:

- GitHub Pages workflow가 repo에 존재하고 `web/dist`를 Pages artifact로 업로드한다.
- project site base path(`/tui_adv/`) build에서 asset URL이 base path를 따른다.
- built JS가 `assets/wasm-pkg/escape_wasm.js`를 module-relative path로 찾는다.
- `VITE_REQUIRE_WASM=true` build에서 WASM load 실패가 legacy fallback으로 숨지 않는다.
- WASM-required Storybook visual QA가 통과한다.
- docs/checklist/main plan이 구현 상태와 일치한다.

## 0.6 2026-05-26 active main plan: Web player start/save UX first slice

이 섹션은 `docs/dev/Web_Player_PokeRogue_Style_Plan.md`의 “PR 2 — Player start/save UX” 중 MVP 우선순위(first slice)를 canonical main plan으로 승격해 구현한 기록이다.
현재 상태: 구현 완료.

목표:

- URL로 들어온 플레이어가 바로 story page로 떨어지기 전에 공개 player 시작 화면을 본다.
- localStorage 기반 저장을 “있음/없음, seed, turn, location, 저장 시각”으로 사용자에게 보여준다.
- 이어하기, 새 게임, 저장 초기화, 새 게임 전 reset confirmation을 제공한다.
- Web renderer는 여전히 `ScenePage`/action id 표시와 storage shell만 담당하고, Rust GameCore / `ScenePage` / WASM JSON boundary는 변경하지 않는다.

구현 결과:

- `web/src/ui/startScreen.ts`를 추가해 start screen HTML, `escape-office.rust.save.v1`, `escape-office.last-run-summary.v1`, legacy save cleanup, save summary read/write를 분리했다.
- `web/src/main.ts`는 첫 로드에서 `data-player-screen="start"` 화면을 렌더링하고, `이어하기`/`새 게임`/`저장 초기화`/`confirm-new-game` 액션 뒤에만 game screen을 시작한다.
- WASM save는 기존 Rust state JSON을 그대로 `escape-office.rust.save.v1`에 저장하고, 공개-safe summary metadata만 `escape-office.last-run-summary.v1`에 따로 저장한다.
- schema mismatch 또는 손상된 summary는 traceback 대신 start screen warning으로 표시하고, 가능한 경우 Rust save JSON에서 seed/turn/location을 복구한다.
- `web/scripts/storybook-reference-qa.mjs`는 start screen이 있으면 `data-player-action="new-game"`을 눌러 실제 Storybook page로 들어간 뒤 기존 DOM/layout/interaction/WASM QA를 수행한다.
- `web/src/styles/storybook.css`에 모바일 portrait board 톤을 유지하는 start card/save panel/confirmation styling을 추가했다.
- `web/src/ui/startScreen.test.ts`가 start screen, continue disabled/enabled, save timestamp, reset confirmation, summary metadata, schema mismatch warning, save cleanup contract를 검증한다.

비목표:

- save JSON export/import.
- 오늘의 seed, 난이도 preset, settings/reduce-motion UI.
- PWA/service worker/backend/account/leaderboard.
- gameplay rule, route truth, eligibility, outcome, ending, achievement, `ScenePage` schema 변경.

핵심 검증 명령:

```bash
source ~/.config/tui_adv/tmp-installs.sh
cd web
npm test
npm run build
# WASM package가 있는 검증 환경에서는:
npm run qa:storybook:visual -- \
  --base-url <verified-local-preview-url> \
  --out-dir /tmp/dudupunch0-tui-adv/storybook-visual-qa-start-save \
  --require-wasm
cd ..
python3 -m pytest tests/test_docs_contract.py tests/test_web_player_deployment_contract.py tests/test_web_visual_qa_contract.py -q
git diff --check
```

완료 기준:

- 첫 화면에 start screen이 표시된다.
- 저장이 없으면 이어하기가 disabled이고 seed 기본값으로 새 게임을 시작할 수 있다.
- 저장이 있으면 이어하기가 enabled이고 seed/turn/location/saved timestamp가 표시된다.
- 기존 저장이 있을 때 새 게임은 confirmation을 거친다.
- 저장 summary schema mismatch는 사용자-facing warning으로 표시된다.
- visual QA는 start screen을 통과해 기존 Storybook DOM/interaction contract를 계속 검증한다.

## 0.7 2026-05-26 active main plan: Web Storybook transition/audio readiness

이 섹션은 `.hermes/plans/2026-05-26_063723-web-storybook-transition-audio-structure-plan.md`의 내용을 canonical main plan으로 승격해 흡수한 Production visual polish slice다.
`.hermes/plans/` 파일은 세션 artifact이며, 실제 작업 순서와 우선순위는 이 섹션과 아래 “현재 최우선 남은 작업” / “다음 액션”을 기준으로 판단한다.
현재 상태: PR A — settings + motion foundation, PR B transition controller, PR C audio engine skeleton 완료. Transition/audio readiness first slice는 renderer-local presentation layer로 닫고, 실제 음악/SFX asset과 soundtrack은 별도 future slice로 분리한다.

목표:

- Web Storybook 화면 전환과 향후 음악/SFX를 renderer-local presentation layer로 붙일 수 있는 구조를 만든다.
- 첫 slice는 “완성된 사운드트랙”이나 큰 애니메이션이 아니라 motion/audio preference, reduced-motion 정책, transition plan type, muted/opt-in audio policy를 고정한다.
- Rust GameCore / `ScenePage` / WASM JSON boundary는 변경하지 않는다. Web renderer는 core truth를 재판정하지 않고 `ScenePage`/`ActionResult`를 표시하고 action id만 전달한다.

아키텍처 경계:

- transition/audio는 `ScenePage` schema가 아니라 Web renderer의 progressive enhancement다.
- renderer가 읽을 수 있는 입력은 `page.mode`, `page.location`, `page.visual`, `page.effect_cues`, `page.status_summary`, 실행한 action id/kind 정도로 제한한다.
- settings는 game save와 분리된 localStorage key `escape-office.player-settings.v1`에 저장한다.
- motion/audio preference는 사용자 설정을 우선하고, 기본 motion은 `prefers-reduced-motion`을 반영한다.
- audio default는 muted/off다. `AudioContext.resume()`은 start screen 또는 settings UI의 사용자 gesture 이후에만 호출한다.
- visual QA에서는 motion off/reduced, audio muted 상태를 강제해 screenshot/layout flake를 만들지 않는다.

명시적 비목표:

- gameplay rule, route truth, eligibility, outcome, ending, achievement 변경.
- `ScenePage` schema 또는 renderer-neutral content bundle 변경.
- save JSON export/import.
- PWA/service worker/backend/account/leaderboard.
- golden screenshot baseline commit.
- 저작권이 불명확한 음악/효과음 asset commit 또는 완성된 사운드트랙 제작.
- SuperLightTUI 오디오/애니메이션 parity 구현.

PR A — settings + motion foundation (완료):

1. `tests/test_docs_contract.py` 등 docs contract를 새 active plan에 맞게 갱신한다.
2. `web/src/ui/settings/playerSettings.ts` 또는 동등한 모듈에 player settings localStorage contract를 추가한다.
   - schema version과 기본값.
   - corrupt settings fallback.
   - reduced-motion preference merge.
   - audio muted/off default.
3. start screen에 audio/motion 상태와 toggle을 연결하되, gameplay action id와 충돌하지 않는 renderer-local action으로 둔다.
4. transition plan type과 reduced-motion no-op 구조를 추가한다.
   - full transition controller는 PR B로 미룰 수 있다.
   - PR A에서는 `start-fade`, `paper-slide`, `ink-pulse`, `danger-glitch`, safe default 같은 mapping과 duration 0 policy를 테스트로 고정한다.
5. 기존 visual QA가 start screen을 통과한 뒤 stable Storybook page를 검사하는 계약을 유지한다.

PR B — transition controller (완료):

- action 실행 전 current page/action context를 캡처하고, action 후 next page와 `transitionPlan(previousPage, nextPage, action)`으로 transition kind를 결정한다.
- reduced/off이면 즉시 render하고, normal이면 shell class/attribute 기반 enter/exit transition을 적용한다.
- `transitionend` 미발생 시 timeout fallback으로 게임이 멈추지 않게 한다.
- `web/src/ui/motion/transitionController.ts`에 renderer-local controller를 추가했고, `web/src/main.ts`는 gameplay action 실행 전후 `ScenePage`를 전달한다.
- CSS transition class/attribute는 `.storybook-shell`에만 붙으며, Rust GameCore / `ScenePage` / WASM JSON boundary는 변경하지 않았다.

PR C — audio engine skeleton (완료):

- `web/src/ui/audio/audioEngine.ts`에 Web Audio API lazy/no-op engine을 만들었다.
- muted 상태에서는 cue를 schedule하지 않고 `AudioContext.resume()`도 호출하지 않는다.
- start/settings UI의 명시 user gesture 이후에만 unlock하며, opt-in 후 one-shot cue와 looping ambience API를 제공한다.
- 첫 slice에서는 generated oscillator backend와 no-op fallback만 사용하고 binary audio asset은 넣지 않았다.
- `web/src/main.ts`는 renderer-local cue scheduling만 수행하며 Rust GameCore / `ScenePage` / WASM JSON boundary는 변경하지 않았다.

핵심 검증 명령:

```bash
source ~/.config/tui_adv/tmp-installs.sh
python3 -m pytest tests/test_docs_contract.py tests/test_web_visual_qa_contract.py tests/test_web_player_deployment_contract.py tests/test_web_packaging_decision.py -q
git diff --check

cd web
npm test -- --run web/src/ui/startScreen.test.ts web/src/ui/storybook/render.test.ts
npm test
node --check scripts/storybook-reference-qa.mjs
```

완료 기준:

- canonical main plan, checklist, Web player plan이 PR C 완료 상태와 다음 분리 slice 방침을 가리킨다.
- player settings는 game save와 분리되어 저장/복구되고 손상 시 기본값으로 fallback한다.
- start screen에서 audio/motion preference를 볼 수 있고 toggle 후 localStorage에 반영된다.
- transition plan type은 action/page/danger/reduced-motion 기준으로 deterministic하게 결정된다.
- audio는 기본 muted/off이고 사용자의 명시 gesture 없이 자동 재생을 시도하지 않는다.
- Web Audio skeleton은 binary asset 없이 generated oscillator/no-op backend만 사용하며 opt-in 후 one-shot cue와 looping ambience API를 제공한다.
- Renderer contract boundary가 유지된다.

## 0.8 2026-05-29 idea_box 전투 시스템 설계 문서화

이 섹션은 `idea_box/combat_system.md`를 프로젝트 문서로 승격한 기록이다.
현재 상태: 설계 문서화 완료. 첫 runtime slice는 아래 0.9 섹션에서 `supply_closet_auto_brawl`로 구현했다.

목표:

- 원본의 “실시간 자동 난투 + 상황 개입” 아이디어를 현재 `escape from the office` 구조에 맞게 정리한다.
- 전투를 Web/terminal renderer-local 실시간 액션으로 만들지 않고, Rust GameCore가 제공하는 `ScenePage`/`ActionResult`/`SceneAction.id` 기반의 짧은 전투 드라마 후보로 제한한다.
- 전투당 개입 요구는 0~3회로 제한하고, 약한 충돌은 자동 처리하며, 정예/보스 장면에서만 결정적 상황 선택지를 제공한다.

반영 결과:

- `docs/design/Combat_System_Auto_Brawl.md`를 추가했다.
- `idea_box/combat_system.md`에 `status: done`, `used_by`, `done_at`, 처리 기록을 남겼다.
- 이번 문서화는 future backlog 정리이며, 현재 active 구현 우선순위인 Web Storybook transition controller를 대체하지 않는다.

후속 구현 후보:

1. 완료: 기존 `encounters.yaml` schema만 사용한 schema-less combat encounter prototype (`supply_closet_auto_brawl`).
2. 반복 가치가 확인되면 `presentation.layout: combat_exchange` 같은 semantic metadata.
3. 여러 전투가 쌓인 뒤에만 Rust core 전용 combat resolver 검토.

## 0.9 2026-05-29 completed slice: schema-less combat encounter prototype runtime

이 섹션은 `docs/design/Combat_System_Auto_Brawl.md`의 PR 1 후보를 첫 runtime slice로 승격한 기록이다.
현재 상태: 구현 완료, `origin/main` 기반 반영됨.

목표:

- 기존 encounter/choice/outcome schema만 사용해 “자동 난투 + 1회 상황 개입” 장면을 실제 런타임 콘텐츠로 노출한다.
- Rust GameCore가 action truth를 소유하고, Web Storybook과 SuperLightTUI는 `ScenePage`/action id를 표시하는 renderer로 유지한다.
- 전투 선택지는 공격 버튼이 아니라 거리, 균형, 시야, 환경 활용을 판단하는 문장형 개입으로 둔다.

아키텍처 경계:

- 새 `CombatState`, 새 combat schema, HP 숫자전, 스킬/쿨타임, 전투 전용 renderer UI를 만들지 않는다.
- 기존 `conditions`, `choices`, `outcome`, resource delta, flag, clue, log, optional `presentation` metadata만 사용한다.
- Web generated data와 Rust content bundle은 같은 renderer-neutral content bundle에서 생성한다.
- Renderer는 combat 여부를 재판정하지 않고 Rust GameCore가 제공한 `ScenePage.visual`, `effect_cues`, `actions`를 표시한다.

반영 결과:

- `src/tui_adv/data/encounters.yaml`의 `supply_closet_cache`에 `brace_for_supply_scuffle` 선택지를 추가했다.
- 새 encounter `supply_closet_auto_brawl`을 추가했다.
  - 위치: `supply_closet`
  - 필수 플래그: `supply_scuffle_started`
  - 금지 플래그: `supply_scuffle_resolved`
  - presentation: `visual_id: supply_closet_scuffle`, `layout: combat_intervention`, stable terms `거리 / 균형 / 소화기 핀`
- 선택지는 3개로 제한했다.
  - `keep_distance_between_shelves`: 관찰/거리 벌리기
  - `hook_cart_to_cabinet`: 카트와 캐비닛으로 균형을 무너뜨리는 개입
  - `pull_extinguisher_pin`: 소화기 분말로 시야를 끊는 위험한 개입
- `crates/escape-core/fixtures/content/content.bundle.json`, `web/src/data/generated/content.bundle.json`, `web/src/data/generated/encounters.json`, `web/src/data/generated/manifest.json`을 재생성했다.

검증 범위:

- Python content loader/CLI smoke가 기존 encounter schema로 전투형 encounter를 통과하는지 확인한다.
- Rust `escape-core` tests가 `ScenePage`와 action id로 전투형 encounter를 표시/해결하는지 확인한다.
- SuperLightTUI smoke가 같은 `visual_id`, `layout`, stable terms, action id를 표시하는지 확인한다.
- Web generated data manifest/count와 content bundle freshness를 확인한다.

후속 후보:

1. 이 slice는 `origin/main` 기반 완료 상태로 유지한다.
2. 전투 후속은 여러 runtime encounter에서 반복 가치가 확인될 때만 presentation metadata 정리 또는 Rust combat resolver로 승격한다.
3. 다음 active 방향은 아래 0.10 storypack/world 일반화다.

## 0.10 2026-05-29 active direction: storypack/world 일반화와 무협 기준팩

사용자 결정에 따라 프로젝트 방향을 “회사 아포칼립스 전용 게임”에서 “storypack/world 기반 선택지 생존 엔진 + Web/default 이구학지 storypack”으로 재정렬한다. 첫 비-office 기준팩은 `wuxia_jianghu_pack`이며, 최신 canonical story는 Notion에서 갱신된 **이구학지 — 천기록**이다. 2026-05-31에는 Notion-origin `야근몽`을 legacy office runtime을 대체하지 않는 별도 office-family 후보 `yageunmong_pack`으로 승격했다.
현재 상태: 설계 문서화 완료, 최신 무협 story 반영 완료, 야근몽 candidate docs/DB 반영 완료, machine-readable storypack DB/preview mode 결정 완료, 무협 runtime preview(`wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`)와 Web/terminal wiring 완료. 2026-06-01 `0.0b` 이후 Web/default storypack은 이구학지이며, office content는 legacy/parity 기준팩으로 남긴다.

문제 인식:

- 기존 문서와 런타임 콘텐츠는 `escape from the office`에 강하게 묶여 있다.
- storypack 형태로 확장하려면 office가 아닌 세계관에서도 성립하는 시스템 기준이 필요하다.
- 첫 비-office 기준팩은 현대 회사원이 본인 몸과 출근복장 그대로 무협 세계에 전이되는 이구학지 무협 세계관으로 정한다.

반영 결과:

- `docs/design/Storypack_World_Model.md`를 추가해 `world_id`, `storypack_id`, surface, resource display alias, route hook을 정리했다.
- `docs/content/storypacks/wuxia_jianghu_pack.md`를 이구학지 — 천기록 무협 기준팩으로 갱신했다.
- `docs/content/encounter_db/wuxia_jianghu_pack.md`에 `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`를 포함한 7개 후보 situation card를 정리했다.
- `docs/content/storypacks/yageunmong_pack.md`와 `docs/content/encounter_db/yageunmong_pack.md`를 추가해 회사 자각몽/현실 앵커/각성편린/퇴근 게이트 후보를 별도 office-dream candidate로 정리했다.
- `docs/content/storypack_db/storypacks.json`와 `docs/content/storypack_db/encounter_situations.json`를 추가해 design-time storypack DB 참조 무결성을 검증한다.
- `docs/dev/Storypack_Runtime_Preview_Mode.md`를 추가해 첫 non-office runtime prototype은 separate preview mode first로 진행하고, 기본 office bundle과 무협 preview bundle을 섞지 않는다고 결정했다.
- `wuxia_commute_rift_arrival`을 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml` source와 별도 generated preview bundle로 구현했다. Rust fixture와 Web generated preview artifact는 기본 `content.bundle.json`와 분리된다.
- `wuxia_heuksa_bang_first_fight`를 같은 preview source/bundle에 추가했고, `escape-terminal --storypack-preview wuxia_jianghu_pack`와 Web start screen preview launcher로 기본 office runtime을 건드리지 않는 opt-in entrypoint를 구현했다.
- `wuxia_cheonggi_record_first_fragment`를 첫 난투 후 천기록 future hook preview로 추가했다. `cheonggi_record_notebook` item과 `wuxia_first_fragment_seen` achievement도 preview bundle에만 들어간다.
- `wuxia_seo_harin_rescue`를 first fight/first fragment 뒤 서하린 구조, 외지인 조사, 청류문 보호/감시 bridge preview로 추가했다. `cheongryu_outer_courtyard` location과 `seo_harin_rescue_resolved`/`taken_under_watch`/`outsider_claim_recorded` hooks는 preview bundle에만 들어간다.
- `wuxia_cheongryu_apprentice_entry`를 rescue 이후 청류문 수습생/잡역/서고 bridge preview로 추가했고, `work_chore_token`과 `cheongryu_apprentice_entry_resolved`/`cheongryu_trial_started` hooks는 preview bundle에만 들어간다.
- `wuxia_cheongryu_raid_route_split`를 apprentice 이후 청류문 습격 route-pressure preview로 추가했고, 정파/사파/천기·귀환 route flags/clues/log/presentation만 preview bundle에 남겼다.
- `wuxia_cheongryu_raid_wounded_fallback`를 raid split fallback 이후 부상자 대피 deferred bridge preview로 추가했고, route graph/faction schema 없이 `deferred_route_reopened`와 route starter flags를 남겼다.
- `wuxia_baekdo_medicine_debt`를 첫 정파 route opener runtime slice로 추가했다. direct/deferred 정파 branch가 공유하는 `righteous_route_started` + `cheongryu_rebuild_thread`만 eligibility로 쓰고, `baekdo_alliance_debt`/`baekdo_medicine_debt`는 flavor hook으로 남긴다.
- `wuxia_black_heaven_escape_price`를 첫 사파 route opener runtime slice로 추가했다. direct/deferred 사파 branch가 공유하는 `sapa_route_started` + `dowol_debt`만 eligibility로 쓰고, `black_heaven_deal_marked`/`black_heaven_escape_marker`는 flavor hook으로 남긴다.
- `route_opener_followup_after_black_heaven` docs-only handoff 결과, 다음 runtime 후보를 천기·귀환 opener `wuxia_heavenly_archive_previous_outsiders`로 결정했다. direct/deferred 천기 branch가 공유하는 `cheonggi_return_route_started` + `cheonggi_record_targeted`만 eligibility로 쓰고, `heavenly_archive_contact`/`heavenly_archive_triage_map_seen`는 flavor hook으로 남긴다.
- `wuxia_heavenly_archive_previous_outsiders`를 첫 천기·귀환 route opener runtime slice로 추가했다. direct/deferred 천기 branch가 공유하는 `cheonggi_return_route_started` + `cheonggi_record_targeted`만 eligibility로 쓰고, `heavenly_archive_contact`/`heavenly_archive_triage_map_seen`는 flavor hook으로 남긴다.
- `route_opener_followup_after_heavenly_archive` docs-only handoff 결과, deferred-offer card `wuxia_wounded_shelter_dawn_offers`를 선택했고 preview runtime으로 구현했다. `stabilize_wounded_until_dawn` branch가 남긴 `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`를 받아, route graph나 any-of schema 없이 피난처 새벽 제안으로 메인 흐름을 다시 연다.
- `AGENTS.md`, `README.md`, `docs/00_Index.md`, `docs/design/Storypack_Encounter_DB.md`, `docs/dev/Checklist.md`를 office-only 표현에서 storypack-capable 표현으로 조정했다.

유지 범위:

- 기존 office runtime 콘텐츠는 삭제하지 않는다.
- `escape from the office`는 legacy/parity 기준팩으로 유지한다. 현재 Web/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, `escape-office` save/localStorage key는 그대로 유지한다.
- Rust GameCore / `ScenePage` / WASM JSON boundary는 계속 renderer-neutral truth를 소유한다.
- non-office runtime content와 `yageunmong_pack` 같은 후보 storypack은 명시적인 storypack preview bundle/flag 경로에서만 투입한다.
- save/localStorage key migration과 천외편린/각성편린 3택 성장 schema는 아직 열지 않는다.

후속 후보:

1. `route_midgame_continuity_after_wounded_shelter`: 다음 docs-only handoff. 세 route opener와 deferred-offer card 구현 이후 post-opener midgame continuity를 route별 3개 card, 공통 bridge, opened flags 기반 schema-less bridge 중 무엇으로 열지 비교한다.
2. `yageunmong_late_night_desk_awake`: 야근몽을 열 경우 기본 office bundle을 대체하지 않는 별도 storypack preview 첫 후보.
3. display alias pass: `health/sanity/battery/hunger/thirst/danger` 내부 field는 유지하되 world별 표시 이름을 분리할지 검토한다.

## 0.11 2026-05-31 idea_box batch: storypack system / 야근몽 후보 문서화

이 섹션은 `idea_box/done/2026-05-29-notion-storypack-system.md`와 `idea_box/done/2026-05-29-notion-office-yageunmong.md`를 backlog 순서대로 감사한 결과다.
현재 상태: docs/data 후보 반영 완료, runtime 구현 미착수, live Notion reference check 완료.

반영 결과:

- 기존 storypack/world 일반화, machine-readable storypack DB, preview mode, `wuxia_jianghu_pack` 반영은 `storypack-system` 아이디어의 큰 축을 이미 부분 반영하고 있었다.
- 빠진 축이었던 `회사 스토리팩: 야근몽`을 별도 office-family candidate로 승격했다.
- `docs/content/storypacks/yageunmong_pack.md`를 추가해 회사 자각몽, 현실 앵커, 각성편린, 퇴근 게이트, 퇴근을 잊은 나 자신을 storypack obligation으로 정리했다.
- `docs/content/encounter_db/yageunmong_pack.md`를 추가해 runtime 승격 전 상황 카드 6개를 만들었다.
- `docs/content/storypack_db/storypacks.json`와 `docs/content/storypack_db/encounter_situations.json`에 `yageunmong_pack` record와 후보 카드 6개를 추가했다.
- `docs/design/Storypack_World_Model.md`, `docs/design/Storypack_Encounter_DB.md`, README/Index/Checklist를 `isolation_pack` / `yageunmong_pack` / `wuxia_jianghu_pack` 3후보 상태에 맞게 동기화했다.
- Notion API로 원본 Markdown을 다시 가져와 local snapshot과 공백 정규화 기준으로 일치함을 확인했다. 따라서 두 idea entry는 `done` 처리했고 `idea_box/done/`으로 옮겼다.

중요 경계:

- `yageunmong_pack`은 legacy office runtime을 대체하지 않는다.
- Web/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다. 기존 `src/tui_adv/data/*.yaml` / 기본 office bundle은 legacy/parity artifact로 유지한다.
- 야근몽 runtime은 별도 storypack preview 또는 명시적 flag 경로가 필요하다.
- 각성편린 3택은 천외편린과 같은 성장 문법을 공유할 수 있지만, 첫 runtime slice에서는 새 reward/ability schema를 열지 않는다.

후속 후보:

1. 야근몽 runtime 후보를 열 경우 첫 slice는 `yageunmong_late_night_desk_awake` preview 또는 각성편린 3택 preview 중 하나만 선택한다.
2. 기본 office bundle과 섞지 않고, explicit storypack preview bundle 또는 preview flag 경로를 사용한다.
3. 무협 후속 결정은 아래 0.12 섹션에서 `wuxia_heuksa_bang_first_fight` 우선으로 확정했다.

## 0.12 2026-05-31 docs-only decision: 무협 후속 preview slice

현재 상태: 설계/handoff 완료, runtime YAML/Rust/Web 구현 미착수.

결정:

- 다음 구현 slice는 같은 `storypack_preview` mode에서 `wuxia_heuksa_bang_first_fight`를 추가하는 것으로 확정한다.
- `preview launcher/UI wiring`은 이번 다음 slice의 선행 조건이 아니라 follow-up 후보로 둔다. 이미 explicit preview export/check 경로와 Rust/Web preview bundle artifact가 있으므로, 두 번째 encounter의 content/schema/parity 검증은 launcher 없이도 가능하다. Launcher는 플레이어 opt-in UX를 개선하는 작업이지 `wuxia_heuksa_bang_first_fight`의 구현 의존성이 아니다.
- `yageunmong_pack`은 이번 무협 후속의 주 대상이 아니며, 별도 storypack preview 후보로만 유지한다.

`wuxia_heuksa_bang_first_fight` 설계 경계:

- 목적: arrival 직후, 이 세계의 폭력이 실제이고 현대 회사원 습관/소지품이 제한적으로만 통한다는 사실을 짧은 schema-less combat intervention encounter로 보여준다.
- 시작 조건: `wuxia_commute_rift_arrival` 이후 preview route에서만 도달한다. 권장 runtime 위치는 `jianghu_market_street` 또는 기존 preview transit을 최소 확장한 시장/골목 입구 위치이며, phase는 `first_brawl`이다.
- 권장 조건: `conditions.locations: [jianghu_market_street]`, `required_flags`는 arrival 선택이 남긴 `wuxia_arrival_hidden` 또는 동등한 arrival-resolved flag, `forbidden_flags: [heuksa_bang_first_fight_resolved]`. 양쪽 arrival 선택지를 모두 first fight로 잇고 싶다면 새 any-of schema를 열지 말고, 기존 `add_flags`만 사용해 두 arrival outcome에 공통 `wuxia_arrival_resolved` flag를 추가한다.
- stable choice id 후보:
  - `run_toward_open_street` — fallback/safe retreat. 부상은 줄지만 흑사방 시선을 남긴다.
  - `deescalate_with_words` — social probe. 말로 시간을 벌고 서하린 구조 hook을 당긴다.
  - `swing_commute_bag` — improvised item use. 가방/출근 물건을 방패처럼 쓰지만 손상 flag를 남긴다.
  - `loosen_tie_and_drop_shoes` — combat reposition. 정장/구두 패널티를 줄이고 기동성 clue를 남긴다.
  - `crash_in_with_body` — high-risk body check. 잠깐 버티거나 더러운 승리 flag를 만들 수 있지만 health cost가 가장 크다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation`만 사용한다.
- 새 `CombatState`, 전투 HP 숫자전, 스킬/쿨타임, combat resolver, reward/ability schema, 천외편린 3택 성장은 열지 않는다.
- 천외편린은 이번 slice에서 직접 구현하지 않고, 첫 전투 후 `wuxia_cheonggi_record_first_fragment` future work로 남긴다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

구현 세션 handoff:

- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - 필요 시 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
  - tests: `tests/test_web_data_export.py`, Rust `escape-core` content bundle metadata/fixture tests, `crates/escape-terminal/tests/cli_smoke.rs`, 필요 시 `crates/escape-wasm` JSON-boundary test
  - docs sync: `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, 이 파일, `docs/dev/Checklist.md`
- 작성할 테스트:
  - exporter check가 preview bundle에 `wuxia_commute_rift_arrival`과 `wuxia_heuksa_bang_first_fight`를 모두 포함하고, 기본 office bundle에는 무협 encounter가 없음을 검증한다.
  - Rust content bundle load가 preview runtime metadata와 두 번째 encounter id를 읽는지 검증한다.
  - terminal smoke가 preview bundle에서 first fight scene/action ids/stable terms를 렌더링하는지 검증한다.
  - Web generated preview bundle이 Rust fixture와 같은 encounter id/count/action id를 갖는지 검증한다.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - Rust tmp toolchain policy를 적용한 뒤 `cargo test -p escape-core --test content_bundle`, `cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_first_fight`, 필요 시 `cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location`
  - `git diff --check`
- 구현 결과: preview source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, generated preview bundle은 `crates/escape-core/fixtures/content/storypack-preview/`와 `web/src/data/generated/storypack-preview/`에만 두었다.
- Web/Rust/terminal parity 확인 항목: runtime metadata 유지, default location 유지, arrival와 first fight encounter id/action id 일치, preview bundle에만 무협 콘텐츠 존재, default office bundle에 무협 id 부재, `ScenePage`/action id renderer-neutral 표시 유지.
- 구현하지 말아야 할 것: preview launcher/UI wiring, storypack 선택 UI, 기본 office bundle 변경, `escape-office` key rename, 천외편린/각성편린 3택 reward schema, yageunmong runtime, 전체 전투 시스템 재설계.

## 0.13 2026-05-31 runtime slice: 무협 preview launcher/UI wiring

현재 상태: 구현 완료.

결정:

- `preview launcher/UI wiring`은 필요하다고 판단했다. 이유는 `wuxia_commute_rift_arrival`와 `wuxia_heuksa_bang_first_fight`가 이미 separate preview bundle에 들어갔지만, 사용자가 Web/terminal에서 기본 office runtime을 건드리지 않고 그 bundle을 선택하는 명시적 entrypoint가 없으면 preview가 개발자용 bundle path 지식에만 묶이기 때문이다.
- 이 slice는 storypack selection/save migration이 아니라 opt-in preview launcher다. 기본 `escape-office` save/localStorage key와 기본 office bundle은 유지한다.

구현 결과:

- Terminal: `escape-terminal --scene content --storypack-preview wuxia_jianghu_pack ...`가 built-in preview fixture를 선택한다. 기존 `--content-bundle <path>` 경로도 유지하고, 두 옵션은 동시에 사용할 수 없다.
- Web: start screen에 `wuxia_jianghu_pack` preview launcher를 노출하고, `web/src/core/contentBundles.ts`에서 기본 office bundle과 Web generated storypack preview bundle을 별도 registry로 관리한다.
- Save boundary: Web storypack preview run은 `escape-office.rust.save.v1` / `escape-office.last-run-summary.v1`를 쓰지 않는다. 기본 office continue/new-game UX는 기존 save key를 그대로 사용한다.
- Default office boundary: `src/tui_adv/data/*.yaml`, `crates/escape-core/fixtures/content/content.bundle.json`, `web/src/data/generated/content.bundle.json`는 변경하지 않는다.

검증 포인트:

- Terminal smoke는 `--storypack-preview wuxia_jianghu_pack`로 first fight까지 scripted route를 실행하고 `dev_desk`가 나타나지 않음을 확인한다.
- Web unit tests는 start screen preview launcher, default office bundle과 storypack preview bundle 분리, `wuxia_heuksa_bang_first_fight` 포함 여부를 검증한다.
- Web build/Vitest는 `/tmp` scratch copy에서 실행한다. repo 내부 `web/node_modules`는 만들지 않는다.

## 0.14 2026-05-31 runtime slice: 무협 `wuxia_cheonggi_record_first_fragment`

현재 상태: 구현 완료.

결정:

- 같은 `wuxia_jianghu_pack` `storypack_preview` bundle에 `wuxia_cheonggi_record_first_fragment`를 세 번째 encounter로 추가했다.
- 이 slice의 역할은 첫 난투 직후 천기록/천외편린 future hook을 보여주는 schema-less preview다. 정식 청류문 수습생/서고 구간과 완전한 천외편린 3택 성장/reward는 아직 열지 않는다.
- 접근 조건은 `conditions.locations: [jianghu_market_street]`, `required_flags: [heuksa_bang_first_fight_resolved]`, `forbidden_flags: [cheonggi_record_first_fragment_resolved]`다. first fight의 어떤 선택지를 골라도 공통 resolved flag로 이어진다.
- stable choice ids는 `choose_guard_basics`, `choose_keep_feet_moving`, `choose_failure_log`, fallback `close_notebook_without_choice`다.
- outcome hook은 기존 schema의 `resources.sanity`, `add_items`, `add_flags`, `add_clues`, `log`, optional `presentation/effect_cues`만 사용한다.
- 새 reward/ability/combat schema, 실제 천외편린 3택 lock-in 시스템, full fragment choice UI, 기본 office bundle 변경, `escape-office` save/localStorage key 변경은 하지 않았다.

구현 결과:

- Source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 `wuxia_cheonggi_record_first_fragment`를 추가했고, `items.yaml`에 `cheonggi_record_notebook`, `achievements.yaml`에 `wuxia_first_fragment_seen`을 추가했다.
- Generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성했다.
- Runtime parity: Python exporter는 preview counts/items/encounter ids를 검증하고, Rust content fixture는 조건/choice/presentation을 인덱싱하며, WASM JSON boundary와 SuperLightTUI smoke는 arrival → market → first fight → first fragment 경로를 검증한다.
- Web boundary: `web/src/core/contentBundles.test.ts`는 default office bundle에 무협 IDs가 없고 preview registry에 세 encounter가 들어 있음을 검증한다.

다음 후보:

1. `wuxia_seo_harin_rescue`: first fight/fragment 이후 서하린 개입, 외지인 조사, 청류문 보호/의심 hook을 schema-less encounter로 연결한다.
2. `wuxia_cheongryu_apprentice_entry`: 수습생/잡역/서고 정리 bridge를 열어 천기록 발현의 정식 위치를 회수한다.
3. 천외편린 3택 reward/ability schema는 위 bridge가 충분히 검증된 뒤 별도 design/runtime slice로 연다.

## 0.15 2026-05-31 docs-only decision: 무협 `wuxia_seo_harin_rescue` 후속 slice

현재 상태: 설계/handoff 완료, 2026-06-01 runtime YAML/Rust/Web preview 구현 완료.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | 무협 후속 개발에 필요한 gap | `wuxia_seo_harin_rescue` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | 세 preview encounter와 preview launcher/UI wiring 완료. 다음 후보는 서하린 구조 또는 청류문 수습생 bridge로 남아 있었다. | 후보를 하나로 확정하고 다음 구현 세션용 파일/테스트/검증 명령이 필요하다. | 다음 구현 slice를 `wuxia_seo_harin_rescue`로 확정한다. | 수정 | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | `storypack_preview` metadata, terminal `--storypack-preview`, Web preview launcher, no default bundle mixing 경계가 완료됐다. | 네 번째 encounter도 같은 preview bundle에만 넣는다는 handoff가 필요하다. | launcher가 선행 조건이 아니므로 구조 content를 바로 추가할 수 있다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | 이구학지 — 천기록 spine은 `rescue_and_investigation` → `cheongryu_apprenticeship` 순서를 가진다. | 현재 runtime preview는 첫 편린까지 있어 관계/거점 bridge가 비어 있다. | 서하린은 early rescuer/mentor 후보이며 청류문 수습생으로 가는 문을 연다. | 수정 | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | `wuxia_seo_harin_rescue` 카드가 후보로 존재하지만 choice/outcome/schema boundary가 얕다. | stable choice id, fallback, flags/clues/log/destination hook, schema non-goal을 구체화해야 한다. | 이번 설계의 중심 카드다. | 수정 | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/storypacks.json`, `docs/content/storypack_db/encounter_situations.json`, `docs/content/storypack_db/README.md` |
| preview runtime source/artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, `crates/escape-core/fixtures/content/storypack-preview/`, `web/src/data/generated/storypack-preview/` | source와 generated preview bundle에는 `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment` 3개 encounter가 있고 runtime metadata는 `storypack_preview`다. | 구현 세션에서 네 번째 encounter와 필요 시 `cheongryu_outer_courtyard` 위치를 추가하고 preview artifacts만 재생성한다. | source/artifact 대상이지만 이번 docs-only 세션에서는 read-only다. | 수정 안 함 | 다음 구현 세션에서 `locations.yaml`, `encounters.yaml`, generated preview bundle 2개 |
| default office boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, Web save/localStorage key | 기본 office bundle에는 무협 id가 없고 `escape-office` key가 유지된다. | 후속 구현도 이 경계를 깨지 않아야 한다. | 구조 slice도 별도 preview bundle에서만 실행한다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_web_data_export.py`, `tests/test_docs_contract.py`, `tests/test_storypack_db.py`, Rust/Web tests | 현재 테스트는 preview count/id, launcher, first fragment parity를 검증한다. | 구현 세션에서 네 번째 encounter id/action flow, default office 무변경, Rust/WASM/terminal/Web parity 테스트를 추가해야 한다. | `wuxia_seo_harin_rescue` route smoke의 검증 대상이다. | docs/data DB를 바꾸면 targeted pytest 실행 | 구현 세션에서 Python/Rust/WASM/terminal/Web test 파일 |

결정:

- 다음 구현 slice는 `wuxia_seo_harin_rescue`로 확정했고, 2026-06-01 preview runtime에 구현했다.
- 이유: 현재 preview route는 arrival → first fight → first fragment까지 완료되어 “폭력”, “천기록 future hook”은 보였지만, 주인공을 다음 거점과 관계망으로 옮기는 구조/조사 bridge가 없다. `wuxia_cheongryu_apprentice_entry`는 서하린의 보호/감시, 청류문 이름, 치료/채무 hook이 있어야 자연스럽게 시작된다.
- `preview launcher/UI wiring`은 이미 완료되어 선행 조건이 아니다. 새 launcher나 storypack selection UI를 다시 열지 않는다.
- `yageunmong_pack`은 이번 무협 후속의 주 대상이 아니며 별도 preview 후보로만 유지한다.

`wuxia_seo_harin_rescue` 설계 경계:

- 목적: first fight/first fragment가 남긴 부상, 흑사방 시선, 사원증/천기록 오해를 서하린 구조, 외지인 조사, 청류문 보호/감시, 다음 수습생 bridge로 연결한다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [jianghu_market_street]`, `required_flags: [heuksa_bang_first_fight_resolved, cheonggi_record_first_fragment_resolved]`, `forbidden_flags: [seo_harin_rescue_resolved]`를 권장한다.
- 위치/phase: 시작 위치는 `jianghu_market_street`, phase는 `rescue_and_investigation`, outcome destination 후보는 새 preview location `cheongryu_outer_courtyard`다.
- 기존 fragment smoke를 가로막지 않기 위해 rescue는 `cheonggi_record_first_fragment_resolved` 뒤에 붙인다. full story 순서 재배치는 별도 sequence 재편 slice로 둔다.
- stable choice id 후보:
  - `tell_plain_truth` — fallback/safe honesty. 진실을 말하지만 감시 대상으로 이동한다.
  - `ask_for_medical_help_first` — survival priority. 치료/안전은 얻지만 채무 hook을 남긴다.
  - `explain_company_and_commute` — workplace memory probe. 현대어가 통하지 않아 오해와 sanity cost를 남긴다.
  - `show_cheonggi_record_page` — risky record disclosure. 천기록을 보여 도움/위험을 동시에 부른다.
  - `hide_employee_badge` — high-risk concealment. 사원증/수첩은 숨기지만 의심 flag를 키운다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `seo_harin_rescue_resolved`, `seo_harin_intervened`, `taken_under_watch`, `outsider_claim_recorded`, `rescue_debt_recorded`, `seo_harin_noticed_cheonggi_record`, `cheonggi_record_must_be_hidden`; `cheongryu_name_heard`, `sect_identity_matters`, `company_words_fail_clue`, `sect_protection_has_price`, `notebook_draws_sect_attention`; 로그는 “구조는 구원이 아니라 보호와 감시의 시작”이라는 톤을 유지한다.
- 새 `RelationScore`, `DebtLedger`, `FactionStanding`, healing schema, companion schema, combat/reward/ability schema, 천외편린 3택 lock-in UI는 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

구현 세션 handoff:

- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`: `cheongryu_outer_courtyard` 또는 동등한 청류문 외곽 거점 위치를 추가하고 기존 market/roadside 연결을 최소화한다.
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_seo_harin_rescue`를 `wuxia_cheonggi_record_first_fragment` 뒤에 추가한다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - 필요 시 docs sync: `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, 이 파일, `docs/dev/Checklist.md`.
- 작성할 테스트:
  - Python exporter/storypack preview test가 preview bundle에 `wuxia_seo_harin_rescue`와 `cheongryu_outer_courtyard`를 포함하고 기본 office bundle에는 무협 id가 없음을 검증한다.
  - Rust `escape-core` content bundle fixture test가 네 번째 encounter의 conditions, choice ids, destination, presentation/effect cues를 읽는지 검증한다.
  - WASM JSON boundary test가 preview route에서 first fragment 뒤 rescue scene/action ids까지 도달하는지 검증한다.
  - `escape-terminal --storypack-preview wuxia_jianghu_pack --tui-smoke` scripted route가 rescue title/action ids/stable terms를 렌더링하는지 검증한다.
  - Web generated preview bundle/registry test가 Rust fixture와 encounter id/count/action id parity를 유지하고 default office save key를 쓰지 않음을 검증한다.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - Rust tmp toolchain policy를 적용한 뒤 `cargo test -p escape-core --test content_bundle`, `cargo test -p escape-wasm json_boundary_reaches_wuxia_seo_harin_rescue_through_preview_bundle`, `cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_seo_harin_rescue` 같은 targeted tests를 추가/실행한다.
  - `git diff --check`
- 생성 artifact 위치: preview source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, generated preview bundle은 `crates/escape-core/fixtures/content/storypack-preview/`와 `web/src/data/generated/storypack-preview/`에만 둔다.
- Web/Rust/terminal parity 확인 항목: preview runtime metadata 유지, default location 유지, 기존 arrival/first fight/first fragment route smoke 유지, rescue encounter id/action id/destination id 일치, default office bundle에 무협 id 부재, `ScenePage`/action id renderer-neutral 표시 유지.
- 구현하지 말아야 할 것: 기본 office bundle 변경, `escape-office` key rename, yageunmong runtime, `wuxia_cheongryu_apprentice_entry`까지 한 번에 구현, 천외편린/각성편린 3택 reward schema, relation/debt/faction/companion schema, 전체 전투 시스템 재설계.

## 0.16 2026-05-31 docs-only follow-up: 무협 `wuxia_cheongryu_apprentice_entry` 후보 설계

현재 상태: 설계/handoff와 preview runtime 구현 완료. `wuxia_seo_harin_rescue` runtime slice가 남긴 hook을 받아 `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`까지 같은 preview bundle에 landing되었다. 현재 다음은 route opener 선택/설계 handoff다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | 무협 후속 개발에 필요한 gap | `wuxia_cheongryu_apprentice_entry` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | `wuxia_seo_harin_rescue`는 다음 구현 slice로 설계/handoff 완료. `wuxia_cheongryu_apprentice_entry`는 2순위 후보로 남아 있었다. | rescue 구현 뒤 바로 착수할 수 있도록 apprenticeship 후보도 시작 조건, choice id, outcome hook, 검증 명령을 고정해야 한다. | rescue가 남기는 보호/감시/채무 hook을 받아 청류문 수습생/잡역/서고 bridge를 연다. | 수정 | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | `storypack_preview` metadata, terminal/Web preview launcher, no default bundle mixing은 완료. | rescue 다음 encounter도 같은 preview bundle에만 추가하고 launcher/UI를 다시 열지 않는다는 경계가 필요하다. | 새 preview mode가 아니라 existing opt-in preview의 후속 content다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | spine은 `rescue_and_investigation` 다음 `cheongryu_apprenticeship`을 요구한다. | 수습생 편입이 어떤 기능을 검증하는지, 천기록/천외편린과 무엇을 분리하는지 명확히 해야 한다. | 이 encounter는 소속/채무/잡일/훈련 허가를 처음 runtime-visible하게 만든다. | 수정 | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | `wuxia_cheongryu_apprentice_entry` 카드가 후보로 존재하지만 choice/outcome/schema boundary가 얕다. | stable choice id, fallback, required flags, optional branch hook, allowed schema, forbidden schema를 구현 handoff 수준으로 구체화해야 한다. | 이번 설계의 중심 카드다. | 수정 | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/storypacks.json`, `docs/content/storypack_db/encounter_situations.json`, `docs/content/storypack_db/README.md` |
| preview runtime source/artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, `crates/escape-core/fixtures/content/storypack-preview/`, `web/src/data/generated/storypack-preview/` | source와 generated preview bundle에는 `wuxia_cheongryu_chore_sparring`을 포함해 `wuxia_cheongryu_raid_wounded_fallback`까지 8개 encounter가 있다. | 다음은 route opener 후보를 바로 구현하기보다 start conditions와 first opener order를 정해야 한다. | source/artifact 구현 완료 상태를 docs에 반영했다. | 수정 | 후속 docs-only handoff에서 plan/docs/next_goal |
| default office boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, Web save/localStorage key | 기본 office bundle에는 무협 id가 없고 `escape-office` key가 유지된다. | apprenticeship slice도 이 경계를 깨지 않아야 한다. | 청류문 수습생 status를 기본 office runtime으로 섞지 않는다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_web_data_export.py`, `tests/test_docs_contract.py`, `tests/test_storypack_db.py`, Rust/Web tests | DB validator는 후보 카드 구조를 검증하고, runtime tests는 현재 6개 preview encounter와 launcher parity를 검증한다. | 다음 구현 세션에서는 raid fallback→wounded fallback route smoke를 추가해야 한다. | route choice id/action id parity와 default office 미변경 검증 대상이다. | docs/data DB 변경으로 targeted pytest 실행 | 구현 세션에서 Python/Rust/WASM/terminal/Web test 파일 |

결정:

- `wuxia_cheongryu_apprentice_entry`는 `wuxia_seo_harin_rescue` 이후의 follow-up 후보로 설계했고, 2026-06-01 preview runtime에 구현 완료했다.
- `wuxia_cheongryu_raid_wounded_fallback`도 preview runtime에 구현 완료했다. 다음은 route opener 후보를 바로 추가하기 전에 direct/deferred branch를 모두 받는 start conditions와 첫 opener order를 문서화하는 일이다.
- `preview launcher/UI wiring`은 이미 완료되어 선행 조건이 아니다. 새 preview selector나 save-key migration을 다시 열지 않는다.
- `yageunmong_pack`은 이번 무협 후속의 주 대상이 아니며 별도 preview 후보로만 유지한다.

`wuxia_cheongryu_apprentice_entry` 설계 경계:

- 목적: 구조 이후 주인공을 청류문 보호 대상에서 수습생/객식/잡역으로 전환하고, 소속·채무·잡일·수련 허가·서고 curiosity hook을 기존 encounter schema만으로 연다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [seo_harin_rescue_resolved, taken_under_watch]`, `forbidden_flags: [cheongryu_apprentice_entry_resolved]`를 권장한다.
- `rescue_debt_recorded`는 rescue choice별 optional hook일 수 있으므로 필수 조건으로 요구하지 않는다. 모든 rescue branch가 apprenticeship으로 이어져야 하면 rescue 구현 때 공통 `taken_under_watch` flag를 보장한다.
- 위치/phase: 시작 위치는 `cheongryu_outer_courtyard`, phase는 `cheongryu_apprenticeship`. 추가 위치 없이 같은 courtyard에서 resolved 상태를 만들고, 반복 잡일 deck/서고 내부는 future work로 둔다.
- stable choice id 후보:
  - `accept_three_month_trial` — fallback/safe acceptance. 석 달 잡역/수습 조건을 받아들이고 보호·채무·잡일 루프를 연다.
  - `request_martial_training_immediately` — impatience probe. 즉시 무공을 요구하지만 문파 규칙과 수련 허가 조건을 배운다.
  - `organize_chores_like_workflow` — workplace skill translation. 회사식 업무 분해/동선 최적화를 잡일에 적용한다.
  - `inspect_archive_during_chore` — risky curiosity. 서고/천기록 resonance를 엿보지만 감시와 피로를 남긴다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `cheongryu_apprentice_entry_resolved`, `cheongryu_trial_started`, `sect_debt_accepted`, `chore_training_open`, `training_request_denied`, `sect_rules_explained`, `modern_workflow_noticed`, `chore_roster_rewritten`, `old_archive_locked_seen`, `archive_curiosity_marked`, `seo_harin_mentor_thread`, `sect_master_watch`; `training_starts_with_labor`, `protection_is_not_membership`, `training_requires_chore_credit`, `workflow_thinking_translates_to_training`, `old_archive_locked`, `cheonggi_record_resonates_near_archive`, `sect_rules_written_in_chores`; 로그는 “보호는 소속이지만 공짜가 아니고, 잡일은 벌이 아니라 수련의 입구”라는 톤을 유지한다.
- 새 `RelationScore`, `DebtLedger`, `FactionStanding`, `TrainingXP`, `ChoreScheduler`, companion schema, combat/reward/ability schema, 천외편린 3택 lock-in UI는 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

구현 완료 기록:

- 구현 상태: `wuxia_cheongryu_apprentice_entry` runtime slice는 같은 preview source에 구현되어 있고, 모든 apprentice outcome이 최소 `cheongryu_apprentice_entry_resolved`, `cheongryu_trial_started`, `seo_harin_mentor_thread`, `destination_id: cheongryu_outer_courtyard`를 남긴다. 이 구현이 다음 raid route split의 선행 조건이다.
- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_seo_harin_rescue` 뒤에 `wuxia_cheongryu_apprentice_entry`를 추가한다.
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`: `cheongryu_outer_courtyard`가 이미 rescue slice에서 추가되지 않았다면 추가한다. 새 location이 불필요하면 같은 courtyard 유지.
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`: `work_chore_token` 같은 preview-only item을 실제로 outcome에 줄 때만 추가한다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - 필요 시 docs sync: `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, 이 파일, `docs/dev/Checklist.md`.
- 구현 검증:
  - Python exporter/storypack preview test가 preview bundle에 `wuxia_seo_harin_rescue`와 `wuxia_cheongryu_apprentice_entry`를 포함하고 기본 office bundle에는 무협 id가 없음을 검증한다.
  - Rust `escape-core` content bundle fixture test가 apprenticeship conditions, fallback choice, choice ids, flags/clues/items/log/presentation hook을 읽는지 검증한다.
  - WASM JSON boundary test가 preview route에서 first fragment → rescue → apprentice scene/action ids까지 도달하는지 검증한다.
  - `escape-terminal --storypack-preview wuxia_jianghu_pack --tui-smoke` scripted route가 apprentice title/action ids/stable terms를 렌더링하는지 검증한다.
  - Web generated preview bundle/registry test가 Rust fixture와 encounter id/count/action id parity를 유지하고 default office save key를 쓰지 않음을 검증한다.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - Rust tmp toolchain policy를 적용한 뒤 `cargo test -p escape-core --test content_bundle`, `cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_apprentice_entry_through_preview_bundle`, `cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_apprentice_entry` 같은 targeted tests를 추가/실행한다.
  - `git diff --check`
- 생성 artifact 위치: preview source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, generated preview bundle은 `crates/escape-core/fixtures/content/storypack-preview/`와 `web/src/data/generated/storypack-preview/`에만 둔다.
- Web/Rust/terminal parity 확인 항목: preview runtime metadata 유지, default location 유지, 기존 arrival/first fight/first fragment/rescue route smoke 유지, apprentice encounter id/action id/required flags 일치, default office bundle에 무협 id 부재, `ScenePage`/action id renderer-neutral 표시 유지.
- 구현하지 말아야 할 것: `wuxia_seo_harin_rescue`를 건너뛰는 apprenticeship 구현, 기본 office bundle 변경, `escape-office` key rename, yageunmong runtime, 천외편린/각성편린 3택 reward schema, relation/debt/faction/companion/training XP/chore scheduler schema, 전체 전투 시스템 재설계.

## 0.17 2026-06-01 runtime slice: 무협 `wuxia_cheongryu_raid_route_split` preview 구현

현재 상태: 설계/handoff와 2026-06-01 preview runtime 구현 완료. `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheonggi_record_first_fragment`의 공통 hook을 받아 `wuxia_cheongryu_raid_route_split`와 `wuxia_cheongryu_raid_wounded_fallback`가 같은 `storypack_preview` bundle에 landing되었다. 다음은 route opener 선택/설계 handoff다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | 무협 후속 개발에 필요한 gap | `wuxia_cheongryu_raid_route_split` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | 즉시 구현 순서는 rescue → apprentice → raid로 정리됐다. raid route split은 다음 구현 후보로 승격됐다. | 후속 구현에서 rescue/apprentice보다 앞서지 않는 순서와 구현 금지선을 유지해야 한다. | 이 card는 청류문 공통 루트가 충분히 쌓인 뒤 정파/사파/천기·귀환 route pressure를 여는 next candidate다. | 수정 | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | preview mode/launcher/UI wiring은 완료. 현재 planned follow-up은 raid route split이다. | raid도 별도 mode가 아니라 같은 preview bundle에서만 실험하되, default bundle과 save key를 건드리지 않아야 한다. | 중반 route split preview는 existing preview entrypoint만 사용한다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | spine은 `cheonggi_record_awakening` 이후 `cheongryu_raid`/`route_commitment`로 간다. | route commitment를 새 route/faction schema 없이 flags/clues/log로만 설계해야 한다. | 이 encounter는 첫 대형 분기 압박이지만, route system 자체 구현은 future work다. | 수정 | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | `wuxia_cheongryu_raid_route_split` 카드가 후보로 있으나 fallback, start conditions, outcome hook, schema boundary가 얕다. | stable choice id 4개, fallback, route flag hook, prerequisites, non-goals를 구체화해야 한다. | 이번 설계의 중심 카드다. | 수정 | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/storypacks.json`, `docs/content/storypack_db/encounter_situations.json`, `docs/content/storypack_db/README.md` |
| preview runtime source/artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, `crates/escape-core/fixtures/content/storypack-preview/`, `web/src/data/generated/storypack-preview/` | source/generated preview bundle에는 `wuxia_cheongryu_chore_sparring`을 포함해 `wuxia_cheongryu_raid_wounded_fallback`까지 8개 encounter가 있다. | route opener 후보는 direct and deferred route starter flags를 모두 받을 설계가 먼저 필요하다. | raid/wounded 구현 source/artifact 대상이다. | 수정 | 다음 docs-only handoff에서 plan/docs/next_goal |
| default office boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, Web save/localStorage key | 기본 office bundle에는 무협 id가 없고 `escape-office` key가 유지된다. | route split도 default office runtime으로 섞지 않아야 한다. | faction route flag는 preview storypack 안에서만 남긴다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_web_data_export.py`, `tests/test_docs_contract.py`, `tests/test_storypack_db.py`, Rust/Web tests | DB validator는 후보 카드 구조를 검증하고 runtime tests는 현재 8개 preview encounter를 검증한다. | route opener handoff 때 direct/deferred route starter coverage를 설계해야 한다. | route flag/action id parity와 default office 미변경 검증 대상이다. | docs/data DB 변경으로 targeted pytest 실행 | 후속 docs-only handoff에서 docs contract 갱신 |

결정:

- `wuxia_cheongryu_raid_route_split`는 기존 후보 목록에서 구체화한 candidate이며, 2026-06-01 preview runtime에 구현 완료했다.
- `wuxia_cheongryu_raid_wounded_fallback`가 같은 preview runtime에 구현 완료되었으므로, 즉시 다음은 route opener 선택/설계 handoff다.
- `wuxia_cheonggi_record_first_fragment`는 이미 preview runtime으로 구현된 foreshadow slice다. raid split은 이 구현의 `cheonggi_record_awakened`/`first_fragment_seen` hook을 재사용하되, 천외편린 3택 reward/ability schema를 확장하지 않는다.
- `preview launcher/UI wiring`은 이미 완료되어 선행 조건이 아니다. 새 selector, save-key migration, default bundle 변경을 열지 않는다.

`wuxia_cheongryu_raid_route_split` 설계 경계:

- 목적: 청류문 공통 루트가 쌓인 뒤 혈월교 습격을 통해 백도맹/흑천련/천기각·귀환 축의 route pressure를 처음 노출한다. 이 slice는 “route commitment 압박”을 보여주되, 완전한 faction reputation/route graph/ending system을 열지 않는다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]` 또는 future `cheongryu_raid_courtyard`, `required_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, cheonggi_record_awakened, first_fragment_seen]`, `forbidden_flags: [cheongryu_raid_route_split_resolved]`를 권장한다.
- `cheonggi_fragment_guard_basics_thread`/`cheonggi_fragment_footwork_thread`/`cheonggi_fragment_failure_log_thread`는 branch flavor로만 쓰고 eligibility 필수 조건으로 삼지 않는다. 모든 first-fragment branch가 raid로 이어져야 하므로 공통 `first_fragment_seen`과 `cheonggi_record_awakened`만 요구한다.
- 위치/phase: 시작 위치는 `cheongryu_outer_courtyard`, phase는 `[cheongryu_raid, route_commitment]`. 필요하면 구현 세션에서 preview-only `cheongryu_raid_courtyard` 또는 `raid_aftermath_shelter` location을 추가하되, 첫 설계는 location schema 확장 없이 existing location + destination hook으로 충분해야 한다.
- stable choice id 후보:
  - `evacuate_the_wounded_first` — fallback/safe human priority. route commitment를 지연하지만 서하린/부상자/청류문 생존 hook을 남긴다.
  - `defend_cheongryu_with_white_path` — righteous route pressure. 백도맹 지원을 받아 청류문을 지키지만 정치적 빚을 남긴다.
  - `trade_with_black_heaven` — sapa survival bargain. 흑천련과 거래해 생존 자원을 얻지만 신뢰/채무 hook을 남긴다.
  - `follow_heavenly_archive` — cheonggi/return truth pressure. 천기각 기록관을 따라 천기록/귀환 단서를 쫓지만 청류문 관계 위험을 남긴다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `cheongryu_raid_route_split_resolved`, `cheongryu_raid_survived`, `route_commitment_pressure`, `wounded_saved_flag`, `seo_harin_survived_raid`, `righteous_route_started`, `baekdo_alliance_debt`, `cheongryu_rebuild_thread`, `sapa_route_started`, `black_heaven_deal_marked`, `dowol_debt`, `cheonggi_return_route_started`, `heavenly_archive_contact`, `cheonggi_record_targeted`; `martial_knowledge_conflict`, `blood_moon_targets_cheonggi_record`, `white_path_help_has_price`, `black_heaven_bargain_has_teeth`, `heavenly_archive_knows_previous_outsiders`, `saving_people_delays_route_choice`; 로그는 “어느 편도 완전히 선하거나 안전하지 않고, 선택하지 않는 것도 대가가 있다”는 톤을 유지한다.
- 새 `FactionStanding`, `RouteGraph`, `BranchLock`, `CompanionDeath`, `MassCombat`, boss combat resolver, reward/ability schema, 천외편린 3택 성장 UI, multi-ending implementation은 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

구현 완료 기록:

- 선행 조건: rescue와 apprentice runtime slice가 preview source에 구현되어 있고, apprentice outcome이 `cheongryu_apprentice_entry_resolved`와 `cheongryu_trial_started`를 남기며, first-fragment preview의 `cheonggi_record_awakened`와 `first_fragment_seen` 공통 hook도 유지함을 확인했다.
- Source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 `wuxia_cheongryu_raid_route_split`를 추가했다. 새 location/item/achievement schema는 열지 않고 `cheongryu_outer_courtyard`를 재사용한다.
- Generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성했다.
- Tests: Python exporter/Web generated data, Rust `escape-core` fixture, `escape-wasm` JSON boundary, `escape-terminal` SuperLightTUI smoke, Web content bundle registry tests가 raid encounter id/count/action id/presentation/default-office separation을 검증한다.
- Route hooks: 모든 raid outcome은 `cheongryu_raid_route_split_resolved`, `cheongryu_raid_survived`, `route_commitment_pressure`, `destination_id: cheongryu_outer_courtyard`를 남긴다. fallback `evacuate_the_wounded_first`는 `route_commitment_deferred`와 `wounded_saved_flag`를 남겨 구현된 `wuxia_cheongryu_raid_wounded_fallback`으로 이어진다.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py::test_export_web_data_builds_wuxia_storypack_preview_bundle -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-core --test content_bundle preview_fixture_indexes_wuxia_first_fight`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_cheongryu_raid_route_split_through_preview_bundle`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_cheongryu_raid_route_split`
  - `cd web && npm exec --package vitest@3.2.4 -- vitest run --config /tmp/dudupunch0-tui-adv/vitest.content-bundles.config.mjs src/core/contentBundles.test.ts`
- 생성 artifact 위치: preview source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, generated preview bundle은 `crates/escape-core/fixtures/content/storypack-preview/`와 `web/src/data/generated/storypack-preview/`에만 둔다.
- Web/Rust/terminal parity 확인 항목: preview runtime metadata 유지, default location 유지, 기존 arrival/first fight/first fragment/rescue/apprentice route smoke 유지, raid encounter id/action id/route flag 일치, default office bundle에 무협 id 부재, `ScenePage`/action id renderer-neutral 표시 유지.
- 구현하지 않은 것: route opener, 기본 office bundle 변경, `escape-office` key rename, yageunmong runtime, faction/reputation/route graph/companion death/mass combat/boss combat schema, 천외편린/각성편린 3택 reward schema, 여러 엔딩 구현.

## 0.18 2026-05-31 docs-only follow-up: 무협 `wuxia_cheongryu_raid_wounded_fallback` 후보 설계

현재 상태: 설계/handoff 완료 후 2026-06-01 preview runtime 구현 완료. `wuxia_cheongryu_raid_wounded_fallback`는 raid split에서 fallback `evacuate_the_wounded_first`를 선택한 경우에만 열 deferred bridge로 landing되었다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | 무협 후속 개발에 필요한 gap | `wuxia_cheongryu_raid_wounded_fallback` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | 구현 backlog는 rescue → apprentice → raid split으로 고정. raid split fallback은 사람을 구하고 route 선택을 미루는 hook으로만 남아 있었다. | fallback이 route opener를 회피해 dead-end가 되지 않도록, route opener 전 공통 deferred follow-up을 설계해야 한다. | 이 card는 `route_commitment_deferred`/`wounded_saved_flag`를 받아 route pressure를 다시 열고 공통 spine에 합류시킨다. | 수정 | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | same `storypack_preview` mode와 launcher/UI wiring은 완료. 후속 content는 preview bundle 안에서만 추가한다. | wounded fallback도 별도 mode나 route graph가 아니라 existing preview bundle의 조건부 content임을 명시해야 한다. | 정파/사파/천기 route opener보다 먼저 설계하지만 구현은 raid split 이후다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | spine은 raid route commitment 이후 각 route opener로 갈 수 있다. | fallback branch가 “인간을 우선한 선택”으로 의미를 가지면서도 분기 선택을 다시 받을 bridge가 필요하다. | wounded fallback은 route 선택을 지연한 대가와 보상을 공통 flags/clues/log로 기록한다. | 수정 | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | raid split 카드는 `evacuate_the_wounded_first` fallback을 설계했지만 그 이후 카드가 없었다. | fallback 후속 id, start conditions, stable choice id, outcome hook, schema boundary를 고정해야 한다. | 이번 설계의 중심 카드다. | 수정 | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/storypacks.json`, `docs/content/storypack_db/encounter_situations.json`, `docs/content/storypack_db/README.md` |
| preview runtime source/artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, `crates/escape-core/fixtures/content/storypack-preview/`, `web/src/data/generated/storypack-preview/` | source/generated preview bundle에는 `wuxia_cheongryu_chore_sparring`을 포함해 `wuxia_cheongryu_raid_wounded_fallback`까지 8개 encounter가 있다. | route opener 구현 전 direct/deferred branch 조건을 고정해야 한다. | source/artifact 대상이며 구현 완료 상태를 docs에 반영했다. | 수정 | 후속 docs-only handoff에서 plan/docs/next_goal |
| default office boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, Web save/localStorage key | 기본 office bundle에는 무협 id가 없고 `escape-office` key가 유지된다. | deferred route follow-up도 default office runtime으로 섞지 않아야 한다. | fallback branch flags는 preview storypack 안에서만 남긴다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_storypack_db.py`, `tests/test_docs_contract.py`, Rust/Web tests | DB validator는 후보 카드 구조를 검증한다. runtime tests는 현재 8개 preview encounter를 검증한다. | future route opener 구현 때 direct/deferred branch route smoke를 추가해야 한다. | fallback choice/action id parity와 default office 미변경 검증 대상이다. | docs/data DB 변경으로 targeted pytest 실행 | 후속 docs-only handoff에서 docs contract 갱신 |

결정:

- `wuxia_cheongryu_raid_wounded_fallback`를 `evacuate_the_wounded_first` 이후를 받는 deferred-route follow-up 후보로 설계 완료한다.
- 이 후보는 정파/사파/천기 route opener보다 먼저 설계한다. 이유는 fallback branch가 사람을 구한 뒤 route 선택을 다시 받을 공통 재합류 지점이 없으면, 이후 route opener 설계가 direct branch만 기준으로 굳어질 수 있기 때문이다.
- `wuxia_cheongryu_raid_wounded_fallback`도 구현 완료했다. 다음은 route opener를 바로 구현하기 전 first opener order와 direct/deferred start conditions를 문서화하는 docs-only handoff다.
- 새 preview launcher/UI wiring은 필요 없다. 기본 office bundle과 `escape-office` save/localStorage key는 변경하지 않는다.

`wuxia_cheongryu_raid_wounded_fallback` 설계 경계:

- 목적: raid split에서 부상자를 먼저 대피시킨 선택을 dead-end가 아니라 “선택을 미룬 대가와 신뢰”로 기록하고, 정파/사파/천기 route opener 전에 route pressure를 다시 노출한다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [raid_aftermath_shelter]` 또는 구현 단순화를 위해 `[cheongryu_outer_courtyard]`, `required_flags: [cheongryu_raid_route_split_resolved, route_commitment_deferred, wounded_saved_flag, cheongryu_raid_survived]`, `forbidden_flags: [cheongryu_raid_wounded_fallback_resolved]`를 권장한다.
- `seo_harin_survived_raid`는 강한 flavor hook이지만 route 재합류의 필수 조건으로 만들지는 않는다. rescue/apprentice 구현 결과에 따라 이름이 달라질 수 있으므로 required flag는 direct fallback이 보장하는 공통 flags로 제한한다.
- 위치/phase: 시작 위치는 future `raid_aftermath_shelter` 또는 existing `cheongryu_outer_courtyard`, phase는 `[cheongryu_raid, route_commitment]`. 새 location을 추가하더라도 preview-only location으로 두고 location/state schema를 확장하지 않는다.
- stable choice id 후보:
  - `stabilize_wounded_until_dawn` — fallback/safe deferred recovery. route 선택을 한 번 더 미루되 부상자 명단과 shelter hook을 남긴다.
  - `ask_baekdo_for_medicine_not_command` — delayed righteous commitment. 백도맹의 약과 호위를 받지만 정치적 빚을 인정한다.
  - `trade_black_heaven_bandages_for_exit` — delayed sapa bargain. 흑천련의 붕대와 탈출로를 얻지만 거래 표식을 남긴다.
  - `follow_archive_triage_map` — delayed cheonggi/return thread. 천기각의 부상자 동선 기록을 따라 천기록 표적 단서를 얻는다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `cheongryu_raid_wounded_fallback_resolved`, `deferred_route_reopened`, `wounded_shelter_stabilized`, `survivor_roll_call_complete`, `righteous_route_started`, `baekdo_medicine_debt`, `sapa_route_started`, `black_heaven_escape_marker`, `cheonggi_return_route_started`, `heavenly_archive_triage_map_seen`, `route_delay_cost_recorded`; `saving_people_changed_witnesses`, `medicine_has_banner`, `black_heaven_help_marks_debt`, `archive_records_count_the_living`, `deferred_choice_is_still_choice`; 로그는 “사람을 구한 선택은 보상을 주지만 세계는 기다려 주지 않는다”는 톤을 유지한다.
- `stabilize_wounded_until_dawn`은 다시 `route_commitment_deferred`를 유지하되 `deferred_route_reopened`와 `route_delay_cost_recorded`를 남긴다. 반복 loop를 피하려면 later 구현에서 forbidden/once-only flag로 한 번만 발생시키고 다음 route offer를 별도 card로 강제한다.
- route choice 계열 3개는 direct raid split과 같은 route starter flags(`righteous_route_started`, `sapa_route_started`, `cheonggi_return_route_started`)를 남길 수 있다. 이렇게 하면 future route opener가 direct branch와 deferred branch를 any-of schema 없이 같은 flag로 받을 수 있다.
- 새 `RouteGraph`, `FactionStanding`, `BranchLock`, `TriageSystem`, `CompanionDeath`, `MassCombat`, boss combat resolver, reward/ability schema, 천외편린 3택 성장 UI, multi-ending implementation은 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

구현 세션 handoff:

- 선행 조건: `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split` runtime slice가 preview source에 구현되어 있고, raid fallback outcome이 `cheongryu_raid_route_split_resolved`, `route_commitment_deferred`, `wounded_saved_flag`, `cheongryu_raid_survived`를 남김을 확인했다.
- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: raid split 뒤에 `wuxia_cheongryu_raid_wounded_fallback`를 추가한다.
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`: 수정하지 않고 기존 `cheongryu_outer_courtyard`를 재사용했다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - 필요 시 docs sync: `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, 이 파일, `docs/dev/Checklist.md`.
- 작성할 테스트:
  - Python exporter/storypack preview test가 preview bundle에 wounded fallback encounter를 포함하고 기본 office bundle에는 무협 id가 없음을 검증한다.
  - Rust `escape-core` content bundle fixture test가 wounded fallback conditions, fallback choice, route starter flags, clues/log/presentation hook을 읽는지 검증한다.
  - WASM JSON boundary test가 `evacuate_the_wounded_first` branch 뒤 wounded fallback scene/action ids까지 도달하는지 검증한다.
  - `escape-terminal --storypack-preview wuxia_jianghu_pack --tui-smoke` scripted route가 wounded fallback title/action ids/stable terms를 렌더링하는지 검증한다.
  - Web generated preview bundle/registry test가 Rust fixture와 encounter id/count/action id parity를 유지하고 default office save key를 쓰지 않음을 검증한다.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - Rust tmp toolchain policy를 적용한 뒤 `cargo test -p escape-core --test content_bundle`, `cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_raid_wounded_fallback_through_preview_bundle`, `cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_raid_wounded_fallback` 같은 targeted tests를 추가/실행한다.
  - `git diff --check`
- 생성 artifact 위치: preview source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, generated preview bundle은 `crates/escape-core/fixtures/content/storypack-preview/`와 `web/src/data/generated/storypack-preview/`에만 둔다.
- Web/Rust/terminal parity 확인 항목: preview runtime metadata 유지, default location 유지, 기존 arrival/first fight/first fragment/rescue/apprentice/raid route smoke 유지, wounded fallback encounter id/action id/fallback route starter flag 일치, default office bundle에 무협 id 부재, `ScenePage`/action id renderer-neutral 표시 유지.
- 구현하지 말아야 할 것: rescue/apprentice/raid를 건너뛰는 wounded fallback 구현, 정파/사파/천기 route opener를 같은 slice에 구현, 기본 office bundle 변경, `escape-office` key rename, yageunmong runtime, route graph/faction/reputation/companion death/triage/mass combat/boss combat schema, 천외편린/각성편린 3택 reward schema, 여러 엔딩 구현.

## 0.19 2026-06-01 docs-only sync: Notion 이구학지 운영 기준 반영

현재 상태: docs-only sync 완료. 이후 follow-up runtime slice에서 `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`까지 storypack preview bundle에 구현했다. 기본 office bundle과 `escape-office` save/localStorage key는 수정하지 않았다.

Live-check한 Notion source:

- `무협 스토리팩: 이구학지 — 천기록` parent page
- `00. 스토리팩 관리 개요`
- `01. 청류문`
- `02. 주요 등장인물` 및 linked child `서하린 상세 설계`
- `03. 세력과 외부 압박`
- `04. 메인 루트 구조`
- `05. 사건 카드 운영 규칙`
- `06. 사이드 퀘스트와 미해결 부채`
- `07. 천기록 / 천외편린 보상`
- `08. 엔딩과 후일담 연결`
- `09. 이구학지 사건 카드 DB` — 26 rows
- `10. 이구학지 후일담 카드 DB` — 17 rows
- `99. 통합 체크포인트`

반영한 precedence:

- Notion parent page is synopsis/early plan. Detailed current Notion rules come from child management docs and the two DBs.
- Repo canonical docs still gate implementation. Notion DB rows become design sources only after being mapped into repo docs.
- Event/epilogue DB row counts are not runtime completion counts.
- 천기록의 정체는 끝까지 밝히지 않는다. “정체 접근”은 기록자의 존재감/시선/실시간 기록을 감지하는 정도로 제한한다.

이번 docs-only sync 결과:

- `docs/dev/Notion_Design_Coverage.md`를 추가해 checked source 목록, 26개 사건 카드 row ↔ repo encounter 후보 mapping, 17개 후일담 카드 future-source status를 기록했다.
- `idea_box/notion_sources.yml`를 추가해 page/data_source id, last edited time, precedence role, handoff mapping을 추적한다.
- `docs/content/storypacks/wuxia_jianghu_pack.md`에 child docs/DB precedence, 청류문 최신 설정, 천기록 최신 정책을 반영했다.
- `docs/content/encounter_db/wuxia_jianghu_pack.md`에 Notion 사건 DB mapping을 추가하고, Notion `wuxia_seoharin_intervention` / `서하린의 개입`이 repo `wuxia_seo_harin_rescue`에 대응함을 명시했다.
- `docs/design/Storypack_World_Model.md`와 `docs/design/Storypack_Encounter_DB.md`에 Notion DB와 repo machine-readable DB의 경계를 반영했다.

이미 runtime에 반영된 것:

- `wuxia_commute_rift_arrival`
- `wuxia_heuksa_bang_first_fight`
- `wuxia_cheonggi_record_first_fragment`
- `wuxia_seo_harin_rescue`
- `wuxia_cheongryu_apprentice_entry`
- `wuxia_cheongryu_raid_route_split`
- `wuxia_cheongryu_raid_wounded_fallback`

아직 runtime 미구현인 것:

- route opener 이후 모든 무협 후보 card.
- Notion 사건 DB 26개 중 위 preview beat로 매핑되지 않은 나머지 rows.
- Notion 후일담 카드 DB 17개 전체.
- full 천외편린 reward/ability schema, faction route graph, epilogue renderer/schema, relation/debt/faction/companion ledgers.

다음 handoff:

- 다음은 즉시 runtime card 구현이 아니라 route opener 선택/설계 sync다. `wuxia_cheongryu_raid_route_split`와 `wuxia_cheongryu_raid_wounded_fallback`이 남긴 direct/deferred flags를 모두 받는 start condition을 문서화해야 한다.
- 최신 Notion 기준으로 `wuxia_seo_harin_rescue`는 Notion 사건 DB `wuxia_seoharin_intervention` / `서하린의 개입`에 대응하고, `wuxia_cheongryu_apprentice_entry`는 `wuxia_qingliu_apprentice_entry` / `청류문 임시 수습생 등록`에 대응하며, `wuxia_cheongryu_raid_route_split`와 `wuxia_cheongryu_raid_wounded_fallback`도 preview runtime 구현이 완료되었다.
- 구현 scope를 full faction reputation/route graph/triage system/companion death/후일담까지 확장하려면 별도 docs-only slice가 먼저 필요하다.


## 0.20 2026-06-01 docs-only route opener handoff: `wuxia_baekdo_medicine_debt`

현재 상태: docs-only route opener 선택/설계 sync 완료, runtime YAML/Rust/Web/generated artifact 미수정. `wuxia_cheongryu_raid_route_split`와 `wuxia_cheongryu_raid_wounded_fallback`가 모두 같은 `storypack_preview` bundle에 landing되어 direct/deferred route starter flags를 남긴다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | route opener 설계 gap | `wuxia_baekdo_medicine_debt` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | wounded fallback까지 preview runtime 완료. 다음은 route opener 축 선택이었다. | 첫 opener를 정파/사파/천기·귀환 중 하나로 고정하고 start conditions를 써야 한다. | 정파 opener가 첫 승격 후보로 결정됐다. | 수정 | 이 파일, `docs/dev/Checklist.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | `storypack_preview` metadata와 launcher/UI wiring은 완료. | route opener도 같은 preview bundle 안에서만 구현해야 한다. | `righteous_route_started` + `cheongryu_rebuild_thread` 조건으로 열 수 있다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | spine은 `route_commitment` 이후 각 route opener로 간다. | 첫 opener의 narrative role과 non-goals가 필요하다. | 백도맹 약상자/청류문 재건 채무가 정파 루트의 첫 대가를 보여준다. | 수정 | storypack/world docs |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | repo 후보 card는 `wuxia_cheongryu_chore_sparring`과 wounded fallback까지 8개였다. | route opener 후보 card를 design-time DB에 추가해야 한다. | `wuxia_baekdo_medicine_debt` candidate를 추가한다. | 수정 | encounter DB markdown, storypack DB JSON/README |
| preview runtime source/artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, Rust/Web preview bundle | `wuxia_cheongryu_chore_sparring` 포함 wounded fallback까지 8개 preview runtime 구현 완료. | 이번 goal은 설계 handoff이므로 runtime source/artifact를 건드리지 않는다. | 다음 구현 세션의 대상이다. | 수정 안 함 | 없음 |
| default office boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, Web save/localStorage key | 기본 office bundle에는 무협 id가 없고 `escape-office` key가 유지된다. | route opener에서도 경계를 유지해야 한다. | 정파 route opener를 default office로 섞지 않는다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_docs_contract.py`, `tests/test_storypack_db.py` | docs/storypack DB contract가 current handoff를 검증한다. | 새 candidate id/count/next_goal assertion이 필요하다. | docs-only handoff가 다음 구현 id를 안정적으로 가리키게 한다. | 수정 | docs/storypack DB tests |

결정:

- 첫 route opener runtime 후보는 `wuxia_baekdo_medicine_debt`로 정한다.
- 이유: `righteous_route_started`와 `cheongryu_rebuild_thread`가 direct raid branch(`defend_cheongryu_with_white_path`)와 deferred wounded branch(`ask_baekdo_for_medicine_not_command`) 양쪽에 모두 남는다. 따라서 새 any-of condition schema 없이 같은 start condition으로 받을 수 있다.
- 정파 opener는 청류문의 최신 운영 기준(따뜻한 언더독 홈베이스, 문제는 내부 악인이 아니라 결핍)과 가장 잘 맞는다. 백도맹의 약상자는 청류문 결핍을 해결하지만 정치적 빚을 남기므로 route pressure를 명확히 보여준다.
- 사파 opener는 거래/암투/부채 표현이 더 강해 relation/debt/faction schema 유혹이 크다. 천기·귀환 opener는 천기록 정체 reveal과 reward/return schema 유혹이 크다. 둘 다 후속 route opener로 둔다.
- `stabilize_wounded_until_dawn`으로 route 선택을 다시 미룬 경우는 `route_commitment_deferred` + `deferred_route_reopened`를 유지하므로, 다음 구현에서 `wuxia_baekdo_medicine_debt`가 자동으로 열리지는 않는다. 이 branch는 later deferred-offer card가 필요하다.

`wuxia_baekdo_medicine_debt` 설계 경계:

- 목적: 정파 route starter를 처음 실제 runtime opener로 받아, 백도맹의 약상자/호위/청류문 재건 지원이 사람을 살리는 동시에 정치적 채무와 명문 중심 질서를 남긴다는 점을 보여준다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [righteous_route_started, cheongryu_rebuild_thread]`, `forbidden_flags: [baekdo_medicine_debt_resolved]`를 권장한다.
- `baekdo_alliance_debt`와 `baekdo_medicine_debt`는 direct/deferred branch flavor hook이다. 둘 중 하나를 required로 만들면 any-of schema가 필요해지므로 eligibility 필수 조건으로 쓰지 않는다.
- stable choice id 후보:
  - `accept_medicine_with_written_debt` — fallback/safe acceptance. 약상자와 호위를 받되 채무 문서를 남긴다.
  - `ask_terms_before_opening_gate` — negotiation probe. 산문을 열기 전에 백도맹 조건을 묻고 정파 질서의 가격을 확인한다.
  - `send_supplies_to_wounded_first` — homebase alignment. 백도맹 지원을 장문/명분보다 부상자와 청류문 사람에게 먼저 돌린다.
  - `compare_banner_to_record_margin` — cheonggi observation. 천기록 여백과 백도맹 깃발의 문장을 비교해 “정의”와 “기록된 채무”가 다를 수 있음을 남긴다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `baekdo_medicine_debt_resolved`, `righteous_route_opened`, `white_path_debt_recorded`, `cheongryu_rebuild_supplies_secured`, `baekdo_terms_questioned`, `namgung_seoyun_notice`, `cheongryu_people_first`, `seo_harin_respect_thread`, `cheonggi_record_notes_baekdo_debt`; `medicine_has_banner`, `white_path_help_has_price`, `order_can_save_and_bind`, `qingliu_survival_needs_outside_help`, `record_counts_debt_not_justice`.
- 새 `RouteGraph`, `FactionStanding`, `DebtLedger`, `RelationScore`, `BranchLock`, `reward_schema`, `ability_schema`, 천외편린 3택 UI, epilogue schema는 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않는다.

다음 구현 세션 handoff:

- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_cheongryu_raid_wounded_fallback` 뒤에 `wuxia_baekdo_medicine_debt`를 추가한다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - tests: `tests/test_web_data_export.py`, `crates/escape-core/tests/content_bundle.rs`, `crates/escape-wasm/tests/json_contract.rs`, `crates/escape-terminal/tests/cli_smoke.rs`, `web/src/core/contentBundles.test.ts`.
  - 필요 시 docs sync: 이 파일, `docs/dev/Checklist.md`, `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, `idea_box/next_goal/README.md`.
- 검증 명령:
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - Rust tmp toolchain policy를 적용한 뒤 `cargo +stable fmt --check`, `cargo +stable test -p escape-core --test content_bundle`, `cargo +stable test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_baekdo_medicine_debt_through_preview_bundle`, `cargo +stable test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_baekdo_medicine_debt`를 추가/실행한다.
  - Web은 `/tmp` scratch copy에서 `npm ci && npm test -- --run src/core/contentBundles.test.ts`로 검증한다.
  - `git diff --check`와 default office diff guard를 실행한다.
- 구현하지 말아야 할 것: 사파/천기 route opener 동시 구현, deferred-offer card 동시 구현, 기본 office bundle 변경, `escape-office` key rename, yageunmong runtime, route graph/faction reputation/debt ledger/relation schema, 천외편린/각성편린 3택 reward schema, epilogue/ending implementation.

## 0.21 2026-06-01 무협 `wuxia_baekdo_medicine_debt` preview runtime slice

현재 상태: 구현 완료. `wuxia_baekdo_medicine_debt`는 첫 정파 route opener로 같은 `wuxia_jianghu_pack` storypack preview source와 Rust/Web preview bundle에 추가했다. Web/default storypack은 0.0b 기준 이구학지로 전환되어 있으므로, 이 slice는 player 기본 흐름에서도 이구학지 본편 진행의 일부가 된다. 과거 office bundle은 legacy/parity fixture로 유지한다.

구현 결과:

- encounter id: `wuxia_baekdo_medicine_debt`
- 조건: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [righteous_route_started, cheongryu_rebuild_thread]`, `forbidden_flags: [baekdo_medicine_debt_resolved]`
- direct branch의 `baekdo_alliance_debt`와 deferred wounded branch의 `baekdo_medicine_debt`는 eligibility 필수가 아니라 flavor hook으로만 남겼다.
- stable choice id: `accept_medicine_with_written_debt`, `ask_terms_before_opening_gate`, `send_supplies_to_wounded_first`, `compare_banner_to_record_margin`
- common hook: `baekdo_medicine_debt_resolved`, `righteous_route_opened`, `destination_id: cheongryu_outer_courtyard`
- branch hook: `white_path_debt_recorded`, `cheongryu_rebuild_supplies_secured`, `baekdo_terms_questioned`, `namgung_seoyun_notice`, `cheongryu_people_first`, `seo_harin_respect_thread`, `cheonggi_record_notes_baekdo_debt`
- presentation: `visual_id: wuxia_baekdo_medicine_debt`, `speaker: 남궁서윤`, `layout: righteous_route_opener`, stable terms `약상자 / 백도맹 / 채무`

변경 범위:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- tests: Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI smoke, Web content bundle registry
- docs/DB: checklist, preview mode contract, storypack doc, encounter DB, machine-readable storypack DB, next_goal prompt

금지선 유지:

- 기본 office `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, legacy `escape-office.*` save/localStorage key는 변경하지 않았다.
- route graph, faction reputation, debt ledger, relation score, reward/ability schema, 천외편린 3택 UI, epilogue/ending schema는 열지 않았다.
- 사파 opener, 천기·귀환 opener, deferred-offer card는 동시에 구현하지 않았다.

다음 handoff:

- `route_opener_followup_after_baekdo` docs-only handoff 결과, 다음 runtime 후보는 사파/흑천련 거래 opener `wuxia_black_heaven_escape_price`로 결정했다.
- 천기·귀환 조사 opener와 `stabilize_wounded_until_dawn` branch를 받는 deferred-offer card는 후속으로 둔다.
- 다음 slice도 Notion reference와 repo storypack/encounter DB를 대조한 뒤 하나만 runtime으로 승격한다.

## 0.22 2026-06-01 docs-only route opener follow-up handoff: `wuxia_black_heaven_escape_price`

현재 상태: docs-only route opener follow-up 선택/설계 sync 완료, runtime YAML/Rust/Web/generated artifact 미수정. `wuxia_baekdo_medicine_debt`까지 runtime에 landing되었고, 다음 route opener 후보 3개를 Notion reference와 repo hooks 기준으로 비교했다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | follow-up gap | `wuxia_black_heaven_escape_price` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | 정파 opener runtime 완료. 다음은 사파/천기/deferred-offer 중 하나를 고르는 handoff였다. | 한 runtime 후보를 고정하고 start conditions를 써야 한다. | 사파 opener가 다음 승격 후보로 결정됐다. | 수정 | 이 파일, `docs/dev/Checklist.md` |
| Notion reference | Notion parent, `03`, `04`, `06`, `07`, `99` | 사파 루트는 흑천련 거래/생존/암투 축이고, 최신 child docs는 외부 압박/결핍, 천기록 정체 미공개를 요구한다. | 사파 opener가 Notion 기준을 어기지 않는지 대조해야 한다. | 거래와 생존을 다루되 relation/debt ledger를 열지 않는다. | 대조 기록 반영 | `docs/dev/Notion_Design_Coverage.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | 이구학지 runtime bundle은 기존 schema만으로 route opener를 연다. | 다음 opener도 same bundle과 renderer-neutral boundary를 유지해야 한다. | `sapa_route_started` + `dowol_debt` 조건으로 열 수 있다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | 사파 루트는 생존, 거래, 암투, 잠입 중심이다. | 첫 사파 opener의 narrative role과 non-goals가 필요하다. | 흑천련의 빠른 도움과 그 값이 사파 route pressure를 보여준다. | 수정 | storypack/world docs |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | 후보 card는 `wuxia_baekdo_medicine_debt`까지 9개였다. | 사파 opener 후보 card를 design-time DB에 추가해야 한다. | 10번째 후보 card로 추가한다. | 수정 | encounter DB markdown, storypack DB JSON/README |
| runtime artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, Rust/Web preview bundle | `wuxia_baekdo_medicine_debt`까지 구현 완료. | 이번 goal은 handoff이므로 runtime source/artifact를 건드리지 않는다. | 다음 구현 세션의 대상이다. | 수정 안 함 | 없음 |
| default/legacy boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, legacy save key | office legacy bundle과 `escape-office.*` key가 유지된다. | follow-up도 경계를 유지해야 한다. | 사파 opener를 office content로 섞지 않는다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_docs_contract.py`, `tests/test_storypack_db.py` | docs/storypack DB contract가 current handoff를 검증한다. | 새 candidate id/count/next_goal assertion이 필요하다. | docs-only handoff가 다음 구현 id를 안정적으로 가리키게 한다. | 수정 | docs/storypack DB tests |

결정:

- 다음 runtime 후보는 `wuxia_black_heaven_escape_price`로 정한다.
- 이유: direct raid branch(`trade_with_black_heaven`)와 deferred wounded branch(`trade_black_heaven_bandages_for_exit`)가 모두 `sapa_route_started`와 `dowol_debt`를 남긴다. 따라서 새 any-of condition schema 없이 같은 start condition으로 받을 수 있다.
- Notion parent synopsis는 사파 루트를 `흑천련 거래`, `암투와 배신`, `밑바닥의 새 질서`로 잡는다. 03/99의 최신 운영 기준은 갈등을 내부 청류문 정치가 아니라 외부 압박/결핍/세력 거래에서 만들라고 한다. 사파 opener는 이 기준에 맞는다.
- 천기·귀환 opener는 `천기록` 정체 reveal, 귀환 시스템, reward/ability schema 유혹이 크다. `07`의 천기록 정책상 아직 정체를 설명하지 않아야 하므로 후속으로 둔다.
- deferred-offer card는 `stabilize_wounded_until_dawn` branch를 받기 위해 필요하지만, 먼저 사파 opener를 구현하면 세 route starter 중 정파/사파가 모두 schema-less opener pattern을 갖게 된다. deferred-offer는 그 뒤 공통 recovery branch로 설계한다.

`wuxia_black_heaven_escape_price` 설계 경계:

- 목적: 사파 route starter를 첫 실제 사파 opener로 받아, 흑천련/도월의 도움은 빠르고 실용적이지만 값과 이름을 남긴다는 점을 보여준다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [sapa_route_started, dowol_debt]`, `forbidden_flags: [black_heaven_escape_price_resolved]`를 권장한다.
- `black_heaven_deal_marked`와 `black_heaven_escape_marker`는 direct/deferred branch flavor hook이다. 둘 중 하나를 required로 만들면 any-of schema가 필요해지므로 eligibility 필수 조건으로 쓰지 않는다.
- stable choice id 후보:
  - `accept_dowol_marker_for_safehouse` — fallback/safe acceptance. 도월의 표식을 받고 흑천련 임시 은신처와 탈출로를 얻는다.
  - `ask_who_collects_the_price` — negotiation probe. 누가, 언제, 어떤 방식으로 값을 받는지 묻는다.
  - `keep_cheongryu_names_off_ledger` — homebase alignment. 청류문 사람들의 이름을 흑천련 장부에서 빼는 조건을 건다.
  - `map_exit_before_following_dowol` — survival observation. 무작정 따라가지 않고 탈출로/추적선을 먼저 기록한다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `black_heaven_escape_price_resolved`, `sapa_route_opened`, `black_heaven_safehouse_marked`, `dowol_terms_questioned`, `cheongryu_names_kept_off_ledger`, `market_route_debt_recorded`, `sapa_survival_principle_seen`; `black_heaven_help_marks_debt`, `black_heaven_bargain_has_teeth`, `survival_bargain_is_not_loyalty`, `sapa_can_save_without_mercy`, `ledger_can_be_bent_not_broken`.
- presentation 권장: `visual_id: wuxia_black_heaven_escape_price`, `speaker: 도월`, `layout: sapa_route_opener`, effect cue stable terms `[탈출로, 흑천련, 값]`.
- 새 `RouteGraph`, `FactionStanding`, `DebtLedger`, `RelationScore`, `BranchLock`, `reward_schema`, `ability_schema`, 천외편린 3택 UI, epilogue schema는 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않는다.

다음 구현 세션 handoff:

- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_baekdo_medicine_debt` 뒤에 `wuxia_black_heaven_escape_price`를 추가한다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - tests: `tests/test_web_data_export.py`, `crates/escape-core/tests/content_bundle.rs`, `crates/escape-wasm/tests/json_contract.rs`, `crates/escape-terminal/tests/cli_smoke.rs`, `web/src/core/contentBundles.test.ts`.
  - 필요 시 docs sync: 이 파일, `docs/dev/Checklist.md`, `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, `idea_box/next_goal/README.md`.
- 검증 명령:
  - `PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - `cargo fmt --check`
  - `cargo test -p escape-core --test content_bundle`
  - `cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_black_heaven_escape_price_through_preview_bundle`
  - `cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_black_heaven_escape_price`
  - `cd web && npm test -- --run src/core/contentBundles.test.ts`
  - `git diff --check`와 default/legacy office diff guard를 실행한다.
- 구현하지 말아야 할 것: 천기·귀환 opener 동시 구현, deferred-offer card 동시 구현, 기본 office bundle 변경, legacy `escape-office` key rename, yageunmong runtime, route graph/faction reputation/debt ledger/relation schema, 천외편린/각성편린 3택 reward schema, epilogue/ending implementation.

## 0.23 2026-06-02 무협 `wuxia_black_heaven_escape_price` preview runtime slice

현재 상태: 구현 완료. `wuxia_black_heaven_escape_price`는 첫 사파 route opener로 같은 `wuxia_jianghu_pack` storypack preview source와 Rust/Web preview bundle에 추가했다. Web/default storypack은 이구학지이므로 이 slice도 player 기본 흐름의 일부다. 과거 office bundle은 legacy/parity fixture로 유지한다.

구현 경계:

- encounter id: `wuxia_black_heaven_escape_price`
- start conditions: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [sapa_route_started, dowol_debt]`, `forbidden_flags: [black_heaven_escape_price_resolved]`
- direct `black_heaven_deal_marked`와 deferred `black_heaven_escape_marker`는 required가 아니라 flavor hook이다. any-of condition schema는 열지 않았다.
- stable choice id: `accept_dowol_marker_for_safehouse`, `ask_who_collects_the_price`, `keep_cheongryu_names_off_ledger`, `map_exit_before_following_dowol`
- common outcome hook: 모든 선택지는 `black_heaven_escape_price_resolved`, `sapa_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- presentation: `visual_id: wuxia_black_heaven_escape_price`, `speaker: 도월`, `layout: sapa_route_opener`, stable terms `탈출로 / 흑천련 / 값`
- 새 route graph, faction reputation, debt ledger, relation score, reward/ability/epilogue schema는 열지 않았다.

검증/계약:

- `tests/test_web_data_export.py`: preview bundle encounter count와 새 encounter order/conditions/presentation/choice ids/outcome hooks를 검증한다.
- `crates/escape-core/tests/content_bundle.rs`: Rust content index가 새 encounter 조건과 fallback outcome을 로드하는지 검증한다.
- `crates/escape-wasm/tests/json_contract.rs`: direct sapa route branch를 지나 `wuxia_black_heaven_escape_price` page와 action result에 도달하는 JSON boundary test를 추가했다.
- `crates/escape-terminal/tests/cli_smoke.rs`: SuperLightTUI smoke가 같은 route opener의 visual/action surface를 표시하는지 검증한다.
- `web/src/core/contentBundles.test.ts`: Web default 이구학지 bundle encounter list에 새 opener가 포함되는지 검증한다.
- generated artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- 기본 `src/tui_adv/data/*.yaml`, 기본 `crates/escape-core/fixtures/content/content.bundle.json`, Web 기본 `web/src/data/generated/content.bundle.json`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

후속 handoff 기록:

- `route_opener_followup_after_black_heaven`를 docs-only로 먼저 열었다.
- 후보는 천기·귀환 opener 또는 `stabilize_wounded_until_dawn` branch를 받는 deferred-offer card였고, 0.24에서 천기·귀환 opener `wuxia_heavenly_archive_previous_outsiders`를 다음 runtime 후보로 골랐다.
- 다음 slice도 Notion reference와 repo storypack/encounter DB를 대조한 상태에서 하나만 runtime으로 승격한다.

## 0.24 2026-06-02 docs-only route opener follow-up handoff: `wuxia_heavenly_archive_previous_outsiders`

현재 상태: docs-only route opener follow-up 선택/설계 sync 완료, runtime YAML/Rust/Web/generated artifact 미수정. 정파 opener `wuxia_baekdo_medicine_debt`와 사파 opener `wuxia_black_heaven_escape_price`가 landing되었으므로, 세 번째 route opener 후보로 천기·귀환 축을 고정했다.

Read-only inventory:

| 영역 | 관련 파일 | 현재 canonical 상태 | follow-up gap | `wuxia_heavenly_archive_previous_outsiders` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| main plan/dev handoff | `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md` | 정파/사파 opener runtime 완료. 다음은 천기·귀환 opener 또는 deferred-offer card 중 하나를 고르는 handoff였다. | 한 runtime 후보를 고정하고 start conditions를 써야 한다. | 천기·귀환 opener가 다음 승격 후보로 결정됐다. | 수정 | 이 파일, `docs/dev/Checklist.md` |
| Notion reference | Notion parent, `04`, `05`, `06`, `07`, `99`, 사건 카드 DB `wuxia_prologue_commute_rift` | 귀환/천기록 route는 천기각 조사, 이전 이방인 기록, 세계 균열, 귀환/정착 질문을 다루되 천기록 정체는 끝까지 밝히지 않는다. | 천기 opener가 귀환 시스템이나 보상 schema를 조기 개방하지 않는지 대조해야 한다. | 서고 기록과 여백 단서만 열고 정답/귀환법은 주지 않는다. | 대조 기록 반영 | `docs/dev/Notion_Design_Coverage.md` |
| preview mode contract | `docs/dev/Storypack_Runtime_Preview_Mode.md` | 이구학지 runtime bundle은 기존 schema만으로 route opener를 연다. | 다음 opener도 same bundle과 renderer-neutral boundary를 유지해야 한다. | `cheonggi_return_route_started` + `cheonggi_record_targeted` 조건으로 열 수 있다. | 수정 | `docs/dev/Storypack_Runtime_Preview_Mode.md` |
| storypack canonical story | `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/design/Storypack_World_Model.md` | 천기각은 천기록의 비밀, 이전 이방인 기록, 귀환 루트와 연결된다. | 첫 천기·귀환 opener의 narrative role과 non-goals가 필요하다. | 이전 이방인 기록을 읽되 귀환 방법/기록자 정체는 future로 남긴다. | 수정 | storypack/world docs |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | 후보 card는 `wuxia_black_heaven_escape_price`까지 10개였다. | 천기 opener 후보 card를 design-time DB에 추가해야 한다. | 11번째 후보 card로 추가한다. | 수정 | encounter DB markdown, storypack DB JSON/README |
| runtime artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, Rust/Web preview bundle | `wuxia_black_heaven_escape_price`까지 구현 완료. | 이번 goal은 handoff이므로 runtime source/artifact를 건드리지 않는다. | 다음 구현 세션의 대상이다. | 수정 안 함 | 없음 |
| default/legacy boundary | `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, legacy save key | office legacy bundle과 `escape-office.*` key가 유지된다. | follow-up도 경계를 유지해야 한다. | 천기 opener를 office content로 섞지 않는다. | 수정 안 함 | 없음 |
| tests/contracts | `tests/test_docs_contract.py`, `tests/test_storypack_db.py` | docs/storypack DB contract가 current handoff를 검증한다. | 새 candidate id/count/next_goal assertion이 필요하다. | docs-only handoff가 다음 구현 id를 안정적으로 가리키게 한다. | 수정 | docs/storypack DB tests |

결정:

- 다음 runtime 후보는 `wuxia_heavenly_archive_previous_outsiders`로 정한다.
- 이유: direct raid branch(`follow_heavenly_archive`)와 deferred wounded branch(`follow_archive_triage_map`)가 모두 `cheonggi_return_route_started`와 `cheonggi_record_targeted`를 남긴다. 따라서 새 any-of condition schema 없이 같은 start condition으로 받을 수 있다.
- Notion parent와 `04. 메인 루트 구조`는 귀환/천기록 route를 천기각 조사, 이전 이방인 기록, 세계 균열, 귀환/정착 질문으로 잡는다. 사건 카드 DB의 `wuxia_prologue_commute_rift`는 현대 출근길 균열이 후속 귀환 계열 카드와 연결 가능하다고 남긴다.
- `07. 천기록 / 천외편린 보상`과 `99. 통합 체크포인트`는 천기록을 검색창으로 만들지 말고, 기록자의 정체를 끝까지 밝히지 말라고 한다. 따라서 이번 opener는 서고 여백, 이전 이방인 기록, 균열 용어만 남기고 정답/귀환법/기록자 이름을 주지 않는다.
- deferred-offer card는 `stabilize_wounded_until_dawn` branch를 받기 위해 필요하지만, 정파/사파 다음에 천기·귀환 opener를 먼저 구현하면 세 route starter 모두 schema-less opener pattern을 갖는다. deferred-offer는 그 뒤 공통 recovery branch로 설계한다.

`wuxia_heavenly_archive_previous_outsiders` 설계 경계:

- 목적: 천기·귀환 route starter를 받아, 천기각 서고에서 이전 이방인 기록과 세계 균열 흔적을 처음 확인한다. 이 장면은 귀환법을 주는 장면이 아니라 “이전에 온 사람이 있었고, 기록은 대답 대신 여백을 남긴다”는 route pressure를 여는 장면이다.
- 시작 조건: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]`, `forbidden_flags: [heavenly_archive_previous_outsiders_resolved]`를 권장한다.
- `heavenly_archive_contact`와 `heavenly_archive_triage_map_seen`는 direct/deferred branch flavor hook이다. 둘 중 하나를 required로 만들면 any-of schema가 필요해지므로 eligibility 필수 조건으로 쓰지 않는다.
- stable choice id 후보:
  - `read_previous_outsider_margins` — fallback/safe reading. 이전 이방인의 여백 기록을 조용히 읽는다.
  - `ask_yeon_soha_what_not_to_read` — boundary probe. 연소하에게 무엇을 읽으면 안 되는지 먼저 묻는다.
  - `mark_current_worldline_without_answer` — no-answer acceptance. 답을 요구하지 않고 현재 세계선의 흔적만 표시한다.
  - `compare_rift_terms_to_commute_memory` — return clue comparison. 서고의 균열 용어를 출근길 기억과 비교한다.
- outcome hook은 기존 runtime schema의 `resources`, `danger`, `add_flags`, `add_clues`, `destination_id`, `log`, optional `presentation/effect_cues`만 사용한다.
- 주요 flags/clues/log 방향: `heavenly_archive_previous_outsiders_resolved`, `cheonggi_return_route_opened`, `previous_outsiders_record_seen`, `yeon_soha_warning_heard`, `worldline_margin_marked`, `commute_rift_terms_compared`; `archive_has_other_outsiders`, `cheonggi_record_refuses_identity_answer`, `return_clue_is_not_return_method`, `worldline_gaps_have_patterns`, `record_gaze_without_name`.
- presentation 권장: `visual_id: wuxia_heavenly_archive_previous_outsiders`, `speaker: 연소하`, `layout: cheonggi_return_opener`, effect cue stable terms `[천기각, 이방인, 균열]`.
- 새 `RouteGraph`, `FactionStanding`, `DebtLedger`, `RelationScore`, `BranchLock`, `return_system`, `reward_schema`, `ability_schema`, 천외편린 3택 UI, epilogue schema는 열지 않는다.
- 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않는다.

다음 구현 세션 handoff:

- 예상 수정 파일:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_black_heaven_escape_price` 뒤에 `wuxia_heavenly_archive_previous_outsiders`를 추가한다.
  - generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
  - tests: `tests/test_web_data_export.py`, `crates/escape-core/tests/content_bundle.rs`, `crates/escape-wasm/tests/json_contract.rs`, `crates/escape-terminal/tests/cli_smoke.rs`, `web/src/core/contentBundles.test.ts`.
  - 필요 시 docs sync: 이 파일, `docs/dev/Checklist.md`, `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, `idea_box/next_goal/README.md`.
- 검증 명령:
  - `PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - `cargo fmt --check`
  - `cargo test -p escape-core --test content_bundle`
  - `cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_heavenly_archive_previous_outsiders_through_preview_bundle`
  - `cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_heavenly_archive_previous_outsiders`
  - `cd web && npm test -- --run src/core/contentBundles.test.ts`
  - `git diff --check`와 default/legacy office diff guard를 실행한다.
- 구현하지 말아야 할 것: deferred-offer card 동시 구현, 귀환/엔딩/후일담 구현, 천기록 기록자 정체 reveal, 기본 office bundle 변경, legacy `escape-office` key rename, yageunmong runtime, route graph/faction reputation/debt ledger/relation schema, 천외편린/각성편린 3택 reward schema, epilogue/ending implementation.

## 0.25 2026-06-02 무협 `wuxia_heavenly_archive_previous_outsiders` preview runtime slice

현재 상태: 구현 완료. `wuxia_heavenly_archive_previous_outsiders`는 첫 천기·귀환 route opener로 같은 `wuxia_jianghu_pack` storypack preview source와 Rust/Web preview bundle에 추가했다. Web/default storypack은 이구학지이므로 이 slice도 player 기본 흐름의 일부다. 과거 office bundle은 legacy/parity fixture로 유지한다.

구현 경계:

- encounter id: `wuxia_heavenly_archive_previous_outsiders`
- start conditions: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]`, `forbidden_flags: [heavenly_archive_previous_outsiders_resolved]`
- direct `heavenly_archive_contact`와 deferred `heavenly_archive_triage_map_seen`는 required가 아니라 flavor hook이다. any-of condition schema는 열지 않았다.
- stable choice id: `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory`
- common outcome hook: 모든 선택지는 `heavenly_archive_previous_outsiders_resolved`, `cheonggi_return_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- presentation: `visual_id: wuxia_heavenly_archive_previous_outsiders`, `speaker: 연소하`, `layout: cheonggi_return_opener`, stable terms `천기각 / 이방인 / 균열`
- 천기록 기록자의 정체 reveal, return system, route graph, faction reputation, debt ledger, relation score, reward/ability/epilogue schema는 열지 않았다.

검증/계약:

- `tests/test_web_data_export.py`: preview bundle encounter count와 새 encounter order/conditions/presentation/choice ids/outcome hooks를 검증한다.
- `crates/escape-core/tests/content_bundle.rs`: Rust content index가 새 encounter 조건과 fallback outcome을 로드하는지 검증한다.
- `crates/escape-wasm/tests/json_contract.rs`: direct 천기 route branch를 지나 `wuxia_heavenly_archive_previous_outsiders` page와 action result에 도달하는 JSON boundary test를 추가했다.
- `crates/escape-terminal/tests/cli_smoke.rs`: SuperLightTUI smoke가 같은 route opener의 visual/action surface를 표시하는지 검증한다.
- `web/src/core/contentBundles.test.ts`: Web default 이구학지 bundle encounter list에 새 opener가 포함되는지 검증한다.
- generated artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- 기본 `src/tui_adv/data/*.yaml`, 기본 `crates/escape-core/fixtures/content/content.bundle.json`, Web 기본 `web/src/data/generated/content.bundle.json`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

다음 handoff:

- `route_opener_followup_after_heavenly_archive`를 docs-only로 먼저 연다.
- 후보는 `stabilize_wounded_until_dawn` branch를 받는 deferred-offer card 또는 세 route opener 이후의 첫 midgame continuity card다.
- 다음 slice도 Notion reference와 repo storypack/encounter DB를 대조한 뒤 하나만 runtime으로 승격한다.

## 0.26 2026-06-02 docs-only route opener follow-up handoff: `wuxia_wounded_shelter_dawn_offers`

현재 상태: docs-only route opener follow-up 선택/설계 sync 완료, runtime YAML/Rust/Web/generated artifact 미수정. 정파/사파/천기·귀환 opener가 모두 구현됐으므로, 남은 route-pressure gap은 `stabilize_wounded_until_dawn` branch가 직접 opener를 타지 않고 `route_commitment_deferred` 상태로 남는 경우다.

대조 결과:

| 영역 | 관련 파일 | 현재 canonical 상태 | follow-up gap | `wuxia_wounded_shelter_dawn_offers` 관련성 | 이번 세션 수정 여부 | 수정한다면 예상 변경 파일 |
|---|---|---|---|---|---|---|
| Notion reference | Notion parent, `04`, `05`, `06`, `07`, `99`, 사건 카드 DB `wuxia_qingliu_attack_after_war` | 루트는 최종 선택 하나로 갑자기 정해지지 않고, 사건 카드 단위로 점차 좁혀진다. 중요한 선택을 미뤄도 메인 엔딩을 막지 않고 미해결 부채/후속 카드로 남긴다. 천기록 정체와 귀환법은 밝히지 않는다. | deferred branch가 안전 선택으로 끝나면 route pressure가 끊긴다. | 부상자를 살린 결과로 제안이 찾아오는 카드이므로 플레이어 비난 없이 미룬 선택을 다시 연다. | 대조 기록 반영 | `docs/dev/Notion_Design_Coverage.md` |
| preview runtime hooks | `wuxia_cheongryu_raid_wounded_fallback` outcome | `stabilize_wounded_until_dawn`은 `cheongryu_raid_wounded_fallback_resolved`, `route_commitment_deferred`, `deferred_route_reopened`, `wounded_shelter_stabilized`, `survivor_roll_call_complete`, `route_delay_cost_recorded`를 남긴다. | 정파/사파/천기 starter flags가 없어 기존 route opener 세 개가 바로 열리지 않는다. | 기존 flags만으로 후속 eligibility를 만들 수 있다. | 문서화만 | 다음 구현 session에서 preview YAML/fixtures/tests |
| encounter situation cards/DB | `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*` | 후보 card는 `wuxia_heavenly_archive_previous_outsiders`까지 11개였다. | deferred-offer 후보 card를 design-time DB에 추가해야 한다. | 12번째 후보 card로 추가한다. | 수정 | encounter DB markdown, storypack DB JSON/README |
| runtime artifacts | `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`, Rust/Web preview bundle | `wuxia_heavenly_archive_previous_outsiders`까지 구현 완료. | 이번 goal은 handoff이므로 runtime source/artifact를 건드리지 않는다. | 다음 구현 세션의 대상이다. | 수정 안 함 | 없음 |

결정:

- 다음 runtime 후보는 `wuxia_wounded_shelter_dawn_offers`로 정한다.
- post-opener midgame continuity는 세 route opener의 opened flags(`righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`)를 any-of로 묶거나 route graph/faction reputation/ending schema를 일찍 요구할 가능성이 높다. 반면 deferred-offer card는 기존 single-branch flags만으로 열린다.
- Notion `04. 메인 루트 구조`의 “플레이 중 선택으로 최종 후보가 좁혀진다”는 원칙과 `06. 사이드 퀘스트와 미해결 부채`의 “방치/미룸은 메인을 막지 않고 풍문·부재·세계 변화·잔상으로 남긴다”는 원칙을 동시에 만족한다.
- 이 카드는 부상자를 돌본 결과로 서하린/남궁서윤/도월/연소하 쪽 제안이 도착하는 장면이다. “네가 늦어서 문제가 생겼다”가 아니라 “사람을 살려 두었기 때문에 다시 선택지가 찾아왔다”로 표현한다.

`wuxia_wounded_shelter_dawn_offers` 설계 경계:

- purpose: `stabilize_wounded_until_dawn` fallback branch를 route opener 전 공통 제안 카드로 회수한다.
- start conditions: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]`, `forbidden_flags: [wounded_shelter_dawn_offers_resolved]`
- flavor-only flags: `survivor_roll_call_complete`, `route_delay_cost_recorded`
- presentation 권장: `visual_id: wuxia_wounded_shelter_dawn_offers`, `speaker: 서하린`, `layout: deferred_route_offer`, effect cue stable terms `[새벽, 부상자, 제안]`.
- stable choice ids:
  - `keep_wounded_shelter_until_noon` — safe/fallback care. `wounded_shelter_until_noon`, `deferred_offer_debt_recorded`를 남긴다.
  - `accept_baekdo_medicine_after_roll_call` — delayed righteous route. `righteous_route_started`, `cheongryu_rebuild_thread`, `baekdo_medicine_debt`를 남겨 다음 턴에 `wuxia_baekdo_medicine_debt`가 열릴 수 있게 한다.
  - `send_word_to_dowol_for_quiet_exit` — delayed sapa route. `sapa_route_started`, `dowol_debt`, `black_heaven_escape_marker`를 남겨 다음 턴에 `wuxia_black_heaven_escape_price`가 열릴 수 있게 한다.
  - `show_archive_map_to_yeon_soha` — delayed cheonggi/return route. `cheonggi_return_route_started`, `cheonggi_record_targeted`, `heavenly_archive_triage_map_seen`를 남겨 다음 턴에 `wuxia_heavenly_archive_previous_outsiders`가 열릴 수 있게 한다.
- common outcome hook: 모든 선택지는 `wounded_shelter_dawn_offers_resolved`, `route_commitment_reopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- clue hooks: `saving_people_changed_witnesses`, `delayed_choice_has_callers`, `care_is_not_route_escape`, `offers_arrive_because_people_lived`, `dawn_shelter_keeps_names`
- non-goals: triage system, companion death, mass combat, route graph, faction reputation, relation score, debt ledger, reward/ability schema, epilogue schema, return system, 천기록 identity reveal, 기본 office bundle 변경.

다음 구현 세션 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_heavenly_archive_previous_outsiders` 뒤에 `wuxia_wounded_shelter_dawn_offers`를 추가한다.
- generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
- tests: `tests/test_web_data_export.py` encounter order/conditions/presentation/choice ids/outcome hooks, `crates/escape-core/tests/content_bundle.rs`, `crates/escape-wasm/tests/json_contract.rs`, `crates/escape-terminal/tests/cli_smoke.rs`, `web/src/core/contentBundles.test.ts`를 추가/갱신한다.
- smoke/check:
  - `PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - `cargo test -p escape-core --test content_bundle`
  - `cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_wounded_shelter_dawn_offers_through_preview_bundle`
  - `cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_wounded_shelter_dawn_offers`
  - `git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json`

## 0.27 2026-06-02 무협 `wuxia_wounded_shelter_dawn_offers` preview runtime slice

현재 상태: 구현 완료. `wuxia_wounded_shelter_dawn_offers`는 `stabilize_wounded_until_dawn` branch가 직접 정파/사파/천기 opener로 가지 않은 경우를 회수하는 deferred-offer card로, 같은 `wuxia_jianghu_pack` storypack preview source와 Rust/Web preview bundle에 추가했다. Web/default storypack은 0.0b 기준 이구학지이므로, 이 slice도 현재 기본 플레이 흐름의 일부다.

구현 결과:

- encounter id: `wuxia_wounded_shelter_dawn_offers`
- 조건: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]`, `forbidden_flags: [wounded_shelter_dawn_offers_resolved]`
- `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 eligibility 필수가 아니라 branch flavor hook으로만 남겼다.
- stable choice id: `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha`
- common outcome hook: 모든 선택지는 `wounded_shelter_dawn_offers_resolved`, `route_commitment_reopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- route reentry hook: 정파/사파/천기 선택지는 각각 기존 opener가 읽는 `righteous_route_started` + `cheongryu_rebuild_thread`, `sapa_route_started` + `dowol_debt`, `cheonggi_return_route_started` + `cheonggi_record_targeted`를 남긴다.
- presentation: `visual_id: wuxia_wounded_shelter_dawn_offers`, `speaker: 서하린`, `layout: deferred_route_offer`, stable terms `새벽 / 부상자 / 제안`

변경 범위:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- tests: Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI smoke, Web content bundle registry
- docs/DB: checklist, preview mode contract, storypack doc, encounter DB, machine-readable storypack DB, next_goal prompt

금지선 유지:

- 기본 office `src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`, legacy `escape-office.*` save/localStorage key는 변경하지 않았다.
- triage system, companion death, mass combat, route graph, faction reputation, debt ledger, relation score, reward/ability schema, epilogue/return system, 천기록 identity reveal은 열지 않았다.
- post-opener midgame continuity card는 동시에 구현하지 않았다.

다음 handoff:

- 다음은 즉시 runtime card 구현이 아니라 `route_midgame_continuity_after_wounded_shelter` docs-only handoff다.
- 세 route opener와 deferred-offer가 모두 구현됐으므로, 다음 후보는 post-opener midgame continuity를 어떤 방식으로 여는지 비교해야 한다.
- 첫 midgame continuity가 route별 3개 card인지, 공통 midgame card인지, 또는 기존 `righteous_route_opened`/`sapa_route_opened`/`cheonggi_return_route_opened`를 아직 any-of 없이 다루는 다른 bridge인지 문서에서 먼저 결정한다.
- 다음 handoff에서도 기본 office bundle, legacy `escape-office` key, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal은 열지 않는다.

## 0.28 2026-06-02 docs-only midgame continuity handoff: `wuxia_mumyeong_first_sighting`

현재 상태: docs-only post-opener midgame continuity 선택/설계 sync 완료, runtime YAML/Rust/Web/generated artifact 미수정. 정파/사파/천기 opener와 deferred-offer card가 모두 구현됐으므로, 다음 gap은 route opener 이후 “첫 중반 연속성”을 route graph 없이 여는 것이다.

Notion live 대조:

- `04. 메인 루트 구조`: 최종 선택 하나로 route가 갑자기 결정되지 않고, 플레이 중 목표 선택/사건/동료 관계/천기록 사용에 따라 후보가 좁혀진다.
- `05. 사건 카드 운영 규칙`: 사건 카드는 사건 ID, 진행 단계, 관련 인물/세력, 선행 조건, 선택지 요약, 결과/보상/미해결 부채/후일담/엔딩 영향을 가진다.
- `06. 사이드 퀘스트와 미해결 부채`: 중요한 미해결 부채는 메인 엔딩을 막지 않고 풍문/부재/세계 변화/잔상으로 남긴다.
- `07. 천기록 / 천외편린 보상`: 무명은 남의 무공을 빠르게 훔쳐 덧씌우는 카피와 주인공의 이해/재구성 성장을 대비시키며, 천기록 정체는 밝히지 않는다.
- `99. 통합 체크포인트`: 무명은 청류문 이탈자이자 흑사방 소속 라이벌이고, 초반에는 주인공보다 살짝 강하며, 보스와 별개로 라이벌 결산을 담당한다.
- 사건 카드 DB `wuxia_mumyeong_first_sighting` / `무명 첫 목격`: 진행 단계는 `중반`, 선행 조건은 “청류문 임시 수습생 등록 이후, 흑사방 압박 사건 발생”, 기능은 무명 존재 암시/서하린의 상처 암시/청류문 무공과 카피 무공 차이 암시다.
- 사건 카드 DB `wuxia_mumyeong_first_confrontation` / `무명 첫 대치`: 선행 조건이 `wuxia_mumyeong_first_sighting` 이후이므로 이번 첫 midgame candidate보다 늦다.
- 사건 카드 DB `wuxia_mumyeong_midgame_reunion` / `무명 중반 재회`: 선행 조건이 첫 대치와 과거 단서 일부이므로 후속 rival arc로 둔다.

후보 비교:

| 후보 | 장점 | 문제 | 결정 |
|---|---|---|---|
| route별 midgame card 3개 | route tone이 선명하다. | 다음 runtime이 3개로 fan-out되고 route graph/faction reputation schema 유혹이 커진다. | 보류 |
| common midgame bridge 1개 | 첫 중반 연속성을 작게 열고 Notion `무명 첫 목격`과 맞는다. | 기존 schema에는 `righteous_route_opened`/`sapa_route_opened`/`cheonggi_return_route_opened` any-of가 없다. | 선택 |
| deferred-offer 후속 bridge | 방금 구현한 `route_commitment_reopened`와 자연스럽다. | direct route opener branch를 제외해 중반 진입이 불공평해진다. | 보류 |
| `wuxia_mumyeong_first_confrontation` | 라이벌 combat를 바로 열 수 있다. | Notion 선행 조건이 `무명 첫 목격` 이후다. | 후속 |
| boss first appearance | 메인 적대 논리를 빠르게 제시한다. | 보스/대형 전투/최종 논리 결산이 커져 현 slice 범위를 넘는다. | 후속 |

결정:

- 다음 runtime 후보는 common midgame bridge `wuxia_mumyeong_first_sighting`로 정한다.
- 새 any-of condition schema를 열지 않는다. 대신 다음 구현 slice에서 세 route opener의 모든 outcome에 기존 `add_flags` schema로 공통 flag `route_opener_resolved`를 추가한다.
- `wuxia_mumyeong_first_sighting`은 `route_opener_resolved`를 required flag로 받아 route별 opener를 모두 같은 start condition으로 회수한다.
- 기존 route-specific opened flags(`righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`)는 eligibility가 아니라 flavor hook으로만 읽는다.
- `keep_wounded_shelter_until_noon`처럼 route opener를 아직 타지 않은 fallback branch는 `route_opener_resolved`가 없으므로 이 midgame card를 바로 열지 않는다.

`wuxia_mumyeong_first_sighting` 설계 경계:

- purpose: route opener 이후 첫 중반 연속성으로, 흑사방 쪽에서 청류문식 흐름을 훔쳐 쓰는 그림자를 목격하게 한다. 무명은 아직 정식 대치하지 않고, 서하린의 침묵과 카피 무공의 이질감만 남긴다.
- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_sighting_resolved]`
- implementation prerequisite: `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`의 모든 choice outcome에 `route_opener_resolved`를 추가한다.
- flavor-only flags: `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`, `white_path_debt_recorded`, `market_route_debt_recorded`, `previous_outsiders_record_seen`
- presentation 권장: `visual_id: wuxia_mumyeong_first_sighting`, `speaker: 서하린`, `layout: midgame_rival_sighting`, effect cue stable terms `[무명, 청류문, 흑사방]`.
- stable choice ids:
  - `watch_the_stolen_qingliu_flow` — safe/fallback observation. `mumyeong_shadow_seen`, `copied_qingliu_flow_noted`를 남긴다.
  - `check_seo_harin_silence` — companion observation. `seo_harin_recognized_mumyeong`, `mumyeong_wound_thread_opened`를 남긴다.
  - `follow_black_serpent_runner` — risky pursuit. `black_serpent_trail_marked`, `mumyeong_pursuit_risk`를 남긴다.
  - `pretend_not_to_see_the_form` — avoid/escalation delay. `mumyeong_clue_deferred`, `unresolved_rival_debt`를 남긴다.
- common outcome hook: 모든 선택지는 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- clue hooks: `mumyeong_exists`, `copied_flow_is_not_qingliu`, `seo_harin_does_not_call_him_traitor`, `black_serpent_uses_borrowed_flows`, `not_seeing_is_also_a_choice`
- non-goals: `wuxia_mumyeong_first_confrontation` 동시 구현, 보스 첫 등장, boss combat resolver, combat schema, route graph, faction reputation, relation/debt ledger, companion system, reward/ability schema, epilogue/return system, 천기록 identity reveal, 기본 office bundle 변경.

다음 구현 세션 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: 세 route opener outcome에 `route_opener_resolved`를 추가하고, `wuxia_wounded_shelter_dawn_offers` 뒤에 `wuxia_mumyeong_first_sighting`를 추가한다.
- generated preview artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`만 재생성한다.
- tests: `tests/test_web_data_export.py`, `crates/escape-core/tests/content_bundle.rs`, `crates/escape-wasm/tests/json_contract.rs`, `crates/escape-terminal/tests/cli_smoke.rs`, `web/src/core/contentBundles.test.ts`를 추가/갱신한다.
- smoke/check:
  - `PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q`
  - `python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check`
  - `cargo test -p escape-core --test content_bundle`
  - `cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_first_sighting_through_preview_bundle`
  - `cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_first_sighting`
  - `git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json`

## 0.29 2026-06-02 무협 `wuxia_mumyeong_first_sighting` preview runtime slice

현재 상태: 구현 완료. `route_midgame_continuity_after_wounded_shelter` handoff에서 고른 common midgame bridge를 separate `wuxia_jianghu_pack` storypack preview runtime에 추가했다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`의 세 route opener(`wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`) 모든 choice outcome에 `route_opener_resolved`를 추가했다.
- `wuxia_mumyeong_first_sighting`를 `wuxia_wounded_shelter_dawn_offers` 뒤에 추가했다.
- start conditions는 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_sighting_resolved]`다.
- stable choice id는 `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form`다.
- 모든 choice outcome은 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary clues는 `mumyeong_exists`, `copied_flow_is_not_qingliu`, `seo_harin_does_not_call_him_traitor`, `black_serpent_uses_borrowed_flows`, `not_seeing_is_also_a_choice`다.
- presentation hook은 `visual_id: wuxia_mumyeong_first_sighting`, `speaker: 서하린`, `layout: midgame_rival_sighting`, stable terms `[무명, 청류문, 흑사방]`다.
- Rust/Web generated preview artifacts만 재생성했다: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`.

검증/계약:

- Python exporter/docs/storypack DB tests, Rust content bundle test, WASM JSON boundary test, terminal smoke test, Web `contentBundles` test를 갱신했다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.
- 새 any-of condition, route graph, faction reputation, relation/debt ledger, combat resolver/schema, boss first appearance, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_midgame_reunion`, reward/ability/epilogue/return system, 천기록 identity reveal은 열지 않았다.

다음 handoff:

- 다음 작업은 `wuxia_mumyeong_first_confrontation_after_sighting` docs-only handoff다.
- Notion 사건 카드 DB `wuxia_mumyeong_first_confrontation` / `무명 첫 대치`는 `wuxia_mumyeong_first_sighting` 이후가 선행 조건이므로 이제 후보가 되었지만, combat schema/route graph/faction reputation을 열지 말지 먼저 문서에서 결정한다.

## 0.30 2026-06-02 docs-only rival confrontation handoff: `wuxia_mumyeong_first_confrontation`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_first_confrontation_after_sighting`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_mumyeong_first_confrontation`로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, 기본 office bundle을 수정하지 않았다.

대조한 Notion source:

- 사건 카드 DB `wuxia_mumyeong_first_confrontation` / `무명 첫 대치`: 선행 조건은 `wuxia_mumyeong_first_sighting` 이후이며, 결과는 무명이 주인공보다 살짝 강한 라이벌로 등장하고 완승보다 버티기/분석이 핵심이라는 것이다.
- 사건 카드 DB `wuxia_mumyeong_midgame_reunion` / `무명 중반 재회`: 선행 조건이 첫 대치 이후와 무명 과거 단서 일부라 아직 이르다.
- 사건 카드 DB `wuxia_boss_first_appearance` / `보스 첫 등장`: 흑사방 보스를 압도적 벽으로 각인시키는 사건이라 boss combat/final-wall pressure를 너무 빨리 연다.
- `04. 메인 루트 구조`: 무명은 라이벌 결산을 담당하고, 보스는 최종 논리 결산을 담당한다.
- `05. 사건 카드 운영 규칙`: 사건 ID/선행 조건/선택지/결과/보상/엔딩 영향을 카드 단위로 관리한다.
- `06. 사이드 퀘스트와 미해결 부채`: 방치/지연은 메인 엔딩을 막지 않고 후일담/잔상으로 남긴다.
- `07. 천기록 / 천외편린 보상`: 주인공의 이해/재구성은 무명의 카피와 대비되지만, 새 reward/ability schema는 아직 열지 않는다.
- `99. 통합 체크포인트`: 무명은 초반 기준 주인공보다 살짝 강하고, 카피 무공은 청류안/천기록의 이해와 대비되어야 한다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_mumyeong_first_confrontation` | Notion 선행 조건이 충족됐고, 첫 목격이 남긴 `mumyeong_first_sighting_resolved`/`midgame_continuity_started`를 바로 회수한다. | 전투 사건이지만 승리 판정/HP 숫자전으로 흐르면 combat schema를 열 위험이 있다. | 다음 runtime 후보 |
| `wuxia_mumyeong_midgame_reunion` | 라이벌/거울 관계를 깊게 만든다. | 첫 대치와 과거 단서 일부가 선행 조건이라 아직 이르다. | 보류 |
| `wuxia_boss_first_appearance` | 흑사방 최종 논리를 빨리 각인할 수 있다. | 보스는 압도적 벽이며 boss combat/final-wall pressure가 커진다. | 보류 |
| route-specific clue bridge | 정파/사파/천기 opener flavor를 더 쌓을 수 있다. | 첫 목격이 이미 common bridge를 열었고, route fan-out을 더 벌릴 수 있다. | 보류 |

결정:

- 다음 runtime 후보는 `wuxia_mumyeong_first_confrontation`로 정한다.
- 새 combat resolver/schema 없이 기존 encounter schema의 `resources`, `danger`, `add_flags`, `add_clues`, `destination_id`, `log`, `presentation`만 사용한다.
- 이 사건은 “무명을 이긴다”가 아니라 “버티고, 읽고, 서하린의 침묵과 카피 무공의 결함을 확인한다”로 표현한다.
- route-specific opener flags(`righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`)와 첫 목격 branch flags는 eligibility가 아니라 branch flavor hook으로만 읽는다.

`wuxia_mumyeong_first_confrontation` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_confrontation_resolved]`
- flavor-only flags: `mumyeong_shadow_seen`, `copied_qingliu_flow_noted`, `seo_harin_recognized_mumyeong`, `mumyeong_wound_thread_opened`, `black_serpent_trail_marked`, `mumyeong_clue_deferred`, `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`
- stable choice ids:
  - `meet_mumyeong_head_on`
  - `endure_until_copy_flow_breaks`
  - `watch_seo_harin_hold_back`
  - `read_mumyeongs_copied_form`
  - `do_not_provoke_mumyeong`
- common outcome hook: 모든 선택지는 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary flags/clues: `rival_endured_not_defeated`, `copied_flow_weakness_noted`, `mumyeong_is_not_boss_wall`, `seo_harin_mumyeong_silence_confirmed`, `copy_style_has_gap`, `winning_is_not_required`.
- presentation 권장: `visual_id: wuxia_mumyeong_first_confrontation`, `speaker: 무명`, `layout: rival_first_confrontation`, effect cue stable terms `[무명, 서하린, 청류문]`.
- non-goals: boss first appearance, `wuxia_mumyeong_midgame_reunion`, 무명 과거 진실 reveal, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, companion death, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, 기본 office bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_mumyeong_first_sighting` 뒤에 `wuxia_mumyeong_first_confrontation`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.31 2026-06-02 무협 `wuxia_mumyeong_first_confrontation` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_first_confrontation_after_sighting` handoff에서 고른 첫 rival confrontation을 separate `wuxia_jianghu_pack` storypack preview runtime에 추가했다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_first_sighting` 뒤에 `wuxia_mumyeong_first_confrontation`를 추가했다.
- start conditions는 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_confrontation_resolved]`다.
- stable choice id는 `meet_mumyeong_head_on`, `endure_until_copy_flow_breaks`, `watch_seo_harin_hold_back`, `read_mumyeongs_copied_form`, `do_not_provoke_mumyeong`다.
- 모든 choice outcome은 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary flags/clues는 `rival_endured_not_defeated`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted`, `rivalry_deferred_not_avoided`, `copy_style_has_gap`, `winning_is_not_required`, `understanding_is_not_copying`이다.
- presentation hook은 `visual_id: wuxia_mumyeong_first_confrontation`, `speaker: 무명`, `layout: rival_first_confrontation`, stable terms `[무명, 서하린, 청류문]`다.
- Rust/Web generated preview artifacts만 재생성했다: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`.

검증/계약:

- Python exporter/docs/storypack DB tests, Rust content bundle test, WASM JSON boundary test, terminal smoke test, Web `contentBundles` test를 갱신했다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.
- 새 combat resolver/schema, HP 숫자전, boss first appearance, `wuxia_mumyeong_midgame_reunion`, route graph, faction reputation, relation/debt ledger, reward/ability/epilogue/return system, 천기록 identity reveal은 열지 않았다.

다음 handoff:

- 다음 작업은 `wuxia_mumyeong_followup_after_first_confrontation` docs-only handoff다.
- Notion 사건 카드 DB `wuxia_mumyeong_copy_style_reveal` / `무명의 카피 무공 공개`는 첫 대치 이후가 선행 조건이고, 카피 무공의 seed 기반 변주와 청류안 분석/천외편린 후보 변형을 다룬다.
- 같은 handoff에서 `wuxia_mumyeong_midgame_reunion`과 `wuxia_boss_first_appearance`도 비교하되, random copy-style system, 천외편린 reward schema, boss combat/final-wall pressure를 바로 열지 말지 먼저 문서에서 결정한다.

## 0.32 2026-06-02 docs-only post-confrontation handoff: `wuxia_mumyeong_copy_style_reveal`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_followup_after_first_confrontation`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_mumyeong_copy_style_reveal`로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, 기본 office bundle을 수정하지 않았다.

대조한 Notion source:

- 사건 카드 DB `wuxia_mumyeong_copy_style_reveal` / `무명의 카피 무공 공개`: 선행 조건은 `wuxia_mumyeong_first_confrontation` 이후이며, 결과는 이번 회차 무명이 어떤 무공을 카피했는지 드러나는 것이다. 보상/기능은 랜덤 카피 무공 힌트, 청류안 분석 문구, 천외편린 후보 변형, 무명 전투 패턴 변주다.
- 사건 카드 DB `wuxia_mumyeong_midgame_reunion` / `무명 중반 재회`: 선행 조건이 첫 대치 이후와 무명 과거 단서 일부라, copy-style reveal로 과거/카피 단서를 더 쌓은 뒤 여는 편이 안전하다.
- 사건 카드 DB `wuxia_boss_first_appearance` / `보스 첫 등장`: 첫 대치 이후 또는 흑사방 압박 일정 수준이 선행 조건이지만, 보스는 최종 논리 결산을 담당하므로 boss-wall pressure를 너무 일찍 연다.
- `04. 메인 루트 구조`: 무명은 라이벌 결산, 보스는 최종 논리 결산을 담당한다.
- `05. 사건 카드 운영 규칙`: 사건 ID/선행 조건/선택지/결과/보상/후일담 연결을 카드 단위로 관리한다.
- `06. 사이드 퀘스트와 미해결 부채`: 중요한 지연/방치는 메인 엔딩을 막지 않고 후일담/잔상으로 남긴다.
- `07. 천기록 / 천외편린 보상`: 주인공은 흐름을 이해하고 자기 몸에 맞게 재구성하며, 무명은 남의 흐름을 훔쳐 덧씌운다. 단, 천기록은 검색창이 아니고 3택 reward schema는 아직 별도 slice다.
- `99. 통합 체크포인트`: 무명의 매 회차 카피는 청류안과 대비되어야 하고, 카피는 구원/비구원 변주를 만들되 단일 결말로 고정하지 않는다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_mumyeong_copy_style_reveal` | Notion 선행 조건이 정확히 첫 대치 이후이고, `mumyeong_first_confrontation_resolved`/`mumyeong_rival_thread_opened`를 바로 회수한다. 청류안 분석과 무명 카피 결함을 flags/clues/log로 표현할 수 있다. | seed 기반 random copy-style table이나 천외편린 reward schema로 번지면 scope가 커진다. | 다음 runtime 후보 |
| `wuxia_mumyeong_midgame_reunion` | 무명을 라이벌/거울 관계로 강화한다. | 첫 대치 이후뿐 아니라 무명 과거 단서 일부가 필요하다. | 보류 |
| `wuxia_boss_first_appearance` | 흑사방 보스의 최종 벽을 각인한다. | boss-wall/final logic 압박, boss combat, 조직력/약점 읽기 schema가 커진다. | 보류 |
| route-specific clue bridge | route flavor를 더 쌓을 수 있다. | 이미 common rival thread가 열렸고, route fan-out을 더 벌릴 수 있다. | 보류 |

결정:

- 다음 runtime 후보는 `wuxia_mumyeong_copy_style_reveal`로 정한다.
- 이 구현은 "이번 회차 random copy-style system"을 실제 seed table/schema로 열지 않고, 기존 encounter schema의 `flags`, `clues`, `log`, `presentation`으로 카피 계열의 윤곽과 결함을 먼저 보여준다.
- 천외편린 후보 변형은 `fragment_candidate_variation_foreshadowed` 같은 clue로만 남기고, reward/ability/3-choice schema는 열지 않는다.
- `wuxia_mumyeong_midgame_reunion`은 copy-style reveal 후 과거 단서가 충분해질 때까지 보류한다.
- `wuxia_boss_first_appearance`는 보스가 담당하는 최종 논리 결산과 boss-wall pressure를 보존하기 위해 보류한다.

`wuxia_mumyeong_copy_style_reveal` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]`, `forbidden_flags: [mumyeong_copy_style_reveal_resolved]`
- flavor-only flags: `copied_flow_weakness_noted`, `cheonggi_copy_contrast_noted`, `seo_harin_mumyeong_silence_confirmed`, `rival_endured_not_defeated`, `rivalry_deferred_not_avoided`, `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`
- stable choice ids:
  - `read_the_stolen_blade_path`
  - `watch_mumyeongs_footwork`
  - `listen_for_breath_mismatch`
  - `wait_for_body_to_shudder`
- common outcome hook: 모든 선택지는 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary flags/clues: `copied_blade_path_noted`, `copied_footwork_noted`, `copied_breath_mismatch_noted`, `copy_side_effect_seen`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `fragment_candidate_variation_foreshadowed`, `understanding_is_not_copying`.
- presentation 권장: `visual_id: wuxia_mumyeong_copy_style_reveal`, `speaker: 서하린`, `layout: copy_style_analysis`, effect cue stable terms `[무명, 청류안, 천기록]`.
- non-goals: seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, boss first appearance, `wuxia_mumyeong_midgame_reunion`, 무명 과거 진실 reveal, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, 기본 office bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_mumyeong_first_confrontation` 뒤에 `wuxia_mumyeong_copy_style_reveal`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.33 2026-06-02 무협 `wuxia_mumyeong_copy_style_reveal` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_followup_after_first_confrontation` handoff에서 고른 카피 무공 공개 사건을 separate `wuxia_jianghu_pack` storypack preview runtime에 추가했다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_first_confrontation` 뒤에 `wuxia_mumyeong_copy_style_reveal`를 추가했다.
- start conditions는 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]`, `forbidden_flags: [mumyeong_copy_style_reveal_resolved]`다.
- stable choice id는 `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder`다.
- 모든 choice outcome은 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary flags/clues는 `copied_blade_path_noted`, `copied_footwork_noted`, `copied_breath_mismatch_noted`, `copy_side_effect_seen`, `copied_form_family_seen`, `copy_is_surface_not_root`, `footwork_without_root_wobbles`, `breath_mismatch_marks_copy`, `fragment_candidate_variation_foreshadowed`, `understanding_is_not_copying`이다.
- presentation hook은 `visual_id: wuxia_mumyeong_copy_style_reveal`, `speaker: 서하린`, `layout: copy_style_analysis`, stable terms `[무명, 청류안, 천기록]`다.
- Rust/Web generated preview artifacts만 재생성했다: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`.

검증/계약:

- Python exporter/docs/storypack DB tests, Rust content bundle test, WASM JSON boundary test, terminal smoke test, Web `contentBundles` test를 갱신했다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.
- seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, boss first appearance, `wuxia_mumyeong_midgame_reunion`, route graph, faction reputation, relation/debt ledger, reward/ability/epilogue/return system, 천기록 identity reveal은 열지 않았다.

다음 handoff:

- 다음 작업은 `wuxia_mumyeong_followup_after_copy_style_reveal` docs-only handoff다.
- 이제 `wuxia_mumyeong_midgame_reunion`의 선행 조건 중 “첫 대치 이후”와 카피 무공/결함 단서는 충족됐지만, “무명 과거 단서 일부”가 충분한지는 다시 Notion과 repo hooks를 대조해야 한다.
- 같은 handoff에서 `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_reads_orthodox_style` 같은 후속 후보도 비교하되, boss combat/final-wall pressure, 무명 과거 진실 reveal, random copy-style table, reward/ability/epilogue/return schema를 바로 열지 말지 먼저 문서에서 결정한다.

## 0.34 2026-06-02 docs-only post-copy-style handoff: `wuxia_mumyeong_reads_orthodox_style`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_followup_after_copy_style_reveal`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_mumyeong_reads_orthodox_style`로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, 기본 office bundle을 수정하지 않았다.

대조한 Notion source:

- 사건 카드 DB `wuxia_mumyeong_midgame_reunion` / `무명 중반 재회`: 선행 조건이 첫 대치 이후와 무명 과거 단서 일부다. 카피 무공 공개로 카피 결함 단서는 생겼지만, 무명이 왜 정파식 제압술과 단절했는지에 대한 과거 단서가 아직 얇다.
- 사건 카드 DB `wuxia_boss_first_appearance` / `보스 첫 등장`: 첫 대치 이후 또는 흑사방 압박 일정 수준이면 열 수 있지만, 보스는 최종 논리 결산을 담당한다. 지금 열면 boss-wall/final logic pressure와 boss combat 기대가 너무 빨리 커진다.
- 사건 카드 DB `wuxia_mumyeong_departure_truth_summary` / `무명 이탈의 진실 정리`: 현악문, 복호금쇄수, 서하린에게 진실 전달, 무명 구원 조건을 한 번에 정리하는 후반/후일담 트리거 카드다. 지금 열면 무명 과거 진실 reveal과 구원 루트를 너무 직접적으로 확정한다.
- 사건 카드 DB `wuxia_mumyeong_reads_orthodox_style` / `무명의 정파 무공 간파`: 무명이 특별한 눈으로 청류문 습격자의 정파 계열 흔적, 특히 현악문과 복호금쇄수를 읽어낸다는 중반 단서 카드다. 카피 무공 공개가 남긴 `copy_style_hint_recorded`, `copied_form_family_seen`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`을 과거 단서로 연결하기 좋다.
- `04. 메인 루트 구조`: 무명은 라이벌 결산을 담당하고, 보스는 최종 논리 결산을 담당한다. 따라서 보스는 아직 보류하고 무명 과거 단서를 먼저 보강한다.
- `05. 사건 카드 운영 규칙`: 사건 ID/선행 조건/선택지/결과/보상/후일담 연결을 카드 단위로 관리한다.
- `07. 천기록 / 천외편린 보상`: 주인공의 이해/재구성은 무명의 카피와 대비된다. 단, 천기록은 검색창이 아니고 3택 reward schema는 아직 별도 slice다.
- `99. 통합 체크포인트`: 현악문, 복호금쇄수, 무명은 서하린의 사제, 천기록 기록자 정체 미공개가 최신 확정 기준이다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_mumyeong_reads_orthodox_style` | copy-style reveal이 남긴 카피 결함/호흡 불일치 clue를 무명의 과거 시야와 현악문/복호금쇄수 단서로 이어 준다. 기존 flags/clues/log/presentation만으로 표현 가능하다. | `wuxia_qingliu_attack_after_war` 전체 backstory나 무명 이탈 진실을 한 번에 열면 scope가 커진다. | 다음 runtime 후보 |
| `wuxia_mumyeong_midgame_reunion` | 라이벌/거울 관계를 깊게 만든다. | 아직 무명 과거 단서 일부가 부족하다. 먼저 정파 무공 간파 단서가 필요하다. | 보류 |
| `wuxia_boss_first_appearance` | 흑사방 보스의 최종 벽을 각인한다. | boss-wall/final logic pressure, boss combat, 조직력/약점 읽기 schema가 커진다. | 보류 |
| `wuxia_mumyeong_departure_truth_summary` | 무명 구원 루트 핵심 진실을 정리한다. | 후반/후일담 트리거 성격이 강하고, 서하린에게 진실 전달과 구원 조건을 너무 빨리 확정한다. | 보류 |

결정:

- 다음 runtime 후보는 `wuxia_mumyeong_reads_orthodox_style`로 정한다.
- 이 구현은 `wuxia_qingliu_attack_after_war` 전체 과거 회상이나 `wuxia_mumyeong_departure_truth_summary`의 진실 reveal을 열지 않는다.
- 현악문/복호금쇄수는 최신 확정명으로 사용하되, “무명이 청류문을 떠난 진실 전체”가 아니라 “무명이 읽어낸 정파식 통제 무공의 흔적”으로 제한한다.
- 천기록/청류안은 카피와 대비되는 관찰/재구성 clue만 남기고, 3택 reward/ability schema는 열지 않는다.

`wuxia_mumyeong_reads_orthodox_style` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_reads_orthodox_style_resolved]`
- flavor-only flags: `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed`, `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`
- stable choice ids:
  - `compare_copied_form_to_old_wound`
  - `trace_qingliu_eye_variation`
  - `reconstruct_mumyeongs_sightline`
  - `stop_before_truth_becomes_accusation`
- common outcome hook: 모든 선택지는 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary flags/clues: `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete`.
- presentation 권장: `visual_id: wuxia_mumyeong_reads_orthodox_style`, `speaker: 천기록`, `layout: orthodox_style_trace`, effect cue stable terms `[현악문, 복호금쇄수, 무명]`.
- non-goals: `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war` full flashback, boss combat, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, 기본 office bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_mumyeong_copy_style_reveal` 뒤에 `wuxia_mumyeong_reads_orthodox_style`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.35 2026-06-02 무협 `wuxia_mumyeong_reads_orthodox_style` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_followup_after_copy_style_reveal` handoff에서 고른 무명의 정파 무공 간파 사건을 separate `wuxia_jianghu_pack` storypack preview runtime에 추가했다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_copy_style_reveal` 뒤에 `wuxia_mumyeong_reads_orthodox_style`를 추가했다.
- start conditions는 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_reads_orthodox_style_resolved]`다.
- stable choice id는 `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation`다.
- 모든 choice outcome은 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary flags/clues는 `old_wound_angle_compared`, `qingliu_eye_variation_traced`, `mumyeong_sightline_reconstructed`, `truth_accusation_avoided`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete`다.
- presentation hook은 `visual_id: wuxia_mumyeong_reads_orthodox_style`, `speaker: 천기록`, `layout: orthodox_style_trace`, stable terms `[현악문, 복호금쇄수, 무명]`다.
- Rust/Web generated preview artifacts만 재생성했다: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`.

검증/계약:

- Python exporter/docs/storypack DB tests, Rust content bundle test, WASM JSON boundary test, terminal smoke test, Web `contentBundles` test를 갱신했다.
- 기본 `src/tui_adv/data/*.yaml`, 기본 `content.bundle.json`, Web 기본 generated bundle, legacy `escape-office` save/localStorage key는 변경하지 않는다.
- 무명 중반 재회, 보스 첫 등장, 무명 이탈 진실 정리, `wuxia_qingliu_attack_after_war` full flashback, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability/epilogue/return system, 천기록 identity reveal은 열지 않았다.

다음 handoff:

- 다음 작업은 `wuxia_mumyeong_followup_after_orthodox_style_trace` docs-only handoff다.
- 이제 `wuxia_mumyeong_midgame_reunion`의 선행 조건인 첫 대치 이후, 카피 무공/결함 단서, 무명 과거 단서 일부가 더 충족됐다.
- 같은 handoff에서 `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`를 비교하되, 보스 전투, 무명 구원 확정, 서하린에게 진실 전달, full flashback, reward/ability/epilogue/return schema를 바로 열지 말지 먼저 문서에서 결정한다.

## 0.36 2026-06-02 docs-only orthodox-style follow-up handoff: `wuxia_mumyeong_midgame_reunion`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_followup_after_orthodox_style_trace`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_mumyeong_midgame_reunion`으로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, Web default bundle, legacy office bundle을 수정하지 않았다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 앞으로의 메인 개발 기준이다.

대조한 Notion source:

- 사건 카드 DB `wuxia_mumyeong_midgame_reunion` / `무명 중반 재회`: 선행 조건은 무명 첫 대치 이후와 무명 과거 단서 일부다. repo runtime에는 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete`가 이미 남아 있어 이제 조건이 충분하다.
- 사건 카드 DB `wuxia_mumyeong_departure_truth_summary` / `무명 이탈의 진실 정리`: 후반 truth/salvation route 카드다. 서하린에게 진실 전달, 무명 구원 조건, 청류문 습격 배후를 한 번에 정리하므로 다음 runtime으로는 너무 크다.
- 사건 카드 DB `wuxia_boss_first_appearance` / `보스 첫 등장`: 무명 첫 대치 이후 열 수 있지만, 보스는 최종 논리 벽을 담당한다. 지금 열면 boss-wall/final logic pressure와 boss combat 기대가 너무 빨리 커진다.
- 사건 카드 DB `wuxia_qingliu_attack_after_war` / `무너져가는 청류문 습격`: 무명 이탈의 직접 원인과 현악문/복호금쇄수 흔적을 제공하지만, full flashback/backstory reveal 성격이 강하다. 이번에는 source clue로만 남긴다.
- `04. 메인 루트 구조`: 무명은 라이벌/거울 관계의 결산을 담당하고, 보스는 최종 논리 결산을 담당한다.
- `05. 사건 카드 운영 규칙`: 사건 카드는 ID, 관련 인물/세력, 선행 조건, 선택지 요약, 결과/보상, 미해결 부채, 후일담 연결을 갖춘다.
- `07. 천기록 / 천외편린 보상`: 천기록은 검색창이 아니며 정답이나 정체를 직접 주지 않는다. 주인공은 이해/재구성하고, 무명은 훔쳐/복사한다.
- `99. 통합 체크포인트`: 무명은 라이벌이고 구원 가능하지만 단일 초식으로 해결되지 않는다. 보스는 구원 대상이 아니라 최종 논리 벽이다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_mumyeong_midgame_reunion` | 첫 대치, 카피 무공, 정파식 통제 무공 clue를 무명/서하린 관계와 라이벌 거울 구조로 이어 준다. 기존 flags/clues/log/presentation만으로 구현 가능하다. | 진실을 추궁으로 만들거나 무명 구원을 확정하면 후반 truth route가 당겨진다. | 다음 runtime 후보 |
| `wuxia_mumyeong_departure_truth_summary` | 무명 이탈의 진실과 구원 route 핵심 조건을 정리한다. | 후반 truth reveal, 서하린에게 진실 전달, 구원 조건 확정 범위가 크다. | 보류 |
| `wuxia_boss_first_appearance` | 흑사방 보스의 압도적 벽을 각인한다. | boss-wall/final logic pressure와 boss combat 기대가 커진다. | 보류 |
| `wuxia_qingliu_attack_after_war` | 현악문/복호금쇄수와 청류문 붕괴 원인을 직접 보여 준다. | full flashback/backstory reveal이 되어 중반 라이벌 재회를 덮는다. | 보류 |

`wuxia_mumyeong_midgame_reunion` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]`, `forbidden_flags: [mumyeong_midgame_reunion_resolved]`
- flavor-only flags: `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete`, `seo_harin_mumyeong_silence_confirmed`, `copied_flow_weakness_noted`, `copy_style_hint_recorded`
- stable choice ids:
  - `ask_why_seoharin_never_called_him_traitor`
  - `show_the_hyeonakmun_trace_without_accusing`
  - `point_out_the_copied_form_gap`
  - `keep_blades_low_and_watch_his_answer`
- common outcome hook: 모든 선택지는 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues: `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation`.
- presentation 권장: `visual_id: wuxia_mumyeong_midgame_reunion`, `speaker: 무명`, `layout: rival_reunion_trace`, effect cue stable terms `[무명, 서하린, 현악문]`.
- non-goals: `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war` full flashback, boss combat, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, legacy office bundle/default bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_mumyeong_reads_orthodox_style` 뒤에 `wuxia_mumyeong_midgame_reunion`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, legacy office `content.bundle.json`, Web legacy generated `content.bundle.json`, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.37 2026-06-02 무협 `wuxia_mumyeong_midgame_reunion` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_midgame_reunion`을 `wuxia_mumyeong_reads_orthodox_style` 뒤에 storypack preview runtime으로 추가했고, Web/terminal default storypack인 `wuxia_jianghu_pack` / **이구학지 — 천기록** 진행선에 포함했다.

구현 결과:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 `wuxia_mumyeong_midgame_reunion`을 추가했다.
- start condition은 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`를 요구하고, `mumyeong_midgame_reunion_resolved`를 forbidden으로 둔다.
- stable choice id는 `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer`다.
- 모든 선택지는 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary clue는 `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation`다.
- presentation은 `visual_id: wuxia_mumyeong_midgame_reunion`, `speaker: 무명`, `layout: rival_reunion_trace`, stable terms `[무명, 서하린, 현악문]`으로 고정했다.
- Rust/Web generated storypack preview bundle을 갱신했다.
  - `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
  - `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

검증/계약:

- Python exporter는 preview encounter count와 새 choice/outcome hook을 검증한다.
- Web content bundle registry는 default `storypack_main` wrapper가 새 encounter까지 포함하는지 검증한다.
- Rust terminal smoke는 orthodox-style trace 이후 `무명 중반 재회` 장면과 네 choice id를 검증한다.
- WASM JSON boundary는 같은 액션 체인에서 재회 page와 `show_the_hyeonakmun_trace_without_accusing` outcome flag/clue를 검증한다.

다음 handoff:

- 다음 작업은 `wuxia_mumyeong_followup_after_midgame_reunion` docs-only handoff다.
- 후보는 `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`를 우선 비교한다.
- 무명 이탈 진실 전체 reveal, 보스 첫 등장/전투, 청류문 습격 full flashback, 서하린에게 진실 전달, 구원 확정, reward/ability/combat/route graph/faction/relation/debt/epilogue/return schema, 천기록 identity reveal은 이 runtime slice에서 열지 않았다.

## 0.38 2026-06-02 docs-only midgame-reunion follow-up handoff: `wuxia_boss_first_appearance`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_followup_after_midgame_reunion`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_boss_first_appearance`로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, Web default bundle, legacy office bundle을 수정하지 않았다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 앞으로의 메인 개발 기준이다.

대조한 Notion source:

- 사건 카드 DB `wuxia_boss_first_appearance` / `보스 첫 등장`: 진행 단계는 중반이고, 선행 조건은 흑사방 압박 일정 수준 또는 무명 첫 대치 이후다. 결과는 보스가 압도적 벽으로 등장하고, 청류안으로 읽어도 몸이 따라가지 못한다는 점을 각인한다. 선택지 요약은 흐름을 본다 / 물러난다 / 서하린을 보호한다 / 무명의 반응을 본다.
- 사건 카드 DB `wuxia_mumyeong_departure_truth_summary` / `무명 이탈의 진실 정리`: 진행 단계가 후반이며, 서하린에게 진실 전달과 무명 구원 조건을 직접 건드린다. 이번 runtime으로 열기에는 truth/salvation route 범위가 너무 크다.
- 사건 카드 DB `wuxia_qingliu_attack_after_war` / `무너져가는 청류문 습격`: 중반 카드이지만 무명 이탈의 직접 계기와 현악문/복호금쇄수 과거를 full backstory로 보여 주는 성격이 강하다. 현재는 boss pressure 전후의 source clue로 보류한다.
- 사건 카드 DB `wuxia_mumyeong_request_for_aid` / `무명의 도움 요청`: Notion search에서 확인한 중반 backstory bridge다. 무명이 청류문을 살리려 했다는 정파 모순 단서를 주지만, repo의 현 handoff 후보 3개에는 아직 올라와 있지 않으므로 이번에는 future bridge로 남긴다.
- `04. 메인 루트 구조`: 최종 결산은 보스가 대표하는 논리이며, 무명은 라이벌 결산을 담당한다.
- `05. 사건 카드 운영 규칙`: 사건 카드는 ID, 관련 인물/세력, 선행 조건, 선택지 요약, 결과/보상, 미해결 부채, 후일담 연결을 갖춘다.
- `07. 천기록 / 천외편린 보상`: 보스는 무공의 흐름보다 사람과 조직의 약점을 읽는다. 천기록/청류안이 강해질수록 이 약점 읽기와 침식 위험을 조심해야 한다.
- `99. 통합 체크포인트`: 보스는 갱생 대상이 아니라 자기 논리와 질서가 있는 벽이며, 단순 무력보다 조직력과 약점 읽기가 핵심이다. 흑사방주 사도는 별호이며 본명은 밝히지 않는다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_boss_first_appearance` | 중반에서 흑사방 보스의 압도감과 조직/약점 읽기 논리를 처음 각인한다. `mumyeong_midgame_reunion_resolved` 이후 기존 flags/clues/log/presentation만으로 구현 가능하다. | boss combat이나 final boss 결산으로 밀면 너무 이르다. | 다음 runtime 후보 |
| `wuxia_mumyeong_departure_truth_summary` | 무명 이탈 진실과 구원 route 조건을 정리한다. | 후반 truth reveal, 서하린에게 진실 전달, 구원 조건 확정 범위가 크다. | 보류 |
| `wuxia_qingliu_attack_after_war` | 현악문/복호금쇄수와 청류문 붕괴 원인을 직접 보여 준다. | full flashback/backstory reveal이 되어 현재 중반 pressure를 덮는다. | 보류 |
| `wuxia_mumyeong_request_for_aid` | 무명이 청류문을 살리려 했다는 bridge로 보스 논리의 설득력을 준비한다. | repo 후보 DB에 아직 next candidate로 구체화되지 않았고, departure truth와 붙으면 후반 reveal을 앞당긴다. | future bridge |

`wuxia_boss_first_appearance` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]`, `forbidden_flags: [boss_first_appearance_resolved]`
- flavor-only flags/clues: `boss_used_mumyeongs_wound`, `hyeonakmun_trace_shared_without_accusation`, `seoharin_does_not_call_mumyeong_traitor`, `rival_mirror_relationship_deepened`, `reunion_truth_deferred`
- stable choice ids:
  - `read_the_boss_flow_and_fail_to_move`
  - `pull_seo_harin_behind_broken_gate`
  - `watch_mumyeong_answer_the_boss`
  - `retreat_before_the_second_step`
- common outcome hook: 모든 선택지는 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues: `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet`.
- presentation 권장: `visual_id: wuxia_boss_first_appearance`, `speaker: 흑사방주`, `layout: boss_wall_pressure`, effect cue stable terms `[흑사방주, 무명, 청류문]`.
- non-goals: boss combat/final boss resolution, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war` full flashback, `wuxia_mumyeong_request_for_aid` backstory bridge, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, legacy office bundle/default bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_mumyeong_midgame_reunion` 뒤에 `wuxia_boss_first_appearance`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, legacy office `content.bundle.json`, Web legacy generated `content.bundle.json`, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.39 2026-06-02 무협 `wuxia_boss_first_appearance` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_followup_after_midgame_reunion` handoff에서 선택한 `wuxia_boss_first_appearance`를 `wuxia_jianghu_pack` storypack preview source에 추가했다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 앞으로의 메인 개발 기준이다.

구현 범위:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_midgame_reunion`
- generated artifacts:
  - `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
  - `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- start conditions: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]`, `forbidden_flags: [boss_first_appearance_resolved]`
- stable choice ids: `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step`
- common outcome hook: `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet`
- presentation: `visual_id: wuxia_boss_first_appearance`, `speaker: 흑사방주`, `layout: boss_wall_pressure`, effect cue stable terms `[흑사방주, 무명, 청류문]`

검증/동기화:

- Python exporter/docs/storypack DB 테스트는 storypack preview encounter count와 보스 첫 등장 conditions/presentation/choice hooks를 검증한다.
- WASM JSON boundary는 `wuxia_mumyeong_midgame_reunion` 이후 보스 첫 등장 scene page와 `watch_mumyeong_answer_the_boss` 결과 hook을 검증한다.
- SuperLightTUI smoke는 같은 action chain으로 `보스 첫 등장`, `wuxia_boss_first_appearance`, `boss_wall_pressure`, stable terms, 네 선택지를 표시하는지 검증한다.
- Web default content bundle registry는 이구학지 default bundle encounter list 끝에 `wuxia_boss_first_appearance`가 포함되는지 확인한다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- boss combat/final boss resolution
- `wuxia_mumyeong_departure_truth_summary`
- `wuxia_qingliu_attack_after_war` full flashback
- `wuxia_mumyeong_request_for_aid` backstory bridge 구현
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle 변경

후속 docs-only handoff 기록:

- `wuxia_boss_followup_after_first_appearance` docs-only handoff는 아래 0.40에서 완료했다.
- 비교 후보는 `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`이었다.
- 결정된 다음 runtime 후보는 `wuxia_mumyeong_request_for_aid`다.

## 0.40 2026-06-02 docs-only boss follow-up handoff: `wuxia_mumyeong_request_for_aid`

현재 상태: docs-only handoff 완료. `wuxia_boss_followup_after_first_appearance`에서 Notion 사건 카드 DB와 repo runtime hook을 대조해 다음 runtime 후보를 `wuxia_mumyeong_request_for_aid`로 결정했다. 이번 slice에서는 runtime YAML, Rust/Web generated preview bundle, Web default bundle, legacy office bundle을 수정하지 않았다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 앞으로의 메인 개발 기준이다.

대조한 Notion source:

- 사건 카드 DB `wuxia_mumyeong_request_for_aid` / `무명의 도움 요청`: 진행 단계는 중반이고, 선행 조건은 무명 과거 단서 획득이다. 결과는 무명이 청류문을 살리기 위해 여러 정파 문파에 도움을 청했으나 거절당했음이 드러나는 것이다. 선택지 요약은 정파 문파들의 기록을 찾는다 / 객잔 소문을 추적한다 / 서하린에게 묻는다. 기능은 무명을 단순 배신자가 아니라 청류문을 살리려다 꺾인 인물로 만들고, 보스의 힘 논리가 왜 무명에게 설득력 있었는지 준비하는 것이다.
- 사건 카드 DB `wuxia_mumyeong_departure_truth_summary` / `무명 이탈의 진실 정리`: 진행 단계가 후반이며, 서하린에게 진실 전달과 무명 구원 조건을 직접 건드린다. 이번 runtime으로 열기에는 truth/salvation route 범위가 너무 크다.
- 사건 카드 DB `wuxia_qingliu_attack_after_war` / `무너져가는 청류문 습격`: 진행 단계는 중반이지만 청류문 습격과 현악문/복호금쇄수의 full source를 보여 주는 backstory 성격이 강하다. 지금 열면 보스 첫 등장 직후의 압박이 full flashback에 덮인다.
- 사건 카드 DB `wuxia_boss_resolution` / `보스 결산`: 진행 단계는 최종장이고, `wuxia_sado_final_battle` 이후 사도 패배/생존/조직망/무명 후계/후일담 출력을 정리한다. 현재는 boss combat/final resolution 범위라 보류한다.
- `04. 메인 루트 구조`: 최종 결산은 보스가 대표하는 논리를 넘는 구조지만, 최종장 전에는 루트 후보를 점진적으로 좁혀야 한다.
- `05. 사건 카드 운영 규칙`: 사건 카드는 ID, 관련 인물/세력, 선행 조건, 선택지 요약, 결과/보상, 미해결 부채, 후일담 연결을 갖춘다.
- `06. 사이드 퀘스트와 미해결 부채`: 중요한 사건을 방치해도 메인 엔딩을 직접 막기보다 후일담/잔상으로 남기는 운영 원칙을 유지한다.
- `07. 천기록 / 천외편린 보상`: 보스는 무공보다 사람과 조직의 약점을 읽으며, 천기록/청류안이 사람의 약점까지 보게 되는 침식 위험으로 이어질 수 있다.
- `99. 통합 체크포인트`: 보스는 갱생하지 않고 자기 논리와 질서를 가진 벽이며, 단순 무력보다 조직력과 약점 읽기가 핵심이다. 무명은 구원 가능하지만 구원 형태는 단일하지 않아야 한다.

후보 비교:

| 후보 | 장점 | 위험 | 결정 |
|---|---|---|---|
| `wuxia_mumyeong_request_for_aid` | 보스 첫 등장으로 열린 “힘의 논리가 무명에게 왜 설득력 있었나”를 중반 backstory bridge로 준비한다. 기존 flags/clues/log/presentation만으로 구현 가능하다. | departure truth와 너무 붙이면 후반 truth reveal이 당겨진다. | 다음 runtime 후보 |
| `wuxia_mumyeong_departure_truth_summary` | 무명 이탈 진실과 구원 route 조건을 정리한다. | 후반 truth reveal, 서하린에게 진실 전달, 구원 조건 확정 범위가 크다. | 보류 |
| `wuxia_qingliu_attack_after_war` | 현악문/복호금쇄수와 청류문 붕괴 원인을 직접 보여 준다. | full flashback/backstory reveal이 되어 현재 중반 pressure를 덮는다. | 보류 |
| `wuxia_boss_resolution` | 보스가 대표하는 최종 논리와 후일담 결산을 정리한다. | final boss resolution, 사도 최종전 결과, 조직망/후일담 출력까지 필요하다. | 보류 |

`wuxia_mumyeong_request_for_aid` 설계 경계:

- start conditions: `runtime_mode: storypack_preview`, `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, mumyeong_mirror_thread_deepened, orthodox_style_trace_recorded, midgame_continuity_started]`, `forbidden_flags: [mumyeong_request_for_aid_resolved]`
- flavor-only flags/clues: `boss_used_mumyeongs_wound`, `hyeonakmun_trace_shared_without_accusation`, `mumyeong_follows_power_that_saw_his_wound`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `qingliu_cannot_outmuscle_boss_yet`, `seoharin_does_not_call_mumyeong_traitor`
- stable choice ids:
  - `search_the_rejected_aid_letters`
  - `follow_old_inn_rumors_about_mumyeong`
  - `ask_seo_harin_what_help_never_came`
  - `keep_the_failed_aid_record_unshown`
- common outcome hook: 모든 선택지는 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues: `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid`.
- presentation 권장: `visual_id: wuxia_mumyeong_request_for_aid`, `speaker: 천기록`, `layout: failed_aid_records`, effect cue stable terms `[무명, 청류문, 정파]`.
- non-goals: `wuxia_mumyeong_departure_truth_summary` truth reveal, 서하린에게 진실 전달, 무명 구원 확정, `wuxia_qingliu_attack_after_war` full flashback, `wuxia_boss_resolution`, boss combat/final boss resolution, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천외편린 3택 성장, epilogue/return system, 천기록 identity reveal, legacy office bundle/default bundle 변경.

다음 runtime 구현 handoff:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`: `wuxia_boss_first_appearance` 뒤에 `wuxia_mumyeong_request_for_aid`를 추가한다.
- Rust/Web generated storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 `src/tui_adv/data/*.yaml`, legacy office `content.bundle.json`, Web legacy generated `content.bundle.json`, legacy `escape-office` save/localStorage key는 변경하지 않는다.

## 0.41 2026-06-02 무협 `wuxia_mumyeong_request_for_aid` preview runtime slice

현재 상태: preview runtime 구현 완료. `wuxia_boss_followup_after_first_appearance` handoff에서 선택한 failed-aid records bridge를 `wuxia_jianghu_pack` preview source와 Rust/Web generated bundle에 올렸다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 앞으로의 메인 개발 기준이다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_first_appearance` 뒤에 `wuxia_mumyeong_request_for_aid`를 추가했다.
- start condition은 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `mumyeong_mirror_thread_deepened`, `orthodox_style_trace_recorded`, `midgame_continuity_started`이며, `mumyeong_request_for_aid_resolved`로 반복을 막는다.
- stable choice id는 `search_the_rejected_aid_letters`, `follow_old_inn_rumors_about_mumyeong`, `ask_seo_harin_what_help_never_came`, `keep_the_failed_aid_record_unshown`다.
- 모든 선택지는 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- `search_the_rejected_aid_letters`는 `rejected_aid_letter_fragment` item을 추가한다.
- clues는 `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid`를 branch별로 남긴다.
- presentation은 `visual_id: wuxia_mumyeong_request_for_aid`, `speaker: 천기록`, `layout: failed_aid_records`, stable terms `[무명, 청류문, 정파]`로 구현했다.
- Rust/Web storypack preview generated bundle은 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.

검증/동기화:

- Python exporter/docs/storypack DB 테스트는 storypack preview encounter/item count와 도움 요청 encounter conditions/presentation/choice hooks/item/clues를 검증한다.
- WASM JSON boundary는 보스 첫 등장 이후 `wuxia_mumyeong_request_for_aid` scene page와 `search_the_rejected_aid_letters` 결과 hook을 검증한다.
- SuperLightTUI smoke는 같은 action chain으로 `무명의 도움 요청`, `wuxia_mumyeong_request_for_aid`, `failed_aid_records`, stable terms, 네 선택지를 표시하는지 검증한다.
- Web default content bundle registry는 이구학지 default bundle encounter list 끝에 `wuxia_mumyeong_request_for_aid`가 포함되는지 확인한다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- `wuxia_mumyeong_departure_truth_summary` truth reveal
- `wuxia_qingliu_attack_after_war` full flashback
- `wuxia_boss_resolution` final boss resolution
- boss combat
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle 변경

후속 docs-only handoff 기록:

- `wuxia_mumyeong_followup_after_failed_aid` docs-only handoff는 완료됐고, 다음 runtime 후보는 `wuxia_mumyeong_awakening`이다.
- 이 구현 전 handoff는 runtime YAML/Rust/Web generated bundle, Web default bundle, legacy office bundle을 변경하지 않았다.
- 다음 runtime 후보를 구현하기 전까지 truth reveal, full flashback, final boss resolution, reward/ability/combat/route graph schema는 열지 않는다.

## 0.42 2026-06-02 docs-only failed-aid follow-up handoff: `wuxia_mumyeong_awakening`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_request_for_aid` 뒤의 후보를 Notion 사건 카드 DB와 repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_mumyeong_awakening`으로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

Notion 대조 source:

- `09. 이구학지 사건 카드 DB`: `wuxia_mumyeong_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`
- 운영 문서 `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`

결정:

- 선택: `wuxia_mumyeong_awakening`. `wuxia_mumyeong_request_for_aid`가 남긴 failed-aid 기록과 `wuxia_mumyeong_reads_orthodox_style`가 남긴 현악문/복호금쇄수 단서를, 무명의 분노와 카피 무공 변질로 연결할 수 있다. Notion의 핵심 대비인 “주인공은 흐름을 이해해 재구성하지만, 무명은 분노 속에서 훔친 흐름을 덧씌운다”를 기존 flags/clues/log/presentation으로 표현한다.
- 보류: `wuxia_mumyeong_departure_truth_summary`. 후반 truth, 서하린에게 진실 전달, 무명 구원 조건을 너무 직접 건드린다.
- 보류: `wuxia_qingliu_attack_after_war`. 현악문/복호금쇄수 source로 중요하지만, 지금 구현하면 full flashback/backstory reveal이 된다.
- 보류: `wuxia_boss_resolution`. final boss/faction/epilogue 결산 범위라 현재 중반 bridge가 아니다.
- 보류: `wuxia_boss_recruits_mumyeong`. `wuxia_mumyeong_destroys_orthodox_sect` 이후 후반 스카웃/동화 사건이라 아직 이르다.
- 보류: `wuxia_mumyeong_destroys_orthodox_sect`. `wuxia_mumyeong_awakening` 이후의 결정적 결과이며, 무명 이탈 이유를 너무 크게 확정한다.

다음 runtime implementation 계약:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_request_for_aid`
- location: `cheongryu_outer_courtyard`
- required flags: `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `midgame_continuity_started`
- forbidden flags: `mumyeong_awakening_resolved`
- flavor-only flags/clues: `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete`, `seoharin_does_not_know_failed_aid`
- stable choice ids: `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation`
- common outcome hooks: `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready`
- presentation: `visual_id: wuxia_mumyeong_awakening`, `speaker: 천기록`, `layout: anger_copy_bloom`, stable terms `[무명, 카피, 분노]`

non-goals:

- reward/ability schema, seed 기반 random copy-style table, combat resolver/schema, route graph/faction reputation/debt/relation schema
- full 청류문 습격 flashback, 무명 이탈 진실 전체 reveal, 서하린에게 진실 전달, 무명 구원 확정
- `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_boss_resolution`
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key, 천기록 identity reveal

## 0.43 2026-06-02 무협 `wuxia_mumyeong_awakening` preview runtime slice

현재 상태: preview runtime 구현 완료. `wuxia_mumyeong_followup_after_failed_aid` handoff에서 선택한 anger/copy bloom bridge를 `wuxia_jianghu_pack` preview source와 Rust/Web generated bundle에 올렸다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_request_for_aid` 뒤에 `wuxia_mumyeong_awakening`을 추가했다.
- start condition은 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `midgame_continuity_started`이며, `mumyeong_awakening_resolved`로 반복을 막는다.
- stable choice id는 `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation`다.
- 모든 선택지는 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- clues는 `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready`를 branch별로 남긴다.
- presentation은 `visual_id: wuxia_mumyeong_awakening`, `speaker: 천기록`, `layout: anger_copy_bloom`, stable terms `[무명, 카피, 분노]`로 구현했다.
- Rust/Web storypack preview generated bundle은 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.

검증/동기화:

- Python exporter/docs/storypack DB 테스트는 storypack preview encounter count와 무명 각성 encounter conditions/presentation/choice hooks/clues를 검증한다.
- WASM JSON boundary는 도움 요청 이후 `wuxia_mumyeong_awakening` scene page와 `compare_anger_to_copied_flow` 결과 hook을 검증한다.
- SuperLightTUI smoke는 같은 action chain으로 `무명의 각성`, `wuxia_mumyeong_awakening`, `anger_copy_bloom`, stable terms, 네 선택지를 표시하는지 검증한다.
- Web default content bundle registry는 이구학지 default bundle encounter list 끝에 `wuxia_mumyeong_awakening`이 포함되는지 확인한다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- `wuxia_mumyeong_departure_truth_summary` truth reveal
- `wuxia_qingliu_attack_after_war` full flashback
- `wuxia_mumyeong_destroys_orthodox_sect` consequence implementation
- `wuxia_boss_recruits_mumyeong` 후반 스카웃 구현
- `wuxia_boss_resolution` final boss resolution
- boss combat
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle 변경

후속 docs-only handoff 기록:

- 다음은 `wuxia_mumyeong_followup_after_awakening` docs-only handoff다.
- 다음 handoff는 최소 `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_resolution`을 Notion 사건 카드 DB와 repo hooks로 다시 비교한다.
- 다음 runtime 후보를 고르기 전까지 정파 문파 멸문, 보스 스카웃, departure truth reveal, full flashback, final boss resolution, reward/ability/combat/route graph schema는 열지 않는다.

## 0.44 2026-06-02 docs-only awakening follow-up handoff: `wuxia_qingliu_attack_after_war`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_awakening` 뒤의 후보를 Notion 사건 카드 DB와 repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_qingliu_attack_after_war`로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

확인한 Notion source:

- `09. 이구학지 사건 카드 DB`: `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_resolution`
- future/guardrail source: `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_request_for_aid`
- 운영 문서: `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`

결정:

- 선택: `wuxia_qingliu_attack_after_war`. Notion row는 중반 단계이며, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_awakening`가 이미 남긴 failed-aid/현악문/복호금쇄수/무명 각성 hook을 가장 직접적으로 받는다.
- 제한: 이번 runtime 구현은 청류문 습격 전체 회상이나 backstory reveal이 아니라 **현악문/복호금쇄수 흔적 조사**로만 다룬다.
- 보류: `wuxia_mumyeong_destroys_orthodox_sect`. 무명 각성 이후의 후반 decisive consequence이며, 정파 문파 멸문과 무명 이탈 이유를 너무 크게 확정한다.
- 보류: `wuxia_boss_recruits_mumyeong`. 정파 문파 멸문 이후 보스가 무명의 상처를 흑사방 power로 전환하는 후반 스카웃 사건이다.
- 보류: `wuxia_mumyeong_departure_truth_summary`. 서하린에게 진실 전달, 무명 구원 조건, 후반 route 조건을 너무 직접적으로 연다.
- 보류: `wuxia_mumyeong_resolution`. 최종장/epilogue routing 범위다.
- 보류: `wuxia_boss_resolution`. final boss/faction/epilogue 결산 범위다.

다음 runtime 구현 계약:

- encounter id: `wuxia_qingliu_attack_after_war`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_awakening`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `mumyeong_awakening_resolved`
  - `mumyeong_awakening_thread_opened`
  - `copy_corruption_thread_opened`
  - `mumyeong_request_for_aid_resolved`
  - `mumyeong_failed_aid_thread_opened`
  - `orthodox_hypocrisy_thread_opened`
  - `mumyeong_reads_orthodox_style_resolved`
  - `orthodox_style_trace_recorded`
  - `midgame_continuity_started`
- forbidden flags:
  - `qingliu_attack_after_war_resolved`
- flavor-only flags/clues:
  - `hyeonakmun_trace_suspected`
  - `bokho_geumsaesu_name_recorded`
  - `mumyeong_tried_to_save_qingliu`
  - `orthodox_refusal_broke_mumyeong`
  - `seoharin_does_not_know_failed_aid`
  - `salvation_truth_still_unready`
  - `awakening_points_to_hyeonakmun_without_full_truth`
- stable choice ids:
  - `inspect_bokho_lock_scars`
  - `compare_hyeonakmun_trace_to_qingliu_wounds`
  - `ask_seo_harin_what_she_saw_afterward`
  - `stop_before_replaying_the_attack`
- common outcome hooks:
  - flags: `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`
  - clues: `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`
  - `destination_id: cheongryu_outer_courtyard`
- presentation: `visual_id: wuxia_qingliu_attack_after_war`, `speaker: 천기록`, `layout: attack_trace_investigation`, stable terms `[청류문, 현악문, 복호금쇄수]`

non-goals:

- 청류문 습격 full flashback/backstory reveal
- 정파 문파 멸문 runtime 구현
- 보스 스카웃/보스 전투/final boss resolution
- 무명 이탈 진실 전체 reveal
- 서하린에게 진실 전달
- 무명 구원 확정
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated artifact 변경은 이 handoff commit에서 하지 않는다

## 0.45 2026-06-02 무협 `wuxia_qingliu_attack_after_war` preview runtime slice

현재 상태: preview runtime 구현 완료. `wuxia_mumyeong_awakening` 뒤에 청류문 습격 흔적 조사 encounter를 추가했고, full flashback/backstory reveal 없이 현악문/복호금쇄수 trace만 기존 encounter schema로 landing했다. 기본 storypack은 계속 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_awakening` 뒤에 `wuxia_qingliu_attack_after_war`를 추가했다.
- start condition은 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`이며, `qingliu_attack_after_war_resolved`로 반복을 막는다.
- stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`다.
- 모든 선택지는 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues는 `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`다.
- presentation은 `visual_id: wuxia_qingliu_attack_after_war`, `speaker: 천기록`, `layout: attack_trace_investigation`, stable terms `[청류문, 현악문, 복호금쇄수]`로 구현했다.

Generated artifacts:

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

검증:

- Python exporter/docs/storypack DB 테스트는 21개 이구학지 encounter와 `wuxia_qingliu_attack_after_war` 카드 구현 상태를 확인한다.
- Rust content bundle fixture 테스트는 청류문 encounter 조건, presentation, 첫 choice outcome hooks를 검증한다.
- WASM JSON boundary는 무명 각성 이후 `wuxia_qingliu_attack_after_war` scene page와 `inspect_bokho_lock_scars` 결과 hook을 검증한다.
- SuperLightTUI smoke는 같은 action chain으로 `무너져가는 청류문 습격의 흔적`, `wuxia_qingliu_attack_after_war`, `attack_trace_investigation`, stable terms, 네 선택지를 표시하는지 검증한다.
- Web default content bundle registry는 이구학지 default bundle encounter list 끝에 `wuxia_qingliu_attack_after_war`가 포함되는지 확인한다.

non-goals:

- 청류문 습격 full flashback/backstory reveal
- 정파 문파 멸문 runtime 구현
- 보스 스카웃/보스 전투/final boss resolution
- 무명 이탈 진실 전체 reveal
- 서하린에게 진실 전달
- 무명 구원 확정
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경

다음은 `wuxia_qingliu_attack_after_war_followup` docs-only handoff다. 다음 handoff는 최소 `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`을 Notion 사건 카드 DB와 repo hooks로 다시 비교한다.

## 0.46 2026-06-02 docs-only post-Qingliu trace handoff: `wuxia_mumyeong_destroys_orthodox_sect`

현재 상태: docs-only handoff 완료. `wuxia_qingliu_attack_after_war` 뒤의 후보를 Notion 사건 카드 DB와 repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_mumyeong_destroys_orthodox_sect`로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

확인한 Notion source:

- parent page `무협 스토리팩: 이구학지 — 천기록`
- `09. 이구학지 사건 카드 DB`: `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`
- 운영 문서: `04. 메인 루트 구조`, `99. 통합 체크포인트`

결정:

- 선택: `wuxia_mumyeong_destroys_orthodox_sect`. `wuxia_qingliu_attack_after_war`가 남긴 `qingliu_attack_trace_confirmed`와 `hyeonakmun_attack_thread_opened`를 가장 직접적으로 회수한다.
- 제한: 다음 runtime 구현은 현악문 멸문 전투나 full flashback이 아니라 **빈 현악문 산문, 부러진 현판, 남은 기록/풍문을 확인하는 consequence trace**로만 다룬다.
- 보류: `wuxia_boss_recruits_mumyeong`. Notion 선행 조건이 정파 문파 멸문 이후라 아직 직접 구현 순서가 아니다.
- 보류: `wuxia_mumyeong_departure_truth_summary`. 후반 truth summary이며 서하린에게 진실 전달과 무명 구원 조건을 너무 직접 연다.
- 보류: `wuxia_mumyeong_resolution`. 최종장 무명 결산과 후일담 routing 범위다.
- 보류: `wuxia_boss_resolution`. `wuxia_sado_final_battle` 이후 결산 카드이며 final boss/faction/epilogue 범위다.
- 보류: `wuxia_seoharin_empty_place`. 초반 서하린/무명 감정선 카드라 post-Qingliu trace 위치에서는 되돌아가는 side beat가 된다.

다음 runtime 구현 계약:

- encounter id: `wuxia_mumyeong_destroys_orthodox_sect`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_qingliu_attack_after_war`
- location: `cheongryu_outer_courtyard`
- required flags: `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`
- forbidden flags: `mumyeong_destroys_orthodox_sect_resolved`
- flavor-only flags/clues: `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `salvation_truth_still_unready`
- stable choice ids: `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`
- common outcome hooks: `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `hyeonakmun_was_destroyed_after_qingliu_attack`, `mumyeong_destroyed_hyeonakmun_alone`, `destruction_is_consequence_not_salvation`, `seoharin_truth_delivery_still_unopened`, `boss_recruitment_still_unopened`
- presentation: `visual_id: wuxia_mumyeong_destroys_orthodox_sect`, `speaker: 천기록`, `layout: hyeonakmun_empty_gate_record`, stable terms `[현악문, 복호금쇄수, 무명]`

non-goals:

- 현악문 멸문 전투 playable combat/full flashback
- 청류문 습격 full flashback/backstory reveal
- 서하린에게 진실 전달, 무명 구원 확정, `wuxia_mumyeong_departure_truth_summary` 구현
- `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`
- boss combat/final boss resolution
- 공동파 본산 직접 악역 확정
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated artifact 변경은 이 handoff commit에서 하지 않는다

## 0.47 2026-06-02 무협 `wuxia_mumyeong_destroys_orthodox_sect` preview runtime slice

현재 상태: 구현 완료. `wuxia_qingliu_attack_after_war` 뒤에 빈 현악문 산문/기록/풍문을 확인하는 limited consequence trace를 추가했다. 이 slice는 현악문 멸문 전투, full flashback, 서하린 truth delivery, 보스 스카웃, reward/ability/faction/relation/debt/epilogue/return schema를 열지 않고 기존 encounter schema와 flags/clues/log/presentation만 사용한다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_qingliu_attack_after_war` 뒤에 `wuxia_mumyeong_destroys_orthodox_sect`를 추가했다.
- location은 `cheongryu_outer_courtyard`, required flags는 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`로 고정했다.
- `mumyeong_destroys_orthodox_sect_resolved` forbidden flag로 반복을 막는다.
- stable choice id는 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`다.
- 모든 outcome은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues는 `hyeonakmun_was_destroyed_after_qingliu_attack`, `mumyeong_destroyed_hyeonakmun_alone`, `destruction_is_consequence_not_salvation`, `seoharin_truth_delivery_still_unopened`, `boss_recruitment_still_unopened`다.
- presentation은 `visual_id: wuxia_mumyeong_destroys_orthodox_sect`, `speaker: 천기록`, `layout: hyeonakmun_empty_gate_record`, stable terms `[현악문, 복호금쇄수, 무명]`로 구현했다.
- Rust/Web storypack preview generated bundle을 재생성했다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, SuperLightTUI smoke, Web default content bundle registry 테스트를 갱신했다.
- 기본 storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 유지한다. legacy office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- 현악문 멸문 전투 playable combat/full flashback
- 청류문 습격 full flashback/backstory reveal
- 서하린에게 진실 전달, 무명 구원 확정, `wuxia_mumyeong_departure_truth_summary` 구현
- `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`
- boss combat/final boss resolution
- 공동파 본산 직접 악역 확정
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경

다음은 `wuxia_mumyeong_destroys_orthodox_sect_followup` docs-only handoff다. 다음 handoff는 최소 `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`를 Notion 사건 카드 DB와 repo hooks로 다시 비교하고, 보스 스카웃/무명 이탈 진실/최종장/서하린 side beat 중 어느 것을 먼저 runtime으로 승격할지 결정한다.

## 0.48 2026-06-02 docs-only Hyeonakmun consequence follow-up handoff: `wuxia_boss_recruits_mumyeong`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_destroys_orthodox_sect` 뒤의 후보를 Notion 사건 카드 DB와 repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_boss_recruits_mumyeong`로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

확인한 Notion source:

- `09. 이구학지 사건 카드 DB`: `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`
- 운영 문서: `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`

결정:

- 선택: `wuxia_boss_recruits_mumyeong`. Notion prerequisite가 정파 문파 멸문 이후이고, repo의 `mumyeong_destroys_orthodox_sect_resolved` / `hyeonakmun_destruction_thread_opened` / `departure_truth_thread_deepened` 및 기존 `boss_first_appearance_resolved` hook과 직접 맞물린다.
- 제한: 다음 runtime 구현은 보스가 무명의 상처와 힘을 보고 흑사방으로 끌어들이는 **boss recruitment trace**로만 다룬다. 구원이나 최종 결산이 아니다.
- 보류: `wuxia_mumyeong_departure_truth_summary`. 후반 truth summary이며 서하린에게 진실 전달과 무명 구원 조건을 직접 연다.
- 보류: `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`. 최종장/후일담 routing과 final boss resolution 범위다.
- 보류: `wuxia_seoharin_empty_place`. 초반 companion/epilogue beat라 현재 현악문 consequence 직후 위치에서는 되돌아가는 side beat가 된다.

다음 runtime 구현 계약:

- encounter id: `wuxia_boss_recruits_mumyeong`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_destroys_orthodox_sect`
- location: `cheongryu_outer_courtyard`
- required flags: `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`
- forbidden flags: `boss_recruits_mumyeong_resolved`
- stable choice ids: `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`
- common outcome hooks: `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `boss_recruited_mumyeong_after_hyeonakmun`, `recruitment_was_not_salvation`, `mumyeong_had_nowhere_to_stand_after_destruction`, `boss_turned_wound_into_power`, `black_serpent_recruits_wounds_not_names`, `boss_reads_people_not_forms`
- presentation: `visual_id: wuxia_boss_recruits_mumyeong`, `speaker: 천기록`, `layout: boss_recruitment_trace`, stable terms `[흑사방주, 무명, 현악문]`

non-goals:

- 무명 이탈 진실 전체 reveal, 서하린에게 진실 전달, 무명 구원 확정
- 보스 전투, final boss resolution, 무명/보스 결산
- epilogue/return system
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated artifact 변경은 이 handoff commit에서 하지 않는다

## 0.49 2026-06-02 무협 `wuxia_boss_recruits_mumyeong` preview runtime slice

현재 상태: 구현 완료. `wuxia_mumyeong_destroys_orthodox_sect` 뒤에 흑사방 보스가 무명의 상처와 힘을 읽고 스카웃하는 recruitment trace를 추가했다. 이 slice는 무명 이탈 진실 전달, 서하린 truth delivery, 무명 구원, 보스 전투/최종 결산, reward/ability/faction/relation/debt/epilogue/return schema를 열지 않고 기존 encounter schema와 flags/clues/log/presentation만 사용한다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_destroys_orthodox_sect` 뒤에 `wuxia_boss_recruits_mumyeong`를 추가했다.
- location은 `cheongryu_outer_courtyard`, required flags는 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`로 고정했다.
- `boss_recruits_mumyeong_resolved` forbidden flag로 반복을 막는다.
- stable choice id는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`다.
- 모든 outcome은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues는 `boss_recruited_mumyeong_after_hyeonakmun`, `recruitment_was_not_salvation`, `mumyeong_had_nowhere_to_stand_after_destruction`, `boss_turned_wound_into_power`, `black_serpent_recruits_wounds_not_names`, `boss_reads_people_not_forms`다.
- presentation은 `visual_id: wuxia_boss_recruits_mumyeong`, `speaker: 천기록`, `layout: boss_recruitment_trace`, stable terms `[흑사방주, 무명, 현악문]`로 구현했다.
- Rust/Web storypack preview generated bundle을 재생성했다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, SuperLightTUI smoke, Web default content bundle registry 테스트를 갱신했다.
- 기본 storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 유지한다. legacy office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- 무명 이탈 진실 전체 reveal, 서하린에게 진실 전달, 무명 구원 확정
- 보스 전투, final boss resolution, 무명/보스 결산
- epilogue/return system
- reward/ability schema, seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경

이후 후속 결정은 아래 `0.50` docs-only handoff에서 완료했다.

## 0.50 2026-06-02 docs-only boss recruitment follow-up handoff: `wuxia_mumyeong_departure_truth_summary`

현재 상태: docs-only handoff 완료. `wuxia_boss_recruits_mumyeong` 뒤의 후보를 Notion 사건 카드 DB와 repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_mumyeong_departure_truth_summary`로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

확인한 Notion source:

- `09. 이구학지 사건 카드 DB`: `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, 기존 late/final 후보
- 운영 문서: `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`

결정:

- 선택: `wuxia_mumyeong_departure_truth_summary`. Notion related events가 `wuxia_mumyeong_request_for_aid`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_awakening`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`까지 이어지고, repo의 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_saw_mumyeongs_wound`, `departure_truth_thread_deepened`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened` hook을 가장 직접 받는다.
- 제한: 다음 runtime 구현은 protagonist/천기록이 무명 이탈의 진실을 **sealed departure truth summary**로 정리해 보관하는 trace다. 진실을 이해할 수는 있지만 아직 서하린에게 전하지 않는다.
- 보류: `wuxia_mumyeong_resolution`. 무명 구원/부분 구원/비구원과 후일담 routing을 결정하는 최종장 카드다.
- 보류: `wuxia_boss_resolution`. `wuxia_sado_final_battle` 이후 보스 생존/패배, 흑사방 network, 무명 successor, epilogue matrix까지 여는 최종 결산 카드다.
- 보류: `wuxia_seoharin_empty_place`. 초반 companion/감정선 beat라 boss recruitment 직후 stage에서는 되돌아가는 side beat가 된다.

다음 runtime 구현 계약:

- encounter id: `wuxia_mumyeong_departure_truth_summary`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_boss_recruits_mumyeong`
- location: `cheongryu_outer_courtyard`
- required flags: `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`
- forbidden flags: `mumyeong_departure_truth_summary_resolved`
- stable choice ids: `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`
- common outcome hooks: `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `departure_truth_can_be_understood_but_not_spoken_yet`, `seoharin_truth_delivery_requires_later_consent`, `boss_used_mumyeongs_wound_after_orthodox_refusal`, `salvation_condition_seen_but_unmet`
- presentation: `visual_id: wuxia_mumyeong_departure_truth_summary`, `speaker: 천기록`, `layout: sealed_departure_truth_summary`, stable terms `[무명, 서하린, 현악문, 흑사방주]`

non-goals:

- 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 조건 만족
- `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, 사도 최종전, epilogue/return system
- route graph, faction reputation, relation/debt ledger, reward/ability schema
- combat resolver/schema 또는 HP 숫자전
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated artifact 변경은 이 handoff commit에서 하지 않는다

## 0.51 2026-06-02 무협 `wuxia_mumyeong_departure_truth_summary` preview runtime slice

현재 상태: 구현 완료. `wuxia_boss_recruits_mumyeong` 뒤에 무명 이탈 진실을 봉인된 요약으로 정리하는 sealed summary trace를 추가했다. 이 slice는 서하린에게 진실을 전달하거나, `told_seoharin_truth`를 추가하거나, 무명 구원/보스 결산/final/epilogue schema를 열지 않고 기존 encounter schema와 flags/clues/log/presentation만 사용한다.

구현 내용:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_recruits_mumyeong` 뒤에 `wuxia_mumyeong_departure_truth_summary`를 추가했다.
- location은 `cheongryu_outer_courtyard`, required flags는 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`로 고정했다.
- `mumyeong_departure_truth_summary_resolved` forbidden flag로 반복을 막는다.
- stable choice id는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`다.
- 모든 outcome은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- primary clues는 `departure_truth_can_be_understood_but_not_spoken_yet`, `seoharin_truth_delivery_requires_later_consent`, `boss_used_mumyeongs_wound_after_orthodox_refusal`, `salvation_condition_seen_but_unmet`다.
- presentation은 `visual_id: wuxia_mumyeong_departure_truth_summary`, `speaker: 천기록`, `layout: sealed_departure_truth_summary`, stable terms `[무명, 서하린, 현악문, 흑사방주]`로 구현했다.
- Rust/Web storypack preview generated bundle을 재생성했다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, SuperLightTUI smoke, Web default content bundle registry 테스트를 갱신했다.
- 기본 storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 유지한다. legacy office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.

non-goals:

- 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 확정
- `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, 사도 최종전, epilogue/return system
- route graph, faction reputation, relation/debt ledger, reward/ability schema
- combat resolver/schema 또는 HP 숫자전
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경

다음은 `wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff다. 다음 handoff는 `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, 남은 late truth/final 후보를 Notion 사건 카드 DB와 repo hooks로 다시 비교하고, truth delivery/salvation/final/epilogue를 열기 전 경계를 문서에서 먼저 결정한다.

## 0.52 2026-06-02 docs-only departure-truth-summary follow-up handoff: `wuxia_seoharin_empty_place`

현재 상태: docs-only handoff 완료. `wuxia_mumyeong_departure_truth_summary` 뒤의 후보를 Notion 사건 카드 DB, 최신 사도 최종전/결산 문서, repo hooks로 다시 대조해, 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정했다. 이 handoff에서는 runtime YAML/Rust/Web generated artifact와 legacy office bundle을 변경하지 않았다.

확인한 Notion source:

- `09. 이구학지 사건 카드 DB`: `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_departure_truth_summary`
- 운영 문서: `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`
- late/final source: `wuxia_sado_final_battle`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `최종장 결산 라우팅 마스터`

결정:

- 선택: `wuxia_seoharin_empty_place`. Notion 원본은 초반 companion/무명 clue card지만, 최신 사도 최종전 2/3페이즈와 최종장 결산 라우팅 마스터가 `비워둔 자리` 확인을 `seoharin_axis`, `remembered_empty_place`, `item_unpriced_wooden_sword` 조건으로 재사용한다. 따라서 sealed departure truth summary 직후, 서하린의 기다림을 소유가 아니라 돌아올 자리로 이해하는 late emotional bridge로 구현하는 것이 지금 schema에서 가장 작고 의미 있는 다음 slice다.
- 보류: `wuxia_mumyeong_resolution`. Notion 본문이 `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `truth_state`, `player_method`, 후일담 enable/suppress matrix를 요구하는 최종장 결산 카드다.
- 보류: `wuxia_boss_resolution`. `wuxia_sado_final_battle` 이후 `combat_result`, `network_handling`, `evidence_state`, `pressure_state`, `item_logs`, final epilogue matrix를 소비하는 보스 결산 카드다.
- 보류: `wuxia_sado_final_battle` 및 phase 2/3 카드. 최종전은 필수 전투, 다중 phase, canonical final input state glossary를 요구하므로 이번 slice에서 바로 열지 않는다.

다음 runtime 구현 계약:

- encounter id: `wuxia_seoharin_empty_place`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_departure_truth_summary`
- location: `cheongryu_outer_courtyard`
- required flags: `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`
- forbidden flags: `seoharin_empty_place_resolved`
- stable choice ids: `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`
- common outcome hooks: `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `seoharin_remembers_without_possessing`, `empty_place_is_return_not_claim`, `mumyeong_place_still_unclaimed`, `unpriced_wooden_sword_condition_seeded`
- presentation: `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`

non-goals:

- 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 조건 만족
- `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, 사도 최종전, final battle
- `item_unpriced_wooden_sword` 실제 item 지급
- final/epilogue/return system
- route graph, faction reputation, relation/debt ledger, reward/ability schema
- combat resolver/schema 또는 HP 숫자전
- 천기록 identity reveal
- legacy office bundle/default bundle, legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated artifact 변경은 이 handoff commit에서 하지 않는다

## 0.53 2026-06-02 무협 `wuxia_seoharin_empty_place` preview runtime slice

현재 상태: preview runtime 구현 완료. `wuxia_mumyeong_departure_truth_summary` 뒤에 `wuxia_seoharin_empty_place`를 추가해 sealed truth summary 이후 서하린의 비워둔 자리를 late emotional bridge로 landing했다.

구현 범위:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- generated artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- required flags: `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`
- forbidden flags: `seoharin_empty_place_resolved`
- stable choice ids: `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`
- common outcome hooks: `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `seoharin_remembers_without_possessing`, `empty_place_is_return_not_claim`, `mumyeong_place_still_unclaimed`, `unpriced_wooden_sword_condition_seeded`, `truth_delivery_still_requires_consent`
- presentation: `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`

검증/경계:

- Rust/Web generated preview bundle과 Web default storypack registry에 반영했다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트를 갱신한다.
- `item_unpriced_wooden_sword`는 지급하지 않고 `unpriced_wooden_sword_condition_seeded` clue로만 남긴다.
- 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, 무명/보스 결산, 사도 최종전, final/epilogue/return schema, combat resolver/schema, route graph/faction reputation/relation/debt/reward schema, 천기록 identity reveal은 열지 않는다.

다음은 `wuxia_seoharin_empty_place_followup` docs-only handoff다. 다음 handoff는 최소 `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_sado_final_battle`, `wuxia_seoharin_unsaid_stay`, `wuxia_seoharin_left_meal`, 남은 final/late companion 후보를 Notion 사건 카드 DB와 최종장 라우팅 문서로 다시 비교한다.

## 0.54 2026-06-02 docs-only Seo Harin empty-place follow-up handoff: `wuxia_seoharin_left_meal`

현재 상태: docs-only handoff 완료. `wuxia_seoharin_empty_place` 뒤의 후보를 Notion 사건 카드 DB와 사도 최종전/무명 결산 문서로 다시 대조해, 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정했다.

대조한 Notion 원문:

- `남겨둔 밥`: 선행 조건은 "서하린 관계축 최소 개방", 보상은 서하린 관계축/청류문 소속감 상승이다. 현재 runtime의 `seoharin_axis_opened`/`empty_place_remembered`와 직접 맞는다.
- `가지 말라는 말`: 귀환 가능성/세력 선택 분기 접근, 서하린 관계축 중 이상, 귀환/정착/침식 후일담 변주를 여는 후반 final relationship trigger다.
- `무명 결산`: 무명 구원/미완 구원/비구원과 후일담 라우팅 매트릭스를 여는 최종장 사건이다.
- `사도 최종전 3페이즈`: `seoharin_axis`, `mumyeong_salvation`, `qingliu_rebuild`, `item_unpriced_wooden_sword`, `cheongirok_state`, `player_method` 같은 상태값을 요구하는 final battle phase다.

선택/보류:

- 선택: `wuxia_seoharin_left_meal`. 이미 열린 서하린 관계축을 daily-care/청류문 소속감 bridge로 낮게 이어 붙일 수 있고, 새 relation/reward/return schema 없이 flags/clues/log/presentation만으로 구현 가능하다.
- 보류: `wuxia_seoharin_unsaid_stay`. 귀환/정착/침식 최종 관계 분기를 열기 때문에 아직 이르다.
- 보류: `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_sado_final_battle`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`. final/epilogue/combat/reward/item-log/relation-state schema를 요구한다.

다음 runtime implementation 계약:

- encounter id: `wuxia_seoharin_left_meal`
- insert_after: `wuxia_seoharin_empty_place`
- required flags: `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`
- forbidden flags: `seoharin_left_meal_resolved`
- stable choice ids: `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`
- common guardrails: no Seo Harin truth delivery, no `told_seoharin_truth`, no final return/settlement choice, no Mumyeong/Boss resolution, no Sado final battle, no epilogue/return schema, no reward/relation schema, no `item_unpriced_wooden_sword` payout, no 천기록 identity reveal.

## 0.55 2026-06-02 무협 `wuxia_seoharin_left_meal` preview runtime slice

현재 상태: preview runtime 구현 완료. `wuxia_seoharin_empty_place` 뒤에 `wuxia_seoharin_left_meal`을 추가해, 비워둔 자리로 열린 서하린 관계축을 청류문 daily-care/belonging bridge로 landing했다.

구현 범위:

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- generated artifacts: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- required flags: `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`
- forbidden flags: `seoharin_left_meal_resolved`
- stable choice ids: `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`
- positive common hooks: `seoharin_left_meal_resolved`, `seoharin_axis_deepened`, `qingliu_belonging_warmed`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`
- refusal hooks: `seoharin_left_meal_resolved`, `seoharin_axis_still_open`, `left_meal_left_untouched`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`
- primary clues: `left_meal_was_kept_for_return`, `belonging_is_daily_care`, `seoharin_care_named_without_claim`, `seoharin_deflects_care_with_plain_words`, `last_bowl_epilogue_seeded`, `belonging_can_be_refused`
- presentation: `visual_id: wuxia_seoharin_left_meal`, `speaker: 서하린`, `layout: left_meal_memory`, stable terms `[서하린, 밥그릇, 청류문, 귀환]`

검증/경계:

- Rust/Web generated preview bundle과 Web default storypack registry에 반영했다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트를 갱신한다.
- `wuxia_seoharin_unsaid_stay`는 final return/settlement relationship branch라 보류한다.
- 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, 무명/보스 결산, 사도 최종전, final/epilogue/return schema, combat resolver/schema, route graph/faction reputation/relation/debt/reward schema, 천기록 identity reveal은 열지 않는다.

다음 `wuxia_seoharin_left_meal_followup` docs-only handoff는 완료됐다. 후속 최종장 후보들은 바로 runtime으로 열기 전에 final-route state dictionary와 result routing contract가 필요하므로 `docs/design/Wuxia_Final_State_Routing.md`를 먼저 추가했다.

## 0.56 2026-06-02 docs-only Seo Harin left-meal follow-up handoff: final state routing contract

현재 상태: docs-only handoff + design contract 완료. `wuxia_seoharin_left_meal` 뒤의 late companion/final 후보를 Notion 원문과 대조한 결과, 다음 runtime을 바로 구현하지 않고 최종장 상태값/결산 라우팅 contract를 먼저 고정하기로 했다.

대조한 Notion 원문:

- `가지 말라는 말`: 귀환 가능성/세력 선택 분기, 서하린 관계축 중 이상, 귀환/정착/침식 관계 후일담을 여는 final relationship trigger다.
- `무명 결산`: `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `truth_state`, `seoharin_axis`, `qingliu_rebuild`, `cheongirok_state`, `player_method`를 소비하는 최종장 결산이다.
- `보스 결산`: `wuxia_sado_final_battle` 이후 `combat_result`, `boss_resolution_route`, `network_handling`, `evidence_state`, `pressure_state`, `item_logs`와 final epilogue matrix를 소비한다.
- `사도 최종전`: 필수 최종전 컨테이너이며 1페이즈 가격표, 2페이즈 약점 통제, 3페이즈 계산식 밖으로 나뉜다.
- `최종장 결산 라우팅 마스터`: `final_result_priority`, `final_epilogue_master_matrix`, conflict rules를 소유한다.
- `사도 최종전 상태값 사전`: canonical final inputs, allowed values, alias/deprecation policy를 소유한다.

선택/보류:

- 선택: `docs/design/Wuxia_Final_State_Routing.md`. 최종전/결산/서하린 late branch가 같은 상태 언어를 쓰도록 `canonical_final_inputs`, `final_result_priority`, `final_epilogue_master_matrix` handoff boundary를 먼저 문서화한다.
- 다음 runtime 후보: `wuxia_sado_final_phase_1_price_tag`. 이 후보는 ledger/evidence/pressure/item log seed를 기존 encounter schema로 남길 수 있어, contract 이후 첫 final-entry slice로 가장 작다.
- 보류: `wuxia_seoharin_unsaid_stay`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_sado_final_battle`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`. final/epilogue/combat/reward/item-log/relation-state schema가 아직 열리지 않았다.

새 design contract:

- doc: `docs/design/Wuxia_Final_State_Routing.md`
- canonical states: `combat_result`, `boss_resolution_route`, `evidence_state`, `network_handling`, `pressure_state`, `seoharin_axis`, `qingliu_rebuild`, `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `truth_state`, `cheongirok_state`, `player_method`, `item_logs`
- priority: `battle_loss`, `corrupted_victory`, `true_route_victory`, `mumyeong_unsaved_victory`, `meaningful_victory`, `incomplete_victory`, `basic_victory`
- alias policy: `item_log_state`는 local helper only이며, `companion_state`, `organization_state`, `black_serpent_new_scale`, `successor_logic`, `route_pressure`, `unpriced_wooden_sword_condition`, `closed_gate_risk`, `alliance_silence_variant`는 새 docs/runtime handoff에서 쓰지 않는다.

계속 닫아 둘 범위:

- 서하린 truth delivery, `told_seoharin_truth`
- 무명 구원 확정, 무명/보스 결산
- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, `item_unpriced_wooden_sword` payout
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

## 0.57 2026-06-02 무협 `wuxia_sado_final_phase_1_price_tag` preview runtime slice

현재 상태: preview runtime 구현 완료. `docs/design/Wuxia_Final_State_Routing.md`가 고정한 final state routing contract를 바탕으로, `사도 최종전 1페이즈: 가격표`를 기존 encounter schema만으로 landing했다.

대조한 Notion 원문:

- `사도 최종전 1페이즈: 가격표`: 흑사방 장부고에서 직접 접근, 장부 파괴, 장부 확보, 인질성 압박 해소 중 무엇을 택했는지 기록한다. 핵심 업데이트는 `network_handling`, `evidence_state`, `pressure_state`, `item_logs`다.
- `사도 최종전`: 최종전 컨테이너이며 phase 1/2/3과 보스 결산으로 이어진다. 이 slice에서는 컨테이너 전체나 combat resolver를 열지 않는다.
- `사도 최종전 2페이즈: 약점 장악`, `사도 최종전 3페이즈: 계산식 밖`: phase 1 결과를 소비하는 future handoff로 남긴다.
- `보스 결산`, `무명 결산`, `가지 말라는 말`: final/epilogue/relationship schema를 요구하므로 계속 보류한다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`에 `black_serpent_ledger_vault`를 추가하고 `cheongryu_outer_courtyard`와 연결했다.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_seoharin_left_meal` 뒤에 `wuxia_sado_final_phase_1_price_tag`를 추가했다.
- required flags는 `seoharin_left_meal_resolved`, `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_first_appearance_resolved`, `black_serpent_core_pressure_opened`, `sealed_departure_truth_summary_prepared`, `midgame_continuity_started`다.
- stable choice id는 `approach_sado_before_the_ledger`, `burn_the_blackscale_ledger`, `secure_the_blackscale_ledger`, `ease_hostage_pressure_first`다.
- common hook은 `sado_final_phase_1_price_tag_resolved`, `final_state_routing_seeded`, `destination_id: black_serpent_ledger_vault`다.
- direct approach는 `final_network_ignored_seeded`, `final_evidence_none_or_low_seeded`, `final_pressure_unresolved_seeded`를 남긴다.
- ledger burn은 `final_network_partially_destroyed_seeded`, `final_evidence_partial_seeded`, `final_pressure_partially_eased_seeded`를 남긴다.
- ledger secure는 item 지급 없이 `final_network_ledger_secured_seeded`, `final_evidence_strong_seeded`, `final_item_logs_blackscale_ledger_seeded`, `item_blackscale_ledger_logged`를 남긴다.
- pressure relief는 `final_pressure_eased_seeded`, `final_qingliu_rebuild_partial_seeded`, `final_seoharin_axis_high_seeded`를 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- `wuxia_sado_final_battle` 전체 컨테이너와 phase 3 runtime 구현
- combat resolver, HP numeric battle
- final epilogue renderer, return/settlement schema
- 서하린 truth delivery, `told_seoharin_truth`
- 무명 구원 확정, 무명/보스 결산
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, `item_unpriced_wooden_sword` payout
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

## 0.58 2026-06-02 무협 `wuxia_sado_final_phase_2_weakpoint_control` preview runtime slice

현재 상태: preview runtime 구현 완료. `사도 최종전 2페이즈: 약점 장악`을 기존 encounter schema만으로 landing했다. 이 slice는 사도 디버프나 numeric battle이 아니라, 플레이어가 서하린/무명/청류문/천기록 압박을 어떤 방식으로 다루는지 final-state seed로 남긴다.

대조한 Notion 원문:

- `사도 최종전 2페이즈: 약점 장악`: 관계축은 사도 약화가 아니라 플레이어 측 대응 선택지와 후일담 조건으로 작동한다. 핵심 업데이트는 `seoharin_axis`, `qingliu_rebuild`, `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `cheongirok_state`, `player_method`다.
- `사도 최종전 3페이즈: 계산식 밖`: phase 2 결과를 소비해 최종 선택지와 승리 계열을 확정하는 future handoff로 남긴다.
- `사도 최종전`: phase 2는 전체 battle container의 일부지만, 이번 slice에서는 combat resolver나 final battle result를 열지 않는다.
- `보스 결산`: phase 2 seed를 소비하는 결산 카드이므로 계속 보류한다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_1_price_tag` 뒤에 `wuxia_sado_final_phase_2_weakpoint_control`을 추가했다.
- required flags는 `sado_final_phase_1_price_tag_resolved`, `final_state_routing_seeded`다.
- stable choice id는 `respond_to_seoharin_pressure`, `return_flow_to_mumyeong`, `read_dangerous_cheongirok_sentence`, `focus_on_sado`다.
- common hook은 `sado_final_phase_2_weakpoint_control_resolved`, `final_phase_2_weakpoint_control_resolved`, `destination_id: black_serpent_ledger_vault`다.
- 서하린 대응은 `final_seoharin_axis_high_seeded`, `final_qingliu_rebuild_partial_seeded`, `final_player_method_outside_calculation_seeded`를 남긴다.
- 무명 대응은 `final_mumyeong_salvation_partial_seeded`, `final_successor_route_suppressed_seeded`, `final_own_flow_choice_opened_seeded`, `final_player_method_protected_as_person_seeded`를 남긴다.
- 천기록 위험 문장은 `final_cheongirok_state_high_use_seeded`, `final_cheongirok_corruption_risk_seeded`, `final_evidence_partial_or_strong_seeded`, `final_player_method_used_as_tool_risk_seeded`를 남긴다.
- 사도 집중은 `final_player_method_direct_boss_focus_seeded`, `final_relationship_pressure_unspent_seeded`, `final_network_residue_possible_seeded`를 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- `wuxia_sado_final_battle` 전체 컨테이너와 phase 3 runtime 구현
- combat resolver, HP numeric battle
- final epilogue renderer, return/settlement schema
- 서하린 truth delivery, `told_seoharin_truth`
- 무명 구원 확정, 무명/보스 결산
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, `item_unpriced_wooden_sword` payout
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_sado_final_phase_3_outside_calculation`이다. Phase 3는 `combat_result`/`boss_resolution_route`와 최종 결과 계열을 건드리므로, 구현 전 Notion source와 repo contract를 다시 대조한다.

## 0.59 2026-06-02 무협 `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice

현재 상태: preview runtime 구현 완료. `사도 최종전 3페이즈: 계산식 밖`을 기존 encounter schema만으로 landing했다. 이 slice는 사도를 약화시키거나 final epilogue를 출력하지 않고, phase 1/2 seed 뒤에 보스 결산으로 넘길 `combat_result`와 `boss_resolution_route` 후보를 flags/clues/log로 남긴다.

대조한 Notion 원문:

- `사도 최종전 3페이즈: 계산식 밖`: 사도가 약해지는 것이 아니라 플레이어가 관계/증거/천기록 상태 덕분에 더 많은 방식으로 버틴다. 선택지는 `비워둔 자리를 떠올린다`, `무명에게 자기 흐름을 선택하게 한다`, `청류문의 뜻으로 버틴다`, `장부의 빈칸을 지적한다`, `사도식 계산으로 맞선다`다.
- `사도 최종전`: phase 3는 전체 battle container의 세 번째 phase지만, 사도는 대화로 약화되지 않고 combat resolver/HP 숫자전은 열지 않는다.
- `보스 결산`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`: phase 3가 넘긴 후보 state를 소비하는 다음 handoff로 남긴다. 실제 후일담 출력/억제 판정은 아직 구현하지 않는다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_2_weakpoint_control` 뒤에 `wuxia_sado_final_phase_3_outside_calculation`을 추가했다.
- required flags는 `sado_final_phase_2_weakpoint_control_resolved`, `final_phase_2_weakpoint_control_resolved`, `final_state_routing_seeded`다.
- stable choice id는 `remember_the_empty_place`, `let_mumyeong_choose_own_flow`, `endure_with_qingliu_will`, `point_to_blank_in_ledger`, `answer_with_sado_calculation`다.
- common hook은 `sado_final_phase_3_outside_calculation_resolved`, `final_phase_3_outside_calculation_resolved`, `final_combat_result_battle_victory_seeded`, `destination_id: black_serpent_ledger_vault`다.
- 비워둔 자리 선택은 `final_boss_resolution_true_route_candidate_seeded`, `final_seoharin_axis_high_preserved_seeded`, `final_unpriced_wooden_sword_condition_raised_seeded`, `final_player_method_outside_calculation_confirmed_seeded`를 남긴다.
- 무명 선택은 `final_boss_resolution_true_or_meaningful_candidate_seeded`, `final_mumyeong_salvation_substantial_candidate_seeded`, `final_successor_route_suppressed_confirmed_seeded`, `final_own_flow_choice_chosen_seeded`, `final_player_method_protected_as_person_confirmed_seeded`를 남긴다.
- 청류문 선택은 `final_boss_resolution_meaningful_candidate_seeded`, `final_qingliu_rebuild_high_candidate_seeded`, `final_pressure_state_eased_confirmed_seeded`, `final_evidence_strong_support_seeded`를 남긴다.
- 장부 빈칸 선택은 `final_boss_resolution_meaningful_or_true_candidate_seeded`, `final_item_logs_blank_ledger_seen_seeded`, `final_evidence_strong_confirmed_seeded`, `final_network_accountability_seeded`를 남긴다.
- 사도식 계산 선택은 `final_boss_resolution_corrupted_candidate_seeded`, `final_cheongirok_state_corruption_high_seeded`, `final_player_method_sado_style_calculation_seeded`, `final_successor_route_active_risk_seeded`를 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- `wuxia_sado_final_battle` 전체 컨테이너와 `wuxia_boss_resolution` 출력기
- combat resolver, HP numeric battle
- final epilogue renderer, return/settlement schema
- 서하린 truth delivery, `told_seoharin_truth`
- 무명 구원 확정, 무명 결산
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, `item_unpriced_wooden_sword` payout
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_boss_resolution`이다. Boss resolution은 phase 3가 남긴 candidate state를 소비해야 하므로, 구현 전 `최종장 결산 라우팅 마스터`와 `보스 결산`의 enable/suppress boundary를 다시 대조한다.

## 0.60 2026-06-02 무협 `wuxia_boss_resolution` preview runtime slice

현재 상태: preview runtime 구현 완료. Notion `보스 결산`과 `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`, 사도 최종전 container를 대조해 기존 encounter schema만으로 landing했다. 이 slice는 최종 후일담 출력기가 아니라 phase 3의 `combat_result`/`boss_resolution_route` 후보를 보스 결산 route와 후속 epilogue candidate seed로 정규화하는 bridge다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_3_outside_calculation` 뒤에 `wuxia_boss_resolution`을 추가했다.
- required flags는 `sado_final_phase_3_outside_calculation_resolved`, `final_phase_3_outside_calculation_resolved`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`다.
- forbidden flag는 `boss_resolution_resolved`다.
- stable choice id는 `confirm_true_route_outside_calculation`, `confirm_meaningful_victory_with_evidence`, `confirm_incomplete_victory_residue`, `confirm_mumyeong_unsaved_successor_risk`, `confirm_corrupted_victory`다.
- common hook은 `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `destination_id: black_serpent_ledger_vault`다.
- 선택지는 true/meaningful/incomplete/mumyeong-unsaved/corrupted route seed와 `final_epilogue_candidates_*_seeded` 계열 후보만 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 무명 구원 확정, 무명 결산
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, `item_unpriced_wooden_sword` payout
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_mumyeong_resolution`이다. Boss resolution이 남긴 `final_mumyeong_resolution_required_seeded` 및 Mumyeong route 후보를 소비하되, 구현 전 Notion `무명 결산`과 final routing contract를 다시 대조한다.

## 0.61 2026-06-02 무협 `wuxia_mumyeong_resolution` preview runtime slice

현재 상태: preview runtime 구현 완료. Notion `무명 결산`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`을 대조해 기존 encounter schema만으로 landing했다. 이 slice는 최종 후일담 출력기가 아니라 boss resolution이 남긴 무명 축 후보를 own-flow/apology/stolen-forms/successor/corruption epilogue candidate seed로 정규화하는 bridge다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_resolution` 뒤에 `wuxia_mumyeong_resolution`을 추가했다.
- required flags는 `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`다.
- forbidden flag는 `mumyeong_resolution_resolved`다.
- stable choice id는 `ask_mumyeong_for_own_flow`, `reveal_boss_used_mumyeongs_wound`, `leave_room_for_unsent_apology`, `let_stolen_forms_end`, `confirm_black_serpent_successor_risk`, `judge_with_sado_style_calculation`다.
- common hook은 `mumyeong_resolution_resolved`, `destination_id: black_serpent_ledger_vault`다.
- 선택지는 own-flow/relational/incomplete/end-of-stolen-forms/black-serpent-successor/corrupted-unsaved route seed와 후속 epilogue candidate seed만 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 실제 아이템 `item_unpriced_wooden_sword` payout
- route graph, faction reputation, relation/debt ledger
- reward/ability schema
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_seoharin_qingliu_resolution`이다. 무명 결산이 남긴 열린 문/닫힌 문/마지막 밥상/청류문 복구 후보를 소비하되, 구현 전 Notion의 서하린 late branch와 final routing contract를 다시 대조한다.

## 0.62 2026-06-02 무협 `wuxia_seoharin_qingliu_resolution` preview runtime slice

현재 상태: preview runtime 구현 완료. Notion `가지 말라는 말`, `서하린의 후일`, `청류문의 후일`, `닫히지 않은 산문`, `닫힌 산문`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`을 대조해 기존 encounter schema만으로 landing했다. 이 slice는 최종 후일담 출력기가 아니라 무명 결산 뒤 서하린/청류문 축 후보를 open-gate/empty-place/Qingliu future/weakened-pressure/closed-gate epilogue candidate seed로 정규화하는 bridge다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_resolution` 뒤에 `wuxia_seoharin_qingliu_resolution`을 추가했다.
- required flags는 `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`다.
- forbidden flag는 `seoharin_qingliu_resolution_resolved`다.
- stable choice id는 `leave_the_gate_unclosed`, `record_qingliu_rebuild_without_glory`, `keep_empty_place_for_return_or_absence`, `mark_qingliu_pressure_still_unresolved`, `close_the_gate_with_sado_logic`다.
- common hook은 `seoharin_qingliu_resolution_resolved`, `destination_id: black_serpent_ledger_vault`다.
- 선택지는 서하린 긍정 후일/열린 산문/비워둔 자리/청류문 안정·약화/닫힌 산문 후보 seed만 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 실제 아이템 `item_unpriced_wooden_sword` payout
- route graph, faction reputation, relation/debt ledger
- reward/ability schema
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_cheongirok_resolution`이다. 서하린/청류문 결산이 남긴 open-gate/closed-gate/Qingliu future 후보와 final routing contract의 `cheongirok_state`/`player_method` 조건을 소비하되, 구현 전 Notion `천기록의 마지막 장`, `08. 엔딩과 후일담 연결`, final routing contract를 다시 대조한다.

## 0.63 2026-06-02 무협 `wuxia_cheongirok_resolution` preview runtime slice

현재 상태: preview runtime 구현 완료. Notion `천기록의 마지막 장`, `07. 천기록 / 천외편린 보상`, `08. 엔딩과 후일담 연결`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`을 대조해 기존 encounter schema만으로 landing했다. 이 slice는 최종 후일담 출력기나 천기록 정체 reveal이 아니라 서하린/청류문 결산 뒤 Cheonggi Record 사용 방식과 마지막 장 후보를 safe/blank/corruption/low-use epilogue candidate seed로 정규화하는 bridge다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_seoharin_qingliu_resolution` 뒤에 `wuxia_cheongirok_resolution`을 추가했다.
- required flags는 `seoharin_qingliu_resolution_resolved`, `boss_resolution_resolved`, `mumyeong_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`다.
- forbidden flag는 `cheongirok_resolution_resolved`다.
- stable choice id는 `turn_the_last_page_without_question`, `leave_blank_as_unpriced_place`, `read_the_lines_that_align_like_ledger`, `close_record_before_it_becomes_answer`, `let_record_reflect_the_method`다.
- common hook은 `cheongirok_resolution_resolved`, `destination_id: black_serpent_ledger_vault`다.
- 선택지는 high-use-not-corruption/safe last page, blank true-route place, ledger-aligned/corruption, low-use silence, player-method reflection seed만 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 실제 아이템 `item_unpriced_wooden_sword` payout
- route graph, faction reputation, relation/debt ledger
- reward/ability schema
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_black_serpent_aftermath`이다. 천기록 마지막 장이 남긴 final epilogue candidate seed를 소비하되, 구현 전 Notion final routing contract와 repo contract를 다시 대조한다.

## 0.64 2026-06-02 무협 `wuxia_black_serpent_aftermath` preview runtime slice

현재 상태: preview runtime 구현 완료. Notion `08. 엔딩과 후일담 연결`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`을 대조해 기존 encounter schema만으로 landing했다. 이 slice는 최종 후일담 출력기가 아니라 천기록 결산 뒤 흑사방 축의 부러진 검은 뱀, 깃발 잔존, 남쪽 장터 빚 풍문, 무림맹 침묵, true-route 억제 후보를 flags/clues/log seed로 정규화하는 bridge다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_cheongirok_resolution` 뒤에 `wuxia_black_serpent_aftermath`를 추가했다.
- required flags는 `cheongirok_resolution_resolved`, `seoharin_qingliu_resolution_resolved`, `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`다.
- forbidden flag는 `black_serpent_aftermath_resolved`다.
- stable choice id는 `mark_broken_serpent_without_erasing_scars`, `fold_the_banner_without_calling_it_gone`, `send_ledger_to_alliance_and_watch_silence`, `listen_for_southern_market_debt_rumor`, `let_true_route_suppress_the_banner`다.
- common hook은 `black_serpent_aftermath_resolved`, `destination_id: black_serpent_ledger_vault`다.
- 선택지는 broken serpent/banner residue/alliance silence/southern market rumor/true-route suppression seed만 남긴다.
- Rust/Web generated storypack preview bundle을 재생성했다.

계속 닫아 둘 범위:

- final epilogue renderer, return/settlement schema
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 실제 아이템 `item_unpriced_wooden_sword` payout
- route graph, faction reputation, relation/debt ledger
- reward/ability schema
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 handoff 후보는 `wuxia_final_epilogue_renderer_contract`이다. 모든 final-entry seed bridge가 후보를 남긴 뒤, 실제 후일담 카드를 렌더링할 계약과 renderer/core boundary를 별도로 설계해야 한다.

## 0.65 2026-06-02 docs-only final epilogue renderer contract handoff

현재 상태: docs-only handoff 완료, runtime YAML/Rust/Web/generated artifact 미수정. Notion `최종장 결산 라우팅 마스터`, `08. 엔딩과 후일담 연결`, `사도 최종전 상태값 사전`을 다시 대조했고, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`가 남긴 seed로 최종 후일담 계약을 열 수 있다고 결정했다.

결정:

- 추가 seed bridge 없이 `wuxia_final_epilogue_renderer_contract` 구현 slice를 다음 후보로 둔다.
- 이 slice는 Web/terminal UI가 아니라 Rust GameCore가 seed consumption, `final_result_priority`, suppress, card ordering을 소유하는 계약이어야 한다.
- Web Storybook과 SuperLightTUI는 core가 제공한 `ScenePage`/structured body를 표시하고 action id만 전달한다. 후일담 카드 enable/suppress를 renderer에서 다시 계산하지 않는다.
- 첫 contract는 boss/흑사방, 무명, 서하린/청류문, 천기록 candidate group을 소비하되, return/settlement, reward/ability, relation/debt/faction ledger, combat-resource system은 열지 않는다.
- suppress는 출력보다 먼저 적용한다. `corrupted_victory`는 `true_route_victory`보다 우선하고, true route는 successor/new scale/new shadow/closed gate/last bowl/banner/southern market rumor를 억제한다.
- strong evidence가 있는 무림맹 침묵은 증거 부족이 아니라 책임 회피 변주로 출력한다.

계속 닫아 둘 범위:

- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- return/settlement schema
- relation/debt/faction ledger
- reward/ability schema와 `item_unpriced_wooden_sword` payout
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 runtime 후보는 `wuxia_final_epilogue_renderer_contract` implementation slice다. 첫 구현 전에는 `docs/design/Wuxia_Final_State_Routing.md`의 `Final Epilogue Renderer Contract Handoff`를 계약으로 보고, card output이 renderer-neutral `ScenePage` mode인지 structured body block convention인지 먼저 결정한다.

## 0.66 2026-06-02 무협 `wuxia_final_epilogue_renderer_contract` runtime implementation slice

현재 상태: runtime 구현 완료. `docs/design/Wuxia_Final_State_Routing.md`의 final epilogue renderer contract handoff를 기준으로, 첫 구현은 새 `ScenePage` mode가 아니라 `ScenePage.body_blocks`의 structured body block convention으로 landing했다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/endings.yaml`에 `wuxia_final_epilogue_renderer_contract` ending을 추가하고 Rust/Web generated storypack preview bundle을 재생성했다.
- Rust GameCore에 `final_epilogue` seed consumer를 추가했다. 이 consumer는 `GameState.flags`/`GameState.clues`만 읽고 final result priority, candidate seed consumption, suppress, card ordering을 소유한다.
- `ScenePage.body_blocks`는 ending text 뒤에 `epilogue_result`, `epilogue_card`, `epilogue_suppressed`, 필요 시 `epilogue_contract_error` block을 append한다.
- final result priority는 `battle_loss` > `corrupted_victory` > `true_route_victory` > `mumyeong_unsaved_victory` > `meaningful_victory` > `incomplete_victory` > `basic_victory` 순서다.
- suppress는 card output 전에 적용한다. `corrupted_victory`는 true-route 후보보다 우선하고, true route는 successor/new scale/new shadow/closed gate/last bowl/banner/southern market rumor를 억제한다.
- strong evidence가 있는 무림맹 침묵은 증거 부족이 아니라 책임 회피 변주로 출력한다.
- Web Storybook과 SuperLightTUI는 core가 만든 body block을 표시만 한다. 후일담 card enable/suppress를 renderer에서 다시 계산하지 않는다.

계속 닫아 둘 범위:

- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- return/settlement schema
- relation/debt/faction ledger
- reward/ability schema와 `item_unpriced_wooden_sword` payout
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 작업은 새 route를 바로 열기보다 `wuxia_final_epilogue_renderer_contract`의 Web Storybook/SuperLightTUI 표시 UX와 플레이 테스트를 검증하는 follow-up이다. 필요하면 그 다음 slice에서 return/settlement, final battle loss path, reward/ability, relation/debt/faction ledger 중 하나를 별도 contract로 연다.

## 0.67 2026-06-02 무협 final epilogue UX/playtest follow-up

현재 상태: UX/playtest follow-up 구현 완료. `wuxia_final_epilogue_renderer_contract`가 Rust GameCore-owned `ScenePage.body_blocks`로 내보내는 후일담을 Web Storybook과 SuperLightTUI에서 사람이 읽을 수 있는 최종 화면으로 표시하도록 다듬었다.

구현:

- Web Storybook은 `epilogue_result`, `epilogue_card`, `epilogue_suppressed`, `epilogue_contract_error` block을 양피지형 후일담 section으로 렌더링한다.
- Web 후일담 section은 본문 prose를 즉시 노출하고, contract metadata는 접힌 `details` 안에 보관한다. `data-body-kind`, `data-source-id`, field key data attribute는 유지해 renderer-neutral contract와 테스트 포인트를 보존한다.
- Web final epilogue 화면에서는 직전 history row가 summary 영역에 중복 노출되지 않게 했다. story history 자체는 그대로 남기며, 결과 화면의 중심은 core-owned 후일담 block이다.
- SuperLightTUI는 epilogue block heading과 compact terminal lines를 출력한다. terminal snapshot 40-line 밀도에서 중요한 카드 prose가 밀려나지 않도록 result/card/suppressed metadata를 선별하고, 긴 줄은 whitespace 기준으로 wrap한다.
- local Web playtest에서 stale ignored `web/src/core/wasm-pkg/`가 final epilogue block을 전달하지 못하는 문제를 확인해, local WASM package를 재빌드한 뒤 fresh Vite server에서 scripted final route를 재검증했다.

검증:

- Web Storybook render test는 epilogue section class, visible prose, metadata details, field key data attribute, stale summary suppression을 검증한다.
- SuperLightTUI smoke는 이구학지 final epilogue contract route가 compact heading/prose를 snapshot 안에 유지하는지 검증한다.
- Browser DOM playtest는 fresh `localhost:8768` run에서 scripted path로 final까지 진입했고, action choice가 비어 있으며 10개 core-owned epilogue block이 표시되는 것을 확인했다.

계속 닫아 둘 범위:

- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- return/settlement schema
- relation/debt/faction ledger
- reward/ability schema와 `item_unpriced_wooden_sword` payout
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 작업은 새 runtime route를 바로 추가하는 것이 아니라, return/settlement, final battle loss path, reward/ability, relation/debt/faction ledger 중 어느 contract를 먼저 열지 선택하는 docs-only handoff다. Web/SuperLightTUI final epilogue 표시 UX는 이 follow-up에서 검증 완료로 본다.

## 0.68 2026-06-02 return/settlement contract docs-only handoff

현재 상태: docs-only handoff 완료, runtime YAML/Rust/Web/generated artifact 미수정. Notion `가지 말라는 말`, `08. 엔딩과 후일담 연결`, `11. True Ending 단일 루트`, `사도 최종전`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`을 다시 대조했다.

결정:

- final epilogue UX/playtest 이후 다음 contract surface는 return/settlement다.
- 첫 runtime 후보는 `wuxia_seoharin_unsaid_stay` / `가지 말라는 말`이다.
- 첫 구현은 귀환 엔딩 자체가 아니라 서하린 late relationship trigger다. 기존 encounter schema로 귀환하고 싶다는 말, 청류문에 남겠다는 말, 불확실성 공유, 회피를 선택지로 열고 return/settlement/corruption 후보 seed만 남긴다.
- `wuxia_seoharin_unsaid_stay`는 `wuxia_seoharin_qingliu_resolution` 뒤, `wuxia_cheongirok_resolution` 앞에 끼워 넣는 후보로 둔다. Cheonggi Record 마지막 장과 final epilogue는 이 late relationship seed를 이후 소비할 수 있다.
- stable choice id는 `say_return_home_honestly`, `say_you_will_stay_with_qingliu`, `share_uncertainty_without_running`, `turn_away_from_the_empty_place`로 둔다.
- 공통 seed는 `seoharin_unsaid_stay_resolved`, `final_return_settlement_contract_seeded`이며, 후보 seed는 `final_return_intent_honest_seeded`, `final_settlement_intent_honest_seeded`, `final_return_settlement_uncertain_shared_seeded`, `final_return_settlement_evasion_seeded`, `final_epilogue_closed_gate_risk_seeded` 계열을 사용한다.

다음 후보 비교 결과:

- `battle_loss` path는 final epilogue consumer가 승인된 loss seed를 인식할 수 있지만, 먼저 열면 final battle container 또는 명시 loss route가 필요하다.
- reward/ability schema는 `천기록 / 천외편린 보상`의 3택 성장 시스템과 연결되지만, final epilogue 직후의 최소 contract보다 범위가 크다.
- relation/debt/faction ledger는 미해결 부채와 세력 압박이 이미 후일담 원칙으로 표현되고 있으므로, return/settlement 및 loss/corruption branch가 실제 runtime seed를 더 남긴 뒤 열어도 된다.

계속 닫아 둘 범위:

- full modern return ending / post-return settlement scene
- return/settlement save/archive schema
- relation/debt/faction ledger
- reward/ability schema와 `item_unpriced_wooden_sword` payout
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 runtime 후보는 `wuxia_seoharin_unsaid_stay` implementation slice다. 구현 전에는 `docs/design/Wuxia_Final_State_Routing.md`의 `Return/Settlement Contract Handoff`를 계약으로 보고, return/settlement/corruption 후보를 기존 flags/clues/log/presentation hook만으로 남긴다.

## 0.69 2026-06-02 무협 `wuxia_seoharin_unsaid_stay` preview runtime slice

현재 상태: runtime implementation 완료. Notion `가지 말라는 말` 원문과 `docs/design/Wuxia_Final_State_Routing.md`의 `Return/Settlement Contract Handoff`를 대조한 뒤, `wuxia_seoharin_qingliu_resolution` 뒤와 `wuxia_cheongirok_resolution` 앞에 late relationship trigger를 삽입했다.

구현:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 `wuxia_seoharin_unsaid_stay`를 추가했다.
- 조건은 `black_serpent_ledger_vault`, `seoharin_qingliu_resolution_resolved`, `final_state_routing_seeded`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`이며, `seoharin_unsaid_stay_resolved`로 반복을 막는다.
- stable choice id는 `say_return_home_honestly`, `say_you_will_stay_with_qingliu`, `share_uncertainty_without_running`, `turn_away_from_the_empty_place`다.
- 공통 seed는 `seoharin_unsaid_stay_resolved`, `final_return_settlement_contract_seeded`다.
- 후보 seed는 `final_return_intent_honest_seeded`, `final_settlement_intent_honest_seeded`, `final_return_settlement_uncertain_shared_seeded`, `final_return_settlement_evasion_seeded`, `final_epilogue_return_absence_candidate_seeded`, `final_epilogue_qingliu_settlement_candidate_seeded`, `final_epilogue_empty_place_kept_open_seeded`, `final_epilogue_closed_gate_risk_seeded`다.
- `wuxia_cheongirok_resolution`의 required flags에 `seoharin_unsaid_stay_resolved`를 추가해 저장/재진입 상태에서도 이 late trigger를 건너뛰지 않게 했다.
- Rust/Web generated preview bundles, Python storypack DB/export tests, Rust content bundle contract, WASM JSON boundary path, terminal smoke path, Web default bundle registry test를 갱신했다.

계속 닫아 둘 범위:

- full modern return ending / post-return settlement scene
- return/settlement save/archive schema
- relation/debt/faction ledger
- reward/ability schema와 `item_unpriced_wooden_sword` payout
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 후보는 `return_settlement_followup_handoff`다. 이 handoff는 `wuxia_seoharin_unsaid_stay`가 남긴 seed를 기준으로 full modern return ending, settlement afterword, corruption/closed-gate branch, 또는 battle-loss/reward/ledger schema 중 무엇을 먼저 열지 다시 비교한다.

## 0.70 2026-06-02 무협 `wuxia_return_settlement_epilogue_contract` runtime slice

현재 상태: docs-only handoff + runtime implementation 완료. Notion `가지 말라는 말`, `08. 엔딩과 후일담 연결`, `01. 메인 엔딩 구조`, `09. 예시 엔딩`, `10. 이구학지 후일담 카드 DB`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`을 다시 대조해, 다음 구현을 새 ending enum/schema가 아니라 기존 `wuxia_final_epilogue_renderer_contract`의 `ScenePage.body_blocks` branch card 확장으로 결정했다.

결정:

- `return`/`settlement`는 플레이어에게 노출되는 enum이 아니라 내부 분류다. 출력은 `돌아온 출근길`, `청류문에 남은 외지인`, 빈자리/닫힌 문 위험 같은 시적 후일담 카드로 둔다.
- `10. 이구학지 후일담 카드 DB`에는 아직 dedicated `돌아온 출근길` row가 없으므로 archive/main-ending schema를 먼저 열지 않는다.
- `wuxia_seoharin_unsaid_stay`가 이미 남긴 four-way seed를 Rust GameCore-owned final epilogue consumer가 소비한다.

구현:

- `crates/escape-core/src/final_epilogue.rs`에 return/settlement branch card group을 추가했다.
- card ids는 `epilogue_wuxia_returned_commute`, `epilogue_wuxia_qingliu_settlement`, `epilogue_wuxia_empty_place_kept_open`, `epilogue_wuxia_closed_gate_risk`다.
- `epilogue_wuxia_closed_gate_risk`는 contradictory direct-state seed가 동시에 있을 때 optimistic return/settlement/open-place branch cards를 `return_settlement_evasion`으로 suppress한다.
- card ordering은 Rust GameCore가 소유한다. return/settlement branch는 terminal 40-line smoke에서도 보이도록 final epilogue card list 앞쪽에 둔다.
- WASM JSON boundary와 SuperLightTUI scripted final route가 `choice:say_return_home_honestly` 뒤 `epilogue_wuxia_returned_commute`를 볼 수 있게 테스트를 갱신했다.

계속 닫아 둘 범위:

- 새 `main_ending_type` runtime enum
- full modern return ending scene / post-return settlement scene
- return/settlement save/archive schema
- relation/debt/faction ledger
- reward/ability schema와 천외편린 3택 UI
- `item_unpriced_wooden_sword` payout
- combat resolver, HP numeric battle, full `wuxia_sado_final_battle` container
- 서하린 truth delivery, `told_seoharin_truth`
- 천기록 기록자 정체 reveal
- legacy office bundle, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key

다음 후보는 `return_settlement_epilogue_followup_handoff`다. 이 handoff는 battle-loss branch, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface 중 무엇을 다음에 열지 비교한다.

## 0.71 2026-06-02 무협 return_settlement_epilogue_followup_handoff docs-only handoff: battle-loss epilogue contract

현재 상태: docs-only handoff 완료. Notion `최종장 결산 라우팅 마스터`, `사도 최종전`, `사도 최종전 상태값 사전`, `08. 엔딩과 후일담 연결`, `닫힌 산문`, `흑사방의 깃발`, `검은 뱀의 새 비늘`, `천기록의 마지막 장`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `01. 메인 엔딩 구조`를 다시 대조했다.

결정:

- 다음 runtime 후보는 `wuxia_battle_loss_epilogue_contract`다.
- `battle_loss`는 `최종장 결산 라우팅 마스터`의 final result priority에서 가장 먼저 평가되며, 후일담 bundle도 Notion `08. 엔딩과 후일담 연결` 및 후일담 DB row들에 이미 명시되어 있다.
- broader corruption/closed-gate branch는 `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page` corruption variant, corrupted result priority로 이미 일부 표현된다. 즉시 비어 있는 구멍은 loss result bundle이다.
- reward/ability schema는 `07. 천기록 / 천외편린 보상` 기준 3택 성장/부작용 시스템이라 범위가 크다.
- relation/debt/faction ledger는 `06. 사이드 퀘스트와 미해결 부채` 기준 후일담 흔적만으로도 우선 표현 가능하므로 지금 ledger schema를 열지 않는다.
- main ending archive/save surface는 `01. 메인 엔딩 구조` 기준 내부 planning vocabulary에 가깝고, player-facing archive/save surface는 아직 열지 않는다.

다음 구현 계약:

- `crates/escape-core/src/final_epilogue.rs`의 기존 final epilogue body block consumer를 확장한다.
- 입력은 명시적 `final_combat_result_battle_loss_seeded` final-state seed다.
- 출력 bundle은 `epilogue_boss_black_serpent_banner`, `epilogue_wuxia_southern_market_rumor`, `epilogue_mumyeong_black_serpent_new_scale`, `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page` corruption variant다.
- `battle_loss`는 `epilogue_boss_broken_black_serpent`, `epilogue_seoharin_open_gate`, `epilogue_mumyeong_stolen_forms_stopped` 같은 optimistic victory cards를 suppress해야 한다.
- 이 slice는 full `wuxia_sado_final_battle` container, combat resolver, HP 숫자전, playable defeat route, archive/save schema를 열지 않는다.

Next runtime implementation candidate: `wuxia_battle_loss_epilogue_contract`.

## 0.72 2026-06-02 무협 `wuxia_battle_loss_epilogue_contract` runtime slice

현재 상태: runtime implementation 완료. `return_settlement_epilogue_followup_handoff`에서 선택한 battle-loss 후일담 bundle을 기존 Rust GameCore-owned final epilogue body block consumer에 구현했다.

구현:

- `crates/escape-core/src/final_epilogue.rs`가 `final_combat_result_battle_loss_seeded`를 `FinalResult::BattleLoss`로 판정하고, optional card seed가 없어도 loss bundle을 출력한다.
- 출력 card ids는 `epilogue_boss_black_serpent_banner`, `epilogue_wuxia_southern_market_rumor`, `epilogue_mumyeong_black_serpent_new_scale`, `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page`다.
- loss variants는 각각 `battle_loss_residue`, `unresolved_debt`, `battle_loss_successor_pressure`, `battle_loss_or_corruption`, `corruption_variant`다.
- `battle_loss`는 `epilogue_boss_broken_black_serpent`, `epilogue_seoharin_open_gate`, `epilogue_mumyeong_stolen_forms_stopped`를 suppress한다.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/endings.yaml`의 final epilogue ending gate는 victory-only required flag를 제거하고, victory/loss 승인 여부는 Rust final epilogue body block consumer의 precondition으로 둔다.
- Rust preview bundle과 Web preview bundle generated artifacts를 갱신했다.

검증:

- Rust route parity direct-state test가 battle-loss bundle과 optimistic card suppress를 검증한다.
- WASM JSON boundary test가 Web renderer로 전달되는 `ScenePage.body_blocks`에도 같은 bundle이 들어오는지 검증한다.
- 이 slice는 새 `ScenePage` mode를 만들지 않고 기존 `epilogue_result`, `epilogue_card`, `epilogue_suppressed` convention을 유지한다.

계속 닫아 둘 범위:

- full `wuxia_sado_final_battle` container
- combat resolver 또는 HP numeric battle
- playable defeat route / loss encounter sequence
- main ending archive/save surface
- relation/debt/faction ledger
- reward/ability schema와 천외편린 3택 성장 UI
- full modern return ending scene 또는 post-return settlement scene
- Seo Harin truth delivery, `told_seoharin_truth`
- Cheonggi Record recorder identity reveal

다음 후보는 `wuxia_battle_loss_epilogue_followup_handoff`다. 이 handoff는 battle-loss bundle이 runtime evidence를 얻은 뒤 full final battle container, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface, 또는 playable defeat-route bridge 중 무엇을 먼저 열지 비교한다.

## 0.73 2026-06-02 무협 `wuxia_battle_loss_epilogue_followup_handoff` docs-only handoff: final-state canonical collapse

현재 상태: docs-only contract selection 완료. Battle-loss 후일담 runtime이 구현된 뒤 Notion 원본과 repo canonical final routing 문서를 다시 대조했고, 다음 runtime 후보를 `wuxia_final_state_canonical_collapse_contract`로 결정했다.

대조한 Notion source:

- `최종장 결산 라우팅 마스터`
- `사도 최종전`
- `사도 최종전 상태값 사전`
- `08. 엔딩과 후일담 연결`
- `06. 사이드 퀘스트와 미해결 부채`
- `07. 천기록 / 천외편린 보상`
- `03. 세력과 외부 압박`
- `엔딩 시스템`
- `01. 메인 엔딩 구조`
- `06. 엔딩 아카이브`

비교 결과:

- full `wuxia_sado_final_battle` container는 최종적으로 필요하지만, Notion은 실제 전투 선택을 1/2/3페이즈 카드가 관리하고 상태값은 후일담 라우팅 언어라고 분리한다. 지금 바로 full container를 열면 combat resolver, playable defeat route, save/archive 경계까지 함께 커진다.
- broader corruption/closed-gate branch는 이미 `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page` corruption variant, corrupted priority, battle-loss loss bundle로 일부 표현된다. 다음에 필요한 것은 새 카드보다 어떤 seed가 canonical `cheongirok_state`/`player_method`/`seoharin_axis`로 접히는지 Rust가 소유하는 것이다.
- reward/ability schema는 Notion `07`의 천외편린 3-choice 구조를 요구하므로 별도 성장 시스템 slice가 필요하다. final-state collapse 없이 먼저 열면 후일담/침식 판정과 결합 지점이 불명확하다.
- relation/debt/faction ledger는 `03`/`06`의 풍문, 공문, 장부, 미해결 부채 표현을 담는 넓은 시스템이다. 현재 final 후일담 소비자는 ledger 없이도 unresolved pressure를 표현할 수 있다.
- main ending archive/save surface는 `엔딩 시스템`/`01. 메인 엔딩 구조`/`06. 엔딩 아카이브` 기준으로 player-facing archive, poetic categories, undiscovered counts까지 포함한다. 현재는 main ending type보다 final state summary가 먼저 필요하다.
- playable defeat-route bridge는 battle-loss 후일담을 실제 플레이 경로로 연결하는 좋은 후보지만, 현 ending gate와 boss/무명/서하린/천기록/흑사방 resolution flags 구조상 final-state collapse 없이 붙이면 loss route와 victory route의 중간 결산이 섞인다.

선택한 다음 runtime:

```yaml
next_runtime:
  id: wuxia_final_state_canonical_collapse_contract
  implementation_owner: crates/escape-core/src/final_epilogue.rs
  output_shape: existing ScenePage.body_blocks
  likely_block_kind: epilogue_state_audit
  source_contracts:
    - docs/design/Wuxia_Final_State_Routing.md
    - docs/dev/Development_Plan.md#0.73
  input_source: existing final_*_seeded flags
  canonical_states:
    - combat_result
    - boss_resolution_route
    - evidence_state
    - network_handling
    - pressure_state
    - seoharin_axis
    - qingliu_rebuild
    - mumyeong_salvation
    - successor_route
    - own_flow_choice
    - truth_state
    - cheongirok_state
    - player_method
    - item_logs
  required_contract:
    - collapse local suffix flags like *_seeded, *_candidate, *_seen, *_trace into canonical state labels
    - preserve final_result_priority ordering, especially battle_loss first and corrupted_victory over partial true-route conditions
    - report ambiguous or missing canonical inputs as structured audit lines, not renderer-side guesses
    - keep Web Storybook and SuperLightTUI display-only
  first_runtime_output:
    - add a Rust-owned final state audit body block beside epilogue_result/card/suppressed
    - direct-state route parity test for true/corrupted/battle-loss summaries
    - WASM JSON boundary test proving the audit block crosses the player boundary
```

계속 닫아 둘 범위:

- full `wuxia_sado_final_battle` container
- combat resolver 또는 HP numeric battle
- playable defeat route / loss encounter sequence
- new `main_ending_type` runtime enum
- main ending archive/save surface
- relation/debt/faction ledger
- reward/ability schema와 천외편린 3택 성장 UI
- full modern return ending scene 또는 post-return settlement scene
- Seo Harin truth delivery, `told_seoharin_truth`
- Cheonggi Record recorder identity reveal

Next runtime implementation candidate: `wuxia_final_state_canonical_collapse_contract`.

## 0.74 2026-06-02 무협 `wuxia_final_state_canonical_collapse_contract` runtime slice

현재 상태: runtime implementation 완료. `wuxia_battle_loss_epilogue_followup_handoff`가 선택한 canonical collapse contract를 Rust GameCore final epilogue consumer에 구현했다.

구현 내용:

- `crates/escape-core/src/final_epilogue.rs`가 `epilogue_result` 직후 `epilogue_state_audit` body block을 출력한다.
- audit block은 `wuxia_final_state_canonical_collapse_contract` source id를 갖고, renderer가 final state를 재계산하지 않아도 되도록 Rust GameCore-owned text contract로 canonical state summary를 제공한다.
- 기존 `final_*_seeded` local flags를 다음 canonical final state labels로 collapse한다.
  - `combat_result`
  - `boss_resolution_route`
  - `evidence_state`
  - `network_handling`
  - `pressure_state`
  - `seoharin_axis`
  - `qingliu_rebuild`
  - `mumyeong_salvation`
  - `successor_route`
  - `own_flow_choice`
  - `truth_state`
  - `cheongirok_state`
  - `player_method`
  - `item_logs`
- `battle_loss`가 `battle_victory`와 함께 수동 direct-state에 들어와도 `combat_result: battle_loss`를 우선하고 `status: ambiguous_priority_applied`로 보고한다.
- boss route seed가 없는 battle-loss 상태는 `boss_resolution_route: not_reached_battle_loss`, `status: derived_by_final_result_priority`로 보고한다.
- conflicting canonical candidates는 renderer가 추측하지 않도록 `status: ambiguous_priority_applied`, `candidate_values`, `consumed_flags`를 audit text에 남긴다.
- missing canonical inputs는 `value: missing`, `status: missing`, `consumed_flags: none`으로 보고한다.
- Web Storybook과 SuperLightTUI는 기존 `ScenePage.body_blocks`를 표시/전달만 하며 audit을 다시 계산하지 않는다.

검증:

- Rust route parity direct-state test가 true/corrupted/battle-loss audit collapse를 검증한다.
- WASM JSON boundary test가 `epilogue_state_audit` block이 player boundary를 통과하는지 검증한다.

계속 닫아 둘 범위:

- full `wuxia_sado_final_battle` container
- combat resolver 또는 HP numeric battle
- playable defeat route / loss encounter sequence
- new `main_ending_type` runtime enum
- main ending archive/save surface
- relation/debt/faction ledger
- reward/ability schema와 천외편린 3택 성장 UI
- full modern return ending scene 또는 post-return settlement scene
- Seo Harin truth delivery, `told_seoharin_truth`
- Cheonggi Record recorder identity reveal

다음 후보는 `wuxia_final_state_canonical_collapse_followup_handoff`다. 이 handoff는 canonical final state audit이 runtime evidence를 얻은 뒤 full final battle container, playable defeat-route bridge, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface 중 무엇을 먼저 열지 다시 비교한다.

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
- storypack/card 후보 DB의 `world_id`/`storypack_id`/taxonomy/fallback/outcome hook 참조 무결성

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
18. Web player deployment readiness 완료: `VITE_BASE_PATH`, module-relative WASM import path, `VITE_REQUIRE_WASM` fatal policy, GitHub Pages workflow, deployment contract test로 URL 즉시 플레이용 static deploy 기반을 고정했다.
19. Web player start/save UX first slice 완료: start screen, 이어하기/새 게임, seed 표시, save timestamp summary, 저장 초기화, reset confirmation, schema mismatch warning, visual QA start-screen 통과를 구현했다.
20. Web Storybook transition/audio readiness PR A 완료: `escape-office.player-settings.v1` localStorage settings, start screen audio/motion toggles, transition plan type, reduced-motion duration 0 policy, audio muted/off opt-in default를 구현했다.
21. 전투 시스템 아이디어 문서화 완료: `idea_box/combat_system.md`를 `docs/design/Combat_System_Auto_Brawl.md`로 승격했고, 자동 난투 + 상황 개입을 Rust GameCore/`ScenePage` 호환 future backlog로 정리했다. 기본 공격 형식/싸우는 형태는 `docs/design/Basic_Combat_Action_Model.md`에 이구학지 기준이지만 office에도 재사용하는 action taxonomy로 분리했다.
22. PR B transition controller 완료: action 실행 전 current page/action context를 캡처하고, action 후 `transitionPlan(previousPage, nextPage, action)`으로 `.storybook-shell` enter/exit class/attribute transition을 적용한다. reduced/off motion은 즉시 render하고, `transitionend` 미발생 시 timeout fallback으로 게임이 멈추지 않게 했다.
23. PR C audio engine skeleton 완료: `web/src/ui/audio/audioEngine.ts`가 lazy Web Audio generated oscillator backend와 no-op fallback을 제공하고, muted 상태 no schedule, user-gesture opt-in unlock, one-shot cue, looping ambience API를 renderer-local로 고정했다. Rust GameCore / `ScenePage` / WASM JSON boundary와 binary audio asset은 변경하지 않았다.
24. schema-less combat encounter prototype 완료: `supply_closet_cache`에서 `supply_closet_auto_brawl`로 이어지는 물품창고 자동 난투를 기존 encounter/choice/outcome schema만으로 구현했고, Rust `ScenePage`, SuperLightTUI, Web generated data에서 같은 action id와 presentation hint를 검증한다.
25. storypack/world 일반화 첫 설계 완료: 초기에는 office를 기본 storypack으로 유지하면서 `docs/design/Storypack_World_Model.md`와 `wuxia_jianghu_pack`을 추가해 무협 세계관을 첫 비-office 기준팩으로 세웠고, 이후 Web/default storypack은 이구학지로 전환했다.
26. 무협 기준팩 최신화 완료: `wuxia_jianghu_pack`의 canonical story를 Notion 최신안 **이구학지 — 천기록**으로 교체했고, 이전 generic 무협 placeholder는 superseded로 정리했다. 후보 카드도 `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment` 중심으로 갱신했다.
27. machine-readable storypack DB 검증 완료: `docs/content/storypack_db/storypacks.json`와 `docs/content/storypack_db/encounter_situations.json`에 office/wuxia 후보 DB를 추가했고, `validate_storypack_db()`로 storypack/world/taxonomy/fallback/outcome hook 참조 무결성을 테스트한다.
28. storypack runtime preview mode 결정 완료: `docs/dev/Storypack_Runtime_Preview_Mode.md`에서 첫 non-office runtime prototype은 **separate preview mode first**로 진행하기로 결정했다. 기본 office bundle과 `src/tui_adv/data/*.yaml`은 그대로 두고, `wuxia_jianghu_pack`은 explicit preview bundle 또는 preview flag로만 runtime에 들어간다.
29. 무협 storypack preview runtime prototype 완료: `wuxia_commute_rift_arrival`을 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml` source와 별도 Rust/Web generated preview bundle로 구현했다. Generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`다. `escape-wasm::new_game_json()`과 `escape-terminal --scene content`는 preview `runtime.default_location`을 사용해 `wuxia_commute_rift`에서 시작하며, 기본 office bundle은 계속 `dev_desk`에서 시작한다.
30. 야근몽 office-dream storypack 후보 문서화 완료: live Notion Markdown을 대조한 뒤 `docs/content/storypacks/yageunmong_pack.md`, `docs/content/encounter_db/yageunmong_pack.md`, storypack DB JSON 후보 record/cards를 추가했고, 관련 idea_box entry 2개를 done 처리했다. runtime 구현은 미착수다.
31. 무협 `wuxia_heuksa_bang_first_fight` preview runtime slice 완료: `jianghu_market_street` 위치와 `wuxia_arrival_hidden` branch에서 이어지는 흑사방 첫 난투를 같은 `wuxia_jianghu_pack` storypack preview source에 추가했다. 구현은 기존 `conditions`, `choices`, `outcome.resources`, `danger`, `add_flags`, `add_clues`, `log`, optional `presentation/effect_cues`만 사용하며 새 combat/reward/ability schema를 열지 않았다. Rust fixture/Web generated preview bundle을 갱신했고, Python exporter, Rust content fixture, WASM JSON boundary action flow, SuperLightTUI smoke가 first fight의 action id/presentation/effect cue parity를 검증한다. 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 변경하지 않았다.
32. 무협 preview launcher/UI wiring 완료: terminal은 `escape-terminal --scene content --storypack-preview wuxia_jianghu_pack` opt-in flag로 built-in preview fixture를 선택한다. Web은 start screen preview launcher와 `web/src/core/contentBundles.ts` registry로 기본 office bundle과 `wuxia_jianghu_pack` generated preview bundle을 분리한다. Web storypack preview run은 기본 office save/localStorage key를 쓰지 않고, default office continue/new-game UX는 기존 key를 유지한다.
33. 무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice 완료: 첫 난투 뒤 같은 `jianghu_market_street`에서 `heuksa_bang_first_fight_resolved`를 요구하는 천기록 첫 편린 encounter를 추가했다. `cheonggi_record_notebook` item과 `wuxia_first_fragment_seen` achievement를 preview bundle에만 추가하고, 선택지는 `choose_guard_basics`, `choose_keep_feet_moving`, `choose_failure_log`, fallback `close_notebook_without_choice`로 고정했다. 새 reward/ability/combat schema나 천외편린 3택 성장 UI는 열지 않고 flags/clues/log/presentation hook만 사용했다.
34. 무협 `wuxia_seo_harin_rescue` preview runtime slice 완료: 서하린 구조/외지인 조사/청류문 보호·감시 bridge를 같은 storypack preview source에 추가했다. `cheongryu_outer_courtyard` location, stable choice id 5개, `seo_harin_rescue_resolved`/`taken_under_watch`/`outsider_claim_recorded` common hooks, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
35. 무협 `wuxia_cheongryu_apprentice_entry` preview runtime slice 완료: rescue 이후 청류문 수습생/잡역/채무/서고 bridge를 같은 storypack preview source에 추가했다. `work_chore_token`, stable choice id 4개, `cheongryu_apprentice_entry_resolved`/`cheongryu_trial_started`/`seo_harin_mentor_thread` common hooks, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
36. 무협 `wuxia_cheongryu_chore_sparring` preview runtime slice 완료: `docs/design/Basic_Combat_Action_Model.md`의 이구학지/office 공용 action taxonomy를 기준으로 청류문 장작 마당의 작은 몸싸움을 schema-less `combat_intervention` encounter로 추가했다. stable choice id 4개, `cheongryu_chore_sparring_resolved`/`chore_sparring_completed` hooks, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했고, CombatState/combat resolver/reward/ability schema와 기본 office bundle은 변경하지 않았다.
37. 무협 `wuxia_cheongryu_raid_route_split` preview runtime slice 완료: rescue/apprentice/chore sparring와 first-fragment 공통 hook 뒤에만 열 중반 route pressure encounter를 같은 storypack preview source에 추가했다. stable choice id 4개, fallback `evacuate_the_wounded_first`, route flags/clues/log/presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
38. 무협 `wuxia_cheongryu_raid_wounded_fallback` deferred-route 후보 설계 완료: raid split의 `evacuate_the_wounded_first` fallback 이후를 받아 route opener 전 공통 재합류를 만드는 prerequisites, stable choice id, fallback, route starter hook, schema non-goal, 구현 handoff를 문서화했다.
39. 무협 `wuxia_cheongryu_raid_wounded_fallback` preview runtime slice 완료: raid split fallback branch 뒤 조건부 deferred bridge를 같은 storypack preview source에 추가했다. stable choice id 4개, `cheongryu_raid_wounded_fallback_resolved`/`deferred_route_reopened`와 route starter flags/clues/log/presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
40. 무협 route opener docs-only handoff 완료: 첫 route opener runtime 후보를 정파/백도맹 약상자 채무 축인 `wuxia_baekdo_medicine_debt`로 결정했다. start conditions는 `righteous_route_started` + `cheongryu_rebuild_thread`이며, direct `baekdo_alliance_debt`와 deferred `baekdo_medicine_debt`는 flavor hook으로만 둔다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
41. 무협 `wuxia_baekdo_medicine_debt` preview runtime slice 완료: 첫 정파 route opener를 같은 storypack preview source에 추가했다. stable choice id 4개, `baekdo_medicine_debt_resolved`/`righteous_route_opened` common hook, 백도맹 약상자/청류문 재건 채무 clues/log/presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
42. 무협 route opener follow-up docs-only handoff 완료: 다음 runtime 후보를 사파/흑천련 거래 opener `wuxia_black_heaven_escape_price`로 결정했다. start conditions는 `sapa_route_started` + `dowol_debt`이며, direct `black_heaven_deal_marked`와 deferred `black_heaven_escape_marker`는 flavor hook으로만 둔다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
43. 무협 `wuxia_black_heaven_escape_price` preview runtime slice 완료: 첫 사파 route opener를 같은 storypack preview source에 추가했다. stable choice id 4개, `black_heaven_escape_price_resolved`/`sapa_route_opened` common hook, 흑천련 탈출로/도월 표식/시장 장부 clues/log/presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
44. 무협 route opener follow-up after black heaven docs-only handoff 완료: 다음 runtime 후보를 천기·귀환 opener `wuxia_heavenly_archive_previous_outsiders`로 결정했다. start conditions는 `cheonggi_return_route_started` + `cheonggi_record_targeted`이며, direct `heavenly_archive_contact`와 deferred `heavenly_archive_triage_map_seen`는 flavor hook으로만 둔다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
45. 무협 `wuxia_heavenly_archive_previous_outsiders` preview runtime slice 완료: 첫 천기·귀환 route opener를 같은 storypack preview source에 추가했다. stable choice id 4개, `heavenly_archive_previous_outsiders_resolved`/`cheonggi_return_route_opened` common hook, 천기각 이전 이방인 기록/균열 clues/log/presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
46. 무협 route opener follow-up after heavenly archive docs-only handoff 완료: 다음 runtime 후보를 deferred-offer card `wuxia_wounded_shelter_dawn_offers`로 결정했다. start conditions는 `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`이며, `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 flavor hook으로만 둔다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
47. 무협 `wuxia_wounded_shelter_dawn_offers` preview runtime slice 완료: `stabilize_wounded_until_dawn` deferred branch를 같은 storypack preview source에 추가했다. stable choice id 4개, `wounded_shelter_dawn_offers_resolved`/`route_commitment_reopened` common hook, 정파/사파/천기 route reentry flags, 새벽/부상자/제안 presentation hook, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
48. 무협 post-opener midgame continuity docs-only handoff 완료: 다음 runtime 후보를 common midgame bridge `wuxia_mumyeong_first_sighting`로 결정했다. 새 any-of condition schema 대신 세 route opener outcome에 공통 `route_opener_resolved` flag를 추가하는 방식으로 `righteous_route_opened`/`sapa_route_opened`/`cheonggi_return_route_opened` fan-in을 처리한다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
49. 무협 `wuxia_mumyeong_first_sighting` preview runtime slice 완료: 세 route opener outcome에 `route_opener_resolved`를 추가하고, 무명 첫 목격 common midgame bridge를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_first_sighting_resolved`/`midgame_continuity_started` common hook, 무명 존재/카피 무공/서하린 침묵 clues, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
50. 무협 rival confrontation docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`와 운영 문서 `04`/`05`/`06`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_first_confrontation`로 결정했다. 첫 대치는 승리 판정이 아니라 버티기/관찰/분석 encounter로 구현하며, combat schema/route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema는 열지 않는다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
51. 무협 `wuxia_mumyeong_first_confrontation` preview runtime slice 완료: 무명 첫 대치를 같은 storypack preview source에 추가했다. stable choice id 5개, `mumyeong_first_confrontation_resolved`/`mumyeong_rival_thread_opened` common hook, 카피 무공 결함/승리 불필요/서하린 침묵/청류안 대비 clues, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 기본 office bundle과 legacy `escape-office` save/localStorage key는 변경하지 않았다.
52. 무협 `wuxia_mumyeong_copy_style_reveal` preview runtime slice 완료: 무명의 카피 무공 공개를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_copy_style_reveal_resolved`/`copy_style_hint_recorded` common hook, 카피 계열/호흡 불일치/겉흐름 복사/천외편린 후보 변형 clues, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. Web/default storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
53. 무협 post-copy-style docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_reads_orthodox_style`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_reads_orthodox_style`로 결정했다. 현악문/복호금쇄수 단서를 flags/clues/log/presentation으로 먼저 열고, 무명 중반 재회, 보스 첫 등장, 무명 이탈 진실 정리, random copy-style/reward/combat/route graph schema는 아직 열지 않는다.
54. 무협 `wuxia_mumyeong_reads_orthodox_style` preview runtime slice 완료: 무명의 정파 무공 간파를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_reads_orthodox_style_resolved`/`orthodox_style_trace_recorded` common hook, 현악문/복호금쇄수/무명 시야 변주/정파식 통제 무공 clues, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web parity tests를 갱신했다. 무명 중반 재회, 보스 첫 등장, 무명 이탈 진실 정리, full flashback, reward/combat/route graph schema는 열지 않았다.
55. terminal default storypack 전환 완료: `escape-terminal --scene content`가 `--content-bundle`/`--storypack-preview` 없이 실행될 때도 `wuxia_jianghu_pack` / **이구학지 — 천기록** built-in fixture를 기본으로 선택한다. legacy office fixture는 `--content-bundle` 명시 경로로 유지한다.
56. 무협 orthodox-style follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_midgame_reunion`으로 결정했다. 정파 무공 흔적과 카피 무공 단서를 무명/서하린 관계로 넘기되, 무명 이탈 진실 정리, boss first appearance, 청류문 습격 full flashback, reward/combat/route graph schema는 아직 열지 않는다.
57. 무협 `wuxia_mumyeong_midgame_reunion` preview runtime slice 완료: 무명 중반 재회를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_midgame_reunion_resolved`/`mumyeong_mirror_thread_deepened` common hook, 서하린 침묵/현악문 흔적/보스가 무명의 상처를 이용했다는 clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
58. 무협 midgame-reunion follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_boss_first_appearance`로 결정했다. 보스 첫 등장은 combat/final resolution이 아니라 압도감, 조직력, 약점 읽기, 무명이 따르는 이유를 기존 flags/clues/log/presentation으로만 각인한다. runtime YAML/Rust/Web/generated artifact는 아직 변경하지 않았다.
59. 무협 `wuxia_boss_first_appearance` preview runtime slice 완료: 보스 첫 등장을 같은 storypack preview source에 추가했다. stable choice id 4개, `boss_first_appearance_resolved`/`boss_wall_thread_opened`/`black_serpent_core_pressure_opened` common hook, 보스의 약점 읽기/최종 논리 벽/무명이 따르는 이유/청류문이 아직 힘으로 넘을 수 없다는 clues, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
60. 무협 boss follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`과 운영 문서 `04`/`05`/`06`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_request_for_aid`로 결정했다. 이 handoff는 보스 첫 등장 이후 무명이 왜 힘의 논리에 끌렸는지 설명하는 failed-aid records bridge이며, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 변경하지 않았다.
61. 무협 `wuxia_mumyeong_request_for_aid` preview runtime slice 완료: 무명의 도움 요청 실패 기록을 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_request_for_aid_resolved`/`mumyeong_failed_aid_thread_opened`/`orthodox_hypocrisy_thread_opened` common hook, `rejected_aid_letter_fragment` item, 무명이 청류문을 살리려 했다는 clue와 정파 거절 clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
62. 무협 failed-aid follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_awakening`으로 결정했다. 이 handoff는 도움 요청 실패와 정파 무공 흔적을 무명의 분노/카피 무공 변질로 잇되, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 변경하지 않았다.
63. 무협 `wuxia_mumyeong_awakening` preview runtime slice 완료: 무명의 각성을 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_awakening_resolved`/`mumyeong_awakening_thread_opened`/`copy_corruption_thread_opened` common hook, 무명의 카피가 재능이 아니라 분노와 상처에서 개화했다는 clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
64. 무협 awakening follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_resolution`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_qingliu_attack_after_war`로 결정했다. 단, 다음 구현은 청류문 습격 full flashback이 아니라 현악문/복호금쇄수 흔적 조사로 제한하고, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 이 handoff에서 변경하지 않았다.
65. 무협 `wuxia_qingliu_attack_after_war` preview runtime slice 완료: 무명 각성 뒤 청류문 외원에 남은 현악문/복호금쇄수 흔적 조사를 같은 storypack preview source에 추가했다. stable choice id 4개, `qingliu_attack_after_war_resolved`/`qingliu_attack_trace_confirmed`/`hyeonakmun_attack_thread_opened` common hook, full flashback을 아직 열지 않는 clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
66. 무협 post-Qingliu trace docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`와 운영 문서 `04`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_destroys_orthodox_sect`로 결정했다. 단, 다음 구현은 현악문 멸문 전투나 full flashback이 아니라 빈 현악문 산문/기록/풍문을 확인하는 consequence trace로 제한하고, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 이 handoff에서 변경하지 않았다.
67. 무협 `wuxia_mumyeong_destroys_orthodox_sect` preview runtime slice 완료: 청류문 흔적 조사 뒤 빈 현악문 산문/부러진 현판/사라진 장부를 확인하는 consequence trace를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_destroys_orthodox_sect_resolved`/`hyeonakmun_destruction_thread_opened`/`departure_truth_thread_deepened` common hook, `hyeonakmun_was_destroyed_after_qingliu_attack`와 `mumyeong_destroyed_hyeonakmun_alone` clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
68. 무협 Hyeonakmun consequence follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_boss_recruits_mumyeong`로 결정했다. 단, 다음 구현은 구원이나 최종 결산이 아니라 보스가 무명의 상처를 흑사방 힘으로 바꾸는 recruitment trace로 제한하고, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 이 handoff에서 변경하지 않았다.
69. 무협 `wuxia_boss_recruits_mumyeong` preview runtime slice 완료: 현악문 consequence trace 뒤 흑사방 보스의 스카웃 흔적을 같은 storypack preview source에 추가했다. stable choice id 4개, `boss_recruits_mumyeong_resolved`/`boss_recruitment_thread_opened` common hook, `boss_recruited_mumyeong_after_hyeonakmun`, `recruitment_was_not_salvation`, `boss_turned_wound_into_power` clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 기본 storypack은 이구학지이며, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
70. 무협 boss recruitment follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`와 운영 문서 `04`/`05`/`07`/`99`를 대조해 다음 runtime 후보를 `wuxia_mumyeong_departure_truth_summary`로 결정했다. 단, 다음 구현은 서하린에게 진실을 전하거나 무명 구원을 확정하는 것이 아니라 `sealed_departure_truth_summary` trace로 제한하고, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 이 handoff에서 변경하지 않았다.
71. 무협 `wuxia_mumyeong_departure_truth_summary` preview runtime slice 완료: 보스 recruitment trace 뒤 무명 이탈 진실의 sealed summary를 같은 storypack preview source에 추가했다. stable choice id 4개, `mumyeong_departure_truth_summary_resolved`/`sealed_departure_truth_summary_prepared`/`truth_delivery_still_unopened` common hook, `departure_truth_can_be_understood_but_not_spoken_yet`, `seoharin_truth_delivery_requires_later_consent`, `boss_used_mumyeongs_wound_after_orthodox_refusal`, `salvation_condition_seen_but_unmet` clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, final/epilogue schema, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
72. 무협 departure truth summary follow-up docs-only handoff 완료: Notion 사건 카드 DB `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_departure_truth_summary`와 최신 사도 최종전/무명 결산/보스 결산/최종장 라우팅 문서를 대조해 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정했다. 단, 다음 구현은 서하린 truth delivery나 무명 구원 확정이 아니라 sealed truth summary 뒤의 late empty-place memory bridge로 제한하고, runtime YAML/Rust/Web/generated artifact와 legacy office bundle은 이 handoff에서 변경하지 않았다.
73. 무협 `wuxia_seoharin_empty_place` preview runtime slice 완료: sealed departure truth summary 뒤 청류문 외곽 마당의 비워둔 자리를 같은 storypack preview source에 추가했다. stable choice id 4개, `seoharin_empty_place_resolved`/`seoharin_axis_opened`/`empty_place_remembered`/`truth_delivery_still_unopened` common hook, `seoharin_remembers_without_possessing`, `empty_place_is_return_not_claim`, `mumyeong_place_still_unclaimed`, `unpriced_wooden_sword_condition_seeded`, `truth_delivery_still_requires_consent` clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. `item_unpriced_wooden_sword` 지급, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, final/epilogue schema, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
74. 무협 Seo Harin empty-place follow-up docs-only handoff 완료: Notion `남겨둔 밥`, `가지 말라는 말`, `무명 결산`, `사도 최종전 3페이즈`를 대조해 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정했다. `남겨둔 밥`은 `seoharin_axis_opened` 뒤에 붙는 daily-care bridge이고, `가지 말라는 말`/무명·보스 결산/사도 최종전은 final/epilogue/combat/reward schema를 요구하므로 보류했다.
75. 무협 `wuxia_seoharin_left_meal` preview runtime slice 완료: `wuxia_seoharin_empty_place` 뒤 청류문에 남겨 둔 식은 밥 한 그릇을 같은 storypack preview source에 추가했다. stable choice id 4개, `seoharin_left_meal_resolved`/`truth_delivery_still_unopened` common hook, `seoharin_axis_deepened`/`qingliu_belonging_warmed` positive hook, `seoharin_axis_still_open`/`left_meal_left_untouched` refusal hook, `left_meal_was_kept_for_return`, `belonging_is_daily_care`, `last_bowl_epilogue_seeded` clue, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web bundle registry 테스트를 갱신했다. 서하린 truth delivery, `told_seoharin_truth`, final return/settlement choice, 무명/보스 결산, 사도 최종전, final/epilogue schema, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
76. 무협 Seo Harin left-meal follow-up docs-only handoff 완료: `가지 말라는 말`, 무명 결산, 보스 결산, 사도 최종전, 최종장 결산 라우팅 마스터, 사도 최종전 상태값 사전을 대조했다. 직접 final runtime을 열기 전 `docs/design/Wuxia_Final_State_Routing.md`로 canonical final inputs/result priority/alias policy를 먼저 고정했고, 다음 runtime 후보를 `wuxia_sado_final_phase_1_price_tag`로 둔다.
77. 무협 `wuxia_sado_final_phase_1_price_tag` preview runtime slice 완료: `docs/design/Wuxia_Final_State_Routing.md` contract 이후 첫 final-entry slice로 사도 최종전 1페이즈 가격표를 구현했다. 새 location `black_serpent_ledger_vault`, stable choice id 4개, `sado_final_phase_1_price_tag_resolved`/`final_state_routing_seeded` common hook, direct/ledger burn/ledger secure/pressure relief별 `network_handling`·`evidence_state`·`pressure_state`·`item_logs` seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web bundle registry 테스트를 갱신했다. combat resolver, HP 숫자전, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
78. 무협 `wuxia_sado_final_phase_2_weakpoint_control` preview runtime slice 완료: 사도 최종전 2페이즈 약점 장악을 기존 encounter schema로 구현했다. stable choice id 4개, `sado_final_phase_2_weakpoint_control_resolved`/`final_phase_2_weakpoint_control_resolved` common hook, 서하린/무명/청류문/천기록 압박 대응별 `seoharin_axis`·`qingliu_rebuild`·`mumyeong_salvation`·`successor_route`·`own_flow_choice`·`cheongirok_state`·`player_method` seed, Rust/Web generated preview artifact, Python/Rust/Web bundle registry 테스트를 갱신했다. combat resolver, HP 숫자전, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
79. 무협 `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice 완료: 사도 최종전 3페이즈 계산식 밖을 기존 encounter schema로 구현했다. stable choice id 5개, `sado_final_phase_3_outside_calculation_resolved`/`final_phase_3_outside_calculation_resolved`/`final_combat_result_battle_victory_seeded` common hook, true/meaningful/corrupted boss resolution candidate seed, 서하린/무명/청류문/장부 빈칸/천기록 계산별 `seoharin_axis`·`mumyeong_salvation`·`successor_route`·`own_flow_choice`·`qingliu_rebuild`·`evidence_state`·`cheongirok_state`·`player_method` seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web bundle registry 테스트를 갱신했다. combat resolver, HP 숫자전, 보스 결산 출력기, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
80. 무협 `wuxia_boss_resolution` preview runtime slice 완료: 보스 결산을 기존 encounter schema로 구현했다. stable choice id 5개, `boss_resolution_resolved`/`final_result_priority_applied_seeded` common hook, true/meaningful/incomplete/mumyeong-unsaved/corrupted route seed, 후속 epilogue candidate seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web bundle registry 테스트를 갱신했다. final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
81. 무협 `wuxia_mumyeong_resolution` preview runtime slice 완료: 무명 결산을 기존 encounter schema로 구현했다. stable choice id 6개, `mumyeong_resolution_resolved` common hook, own-flow/relational/incomplete/end-of-stolen-forms/black-serpent-successor/corrupted-unsaved route seed, 후속 epilogue candidate seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web bundle registry 테스트를 갱신했다. final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
82. 무협 `wuxia_cheongirok_resolution` preview runtime slice 완료: 서하린/청류문 결산 뒤 천기록 마지막 장 route seed bridge를 기존 encounter schema로 구현했다. stable choice id 5개, `cheongirok_resolution_resolved` common hook, high-use-not-corruption/blank true-route/ledger-aligned corruption/low-use silence/player-method reflection seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
83. 무협 `wuxia_black_serpent_aftermath` preview runtime slice 완료: 천기록 결산 뒤 흑사방 aftermath seed bridge를 기존 encounter schema로 구현했다. stable choice id 5개, `black_serpent_aftermath_resolved` common hook, broken serpent/banner residue/alliance silence/southern market rumor/true-route suppression seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
84. 무협 final epilogue renderer contract docs-only handoff 완료: Notion `최종장 결산 라우팅 마스터`, `08. 엔딩과 후일담 연결`, `사도 최종전 상태값 사전`과 지금까지의 final-entry seed bridge를 대조했다. 추가 seed bridge 없이 `wuxia_final_epilogue_renderer_contract` implementation slice를 다음 후보로 둘 수 있다고 결정했고, Rust GameCore가 seed consumption/final_result_priority/suppress/card ordering을 소유하며 Web Storybook과 SuperLightTUI는 core 결과를 표시만 한다는 boundary를 `docs/design/Wuxia_Final_State_Routing.md`에 기록했다. runtime YAML/Rust/Web/generated artifact, final epilogue output schema, return/settlement schema, combat resolver, reward/ability schema, relation/debt/faction ledger, `item_unpriced_wooden_sword` payout, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
85. 무협 `wuxia_final_epilogue_renderer_contract` runtime implementation slice 완료: 최종 후일담 출력기를 새 `ScenePage` mode가 아니라 structured `ScenePage.body_blocks` convention으로 구현했다. Rust GameCore가 final result priority, candidate seed consumption, suppress, card ordering을 소유하고 `epilogue_result`/`epilogue_card`/`epilogue_suppressed` block을 출력한다. Web Storybook과 SuperLightTUI는 core block을 표시만 하며 후일담 카드를 재계산하지 않는다. preview ending과 Rust/Web generated bundle, Rust core direct-state test, WASM JSON contract, SuperLightTUI smoke, Web Storybook pass-through test를 갱신했다. combat resolver, HP 숫자전, return/settlement schema, reward/ability schema, relation/debt/faction ledger, `item_unpriced_wooden_sword` payout, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
86. 무협 final epilogue UX/playtest follow-up 완료: Web Storybook이 core-owned `epilogue_*` body block을 양피지형 후일담 section으로 렌더링하고, prose를 즉시 노출하며 metadata는 접힌 contract 기록으로 보관한다. SuperLightTUI는 epilogue heading/compact line/wrap을 적용해 40-line snapshot 안에서 핵심 카드가 밀려나지 않게 했다. local ignored `web/src/core/wasm-pkg/`가 stale이면 Web playtest가 final epilogue block을 못 받는 문제를 확인했고, local WASM package 재빌드 후 fresh `localhost:8768` run에서 scripted final route가 10개 epilogue block과 빈 action choice로 끝나는 것을 검증했다. combat resolver, HP 숫자전, return/settlement schema, reward/ability schema, relation/debt/faction ledger, `item_unpriced_wooden_sword` payout, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
87. 무협 return/settlement contract docs-only handoff 완료: Notion `가지 말라는 말`, `08. 엔딩과 후일담 연결`, `11. True Ending 단일 루트`, `사도 최종전`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`을 대조해 다음 contract surface를 return/settlement로 결정했다. 다음 runtime 후보는 `wuxia_seoharin_unsaid_stay`이며, 첫 구현은 귀환 엔딩 자체가 아니라 서하린 late relationship trigger로 `final_return_settlement_contract_seeded`와 return/settlement/corruption 후보 seed만 기존 flags/clues/log/presentation hook으로 남긴다. full modern return ending, return/settlement save/archive schema, relation/debt/faction ledger, reward/ability schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
88. 무협 `wuxia_seoharin_unsaid_stay` preview runtime slice 완료: Notion `가지 말라는 말`과 return/settlement contract handoff를 대조해 서하린 late relationship trigger를 기존 encounter schema로 구현했다. `wuxia_seoharin_qingliu_resolution` 뒤, `wuxia_cheongirok_resolution` 앞에 삽입했고, stable choice id 4개, `seoharin_unsaid_stay_resolved`/`final_return_settlement_contract_seeded` common hook, return/settlement/uncertainty/evasion 후보 seed, Rust/Web generated preview artifact, Python/Rust/WASM/terminal/Web default bundle registry 테스트를 갱신했다. full modern return ending, return/settlement save/archive schema, relation/debt/faction ledger, reward/ability schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
89. 무협 `wuxia_return_settlement_epilogue_contract` runtime slice 완료: `return_settlement_followup_handoff`에서 Notion return/settlement/afterword sources를 다시 대조한 뒤, 새 ending enum/schema 없이 기존 final epilogue body block consumer에 return/settlement branch card group을 추가했다. `epilogue_wuxia_returned_commute`, `epilogue_wuxia_qingliu_settlement`, `epilogue_wuxia_empty_place_kept_open`, `epilogue_wuxia_closed_gate_risk`를 Rust GameCore가 seed consumption/card ordering/suppress로 소유하고 Web Storybook/SuperLightTUI는 표시만 한다. full modern return scene, save/archive schema, relation/debt/faction ledger, reward/ability schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal, legacy office bundle과 `escape-office` save/localStorage key는 변경하지 않았다.
90. 무협 `return_settlement_epilogue_followup_handoff` docs-only handoff 완료: return/settlement branch runtime 이후 Notion final routing master, final battle/state glossary, ending-afterword links, epilogue DB rows, side debt, reward, main ending structure를 다시 대조했고 다음 runtime 후보를 `wuxia_battle_loss_epilogue_contract`로 결정했다. 다음 구현은 기존 Rust final epilogue body block consumer가 `final_combat_result_battle_loss_seeded`를 소비해 loss afterword bundle을 출력하는 것이다. full final battle container, combat resolver, HP 숫자전, archive/save schema, relation/debt/faction ledger, reward/ability schema는 변경하지 않았다.
91. 무협 `wuxia_battle_loss_epilogue_contract` runtime slice 완료: 기존 Rust final epilogue body block consumer가 `final_combat_result_battle_loss_seeded`를 소비해 Notion battle-loss afterword bundle을 출력한다. `epilogue_boss_black_serpent_banner`, `epilogue_wuxia_southern_market_rumor`, `epilogue_mumyeong_black_serpent_new_scale`, `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page`를 `epilogue_card`로 출력하고, optimistic victory cards는 `suppressed_by: battle_loss`로 억제한다. final epilogue YAML gate는 victory-only required flag를 제거하고 Rust consumer precondition이 victory/loss 승인 여부를 소유한다. full final battle container, combat resolver, HP 숫자전, playable defeat route, archive/save schema, relation/debt/faction ledger, reward/ability schema는 변경하지 않았다.
92. 무협 `wuxia_battle_loss_epilogue_followup_handoff` docs-only handoff 완료: battle-loss runtime evidence 이후 Notion `최종장 결산 라우팅 마스터`, `사도 최종전`, `사도 최종전 상태값 사전`, `08. 엔딩과 후일담 연결`, side-debt/reward/faction/ending/archive sources를 재대조했고, full final battle container, broader corruption/closed-gate, reward/ability, relation/debt/faction ledger, archive/save, playable defeat-route bridge를 비교했다. 다음 runtime 후보는 `wuxia_final_state_canonical_collapse_contract`이며, 기존 `final_*_seeded` local flags를 canonical final state labels로 접는 Rust GameCore-owned audit contract를 먼저 구현한다.
93. 무협 `wuxia_final_state_canonical_collapse_contract` runtime slice 완료: 기존 Rust final epilogue body block consumer가 `epilogue_result` 직후 `epilogue_state_audit`를 출력한다. audit block은 `combat_result`, `boss_resolution_route`, `evidence_state`, `network_handling`, `pressure_state`, `seoharin_axis`, `qingliu_rebuild`, `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `truth_state`, `cheongirok_state`, `player_method`, `item_logs`를 기존 `final_*_seeded` local flags에서 collapse하고, missing/ambiguous/derived-by-priority 상태를 renderer-neutral text contract로 보고한다. Rust route parity와 WASM JSON boundary test를 추가했다. full final battle container, combat resolver, HP 숫자전, playable defeat route, archive/save schema, relation/debt/faction ledger, reward/ability schema는 변경하지 않았다.

현재 최우선 남은 작업:

1. 무협 storypack preview/main의 다음 작업은 `wuxia_final_state_canonical_collapse_followup_handoff` docs-only contract slice다. `wuxia_final_state_canonical_collapse_contract` runtime이 기존 `final_*_seeded` local flags를 Rust-owned `epilogue_state_audit` block으로 collapse했으므로, 이제 full final battle container, playable defeat-route bridge, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface 중 무엇을 다음에 열지 비교한다. `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, final epilogue UX/playtest follow-up, return/settlement contract handoff, return/settlement follow-up handoff, return/settlement epilogue contract runtime, return/settlement epilogue follow-up handoff, battle-loss epilogue contract runtime, battle-loss epilogue follow-up handoff, final-state canonical collapse runtime은 완료됐고, `docs/design/Wuxia_Final_State_Routing.md`가 canonical final inputs/result priority/alias policy/final epilogue seed-consumption contract와 return/settlement/battle-loss/final-state-collapse contract를 소유한다.
   - 현재 Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
   - `escape from the office` / office isolation 계열은 legacy/parity content로 유지한다.
   - machine-readable storypack DB, preview mode 결정, `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, boss follow-up handoff, failed-aid follow-up handoff, Web/default 이구학지 start/save wiring, terminal default 이구학지 bundle 선택은 완료했다.
   - copy-style reveal은 `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남겼다.
   - orthodox style trace는 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete` hook을 남겼다.
   - midgame reunion은 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook을 남겼다.
   - boss first appearance는 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook을 남겼다.
   - mumyeong request for aid는 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `rejected_aid_letters_read`, `inn_rumor_thread_followed`, `seoharin_failed_aid_question_asked`, `failed_aid_record_kept_unshown`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid` hook과 `rejected_aid_letter_fragment` item을 남겼다.
   - mumyeong awakening은 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready` hook을 남겼다.
   - `wuxia_qingliu_attack_after_war` 구현은 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`를 요구하고, `qingliu_attack_after_war_resolved`로 반복을 막는다.
   - stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`로 고정한다.
   - common hook은 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_destroys_orthodox_sect` 구현은 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`를 요구하고, `mumyeong_destroys_orthodox_sect_resolved`로 반복을 막는다.
   - stable choice id는 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`로 고정한다.
   - common hook은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_boss_recruits_mumyeong` 구현은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`를 요구하고, `boss_recruits_mumyeong_resolved`로 반복을 막는다.
   - stable choice id는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`로 고정한다.
   - common hook은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_departure_truth_summary` 구현은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`를 요구하고, `mumyeong_departure_truth_summary_resolved`로 반복을 막는다.
   - stable choice id는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`로 고정한다.
   - common hook은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_seoharin_empty_place` 구현은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고, `seoharin_empty_place_resolved`로 반복을 막는다.
   - stable choice id는 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`로 고정한다.
   - common hook은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`다.
   - `preview launcher/UI wiring`은 이미 구현했으므로 후속 slice에서 다시 구현하지 않는다.
   - route opener 후속도 faction/route graph schema를 열지 않고 flags/clues/log/presentation으로만 남긴다.
   - `yageunmong_pack`은 docs/data 후보로 반영됐지만 기본 office runtime을 대체하지 않는다. 야근몽 runtime은 별도 preview 후보로만 연다.
   - legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 계속 바꾸지 않는다.
   - 천외편린/각성편린 3택 성장 schema, relation/debt/faction/companion schema는 별도 검증 전까지 열지 않는다.
   - 보스 첫 등장, 무명의 도움 요청 실패 기록, 무명의 각성, 청류문 흔적 조사, 현악문 멸문 consequence trace, 보스 스카웃 trace, 무명 이탈 진실 sealed summary, 서하린 empty-place bridge는 열었다.
   - `wuxia_seoharin_left_meal`은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고, `seoharin_left_meal_resolved`로 반복을 막는다.
   - stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
   - common hook은 `seoharin_left_meal_resolved`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`이며 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`를 남긴다.
   - `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`은 preview/main runtime 구현 완료다. full return/settlement ending, return/settlement save/archive schema, full combat resolver 후보는 보류한다.
   - Rust GameCore / `ScenePage` / WASM JSON boundary 책임 분리와 renderer-neutral 원칙을 유지한다.

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
8. Web player start/save UX first slice 후속: save JSON export/import, settings/reduce-motion UI, 오늘의 seed는 별도 승격 전까지 열지 않는다.
9. 여러 히든 현실 보물
10. 전투 시스템 후속 slice는 `docs/design/Basic_Combat_Action_Model.md`의 action taxonomy를 기준으로 `supply_closet_auto_brawl`와 `wuxia_cheongryu_chore_sparring` 이후에도 반복 가치가 확인될 때만 presentation metadata 정리 또는 Rust combat resolver로 승격한다.
11. 무협 storypack 후속: 정파/사파/천기·귀환 opener(`wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`), deferred-offer card `wuxia_wounded_shelter_dawn_offers`, common midgame bridge `wuxia_mumyeong_first_sighting`, rival first confrontation `wuxia_mumyeong_first_confrontation`, copy-style reveal `wuxia_mumyeong_copy_style_reveal`, orthodox style trace `wuxia_mumyeong_reads_orthodox_style`, midgame reunion `wuxia_mumyeong_midgame_reunion`, boss first appearance `wuxia_boss_first_appearance`, Mumyeong aid request `wuxia_mumyeong_request_for_aid`, Mumyeong awakening `wuxia_mumyeong_awakening`, Qingliu attack trace `wuxia_qingliu_attack_after_war`, Hyeonakmun consequence trace `wuxia_mumyeong_destroys_orthodox_sect`, boss recruitment trace `wuxia_boss_recruits_mumyeong`, sealed departure truth summary `wuxia_mumyeong_departure_truth_summary`, Seo Harin empty-place bridge `wuxia_seoharin_empty_place`, Seo Harin left-meal bridge `wuxia_seoharin_left_meal`, Sado final phase 1 price-tag/ledger bridge `wuxia_sado_final_phase_1_price_tag`, Sado final phase 2 weakpoint-control bridge `wuxia_sado_final_phase_2_weakpoint_control`, Sado final phase 3 outside-calculation bridge `wuxia_sado_final_phase_3_outside_calculation`, boss resolution route seed bridge `wuxia_boss_resolution`, Mumyeong resolution route seed bridge `wuxia_mumyeong_resolution`, Seo Harin/Qingliu resolution route seed bridge `wuxia_seoharin_qingliu_resolution`, Seo Harin return/settlement trigger `wuxia_seoharin_unsaid_stay`, Cheonggi Record resolution route seed bridge `wuxia_cheongirok_resolution`, Black Serpent aftermath seed bridge `wuxia_black_serpent_aftermath`, final epilogue seed consumer `wuxia_final_epilogue_renderer_contract`, return/settlement branch consumer `wuxia_return_settlement_epilogue_contract`, battle-loss epilogue branch consumer `wuxia_battle_loss_epilogue_contract`, battle-loss epilogue follow-up handoff, final-state canonical collapse runtime까지 구현/검증 완료했다. 다음 후보는 `wuxia_final_state_canonical_collapse_followup_handoff` docs-only contract slice다.
12. 천외편린/각성편린 3택 reward/ability schema는 schema-less bridge가 충분히 검증된 뒤 별도 slice로 검토한다.
13. 야근몽 storypack preview 후속: `yageunmong_late_night_desk_awake` 또는 각성편린 3택 preview를 별도 storypack preview로 열지 결정한다.

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

1. 다음 무협 storypack preview/main 작업은 `wuxia_final_state_canonical_collapse_followup_handoff` docs-only contract slice다. 이 slice는 final-state audit runtime evidence 이후 full final battle container, playable defeat-route bridge, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface 중 무엇을 먼저 열지 비교한다. `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`, `wuxia_battle_loss_epilogue_contract`, `wuxia_final_state_canonical_collapse_contract`, final epilogue UX/playtest follow-up, return/settlement contract handoff, return/settlement epilogue follow-up handoff, battle-loss epilogue follow-up handoff는 final state routing contract 기준으로 구현/검증 완료했다.
   - `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`는 이미 이구학지 runtime bundle에 구현되어 있다.
   - `wuxia_final_epilogue_renderer_contract_handoff`는 추가 seed bridge 없이 implementation slice를 열 수 있다고 결정했고, 해당 slice는 structured `ScenePage.body_blocks` convention으로 구현 완료됐다. Rust GameCore가 candidate seed consumption, `final_result_priority`, suppress, card ordering을 소유하고 Web Storybook/SuperLightTUI는 core 결과를 표시만 한다.
   - Web/terminal default storypack은 이구학지이며, terminal도 `--scene content` 기본 실행에서 같은 bundle을 사용한다. `--storypack-preview wuxia_jianghu_pack`는 명시적 동일 경로로 남겼고, Web의 별도 preview launcher는 이구학지가 기본이 되면서 목록에서 비워 두었다.
   - 이구학지 runtime은 계속 `storypack_preview` 계열 bundle metadata와 `default_location: wuxia_commute_rift` 시작점을 유지하되, Web player에서는 이를 `storypack_main`으로 감싼 default bundle JSON으로 사용한다.
   - `wuxia_mumyeong_copy_style_reveal` 구현으로 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook이 생겼다.
   - `wuxia_mumyeong_reads_orthodox_style` 구현으로 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook이 생겼다.
   - `wuxia_mumyeong_midgame_reunion` 구현으로 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook이 생겼다.
   - `wuxia_boss_first_appearance` 구현으로 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook이 생겼다.
   - `wuxia_mumyeong_request_for_aid` 구현으로 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `rejected_aid_letters_read`, `inn_rumor_thread_followed`, `seoharin_failed_aid_question_asked`, `failed_aid_record_kept_unshown`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid` hook과 `rejected_aid_letter_fragment` item이 생겼다.
   - `wuxia_mumyeong_awakening` 구현으로 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready` hook이 생겼다.
   - `wuxia_mumyeong_followup_after_awakening` handoff에서 정한 `wuxia_qingliu_attack_after_war`는 preview runtime 구현 완료다. 구현 범위는 full flashback이 아니라 현악문/복호금쇄수 흔적 조사였다.
   - required flags는 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`다.
   - stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`다.
   - `wuxia_qingliu_attack_after_war_followup` handoff에서 정한 `wuxia_mumyeong_destroys_orthodox_sect`는 preview runtime 구현 완료다. 구현 범위는 현악문 멸문 전투가 아니라 빈 현악문 산문/기록/풍문을 확인하는 trace encounter였다.
   - required flags는 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`다.
   - stable choice id는 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`다.
   - common hook은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_destroys_orthodox_sect_followup` handoff에서 정한 `wuxia_boss_recruits_mumyeong`는 preview runtime 구현 완료다. 구현 범위는 구원이나 최종 결산이 아니라 흑사방 보스의 recruitment trace였다.
   - required flags는 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`다.
   - stable choice id는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`다.
   - common hook은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_boss_recruits_mumyeong_followup` handoff에서 정한 `wuxia_mumyeong_departure_truth_summary`는 preview runtime 구현 완료다.
   - required flags는 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`다.
   - stable choice id는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`다.
   - common hook은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_mumyeong_departure_truth_summary`, `speaker: 천기록`, `layout: sealed_departure_truth_summary`, stable terms `[무명, 서하린, 현악문, 흑사방주]`다.
   - `wuxia_mumyeong_departure_truth_summary_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정했다.
   - required flags는 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`다.
   - stable choice id는 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`다.
   - common hook은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - `wuxia_seoharin_empty_place_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정했고, 해당 slice는 preview runtime 구현 완료다.
   - required flags는 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`다.
   - stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
   - common hook은 `seoharin_left_meal_resolved`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`이며, 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`를 남긴다.
   - `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`는 preview/main runtime 구현 완료다. ledger/evidence/pressure/item-log seed, weakpoint/final-method seed, outside-calculation result candidate seed, boss-resolution route seed, Mumyeong-resolution route seed, Seo Harin/Qingliu epilogue candidate seed, Seo Harin return/settlement trigger seed, Cheonggi Record last-page seed, Black Serpent aftermath seed, final epilogue card output을 Rust GameCore-owned path로 연결했다. `wuxia_sado_final_battle`, full return/settlement, 남은 final/late companion 후보는 계속 보류한다.
   - seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, boss combat/final resolution, 서하린에게 진실 전달, 무명 구원 확정, `told_seoharin_truth`, 무명/보스 결산, epilogue/return system은 바로 열지 않는다.
   - legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 바꾸지 않는다.
   - Rust GameCore / `ScenePage` / WASM JSON boundary가 가진 gameplay truth를 renderer가 재계산하지 않는다.
   - route graph/faction reputation/debt ledger/relation schema, return system, 천기록 정체 reveal, 천외편린 3택 성장/reward/ability schema는 아직 열지 않고, 필요한 경우 `flags`/`clues`/`log`/`presentation` hook으로만 future work를 남긴다.
2. 야근몽 runtime 후보는 `yageunmong_late_night_desk_awake` 또는 각성편린 3택 preview로만 열고, 기본 office bundle을 자동 rewrite하지 않는다.
3. 실제 음악/SFX asset과 soundtrack은 저작권/라이선스 정책이 정리되기 전까지 열지 않는다.
