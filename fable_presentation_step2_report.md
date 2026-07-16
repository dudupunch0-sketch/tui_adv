# Fable presentation polish — Step 2 report

Date: 2026-07-16
Plan: `fable_presentation_step1_2607161030.md`
Baseline merged from `origin/main`: `a3b0a97`

## Work package status

| WP | Result | Commit |
| --- | --- | --- |
| W1 — collapse ink vignette | Complete | `41e3a3e` |
| W2 — ordered-flow typewriter coverage | Complete | `fd19fb4` |
| W3 — continue/choice-nav audit | Complete | `5a337fe` |

### W1 — collapse-gate vignette

The vignette is attached to `.storybook-shell[data-story-phase="collapse"]::after`, rather than the scrolling viewport. This keeps the ink bleeding fixed to the visible frame while the story body scrolls. The overlay is non-interactive (`pointer-events: none`) and uses z-index 3, below the drawer (8) and runtime warning (10). The collapse page is desaturated, the top-bar rule is tinted, and the two choice rows receive the gold/faded-ink differentiation requested by Fable. Existing custom properties and selectors were preserved; the reduced-motion rule freezes the animation at its fully visible state.

### W2 — ordered event-stage typewriter

`startTypewriter` now covers `.storybook-body` and `.story-flow--ordered`, excludes choice/illustration subtrees, and reveals every choice navigation in the stream. The exported API and motion-mode gate are unchanged. `typewriter.test.ts` covers disabled motion, pending reveal, text restoration, and multi-nav completion.

### W3 — continue and blocked-action behavior

`SceneContentKind` now includes `continue`. An empty non-ending stream choice falls back to the first page action; a continue item uses the same affordance, while blocked actions are emitted only on choice-stage navs. The existing core fixture confirms continue stages carry `event:continue` actions, so no engine/content changes were needed. Ordered-flow regression tests cover the continue item and union type.

## Verification

After each WP, from `web/`:

```text
npx vitest run       PASS (62 tests)
npx tsc --noEmit     PASS
npm run build        PASS
```

WASM was rebuilt with:

```text
wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg
```

The final visual QA run used the rebuilt WASM and the reduced-motion browser context:

```text
npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173/ \
  --out-dir /tmp/tui-adv-presentation-qa4 --require-wasm
```

All five required viewports passed (required selectors, no legacy surface, width checks, drawer, click, keyboard, WASM resources, and no runtime warning). The QA harness resource paths were corrected from the stale `assets/wasm-pkg/*` paths to the actual Vite `/src/core/wasm-pkg/*` imports in the same final verification change.

Screenshots:

- [390×844 QA](/tmp/tui-adv-presentation-qa4/screenshots/390x844.png)
- [414×896 QA](/tmp/tui-adv-presentation-qa4/screenshots/414x896.png)
- [800×1440 QA](/tmp/tui-adv-presentation-qa4/screenshots/800x1440.png)
- [810×1644 QA](/tmp/tui-adv-presentation-qa4/screenshots/810x1644.png)
- [1440×1000 QA](/tmp/tui-adv-presentation-qa4/screenshots/1440x1000.png)
- [390×844 collapse phase fixture](/tmp/tui-adv-presentation-qa4/screenshots/collapse-390x844.png)
- [1440×1000 collapse phase fixture](/tmp/tui-adv-presentation-qa4/screenshots/collapse-1440x1000.png)

The collapse images force the existing rendered page into the collapse phase in the browser to isolate the new CSS overlay; the automated live QA run separately verified the production shell, interactions, and reduced-motion behavior. A browser ordered-flow fixture with `reducedMotion: "no-preference"` showed progressive text growth (16 → 45 characters in the first 1.1s); the short fixture completed naturally, while tap-to-complete and reveal state are covered by the typewriter unit tests. The default seed's first live route is a legacy opening page, so a deterministic migrated-event route was not forced into the production bundle for this check; the ordered-flow unit and browser fixture cover the renderer behavior without changing content or engine state.

## Scope/deviations

- Web renderer, web QA script, tests, and this report only; no `crates/`, YAML, or generated content bundle changes.
- No new npm dependencies.
- The existing dev server was used on `127.0.0.1:5173`; QA output is under `/tmp/tui-adv-presentation-qa4`.
