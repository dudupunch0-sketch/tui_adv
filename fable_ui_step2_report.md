# Fable UI Step 2 Report

Date: 2026-07-11
Branch: `codex/fable-ui-step1-wp0`

## Completed work packages

- WP-0: docs contract now follows the final epilogue module boundary.
- WP-U1 to WP-U8: duplicate titles, Korean labels, empty-scene restart,
  mobile choice metadata, abandon confirmation, useful dock content, action
  kinds, and visual captions are covered.
- WP-V1: ink storybook tokens and Noto Serif KR typography are active.
- WP-V2: code-authored inline SVG ink scenes replace the visual catalog; the
  complete planned wuxia scene table and deterministic location variants are covered.
- WP-V3: combat uses the core scene information instead of fabricated combat UI.
- WP-V4 to WP-V7: footnote status strip, danger thread, ink choices, and one
  bottom-sheet information drawer are active.
- WP-V8: the title screen uses an authored ink scene and no external image.
- WP-V9: transition visuals use ink wash, seal flash, and drying-ink effects.
- WP-V10: the active Mobile Ink Storybook UI contract was checked against the
  implementation and its automated QA flow was updated for the folded start
  menu and the footnote drawer interaction.

## Intentional deviations

- WP-V4, WP-V5, and WP-V7 were committed together because the incoming active
  UI contract made the footnote strip and the unified drawer one inseparable
  renderer change.
- The active contract/documentation update arrived through the merged upstream
  revision, so this branch implements against it instead of duplicating docs edits.

## Verification

- `cd web && npm test && npx tsc --noEmit && npm run build:wasm` (39 tests)
- `./.venv/bin/python -m pytest tests/ -q` (97 passed)
- `cargo test --workspace`
- `npm run qa:storybook:visual -- --base-url http://127.0.0.1:4173/ --out-dir /tmp/dudu/storybook-visual-qa --require-wasm`
  (390x844, 414x896, 800x1440, 810x1644, and 1440x1000 passed, including
  choice click, number-key action, and the footnote drawer)
