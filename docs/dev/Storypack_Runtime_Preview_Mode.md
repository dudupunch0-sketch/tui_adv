# Storypack runtime preview mode

Status: 결정 문서 + final epilogue runtime 구현 완료 + return/settlement branch consumer 구현 완료

## Decision: separate preview mode first

첫 비-office runtime prototype은 legacy office runtime bundle에 바로 섞지 않고, **separate preview mode first**로 진행한다.

2026-06-01 `docs/dev/Development_Plan.md` 0.0b 이후 Web/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 전환했다. 2026-06-02 이후 terminal content scene도 bundle 인자를 생략하면 이구학지 fixture를 기본으로 사용한다. 아래 preview-mode contract는 기존 office bundle과 Rust fixture를 섞지 않기 위한 경계로 유지하며, office content는 legacy/parity 경로로 남긴다.

핵심 결정:

- `src/tui_adv/data/*.yaml` remains legacy office runtime content.
- `wuxia_jianghu_pack` is the Web/default storypack through `web/src/core/contentBundles.ts`; terminal `--scene content` also defaults to the same built-in bundle when no explicit content bundle is provided.
- legacy `escape-office` save/localStorage keys remain unchanged; Web 이구학지 runs use `igu-hakji.*` keys.
- no legacy bundle mixing: 기본 Rust office `content.bundle.json` and legacy generated content remain office-only fixtures.
- renderer는 `ScenePage`만 표시하고, world별 gameplay truth를 Web/SuperLightTUI renderer에서 재계산하지 않는다.
- implemented launcher/default wiring: terminal은 기본 content scene과 명시적 `--storypack-preview wuxia_jianghu_pack` 모두 같은 이구학지 bundle을 사용하고, Web 새 게임도 이구학지 default bundle을 사용한다.

## 왜 gating이 아니라 preview mode인가

`world_id`/`storypack_id` gating을 기본 runtime schema에 즉시 넣으면 다음 문제가 생긴다.

1. 기존 office YAML, Rust content bundle, Web generated bundle, save/localStorage key를 한 번에 건드리게 된다.
2. 첫 무협 prototype은 아직 gameplay schema 확장보다 “기존 encounter schema로 표현 가능한가”를 확인하는 단계다.
3. 기본 번들의 `default_location`, route smoke, Web player start/save UX가 office 전제를 갖고 있으므로, 무협 콘텐츠를 같은 bundle에 넣으면 시작 위치와 encounter-first routing이 쉽게 충돌한다.

따라서 첫 단계는 별도 preview mode다. 이 결정은 gating을 영구히 포기한다는 뜻이 아니다. preview mode로 `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`가 기존 renderer-neutral boundary에서 작동함을 확인했다. `wuxia_final_epilogue_renderer_contract`는 final state routing gate와 Cheonggi Record/Black Serpent seed 이후 core-owned structured body blocks를 출력하는 first final epilogue consumer이며, `wuxia_seoharin_unsaid_stay`는 full return schema 없이 return/settlement trigger seed를 남기는 first late relationship bridge다. `wuxia_return_settlement_epilogue_contract`는 그 seed를 `epilogue_wuxia_returned_commute`, `epilogue_wuxia_qingliu_settlement`, `epilogue_wuxia_empty_place_kept_open`, `epilogue_wuxia_closed_gate_risk` body block cards로 소비하되 새 return schema나 archive/save surface를 열지 않는다. 다중 storypack 선택 UI/save migration이 필요해질 때 runtime-level gating을 별도로 연다.

2026-06-02 `return_settlement_epilogue_followup_handoff`에서 battle-loss epilogue handoff selected 상태가 되었고, 다음 runtime 후보는 `wuxia_battle_loss_epilogue_contract`다. 이 후보도 renderer-neutral boundary를 유지하며 full combat resolver나 HP 숫자전을 열지 않는다.

## Preview mode contract

첫 runtime prototype은 다음 경계 안에서 구현한다.

1. legacy office runtime은 그대로 둔다.
   - `src/tui_adv/data/*.yaml`은 계속 `escape from the office` legacy 콘텐츠다.
   - `crates/escape-core/fixtures/content/content.bundle.json`와 `web/src/data/generated/content.bundle.json`는 legacy office artifact다.
2. 무협 prototype은 명시적 preview 입력으로만 열린다.
   - 예: `docs/content/storypack_db/`에서 고른 카드 1개를 별도 preview YAML/JSON fixture로 만들거나, 명시적 preview flag가 있는 export 경로에서만 bundle을 만든다.
   - preview path 이름에는 `wuxia_jianghu_pack` 또는 `storypack-preview`가 들어가야 한다.
3. preview bundle은 최소 metadata를 가진다.
   - `world_id: wuxia_jianghu`
   - `storypack_id: wuxia_jianghu_pack`
   - `runtime_mode: storypack_preview`
   - `default_location` 또는 opening scene이 office 기본 시작점과 구분되어야 한다.
4. renderer-neutral boundary를 유지한다.
   - Rust GameCore가 action eligibility/outcome/ending truth를 소유한다.
   - Web Storybook과 SuperLightTUI는 `ScenePage`와 action id만 표시/전달한다.
   - renderer는 `world_id`를 보고 gameplay branch를 계산하지 않는다.
5. save/key migration은 열지 않는다.
   - legacy `escape-office` save/localStorage keys remain unchanged.
   - Web default 이구학지 run은 `igu-hakji.*` key를 쓰고, legacy office save와 자동 호환시키지 않는다.

## 구현된 첫 prototype

`wuxia_commute_rift_arrival`을 첫 schema-less runtime preview로 구현했다.

Preview source / artifacts:

- source YAML: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml`
- Rust fixture bundle: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- Web generated preview bundle: `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

Runtime metadata:

```yaml
runtime_mode: storypack_preview
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
default_location: wuxia_commute_rift
```

`escape-terminal --scene content`와 `escape-wasm::new_game_json()`은 preview bundle의 `runtime.default_location`을 사용해 `dev_desk`가 아니라 `wuxia_commute_rift`에서 새 게임을 시작한다. `runtime` metadata가 없는 기본 office bundle은 기존처럼 `dev_desk`에서 시작한다.

Preview smoke:

```bash
python scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check

cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location
cargo test -p escape-wasm json_boundary_reaches_wuxia_first_fight_through_preview_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_apprentice_entry_through_preview_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_raid_route_split_through_preview_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_wounded_shelter_dawn_offers_through_preview_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_copy_style_reveal_through_preview_bundle
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_arrival
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_first_fight
cargo test -p escape-terminal content_tui_smoke_launches_wuxia_storypack_preview_by_opt_in_flag
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheonggi_record_first_fragment
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_seo_harin_rescue
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_apprentice_entry
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_raid_route_split
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_wounded_shelter_dawn_offers
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_copy_style_reveal
```

## 첫 prototype 후보

1. `wuxia_commute_rift_arrival` — 구현 완료
   - preview mode smoke에 가장 안전하다.
   - opening scene 자체가 office 기본 시작점과 분리되어야 한다는 contract를 잘 드러낸다.
   - 새 성장 schema 없이 flags/clues/items/logs로 표현 가능하다.
2. `wuxia_heuksa_bang_first_fight` — 구현 완료
   - 기존 schema-less combat prototype 경험을 재사용했다.
   - arrival가 이미 preview default-location/runtime metadata를 검증했으므로, 두 번째 slice는 같은 preview bundle 안에서 encounter 확장을 검증했다.
   - 목표는 승리/패배 숫자 전투가 아니라 “이 세계의 폭력이 실제다”, “출근복/구두/가방/사원증이 전투 변수다”, “서하린 구조 hook이 열린다”를 flags/clues/logs로 고정하는 것이다.
3. `wuxia_cheonggi_record_first_fragment` — 구현 완료
   - 첫 난투 뒤 천기록/천외편린 future hook을 여는 schema-less preview다.
   - 실제 천외편린 3택 reward/ability schema나 fragment choice UI는 열지 않고, choice별 thread flag/clue/log만 남긴다.
   - 정식 청류문 수습생/서고 구간은 `wuxia_cheongryu_apprentice_entry`에서 schema-less bridge로 회수했다.
4. `wuxia_seo_harin_rescue` — 구현 완료
   - first fight/first fragment 뒤 서하린 구조, 외지인 조사, 청류문 보호/감시 bridge를 연다.
   - `cheongryu_outer_courtyard` destination과 `seo_harin_rescue_resolved`/`taken_under_watch` 공통 hook을 preview bundle에만 남긴다.
5. `wuxia_cheongryu_apprentice_entry` — 구현 완료
   - rescue 뒤 청류문 수습생/잡역/채무/서고 curiosity bridge를 연다.
   - `cheongryu_apprentice_entry_resolved`/`cheongryu_trial_started`/`seo_harin_mentor_thread` 공통 hook과 `work_chore_token`을 preview bundle에만 남긴다.
6. `wuxia_cheongryu_raid_route_split` — 구현 완료
   - apprentice 뒤 청류문 습격과 정파/사파/천기·귀환 route pressure를 연다.
   - `cheongryu_raid_route_split_resolved`/`cheongryu_raid_survived`/`route_commitment_pressure` 공통 hook과 deferred fallback branch hook을 preview bundle에만 남긴다.
7. `wuxia_cheongryu_raid_wounded_fallback` — 구현 완료
   - raid split의 `evacuate_the_wounded_first` 뒤 route 선택을 미룬 대가와 재합류 hook을 연다.
   - `cheongryu_raid_wounded_fallback_resolved`/`deferred_route_reopened`와 정파·사파·천기 route starter flags를 preview bundle에만 남긴다.
8. `wuxia_baekdo_medicine_debt` — 구현 완료
   - 정파 route starter를 받아 백도맹 약상자/청류문 재건 채무 opener를 연다.
   - `righteous_route_started` + `cheongryu_rebuild_thread`만 eligibility로 쓰고, `baekdo_alliance_debt`/`baekdo_medicine_debt`는 branch flavor hook으로만 남긴다.
9. `wuxia_black_heaven_escape_price` — 구현 완료
   - 사파 route starter를 받아 흑천련 탈출로/도월 표식/시장 장부의 값 opener를 연다.
   - `sapa_route_started` + `dowol_debt`만 eligibility로 쓰고, `black_heaven_deal_marked`/`black_heaven_escape_marker`는 branch flavor hook으로만 남긴다.
10. `wuxia_heavenly_archive_previous_outsiders` — 구현 완료
   - 천기·귀환 route starter를 받아 천기각 이전 이방인 기록과 세계 균열 단서를 여는 opener다.
   - `cheonggi_return_route_started` + `cheonggi_record_targeted`만 eligibility로 쓰고, `heavenly_archive_contact`/`heavenly_archive_triage_map_seen`는 branch flavor hook으로만 남긴다.
   - 천기록 정체 reveal, return system, reward/ability schema는 열지 않는다.
11. `wuxia_wounded_shelter_dawn_offers` — 구현 완료
   - `stabilize_wounded_until_dawn` branch를 받아 부상자 피난처의 새벽 제안으로 route pressure를 다시 연다.
   - `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`만 eligibility로 쓰고, `survivor_roll_call_complete`/`route_delay_cost_recorded`는 branch flavor hook으로만 남긴다.
   - `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha` stable choice ids와 `route_commitment_reopened` hook을 남긴다.
   - route graph/faction reputation/debt ledger/relation schema 없이 기존 route starter flags를 다음 opener에 넘긴다.
12. `wuxia_mumyeong_copy_style_reveal` — 구현 완료
   - 첫 대치 이후 무명이 덧씌운 카피 무공 계열의 윤곽과 결함을 청류안/천기록 대비로 드러낸다.
   - `mumyeong_first_confrontation_resolved` + `mumyeong_rival_thread_opened` + `midgame_continuity_started`만 eligibility로 쓰고, 첫 대치 branch flags와 route opener flags는 flavor hook으로만 남긴다.
   - `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder` stable choice ids와 `mumyeong_copy_style_reveal_resolved`/`copy_style_hint_recorded` hook을 남긴다.
   - seed 기반 random copy-style table, 천외편린 reward/ability schema, boss combat, 무명 중반 재회, 무명 과거 진실 reveal은 열지 않는다.
13. `wuxia_mumyeong_reads_orthodox_style` — 구현 완료
   - copy-style reveal 뒤 무명의 특별한 눈이 읽어낸 정파식 제압술 흔적을 현악문/복호금쇄수 단서로 연결했다.
   - `mumyeong_copy_style_reveal_resolved` + `copy_style_hint_recorded` + `midgame_continuity_started` + `first_fragment_seen`를 eligibility로 쓴다.
   - 무명 이탈 진실 전체, 서하린에게 진실 전달, boss first appearance, 중반 재회, 3택 reward schema는 아직 열지 않는다.
14. `wuxia_mumyeong_midgame_reunion` — 구현 완료
   - orthodox-style trace 뒤 무명과 다시 마주치는 중반 rival reunion으로 구현했다.
   - `mumyeong_reads_orthodox_style_resolved` + `orthodox_style_trace_recorded` + `mumyeong_first_confrontation_resolved` + `mumyeong_rival_thread_opened`를 eligibility로 쓴다.
   - `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer` stable choice ids와 `mumyeong_midgame_reunion_resolved`/`mumyeong_mirror_thread_deepened` hook을 남긴다.
   - 무명 이탈 진실 전체, 서하린에게 진실 전달, boss first appearance/combat, full flashback, 3택 reward schema는 아직 열지 않는다.
15. `wuxia_boss_first_appearance` — 구현 완료
   - `wuxia_mumyeong_followup_after_midgame_reunion` handoff에서 선택했고, preview runtime으로 구현했다.
   - `mumyeong_midgame_reunion_resolved` + `mumyeong_mirror_thread_deepened` + `cheongryu_raid_survived` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step` stable choice ids와 `boss_first_appearance_resolved`/`boss_wall_thread_opened`/`black_serpent_core_pressure_opened` hook을 남긴다.
   - boss combat/final boss resolution, 무명 이탈 진실 전체, 서하린에게 진실 전달, full flashback, 3택 reward schema는 아직 열지 않는다.
16. `wuxia_mumyeong_request_for_aid` — 구현 완료
   - `wuxia_boss_followup_after_first_appearance` handoff에서 선택했고, preview runtime으로 구현했다.
   - `boss_first_appearance_resolved` + `boss_wall_thread_opened` + `black_serpent_core_pressure_opened` + `mumyeong_mirror_thread_deepened` + `orthodox_style_trace_recorded` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `search_the_rejected_aid_letters`, `follow_old_inn_rumors_about_mumyeong`, `ask_seo_harin_what_help_never_came`, `keep_the_failed_aid_record_unshown` stable choice ids와 `mumyeong_request_for_aid_resolved`/`mumyeong_failed_aid_thread_opened`/`orthodox_hypocrisy_thread_opened` hook을 남긴다.
   - 무명 이탈 진실 전체, 서하린에게 진실 전달, full flashback, boss combat/final boss resolution, 3택 reward schema는 아직 열지 않는다.

17. `wuxia_mumyeong_awakening` — 구현 완료
   - `wuxia_mumyeong_followup_after_failed_aid` handoff에서 선택했고, preview runtime으로 구현했다.
   - `mumyeong_request_for_aid_resolved` + `mumyeong_failed_aid_thread_opened` + `orthodox_hypocrisy_thread_opened` + `mumyeong_reads_orthodox_style_resolved` + `orthodox_style_trace_recorded` + `mumyeong_copy_style_reveal_resolved` + `copy_style_hint_recorded` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation` stable choice ids와 `mumyeong_awakening_resolved`/`mumyeong_awakening_thread_opened`/`copy_corruption_thread_opened` hook을 남긴다.
   - 무명 이탈 진실 전체, 서하린에게 진실 전달, full flashback, boss combat/final boss resolution, random copy-style/reward schema는 아직 열지 않는다.

18. `wuxia_qingliu_attack_after_war` — 구현 완료
   - `wuxia_mumyeong_followup_after_awakening` handoff에서 선택했고, preview runtime으로 구현했다.
   - `mumyeong_awakening_resolved` + `mumyeong_awakening_thread_opened` + `copy_corruption_thread_opened` + `mumyeong_request_for_aid_resolved` + `mumyeong_failed_aid_thread_opened` + `orthodox_hypocrisy_thread_opened` + `mumyeong_reads_orthodox_style_resolved` + `orthodox_style_trace_recorded` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack` stable choice ids와 `qingliu_attack_after_war_resolved`/`qingliu_attack_trace_confirmed`/`hyeonakmun_attack_thread_opened` hook을 남긴다.
   - full Qingliu attack flashback, 무명 이탈 진실 전체, 서하린에게 진실 전달, 정파 문파 멸문, boss recruit/final boss resolution, random copy-style/reward schema는 아직 열지 않는다.

19. `wuxia_mumyeong_destroys_orthodox_sect` — 구현 완료
   - `wuxia_qingliu_attack_after_war_followup` handoff에서 선택했고, preview runtime으로 구현했다.
   - `qingliu_attack_after_war_resolved` + `qingliu_attack_trace_confirmed` + `hyeonakmun_attack_thread_opened` + `mumyeong_awakening_resolved` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead` stable choice ids와 `mumyeong_destroys_orthodox_sect_resolved`/`hyeonakmun_destruction_thread_opened`/`departure_truth_thread_deepened` hook을 남긴다.
   - playable Hyeonakmun destruction combat, full flashback, Seo Harin truth delivery, Mumyeong salvation confirmation, boss recruitment, final boss resolution, random copy-style/reward schema는 아직 열지 않는다.

20. `wuxia_boss_recruits_mumyeong` — 구현 완료
   - `wuxia_mumyeong_destroys_orthodox_sect_followup` handoff에서 선택했고, preview runtime으로 구현했다.
   - `mumyeong_destroys_orthodox_sect_resolved` + `hyeonakmun_destruction_thread_opened` + `departure_truth_thread_deepened` + `boss_first_appearance_resolved` + `boss_wall_thread_opened` + `black_serpent_core_pressure_opened` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent` stable choice ids와 `boss_recruits_mumyeong_resolved`/`boss_recruitment_thread_opened` hook을 남긴다.
   - departure-truth reveal, Seo Harin truth delivery, Mumyeong salvation confirmation, boss combat/final boss resolution, random copy-style/reward schema는 아직 열지 않는다.

21. `wuxia_mumyeong_departure_truth_summary` — 구현 완료
   - `wuxia_boss_recruits_mumyeong_followup` handoff에서 선택했고, preview runtime으로 구현했다.
   - `boss_recruits_mumyeong_resolved` + `boss_recruitment_thread_opened` + `mumyeong_destroys_orthodox_sect_resolved` + `hyeonakmun_destruction_thread_opened` + `departure_truth_thread_deepened` + `mumyeong_request_for_aid_resolved` + `mumyeong_failed_aid_thread_opened` + `orthodox_hypocrisy_thread_opened` + `mumyeong_awakening_resolved` + `midgame_continuity_started`를 eligibility로 쓴다.
   - `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it` stable choice ids와 `mumyeong_departure_truth_summary_resolved`/`sealed_departure_truth_summary_prepared`/`truth_delivery_still_unopened` hook을 남긴다.
   - Seo Harin truth delivery, `told_seoharin_truth`, Mumyeong salvation confirmation, boss combat/final boss resolution, epilogue/return, random copy-style/reward schema는 아직 열지 않는다.

## 후속 slice 기준

`wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`는 같은 preview/main storypack 경로에 추가되었다. 이미 preview export/check command, Rust/Web preview bundle artifact, terminal default/explicit `--storypack-preview wuxia_jianghu_pack`, Web default 이구학지 start/save wiring이 있으므로, 다음은 launcher나 천외편린 reward schema가 아니라 `wuxia_battle_loss_epilogue_contract` 구현이다.

Latest implemented runtime: `wuxia_return_settlement_epilogue_contract`. `docs/design/Wuxia_Final_State_Routing.md` contract, Notion return/settlement and afterword sources, and the Seo Harin late relationship seed bridge were rechecked before implementation. The next runtime candidate is `wuxia_battle_loss_epilogue_contract`.

Latest implemented encounter runtime: `wuxia_seoharin_unsaid_stay`. It is a return/settlement trigger seed bridge inserted between Seo Harin/Qingliu resolution and Cheonggi Record resolution; it keeps full return/settlement schema closed.

Historical latest marker before the final epilogue implementation: Latest implemented runtime: `wuxia_black_serpent_aftermath`.
Historical latest marker sentence: Latest implemented runtime is `wuxia_black_serpent_aftermath`.

Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff. This historical handoff is now implemented; it remains here as the boundary note for older runtime slices. Current next gate is final epilogue UX/playtest and contract follow-up validation. The next slice must still avoid truth delivery, `told_seoharin_truth`, return/settlement schema, route/faction/relation/debt/reward schema, combat resolver/HP numeric battle, Cheonggi Record identity reveal, or `item_unpriced_wooden_sword` payout unless a new approved runtime contract opens them.

구현된 rescue slice:

```yaml
id: wuxia_seo_harin_rescue
status: implemented_in_storypack_preview
purpose: first fight/first fragment 이후 서하린 구조, 외지인 조사, 청류문 보호/감시, 수습생 bridge를 연다.
conditions:
  locations: [jianghu_market_street]
  required_flags: [heuksa_bang_first_fight_resolved, cheonggi_record_first_fragment_resolved]
  forbidden_flags: [seo_harin_rescue_resolved]
recommended_destination: cheongryu_outer_courtyard
choices:
  - id: tell_plain_truth              # fallback / safe honesty
  - id: ask_for_medical_help_first    # survival priority / rescue debt hook
  - id: explain_company_and_commute   # workplace memory misunderstanding
  - id: show_cheonggi_record_page     # risky record disclosure
  - id: hide_employee_badge           # high-risk concealment
schema_boundary:
  allowed: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden: [RelationScore, DebtLedger, FactionStanding, healing_schema, companion_schema, CombatState, reward_schema, ability_schema, fragment_choice_reward]
```

구현된 chore sparring slice:

```yaml
id: wuxia_cheongryu_chore_sparring
storypack_id: wuxia_jianghu_pack
runtime_mode: storypack_preview
status: implemented_in_storypack_preview
purpose: 청류문 수습생 잡역 중 작은 몸싸움을 이구학지/office 공용 basic combat action taxonomy로 검증한다.
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, cheonggi_record_awakened, first_fragment_seen]
  forbidden_flags: [cheongryu_chore_sparring_resolved]
choices:
  - id: step_back_with_firewood       # fallback / safe reposition
  - id: let_shoulder_turn_with_push   # flow response
  - id: plant_bare_foot_in_dust       # grounded body check
  - id: ask_harin_what_changed        # mentor question
schema_boundary:
  allowed: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden: [CombatState, combat resolver, HP 숫자전, skill cooldown, reward_schema, ability_schema, relation/debt/faction schema, default office bundle changes]
```

구현된 raid route split slice:

```yaml
id: wuxia_cheongryu_raid_route_split
storypack_id: wuxia_jianghu_pack
runtime_mode: storypack_preview
status: implemented_in_storypack_preview
start_after:
  - wuxia_cheongryu_apprentice_entry
required_flags:
  - cheongryu_apprentice_entry_resolved
  - cheongryu_trial_started
  - cheonggi_record_awakened
  - first_fragment_seen
  - cheongryu_chore_sparring_resolved
forbidden_flags:
  - cheongryu_raid_route_split_resolved
choices:
  - id: evacuate_the_wounded_first       # fallback / safe human priority
  - id: defend_cheongryu_with_white_path # righteous route pressure
  - id: trade_with_black_heaven          # sapa survival bargain
  - id: follow_heavenly_archive          # cheonggi/return truth pressure
common_outcome_hooks:
  - cheongryu_raid_route_split_resolved
  - cheongryu_raid_survived
  - route_commitment_pressure
non_goals:
  - new faction standing schema
  - route graph schema
  - companion death schema
  - mass combat/boss combat resolver
  - reward/ability/fragment-choice schema
  - default office bundle changes
```

구현된 wounded fallback slice:

```yaml
id: wuxia_cheongryu_raid_wounded_fallback
status: implemented_in_storypack_preview
precondition: `wuxia_cheongryu_raid_route_split` runtime slice와 `evacuate_the_wounded_first` branch hook.
purpose: 부상자 대피 fallback 이후 route 선택을 미룬 대가와 재합류 hook을 연다.
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [cheongryu_raid_route_split_resolved, route_commitment_deferred, wounded_saved_flag, cheongryu_raid_survived]
  forbidden_flags: [cheongryu_raid_wounded_fallback_resolved]
choices:
  - id: stabilize_wounded_until_dawn          # fallback / safe deferred recovery
  - id: ask_baekdo_for_medicine_not_command  # delayed righteous commitment
  - id: trade_black_heaven_bandages_for_exit # delayed sapa bargain
  - id: follow_archive_triage_map            # delayed cheonggi/return thread
common_outcome_hooks:
  - cheongryu_raid_wounded_fallback_resolved
  - deferred_route_reopened
  - route_commitment_deferred | righteous_route_started | sapa_route_started | cheonggi_return_route_started
schema_boundary:
  allowed: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden: [RouteGraph, FactionStanding, BranchLock, TriageSystem, CompanionDeath, MassCombat, boss_combat_resolver, CombatState, reward_schema, ability_schema, fragment_choice_reward, multi_ending_implementation]
```

`wuxia_baekdo_medicine_debt` — preview runtime 구현 완료. 첫 route opener는 정파/백도맹 약상자 채무 축으로 landing했다.

`wuxia_black_heaven_escape_price` — preview runtime 구현 완료. 첫 사파 route opener는 `sapa_route_started` + `dowol_debt`를 required flags로 쓰고, `black_heaven_deal_marked`와 `black_heaven_escape_marker`는 direct/deferred branch flavor hook으로만 읽는다. 후속 `route_opener_followup_after_black_heaven` handoff에서 천기·귀환 opener를 다음 후보로 골랐다.

`wuxia_heavenly_archive_previous_outsiders` — preview runtime 구현 완료. 첫 천기·귀환 route opener는 `cheonggi_return_route_started` + `cheonggi_record_targeted`를 required flags로 쓰고, `heavenly_archive_contact`와 `heavenly_archive_triage_map_seen`는 direct/deferred branch flavor hook으로만 읽는다. `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory` stable choice ids와 `cheonggi_return_route_opened` hook을 남기고, 천기록 정체 reveal/return system/reward schema는 열지 않는다.

`wuxia_wounded_shelter_dawn_offers` — preview runtime 구현 완료. `stabilize_wounded_until_dawn` branch가 남긴 `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`를 required flags로 쓰고, `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 flavor hook으로만 읽는다. `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha` stable choice ids와 `route_commitment_reopened` hook을 남긴다. triage/companion death/mass combat/route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않았다.

`wuxia_mumyeong_first_sighting` — preview runtime 구현 완료. 세 route opener outcome에 공통 `route_opener_resolved` flag를 추가하고, 세 route opener 이후 첫 common midgame bridge로 Notion 사건 카드 DB `무명 첫 목격`을 구현했다. `route_opener_resolved` + `cheongryu_raid_survived` + `cheongryu_trial_started` + `first_fragment_seen`를 required flags로 사용하며, stable choice ids는 `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form`다. 무명 존재/서하린 침묵/청류문식 카피 무공의 이질감을 flags/clues/log/presentation으로만 남기고, any-of condition schema, route graph, faction reputation, relation/debt ledger, combat schema, boss combat, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않았다.

`wuxia_mumyeong_first_confrontation` — preview runtime 구현 완료. Notion 사건 카드 DB `무명 첫 대치`는 첫 목격 이후가 선행 조건이고, 완승보다 버티기/분석/카피 무공 관찰이 핵심이다. Runtime은 `mumyeong_first_sighting_resolved` + `midgame_continuity_started` + `cheongryu_raid_survived` + `first_fragment_seen`를 required flags로 쓰고, `meet_mumyeong_head_on`, `endure_until_copy_flow_breaks`, `watch_seo_harin_hold_back`, `read_mumyeongs_copied_form`, `do_not_provoke_mumyeong` stable choice ids를 사용한다. 모든 outcome은 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `destination_id: cheongryu_outer_courtyard`를 남긴다. 이 대치는 전투 사건처럼 보이지만 combat resolver/schema, HP 숫자전, boss combat, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal을 열지 않고 기존 flags/clues/log/presentation만 사용한다.

`wuxia_mumyeong_copy_style_reveal` — preview runtime 구현 완료. Notion 사건 카드 DB `무명의 카피 무공 공개`는 첫 대치 이후가 선행 조건이고, 이번 회차 무명이 훔쳐 덧씌운 무공 계열과 결함을 드러낸다. Runtime은 `mumyeong_first_confrontation_resolved` + `mumyeong_rival_thread_opened` + `midgame_continuity_started`를 required flags로 쓰고, `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder` stable choice ids를 사용한다. 모든 outcome은 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard`를 남긴다. 이 사건은 random copy-style table이나 천외편린 reward schema가 아니라 copied-family/호흡 불일치/겉흐름 복사 clues만 남긴다.

`wuxia_mumyeong_reads_orthodox_style` — preview runtime 구현 완료. Notion 사건 카드 DB `무명의 정파 무공 간파`은 copy-style reveal 뒤 무명의 과거 시야와 정파식 제압술 흔적을 연결한다. Runtime은 `mumyeong_copy_style_reveal_resolved` + `copy_style_hint_recorded` + `midgame_continuity_started` + `first_fragment_seen`를 required flags로 쓰고, `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation` stable choice ids를 사용한다. 모든 outcome은 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard`를 남긴다. 이 사건은 현악문/복호금쇄수 clue만 남기고, `wuxia_qingliu_attack_after_war` full flashback이나 `wuxia_mumyeong_departure_truth_summary` 진실 reveal은 열지 않는다.

Launcher/entrypoint contract:

- Terminal: `escape-terminal --scene content --seed 123 --tui-smoke`는 기본으로 built-in 이구학지 fixture를 사용한다. `--storypack-preview wuxia_jianghu_pack`는 명시적 동일 경로로 남기며, `--content-bundle <path>`는 legacy office fixture 등 override에 사용한다. `--content-bundle`과 `--storypack-preview`는 함께 사용할 수 없다.
- Web: start screen의 새 게임 기본 경로는 `wuxia_jianghu_pack` / 이구학지다. 기존 Web start screen preview launcher는 이 default 전환 전 opt-in entrypoint였고, 이구학지 run은 `igu-hakji.*` localStorage key를 쓰며 legacy office save와 섞지 않는다.
- Web bundle registry: `web/src/core/contentBundles.ts`는 이구학지 default bundle JSON과 legacy office/generated bundle 경계를 분리해 제공한다.
- 기본 office artifact(`src/tui_adv/data/*.yaml`, 기본 Rust/Web `content.bundle.json`)는 launcher slice에서도 변경하지 않는다.

금지선:

- 기본 office bundle에 무협 encounter를 직접 추가하지 않는다.
- `ScenePage`에 world별 renderer field를 추가하지 않는다.
- `escape-office` save/localStorage key를 rename하지 않는다.
- 천기록/천외편린 3택 성장 schema를 열지 않는다.
- 실제 회사/통근 경로/사원증 정보 또는 private-only reality hint를 넣지 않는다.
- 새 `CombatState`, combat resolver, HP 숫자전, 스킬/쿨타임, reward/ability schema, relation/debt/faction/companion schema를 추가하지 않는다.

구현된 runtime design:

```yaml
id: wuxia_heuksa_bang_first_fight
conditions:
  locations: [jianghu_market_street]
  required_flags: [wuxia_arrival_hidden]  # or shared wuxia_arrival_resolved if both arrival choices should route here
  forbidden_flags: [heuksa_bang_first_fight_resolved]
presentation:
  visual_id: wuxia_heuksa_bang_first_fight
  speaker: 흑사방 말단
  layout: combat_intervention
  effect_cues:
    - kind: glyph_anomaly
      source: market_brawl
      stable_terms: [거리, 구두, 사원증]
choices:
  - id: run_toward_open_street        # fallback / safe retreat
  - id: deescalate_with_words         # social probe
  - id: swing_commute_bag             # improvised item use
  - id: loosen_tie_and_drop_shoes     # combat reposition
  - id: crash_in_with_body            # high risk body check
```

```yaml
id: wuxia_cheonggi_record_first_fragment
conditions:
  locations: [jianghu_market_street]
  required_flags: [heuksa_bang_first_fight_resolved]
  forbidden_flags: [cheonggi_record_first_fragment_resolved]
presentation:
  visual_id: wuxia_cheonggi_record_first_fragment
  speaker: 천기록
  layout: cheonggi_record
  effect_cues:
    - kind: glyph_anomaly
      source: notebook_oracle
      stable_terms: [업무수첩, 천기록, 실패 기록]
choices:
  - id: choose_guard_basics           # defensive training thread flag
  - id: choose_keep_feet_moving       # mobility training thread flag
  - id: choose_failure_log            # reflection/failure-log thread flag
  - id: close_notebook_without_choice # fallback / safe delay
```

기존 schema만 사용한다.

- `conditions.locations`, `required_flags`, `forbidden_flags`
- `choices[].cost`, `choices[].outcome.resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`
- optional `presentation.visual_id`, `speaker`, `layout`, `effect_cues`

검증은 최소 다음을 포함한다.

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
PYTHONPATH=src python3 -m pytest \
  tests/test_web_data_export.py \
  tests/test_docs_contract.py \
  tests/test_storypack_db.py \
  -q
python3 scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
cargo test -p escape-core --test content_bundle content_bundle_loads_optional_storypack_preview_runtime_metadata
cargo test -p escape-core --test content_bundle preview_fixture_indexes_wuxia_first_fight
cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location
cargo test -p escape-wasm json_boundary_reaches_wuxia_first_fight_through_preview_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_apprentice_entry_through_preview_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_cheongryu_raid_route_split_through_preview_bundle
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_arrival
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_first_fight
cargo test -p escape-terminal content_tui_smoke_launches_wuxia_storypack_preview_by_opt_in_flag
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheonggi_record_first_fragment
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_seo_harin_rescue
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_apprentice_entry
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_cheongryu_raid_route_split
python3 -m compileall -q src tests
cargo fmt --check
git diff --check
```
