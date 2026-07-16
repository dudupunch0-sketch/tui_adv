# Game Loop Expansion, Slice 3 — Leveling, Insights (기연), Item Details, HUD Vitals (Plan by Fable, implementation by codex)

Date: 2026-07-16
Baseline: `main` HEAD `f426193` ("feat(web): ink-lacquer start menu redesign, fix delete-save closing the menu (#149)")
Predecessors: slice 1 (#134/#136/#137: stats, checks, traits, experience), slice 2 (#139/#140: check reveal, collapse gate), presentation steps (#143/#146/#147), action-result beat (PR #150 — assume merged; if not, rebase onto it).

Complete implementation directive. Execute WPs **in order, one at a time**; after each WP run verification, one commit, continue. Record status/deviations in `fable_gameloop3_step2_report.md` (repo root).

Build environment: `cargo`/`wasm-pack`/`gh` are WSL-only (`AGENTS.md`). Web commands run in `web/`.

## 0. Scope (decided with the user, 2026-07-16 playtest review)

1. **P1 — Beat: remove timed auto-advance.** The action-result beat currently advances on tap/key **or a 2.6s timer**. Remove the timer; tap/key only.
2. **P2 — Vitals terminology + HUD gauges.** Delta logs say 체력/정신력 but the HUD/drawer relabel the same resources 몸/마음. Unify on **체력/정신력**. Replace the ●●●○○ 5-dot rows (20 HP per dot — a −5 change is invisible) with thin ink-stroke gauges + numeric readout, pulsing on change.
3. **P3 — Leveling.** Stats (논리/공감/의지/평정/인터페이스/신체) are live (2d6+stat checks) but have no growth path. Add threshold-based stat points + allocation from the drawer.
4. **P4 — Insights (기연) with effects.** New lightweight system: named/described insights earned from encounter outcomes, each granting a permanent check bonus to one ability. Drawer section + check-banner integration.
5. **P5 — Item details.** Drawer inventory rows become tappable: placeholder pixel icon, description, and a 사용 button when usable. **No drop/discard** (explicitly excluded by the user).
6. **P6 — Trait detail.** Tapping the title seal (e.g. 청류문 초학자) shows its description; effects line reads honestly ("효과: 아직 새겨지지 않음" — trait effects remain out of scope).

### 0.1 Hard invariants (inherited; all binding)

1. Renderer boundary: probabilities, roll math, point accounting, item effects all in Rust core. The web renders ScenePage fields; no TS recomputation.
2. New ScenePage fields additive-optional (`Option<T>`/empty-vec + `skip_serializing_if`); TS mirrors optional; absent ⇒ byte-identical baseline JSON. New GameState fields `#[serde(default)]`; `SAVE_SCHEMA_VERSION` stays 1; old saves load.
3. Bundle schema extensions additive-optional; `CONTENT_BUNDLE_SCHEMA_VERSION` stays 1. YAML/artifact changes run `python scripts/export_web_data.py --write` then `--check` + Rust fixture sync in the same commit.
4. Existing action prefixes and localStorage keys frozen. This slice adds exactly one new action prefix: `train:` (WP-S2). No new storage keys.
5. QA contract selectors (storybook-reference-qa.mjs REQUIRED/FORBIDDEN) and existing render.test asserts stay valid; value-level contract-test updates (core_contract/json_contract/route_parity) allowed where a WP says so. Route graph / reachable ending ids must not change.
6. No new npm/cargo dependencies. Existing CSS tokens only. All new player-facing text is original Korean (게임체, consistent with 천기록/기연 diction). escape-terminal keeps compiling (no display obligation).
7. Reduced-motion: every new animation's resting frame is the correct final state.

### 0.2 Verification (after every WP, WSL for cargo/python)

```bash
cargo test --workspace
python scripts/export_web_data.py --check     # content-touching WPs
cd web && npx vitest run && npx tsc --noEmit && npm run build && cd ..
```

Once at the end, against a dev server with rebuilt wasm (`wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg`):

```bash
npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173/ --out-dir /tmp/tui-adv-gameloop3-qa --require-wasm
```

### 0.3 Commit convention

One commit per WP: `feat(core): … [WP-S<n>]` / `feat(web): … [WP-W<n>]` / `feat(content): … [WP-C<n>]` / `docs(design): … [WP-D<n>]`.

## 1. Current-state facts (2026-07-16 audit — trust, re-verify only if the tree moved)

- Beat timer: `web/src/main.ts` `presentActionResultBeat` — `window.setTimeout(finish, 2600)`.
- Label remap: `web/src/ui/storybook/render.ts` `storyResources()` forces health→몸, sanity→마음; core delta logs use `resource_label()` → 체력/정신력. Same resources, two names.
- Dots: `renderSlotRow` renders 5 glyphs at 20 points each.
- Abilities all start at 2 (`default_abilities()`); checks resolve `2d6 + ability >= difficulty` (`resolve_ability_check`); pre-shown odds via `ability_check_success_percent(ability, difficulty)`. No growth mechanism anywhere.
- Items: `ItemDef { id, name, description, item_type ("type"), usable, use_effects }`. Usable inventory items already surface as `use:{id}` actions with the item name as label; `apply_item_action` applies `use_effects` and consumes non-reusable items (verify exact consume semantics in code). Item descriptions never reach ScenePage (default pack is embedded in wasm — the web cannot read the bundle directly; everything player-visible must ride ScenePage).
- Traits: `TraitDef { id, name, description }`; `CharacterSummary { name, title_label, abilities }` — description not exposed. No trait effects (by design, slice 2).
- No skill/insight concept. Clues are bare ids (unsuitable for display).
- `ScenePage.content_labels` exists (items/achievements id→label). Drawer sections live in `renderBottomDock` (render.ts); drawer buttons already route through `data-player-action`/`data-action-id` wiring in main.ts.

## 2. Target design (implement exactly this)

### 2.1 P1 — Beat advance (web)

Remove the `setTimeout`/timer path from `presentActionResultBeat`; keep pointerdown/keydown (capture, single-fire). Update the hint copy if needed (unchanged: "화면을 누르면 계속").

### 2.2 P2 — Vitals (web-only)

- `storyResources()`: relabel health→**체력**, sanity→**정신력** (HUD and drawer 상태 both flow from here; delta logs already match).
- Replace `renderSlotRow` dots with an ink gauge row per vital:
  `<div class="hud-vital" data-resource-id data-band><span class="hud-vital__label">체력</span><span class="hud-vital__track"><span class="hud-vital__fill" style="--fill: NN%"></span></span><span class="hud-vital__value">95</span></div>`
  Track ~64–90px (clamp for 390px viewports; the bottom bar must still fit 2 vitals + 천기 gauge + 상세 button without overflow — verify at 390). Fill color by band: normal `--ink`, warning `#8a6a24`, critical `--seal-red`. Keep `aria-label="체력 정상 95"`-style text.
- Change pulse: in `main.ts`, where prev/next pages are both known (runAction), after the next page renders set `data-pulse="gain|loss"` on the vital whose value changed; CSS keyframe flashes the fill (≤400ms, resting state = normal). Remove attribute on animationend.
- Tests: update render.test asserts that reference 몸/마음/hud-slot as needed; add asserts for `.hud-vital__value` and 체력/정신력 labels.

### 2.3 P3 — Leveling (engine + web)

Engine (`state.rs`, `content.rs`, `turn.rs`, `scene_page.rs`):

- Bundle runtime meta, optional: `leveling: { thresholds: [u32, ...] }` — experience values at which one stat point is earned. Validation: strictly increasing, non-empty.
- `GameState.spent_stat_points: u32` (`#[serde(default)]`). Earned points = number of thresholds ≤ current experience (pure function of experience + meta — no new earned-state to store). Available = earned − spent (saturating).
- New action prefix `train:{ability_id}` (add `ACTION_PREFIX_TRAIN` beside the existing prefixes). Validity: leveling meta present, available > 0, ability id one of the fixed six, current value < **cap 5**. Effect: `abilities[id] += 1`, `spent_stat_points += 1`, delta log `+ {ability_label} 수련 1`. **No turn advance**: handle `train:` before the encounter pipeline — same turn, same encounter, no danger/eligibility/ending re-evaluation; rebuild the page from the mutated state.
- Threshold crossing feedback: when an outcome's experience gain crosses ≥1 threshold, append delta log `+ 수련 기회 {n}` (so the beat/floats surface it).
- `CharacterSummary` gains `stat_points: u32` (available; serialize always — it's inside an already-optional struct) so the renderer needs no math.
- Unit tests: earned/available accounting, cap, train validity (no meta / no points / cap / bad id), no-turn-advance, save without `spent_stat_points` loads, office pack (no meta) byte-identical.

Web (`render.ts`, drawer 인물 section):

- When `stat_points > 0`: badge in the 인물 header — `<span class="ability-points" data-region="ability-points">수련 가능 {n}</span>` — and each `.ability-row` below cap gets `<button class="ability-train" data-action-id="train:{id}" aria-label="{label} 수련">+</button>`. Buttons reuse the existing `[data-action-id]` click wiring (runAction). At cap, no button.
- After training the page re-renders (drawer closes — acceptable; note in report if it feels bad, do not fix here).
- Tests: badge/buttons render with points, absent without; cap hides button.

### 2.4 P4 — Insights 기연 (engine + web + content)

Engine:

- Bundle section `insights` (optional): `InsightDef { id, name, description, check_bonus: Option<CheckBonusDef { ability, bonus }> }`. Validation: unique ids; `check_bonus.ability` must be one of the six; bonus in 1..=2; outcomes' `add_insights` reference existing ids.
- `GameState.insights: Vec<String>` (`#[serde(default)]`, no duplicates). `OutcomeDef.add_insights: Vec<String>` (`#[serde(default)]`). On gain: delta log `+ 기연: {name}`.
- Check integration (core-side only): let `insight_bonus(state, content, ability) = sum of owned insights' check_bonus for that ability`. `resolve_ability_check` adds it to the total; `ActionCheckInfo.success_percent` is computed from `ability_value + insight_bonus` so the pre-shown odds stay honest. `CheckResolution` gains `insight_bonus: i32` (serialize; TS mirror) so the banner can show it. Keep the roll hash input string byte-identical (it must NOT include the bonus).
- `ScenePage.insights: Vec<InsightStatus { id, name, description, effect_text }>` (`skip_serializing_if = "Vec::is_empty"`). `effect_text` core-built, e.g. `평정 판정 +1`; empty string when no effect.
- Unit tests: bonus sums, percent includes bonus, dice unchanged for same seed/turn (hash regression), duplicate-gain idempotent, old save loads, empty-insights serialization identical to baseline.

Web:

- Check banner math line appends ` +기연 {n}` when `insight_bonus > 0` (both banner and beat reuse `renderCheckResolution` — one change).
- Drawer: new section between 인물 and 소지품: `<section aria-label="기연" data-dock="insights"><h2><span aria-hidden="true">緣</span>기연</h2>…` — each row: name + effect_text tag; tap toggles an inline description (same disclosure pattern as WP-W3 items). Empty state: `아직 맺은 기연이 없다.`
- Tests: section renders with insights, hidden without; banner shows `+기연 1`.

### 2.5 P5 — Item details (engine + web; NO drop)

Engine:

- `ScenePage.inventory_details: Vec<ItemDetail { id, name, description, item_type, usable }>` (`skip_serializing_if` empty) — exactly the ids in `inventory_summary.items`, data from ContentIndex. `usable` = ItemDef.usable && has use_effects (whether it is usable *right now* is decided by the presence of a `use:{id}` action on the page — renderer cross-checks, no new core logic).
- Unit test: shape + ordering matches inventory_summary.

Web (drawer 소지품):

- Each row becomes a disclosure: `<button class="item-row" data-item-id aria-expanded>` containing a **pixel icon placeholder** `<span class="item-icon" data-item-icon="{id}" style="--icon-hue: {h}" aria-hidden="true"></span>` (deterministic hue from a small hash of the id, muted via fixed ink-palette saturation/lightness; `data-item-icon` is the future sprite hook — swapping to real icons must require only CSS/asset changes) + name.
- Tap expands `<div class="item-detail">`: description, and — if a `use:{id}` action exists in `page.actions` — a `[사용]` button with `data-action-id="use:{id}"` (existing wiring; using an item re-renders the page and plays the result beat). If `usable` but no matching action this turn: disabled button labeled `지금은 쓸 수 없다`. No drop button anywhere.
- Expansion is DOM-only (no re-render); wire a small toggle handler in main.ts alongside the drawer wiring.
- Tests: detail markup, use-button enabled/disabled logic, icon hook attribute.

### 2.6 P6 — Trait detail (engine + web)

- Engine: `CharacterSummary.title_description: Option<String>` (TraitDef.description). Test: present with trait, absent without.
- Web: the title seal in the drawer becomes a disclosure button; expanded panel shows the description plus a fixed line `효과: 아직 새겨지지 않음` (single source of truth for this string in one const — a later slice replaces it with real effects). Tests: toggle markup + description rendering.

## 3. Work packages

Order: **D1 → W1 → S1 → S2 → S3 → W2 → W3 → W4 → C1 → D2.** If a WP looks riskier than described, skip, note why in commit + report, continue.

- **WP-D1** — docs first: add §2 designs to `docs/design/Progression_and_Title_Model.md` (Leveling, Insights, Item Details sections; amend Check Resolution for insight_bonus), register in `docs/00_Index.md` / `docs/dev/Development_Plan.md`, satisfy `tests/test_docs_contract.py`.
- **WP-W1** — P1 + P2 (beat timer removal, 체력/정신력 unification, HUD ink gauges + pulse). Web-only.
- **WP-S1** — P5 + P6 engine: `inventory_details`, `title_description`. **Test edits allowed**: additive asserts only.
- **WP-S2** — P3 engine: leveling meta, `train:` prefix, stat points, cap, no-turn-advance, threshold delta log. **Test edits allowed**: value-level updates in contract tests where new fields appear.
- **WP-S3** — P4 engine: insights section, add_insights, check-bonus integration, `ScenePage.insights`, `CheckResolution.insight_bonus`. **Test edits allowed**: `json_contract.rs` gaining the new optional fields; hash-regression test mandatory.
- **WP-W2** — P3 web: 인물 badge + train buttons. TS mirrors for all new fields land here (or in the first web WP that needs them).
- **WP-W3** — P5 web: item rows, pixel icons, detail disclosure, 사용 wiring.
- **WP-W4** — P4 + P6 web: 기연 section, banner `+기연 n`, trait detail disclosure.
- **WP-C1** — wuxia content: runtime `leveling.thresholds` (pick 2–3 values reachable in the preview — sum the encounter experience table first; e.g. [30, 80] if a full preview run earns ~100); `insights` section with 2–3 insights (original Korean, e.g. 청류심법 입문 → 평정 판정 +1) attached to existing encounter outcomes via `add_insights` (no new encounters/items/flags); exporter `--write`/`--check` + fixture sync in the same commit. **Guard**: route_parity ending ids unchanged; insight check bonuses change check outcomes only through resource/experience/log deltas placed per slice-2 rules — if any seed reaches a different ending id, reduce/move the bonus.
- **WP-D2** — closeout: docs/checklists sync, full §0.2 suite, one `qa:storybook:visual --require-wasm` run, manual flow (below), write the report.

Manual flow (WP-D2, dev server, motion on): new run → earn experience past the first threshold → beat shows `+ 수련 기회 1` → drawer 인물 shows 수련 가능 1 → train 평정 (+ button) → drawer reopens: 평정 3, badge gone → a 평정 check's shown odds are higher → gain an insight via its encounter → beat shows `+ 기연: …` → drawer 기연 lists it with effect → related check banner shows `+기연 1` → 소지품: tap 천기록이 깃든 업무수첩 → description opens, no drop button → tap a usable consumable → 사용 → result beat plays → HUD 체력/정신력 gauges show numbers and pulse on change.

## 4. Explicitly out of scope

- Item drop/discard (user decision), inventory capacity, item icons beyond the pixel placeholder hook.
- Trait numeric effects (P6 shows the honest placeholder line), trait switching UI.
- Insight effects beyond the single `check_bonus` type (no resource passives, no active skills).
- Level numbers/XP bars beyond the existing 천기 gauge; stat respec; ability caps above 5.
- Danger-based content conditions; any start-screen or typewriter changes; terminal renderer display work.

## 5. Final checklist

- [ ] One commit per WP, in order.
- [ ] `cargo test --workspace`, exporter `--check`, `vitest`, `tsc`, `build` green after each WP.
- [ ] Office pack ScenePage JSON byte-identical (no leveling/insights meta ⇒ no new fields serialized).
- [ ] Old saves (no `spent_stat_points`/`insights`) load; roll-hash regression test passes.
- [ ] `qa:storybook:visual --require-wasm` all 5 viewports; bottom bar fits at 390px with the new gauges.
- [ ] Manual flow above completed and reported with screenshots.
- [ ] Report written to `fable_gameloop3_step2_report.md`.
