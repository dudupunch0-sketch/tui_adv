# Plan — Event/Stage/ContentBlock Migration, Wave 1 (Early-Game Arc)

Author: Fable (plan/review). Implementer: codex.
Baseline: `origin/main` `5ab25cb` (`docs(sync): close Gameloop 3 insight DB reconcile in Notion ledger (#159)`).
Design authority: `docs/design/Event_Stage_Content_Model.md` (canonical grammar), `docs/dev/Data_Schema.md` (wire schema), `docs/design/Mobile_Ink_Storybook_UI.md` (layout).
Report file: write `fable_eventstage_step2_report.md` at repo root (same format as `fable_gameloop3_step2_report.md`).

## 0. Why this slice

Development_Plan.md marks Event/Stage/ContentBlock conversion as the next active work after
Gameloop 3 closed. Today only **4 of 44** wuxia encounters use the staged model
(`wuxia_commute_rift_arrival`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`,
`wuxia_sado_final_battle`). Every other encounter still renders as a legacy single-body page, so
the player experiences two different presentation grammars within the first ten minutes.

Wave 1 converts the **always-seen early-game arc** so a fresh run is staged end-to-end, and closes
the one real model gap the conversion will hit: **check-branch-dependent result narration**.

## 0.1 Hard invariants (violating any of these fails review)

1. **Route graph invariance.** Encounter ids, choice ids, costs, outcomes, checks, flags, clues,
   destinations, experience, insights, endings: byte-identical semantics. The conversion
   redistributes *presentation* only. `route_parity.rs` and `json_contract.rs` must pass
   without weakening any assertion (value-level additions are fine; deletions are not).
2. **Roll hash regression.** The 2d6 hash input string `"{seed}:{turn}:{ability}:{difficulty}"`
   stays byte-identical. No new randomness.
3. **Additive schema only.** New fields are `Option<T>`/`#[serde(default)]` +
   `skip_serializing_if`. Old bundles and old saves must keep loading
   (`SAVE_SCHEMA_VERSION` stays 1; `CONTENT_BUNDLE_SCHEMA_VERSION` stays 1).
4. **No new action prefixes, no new localStorage keys, no new deps.**
5. **Original Korean text.** Narrative sentences move verbatim from the legacy `body` into
   blocks. Splitting a paragraph at sentence boundaries is allowed; rewriting, summarizing, or
   inventing new prose is not. (Short connective stage titles/ids are new — ids in English,
   snake_case, stable.)
6. **Office pack untouched.** `content.bundle.json` for office changes only if a shared fixture
   count line forces it; the office YAML gains no `event:` sections.
7. Renderer boundary: all stage-cursor/branch decisions in Rust core; web reads
   `content_stream` only.

## 1. Scope — Wave 1 conversion set (10 encounters)

Convert in this order (early-game main line first, then the cheongryu arc the player hits in
every run):

1. `wuxia_heuksa_bang_first_fight` — has a physical check; first conversion with branch results
2. `wuxia_cheongryu_apprentice_entry`
3. `wuxia_cheongryu_chore_sparring`
4. `wuxia_cheongryu_raid_route_split`
5. `wuxia_cheongryu_raid_wounded_fallback`
6. `wuxia_baekdo_medicine_debt`
7. `wuxia_black_heaven_escape_price`
8. `wuxia_heavenly_archive_previous_outsiders`
9. `wuxia_wounded_shelter_dawn_offers`
10. `wuxia_mumyeong_first_sighting`

Everything else (mumyeong midgame chain, resolutions, epilogue bridges) is Wave 2+ — do NOT
convert opportunistically. If a listed encounter turns out to be structurally hostile to staging
(e.g. collapse-gate special casing), skip it, keep it legacy, and record the reason in the report.

## 2. Target designs

### 2.1 P1 — Check-branch result blocks (core, the only schema change)

Problem: a ResultStage's blocks are static, but checked choices have success/failure branches
with different narration. Today the check banner + delta logs carry the difference; the staged
result prose cannot.

Design (additive):

- `ContentBlockDef` gains `#[serde(default)] pub branch: Option<String>` with allowed values
  `"success" | "failure"` (validate at index time; reject other values).
- Blocks with `branch: None` always render. Blocks with a branch render only when the previous
  check resolution matches: at `content_stream` build time, filter ResultStage blocks against
  `state.last_check` (`Some(res)` → keep `None` + matching branch; `None`/no check → keep only
  `branch: None` blocks).
- Mirror the optional field in `web/src/core/types.ts` (`SceneContentItem` already carries
  kind/text/etc. — extend whatever struct maps `ContentBlockDef` into stream items; the web
  renderer needs **no** logic change because filtering happens core-side).
- Tests (core): a staged encounter with a checked choice renders success-only blocks on success
  and failure-only blocks on failure; a bundle with `branch: "sucess"` (typo) fails validation;
  legacy bundles without `branch` load unchanged.

### 2.2 P2 — Conversion pattern per encounter (content)

For each encounter in §1, add an `event:` section following the existing staged encounters as
the template (see `wuxia_seo_harin_rescue` in `encounters.yaml`). Rules:

- Minimum shape `StoryStage → ChoiceStage → ResultStage`; use extra StoryStages where the legacy
  body clearly has two beats (scene-setting vs. immediate pressure). Do not exceed what the
  existing prose supports.
- Every choice gets its own ResultStage via `next_stage_id`, starting with a `result_summary`
  block, then the outcome `log` line as narration (verbatim). For checked choices use §2.1
  `branch:` blocks carrying the success/failure branch `log` lines.
- One `illustration` block per event minimum (Event_Stage_Content_Model.md contract):
  `visual_id` = the encounter's existing `presentation.visual_id` (or encounter id if none),
  `placeholder: true` when no manifest asset exists, plus a Korean `alt` describing the scene.
  Do NOT invent new asset files; this feeds the existing illustration gap pipeline
  (`docs/design/Wuxia_Illustration_Gap_Handoff_260716.md`) — keep visual ids consistent with
  that document's required IDs where the encounter appears there.
- `dialogue` blocks only where the legacy body already quotes a speaker (서하린, 무명 등);
  set `speaker` to the actual name. Never label plain narration as 천기록 (§천기록 surface rule).
- Stage ids: `<short>_story` / `<short>_choice` / `<choice_id>_result` pattern, stable.

### 2.3 P3 — Verification content parity

- Regenerate bundles with `scripts/export_web_data.py --write` (default + wuxia preview) and the
  Rust/web fixtures; `--check` must pass afterwards.
- `event_stage.rs`: add coverage for at least `wuxia_heuksa_bang_first_fight` (checked choice:
  success and failure branch streams differ; stage order preserved; choice rejected before its
  ChoiceStage) and one plain conversion (e.g. `wuxia_cheongryu_apprentice_entry`).
- Contract guard: a test asserting the Wave-1 ten encounters all expose `content_stream` on
  entry and that their action ids equal the legacy choice id set (conversion completeness +
  route invariance in one place).

### 2.4 P4 — Web sanity (no feature work)

The web renderer already handles `content_stream` (`.story-flow--ordered`, typewriter,
`data-region="body"`, action-result beat suppression of authored `result_summary`). Wave 1 adds
no web features. Required checks only:

- Typewriter still reveals staged narration progressively at 390px (motion-on Playwright
  context), and the check banner + 成/敗 seal renders inside staged flow for the heuksa fight.
- Five-viewport `npm run qa:storybook:visual -- --require-wasm` passes (390/414/800/810/1440).
- If a rendering defect surfaces (overlap, ordering, beat duplication), fix minimally in
  `render.ts`/CSS within ink tokens; no new color literals; reduced-motion resting frames stay
  correct.

## 3. Work packages (execute in order)

- **WP-D1** — Update `docs/dev/Data_Schema.md` with the `branch` field contract and
  `docs/design/Event_Stage_Content_Model.md` with a short "branch blocks in ResultStage"
  subsection (keep both docs' ownership boundaries).
- **WP-S1** — P1 core: `branch` field, index-time validation, content_stream filtering, tests.
- **WP-C1** — P2 conversion, encounters 1–5 (main line + cheongryu arc). Exporter/fixtures,
  P3 tests for the converted set.
- **WP-C2** — P2 conversion, encounters 6–10. Exporter/fixtures, extend the completeness guard.
- **WP-W1** — P4 web sanity run + minimal fixes.
- **WP-D2** — Report `fable_eventstage_step2_report.md`; update `docs/dev/Development_Plan.md`
  active-work line and `docs/dev/Checklist.md`.
- **WP-D3 (Notion closeout, standing rule 6)** — update `idea_box/notion_sources.yml`
  (sync base commit) and the Notion page `13. 런타임 시스템 현황` (staged coverage 4→14, wave
  scope, deviations). If Notion access is unavailable in your environment, record the exact
  pending edits in the report instead and flag for Fable.

## 4. Verification commands (all must pass; run in WSL `~/work/tui-adv` except web)

```
cargo test --workspace
.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py
python3 scripts/export_web_data.py --check   # plus the wuxia preview --check variant
cd web && npx vitest run && npx tsc --noEmit && npm run build
wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg
cd web && npm run qa:storybook:visual -- --require-wasm --base-url http://127.0.0.1:5173/ --out-dir /tmp/tui-adv-eventstage-qa
```

Manual flow checklist (record evidence in the report):

- New game at 390px: arrival → heuksa fight → apprentice entry all render as ordered streams;
  no screen shows the legacy fixed-slot layout mid-arc.
- Heuksa fight: pick the checked choice twice across two seeds — success run shows the success
  branch narration, failure run the failure branch; dice banner totals match the math line.
- Save mid-event (staged), reload, resume: same stage position (`active_event_id` /
  `event_stage_index` round-trip).
- Badge ending route (`grip_employee_badge`) still ends the run with the ending restart button.

## 5. Out of scope (do not touch)

- Wave 2+ encounter conversions; office pack YAML; endings/epilogue renderer.
- New illustrations or artManifest entries (separate gemini track via the gap handoff doc).
- Leveling/insight/item systems (Gameloop 3 is closed; regressions there are review failures).
- Any UI redesign beyond §2.4 minimal fixes.
