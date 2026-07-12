# Gameloop Slice 1 — Post-merge Fix Directive (Plan by Fable, implementation by gemini)

Date: 2026-07-12
Baseline: `main` HEAD `5ff5381` ("feat: 이구학지 게임 루프 1차 확장 — 스탯, 판정, 칭호, 경험치 (#134)")
Parent plan: [fable_gameloop_step1_2607121314.md](fable_gameloop_step1_2607121314.md)

Review of #134 found 2 defects that must be fixed and 3 procedural gaps.
Work through the WPs **in order, one commit per WP**. After each WP run its
verification. Record deviations in `fable_gameloop_step2_report.md` (WP-F3).

Environment: run all builds/tests in WSL at `~/work/tui-adv`
(`cargo`/`wasm-pack`/`gh` do not exist on Windows).

## Invariants (unchanged from parent plan)

- Do not add/rename/remove any existing `ScenePage` field. New-field rules
  from the parent plan stay in force (this directive needs no schema change).
- Any YAML content change must regenerate bundles in the same commit:
  `python3 scripts/export_web_data.py --write` then `--check` must pass,
  including the Rust fixtures under `crates/escape-core/fixtures/`.
- Action ids, localStorage keys, QA selectors: untouched.
- Do not weaken or delete contract asserts to make tests pass; update
  expected values only where this directive says so.

## Verification (after every WP, WSL)

```bash
cargo test --workspace
cd web && npm test && npx tsc --noEmit && npm run build && cd ..
./.venv/bin/python -m pytest tests/ -q        # must be 0 failed by WP-F1
python3 scripts/export_web_data.py --check    # WPs touching content
```

---

### WP-F1: Repair `tests/test_web_data_export.py` (5 failures on main)

`scripts/export_web_data.py` now emits `traits` (manifest count, per-file
JSON, bundle section) but the pytest contract was never updated. Currently:

```
E   Left contains 1 more item:
E   {'traits': 0}
FAILED tests/test_web_data_export.py::test_export_web_data_builds_public_manifest_with_expected_counts
```

Fix all 5 failing tests by extending the expected data — add `traits` to the
expected manifest counts dict, expected generated-file lists, and expected
bundle sections wherever asserted. Do not delete any existing assertion.
Add one new assertion: the wuxia preview bundle contains 2 traits with ids
`wuxia_apprentice`, `wuxia_swordmaster`.

**Verify**: `pytest tests/ -q` → 0 failed.
**Commit**: `test(data): sync web data export contract with traits [WP-F1]`

### WP-F2: Make the experience gauge reachable

Current state: `experience_target: 100`, but XP exists only in the first 3
encounters (15 per choice, choices are mutually exclusive) → max 45 per run.
The gauge can never pass ~45%. Also `swing_commute_bag` (in
`wuxia_heuksa_bang_first_fight`) is the only choice in its encounter with no
XP — unintended.

Fix in `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`:

1. Give `swing_commute_bag` `experience: 15` like its siblings.
2. Distribute XP across the main storyline so that **every route that reaches
   an ending accumulates ≥ 100**. Rule of thumb: every story-critical
   encounter resolution grants 10–15; minor/optional ones grant 0–5. Keep all
   choices within one encounter equal-XP unless the plan doc says otherwise
   (XP is a pacing gauge, not a scoring system — see
   `docs/design/Progression_and_Title_Model.md`).
3. Compute, don't guess: sum XP along each canonical route replayed by
   `crates/escape-wasm/tests/json_contract.rs` (those action sequences are
   the authoritative route list). Put the per-route sums in the step2 report.
4. If no clean distribution reaches 100 on every route, lower
   `experience_target` to the smallest per-route sum instead — reachability
   beats the round number.
5. Regenerate bundles + fixtures (`--write`, then `--check`).

Expected test fallout: XP delta lines (`+ 경험 N`) appear in more logs.
Update only the exact-log asserts in `json_contract.rs` /
`core_contract.rs` that now differ; keep the narrative strings intact.
Route reachability must not change — if `route_parity` fails, your content
edit changed a condition somewhere; revert that edit and redo.

**Commit**: `content(wuxia): distribute experience so target is reachable [WP-F2]`

### WP-F3: Write the missing step 2 report

Create `fable_gameloop_step2_report.md` (repo root), retroactively covering
#134 plus this fix cycle: per-WP status, deviations from the parent plan
(at minimum: history logs joined with `\n` into one entry instead of separate
entries; `json_contract.rs` needed no changes originally; `protagonist_name`
left as the default `당신`), the WP-F2 per-route XP table, and final
verification output summary.

**Commit**: `docs: add gameloop step2 report [WP-F3]`

### WP-F4: Remove the `tests/conftest.py` read_text monkeypatch

`tests/conftest.py` (from #133) globally patches `Path.read_text` so any test
reading `docs/dev/Development_Plan.md` silently gets
`Development_Plan_Archive.md` appended. This hides real doc drift and leaks
into every pytest.

1. Find the docs-contract tests that need the combined content
   (`grep -rn "Development_Plan" tests/`).
2. In those tests (or a small helper inside `test_docs_contract.py`), read
   both files explicitly and concatenate.
3. Delete `tests/conftest.py`.
4. `pytest tests/ -q` must stay 0 failed.

**Commit**: `test(docs): read plan archive explicitly, drop read_text monkeypatch [WP-F4]`

### WP-F5 (optional, skip if risky): Minor cleanups

- `protagonist_name`: replace the default `당신` with the honorific actually
  used by the wuxia narration if one exists (grep the pack YAML for how the
  protagonist is addressed); if the narration genuinely uses `당신`, drop the
  key entirely and note it in the report.
- `crates/escape-core/src/turn.rs` `ability_check_success_percent`: the
  `2 => 36` match arm is dead (guarded by the `need <= 2` early return).
  Remove the arm; keep the tests.

**Commit**: `chore: gameloop minor cleanups [WP-F5]`

---

## Final checklist

- [ ] One commit per WP, in order F1 → F5.
- [ ] `pytest tests/ -q` 0 failed (was 5 failed on baseline).
- [ ] `cargo test --workspace`, `npm test`, `tsc`, `npm run build` all green.
- [ ] `export_web_data.py --check` green after content edits.
- [ ] Per-route XP sums ≥ `experience_target` documented in the step2 report.
- [ ] `tests/conftest.py` deleted; docs-contract tests read the archive explicitly.
- [ ] Deviations recorded in `fable_gameloop_step2_report.md`.

Out of scope: any visual styling (reserved for the Claude-owned polish step),
new ScenePage fields, new systems, balance changes beyond XP distribution.
