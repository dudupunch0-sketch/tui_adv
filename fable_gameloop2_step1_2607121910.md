# Game Loop Expansion, Slice 2 — Check Dramatization, Death Gate, Content Debt (Plan by Fable, implementation by codex/gemini)

Date: 2026-07-12
Baseline: `main` HEAD `8be4adf` ("feat(web): ink-wash polish for choice checks, character drawer, progression gauge (#137)")
Reference: [docs/reference/Life_in_Adventure_Play_Reference.md](docs/reference/Life_in_Adventure_Play_Reference.md) — all UX-grammar requirements trace to this document. Do not re-read the original images.
Predecessor: `fable_gameloop_step1_2607121314.md` (merged as #134/#136/#137). This plan assumes all of its systems (character_summary, ActionCheckInfo, delta logs, traits, progression) are live.

This document is a complete implementation directive. Read it fully, then execute work packages **in order, one at a time**. After each WP: run verification → one git commit → next WP. Record completed/skipped WPs and any deviations in `fable_gameloop2_step2_report.md` (repo root).

Build environment: `cargo` / `wasm-pack` / `gh` exist **only in WSL** (`AGENTS.md`). Run all verification in WSL `~/work/tui-adv`.

---

## 0. Scope

Four tracks. A and B are content/architecture debt found during the step-1 QA pass; C and D are the next reference-grammar slice.

- **A — Content-owned labels.** Item and achievement display names already exist in bundle YAML (`items[].name`, `achievements[].name`), but the web renderer ignores them and keeps a duplicated hardcoded dictionary (`web/src/ui/storybook/labels.ts`). Every future content addition silently falls back to a raw id with a "미번역" note. Fix the architecture: ScenePage carries display labels; the TS dictionary becomes a legacy fallback only.
- **B — Check density in wuxia pack.** The wuxia preview pack contains **zero** `check:` blocks (verified by grep), so the entire step-1 probability-disclosure UI never appears in the shipped storypack. Add 2d6 checks to early/mid encounters.
- **C — Check resolution reveal ("ritualized roll").** Reference §3 / adaptation note: keep 2d6, but *show* the roll. After a checked choice resolves, the next page shows dice values, modifier, difficulty, and verdict. Engine exposes structured data; renderer shows a structural banner (visual polish is a later Claude step, same split as step 1).
- **D — Death gate + second wind (기사회생).** Reference §3.7: at health 0, instead of instant run end, offer a one-per-run revival choice or accept the ending. Implemented as a generic, bundle-opt-in "collapse gate", assembled from existing content primitives.

**Not in this slice** (deferred, as in step 1 §4): alignment axis, romance, shop/economy/gems, ranking, character creation screen, level-up stat allocation, trait numeric effects, encyclopedia list UI, visual redesign. The *pricing* of second wind uses experience, not a new currency.

**Visual polish is NOT in this slice.** Web WPs make data appear structurally, reusing existing ink-wash tokens only. Claude will polish `check-resolution` and the collapse screen in a separate step — attach the exact class names / data attributes specified below so that step can restyle without touching TS logic.

### 0.1 Hard invariants (inherited from step 1 §0.1 — all still binding)

1. **Renderer boundary**: eligibility/outcome/ending/probability/roll logic lives in Rust GameCore. The renderer never recomputes; it displays ScenePage fields.
2. **New ScenePage fields are additive-optional**: Rust `Option<T>` + `#[serde(skip_serializing_if = "Option::is_none")]`; TS mirror with `?`. Absent value ⇒ serialized JSON byte-identical to baseline.
3. **Existing field names, action-id prefixes, and localStorage keys are frozen.** No new localStorage keys in this slice.
4. **New GameState fields take `#[serde(default)]`** — old saves must load. `SAVE_SCHEMA_VERSION` stays 1.
5. **Bundle schema extensions are additive-optional.** `CONTENT_BUNDLE_SCHEMA_VERSION` stays 1. Any WP touching YAML/generated artifacts runs `python scripts/export_web_data.py --write` then `--check` and syncs Rust fixtures in the same commit.
6. **QA contract selectors** in `storybook-reference-qa.mjs` and existing `render.test.ts` asserts stay valid.
7. **No new npm/cargo dependencies.**
8. **No copying of reference-game text, art, or proper nouns** — all new narrative text is original 이구학지-world writing, in Korean.
9. **Never color-only information**: verdicts and deltas always carry text/symbols too.
10. **escape-terminal must keep compiling**; it is not required to display new fields. `route_parity.rs` failures mean the logic change is wrong, not the test.

Slice-specific additions:

11. **Adding a check must not change the route graph.** Failure branches may only apply resource/experience deltas and narrative logs — never remove items/flags/clues required for progression, never change `destination_id`. Reachable encounters/endings before == after (see WP-C1 guard).
12. **Probability & dice stay core-side.** `success_percent` display exists; do not add any TS dice math.

### 0.2 Verification commands (after every WP, in WSL)

```bash
cargo test --workspace
python scripts/export_web_data.py --check     # only WPs that touch content artifacts
cd web && npm test && npx tsc --noEmit && npm run build && cd ..
./.venv/bin/python -m pytest tests/ -q        # only WPs that touch docs/data contracts
```

Once after all WPs:

```bash
cd web && npm run build:player
npm run qa:storybook:visual -- --require-wasm --base-url http://127.0.0.1:4173/ --out-dir /tmp/tui-adv-gameloop2-qa
```

### 0.3 Commit convention

One commit per WP: `feat(core): … [WP-S<n>]` / `feat(web): … [WP-W<n>]` / `feat(content): … [WP-C<n>]` / `docs(design): … [WP-D<n>]`.

---

## 1. Current-state findings (2026-07-12 code audit — trust these, re-verify only if the tree moved)

- `web/src/ui/storybook/labels.ts` hardcodes 13 achievement + 17 item labels. Bundle YAML already carries `name` for every item/achievement in both packs (`src/tui_adv/data/*.yaml`, `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml`). Coverage happens to be complete *today*; the defect is architectural duplication.
- `ScenePage.inventory_summary.items` and `achievement_summary.unlocked/newly_unlocked` are **id lists** — names never leave core, although `ContentIndex` has them.
- Wuxia pack: no `check:` anywhere in `encounters.yaml`. Office pack (`src/tui_adv/data/encounters.yaml`) has working examples (e.g. `ability: interface / difficulty: 10`) — copy that YAML shape.
- Check resolution: `turn.rs::ability_check_succeeds` rolls `roll_2d6(hash(seed:turn:ability:difficulty))`, picks `check.success` or `check.failure`, and **discards the dice values**. Nothing reaches ScenePage.
- Death: no engine-level death gate. The wuxia pack has only 2 endings (`kind: preview`, `kind: final_epilogue_contract`) — **no death ending exists**, so health 0 currently has no dramatic consequence in the shipped pack.
- `OutcomeDef` already supports: `resources`, `experience`, `set_trait`, `add_flags/remove_flags`, `destination_id` — the collapse gate needs no new outcome machinery.
- `render.ts::storyPhase()` derives `data-story-phase` from visual kind / history — extend, don't replace.

## 2. Target design (implement exactly this)

### 2.1 A — Content-owned display labels

New optional ScenePage field (Rust `scene_page.rs`, mirrored in `web/src/core/types.ts`):

```rust
/// Labels for ids referenced elsewhere on this page. Some only when content provides names.
pub content_labels: Option<ContentLabels>,
// ContentLabels {
//   items: Vec<LabeledId>,        // exactly the ids in inventory_summary (incl. none if empty)
//   achievements: Vec<LabeledId>, // exactly the ids in achievement_summary.unlocked ∪ newly_unlocked
// }
// LabeledId { id: String, label: String }   // label = ItemDef.name / AchievementDef.name
```

Rules:
- Populate from `ContentIndex` at ScenePage assembly. Ids missing from the index (should not happen) are simply omitted from the vec — renderer falls back.
- Renderer lookup order: `content_labels` → legacy dictionary in `labels.ts` → humanized id + `미번역` note. Do **not** delete `labels.ts`; reduce it to the fallback layer (keep existing entries as safety net for old saves rendered against changed bundles).
- Delta log lines (`+ {아이템명}` etc.) already receive names core-side — unchanged.

### 2.2 B — Wuxia check-density content pass

- Add `check:` blocks to **4–6 existing choices** across the first ~10 turns of play (arrival / sect-apprenticeship / early-brawl encounters). No new encounters, no new items, no label changes in this track.
- YAML shape (same as office pack):

```yaml
check:
  ability: physical        # one of: logic|empathy|volition|composure|interface|physical
  difficulty: 9            # 2d6 + ability(2 at start) → pick 8..10 for 58–83% starting odds
  success: { log: "...", experience: 4 }          # may reuse/extend the choice's prior outcome
  failure: { log: "...", resources: { sanity: -6 } }
```

- Difficulty guidance: early encounters 8–9 (favorable/uncertain band), one riskier 10–11 to show the red band. At least 3 different abilities across the added checks so the stat grid matters.
- **Invariant 11 applies**: when converting an existing always-on outcome into a checked one, put the progression-critical effects (flags/items/destination) in the *choice-level* `outcome`, and only resource/experience/log deltas in `success`/`failure`. `apply_action` applies choice outcome first, then the check branch (verified in `turn.rs`), so progression is unconditional and the roll only modulates cost/reward.

### 2.3 C — Check resolution reveal

Engine (`turn.rs`, `state.rs`, `scene_page.rs`):

```rust
// GameState (serde default; cleared at the start of every apply_action, set when a check resolves)
pub last_check: Option<CheckResolution>,

// ScenePage (additive-optional; copied from GameState.last_check at assembly)
pub check_result: Option<CheckResolution>,
// CheckResolution {
//   ability_id: String, ability_label: String,
//   dice: (i32, i32), ability_value: i32, difficulty: i32,
//   total: i32,                    // dice.0 + dice.1 + ability_value
//   success: bool,
// }
```

- Refactor `ability_check_succeeds` so dice values are returned, not discarded (e.g. `resolve_ability_check(state, check) -> CheckResolution`). The hash input string must stay byte-identical (`"{seed}:{turn}:{ability}:{difficulty}"`) — otherwise every parity/contract test shifts.
- Lifecycle: `apply_action` first sets `last_check = None`, then, if the chosen action had a check, stores the resolution. Result: the field is Some on exactly the one page following a checked choice, None afterwards. Old saves (field absent) deserialize as None.
- Doc-comment the determinism caveat as in step 1 §2.2.

Renderer (`render.ts` — structural only):

```html
<aside class="check-resolution" data-region="check-result"
       data-check-outcome="success|failure" data-ability-id="...">
  <span class="check-resolution__dice" aria-hidden="true">⚃ ⚁</span>
  <span class="check-resolution__math">2d6 4+2 +{ability_label} {ability_value} = {total} / 목표 {difficulty}</span>
  <span class="check-resolution__verdict">성공|실패</span>
</aside>
```

- Placement: inside `.storybook-body`, directly **above** the inline result log (`.story-result-log`).
- Dice glyphs: map 1..6 → ⚀⚁⚂⚃⚄⚅, `aria-hidden`; the math line carries the numbers as text (invariant 9). Whole aside gets `aria-label="판정 결과: {verdict}"`.
- No animation work in this slice — Claude's polish step will animate using these hooks. Minimal CSS only: monospace caption using existing tokens, verdict colored `--jade` / `--seal-red`.

### 2.4 D — Collapse gate (death + second wind)

Bundle runtime meta, optional (like `progression`):

```yaml
runtime:
  collapse:
    encounter_id: wuxia_collapse_gate   # must exist in encounters
    resource_id: health                 # only "health" supported this slice; validate
    used_flag: second_wind_used
```

Engine rule (`turn.rs`, where the post-action encounter/ending selection happens):

- After an action resolves, if `collapse` meta exists AND `player.health <= 0` AND `used_flag` not in flags AND the current encounter is not the collapse encounter itself → **force the next encounter to `collapse.encounter_id` and skip ending selection this turn**. Do not clamp or modify health — content decides.
- If `used_flag` is already set (or the player is inside the collapse encounter), normal rules apply: ending selection runs and a death ending can fire.
- Unit-test the gate: triggers once; does not re-trigger after `used_flag`; office pack (no meta) byte-identical baseline.

Bundle validation (`validate_content_bundle` / `index_content_bundle`): `collapse.encounter_id` exists; `resource_id == "health"`; `used_flag` non-empty. Reject otherwise with a clear error.

Wuxia content (all original Korean text):

- New encounter `wuxia_collapse_gate` (visual `kind: collapse_gate`) — the record opens its final page. Two choices, both plain outcomes:
  - **기사회생**: `resources: { health: +40 }`, `add_flags: [second_wind_used]`, `experience: -10` (floor at 0 is engine-guaranteed since step 1), log line describing the 천기록 burning a page to pull the protagonist back.
  - **안식**: `add_flags: [accept_final_rest]`.
- New death ending (`kind: death`, high priority) with `conditions: { required_flags: [accept_final_rest] }` plus an epilogue-contract-compatible body, mirroring how the two existing endings are wired.
- This is the **only** new encounter and the **only** new ending allowed in this slice. `contentBundles.test.ts` fixtures updated accordingly (encounter id list grows by exactly one).

Renderer: extend `storyPhase()` — `visual.kind === 'collapse_gate'` → phase `collapse`; add `data-story-phase="collapse"` styling hook with minimal treatment (existing `--seal-red` edge accent on `.storybook-page`; the ink-vignette dramatization is Claude's later step). Choices/keyboard/QA selectors unchanged.

## 3. Work packages

Order: **D1 → S1 → S2 → S3 → W1 → W2 → W3 → C1 → C2 → D2**. S/C: codex preferred; W: codex or gemini. If a WP looks riskier than described, skip it, note why in the commit message and report, and continue.

### WP-D1: design doc first
Add §2 of this plan to `docs/design/Progression_and_Title_Model.md` (new sections: Content Labels, Check Resolution, Collapse Gate) or a sibling design doc; register in `docs/00_Index.md`; add the slice to `docs/dev/Development_Plan.md`. Satisfy `tests/test_docs_contract.py`.
**Verify**: pytest docs contract.

### WP-S1: content_labels (§2.1 engine)
`ContentLabels`/`LabeledId` types + assembly from ContentIndex. Unit tests: shape, empty-inventory case, id-not-in-index omission.
**Test edits allowed**: additive asserts in `core_contract.rs`; existing asserts must pass untouched (invariant 2).

### WP-S2: check resolution (§2.3 engine)
`CheckResolution`, `GameState.last_check`, refactored roll fn, lifecycle. Unit tests: set-on-check, cleared-on-next-action, absent-field save loads, hash-input regression test (same seed/turn/ability/difficulty ⇒ same dice as before the refactor — assert against a hardcoded expected pair).
**Test edits allowed**: `json_contract.rs` asserts gaining a `check_result` field on the one page after a checked choice.

### WP-S3: collapse gate (§2.4 engine + validation)
Gate rule + bundle meta parsing + validation + unit tests listed in §2.4.

### WP-W1: TS mirrors + label preference
Mirror `content_labels` / `check_result` in `types.ts` (snake_case). `labels.ts`: export same API, but resolution order per §2.1 — add `contentLabel*` lookups fed from the current page (thread via existing render call path, not a global). `render.test.ts`: fixture with content_labels overriding a wrong dictionary entry proves precedence.
**Verify**: `npm test`, `tsc --noEmit`.

### WP-W2: check resolution banner (§2.3 renderer)
Markup exactly as specified (class names are a contract with the later polish step). Minimal CSS with existing tokens. Tests: banner renders with dice glyph + math text + verdict; absent when `check_result` missing.

### WP-W3: collapse phase hook (§2.4 renderer)
`storyPhase()` extension + minimal `data-story-phase="collapse"` CSS. Test: fixture with `collapse_gate` visual kind sets the attribute.

### WP-C1: wuxia check-density pass (§2.2)
4–6 checks per the design rules. Then `export_web_data.py --write` → `--check` → fixture sync, one commit.
**Guard for invariant 11**: before/after, dump the reachable encounter-id and ending-id sets (route_parity already walks routes — its ending-per-seed table may change *values* only if a failure branch changes resource totals; if any seed reaches a *different ending id* than before, revert that check placement).
**Test edits allowed**: value-level asserts in `json_contract.rs` / `route_parity.rs` (action `check` fields, logs, resources). Encounter/ending id lists unchanged in this WP.

### WP-C2: collapse content (§2.4 content)
`wuxia_collapse_gate` encounter + death ending + runtime `collapse` meta + export/fixture sync. Manual sanity: force health to 0 in a unit test route and assert gate → revive → play continues; gate → 안식 → death ending.
**Test edits allowed**: fixtures gaining the new encounter/ending ids; `contentBundles.test.ts` id list +1 each.

### WP-D2: closeout
Docs/checklist sync (`docs/dev/Checklist.md`, `Development_Plan.md`, design doc vs implementation — fix the implementation if they diverge). Full §0.2 suite + the one-shot `qa:storybook:visual` run. Write `fable_gameloop2_step2_report.md`.

## 4. Explicitly out of scope (do not do)

- Any animation/motion work for the check banner or collapse screen (Claude's polish step).
- Gems or any new currency; second wind costs experience only.
- New localStorage keys, encyclopedia UI, character creation, trait numeric effects.
- Renaming abilities for the wuxia world (content-layer concern, later slice).
- Deleting `labels.ts` or changing its exported API surface.
- More than one new encounter / one new ending.

## 5. Final checklist

- [ ] One commit per WP, in order (D1 → S1–S3 → W1–W3 → C1–C2 → D2).
- [ ] `cargo test --workspace` green; `npm test && tsc && build` green; `export_web_data.py --check` green; pytest green.
- [ ] Office pack ScenePage JSON byte-identical to baseline (no collapse meta, no content_labels regression — verify with the existing contract tests).
- [ ] Old save without `last_check` loads.
- [ ] Manual flow: new run → early checked choice shows odds → resolve → dice banner with verdict → drawer stats explain the modifier → force low health → collapse gate → 기사회생 → `- 경험 10` delta log + gauge drop → second death → 안식 → death ending → start screen run count grows.
- [ ] Deviations recorded in `fable_gameloop2_step2_report.md`.
