use escape_wasm::{
    apply_action_json, load_state_json, new_game_json, save_state_json, scene_page_json,
};
use serde_json::Value;

const CONTENT_BUNDLE: &str = include_str!("../../escape-core/fixtures/content/content.bundle.json");

const WUXIA_PREVIEW_BUNDLE: &str = include_str!(
    "../../escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json"
);

fn wuxia_state_after_actions(actions: &[&str]) -> String {
    let mut state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    for action in actions {
        let result_json = apply_action_json(&state_json, WUXIA_PREVIEW_BUNDLE, action)
            .unwrap_or_else(|err| panic!("preview action {action} should serialize: {err}"));
        let result: Value =
            serde_json::from_str(&result_json).expect("preview action result should parse");
        state_json = serde_json::to_string(&result["state"]).expect("state should stringify");
    }
    state_json
}

#[test]
fn json_boundary_creates_scene_page_applies_action_and_roundtrips_save() {
    let state_json = new_game_json(123, CONTENT_BUNDLE).expect("new game should serialize");
    let state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    assert_eq!(state["seed"], 123);
    assert_eq!(state["location_id"], "dev_desk");
    assert_eq!(state["turn"], 0);
    assert_eq!(state["player"]["hunger"], 0);
    assert_eq!(state["player"]["thirst"], 0);

    let page_json =
        scene_page_json(&state_json, CONTENT_BUNDLE).expect("scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "퇴사자의 메신저");
    assert_eq!(page["actions"][0]["id"], "choice:check_message");
    assert_eq!(page["status_summary"]["resources"][3]["id"], "hunger");
    assert_eq!(page["status_summary"]["resources"][4]["id"], "thirst");

    let result_json = apply_action_json(&state_json, CONTENT_BUNDLE, "choice:check_message")
        .expect("action result should serialize");
    let result: Value =
        serde_json::from_str(&result_json).expect("action result JSON should parse");
    assert_eq!(result["encounter_id"], "ex_employee_messenger");
    assert_eq!(result["action_id"], "choice:check_message");
    assert_eq!(result["state"]["turn"], 1);
    assert_eq!(result["state"]["player"]["battery"], 97);
    assert_eq!(result["state"]["player"]["hunger"], 1);
    assert_eq!(result["state"]["player"]["thirst"], 2);
    assert_eq!(result["logs"][0], "퇴사자의 메시지를 확인했다.");
    assert_eq!(
        result["newly_unlocked_achievements"][0],
        "first_signal_received"
    );

    let next_state_json = serde_json::to_string(&result["state"]).expect("state should stringify");
    let next_page_json = scene_page_json(&next_state_json, CONTENT_BUNDLE)
        .expect("next scene page should serialize");
    let next_page: Value =
        serde_json::from_str(&next_page_json).expect("next page JSON should parse");
    assert_eq!(next_page["mode"], "movement");
    assert_eq!(
        next_page["history_entries"][0]["text"],
        "퇴사자의 메시지를 확인했다."
    );
    assert_eq!(
        next_page["achievement_summary"]["unlocked"][0],
        "first_signal_received"
    );

    let save_json = save_state_json(&next_state_json).expect("save envelope should serialize");
    let save: Value = serde_json::from_str(&save_json).expect("save envelope JSON should parse");
    assert_eq!(save["schema_version"], 1);
    assert_eq!(save["state"]["turn"], 1);

    let restored_json = load_state_json(&save_json).expect("save envelope should load");
    let restored: Value =
        serde_json::from_str(&restored_json).expect("restored state should parse");
    assert_eq!(restored, result["state"]);
}

#[test]
fn json_boundary_uses_storypack_preview_default_location() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    assert_eq!(state["location_id"], "wuxia_commute_rift");

    let page_json = scene_page_json(&state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("preview scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "출근길 균열");
    assert_eq!(page["location"]["id"], "wuxia_commute_rift");
    assert_eq!(page["visual"]["kind"], "storypack_preview");
    assert_eq!(page["actions"][0]["id"], "choice:grip_employee_badge");

    let result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:grip_employee_badge",
    )
    .expect("preview action result should serialize");
    let result: Value =
        serde_json::from_str(&result_json).expect("action result JSON should parse");
    assert_eq!(result["encounter_id"], "wuxia_commute_rift_arrival");
    assert_eq!(result["state"]["flags"][0], "wuxia_arrival_grounded");
    assert_eq!(
        result["newly_unlocked_achievements"][0],
        "wuxia_first_arrival"
    );
}

#[test]
fn json_boundary_reaches_wuxia_first_fight_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    assert_eq!(arrival_result["state"]["location_id"], "jianghu_roadside");
    assert_eq!(arrival_result["state"]["flags"][0], "wuxia_arrival_hidden");

    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");
    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    assert_eq!(move_result["state"]["location_id"], "jianghu_market_street");

    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");
    let page_json = scene_page_json(&market_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("first fight scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "흑사방 첫 난투");
    assert_eq!(page["location"]["id"], "jianghu_market_street");
    assert_eq!(page["visual"]["id"], "wuxia_heuksa_bang_first_fight");
    assert_eq!(page["visual"]["kind"], "combat_intervention");
    assert_eq!(page["effect_cues"][0]["stable_terms"][0], "거리");
    let action_ids: Vec<&str> = page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        action_ids,
        vec![
            "choice:run_toward_open_street",
            "choice:deescalate_with_words",
            "choice:swing_commute_bag",
            "choice:loosen_tie_and_drop_shoes",
            "choice:crash_in_with_body",
        ]
    );

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    assert_eq!(
        fight_result["encounter_id"],
        "wuxia_heuksa_bang_first_fight"
    );
    assert_eq!(fight_result["state"]["player"]["health"], 97);
    assert!(fight_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "heuksa_bang_first_fight_resolved"));

    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");
    let fragment_page_json = scene_page_json(&post_fight_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("first fragment scene page should serialize");
    let fragment_page: Value =
        serde_json::from_str(&fragment_page_json).expect("fragment page JSON should parse");
    assert_eq!(fragment_page["mode"], "encounter");
    assert_eq!(fragment_page["title"], "천기록 첫 편린");
    assert_eq!(
        fragment_page["visual"]["id"],
        "wuxia_cheonggi_record_first_fragment"
    );
    assert_eq!(fragment_page["visual"]["kind"], "cheonggi_record");
    let fragment_action_ids: Vec<&str> = fragment_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        fragment_action_ids,
        vec![
            "choice:choose_guard_basics",
            "choice:choose_keep_feet_moving",
            "choice:choose_failure_log",
            "choice:close_notebook_without_choice",
        ]
    );

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    assert_eq!(
        fragment_result["encounter_id"],
        "wuxia_cheonggi_record_first_fragment"
    );
    assert_eq!(
        fragment_result["state"]["inventory"][0],
        "cheonggi_record_notebook"
    );
    assert!(fragment_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheonggi_fragment_failure_log_thread"));
    assert_eq!(
        fragment_result["newly_unlocked_achievements"][0],
        "wuxia_first_fragment_seen"
    );
}

#[test]
fn json_boundary_reaches_wuxia_cheongryu_apprentice_entry_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_page_json = scene_page_json(&post_fragment_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("seo harin rescue scene page should serialize");
    let rescue_page: Value =
        serde_json::from_str(&rescue_page_json).expect("rescue page JSON should parse");
    assert_eq!(rescue_page["mode"], "encounter");
    assert_eq!(rescue_page["title"], "서하린의 개입");
    assert_eq!(rescue_page["location"]["id"], "jianghu_market_street");
    assert_eq!(rescue_page["visual"]["id"], "wuxia_seo_harin_rescue");
    assert_eq!(rescue_page["visual"]["kind"], "rescue_and_investigation");
    assert_eq!(rescue_page["effect_cues"][0]["stable_terms"][0], "서하린");
    let rescue_action_ids: Vec<&str> = rescue_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        rescue_action_ids,
        vec![
            "choice:tell_plain_truth",
            "choice:ask_for_medical_help_first",
            "choice:explain_company_and_commute",
            "choice:show_cheonggi_record_page",
            "choice:hide_employee_badge",
        ]
    );

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    assert_eq!(rescue_result["encounter_id"], "wuxia_seo_harin_rescue");
    assert_eq!(
        rescue_result["state"]["location_id"],
        "cheongryu_outer_courtyard"
    );
    assert!(rescue_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "seo_harin_rescue_resolved"));
    assert!(rescue_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "taken_under_watch"));

    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");
    let apprentice_page_json = scene_page_json(&post_rescue_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("cheongryu apprentice scene page should serialize");
    let apprentice_page: Value =
        serde_json::from_str(&apprentice_page_json).expect("apprentice page JSON should parse");
    assert_eq!(apprentice_page["mode"], "encounter");
    assert_eq!(apprentice_page["title"], "청류문 임시 수습생 등록");
    assert_eq!(
        apprentice_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        apprentice_page["visual"]["id"],
        "wuxia_cheongryu_apprentice_entry"
    );
    assert_eq!(
        apprentice_page["visual"]["kind"],
        "cheongryu_apprenticeship"
    );
    let apprentice_action_ids: Vec<&str> = apprentice_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        apprentice_action_ids,
        vec![
            "choice:accept_three_month_trial",
            "choice:request_martial_training_immediately",
            "choice:organize_chores_like_workflow",
            "choice:inspect_archive_during_chore",
        ]
    );
    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    assert_eq!(
        apprentice_result["encounter_id"],
        "wuxia_cheongryu_apprentice_entry"
    );
    assert!(apprentice_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheongryu_apprentice_entry_resolved"));
    assert!(apprentice_result["state"]["inventory"]
        .as_array()
        .expect("inventory should be an array")
        .iter()
        .any(|item| item == "work_chore_token"));
}

#[test]
fn json_boundary_reaches_wuxia_cheongryu_raid_route_split_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_page_json = scene_page_json(&post_apprentice_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("chore sparring scene page should serialize");
    let sparring_page: Value =
        serde_json::from_str(&sparring_page_json).expect("sparring page JSON should parse");
    assert_eq!(sparring_page["mode"], "encounter");
    assert_eq!(sparring_page["title"], "청류문 장작 마당 첫 겨루기");
    assert_eq!(sparring_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        sparring_page["visual"]["id"],
        "wuxia_cheongryu_chore_sparring"
    );
    assert_eq!(sparring_page["visual"]["kind"], "combat_intervention");
    assert_eq!(sparring_page["effect_cues"][0]["stable_terms"][0], "균형");
    let sparring_action_ids: Vec<&str> = sparring_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        sparring_action_ids,
        vec![
            "choice:step_back_with_firewood",
            "choice:let_shoulder_turn_with_push",
            "choice:plant_bare_foot_in_dust",
            "choice:ask_harin_what_changed",
        ]
    );

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    assert_eq!(
        sparring_result["encounter_id"],
        "wuxia_cheongryu_chore_sparring"
    );
    assert!(sparring_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheongryu_chore_sparring_resolved"));
    assert!(sparring_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "balance_matters_more_than_force"));
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_page_json = scene_page_json(&post_sparring_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("raid route split scene page should serialize");
    let raid_page: Value =
        serde_json::from_str(&raid_page_json).expect("raid page JSON should parse");
    assert_eq!(raid_page["mode"], "encounter");
    assert_eq!(raid_page["title"], "청류문 습격과 갈라지는 길");
    assert_eq!(raid_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        raid_page["visual"]["id"],
        "wuxia_cheongryu_raid_route_split"
    );
    assert_eq!(raid_page["visual"]["kind"], "raid_route_pressure");
    assert_eq!(raid_page["effect_cues"][0]["stable_terms"][0], "청류문");
    let raid_action_ids: Vec<&str> = raid_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        raid_action_ids,
        vec![
            "choice:evacuate_the_wounded_first",
            "choice:defend_cheongryu_with_white_path",
            "choice:trade_with_black_heaven",
            "choice:follow_heavenly_archive",
        ]
    );

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:evacuate_the_wounded_first",
    )
    .expect("evacuate wounded raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    assert_eq!(
        raid_result["encounter_id"],
        "wuxia_cheongryu_raid_route_split"
    );
    assert!(raid_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheongryu_raid_route_split_resolved"));
    assert!(raid_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "route_commitment_deferred"));
    assert!(raid_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "saving_people_delays_route_choice"));
}

#[test]
fn json_boundary_reaches_wuxia_cheongryu_raid_wounded_fallback_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:evacuate_the_wounded_first",
    )
    .expect("evacuate wounded raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let wounded_page_json = scene_page_json(&post_raid_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("wounded fallback scene page should serialize");
    let wounded_page: Value =
        serde_json::from_str(&wounded_page_json).expect("wounded page JSON should parse");
    assert_eq!(wounded_page["mode"], "encounter");
    assert_eq!(wounded_page["title"], "부상자 피난처와 미뤄진 선택");
    assert_eq!(wounded_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        wounded_page["visual"]["id"],
        "wuxia_cheongryu_raid_wounded_fallback"
    );
    assert_eq!(
        wounded_page["visual"]["kind"],
        "wounded_fallback_route_pressure"
    );
    assert_eq!(wounded_page["effect_cues"][0]["stable_terms"][0], "부상자");
    let wounded_action_ids: Vec<&str> = wounded_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        wounded_action_ids,
        vec![
            "choice:stabilize_wounded_until_dawn",
            "choice:ask_baekdo_for_medicine_not_command",
            "choice:trade_black_heaven_bandages_for_exit",
            "choice:follow_archive_triage_map",
        ]
    );

    let wounded_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:stabilize_wounded_until_dawn",
    )
    .expect("stabilize wounded fallback action should serialize");
    let wounded_result: Value = serde_json::from_str(&wounded_result_json)
        .expect("wounded fallback action result should parse");
    assert_eq!(
        wounded_result["encounter_id"],
        "wuxia_cheongryu_raid_wounded_fallback"
    );
    assert!(wounded_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheongryu_raid_wounded_fallback_resolved"));
    assert!(wounded_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "deferred_route_reopened"));
    assert!(wounded_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "deferred_choice_is_still_choice"));
}

#[test]
fn json_boundary_reaches_wuxia_wounded_shelter_dawn_offers_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:evacuate_the_wounded_first",
    )
    .expect("evacuate wounded raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let wounded_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:stabilize_wounded_until_dawn",
    )
    .expect("stabilize wounded fallback action should serialize");
    let wounded_result: Value = serde_json::from_str(&wounded_result_json)
        .expect("wounded fallback action result should parse");
    let post_wounded_state_json =
        serde_json::to_string(&wounded_result["state"]).expect("state should stringify");

    let offers_page_json = scene_page_json(&post_wounded_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("wounded shelter dawn offers scene page should serialize");
    let offers_page: Value =
        serde_json::from_str(&offers_page_json).expect("offers page JSON should parse");
    assert_eq!(offers_page["mode"], "encounter");
    assert_eq!(offers_page["title"], "부상자 피난처의 새벽 제안");
    assert_eq!(offers_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        offers_page["visual"]["id"],
        "wuxia_wounded_shelter_dawn_offers"
    );
    assert_eq!(offers_page["visual"]["kind"], "deferred_route_offer");
    assert_eq!(offers_page["effect_cues"][0]["stable_terms"][0], "새벽");
    let offer_action_ids: Vec<&str> = offers_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        offer_action_ids,
        vec![
            "choice:keep_wounded_shelter_until_noon",
            "choice:accept_baekdo_medicine_after_roll_call",
            "choice:send_word_to_dowol_for_quiet_exit",
            "choice:show_archive_map_to_yeon_soha",
        ]
    );

    let offers_result_json = apply_action_json(
        &post_wounded_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_baekdo_medicine_after_roll_call",
    )
    .expect("accept baekdo medicine after roll call action should serialize");
    let offers_result: Value =
        serde_json::from_str(&offers_result_json).expect("offers action result should parse");
    assert_eq!(
        offers_result["encounter_id"],
        "wuxia_wounded_shelter_dawn_offers"
    );
    assert!(offers_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "wounded_shelter_dawn_offers_resolved"));
    assert!(offers_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "righteous_route_started"));
    assert!(offers_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "offers_arrive_because_people_lived"));
}

#[test]
fn json_boundary_reaches_wuxia_baekdo_medicine_debt_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:defend_cheongryu_with_white_path",
    )
    .expect("white path raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let baekdo_page_json = scene_page_json(&post_raid_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("baekdo medicine debt scene page should serialize");
    let baekdo_page: Value =
        serde_json::from_str(&baekdo_page_json).expect("baekdo page JSON should parse");
    assert_eq!(baekdo_page["mode"], "encounter");
    assert_eq!(baekdo_page["title"], "백도맹 약상자와 청류문의 채무");
    assert_eq!(baekdo_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(baekdo_page["visual"]["id"], "wuxia_baekdo_medicine_debt");
    assert_eq!(baekdo_page["visual"]["kind"], "righteous_route_opener");
    assert_eq!(baekdo_page["effect_cues"][0]["stable_terms"][0], "약상자");
    let baekdo_action_ids: Vec<&str> = baekdo_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        baekdo_action_ids,
        vec![
            "choice:accept_medicine_with_written_debt",
            "choice:ask_terms_before_opening_gate",
            "choice:send_supplies_to_wounded_first",
            "choice:compare_banner_to_record_margin",
        ]
    );

    let baekdo_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_medicine_with_written_debt",
    )
    .expect("accept medicine with written debt action should serialize");
    let baekdo_result: Value =
        serde_json::from_str(&baekdo_result_json).expect("baekdo action result should parse");
    assert_eq!(baekdo_result["encounter_id"], "wuxia_baekdo_medicine_debt");
    assert!(baekdo_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "baekdo_medicine_debt_resolved"));
    assert!(baekdo_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "righteous_route_opened"));
    assert!(baekdo_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "route_opener_resolved"));
    assert!(baekdo_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "qingliu_survival_needs_outside_help"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_first_sighting_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:defend_cheongryu_with_white_path",
    )
    .expect("white path raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let baekdo_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_medicine_with_written_debt",
    )
    .expect("accept medicine with written debt action should serialize");
    let baekdo_result: Value =
        serde_json::from_str(&baekdo_result_json).expect("baekdo action result should parse");
    let post_baekdo_state_json =
        serde_json::to_string(&baekdo_result["state"]).expect("state should stringify");

    let sighting_page_json = scene_page_json(&post_baekdo_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("mumyeong first sighting scene page should serialize");
    let sighting_page: Value =
        serde_json::from_str(&sighting_page_json).expect("sighting page JSON should parse");
    assert_eq!(sighting_page["mode"], "encounter");
    assert_eq!(sighting_page["title"], "무명 첫 목격");
    assert_eq!(sighting_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        sighting_page["visual"]["id"],
        "wuxia_mumyeong_first_sighting"
    );
    assert_eq!(sighting_page["visual"]["kind"], "midgame_rival_sighting");
    assert_eq!(sighting_page["effect_cues"][0]["stable_terms"][0], "무명");
    let sighting_action_ids: Vec<&str> = sighting_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        sighting_action_ids,
        vec![
            "choice:watch_the_stolen_qingliu_flow",
            "choice:check_seo_harin_silence",
            "choice:follow_black_serpent_runner",
            "choice:pretend_not_to_see_the_form",
        ]
    );

    let sighting_result_json = apply_action_json(
        &post_baekdo_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:watch_the_stolen_qingliu_flow",
    )
    .expect("watch stolen qingliu flow action should serialize");
    let sighting_result: Value =
        serde_json::from_str(&sighting_result_json).expect("sighting action result should parse");
    assert_eq!(
        sighting_result["encounter_id"],
        "wuxia_mumyeong_first_sighting"
    );
    assert!(sighting_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_first_sighting_resolved"));
    assert!(sighting_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "midgame_continuity_started"));
    assert!(sighting_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "mumyeong_exists"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_first_confrontation_through_preview_bundle() {
    let post_sighting_state_json = wuxia_state_after_actions(&[
        "choice:follow_roadside_dust",
        "move:jianghu_market_street",
        "choice:run_toward_open_street",
        "choice:choose_failure_log",
        "choice:tell_plain_truth",
        "choice:accept_three_month_trial",
        "choice:step_back_with_firewood",
        "choice:defend_cheongryu_with_white_path",
        "choice:accept_medicine_with_written_debt",
        "choice:watch_the_stolen_qingliu_flow",
    ]);

    let confrontation_page_json = scene_page_json(&post_sighting_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("mumyeong first confrontation scene page should serialize");
    let confrontation_page: Value = serde_json::from_str(&confrontation_page_json)
        .expect("confrontation page JSON should parse");
    assert_eq!(confrontation_page["mode"], "encounter");
    assert_eq!(confrontation_page["title"], "무명 첫 대치");
    assert_eq!(
        confrontation_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        confrontation_page["visual"]["id"],
        "wuxia_mumyeong_first_confrontation"
    );
    assert_eq!(
        confrontation_page["visual"]["kind"],
        "rival_first_confrontation"
    );
    assert_eq!(
        confrontation_page["effect_cues"][0]["stable_terms"][0],
        "무명"
    );
    let confrontation_action_ids: Vec<&str> = confrontation_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        confrontation_action_ids,
        vec![
            "choice:meet_mumyeong_head_on",
            "choice:endure_until_copy_flow_breaks",
            "choice:watch_seo_harin_hold_back",
            "choice:read_mumyeongs_copied_form",
            "choice:do_not_provoke_mumyeong",
        ]
    );

    let confrontation_result_json = apply_action_json(
        &post_sighting_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:endure_until_copy_flow_breaks",
    )
    .expect("endure until copy flow breaks action should serialize");
    let confrontation_result: Value = serde_json::from_str(&confrontation_result_json)
        .expect("confrontation action result should parse");
    assert_eq!(
        confrontation_result["encounter_id"],
        "wuxia_mumyeong_first_confrontation"
    );
    assert!(confrontation_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_first_confrontation_resolved"));
    assert!(confrontation_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_rival_thread_opened"));
    assert!(confrontation_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "copy_style_has_gap"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_copy_style_reveal_through_preview_bundle() {
    let post_confrontation_state_json = wuxia_state_after_actions(&[
        "choice:follow_roadside_dust",
        "move:jianghu_market_street",
        "choice:run_toward_open_street",
        "choice:choose_failure_log",
        "choice:tell_plain_truth",
        "choice:accept_three_month_trial",
        "choice:step_back_with_firewood",
        "choice:defend_cheongryu_with_white_path",
        "choice:accept_medicine_with_written_debt",
        "choice:watch_the_stolen_qingliu_flow",
        "choice:endure_until_copy_flow_breaks",
    ]);

    let copy_style_page_json =
        scene_page_json(&post_confrontation_state_json, WUXIA_PREVIEW_BUNDLE)
            .expect("mumyeong copy style reveal scene page should serialize");
    let copy_style_page: Value =
        serde_json::from_str(&copy_style_page_json).expect("copy style page JSON should parse");
    assert_eq!(copy_style_page["mode"], "encounter");
    assert_eq!(copy_style_page["title"], "무명의 카피 무공 공개");
    assert_eq!(
        copy_style_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        copy_style_page["visual"]["id"],
        "wuxia_mumyeong_copy_style_reveal"
    );
    assert_eq!(copy_style_page["visual"]["kind"], "copy_style_analysis");
    assert_eq!(
        copy_style_page["effect_cues"][0]["stable_terms"][1],
        "청류안"
    );
    let copy_style_action_ids: Vec<&str> = copy_style_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        copy_style_action_ids,
        vec![
            "choice:read_the_stolen_blade_path",
            "choice:watch_mumyeongs_footwork",
            "choice:listen_for_breath_mismatch",
            "choice:wait_for_body_to_shudder",
        ]
    );

    let copy_style_result_json = apply_action_json(
        &post_confrontation_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:listen_for_breath_mismatch",
    )
    .expect("listen for breath mismatch action should serialize");
    let copy_style_result: Value =
        serde_json::from_str(&copy_style_result_json).expect("copy style action should parse");
    assert_eq!(
        copy_style_result["encounter_id"],
        "wuxia_mumyeong_copy_style_reveal"
    );
    assert!(copy_style_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_copy_style_reveal_resolved"));
    assert!(copy_style_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "copy_style_hint_recorded"));
    assert!(copy_style_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "breath_mismatch_marks_copy"));
    assert!(copy_style_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "understanding_is_not_copying"));
}

#[test]
fn json_boundary_reaches_wuxia_black_heaven_escape_price_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:trade_with_black_heaven",
    )
    .expect("black heaven raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let black_heaven_page_json = scene_page_json(&post_raid_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("black heaven escape price scene page should serialize");
    let black_heaven_page: Value =
        serde_json::from_str(&black_heaven_page_json).expect("black heaven page JSON should parse");
    assert_eq!(black_heaven_page["mode"], "encounter");
    assert_eq!(black_heaven_page["title"], "흑천련 탈출로의 값");
    assert_eq!(
        black_heaven_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        black_heaven_page["visual"]["id"],
        "wuxia_black_heaven_escape_price"
    );
    assert_eq!(black_heaven_page["visual"]["kind"], "sapa_route_opener");
    assert_eq!(
        black_heaven_page["effect_cues"][0]["stable_terms"][0],
        "탈출로"
    );
    let black_heaven_action_ids: Vec<&str> = black_heaven_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        black_heaven_action_ids,
        vec![
            "choice:accept_dowol_marker_for_safehouse",
            "choice:ask_who_collects_the_price",
            "choice:keep_cheongryu_names_off_ledger",
            "choice:map_exit_before_following_dowol",
        ]
    );

    let black_heaven_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_dowol_marker_for_safehouse",
    )
    .expect("accept dowol marker action should serialize");
    let black_heaven_result: Value = serde_json::from_str(&black_heaven_result_json)
        .expect("black heaven action result should parse");
    assert_eq!(
        black_heaven_result["encounter_id"],
        "wuxia_black_heaven_escape_price"
    );
    assert!(black_heaven_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "black_heaven_escape_price_resolved"));
    assert!(black_heaven_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "sapa_route_opened"));
    assert!(black_heaven_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "route_opener_resolved"));
    assert!(black_heaven_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "survival_bargain_is_not_loyalty"));
}

#[test]
fn json_boundary_reaches_wuxia_heavenly_archive_previous_outsiders_through_preview_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let arrival_result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_roadside_dust",
    )
    .expect("arrival route action should serialize");
    let arrival_result: Value =
        serde_json::from_str(&arrival_result_json).expect("arrival action result should parse");
    let roadside_state_json =
        serde_json::to_string(&arrival_result["state"]).expect("state should stringify");

    let move_result_json = apply_action_json(
        &roadside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "move:jianghu_market_street",
    )
    .expect("market movement should serialize");
    let move_result: Value =
        serde_json::from_str(&move_result_json).expect("movement action result should parse");
    let market_state_json =
        serde_json::to_string(&move_result["state"]).expect("state should stringify");

    let fight_result_json = apply_action_json(
        &market_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:run_toward_open_street",
    )
    .expect("fallback fight action should serialize");
    let fight_result: Value =
        serde_json::from_str(&fight_result_json).expect("fight action result should parse");
    let post_fight_state_json =
        serde_json::to_string(&fight_result["state"]).expect("state should stringify");

    let fragment_result_json = apply_action_json(
        &post_fight_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:choose_failure_log",
    )
    .expect("first fragment action should serialize");
    let fragment_result: Value = serde_json::from_str(&fragment_result_json)
        .expect("fragment action result JSON should parse");
    let post_fragment_state_json =
        serde_json::to_string(&fragment_result["state"]).expect("state should stringify");

    let rescue_result_json = apply_action_json(
        &post_fragment_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:tell_plain_truth",
    )
    .expect("plain truth rescue action should serialize");
    let rescue_result: Value =
        serde_json::from_str(&rescue_result_json).expect("rescue action result should parse");
    let post_rescue_state_json =
        serde_json::to_string(&rescue_result["state"]).expect("state should stringify");

    let apprentice_result_json = apply_action_json(
        &post_rescue_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:accept_three_month_trial",
    )
    .expect("three month trial action should serialize");
    let apprentice_result: Value = serde_json::from_str(&apprentice_result_json)
        .expect("apprentice action result should parse");
    let post_apprentice_state_json =
        serde_json::to_string(&apprentice_result["state"]).expect("state should stringify");

    let sparring_result_json = apply_action_json(
        &post_apprentice_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:step_back_with_firewood",
    )
    .expect("step back sparring action should serialize");
    let sparring_result: Value =
        serde_json::from_str(&sparring_result_json).expect("sparring action result should parse");
    let post_sparring_state_json =
        serde_json::to_string(&sparring_result["state"]).expect("state should stringify");

    let raid_result_json = apply_action_json(
        &post_sparring_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:follow_heavenly_archive",
    )
    .expect("heavenly archive raid action should serialize");
    let raid_result: Value =
        serde_json::from_str(&raid_result_json).expect("raid action result should parse");
    let post_raid_state_json =
        serde_json::to_string(&raid_result["state"]).expect("state should stringify");

    let archive_page_json = scene_page_json(&post_raid_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("heavenly archive previous outsiders scene page should serialize");
    let archive_page: Value =
        serde_json::from_str(&archive_page_json).expect("archive page JSON should parse");
    assert_eq!(archive_page["mode"], "encounter");
    assert_eq!(archive_page["title"], "천기각 이전 이방인 기록");
    assert_eq!(archive_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        archive_page["visual"]["id"],
        "wuxia_heavenly_archive_previous_outsiders"
    );
    assert_eq!(archive_page["visual"]["kind"], "cheonggi_return_opener");
    assert_eq!(archive_page["effect_cues"][0]["stable_terms"][0], "천기각");
    let archive_action_ids: Vec<&str> = archive_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        archive_action_ids,
        vec![
            "choice:read_previous_outsider_margins",
            "choice:ask_yeon_soha_what_not_to_read",
            "choice:mark_current_worldline_without_answer",
            "choice:compare_rift_terms_to_commute_memory",
        ]
    );

    let archive_result_json = apply_action_json(
        &post_raid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:read_previous_outsider_margins",
    )
    .expect("read previous outsider margins action should serialize");
    let archive_result: Value =
        serde_json::from_str(&archive_result_json).expect("archive action result should parse");
    assert_eq!(
        archive_result["encounter_id"],
        "wuxia_heavenly_archive_previous_outsiders"
    );
    assert!(archive_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "heavenly_archive_previous_outsiders_resolved"));
    assert!(archive_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "cheonggi_return_route_opened"));
    assert!(archive_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "route_opener_resolved"));
    assert!(archive_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "return_clue_is_not_return_method"));
}

#[test]
fn json_boundary_reports_user_facing_errors() {
    let state_json = new_game_json(123, CONTENT_BUNDLE).expect("new game should serialize");

    let malformed_state_error = scene_page_json("{not json", CONTENT_BUNDLE)
        .expect_err("malformed state should be a normal error");
    assert!(malformed_state_error.contains("invalid state JSON"));

    let unknown_action_error = apply_action_json(&state_json, CONTENT_BUNDLE, "choice:missing")
        .expect_err("unknown action should be a normal error");
    assert!(unknown_action_error.contains("unknown action id: choice:missing"));

    let malformed_save_error =
        load_state_json("[]").expect_err("malformed save should be rejected");
    assert!(malformed_save_error.contains("invalid save JSON"));
}
