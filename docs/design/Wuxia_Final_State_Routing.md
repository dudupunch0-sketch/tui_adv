# Wuxia Final State Routing

Status: Notion-synced design contract + final epilogue runtime implementation + return/settlement epilogue branch runtime, no dedicated result schema

Primary storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**

This document records the final-chapter state, routing vocabulary, final epilogue seed-consumption contract, and return/settlement branch consumption used by the current runtime slices. It does not add a combat resolver, HP numeric battle, full return/settlement schema, relation ledger, reward schema, or item payout. Runtime work still has to pass through `docs/dev/Development_Plan.md`.

## Source References

The 2026-06-02 `wuxia_seoharin_left_meal_followup` check compared late companion and final-route candidates against these Notion sources:

| source | Notion id | repo role |
|---|---|---|
| `최종장 결산 라우팅 마스터` | `37237e69-695e-81d2-8ce2-d1c738c3e923` | final result priority and final epilogue master matrix |
| `08. 엔딩과 후일담 연결` | `37137e69-695e-8177-a228-d7f96d084622` | epilogue card map and output/suppress operating principle |
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
- no dedicated final epilogue schema beyond structured `ScenePage.body_blocks`
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

`final_epilogue_master_matrix` is consumed by `wuxia_final_epilogue_renderer_contract` as the first runtime consumer. This document records its input vocabulary and the seed-consumption contract; the current runtime implementation exposes the result as structured `ScenePage.body_blocks`, not as a new top-level result schema.

## Final Epilogue Renderer Contract Handoff

Decision from the 2026-06-02 `wuxia_final_epilogue_renderer_contract_handoff`: no additional seed bridge is required before opening the final epilogue contract. `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, and `wuxia_black_serpent_aftermath` already leave the candidate seeds needed by the Notion master matrix.

The first implementation slice opened a core-owned final epilogue seed consumer under this boundary:

- Rust GameCore owns final result priority, seed consumption, suppress resolution, and card ordering.
- Web Storybook and SuperLightTUI render the core result only; they must not recompute route quality, enable conditions, or suppress rules.
- Candidate seeds remain `flags`/`clues`/`log` inputs until a dedicated card-output schema is approved.
- The selected output shape is a structured body block convention inside the existing `ScenePage.mode: ending` path. Each emitted card exposes its card id, variant, group, consumed seeds, and body text; suppressed candidates expose their suppressed-by reason through `epilogue_suppressed`.
- The contract may output main result plus epilogue cards, but it must not add return/settlement, reward/ability, relation/debt/faction, or combat-resource systems.

Required preconditions for the first contract implementation:

- `boss_resolution_resolved`
- `mumyeong_resolution_resolved`
- `seoharin_qingliu_resolution_resolved`
- `cheongirok_resolution_resolved`
- `black_serpent_aftermath_resolved`
- `final_result_priority_applied_seeded`
- `final_combat_result_battle_victory_seeded` or an explicitly approved `battle_loss` path
- `final_state_routing_seeded`

The consumer must apply these steps in order:

1. Resolve `boss_resolution_route` through `final_result_priority`.
2. Build candidate card groups from boss, Mumyeong, Seo Harin/Qingliu, Cheonggi Record, and Black Serpent seeds.
3. Apply suppress rules before output. Suppressed cards are not emitted as independent simultaneous cards.
4. Allow coexistence only where the Notion matrix allows it, such as alliance silence with southern market rumor.
5. Emit cards as aftermath/reward text, not as moral scoring, combat scoring, or player blame.

Minimum candidate groups:

| group | candidate cards | required source |
|---|---|---|
| boss / Black Serpent | broken serpent, banner, alliance silence, southern market rumor | `wuxia_boss_resolution`, `wuxia_black_serpent_aftermath` |
| Mumyeong | own-flow salvation, second wooden sword, unsent apology, end of stolen forms, new scale/new shadow, last bowl | `wuxia_mumyeong_resolution` |
| Seo Harin / Qingliu | Seo Harin future, empty place, open gate, closed gate, Qingliu future, restored martial art | `wuxia_seoharin_qingliu_resolution` |
| Cheonggi Record | safe high-use last page, true-route blank-place variant, corruption variant, low-use silence | `wuxia_cheongirok_resolution` |

Mandatory suppress examples:

- `corrupted_victory` overrides `true_route_victory`.
- `true_route_victory` suppresses successor/new scale/new shadow, closed gate, last bowl, banner, and southern market rumor.
- `closed_gate` and `open_gate` are mutually exclusive.
- `empty_place` and `last_bowl` are mutually exclusive.
- banner and southern market rumor can coexist only when not suppressed by true route, core network cut, or eased pressure.
- strong evidence changes alliance silence into responsibility evasion, not proof failure.

Implementation choices resolved by the first slice:

- card output is structured body blocks first, not a new renderer-neutral `ScenePage` mode.
- suppressed-card audit output is core-owned and renderer-visible as `epilogue_suppressed` body blocks when a suppressed candidate was actually present.
- card order is fixed by group/order tables in Rust GameCore rather than route-score UI.
- `battle_loss` is recognized by result priority when its approved seed exists, but no numeric combat resolver or HP battle path is opened in this slice.

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

Latest implemented runtime slice: `wuxia_battle_loss_epilogue_contract`.

`wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, and `wuxia_black_serpent_aftermath` now use the existing encounter schema to seed final-state clues/flags/logs for the Sado final phases, boss-resolution route bridge, Mumyeong-resolution route bridge, Seo Harin/Qingliu epilogue candidate bridge, Cheonggi Record last-page bridge, and Black Serpent aftermath bridge. `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`, and `wuxia_battle_loss_epilogue_contract` consume those candidate/result seeds through Rust GameCore-owned structured body blocks without opening combat resolver, HP numeric combat, full return/settlement schema, `item_unpriced_wooden_sword` payout, Seo Harin truth delivery, Cheonggi Record recorder identity reveal, or `told_seoharin_truth` unless a new approved runtime contract opens them.

## Return/Settlement Contract Handoff

Decision from the 2026-06-02 `wuxia_return_settlement_contract_handoff`: the next contract surface after final epilogue UX/playtest is return/settlement, starting with `wuxia_seoharin_unsaid_stay` / `가지 말라는 말`.

Runtime status: implemented in the preview/main storypack path as `wuxia_seoharin_unsaid_stay`. This implementation inserted the trigger after `wuxia_seoharin_qingliu_resolution` and before `wuxia_cheongirok_resolution`, then made `wuxia_cheongirok_resolution` require `seoharin_unsaid_stay_resolved` so the late relationship seed cannot be skipped in the normal final route.

Notion evidence:

- `가지 말라는 말` explicitly opens return, settlement, and corruption afterword variations through Seo Harin's late relationship branch.
- `08. 엔딩과 후일담 연결` keeps "현대 귀환 성공 -> 돌아온 출근길 / 현대 잔상" as an ending/afterword link not yet represented in runtime.
- `11. True Ending 단일 루트` says the true-route baseline is complete and the next expansion is to insert return, settlement, corruption, conquest, Mumyeong-unsaved, and Seo-Harin-distortion branches into that baseline.

Selected next implementation slice:

| field | value |
|---|---|
| runtime id | `wuxia_seoharin_unsaid_stay` |
| Notion source | `가지 말라는 말` / `37137e69-695e-8138-a41d-e153190f85aa` |
| insert after | `wuxia_seoharin_qingliu_resolution` |
| insert before | `wuxia_cheongirok_resolution` |
| purpose | seed return/settlement/corruption relationship intent before Cheonggi Record and final epilogue consume late final context |

First runtime scope:

- Use the existing encounter schema only.
- Add a late relationship trigger that asks whether the protagonist wants to return, stay, remain uncertain, or avoid the question.
- Leave `seoharin_unsaid_stay_resolved` and `final_return_settlement_contract_seeded`.
- Seed return/settlement/corruption candidates through flags/clues/log/presentation only.
- Do not create a new return ending schema, modern-life settlement schema, relation ledger, or save/archive surface.

Implementation result:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml` owns the runtime source.
- Rust/Web generated preview bundles include the encounter.
- `wuxia_cheongirok_resolution` now requires `seoharin_unsaid_stay_resolved`.
- The next contract decision is `return_settlement_followup_handoff`, which compares full return ending, settlement afterword, corruption/closed-gate branch, battle-loss path, reward/ability schema, and relation/debt/faction ledger.

Stable choice ids for the first runtime:

| choice id | meaning | expected seed direction |
|---|---|---|
| `say_return_home_honestly` | 귀환하고 싶다고 솔직히 말한다 | return intent without erasing Seo Harin's place |
| `say_you_will_stay_with_qingliu` | 청류문에 남겠다고 말한다 | settlement intent without treating belonging as payment |
| `share_uncertainty_without_running` | 아직 모르겠다고 말한다 | both return and settlement remain open |
| `turn_away_from_the_empty_place` | 말을 돌린다 | closed-gate / distortion risk candidate |

Expected seed vocabulary:

```yaml
return_settlement_contract_handoff:
  required_flags:
    - seoharin_qingliu_resolution_resolved
    - final_state_routing_seeded
    - final_result_priority_applied_seeded
    - final_combat_result_battle_victory_seeded
  common_flags:
    - seoharin_unsaid_stay_resolved
    - final_return_settlement_contract_seeded
  candidate_flags:
    return:
      - final_return_intent_honest_seeded
      - final_epilogue_return_absence_candidate_seeded
    settlement:
      - final_settlement_intent_honest_seeded
      - final_epilogue_qingliu_settlement_candidate_seeded
    uncertain:
      - final_return_settlement_uncertain_shared_seeded
      - final_epilogue_empty_place_kept_open_seeded
    evasion:
      - final_return_settlement_evasion_seeded
      - final_epilogue_closed_gate_risk_seeded
  clue_examples:
    - leaving_can_still_leave_a_place
    - staying_is_not_payment
    - uncertainty_can_be_shared_without_escape
    - evasion_prices_waiting
```

Rejected as next contract:

- `battle_loss` path: the final epilogue consumer can recognize an approved loss seed, but opening it first would require a final battle container or explicit loss route before this late relationship branch has a place to attach.
- reward/ability schema: `천기록 / 천외편린 보상` remains important, but it is a broader three-choice growth system rather than the immediate final afterword contract.
- relation/debt/faction ledger: unresolved debt and faction pressure already appear as afterword principles; a ledger should wait until return/settlement and loss/corruption branches show which persistent axes are worth storing.

Guardrails:

- no full modern return ending or post-return settlement scene in this slice
- no new `ScenePage` mode
- no return/settlement save/archive schema
- no `told_seoharin_truth`
- no Seo Harin truth-delivery scene
- no `item_unpriced_wooden_sword` payout
- no relation/debt/faction ledger
- no reward/ability schema
- no combat resolver or HP numeric battle

## Return/Settlement Follow-up Handoff

Decision from the 2026-06-02 `return_settlement_followup_handoff`: the next implementation after `wuxia_seoharin_unsaid_stay` is `wuxia_return_settlement_epilogue_contract`.

Notion comparison:

- `가지 말라는 말` already leaves four concrete player intents: honest return, honest settlement, shared uncertainty, and evasion.
- `08. 엔딩과 후일담 연결` links modern return to "돌아온 출근길 / 현대 잔상", while keeping Seo Harin/Qingliu cards as separate afterword cards.
- `01. 메인 엔딩 구조` keeps `return` and `settlement` as internal planning categories; player-facing output should use poetic titles such as "돌아온 출근길" and "청류문에 남은 외지인" rather than exposing enum names.
- `09. 예시 엔딩` has a concrete "돌아온 출근길" direction and a "청류문에 남은 외지인" direction, but marks these as verification examples that still need final story review.
- `10. 이구학지 후일담 카드 DB` has Seo Harin/Qingliu/closed-gate cards but no dedicated committed "돌아온 출근길" row yet, so the first runtime should be a branch card group in the existing final epilogue body block convention rather than a new archive/main-ending schema.
- `07. 천기록 / 천외편린 보상` is a broader three-choice reward/growth system and remains larger than this immediate afterword contract.
- `06. 사이드 퀘스트와 미해결 부채` supports unresolved debt as afterword traces, not as a blocking ledger schema.

Selected implementation:

| field | value |
|---|---|
| runtime id | `wuxia_return_settlement_epilogue_contract` |
| implementation owner | `crates/escape-core/src/final_epilogue.rs` |
| output shape | existing `ScenePage.body_blocks` with `kind: epilogue_card` / `epilogue_suppressed` |
| input bridge | `wuxia_seoharin_unsaid_stay` seed flags |
| renderer role | Web Storybook and SuperLightTUI display only |

Implemented branch cards:

| card id | variant | consumed seeds |
|---|---|---|
| `epilogue_wuxia_returned_commute` | `honest_return` | `final_return_intent_honest_seeded`, `final_epilogue_return_absence_candidate_seeded` |
| `epilogue_wuxia_qingliu_settlement` | `honest_settlement` | `final_settlement_intent_honest_seeded`, `final_epilogue_qingliu_settlement_candidate_seeded` |
| `epilogue_wuxia_empty_place_kept_open` | `uncertain_shared` | `final_return_settlement_uncertain_shared_seeded`, `final_epilogue_empty_place_kept_open_seeded` |
| `epilogue_wuxia_closed_gate_risk` | `evasion_risk` | `final_return_settlement_evasion_seeded`, `final_epilogue_closed_gate_risk_seeded` |

Conflict rule:

- `epilogue_wuxia_closed_gate_risk` suppresses the optimistic return/settlement/open-place branch cards if contradictory seeds are manually present in the same state.
- It does not automatically become `epilogue_seoharin_closed_gate`; the closed gate remains a darker corruption/possession result owned by the existing Seo Harin/Qingliu axis.

Still closed:

- new `main_ending_type` runtime enum
- full modern return ending scene or post-return settlement scene
- return/settlement save/archive schema
- relation/debt/faction ledger
- reward/ability schema and 천외편린 three-choice growth UI
- `item_unpriced_wooden_sword` payout
- combat resolver, HP numeric battle, or full `wuxia_sado_final_battle`
- Seo Harin truth delivery, `told_seoharin_truth`
- Cheonggi Record recorder identity reveal

Next contract decision: `return_settlement_epilogue_followup_handoff`, comparing battle-loss branch, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, and main ending archive/save surfaces after the return/settlement branch cards have runtime evidence.

## Return/Settlement Epilogue Follow-up Handoff

Decision from the 2026-06-02 `return_settlement_epilogue_followup_handoff`: the next implementation after `wuxia_return_settlement_epilogue_contract` is `wuxia_battle_loss_epilogue_contract`.

Notion comparison:

- `최종장 결산 라우팅 마스터` already defines `battle_loss` as the first final result priority and maps it to `epilogue_boss_black_serpent_banner`, `epilogue_wuxia_southern_market_rumor`, `epilogue_mumyeong_black_serpent_new_scale`, `epilogue_seoharin_closed_gate`, and `epilogue_tianjilu_last_page_corruption_variant`.
- `사도 최종전` and `사도 최종전 상태값 사전` both treat `combat_result: battle_loss` as a canonical final value, but they do not require a numeric HP resolver for this handoff.
- `08. 엔딩과 후일담 연결` repeats the battle-loss output bundle and frames it as darker afterword bias, not player blame.
- `흑사방의 깃발`, `검은 뱀의 새 비늘`, `닫힌 산문`, and `천기록의 마지막 장` all have battle-loss/corruption variants in the epilogue DB.
- The broader closed-gate/corruption branch is already partially represented by `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page` corruption variant, and corrupted result priority; the missing immediate gap is the loss result bundle itself.
- `07. 천기록 / 천외편린 보상` still implies a broader three-choice reward/ability system and should remain future work.
- `06. 사이드 퀘스트와 미해결 부채` supports unresolved pressure as afterword traces, but it does not require a relation/debt/faction ledger before the loss bundle can be printed.
- `01. 메인 엔딩 구조` keeps `main_ending_type` as internal planning vocabulary, so this slice should not open a player-facing archive/save surface.

Selected implementation:

| field | value |
|---|---|
| runtime id | `wuxia_battle_loss_epilogue_contract` |
| implementation owner | `crates/escape-core/src/final_epilogue.rs` |
| output shape | existing `ScenePage.body_blocks` |
| input bridge | explicit `final_combat_result_battle_loss_seeded` final-state seed |
| renderer role | Web Storybook and SuperLightTUI display only |

Intended battle-loss bundle:

| card id | variant direction | source |
|---|---|---|
| `epilogue_boss_black_serpent_banner` | `battle_loss_residue` | Notion `흑사방의 깃발` |
| `epilogue_wuxia_southern_market_rumor` | `unresolved_debt` | Notion `06`, `08`, final routing master |
| `epilogue_mumyeong_black_serpent_new_scale` | `battle_loss_successor_pressure` | Notion `검은 뱀의 새 비늘` |
| `epilogue_seoharin_closed_gate` | `battle_loss_or_corruption` | Notion `닫힌 산문` |
| `epilogue_tianjilu_last_page` | `corruption_variant` | Notion `천기록의 마지막 장` |

Conflict rule:

- `battle_loss` should suppress optimistic victory cards such as `epilogue_boss_broken_black_serpent`, `epilogue_seoharin_open_gate`, and `epilogue_mumyeong_stolen_forms_stopped`.
- It may coexist with unresolved-pressure rumor cards.
- It must not imply that a full numeric combat resolver, HP battle, or playable defeat route has been opened.

Still closed:

- full `wuxia_sado_final_battle` container
- combat resolver or HP numeric battle
- main ending archive/save surface
- relation/debt/faction ledger
- reward/ability schema and 천외편린 three-choice growth UI
- full modern return ending scene or post-return settlement scene
- `item_unpriced_wooden_sword` payout
- Seo Harin truth delivery, `told_seoharin_truth`
- Cheonggi Record recorder identity reveal

Implementation status: `wuxia_battle_loss_epilogue_contract` is now implemented in `crates/escape-core/src/final_epilogue.rs`. It consumes `final_combat_result_battle_loss_seeded`, emits the five battle-loss cards above, suppresses optimistic victory cards through `battle_loss`, and keeps the final epilogue ending YAML gate from being victory-only.

Next contract decision: `wuxia_battle_loss_epilogue_followup_handoff`, comparing full final battle container, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface, and playable defeat-route bridge after the battle-loss branch cards have runtime evidence.
