# Gameloop 3 Post-Merge Refactor and Closeout Plan

Date: 2026-07-16
Baseline: `main` HEAD `c33fc75` (`feat: implement Gameloop 3 story stages and progression (#155)`)
Source review: Codex review of `fable_gameloop3_step2_report.md` against `cef38c0..c33fc75`
Implementation report: `fable_gameloop3_refactor_step2_report.md`

## 1. Objective

Close the verified Gameloop 3 defects without expanding the feature set:

1. Prevent the 390 px bottom HUD from overlapping.
2. Reject training after a content ending.
3. Make duplicated insight IDs harmless at runtime.
4. Preserve valid inventory rendering when `inventory_details` is absent.
5. Add the missing contract and visual regression coverage.
6. Complete Development Plan, Checklist, ledger, and Notion runtime-status synchronization.

The implementation is a corrective refactor. It must preserve route topology, action IDs, save schema version, bundle schema version, roll hashing, content rewards, and all existing player-visible behavior outside the defects above.

## 2. Review Baseline

The following commands passed on `c33fc75` during review:

- `cargo test --workspace`
- `.venv/bin/pytest -q tests/test_web_data_export.py` — 12 passed
- default and wuxia preview exporter `--check`
- `cd web && npx vitest run` — 69 passed
- `cd web && npx tsc --noEmit`
- `cd web && npm run build`
- WASM rebuild
- Storybook visual QA at 390, 414, 768, 1024, and 1440 px

Passing QA is not sufficient for the current HUD. The 390×844 screenshot visibly overlaps the sanity value and Cheon-gi progression area because the QA gate checks overflow but not sibling bounding-box collisions.

## 3. Scope

### R1 — Core action and state normalization

- Move the ending-state guard ahead of `train:` dispatch.
- Treat `GameState.insights` as an ordered set at every read boundary that affects gameplay or ScenePage output.
- Preserve first-occurrence order when deduplicating insights.
- Do not rewrite or migrate saves solely to normalize duplicated IDs.

### R2 — Web compatibility and mobile HUD

- Make the bottom HUD collision-safe at 390 px.
- Keep two numeric vitals, the Cheon-gi gauge, and the detail button visible.
- Render inventory rows without false disclosure semantics when detail data is absent.

### R3 — Regression coverage

- Add the missing core validity, accounting, idempotence, and probability tests.
- Add render tests for the optional inventory-detail path.
- Extend visual QA with explicit bottom-HUD non-overlap assertions.

### R4 — Closeout synchronization

- Correct canonical plan/checklist status.
- Update `idea_box/notion_sources.yml` with the final implementation commit.
- Update Notion page `13. 런타임 시스템 현황 (repo 동기화)` after the code commits are final.
- Record exact verification evidence in the refactor report.

## 4. Hard Invariants

All inherited Gameloop 3 invariants remain binding.

1. Rust owns action validity, point accounting, check math, insight effects, rewards, and state mutation.
2. `SAVE_SCHEMA_VERSION` remains `1`. Existing saves without `spent_stat_points`, `insights`, or `last_check.insight_bonus` must load.
3. `CONTENT_BUNDLE_SCHEMA_VERSION` remains `1`.
4. `ScenePage.inventory_details` and `ScenePage.insights` remain additive-optional. Absence is a supported input state.
5. Existing action prefixes and storage keys remain frozen. Do not add another prefix.
6. The `train:` action remains non-turn-advancing in non-ending gameplay.
7. Roll hash inputs and dice results remain byte-identical for the same seed, turn, encounter, and action.
8. Route graph, encounter destinations, reachable ending IDs, reward placement, and wuxia content data remain unchanged.
9. No new Cargo, npm, or Python dependency.
10. Use existing design tokens only. Do not introduce new color literals.
11. The fixed three-region game frame remains strict. Body content must not enter either fixed bar.
12. Every animation must rest at its correct final state under reduced motion.
13. Do not modify or regenerate content bundles unless an exporter check proves drift. This refactor should not touch content YAML or generated bundle JSON.
14. Each work package is one commit, in the order below. Run its stated verification before continuing.

## 5. Chosen Design

### 5.1 Ending guard

In `crates/escape-core/src/turn.rs`, resolve the current location as today, then reject every action when `current_content_ending(content, state).is_some()`. Only after that guard may the function dispatch `train:`, `use:`, event, choice, or movement actions.

This keeps one terminal-state rule for every mutating action. Do not add a train-specific ending check inside `apply_train_action`; that would leave duplicated policy and permit future dispatch-order regressions.

Expected behavior:

- Non-ending `train:{ability}` retains current validation and does not advance the turn.
- Ending-state `train:{ability}` returns `ContentActionError::UnknownAction`.
- Ending ScenePages must not expose training buttons. Enforce this through the core action surface or a ScenePage terminal-state condition, not through TypeScript-only policy.

### 5.2 Insight normalization

Do not mutate a loaded save as a side effect of rendering. Introduce a small ordered-unique iterator/helper in the Rust core and use it for both:

- `insight_bonus(...)`
- `ScenePage.insights` construction

The first occurrence wins and defines display order. Unknown IDs remain ignored as today. A duplicated known ID contributes one bonus and one drawer row.

Keep gain-time idempotence in `apply_outcome`. Add tests that apply the same insight reward twice and separately deserialize a duplicate-bearing save.

Rejected alternative: schema migration or eager save rewriting. It is unnecessary for a locally repairable invariant and would add versioning behavior to a feature introduced in the same release.

### 5.3 Mobile HUD allocation

Preserve the current DOM contract:

- `.storybook-hud`
- `.hud-vital-slots`
- `.hud-vital[data-resource-id]`
- `.story-progress-mini`
- `.dock-toggle`

At the narrow breakpoint, allocate explicit collision-safe grid columns rather than relying on `auto 1fr auto` with an unconstrained flex child. The vital group must be allowed to shrink without overlapping the center region. The implementation may shorten track widths at 390 px, but must not hide labels or numeric values.

Required layout behavior at 390 px:

- Both vital rows are visible.
- `체력`, `정신력`, both numeric values, the Cheon-gi label/value, gauge, and detail button are visible.
- No bounding boxes overlap.
- The HUD stays inside the viewport.
- Text does not wrap into a second HUD row unless the final implementation deliberately changes the mobile HUD to a tested two-row layout. Prefer a single row.

Add bounding-box assertions to `web/scripts/storybook-reference-qa.mjs`. At the 390 and 414 viewports, compare the rectangles for:

- the last `.hud-vital` against `.story-progress-mini`
- `.story-progress-mini` against `.dock-toggle`
- `.hud-vital-slots` against the viewport edges

Fail when horizontal intersection is greater than zero, allowing at most a 1 px rounding tolerance. Include useful rectangle values in the failure message.

### 5.4 Optional inventory details

In `web/src/ui/storybook/render.ts`, branch on `InventoryDetail` availability.

- With detail: retain the disclosure button, target panel, icon hook, description, type, and use-button behavior.
- Without detail: render a non-interactive item row with the icon and fallback label. Do not emit `aria-expanded`, `data-disclosure-toggle`, or `data-disclosure-target`.

Do not synthesize a fake description. Absence is a compatibility state, not an error message.

## 6. Work Packages

Order is fixed: **RF1 → RF2 → RF3 → RF4 → RF5**.

### WP-RF1 — Core terminal-state and insight invariants

Files:

- `crates/escape-core/src/turn.rs`
- `crates/escape-core/src/scene_page.rs`
- `crates/escape-core/tests/core_contract.rs`
- `crates/escape-core/tests/route_parity.rs` only if an ending-state fixture already belongs there

Implementation:

1. Move the global ending guard before `train:` dispatch.
2. Ensure ending ScenePages do not advertise a training affordance/action.
3. Add an ordered-unique insight helper and reuse it for bonus calculation and ScenePage projection.
4. Preserve unknown-ID behavior and first-occurrence display order.

Required tests:

- training with no leveling metadata is rejected
- training with zero available points is rejected
- training with an unknown ability ID is rejected
- multiple thresholds earn the correct available point count
- spent points reduce availability with saturating arithmetic
- training at cap 5 is rejected
- valid training changes one ability and `spent_stat_points`, but not turn/location/encounter
- training after an ending is rejected and no training action is exposed
- crossing one and multiple thresholds produces the exact `+ 수련 기회 {n}` log
- applying the same insight reward twice stores one ID and logs the gain once
- two different insights for one ability sum once each
- duplicated save IDs contribute one bonus and one ScenePage row
- success percentage includes the deduplicated insight bonus
- identical seed/turn/action still produces identical dice
- legacy save fields remain optional

Verification:

```bash
cargo test -p escape-core
cargo test -p escape-wasm
cargo test --workspace
```

Commit:

```text
fix(core): enforce terminal training and unique insight effects [WP-RF1]
```

### WP-RF2 — Inventory compatibility rendering

Files:

- `web/src/ui/storybook/render.ts`
- `web/src/ui/storybook/render.test.ts`

Implementation:

1. Split detailed disclosure rendering from detail-absent fallback rendering.
2. Keep labels escaped and icon hue deterministic.
3. Preserve enabled and disabled `use:{id}` behavior when details exist.

Required tests:

- detailed item emits a matching toggle and target ID
- usable item with an action emits enabled `사용`
- usable item without an action emits disabled `지금은 쓸 수 없다`
- non-usable item emits no use button
- absent `inventory_details` emits a labeled, non-interactive row
- absent details emit no false `aria-expanded` or disclosure target
- duplicate inventory IDs still receive unique DOM target IDs when details exist

Verification:

```bash
cd web
npx vitest run src/ui/storybook/render.test.ts
npx tsc --noEmit
npm run build
```

Commit:

```text
fix(web): preserve inventory rows without optional details [WP-RF2]
```

### WP-RF3 — Collision-safe mobile HUD and QA gate

Files:

- `web/src/styles/storybook.css`
- `web/scripts/storybook-reference-qa.mjs`
- a QA test file only if the repository already unit-tests the QA helper

Implementation:

1. Correct narrow HUD column allocation and shrink behavior.
2. Remove obsolete `.hud-slot-*` mobile rules if repository-wide search proves they have no live renderer or fixture dependency.
3. Add the explicit non-overlap QA assertions described in §5.3.
4. Keep reduced-motion behavior unchanged.

Required automated checks:

- 390 and 414 px HUD sibling rectangles do not intersect
- HUD scroll width does not exceed viewport width
- all required HUD selectors remain present
- existing five-viewport visual QA remains green

Manual acceptance:

1. Inspect `390x844.png` and `414x896.png` from the new QA output.
2. Confirm the second vital value ends before the Cheon-gi region begins.
3. Confirm the detail button remains fully visible and tappable.
4. Confirm values `0`, `5`, `95`, and `100` do not change the layout contract. Use a fixture/evaluate render if the live path does not naturally produce these values.
5. With motion enabled, trigger a health or sanity change and confirm the pulse ends at the normal fill style.
6. With reduced motion, confirm no pulse animation runs and the correct fill remains visible.

Verification:

```bash
cd web
npx vitest run
npx tsc --noEmit
npm run build
cd ..
wasm-pack build crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg
cd web
npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173/ --out-dir /tmp/tui-adv-gameloop3-refactor-qa --require-wasm
```

Note: `wasm-pack --out-dir` is resolved relative to the crate directory. Use `../../web/src/core/wasm-pkg`; do not use `web/src/core/wasm-pkg`, which writes under `crates/escape-wasm/`.

Commit:

```text
fix(web): prevent narrow HUD collisions and gate geometry [WP-RF3]
```

### WP-RF4 — Canonical repo closeout

Files:

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `fable_gameloop3_refactor_step2_report.md`

Implementation:

1. Mark Slice 3 implemented and review-fixed in the canonical Development Plan.
2. Remove or correct the stale Slice 2 “in progress” entry.
3. Update Checklist entries using its existing format; do not create a second source of next-work priority.
4. Record every command, test count, QA viewport, screenshot path, and manual acceptance result in the report.
5. Record deviations and remaining advisory debt. Do not claim the full manual flow unless every step was executed in this session.

Verification:

```bash
.venv/bin/pytest -q tests/test_docs_contract.py
git diff --check
```

Commit:

```text
docs(dev): close Gameloop 3 review findings [WP-RF4]
```

### WP-RF5 — Notion reverse sync and final ledger

Files and external target:

- `idea_box/notion_sources.yml`
- Notion page `13. 런타임 시스템 현황 (repo 동기화)`
- Notion `15. 기연 DB` only if its runtime-state rows do not already show the three implemented insights

Precondition: RF1–RF4 commits and their verification must be complete. Determine the final code/doc commit hash before editing the ledger.

Implementation:

1. Update the runtime-status page with the exact final base commit and a concise Slice 3 status:
   - leveling/training implemented
   - insight acquisition and check bonuses implemented
   - item and title detail surfaces implemented
   - post-merge HUD, terminal training, insight dedupe, and optional-detail fixes verified
2. Compare `15. 기연 DB` rows to the three runtime insight IDs. Change `런타임 상태` to implemented only for rows that map exactly to committed runtime definitions.
3. Update `idea_box/notion_sources.yml` timestamps, sync base commit, affected source status, and divergence notes using the existing schema.
4. Re-read the Notion pages after writing. Do not infer success from the write response alone.
5. Add the Notion page IDs/titles, final status, and re-read result to the refactor report.

Verification:

```bash
.venv/bin/pytest -q tests/test_docs_contract.py
git diff --check
git status --short --branch -uall
```

Commit:

```text
docs(sync): record Gameloop 3 refactor runtime state [WP-RF5]
```

If Notion access is unavailable, do not mark RF5 complete and do not claim the slice fully closed. Commit only an accurate repo-side ledger note if the ledger schema supports a pending external sync state; otherwise leave the WP uncommitted and report the blocker.

## 7. Final Verification Gate

Run from WSL on the final commit:

```bash
git status --short --branch -uall
cargo test --workspace
.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py
python3 scripts/export_web_data.py --check
python3 scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
cd web
npx vitest run
npx tsc --noEmit
npm run build
cd ..
wasm-pack build crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg
```

Start the web dev server, then run:

```bash
cd web
npm run qa:storybook:visual -- \
  --base-url http://127.0.0.1:5173/ \
  --out-dir /tmp/tui-adv-gameloop3-refactor-qa \
  --require-wasm
```

Final Git checks:

```bash
git diff --check
git status --short --branch -uall
git log --oneline --decorate -8
```

The final report must state exact totals. “Passed previously” is not evidence.

## 8. Explicitly Out of Scope

- New abilities, higher caps, respec, level UI, or XP redesign.
- New insight effect types or content rewards.
- Item drop, discard, capacity, or real sprite assets.
- Trait numeric effects or switching.
- Save schema migration or version bump.
- Bundle schema change or generated content rewrite.
- Route, encounter, destination, ending, or reward rebalance.
- General drawer redesign, typography polish, or unrelated CSS cleanup.
- Start screen, typewriter, action-beat, or transition redesign.

## 9. Risk and Rollback

Primary risk: the HUD fix may pass at 390 px while regressing wider layouts. The five-viewport QA plus explicit 390/414 geometry checks are mandatory.

Secondary risk: deduplication could reorder insights. Preserve first occurrence and test ScenePage order.

Rollback is commit-local. RF1, RF2, and RF3 are independently revertible without data migration. RF4 and RF5 must be reverted or corrected together only if their completion claims become false.

This plan assumes `c33fc75` remains the implementation baseline. If `main` moves, the implementer must re-read the affected functions and update the baseline in the report before editing; do not silently apply stale line-level instructions.

## 10. Final Checklist

- [ ] Baseline and clean worktree recorded before RF1.
- [ ] Ending-state training rejected by core and absent from the UI action surface.
- [ ] Duplicate insight IDs produce one bonus and one drawer row.
- [ ] Missing `inventory_details` produces a non-interactive, correctly labeled item row.
- [ ] 390 and 414 HUD geometry assertions pass.
- [ ] Five official QA viewports pass with rebuilt WASM.
- [ ] Motion-on pulse and reduced-motion resting state checked.
- [ ] No content YAML, route, reward, generated bundle, schema version, dependency, or storage-key drift.
- [ ] Development Plan and Checklist reflect the actual completed state.
- [ ] Notion runtime page and applicable insight rows re-read after update.
- [ ] `idea_box/notion_sources.yml` records the exact final sync base commit.
- [ ] Refactor report contains current-session command output totals and artifact paths.
- [ ] One verified commit per WP, in the prescribed order.
