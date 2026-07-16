# Gameloop 3 Post-Merge Refactor — RF4 보고서

작성일: 2026-07-16
기준 계획: `fable_gameloop3_refactor_step1_2607161557.md`
계획 baseline: `c33fc75` (`feat: implement Gameloop 3 story stages and progression (#155)`)
현재 브랜치: `codex/gameloop3-refactor-plan`

## 결과

RF1–RF3 리뷰 결함을 수정한 뒤 RF4 문서 closeout을 반영했다. Slice 3은 구현·review-fixed
상태이며, 다음 active work는 Event/Stage/ContentBlock 전환이다. 콘텐츠 YAML, 생성 bundle,
ledger, Notion, route topology, save/content schema version은 이 refactor에서 변경하지
않았다.

## 반영된 커밋

| WP | 커밋 | 반영 내용 |
|---|---|---|
| RF1 | `3230df5` | ending-state `train:` guard, ordered insight dedupe, core regression coverage |
| RF2 | `256aa4f` | `inventory_details` 부재 시 비대화형 item row fallback |
| RF3 | `2f39cd2` | narrow HUD grid allocation, 390/414 non-overlap QA assertions |
| RF4 | 이 보고서의 closeout 커밋 | Development Plan/Checklist/report 동기화 |

## RF1–RF3 변경 요약

- RF1은 `train:` dispatch보다 terminal-state guard를 먼저 적용하고, 저장/런타임의 중복
  insight ID를 첫 등장 순서의 ordered set으로 읽도록 했다. 중복 보너스와 ScenePage
  drawer 행은 한 번만 생성되며, gain-time idempotence와 기존 roll hash는 유지된다.
- RF2는 `inventory_details`가 있는 경우의 disclosure/use 동작을 유지하면서, 상세가
  없는 legacy/호환 입력에서는 label과 icon만 가진 비대화형 행을 표시한다. 이 fallback은
  `aria-expanded`, disclosure target을 내보내지 않는다.
- RF3는 390/414px에서 두 vital, 천기 진행 영역, 상세 버튼이 겹치지 않도록 narrow grid와
  bounding-box gate를 추가했다. 기존 renderer가 사용하는 `.story-progress-rail` selector를
  그대로 검증한다.

## 현재 세션에서 실행한 검증

모든 명령은 WSL의 `~/work/tui-adv`에서 실행했다.

```text
cargo test --workspace
  164 passed, 0 failed (escape-core/content 9, core_contract 32,
  event_stage 4, route_parity 23, escape-terminal 61, escape-wasm/json 35).
  기존 dead_code warning 1건(raw_glyphfx_wave)은 실패가 아니다.

.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py
  72 passed in 3.95s

python3 scripts/export_web_data.py --check
  web data is up to date

python3 scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
  storypack preview bundle is up to date (두 대상 모두 통과)

cd web && npx vitest run
  13 test files, 71 passed, 0 failed

cd web && npx tsc --noEmit
  passed

cd web && npm run build
  Vite production build passed

wasm-pack build crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg
  passed; generated package at web/src/core/wasm-pkg

cd web && npm run qa:storybook:visual -- \
  --base-url http://127.0.0.1:5173/ \
  --out-dir /tmp/tui-adv-gameloop3-refactor-qa \
  --require-wasm
  passed with WASM and no runtime warning

git diff --check
  passed after RF4 document edits

.venv/bin/pytest -q tests/test_docs_contract.py
  60 passed (RF4 document contract check)
```

## Visual QA evidence

The required five viewports passed: `390x844`, `414x896`, `800x1440`, `810x1644`, and
`1440x1000`. The QA report and screenshots are:

- Report: `/tmp/tui-adv-gameloop3-refactor-qa/visual-qa-report.json`
- `/tmp/tui-adv-gameloop3-refactor-qa/screenshots/390x844.png`
- `/tmp/tui-adv-gameloop3-refactor-qa/screenshots/414x896.png`
- `/tmp/tui-adv-gameloop3-refactor-qa/screenshots/800x1440.png`
- `/tmp/tui-adv-gameloop3-refactor-qa/screenshots/810x1644.png`
- `/tmp/tui-adv-gameloop3-refactor-qa/screenshots/1440x1000.png`

At 390px, the automated report recorded non-overlapping last-vital/progress and
last-vital/drawer rectangles, viewport-contained HUD controls, and visible content. The
same gate passed at 414px.

## Manual acceptance and deviations

- This session executed the automated five-viewport QA and its 390/414 geometry assertions.
  A separate human visual inspection of the PNGs, a full new-game manual flow, a motion-on
  vital change, and a reduced-motion pulse interaction were **not executed**; they are not
  claimed as complete here.
- The plan names `.story-progress-mini`, but the existing renderer, tests, CSS, and QA contract
  use `.story-progress-rail`. This is a naming deviation from the refactor plan, intentionally
  preserved to avoid breaking the established selector contract; no rename was introduced.
- RF5 (Notion runtime page and `idea_box/notion_sources.yml` reverse sync) remains a separate
  pending WP and is intentionally not included in this RF4 commit.
