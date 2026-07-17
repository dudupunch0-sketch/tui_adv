# Plan — Event/Stage/ContentBlock Migration, Wave 3 (Finale + Collapse Gate) — Full Coverage

Author: Fable (plan/review). Implementer: codex.
Baseline: `origin/main` `b7413fc` (`fix(content): replace truncated/English illustration alts (#166)`).
Predecessors: `fable_eventstage_step1_2607171255.md` (Wave 1), `fable_eventstage_wave2_step1_2607171454.md`
(Wave 2) — same contract; where silent, those rules apply. Report file:
`fable_eventstage_wave3_step2_report.md` at repo root.

## 0. State and goal

30 of 44 encounters are staged. Wave 3 converts the remaining **14** — the finale arc plus the
collapse gate — reaching **44/44** and retiring the legacy fixed-slot page from wuxia play
entirely. A survey of the Wave 3 set found **zero checked choices**, so no `branch:` fixtures
are expected (state this in the report, as in Wave 2).

## 0.1 Hard invariants — identical to Wave 1 §0.1 / Wave 2 §0.1

Route-graph invariance, roll-hash byte identity, additive-only schema, no new action
prefixes/storage keys/deps, original Korean prose verbatim (sentence-boundary splits only),
office pack untouched, core-side cursor/branch decisions only. Additionally for Wave 3:
**do not touch** `web/public/assets/art/` or `artManifest.ts` (gemini's illustration track is
paused but still owns those files), and do not modify the final-epilogue renderer contract
(`epilogue_*` body blocks, `final_epilogue.rs`).

## 1. Scope — Wave 3 conversion set (14 encounters, this order)

Finale chain (flag-gated play order):

1. `wuxia_sado_final_phase_1_price_tag`
2. `wuxia_cheonoe_analysis_thread_phase1_bridge`
3. `wuxia_sado_final_phase_2_weakpoint_control`
4. `wuxia_sado_final_phase_3_outside_calculation`
5. `wuxia_sado_battle_loss_route_bridge`
6. `wuxia_boss_resolution`
7. `wuxia_mumyeong_resolution`
8. `wuxia_seoharin_qingliu_resolution`
9. `wuxia_seoharin_unsaid_stay`
10. `wuxia_cheongirok_resolution`
11. `wuxia_black_serpent_aftermath`
12. `wuxia_return_modern_commute_scene`
13. `wuxia_settlement_stay_scene`

Special (last, own WP with the design in §2.2):

14. `wuxia_collapse_gate`

If any encounter resists clean staging, skip, keep legacy, record why in the report — but the
expectation after this wave is 44/44 (or an explicit, justified exception list).

## 2. Wave-3-specific designs

### 2.1 Finale conversions (encounters 1–13)

Same pattern as Waves 1–2 (story → choice → per-choice ResultStage; `result_summary` +
verbatim outcome log; one `illustration` block per event reusing `presentation.visual_id`,
`placeholder: true` where unmapped, **complete one-line Korean scene-description `alt`** — the
Wave 2 review rejected truncated body prefixes and English placeholders; write alts like
"시장 입구에서 몽둥이를 든 흑사방 말단들과 대치하는 외지인"). Additions:

- These encounters seed final-state flags consumed by the final epilogue ending
  (`wuxia_final_epilogue_renderer_contract`). Seeding stays in outcomes (engine); never
  narrate flag names.
- `wuxia_cheongirok_resolution` may use `document` blocks for lines where the 천기록 itself
  writes/reacts (Wave 2 §2.3 rules); all other finale encounters use plain narration unless the
  legacy body actually quotes the record. Never `speaker: 천기록` on narration.
- When a resolution's outcome completes the final-epilogue conditions, the ending page may
  legitimately supersede that choice's ResultStage on the next view (engine order: pending
  ending wins). Author the ResultStage anyway (grammar contract); the action-result beat still
  shows the outcome logs. Verify no stuck state in tests: after such a choice, the next page is
  the ending and no action is silently rejected in a loop.

### 2.2 Collapse gate staging (encounter 14) — design decided, follow exactly

Investigated engine facts this design relies on (verify with tests, do not re-derive):

- `current_content_encounter` prefers the **active staged event** over the pending collapse
  gate (turn.rs — active_event_id branch precedes `collapse_gate_pending`), and
  `active_event_id` clears only at event end. Therefore the gate **never interrupts a staged
  event mid-cursor** — it claims the view after the current event finishes. No
  suspend/restore machinery is needed; the gate stages like any encounter.
- `storyPhase()` in render.ts checks `page.visual.kind === 'collapse_gate'` **before**
  `content_stream`, so `data-story-phase="collapse"` and the ink-vignette CSS survive staging
  unchanged. Do not alter that ordering.
- `current_content_ending` returns None while the gate is pending, and the death ending
  (`wuxia_death_rest`, `max_resources: {health: 0}`) fires only after 안식 sets the used_flag.

Conversion shape:

- `collapse_story` (StoryStage): the legacy body verbatim (붉게 번지는 획 서사) + the existing
  `wuxia_collapse_gate` illustration block (placeholder rules as usual).
- `collapse_choice` (ChoiceStage): the existing two choices, order preserved — 기사회생 first,
  안식 last (the `.storybook-choices li:first-child / li:last-child` styling contract from the
  slice-2 fix depends on this order).
- Per-choice ResultStages:
  - 기사회생 result: outcome log verbatim (health +40 revival narration). Play continues after
    the event ends.
  - 안식 result: author it for grammar completeness, but note in a YAML comment that the death
    ending supersedes it at next page build (the beat still shows the outcome logs). This is
    accepted, documented behavior — do not add engine changes to force the stage to display.
- Engine tests (core):
  - health 0 **mid-staged-event** → gate does not appear until the active event completes;
    after completion the gate's staged story stage is the current view.
  - gate staged flow: continue → choice; 기사회생 → its ResultStage renders (health > 0), event
    ends, normal play resumes, gate cannot re-trigger (used_flag).
  - 안식 → next page is the `wuxia_death_rest` ending; no action-rejection loop.
  - Existing collapse contract tests (core_contract/json_contract collapse coverage) keep
    passing unweakened.

### 2.3 Legacy-path retirement guard

After 44/44, add one guard test: every wuxia preview encounter has `event.stages`
(`encounters().all(|e| e.event.is_some())`). Do NOT delete the legacy `body`/`presentation`
fields or the legacy rendering path — terminal snapshots and office pack still use them; this
wave only guarantees wuxia content is fully staged.

## 3. Work packages

- **WP-C1** — Convert finale encounters 1–7. Exporter/fixtures `--write` + `--check`, wave3
  guard test (same shape as wave2: staged entry, choice-id parity, illustration presence).
- **WP-C2** — Convert encounters 8–13. Extend guard; §2.1 ending-supersede test.
- **WP-S1** — §2.2 collapse gate conversion + the four engine tests.
- **WP-C3** — §2.3 full-coverage guard test.
- **WP-W1** — Web sanity: five-viewport QA; verify collapse vignette + choice styling on the
  staged gate (if Chrome is still unavailable in WSL, defer explicitly — Fable covers at review).
- **WP-D1** — Report; Development_Plan active-work line (Event/Stage migration **complete**;
  next candidates listed, e.g. trait effects slice, usable-item content); Checklist.
- **WP-D2** — Notion closeout per standing rule 6: ledger sync base + Wave 3 note; live page 13
  edits deferred to Fable if Notion unreachable (record pending edits in the report).

## 4. Verification — same command set as Waves 1–2

cargo test --workspace / pytest (web_data_export + docs_contract) / exporter `--check` both
bundles / `cd web && npm test && npx tsc --noEmit && npm run build` / wasm-pack build /
five-viewport `qa:storybook:visual --require-wasm`.

Manual flow checklist (report evidence or defer explicitly):

- Collapse gate at 390px: vignette phase active on the staged gate page, 기사회생/안식 cards
  visually differentiated, 기사회생 revives and play continues, 안식 reaches the death ending
  with the restart button.
- One finale resolution: choice → beat → (ResultStage or legitimate ending supersede) with no
  stuck state.
- Save/reload inside a Wave 3 ResultStage restores the cursor.

## 5. Out of scope

Art assets/manifest (gemini), final-epilogue renderer internals, trait effects, usable-item
content, office pack, any schema change beyond what §2 requires (expected: none — Wave 3 should
need zero new fields).
