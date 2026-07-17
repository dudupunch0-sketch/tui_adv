# Plan — Event/Stage/ContentBlock Migration, Wave 2 (Midgame Chain)

Author: Fable (plan/review). Implementer: codex.
Baseline: `origin/main` `fb718ed` (`docs(sync): record Event/Stage Wave 1 review closeout (#163)`).
Predecessor: `fable_eventstage_step1_2607171255.md` (Wave 1, merged as #161) — same contract; this
document only states scope, deltas, and Wave-2-specific rules. Where silent, Wave 1 rules apply.
Design authority: `docs/design/Event_Stage_Content_Model.md`, `docs/dev/Data_Schema.md`.
Report file: `fable_eventstage_wave2_step2_report.md` at repo root.

## 0. State and goal

After Wave 1, 14 of 44 wuxia encounters are staged. Wave 2 converts the **midgame chain** (the
mumyeong/boss arc, fragment rewards, seoharin beats) — 16 encounters — so play is staged
continuously from the opening through the departure-truth summary. Wave 3 (final battle phases,
resolutions, epilogue bridges, ~13 encounters + the collapse gate decision) stays out of scope.

## 0.1 Hard invariants — identical to Wave 1 §0.1

Route-graph invariance (ids/costs/outcomes/checks/flags/clues/destinations byte-identical),
roll-hash byte identity, additive-only schema, no new action prefixes/storage keys/deps,
**original Korean prose moved verbatim** (sentence-boundary splits only), office pack untouched,
all branch/cursor decisions core-side.

## 1. Scope — Wave 2 conversion set (16 encounters, this order)

Mumyeong/boss midgame chain (gated sequence — convert in play order):

1. `wuxia_mumyeong_first_confrontation`
2. `wuxia_mumyeong_copy_style_reveal`
3. `wuxia_mumyeong_reads_orthodox_style`
4. `wuxia_mumyeong_midgame_reunion`
5. `wuxia_boss_first_appearance`
6. `wuxia_mumyeong_request_for_aid`
7. `wuxia_mumyeong_awakening`
8. `wuxia_qingliu_attack_after_war`
9. `wuxia_mumyeong_destroys_orthodox_sect`
10. `wuxia_boss_recruits_mumyeong`
11. `wuxia_mumyeong_departure_truth_summary`

Interleaved beats:

12. `wuxia_cheonggi_record_writing_sense` — the one true 천기록-intervention scene; see §2.3
13. `wuxia_cheonoe_pyeonrin_first_reward` — 1-of-3 fragment choice
14. `wuxia_cheonoe_pyeonrin_second_reward` — 1-of-3 fragment choice
15. `wuxia_seoharin_empty_place`
16. `wuxia_seoharin_left_meal`

Explicitly NOT in Wave 2: `wuxia_sado_*` (final battle container/phases/loss bridge),
`wuxia_cheonoe_analysis_thread_phase1_bridge` (lives inside the final battle),
all `*_resolution` / aftermath / return / settlement scenes, and `wuxia_collapse_gate`
(engine-driven gate — staging it needs its own design; leave legacy). If any listed encounter
resists clean staging, skip it, keep it legacy, and record why in the report.

## 2. Wave-2-specific designs

### 2.1 P1 — Validation hardening (core, small)

From the Wave 1 review:

- Reject `branch:` on blocks outside `kind: result` stages at index time (today a misplaced
  branch block silently always renders).
- Verify (and add if missing) index-time validation that every `next_stage_id` — on stages and
  on choice refs — resolves to an existing stage id in the same event. Unknown refs must fail
  bundle indexing, not surface as runtime cursor jumps.
- Tests: bundle with `branch` on a story block fails; bundle with dangling `next_stage_id`
  fails; existing bundles still index.

### 2.2 P2 — Conversion pattern

Same as Wave 1 §2.2 (story → choice → per-choice ResultStage via `next_stage_id`;
`result_summary` + verbatim outcome `log`; `branch:` blocks for checked choices; one
`illustration` block per event with the existing `presentation.visual_id`, `placeholder: true`
when unmapped, Korean `alt`; `dialogue` only where the legacy body already quotes a speaker —
무명, 서하린, 흑사방 보스, 청류문 장문인). Two additions:

- Several midgame encounters share `destination_id: cheongryu_outer_courtyard` returns — the
  destination stays on the outcome (engine), never becomes narration.
- Fragment rewards (13/14): three thread choices each get a ResultStage; keep the "two unchosen
  fragments are lost" line in the chosen ResultStage only if it already exists in that outcome's
  log — do not add new prose.

### 2.3 천기록 surface — `wuxia_cheonggi_record_writing_sense`

This is the canonical place for the `document` block kind (Event_Stage_Content_Model.md §천기록
surface): lines where the record itself writes/reacts become `document` blocks; ordinary
narration stays `narration`. Do NOT set `speaker: 천기록` on narration anywhere in Wave 2.
If the web renderer has no distinct styling for `document` items yet, minimal ink-token styling
(e.g. a ruled paper inset) is allowed in `render.ts`/CSS; keep it subtle and reduced-motion-safe.

### 2.4 P3 — Tests

- `event_stage_wave2.rs` guard mirroring Wave 1: all 16 expose `content_stream` on entry,
  staged action ids equal legacy choice ids, illustration block present.
- One checked-choice branch test if any Wave 2 encounter has a check (survey first; if none has
  a check, state that in the report instead of forcing one).
- One `document`-block assertion for `wuxia_cheonggi_record_writing_sense`.
- Keep every existing route_parity/json_contract assertion passing unweakened — the midgame
  chain is densely flag-gated; the parity suite is the real referee here.

### 2.5 P4 — Web sanity

Same as Wave 1 §2.4 (no feature work; five-viewport QA; typewriter/beat checks). Plus: verify
the `document` block renders acceptably at 390px. Attempt the Playwright browser install for
motion checks; if the environment still lacks Chrome, list the exact deferred manual items in
the report (Fable will run them at review, as in Wave 1).

## 3. Work packages

- **WP-S1** — §2.1 validation hardening + tests.
- **WP-C1** — Convert encounters 1–6. Exporter/fixtures (`--write` then `--check`), guard test.
- **WP-C2** — Convert encounters 7–11. Same gates.
- **WP-C3** — Convert encounters 12–16 (incl. `document` surface). Same gates.
- **WP-W1** — Web sanity run + minimal fixes (incl. `document` styling if needed).
- **WP-D1** — Report; update Development_Plan active-work line + Checklist; note staged
  coverage 14→30 of 44.
- **WP-D2 (Notion closeout, standing rule 6)** — ledger `idea_box/notion_sources.yml` sync base
  commit + Wave 2 note. Live page 13 edit: if Notion is unreachable from your environment,
  record the pending edits in the report for Fable (Wave 1 pattern).

## 4. Verification — identical command set to Wave 1 §4

cargo test --workspace / pytest (web_data_export + docs_contract) / exporter `--check` both
bundles / `cd web && npm test && npx tsc --noEmit && npm run build` (npm test now includes the
art-asset gate) / wasm-pack build / five-viewport `qa:storybook:visual --require-wasm`.

Manual flow checklist (report evidence or defer explicitly):

- 390px staged flow through `wuxia_mumyeong_first_confrontation` → `copy_style_reveal`:
  ordered stream, typewriter, no legacy fixed-slot page mid-chain.
- Fragment reward: pick one thread, confirm its ResultStage narration and that re-entry is
  blocked by the existing resolved flag.
- `wuxia_cheonggi_record_writing_sense`: `document` blocks visually distinct from narration.
- Save/reload inside a Wave 2 ResultStage restores the cursor.

## 5. Out of scope

Wave 3 encounters and the collapse gate; new illustrations/artManifest entries (gemini's track
is running in parallel — do not touch `web/public/assets/art/` or `artManifest.ts` at all in
this slice, even for placeholders, to avoid merge conflicts); trait effects; usable-item
content; any UI redesign.
