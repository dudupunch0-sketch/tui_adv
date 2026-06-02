---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_final_epilogue_renderer_contract_implementation
previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff
mode: contract-first-runtime-implementation
---

# next_goal

이 폴더는 다른 Hermes/agent 세션에 넘길 단일 prompt entry point다. 새 세션에는 긴 프롬프트를 복사하지 말고 아래처럼 짧게 지시한다.

```text
이 repo의 idea_box/next_goal/ 폴더를 읽고 README의 현재 목표만 수행해. repo canonical docs와 충돌하면 canonical docs를 우선하고 충돌 사실을 보고해.
```

운영 원칙:

- 이 README는 “지금 다음으로 할 일” 하나만 가리킨다.
- 목표가 바뀌면 새 파일을 추가하지 말고 이 README를 교체/갱신한다.
- 최종 source of truth는 이 README가 아니라 repo canonical docs다.

## 현재 목표

`wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

`wuxia_seoharin_left_meal` runtime implementation은 완료됐다. `wuxia_seoharin_left_meal_followup` docs-only handoff도 완료됐고, 그 결과 직접 final runtime을 열기 전에 `docs/design/Wuxia_Final_State_Routing.md`로 final-route state dictionary/result routing contract를 먼저 고정했다.

`wuxia_sado_final_phase_1_price_tag` runtime implementation도 완료됐다. 사도 최종전 1페이즈는 combat resolver 없이 ledger/evidence/pressure/item-log seed만 기존 encounter schema로 남겼다.

`wuxia_sado_final_phase_2_weakpoint_control` runtime implementation도 완료됐다. 사도 최종전 2페이즈는 combat resolver 없이 서하린/무명/청류문/천기록 압박 대응을 final-state seed로만 남겼다.

`wuxia_sado_final_phase_3_outside_calculation` runtime implementation도 완료됐다. 사도 최종전 3페이즈는 combat resolver 없이 `combat_result`/`boss_resolution_route` 후보와 관계/증거/천기록/player_method seed만 남겼다.

`wuxia_boss_resolution` runtime implementation도 완료됐다. 보스 결산은 final epilogue renderer 없이 `final_result_priority`와 후속 epilogue candidate seed만 기존 encounter schema로 남겼다.

`wuxia_mumyeong_resolution` runtime implementation도 완료됐다. 무명 결산은 final epilogue renderer 없이 own-flow/relational/incomplete/stolen-forms/successor/corruption route seed와 후속 epilogue candidate seed만 기존 encounter schema로 남겼다.

`wuxia_seoharin_qingliu_resolution` runtime implementation도 완료됐다. 서하린/청류문 결산은 final epilogue renderer 없이 열린 산문/닫힌 산문, 서하린 후일담, 청류문 복구/잔존 압박 후보 seed만 기존 encounter schema로 남겼다.

`wuxia_cheongirok_resolution` runtime implementation도 완료됐다. 천기록 결산은 final epilogue renderer 없이 마지막 장의 high-use-not-corruption, blank true-route place, corruption, low-use silence, method-reflection 후보 seed만 기존 encounter schema로 남겼다.

`wuxia_black_serpent_aftermath` runtime implementation도 완료됐다. 흑사방 aftermath는 final epilogue renderer 없이 부러진 검은 뱀, 깃발 잔존, 남쪽 장터 빚 풍문, 무림맹 침묵, true-route 억제 후보 seed만 기존 encounter schema로 남겼다.

`wuxia_final_epilogue_renderer_contract_handoff` docs-only handoff도 완료됐다. Notion의 최종장 결산 라우팅 source, `08. 엔딩과 후일담 연결`, `사도 최종전 상태값 사전`, 지금까지 쌓인 보스/무명/서하린/청류문/천기록/흑사방 aftermath seed를 repo canonical docs와 다시 대조했고, 추가 seed bridge 없이 다음 implementation slice를 열 수 있다고 결정했다.

현재 목표는 `wuxia_final_epilogue_renderer_contract_implementation`이다. `docs/design/Wuxia_Final_State_Routing.md`의 `Final Epilogue Renderer Contract Handoff`를 기준으로 Rust GameCore가 candidate seed consumption, `final_result_priority`, suppress, card ordering을 소유하는 첫 구현 slice를 설계/구현한다. Web Storybook과 SuperLightTUI는 core 결과를 표시만 해야 하며, 후일담 카드 enable/suppress를 renderer에서 다시 계산하면 안 된다.

## 구현 전 비교할 후보

최소 다음 후보와 contract input을 비교한다.

- `wuxia_boss_resolution`
- `wuxia_mumyeong_resolution`
- `wuxia_seoharin_qingliu_resolution`
- `wuxia_cheongirok_resolution`
- `wuxia_black_serpent_aftermath`
- `wuxia_final_epilogue_renderer_contract`
- `epilogue_tianjilu_last_page`
- `epilogue_open_gate` / `epilogue_closed_gate`
- `wuxia_seoharin_unsaid_stay`
- `wuxia_seoharin_empty_place`
- `wuxia_seoharin_left_meal`
- `docs/design/Wuxia_Final_State_Routing.md`의 canonical final inputs와 boundary
- `docs/design/Wuxia_Final_State_Routing.md`의 `Final Epilogue Renderer Contract Handoff`
- possible output surface: renderer-neutral `ScenePage` mode vs structured body block convention

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.55`: `wuxia_seoharin_left_meal` preview runtime slice
  - section `0.56`: final state routing contract handoff
  - section `0.57`: `wuxia_sado_final_phase_1_price_tag` preview runtime slice
  - section `0.58`: `wuxia_sado_final_phase_2_weakpoint_control` preview runtime slice
  - section `0.59`: `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice
  - section `0.60`: `wuxia_boss_resolution` preview runtime slice
  - section `0.61`: `wuxia_mumyeong_resolution` preview runtime slice
  - section `0.62`: `wuxia_seoharin_qingliu_resolution` preview runtime slice
  - section `0.63`: `wuxia_cheongirok_resolution` preview runtime slice
  - section `0.64`: `wuxia_black_serpent_aftermath` preview runtime slice
  - section `0.65`: final epilogue renderer contract handoff
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/design/Wuxia_Final_State_Routing.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`

## 이미 완료된 기반

- Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
- `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`까지 runtime slice는 완료됐다.
- `docs/design/Wuxia_Final_State_Routing.md`는 `canonical_final_inputs`, `final_result_priority`, `final_epilogue_master_matrix`, final epilogue seed-consumption contract, state alias/deprecation policy를 소유한다.
- `wuxia_seoharin_left_meal`은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고 `seoharin_left_meal_resolved`로 반복을 막는다.
- stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
- 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`와 `left_meal_was_kept_for_return`/`belonging_is_daily_care` clue를 남기고, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`와 `last_bowl_epilogue_seeded` clue를 남긴다.
- `wuxia_sado_final_phase_1_price_tag`는 `seoharin_left_meal_resolved`, `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_first_appearance_resolved`, `black_serpent_core_pressure_opened`, `sealed_departure_truth_summary_prepared`, `midgame_continuity_started`를 요구하고 `sado_final_phase_1_price_tag_resolved`로 반복을 막는다.
- phase 1 stable choice id는 `approach_sado_before_the_ledger`, `burn_the_blackscale_ledger`, `secure_the_blackscale_ledger`, `ease_hostage_pressure_first`다.
- phase 1은 `final_state_routing_seeded`와 `final_price_tag_*` flags, `final_network_*_seeded`, `final_evidence_*_seeded`, `final_pressure_*_seeded`, `final_item_logs_blackscale_ledger_seeded` 같은 seed hooks만 남기며 실제 item payout이나 final battle을 열지 않는다.
- `wuxia_sado_final_phase_2_weakpoint_control`은 `sado_final_phase_1_price_tag_resolved`, `final_state_routing_seeded`를 요구하고 `sado_final_phase_2_weakpoint_control_resolved`로 반복을 막는다.
- phase 2 stable choice id는 `respond_to_seoharin_pressure`, `return_flow_to_mumyeong`, `read_dangerous_cheongirok_sentence`, `focus_on_sado`다.
- phase 2는 `final_seoharin_axis_high_seeded`, `final_qingliu_rebuild_partial_seeded`, `final_mumyeong_salvation_partial_seeded`, `final_successor_route_suppressed_seeded`, `final_own_flow_choice_opened_seeded`, `final_cheongirok_state_high_use_seeded`, `final_cheongirok_corruption_risk_seeded`, `final_player_method_*` 같은 seed hooks만 남기며 combat/final result를 확정하지 않는다.
- `wuxia_sado_final_phase_3_outside_calculation`은 `sado_final_phase_2_weakpoint_control_resolved`, `final_phase_2_weakpoint_control_resolved`, `final_state_routing_seeded`를 요구하고 `sado_final_phase_3_outside_calculation_resolved`로 반복을 막는다.
- phase 3 stable choice id는 `remember_the_empty_place`, `let_mumyeong_choose_own_flow`, `endure_with_qingliu_will`, `point_to_blank_in_ledger`, `answer_with_sado_calculation`이다.
- phase 3은 `final_combat_result_battle_victory_seeded`, `final_boss_resolution_*_candidate_seeded`, `final_seoharin_axis_high_preserved_seeded`, `final_mumyeong_salvation_substantial_candidate_seeded`, `final_successor_route_suppressed_confirmed_seeded`, `final_evidence_strong_confirmed_seeded`, `final_cheongirok_state_corruption_high_seeded`, `final_player_method_*` 같은 후보 seed만 남기며 final battle/epilogue/result를 확정하지 않는다.
- `wuxia_boss_resolution`은 `sado_final_phase_3_outside_calculation_resolved`, `final_phase_3_outside_calculation_resolved`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`를 요구하고 `boss_resolution_resolved`로 반복을 막는다.
- boss resolution stable choice id는 `confirm_true_route_outside_calculation`, `confirm_meaningful_victory_with_evidence`, `confirm_incomplete_victory_residue`, `confirm_mumyeong_unsaved_successor_risk`, `confirm_corrupted_victory`다.
- boss resolution은 `final_boss_resolution_*_seeded`, `final_result_priority_applied_seeded`, `final_epilogue_candidates_*_seeded`, `final_mumyeong_resolution_required_seeded` 같은 후보 seed만 남기며 final epilogue renderer나 무명 결산을 확정하지 않는다.
- `wuxia_mumyeong_resolution`은 `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`를 요구하고 `mumyeong_resolution_resolved`로 반복을 막는다.
- mumyeong resolution stable choice id는 `ask_mumyeong_for_own_flow`, `reveal_boss_used_mumyeongs_wound`, `leave_room_for_unsent_apology`, `let_stolen_forms_end`, `confirm_black_serpent_successor_risk`, `judge_with_sado_style_calculation`다.
- mumyeong resolution은 `final_mumyeong_resolution_*_seeded`, `final_epilogue_mumyeong_*_candidate_seeded`, `final_epilogue_seoharin_*_candidate_seeded` 같은 후보 seed만 남기며 final epilogue renderer나 서하린/청류문 결산을 확정하지 않는다.
- `wuxia_seoharin_qingliu_resolution`은 `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`를 요구하고 `seoharin_qingliu_resolution_resolved`로 반복을 막는다.
- seoharin/qingliu resolution stable choice id는 `leave_the_gate_unclosed`, `record_qingliu_rebuild_without_glory`, `keep_empty_place_for_return_or_absence`, `mark_qingliu_pressure_still_unresolved`, `close_the_gate_with_sado_logic`다.
- seoharin/qingliu resolution은 `final_seoharin_qingliu_resolution_*_seeded`, `final_epilogue_seoharin_*_candidate_seeded`, `final_epilogue_qingliu_*_candidate_seeded` 같은 후보 seed만 남기며 final epilogue renderer나 return/settlement schema를 열지 않는다.
- `wuxia_cheongirok_resolution`은 `seoharin_qingliu_resolution_resolved`, `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`를 요구하고 `cheongirok_resolution_resolved`로 반복을 막는다.
- cheongirok resolution stable choice id는 `turn_the_last_page_without_question`, `leave_blank_as_unpriced_place`, `read_the_lines_that_align_like_ledger`, `close_record_before_it_becomes_answer`, `let_record_reflect_the_method`다.
- cheongirok resolution은 `final_cheongirok_resolution_*_seeded`, `final_epilogue_tianjilu_*_seeded`, `final_player_method_reflected_not_judged_seeded` 같은 후보 seed만 남기며 final epilogue renderer, return/settlement schema, 천기록 기록자 정체 reveal을 열지 않는다.
- `wuxia_black_serpent_aftermath`는 `cheongirok_resolution_resolved`, `seoharin_qingliu_resolution_resolved`, `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`를 요구하고 `black_serpent_aftermath_resolved`로 반복을 막는다.
- black serpent aftermath stable choice id는 `mark_broken_serpent_without_erasing_scars`, `fold_the_banner_without_calling_it_gone`, `send_ledger_to_alliance_and_watch_silence`, `listen_for_southern_market_debt_rumor`, `let_true_route_suppress_the_banner`다.
- black serpent aftermath는 `final_black_serpent_aftermath_*_seeded`, `final_black_serpent_banner_*_candidate_seeded`, `final_southern_market_rumor_*_candidate_seeded`, `final_alliance_silence_responsibility_evasion_seeded` 같은 후보 seed만 남기며 final epilogue renderer, return/settlement schema를 열지 않는다.
- `wuxia_final_epilogue_renderer_contract_handoff`는 추가 seed bridge 없이 다음 implementation slice를 열 수 있다고 결정했다.
- 첫 implementation은 최소 boss/흑사방, 무명, 서하린/청류문, 천기록 candidate group을 core-owned seed consumer로 소비해야 한다.
- suppress는 card output보다 먼저 적용한다. `corrupted_victory`는 `true_route_victory`보다 우선하고, true route는 successor/new scale/new shadow/closed gate/last bowl/banner/southern market rumor를 suppress한다.
- strong evidence가 있는 `epilogue_boss_alliance_silence`는 증거 부족이 아니라 책임 회피 변주로 출력한다.

## 금지선

- 서하린에게 무명 이탈 진실을 직접 전달하지 않는다.
- `told_seoharin_truth` flag를 추가하지 않는다.
- final battle이나 return/settlement 결산을 바로 구현하지 않는다.
- 보스 결산은 route seed bridge까지만 구현됐으며, 별도 승인된 runtime contract 없이 최종 결산 출력기나 후일담을 바로 구현하지 않는다.
- final epilogue renderer contract는 열 수 있지만, return schema, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천기록 identity reveal을 바로 열지 않는다.
- `item_unpriced_wooden_sword`를 실제 아이템으로 지급하지 않는다.
- legacy office bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key를 변경하지 않는다.
- `wuxia_final_epilogue_renderer_contract`를 구현하더라도 combat resolver, HP numeric battle, return/settlement schema를 바로 열지 않는다.

## 이번 implementation 산출물

첫 implementation slice를 정한 뒤 최소 다음 파일을 갱신한다.

- `docs/design/Wuxia_Final_State_Routing.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/storypacks.json`
- `idea_box/next_goal/README.md`
