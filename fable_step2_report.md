# Fable Step 2 Report

Date: 2026-07-08

## Completed work packages

1. WP-B1 — deduplicated content bundle start-location resolution into `ContentBundle::start_location_id()` and replaced terminal/wasm duplicate helpers.
2. WP-A1 — split `crates/escape-core/src/final_epilogue.rs` into the requested directory module layout.
3. WP-A2 — extracted duplicated final-epilogue card body strings into constants in `final_epilogue/cards.rs`.
4. WP-A3 — replaced fragile final-epilogue audit positional/index access with key/first-based handling.
5. WP-A4 — centralized `escape-core` resource ids and action-id prefixes.
6. WP-B2 — split `escape-terminal/src/main.rs` into focused terminal modules.
7. WP-B3 — deduplicated terminal action range/line formatters.
8. WP-C1 — centralized web localStorage keys and `StorageLike`.
9. WP-C2 — deduplicated shared web helpers (`errorMessage`, `DEFAULT_SEED`, player action button wiring).
10. WP-C3 — reshaped `main.ts` player-action dispatch into a command map.
11. WP-D1 — deduplicated `export_web_data.py` bundle check/stale-report paths and added private-secret mirror comments.

## Skipped / not completed in this run

- WP-C4 was optional and was not completed.

## Verification performed

- `cargo test --workspace` passed after every Rust-affecting work package and in the final verification pass.
- `cd web && npm install` completed with no dependency changes committed.
- `cd web && npm test && npx tsc --noEmit` passed after every web-affecting work package and in the final verification pass.
- `cd web && npm run build` passed after WP-C3 and in the final verification pass.
- WP-A2 byte-diff smoke check passed for `cargo run -p escape-terminal -- --scene content --seed 123 --tui-smoke` against the pre-WP-A2 tree.
- WP-B3 byte-diff smoke checks passed for `--tui-smoke`, `--app-smoke --tick 7`, and `--smoke` with `--scene content --seed 123` against the pre-WP-B3 tree.
- `./.venv/bin/python -m pytest tests/test_web_data_export.py -q` passed after WP-D1.
- `./.venv/bin/python scripts/export_web_data.py --check --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json` exited 0 after WP-D1.
- `./.venv/bin/python scripts/export_web_data.py --check --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json` exited 0 after WP-D1.

## Known verification notes

- `python3 -m pytest tests/ -q` fails in the non-venv Python because `yaml` is unavailable.
- `./.venv/bin/python -m pytest tests/ -q` still has the known docs-contract failures that read the old `crates/escape-core/src/final_epilogue.rs` path after WP-A1 split; targeted web-data export tests pass under the repo venv.
- `cargo fmt -p escape-core` and `cargo fmt -p escape-terminal` repeatedly formatted large test files; those unrelated formatting changes were reverted before commits.
