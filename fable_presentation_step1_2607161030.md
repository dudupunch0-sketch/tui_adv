# Presentation Polish, Step 1 — Collapse Ink Vignette + Typewriter Coverage for Event Stages (Plan by Fable, implementation by codex)

Date: 2026-07-16
Baseline: `main` HEAD `482918c` ("feat: add event stage story flow model (#144)")
Predecessors: #143 (game-frame redesign: 3-region layout, typewriter, check dramatization), #144 (event stage content model, `ScenePage.content_stream` ordered flow).

This document is a complete implementation directive. Execute work packages **in order, one at a time**; after each WP run verification, make one commit, then continue. Record status and deviations in `fable_presentation_step2_report.md` (repo root).

Build environment: `cargo` / `gh` are WSL-only (`AGENTS.md`). Web commands run in `web/`.

---

## 0. Scope and background

Three tracks, all web-renderer-only (no engine changes expected; WP-W3 may reveal a core question — report it, do not fix core unilaterally):

- **A — Collapse-gate ink vignette.** The death's-door screen (`data-story-phase="collapse"`) currently has placeholder styling (faint red gradient). It must read as "the record itself is failing", not "slightly pink page".
- **B — Typewriter regression on event-stage pages.** `web/src/ui/storybook/typewriter.ts` targets `.storybook-body`, but #144's ordered flow renders `.story-flow--ordered` **without** `.storybook-body`. Migrated wuxia events — the flagship content — silently lose the typewriter effect. PC/legacy pages still type; event-stage pages do not.
- **C — Continue/empty-choice audit in the ordered stream.** `renderContentItem` routes `kind === 'continue'` into `renderChoices(page, item.actions ?? [])`; with an empty `actions` array on a non-ending page this renders "현재 실행할 수 있는 행동이 없다" plus a "처음 화면으로 돌아간다" button **in the middle of a live event**. Also `'continue'` is missing from the `SceneContentKind` union, and `page.blocked_actions` are appended to every choice nav.

### 0.1 Hard invariants

1. Web-only: do not modify crates/, YAML content, or generated bundles in this plan.
2. Existing CSS custom properties only (`--ink`, `--ink-body`, `--ink-faded`, `--ink-wash`, `--seal-red`, `--seal-red-lit`, `--jade`, `--gold-leaf`, `--paper*`, `--line-soft`, `--line-hard`). No new color literals except rgba() derived from those exact hues.
3. Class names and data attributes referenced by tests/QA stay: `data-story-phase="collapse"`, `.storybook-choices[data-region="choices"]`, `.choice-row[data-action-id]`, `.game-viewport`, `.storybook-hud`, all REQUIRED selectors in `web/scripts/storybook-reference-qa.mjs`.
4. Any new overlay: `pointer-events: none`, z-index **below** the drawer sheet (`.storybook-dock`, z-index 8) and `.storybook-runtime-warning` (z-index 10). Choices must stay clickable and fully legible.
5. Reduced motion: the global rule zeroes `animation-duration` and `animation-delay` inside `.storybook-shell`. Design every animation so its 100% keyframe (with `animation-fill-mode: both`) is the correct final resting state. Looping animations must look acceptable frozen at 100%.
6. Typewriter must remain OFF when `resolveMotionMode(...) !== 'normal'` (existing gate in `main.ts` — do not weaken it).
7. No new npm dependencies. No markup changes that alter action ids or keyboard-number behavior.

### 0.2 Verification (after every WP)

```bash
cd web
npx vitest run          # all green
npx tsc --noEmit
npm run build
```

Once after all WPs, with a dev server running (`npm run dev`, needs wasm-pkg built — in WSL: `wasm-pack build crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg`):

```bash
npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173/ --out-dir /tmp/tui-adv-presentation-qa
```

All 5 viewports must pass. (#144 skipped this — do not skip it.)

### 0.3 Commit convention

One commit per WP: `feat(web): <desc> [WP-W1]` / `fix(web): <desc> [WP-W2|W3]`.

---

## 1. WP-W1 — Collapse-gate ink vignette (Track A)

**Design intent.** Reference grammar (`docs/reference/Life_in_Adventure_Play_Reference.md` §3.7) is a red vignette closing in at death. Our translation is ink-wash, not blood: **wet ink bleeding inward from the frame edges, tinted seal-red**, as if the 천기록's pages are soaking through. Edges darken; the center column (narrative + the two choices 기사회생/안식) stays fully legible.

**Implementation (CSS-only, `web/src/styles/storybook.css`):**

1. Vignette layer on `.storybook-shell[data-story-phase="collapse"] .game-viewport::after` (the viewport already has `position: relative`; if a pseudo-element conflicts, add a dedicated `.collapse-vignette` div in `render.ts` guarded by phase — prefer the pseudo-element):
   - `position: absolute; inset: 0; pointer-events: none; z-index: 3;`
   - Layered backgrounds: a base `radial-gradient(ellipse at 50% 45%, transparent 52%, rgba(158, 60, 63, 0.26) 80%, rgba(32, 20, 22, 0.5) 100%)` plus 2–3 smaller offset radial gradients near corners (rgba of --seal-red / --ink at 0.10–0.18) to break the perfect ellipse into an irregular ink-blot silhouette.
   - NOTE: the viewport scrolls. `::after` on a scroll container scrolls with content — if the vignette must hug the visible frame, attach it to `.storybook-shell[data-story-phase="collapse"]::after` instead (shell already has a decorative `::after` — in that case use a new child div `.collapse-vignette` appended in `render.ts` when phase is collapse, absolutely positioned over the shell grid rows). Choose whichever keeps the vignette fixed to the frame while content scrolls; state the choice in the report.
2. Entrance: vignette breathes in once (~600ms, opacity 0→1 with a slight scale 1.06→1), then a slow pulse loop (~3s, opacity 0.85↔1.0). Both `animation-fill-mode: both`; the 100% frame is the fully-visible resting state (invariant 5).
3. Page desaturation: `.storybook-shell[data-story-phase="collapse"] .game-viewport { filter: saturate(0.8); }`.
4. Top bar accent: in collapse phase, tint the top bar's border-bottom to `rgba(158, 60, 63, 0.55)`.
5. Choice differentiation (border/background only, same font size — never vary typography between states): the 기사회생 choice card gets a `--gold-leaf` accent border; 안식 gets a faded-ink treatment (lower-contrast border, slightly transparent background). Target them structurally: the collapse encounter renders exactly two `.choice-row` buttons; use `.storybook-page[data-story-phase="collapse"] .choice-row:first-of-type` / `:last-of-type` (list order is content-defined: 기사회생 first, 안식 second in `wuxia_collapse_gate`).
6. Keep the existing `.storybook-page[data-story-phase="collapse"]` gradient as the base layer; strengthen only via the layers above.

**Tests:** extend `render.test.ts` collapse fixture assertions only if you add markup (e.g. `.collapse-vignette`). CSS-only changes need no new unit tests.

**Visual check:** render a collapse fixture (see the existing `forces data-story-phase="collapse"` test for the shape) via the dev server + Playwright screenshot at 390×844 and 1440×1000, attach paths in the report. In a live run you can also reach it by dropping health to 0.

## 2. WP-W2 — Typewriter coverage for the ordered content stream (Track B)

**Goal:** event-stage pages (`content_stream` present) get the same typewriter presentation as legacy pages: body text types in with the ink caret, tap-to-complete, choices fade in after the text settles.

**Implementation (`web/src/ui/storybook/typewriter.ts` — keep the exported API identical):**

1. Target container: change `shell.querySelector('.storybook-body')` to cover both layouts, e.g. `shell.querySelector('.storybook-body') ?? shell.querySelector('.story-flow--ordered')`.
2. Exclusions in `collectTextNodes` (extend the existing `closest()` check): skip text inside `.check-resolution` (already), and additionally `.storybook-choices` (choice labels must never type — they are inside the ordered flow now), `.story-illustration`, and `.story-result-log` stays included (unchanged).
3. Choices reveal: `.storybook-choices` may now sit **inside** the typed container and there may be more than one nav in the stream. `revealChoices` must select **all** `.storybook-choices` in the shell (`querySelectorAll`) and set the attribute on each. The CSS (`[data-reveal="pending"]` opacity 0) already applies per-nav.
4. The caret must not be appended inside excluded subtrees — current logic appends to the typed node's parent, which is safe once choice text is excluded.
5. Skip behavior unchanged: pointerdown on `.game-viewport` completes instantly (capture-phase, swallowed). Verify tapping during typing does not activate a choice underneath.

**Tests (vitest, jsdom):** the module is DOM-driven; add a small unit test file `typewriter.test.ts`: build a container with `.story-flow--ordered` including a `.storybook-choices` nav and a narration block; run `startTypewriter` with `enabled: false` → choices get `data-reveal="settled"`; with `enabled: true` → choices start `pending`, narration text is emptied, and `finish()` restores full text and sets `done`. (Use fake timers or call `finish()` directly — do not depend on real intervals.)

**Motion-on end-to-end check (report evidence, Playwright with `reducedMotion: 'no-preference'`):** start a new run, navigate to an event-stage page (migrated wuxia events use the ordered flow from turn 1), sample `.story-flow--ordered` text length at ~250ms and ~1100ms (must grow), tap the viewport (choices reveal `done`, full text restored). Mirror of the check used for #143 — see `fable_presentation_step2_report.md` template note.

## 3. WP-W3 — Continue-stage and stream choice-nav audit (Track C)

1. Add `'continue'` to the `SceneContentKind` union in `web/src/core/types.ts` (type-only; serialization unchanged).
2. In `renderContentItem`, when `kind === 'continue'` (or a choice item whose `actions` is empty on a **non-ending** page): render a single continue affordance instead of the empty-choice fallback — a `.choice-row`-styled button reusing the FIRST action in `page.actions` if the stream item's own list is empty, and if `page.actions` is also empty, fall back to the existing `renderEmptyChoiceRows` behavior unchanged. First **verify against core behavior**: reproduce an actual continue stage in a live run or core test fixture and check what `content_stream` carries in `actions` for it. If core always populates `actions` for continue stages, limit this WP to the type union + a regression test and say so in the report. Do NOT change core.
3. Blocked actions: render `page.blocked_actions` only in the nav belonging to the **choice** stage (`kind === 'choice'`), not in continue navs. If only one nav ever renders per page in practice, keep the code path but add a render test documenting it.
4. Tests: extend the ordered-flow test with (a) a continue item with one action → renders exactly one `.choice-row`, no "행동이 없다" text; (b) union accepts `'continue'` without a cast.

## 4. Out of scope

- Engine/content changes (crates/, YAML, bundles, exporter).
- New motion preferences or settings UI.
- Styling the NO IMAGE placeholder beyond what #144 shipped (intentional content-debt marker).
- Audio.

## 5. Final checklist

- [ ] One commit per WP, in order W1 → W2 → W3.
- [ ] `npx vitest run`, `npx tsc --noEmit`, `npm run build` green after each WP.
- [ ] `qa:storybook:visual` passes all 5 viewports at the end.
- [ ] Collapse screen screenshots (390 + 1440) attached/linked in the report; choices legible over the vignette; drawer opens above it.
- [ ] Motion-on typewriter evidence for an event-stage page (text grows, tap completes, choices reveal).
- [ ] Reduced-motion sanity: with `reducedMotion: 'reduce'`, collapse vignette is fully visible in its resting state and no text is hidden.
- [ ] Report written to `fable_presentation_step2_report.md`.
