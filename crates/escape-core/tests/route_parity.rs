use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content,
    new_game_from_content_at, scene_page_from_content, ContentIndex, GameState, SceneMode,
};

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");
const WUXIA_PREVIEW_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

fn content() -> ContentIndex {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    index_content_bundle(&bundle).expect("content bundle should index")
}

fn wuxia_content() -> ContentIndex {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).expect("wuxia bundle should load");
    index_content_bundle(&bundle).expect("wuxia bundle should index")
}

fn seen_all_encounters(state: &mut GameState, content: &ContentIndex) {
    state.seen_encounters = content
        .encounters()
        .map(|encounter| encounter.id.clone())
        .collect();
}

fn action_ids(state: &GameState, content: &ContentIndex) -> Vec<String> {
    scene_page_from_content(state, content)
        .expect("scene page should render")
        .actions
        .into_iter()
        .map(|action| action.id)
        .collect()
}

fn apply_sequence(mut state: GameState, content: &ContentIndex, actions: &[&str]) -> GameState {
    for action_id in actions {
        state = apply_action_from_content(&state, content, action_id)
            .unwrap_or_else(|error| panic!("{action_id} should resolve: {error}"))
            .state;
    }
    state
}

fn wuxia_final_state_with_flags(flags: &[&str], clues: &[&str]) -> GameState {
    let content = wuxia_content();
    let mut state = new_game_from_content_at(123, &content, "black_serpent_ledger_vault")
        .expect("wuxia final state should start in ledger vault");
    state.flags = flags.iter().map(|flag| (*flag).to_string()).collect();
    state.clues = clues.iter().map(|clue| (*clue).to_string()).collect();
    state
}

fn body_text_for_kind(page: &escape_core::ScenePage, kind: &str) -> String {
    page.body_blocks
        .iter()
        .filter(|block| block.kind == kind)
        .map(|block| block.text.as_str())
        .collect::<Vec<_>>()
        .join("\n---\n")
}

#[test]
fn movement_pages_expose_inventory_use_actions_and_full_pressure_resources() {
    let content = content();
    let mut state = new_game_from_content_at(123, &content, "dev_office")
        .expect("content-backed game should start at office");
    seen_all_encounters(&mut state, &content);
    state.inventory.push("first_aid_kit".to_string());
    state.player.health = 50;
    state.player.hunger = 100;
    state.player.thirst = 60;

    let page = scene_page_from_content(&state, &content).expect("scene page should render");

    assert_eq!(page.mode, SceneMode::Movement);
    assert!(page
        .actions
        .iter()
        .any(|action| action.id == "move:hallway"));
    assert!(page.actions.iter().any(|action| {
        action.id == "use:first_aid_kit" && action.kind == "use" && action.label == "구급상자"
    }));
    assert_eq!(
        page.inventory_summary.items,
        vec!["first_aid_kit".to_string()]
    );
    assert_eq!(
        page.status_summary
            .resources
            .iter()
            .map(|resource| resource.id.as_str())
            .collect::<Vec<_>>(),
        vec!["health", "sanity", "battery", "hunger", "thirst"]
    );
    assert!(page
        .pressure_cues
        .iter()
        .any(|cue| cue.kind == "high_thirst" && cue.resource_id == "thirst"));
}

#[test]
fn item_use_consumes_inventory_applies_effects_advances_turn_and_unlocks_no_duplicates() {
    let content = content();
    let mut state = new_game_from_content(123, &content).expect("content-backed game should start");
    state.inventory.push("first_aid_kit".to_string());
    state.player.health = 50;

    let result = apply_action_from_content(&state, &content, "use:first_aid_kit")
        .expect("usable inventory item should resolve through Rust core");

    assert_eq!(result.encounter_id, "item");
    assert_eq!(result.action_id, "use:first_aid_kit");
    assert_eq!(result.state.turn, 1);
    assert_eq!(result.state.player.health, 80);
    assert_eq!(result.state.player.hunger, 1);
    assert_eq!(result.state.player.thirst, 2);
    assert!(result.state.inventory.is_empty());
    assert_eq!(
        result.logs,
        vec!["구급상자를 사용했다. 플라스틱 붕대 냄새가 상처보다 먼저 덮였다.".to_string()]
    );

    let page = scene_page_from_content(&result.state, &content).expect("scene page should render");
    assert!(page.inventory_summary.items.is_empty());
}

#[test]
fn ability_check_choice_uses_seeded_rust_core_roll_and_applies_success_outcome() {
    let content = content();
    let mut state = new_game_from_content(123, &content).expect("content-backed game should start");
    state.player.abilities.insert("interface".to_string(), 4);

    let page = scene_page_from_content(&state, &content).expect("scene page should render");
    assert!(page
        .actions
        .iter()
        .any(|action| action.id == "choice:trace_packet_delay"));
    assert!(!page
        .blocked_actions
        .iter()
        .any(|action| action.id == "choice:trace_packet_delay"));

    let result = apply_action_from_content(&state, &content, "choice:trace_packet_delay")
        .expect("ability-gated action should resolve");

    assert_eq!(result.state.turn, 1);
    assert_eq!(result.state.player.battery, 98);
    assert!(result
        .state
        .clues
        .contains(&"delayed_packet_route".to_string()));
    assert!(result
        .state
        .flags
        .contains(&"network_truth_hint".to_string()));
    assert_eq!(
        result.logs,
        vec![
            "알림 패킷을 조심스럽게 붙잡았다.".to_string(),
            "지연 시간 사이에서 숨은 라우팅을 찾았다.".to_string(),
        ]
    );
}

#[test]
fn action_results_unlock_achievements_and_scene_page_summarizes_them() {
    let content = content();
    let state = new_game_from_content(123, &content).expect("content-backed game should start");

    let result = apply_action_from_content(&state, &content, "choice:check_message")
        .expect("content-backed action should resolve");

    assert_eq!(
        result.newly_unlocked_achievements,
        vec!["first_signal_received".to_string()]
    );
    assert_eq!(
        result.state.unlocked_achievements,
        vec!["first_signal_received".to_string()]
    );

    let page = scene_page_from_content(&result.state, &content).expect("scene page should render");
    assert_eq!(
        page.achievement_summary.unlocked,
        vec!["first_signal_received".to_string()]
    );
    assert!(page.achievement_summary.newly_unlocked.is_empty());
}

#[test]
fn ending_scene_pages_cover_escape_failure_truth_conquest_and_public_reality_rewards() {
    let content = content();

    let mut escape = new_game_from_content_at(123, &content, "emergency_stairs")
        .expect("content-backed game should start at stairs");
    escape.flags.push("escape_route_completed".to_string());
    let escape_page =
        scene_page_from_content(&escape, &content).expect("ending page should render");
    assert_eq!(escape_page.mode, SceneMode::Ending);
    assert_eq!(escape_page.title, "퇴근 성공");
    assert_eq!(escape_page.visual.id, "ending:escape_commute");
    assert!(escape_page.actions.is_empty());

    let mut failure = escape.clone();
    failure.flags.clear();
    failure.flags.push("spatial_exit_failed".to_string());
    let failure_page =
        scene_page_from_content(&failure, &content).expect("failure page should render");
    assert_eq!(failure_page.mode, SceneMode::Ending);
    assert_eq!(failure_page.title, "게임오버: 계단이 접혔다");

    let mut truth = new_game_from_content_at(123, &content, "security_room")
        .expect("content-backed game should start at security room");
    truth.inventory.push("ex_employee_memo".to_string());
    truth.clues.extend([
        "meeting_pattern_noticed".to_string(),
        "server_log_fragment".to_string(),
    ]);
    truth.flags.extend([
        "impossible_meeting_saved".to_string(),
        "isolation_protocol_revealed".to_string(),
    ]);
    let truth_page = scene_page_from_content(&truth, &content).expect("truth ending should render");
    assert_eq!(truth_page.mode, SceneMode::Ending);
    assert_eq!(truth_page.title, "격리 프로토콜의 진실");

    let mut conquest = new_game_from_content_at(123, &content, "server_room")
        .expect("content-backed game should start at server room");
    conquest.flags.push("network_admin_claimed".to_string());
    let conquest_page =
        scene_page_from_content(&conquest, &content).expect("conquest ending should render");
    assert_eq!(conquest_page.mode, SceneMode::Ending);
    assert_eq!(conquest_page.title, "사내망 관리자 권한");

    let mut hidden =
        new_game_from_content(123, &content).expect("content-backed game should start");
    hidden.inventory.push("crumpled_printout".to_string());
    hidden.flags.extend([
        "printer_secret_started".to_string(),
        "pantry_hint_seen".to_string(),
    ]);
    let hidden_page =
        scene_page_from_content(&hidden, &content).expect("hidden ending should render");
    assert_eq!(hidden_page.mode, SceneMode::Ending);
    assert_eq!(hidden_page.title, "첫 번째 현실 연결 힌트");
    let reward_text = hidden_page
        .body_blocks
        .iter()
        .map(|block| block.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(reward_text.contains("현실 연결 힌트: 첫 번째 현실 연결 힌트"));
    assert!(reward_text.contains("private_only"));
    assert!(!reward_text.contains("final_hint"));
    assert!(!reward_text.contains("treasure_location"));
}

#[test]
fn escape_commute_scene_page_includes_post_escape_aftermath_report() {
    let content = content();
    let mut escape = new_game_from_content_at(123, &content, "emergency_stairs")
        .expect("content-backed game should start at stairs");
    escape.flags.push("escape_route_completed".to_string());

    let page = scene_page_from_content(&escape, &content).expect("escape ending should render");
    let body = page
        .body_blocks
        .iter()
        .map(|block| block.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert_eq!(page.mode, SceneMode::Ending);
    assert_eq!(page.title, "퇴근 성공");
    assert!(body.contains("[POST-ESCAPE REPORT]"));
    assert!(body.contains("survivor_count: 1"));
    assert!(body.contains("evidence_level: 0"));
    assert!(body.contains("company_response: denial"));
    assert!(body.contains("employee_status: access_revoked"));
    assert!(body.contains("risk_level: ongoing"));
    assert!(body.contains("ENDING: 정문 밖"));
    assert!(!body.contains("final_hint"));
    assert!(!body.contains("actual_ip_address"));
    assert!(!body.contains("office_location"));
    assert!(!body.contains("treasure_location"));
}

#[test]
fn wuxia_final_epilogue_scene_page_outputs_core_owned_cards_and_true_route_suppression() {
    let content = wuxia_content();
    let state = wuxia_final_state_with_flags(
        &[
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded",
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_epilogue_candidates_true_route_seeded",
            "final_broken_black_serpent_epilogue_candidate_seeded",
            "final_black_serpent_banner_candidate_reinforced_seeded",
            "final_southern_market_rumor_candidate_reinforced_seeded",
            "final_mumyeong_resolution_own_flow_salvation_seeded",
            "final_epilogue_mumyeong_stolen_forms_stopped_candidate_seeded",
            "final_epilogue_mumyeong_second_wooden_sword_candidate_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
            "final_epilogue_mumyeong_new_shadow_variant_seeded",
            "final_epilogue_seoharin_future_candidate_seeded",
            "final_epilogue_seoharin_empty_place_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_closed_gate_candidate_seeded",
            "final_epilogue_qingliu_future_candidate_seeded",
            "final_epilogue_qingliu_restored_martial_art_conditional_seeded",
            "final_epilogue_tianjilu_true_route_variant_seeded",
        ],
        &["true_route_can_suppress_banner_and_rumor"],
    );

    let page =
        scene_page_from_content(&state, &content).expect("final epilogue page should render");
    let result_text = body_text_for_kind(&page, "epilogue_result");
    let card_text = body_text_for_kind(&page, "epilogue_card");
    let suppressed_text = body_text_for_kind(&page, "epilogue_suppressed");

    assert_eq!(page.mode, SceneMode::Ending);
    assert_eq!(page.title, "이구학지 결산");
    assert!(result_text.contains("final_result_key: true_route_victory"));
    assert!(result_text.contains("owned_by: Rust GameCore"));
    assert!(card_text.contains("card_id: epilogue_boss_broken_black_serpent"));
    assert!(card_text.contains("variant: true_route_victory"));
    assert!(card_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
    assert!(card_text.contains("card_id: epilogue_mumyeong_second_wooden_sword"));
    assert!(card_text.contains("card_id: epilogue_seoharin_future"));
    assert!(card_text.contains("card_id: epilogue_seoharin_empty_place"));
    assert!(card_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(card_text.contains("card_id: epilogue_qingliu_future"));
    assert!(card_text.contains("card_id: epilogue_qingliu_restored_martial_art"));
    assert!(card_text.contains("card_id: epilogue_tianjilu_last_page"));
    assert!(card_text.contains("consumed_seeds: final_epilogue_tianjilu_true_route_variant_seeded"));
    assert!(!card_text.contains("card_id: epilogue_boss_black_serpent_banner"));
    assert!(!card_text.contains("card_id: epilogue_wuxia_southern_market_rumor"));
    assert!(!card_text.contains("card_id: epilogue_mumyeong_black_serpent_new_scale"));
    assert!(!card_text.contains("card_id: epilogue_mumyeong_new_shadow"));
    assert!(!card_text.contains("card_id: epilogue_seoharin_closed_gate"));
    assert!(suppressed_text.contains("card_id: epilogue_boss_black_serpent_banner"));
    assert!(suppressed_text.contains("card_id: epilogue_wuxia_southern_market_rumor"));
    assert!(suppressed_text.contains("card_id: epilogue_mumyeong_black_serpent_new_scale"));
    assert!(suppressed_text.contains("card_id: epilogue_mumyeong_new_shadow"));
    assert!(suppressed_text.contains("card_id: epilogue_seoharin_closed_gate"));
    assert!(suppressed_text.contains("suppressed_by: true_route_victory"));
    assert!(!card_text.contains("told_seoharin_truth"));
    assert!(!card_text.contains("item_unpriced_wooden_sword payout"));
}

#[test]
fn wuxia_final_epilogue_consumes_return_settlement_branch_seeds() {
    let content = wuxia_content();
    let base_flags = [
        "boss_resolution_resolved",
        "mumyeong_resolution_resolved",
        "seoharin_qingliu_resolution_resolved",
        "cheongirok_resolution_resolved",
        "black_serpent_aftermath_resolved",
        "final_result_priority_applied_seeded",
        "final_combat_result_battle_victory_seeded",
        "final_state_routing_seeded",
        "final_boss_resolution_true_route_confirmed_seeded",
        "final_epilogue_candidates_true_route_seeded",
    ];

    let mut return_flags = base_flags.to_vec();
    return_flags.extend([
        "final_return_settlement_contract_seeded",
        "final_return_intent_honest_seeded",
        "final_epilogue_return_absence_candidate_seeded",
    ]);
    let return_page =
        scene_page_from_content(&wuxia_final_state_with_flags(&return_flags, &[]), &content)
            .expect("return branch epilogue should render");
    let return_cards = body_text_for_kind(&return_page, "epilogue_card");
    assert!(
        return_cards.contains("card_id: epilogue_wuxia_returned_commute"),
        "{return_cards}"
    );
    assert!(return_cards.contains("variant: honest_return"));
    assert!(return_cards.contains("final_epilogue_return_absence_candidate_seeded"));
    assert!(!return_cards.contains("card_id: epilogue_wuxia_qingliu_settlement"));

    let mut settlement_flags = base_flags.to_vec();
    settlement_flags.extend([
        "final_return_settlement_contract_seeded",
        "final_settlement_intent_honest_seeded",
        "final_epilogue_qingliu_settlement_candidate_seeded",
    ]);
    let settlement_page = scene_page_from_content(
        &wuxia_final_state_with_flags(&settlement_flags, &[]),
        &content,
    )
    .expect("settlement branch epilogue should render");
    let settlement_cards = body_text_for_kind(&settlement_page, "epilogue_card");
    assert!(settlement_cards.contains("card_id: epilogue_wuxia_qingliu_settlement"));
    assert!(settlement_cards.contains("variant: honest_settlement"));
    assert!(settlement_cards.contains("final_epilogue_qingliu_settlement_candidate_seeded"));

    let mut uncertain_flags = base_flags.to_vec();
    uncertain_flags.extend([
        "final_return_settlement_contract_seeded",
        "final_return_settlement_uncertain_shared_seeded",
        "final_epilogue_empty_place_kept_open_seeded",
    ]);
    let uncertain_page = scene_page_from_content(
        &wuxia_final_state_with_flags(&uncertain_flags, &[]),
        &content,
    )
    .expect("uncertain branch epilogue should render");
    let uncertain_cards = body_text_for_kind(&uncertain_page, "epilogue_card");
    assert!(uncertain_cards.contains("card_id: epilogue_wuxia_empty_place_kept_open"));
    assert!(uncertain_cards.contains("variant: uncertain_shared"));
    assert!(uncertain_cards.contains("final_epilogue_empty_place_kept_open_seeded"));
}

#[test]
fn wuxia_final_epilogue_evasion_risk_suppresses_return_settlement_candidates() {
    let content = wuxia_content();
    let state = wuxia_final_state_with_flags(
        &[
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded",
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_epilogue_candidates_true_route_seeded",
            "final_return_settlement_contract_seeded",
            "final_return_intent_honest_seeded",
            "final_epilogue_return_absence_candidate_seeded",
            "final_settlement_intent_honest_seeded",
            "final_epilogue_qingliu_settlement_candidate_seeded",
            "final_return_settlement_uncertain_shared_seeded",
            "final_epilogue_empty_place_kept_open_seeded",
            "final_return_settlement_evasion_seeded",
            "final_epilogue_closed_gate_risk_seeded",
        ],
        &[],
    );

    let page = scene_page_from_content(&state, &content)
        .expect("evasion risk branch epilogue should render");
    let card_text = body_text_for_kind(&page, "epilogue_card");
    let suppressed_text = body_text_for_kind(&page, "epilogue_suppressed");

    assert!(card_text.contains("card_id: epilogue_wuxia_closed_gate_risk"));
    assert!(card_text.contains("variant: evasion_risk"));
    assert!(!card_text.contains("card_id: epilogue_wuxia_returned_commute"));
    assert!(!card_text.contains("card_id: epilogue_wuxia_qingliu_settlement"));
    assert!(!card_text.contains("card_id: epilogue_wuxia_empty_place_kept_open"));
    assert!(suppressed_text.contains("card_id: epilogue_wuxia_returned_commute"));
    assert!(suppressed_text.contains("card_id: epilogue_wuxia_qingliu_settlement"));
    assert!(suppressed_text.contains("card_id: epilogue_wuxia_empty_place_kept_open"));
    assert!(suppressed_text.contains("suppressed_by: return_settlement_evasion"));
}

#[test]
fn wuxia_final_epilogue_battle_loss_outputs_loss_bundle_and_suppresses_optimistic_cards() {
    let content = wuxia_content();
    let state = wuxia_final_state_with_flags(
        &[
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_loss_seeded",
            "final_state_routing_seeded",
            "final_broken_black_serpent_epilogue_candidate_seeded",
            "final_epilogue_mumyeong_stolen_forms_stopped_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_seeded",
        ],
        &[],
    );

    let page =
        scene_page_from_content(&state, &content).expect("battle-loss epilogue should render");
    let result_text = body_text_for_kind(&page, "epilogue_result");
    let card_text = body_text_for_kind(&page, "epilogue_card");
    let suppressed_text = body_text_for_kind(&page, "epilogue_suppressed");

    assert!(result_text.contains("final_result_key: battle_loss"));
    assert!(card_text.contains("card_id: epilogue_boss_black_serpent_banner"));
    assert!(card_text.contains("variant: battle_loss_residue"));
    assert!(card_text.contains("card_id: epilogue_wuxia_southern_market_rumor"));
    assert!(card_text.contains("variant: unresolved_debt"));
    assert!(card_text.contains("card_id: epilogue_mumyeong_black_serpent_new_scale"));
    assert!(card_text.contains("variant: battle_loss_successor_pressure"));
    assert!(card_text.contains("card_id: epilogue_seoharin_closed_gate"));
    assert!(card_text.contains("variant: battle_loss_or_corruption"));
    assert!(card_text.contains("card_id: epilogue_tianjilu_last_page"));
    assert!(card_text.contains("variant: corruption_variant"));
    assert!(card_text.contains("final_combat_result_battle_loss_seeded"));
    assert!(!card_text.contains("card_id: epilogue_boss_broken_black_serpent"));
    assert!(!card_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(!card_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
    assert!(suppressed_text.contains("card_id: epilogue_boss_broken_black_serpent"));
    assert!(suppressed_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(suppressed_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
    assert!(suppressed_text.contains("suppressed_by: battle_loss"));
}

#[test]
fn wuxia_final_epilogue_corrupted_priority_overrides_true_route_candidates() {
    let content = wuxia_content();
    let state = wuxia_final_state_with_flags(
        &[
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded",
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_epilogue_candidates_true_route_seeded",
            "final_boss_resolution_corrupted_victory_seeded",
            "final_epilogue_candidates_corrupted_seeded",
            "final_cheongirok_state_corruption_high_confirmed_seeded",
            "final_player_method_sado_style_calculation_echo_seeded",
            "final_epilogue_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_empty_place_candidate_seeded",
            "final_epilogue_mumyeong_stolen_forms_stopped_candidate_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
            "final_epilogue_seoharin_closed_gate_candidate_seeded",
            "final_epilogue_tianjilu_last_page_corruption_variant_seeded",
        ],
        &[],
    );

    let page =
        scene_page_from_content(&state, &content).expect("final epilogue page should render");
    let result_text = body_text_for_kind(&page, "epilogue_result");
    let card_text = body_text_for_kind(&page, "epilogue_card");
    let suppressed_text = body_text_for_kind(&page, "epilogue_suppressed");

    assert!(result_text.contains("final_result_key: corrupted_victory"));
    assert!(card_text.contains("card_id: epilogue_boss_broken_black_serpent"));
    assert!(card_text.contains("card_id: epilogue_mumyeong_black_serpent_new_scale"));
    assert!(card_text.contains("card_id: epilogue_seoharin_closed_gate"));
    assert!(card_text.contains("variant: corruption_variant"));
    assert!(!card_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(!card_text.contains("card_id: epilogue_seoharin_empty_place"));
    assert!(!card_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
    assert!(suppressed_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(suppressed_text.contains("card_id: epilogue_seoharin_empty_place"));
    assert!(suppressed_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
    assert!(suppressed_text.contains("suppressed_by: corrupted_victory"));
}

#[test]
fn wuxia_final_epilogue_strong_evidence_alliance_silence_is_responsibility_evasion() {
    let content = wuxia_content();
    let state = wuxia_final_state_with_flags(
        &[
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded",
            "final_boss_resolution_meaningful_victory_seeded",
            "final_epilogue_candidates_meaningful_seeded",
            "final_evidence_strong_seeded",
            "final_alliance_silence_strong_evidence_variant_seeded",
            "final_black_serpent_aftermath_alliance_silence_seeded",
            "final_alliance_silence_responsibility_evasion_seeded",
        ],
        &["strong_evidence_turns_silence_into_responsibility_evasion"],
    );

    let page =
        scene_page_from_content(&state, &content).expect("final epilogue page should render");
    let result_text = body_text_for_kind(&page, "epilogue_result");
    let card_text = body_text_for_kind(&page, "epilogue_card");

    assert!(result_text.contains("final_result_key: meaningful_victory"));
    assert!(card_text.contains("card_id: epilogue_boss_alliance_silence"));
    assert!(card_text.contains("variant: responsibility_evasion"));
    assert!(card_text.contains("final_alliance_silence_responsibility_evasion_seeded"));
    assert!(card_text.contains("증거 부족 판정이 아니다"));
}

#[test]
fn schema_less_combat_prototype_renders_via_scene_page_and_resolves_with_existing_actions() {
    let content = content();
    let mut combat_prompt = apply_sequence(
        new_game_from_content(123, &content).expect("content-backed game should start"),
        &content,
        &[
            "choice:check_message",
            "move:dev_office",
            "move:supply_closet",
            "choice:brace_for_supply_scuffle",
        ],
    );

    let page = scene_page_from_content(&combat_prompt, &content)
        .expect("combat prototype scene page should render");
    assert_eq!(page.mode, SceneMode::Encounter);
    assert_eq!(page.title, "물품창고 자동 난투");
    assert_eq!(page.visual.id, "supply_closet_scuffle");
    assert_eq!(page.visual.kind, "combat_intervention");
    assert_eq!(
        page.actions
            .iter()
            .map(|action| action.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "choice:keep_distance_between_shelves",
            "choice:hook_cart_to_cabinet",
            "choice:pull_extinguisher_pin",
        ]
    );
    assert!(page
        .effect_cues
        .iter()
        .any(|cue| cue.kind == "glyph_anomaly" && cue.source == "shelf_impact"));

    combat_prompt =
        apply_action_from_content(&combat_prompt, &content, "choice:hook_cart_to_cabinet")
            .expect("combat intervention should resolve")
            .state;
    assert!(combat_prompt
        .flags
        .contains(&"combat_intervention_success".to_string()));
    assert!(combat_prompt
        .clues
        .contains(&"improvised_distance_control".to_string()));
    assert!(
        !action_ids(&combat_prompt, &content).contains(&"choice:hook_cart_to_cabinet".to_string())
    );
}

#[test]
fn scripted_major_route_smokes_reach_expected_endings_through_current_actions() {
    let content = content();

    let mut escape = new_game_from_content_at(123, &content, "emergency_stairs")
        .expect("content-backed game should start at stairs");
    escape = apply_action_from_content(&escape, &content, "choice:align_breathing_floor")
        .expect("escape setup should resolve")
        .state;
    assert!(action_ids(&escape, &content).contains(&"choice:solve_distorted_floor".to_string()));
    escape = apply_action_from_content(&escape, &content, "choice:solve_distorted_floor")
        .expect("escape final should resolve")
        .state;
    assert_eq!(
        scene_page_from_content(&escape, &content)
            .expect("escape ending should render")
            .title,
        "퇴근 성공"
    );

    let failure = apply_sequence(
        new_game_from_content_at(123, &content, "emergency_stairs")
            .expect("content-backed game should start at stairs"),
        &content,
        &[
            "choice:align_breathing_floor",
            "choice:walk_down_wrong_stairs",
        ],
    );
    assert_eq!(
        scene_page_from_content(&failure, &content)
            .expect("failure ending should render")
            .title,
        "게임오버: 계단이 접혔다"
    );

    let mut conquest = new_game_from_content_at(123, &content, "server_room")
        .expect("content-backed game should start at server room");
    conquest = apply_action_from_content(&conquest, &content, "choice:assume_admin_console")
        .expect("conquest action should resolve")
        .state;
    assert_eq!(
        scene_page_from_content(&conquest, &content)
            .expect("conquest ending should render")
            .title,
        "사내망 관리자 권한"
    );

    let truth = apply_sequence(
        new_game_from_content(123, &content).expect("content-backed game should start"),
        &content,
        &[
            "choice:search_ex_employee",
            "move:dev_office",
            "move:meeting_room",
            "choice:save_impossible_minutes",
            "move:dev_office",
            "move:hallway",
            "move:security_room",
            "choice:replay_delayed_cctv",
        ],
    );
    assert_eq!(
        scene_page_from_content(&truth, &content)
            .expect("truth ending should render")
            .title,
        "격리 프로토콜의 진실"
    );

    let hidden = apply_sequence(
        new_game_from_content(123, &content).expect("content-backed game should start"),
        &content,
        &[
            "choice:ignore_phone",
            "move:dev_office",
            "move:printer_area",
            "choice:take_printout",
            "move:pantry",
            "choice:look_behind_machine",
        ],
    );
    let hidden_page =
        scene_page_from_content(&hidden, &content).expect("hidden ending should render");
    assert_eq!(hidden_page.title, "첫 번째 현실 연결 힌트");
    assert!(hidden_page
        .body_blocks
        .iter()
        .any(|block| block.kind == "clue" && block.text.contains("private_only")));
}
