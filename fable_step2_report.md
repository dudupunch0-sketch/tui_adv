# Fable Step 2 Report

Date: 2026-07-08

## Merge-recovery note

During PR merge/replay recovery, the Step 2 code was present but `crates/escape-terminal/src/main.rs` still retained the pre-split implementations after the new WP-B2 module files had been added. This meant the split module files existed but were shadowed by duplicate local functions in `main.rs`.

Recovery action in this pass:

- Re-checked `fable_step1_2607080932.md` from WP-A4 onward.
- Confirmed WP-A4 resource/action-prefix centralization is present in `crates/escape-core/src/resources.rs` and that the listed core call sites use those constants.
- Removed the stale duplicated terminal implementations from `crates/escape-terminal/src/main.rs`, leaving `main.rs` with the intended WP-B2 responsibilities only: `main()`, `run()`, `run_printer_scene()`, and `run_content_scene()`.
- Re-ran main verification directly after the cleanup.

## Completed work packages present in the recovered tree

1. WP-B1 — deduplicated content bundle start-location resolution into `ContentBundle::start_location_id()` and replaced terminal/wasm duplicate helpers.
2. WP-A1 — split `crates/escape-core/src/final_epilogue.rs` into the requested directory module layout.
3. WP-A2 — extracted duplicated final-epilogue card body strings into constants in `final_epilogue/cards.rs`.
4. WP-A3 — replaced fragile final-epilogue audit positional/index access with key/first-based handling.
5. WP-A4 — centralized `escape-core` resource ids and action-id prefixes.
6. WP-B2 — split `escape-terminal/src/main.rs` into focused terminal modules; merge recovery removed stale duplicate implementations left behind in `main.rs`.
7. WP-B3 — deduplicated terminal action range/line formatters.
8. WP-C1 — centralized web localStorage keys and `StorageLike`.
9. WP-C2 — deduplicated shared web helpers (`errorMessage`, `DEFAULT_SEED`, player action button wiring).
10. WP-C3 — reshaped `main.ts` player-action dispatch into a command map.
11. WP-D1 — deduplicated `export_web_data.py` bundle check/stale-report paths and added private-secret mirror comments.
12. WP-C4 — split Web Storybook final-epilogue rendering helpers into `web/src/ui/storybook/renderEpilogue.ts`.

## Skipped / not completed

- No Fable Step 1 work packages remain open.

## Verification performed in this recovery pass

- `cargo test --workspace` passed.
- `cd web && npm test && npx tsc --noEmit` passed.
- `cd web && npm run build` passed after WP-C4.
- `./.venv/bin/python -m pytest tests/ -q` was run and still has the known docs-contract failures that read the old `crates/escape-core/src/final_epilogue.rs` path after the WP-A1 split; all other pytest tests passed.

## Known verification notes

- `python3 -m pytest tests/ -q` may fail in the non-venv Python when `yaml` is unavailable; use the repo venv for pytest.
- The remaining repo-venv pytest failures are docs-contract path-sync failures caused by tests still reading `crates/escape-core/src/final_epilogue.rs` after WP-A1 split.
- `cargo fmt -p escape-terminal` can format large test files; unrelated test formatting was reverted during recovery.
