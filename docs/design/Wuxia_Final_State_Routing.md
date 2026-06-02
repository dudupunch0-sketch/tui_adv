# Wuxia Final State Routing

Status: Notion-synced design contract, not runtime schema

Primary storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**

This document records the final-chapter state and routing vocabulary that must be settled before the next final-route runtime slices. It does not add a combat resolver, HP numeric battle, epilogue renderer, relation ledger, reward schema, or item payout. Runtime work still has to pass through `docs/dev/Development_Plan.md`.

## Source References

The 2026-06-02 `wuxia_seoharin_left_meal_followup` check compared late companion and final-route candidates against these Notion sources:

| source | Notion id | repo role |
|---|---|---|
| `최종장 결산 라우팅 마스터` | `37237e69-695e-81d2-8ce2-d1c738c3e923` | final result priority and final epilogue master matrix |
| `사도 최종전 상태값 사전` | `37337e69-695e-81c7-a9fd-e0a0e22005e2` | canonical final inputs and alias/deprecation policy |
| `사도 최종전` | `37237e69-695e-8169-97a3-d8106a817275` | required final battle container |
| `사도 최종전 1페이즈: 가격표` | `37237e69-695e-81e2-aac7-cecfce3e4239` | implemented first final-entry runtime slice after this contract |
| `사도 최종전 2페이즈: 약점 장악` | `37237e69-695e-8184-8beb-ccf56ae8fcd1` | implemented second final-entry runtime slice after phase 1 |
| `사도 최종전 3페이즈: 계산식 밖` | `37237e69-695e-8107-b9ab-cab708a6c5dd` | implemented third final-entry runtime slice after phase 2 |
| `보스 결산` | `37137e69-695e-8164-ab41-e794aa886dae` | deferred boss result card |
| `무명 결산` | `37137e69-695e-8159-8032-ce5f108ca6c8` | deferred Mumyeong salvation result card |
| `가지 말라는 말` | `37137e69-695e-8138-a41d-e153190f85aa` | deferred Seo Harin final relationship branch |

## Contract Boundary

`canonical_final_inputs` are descriptive design states, not a new runtime schema yet. They may be seeded by existing `flags`, `clues`, `log`, and `presentation` hooks until a dedicated schema is opened.

This contract explicitly keeps these surfaces closed:

- no combat resolver
- no HP numeric battle
- no final epilogue renderer
- no return/settlement schema
- no relation/debt/faction ledger
- no reward/ability schema
- no `item_unpriced_wooden_sword` payout
- no `told_seoharin_truth`

## Canonical Final Inputs

| key | allowed values | meaning |
|---|---|---|
| `combat_result` | `battle_loss`, `battle_victory` | final battle result, produced only by the final battle container |
| `boss_resolution_route` | `battle_loss`, `basic_victory`, `incomplete_victory`, `meaningful_victory`, `true_route_victory`, `corrupted_victory`, `mumyeong_unsaved_victory` | derived boss result route, not chosen directly by the player |
| `evidence_state` | `none_or_low`, `partial`, `strong` | whether ledger/evidence survives enough to affect outside judgment |
| `network_handling` | `ignored`, `partially_destroyed`, `ledger_secured`, `core_network_cut` | what happened to the Black Serpent network and ledgers |
| `pressure_state` | `unresolved`, `eased` | whether hostage/sect/public pressure was reduced before the result |
| `seoharin_axis` | `low`, `high`, `distorted_high` | Seo Harin relationship/consent/possession axis for late branches |
| `qingliu_rebuild` | `low`, `partial`, `high` | Qingliu recovery and future stability |
| `mumyeong_salvation` | `low`, `partial`, `substantial` | Mumyeong salvation trajectory, not a direct win flag |
| `successor_route` | `active`, `weakened`, `suppressed` | whether the Black Serpent successor logic survives through Mumyeong or others |
| `own_flow_choice` | `not_opened`, `opened_but_unresolved`, `chosen` | whether the player has chosen a method beyond copied forms |
| `truth_state` | `unknown`, `partial_truth_known`, `truth_known_and_not_forced` | truth handling, with consent boundaries preserved |
| `cheongirok_state` | `low_use`, `high_use`, `corruption_high` | how the Cheonggi Record was used and whether that use became corrupting |
| `player_method` | `outside_calculation`, `protected_as_person`, `direct_boss_focus`, `used_as_tool`, `sado_style_calculation` | final method classification used by routing |
| `item_logs` | `item_blackscale_ledger`, `item_old_black_serpent_hood`, `item_ownerless_orthodox_token`, `item_folded_qingliu_patron_letter`, `item_sado_brush`, `item_sado_old_glove`, `item_unpriced_wooden_sword` | final item log identifiers; these are not automatic item payouts |

## State Lifecycle

1. `wuxia_sado_final_phase_1_price_tag` can seed `network_handling`, `evidence_state`, `pressure_state`, and `item_logs` through existing encounter hooks.
2. `wuxia_sado_final_phase_2_weakpoint_control` can seed `seoharin_axis`, `qingliu_rebuild`, `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `truth_state`, `cheongirok_state`, `player_method`, and evidence pressure through existing encounter hooks.
3. `wuxia_sado_final_phase_3_outside_calculation` can seed `combat_result`, `boss_resolution_route` candidates, final `seoharin_axis`/`qingliu_rebuild`/`mumyeong_salvation`/`successor_route`/`own_flow_choice`, evidence pressure, `cheongirok_state`, and `player_method` interpretation through existing encounter hooks. These are still preview seeds, not a dedicated result schema.
4. `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, Black Serpent settlement, and Cheonggi Record settlement consume these states instead of recomputing them independently.

## Final Result Priority

`final_result_priority` follows this order when multiple conditions appear true:

1. `battle_loss`
2. `corrupted_victory`
3. `true_route_victory`
4. `mumyeong_unsaved_victory`
5. `meaningful_victory`
6. `incomplete_victory`
7. `basic_victory`

`final_epilogue_master_matrix` is a future consumer. This document only records its input vocabulary; it does not implement the epilogue matrix.

## State Alias And Deprecation Policy

`state_alias_and_deprecation_policy`:

- `item_log_state` is `local_helper_only`; do not promote it to a top-level canonical state.
- Deprecated aliases include `companion_state`, `organization_state`, `black_serpent_new_scale`, `successor_logic`, `route_pressure`, `unpriced_wooden_sword_condition`, `closed_gate_risk`, and `alliance_silence_variant`.
- New docs and runtime handoffs should use the canonical names above.

## Runtime Handoff

Deferred until this contract exists and a runtime handoff explicitly opens them:

- `wuxia_seoharin_unsaid_stay`
- full `wuxia_mumyeong_resolution` epilogue renderer beyond the implemented route seed bridge
- full `wuxia_boss_resolution` epilogue renderer beyond the implemented route seed bridge
- `wuxia_sado_final_battle`

Latest implemented runtime slice: `wuxia_seoharin_qingliu_resolution`.

`wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, and `wuxia_seoharin_qingliu_resolution` now use the existing encounter schema to seed final-state clues/flags/logs for the Sado final phases, boss-resolution route bridge, Mumyeong-resolution route bridge, and Seo Harin/Qingliu epilogue candidate bridge. The next runtime candidate is `wuxia_cheongirok_resolution` handoff. It must consume Cheonggi Record state and final epilogue candidate seeds without opening combat resolver, HP numeric combat, final epilogue schema, `item_unpriced_wooden_sword` payout, Seo Harin truth delivery, or `told_seoharin_truth` unless a new approved runtime contract opens them.
