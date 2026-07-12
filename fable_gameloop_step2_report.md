# Gameloop step 2 report

Date: 2026-07-12

## Work-package status

| WP | Status | Notes |
| --- | --- | --- |
| F1 | complete | Export contract now covers 	raits, including the wuxia preview trait IDs. |
| F2 | complete | Experience is reachable on every canonical ending route; preview bundles regenerated. |
| F3 | complete | This report records the post-merge fix cycle. |
| F4 | pending | Replace the global Path.read_text monkeypatch with explicit active-plan/archive reads. |
| F5 | complete | Removed the redundant protagonist override and unreachable probability arm. |

## Deviations from the parent plan

- History deltas are joined with \n into one history entry rather than appended as separate entries.
- json_contract.rs needed no changes in the original gameloop implementation.
- protagonist_name remains the default 당신; F5 will decide whether the narration supports a more specific honorific.
- F1s pre-existing test failures were reproduced as contract omissions and corrected in this fix cycle.

## F2 canonical-route XP reachability

The canonical replay routes in crates/escape-wasm/tests/json_contract.rs share the seven required opening resolutions: arrival, first fight, first fragment, Seo Harin rescue, apprentice entry, chore sparring, and raid split. Each now grants 15 experience regardless of choice, so all ending routes have the same minimum before their later branch-specific scenes.

| Canonical ending branch | Mandatory XP | Target | Result |
| --- | ---: | ---: | --- |
| righteous / Cheongirok resolution | 105 | 100 | reachable |
| wounded or deferred branch | 105 | 100 | reachable |
| Black Heaven branch | 105 | 100 | reachable |
| Heavenly Archive branch | 105 | 100 | reachable |

swing_commute_bag now grants 15 experience, matching its sibling choices.

## Verification summary

- WP-F1: ./.venv/bin/python -m pytest tests/ -q — 97 passed.
- WP-F2: cargo test --workspace, python3 scripts/export_web_data.py --check, cd web && npm test && npx tsc --noEmit && npm run build, and ./.venv/bin/python -m pytest tests/ -q — passed (97 pytest tests).
