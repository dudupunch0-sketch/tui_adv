use escape_wasm::{
    apply_action_json, load_state_json, new_game_json, save_state_json, scene_page_json,
};
use serde_json::{json, Value};

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
fn json_boundary_reaches_wuxia_mumyeong_reads_orthodox_style_through_preview_bundle() {
    let post_copy_style_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
    ]);

    let orthodox_page_json = scene_page_json(&post_copy_style_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("mumyeong orthodox style trace scene page should serialize");
    let orthodox_page: Value =
        serde_json::from_str(&orthodox_page_json).expect("orthodox style page JSON should parse");
    assert_eq!(orthodox_page["mode"], "encounter");
    assert_eq!(orthodox_page["title"], "무명의 정파 무공 간파");
    assert_eq!(orthodox_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        orthodox_page["visual"]["id"],
        "wuxia_mumyeong_reads_orthodox_style"
    );
    assert_eq!(orthodox_page["visual"]["kind"], "orthodox_style_trace");
    assert_eq!(
        orthodox_page["effect_cues"][0]["stable_terms"][1],
        "복호금쇄수"
    );
    let orthodox_action_ids: Vec<&str> = orthodox_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        orthodox_action_ids,
        vec![
            "choice:compare_copied_form_to_old_wound",
            "choice:trace_qingliu_eye_variation",
            "choice:reconstruct_mumyeongs_sightline",
            "choice:stop_before_truth_becomes_accusation",
        ]
    );

    let orthodox_result_json = apply_action_json(
        &post_copy_style_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:reconstruct_mumyeongs_sightline",
    )
    .expect("reconstruct mumyeong sightline action should serialize");
    let orthodox_result: Value =
        serde_json::from_str(&orthodox_result_json).expect("orthodox action should parse");
    assert_eq!(
        orthodox_result["encounter_id"],
        "wuxia_mumyeong_reads_orthodox_style"
    );
    assert!(orthodox_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_reads_orthodox_style_resolved"));
    assert!(orthodox_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "orthodox_style_trace_recorded"));
    assert!(orthodox_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "bokho_geumsaesu_name_recorded"));
    assert!(orthodox_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "departure_truth_still_incomplete"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_midgame_reunion_through_preview_bundle() {
    let post_orthodox_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
    ]);

    let reunion_page_json = scene_page_json(&post_orthodox_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("mumyeong midgame reunion scene page should serialize");
    let reunion_page: Value =
        serde_json::from_str(&reunion_page_json).expect("reunion page JSON should parse");
    assert_eq!(reunion_page["mode"], "encounter");
    assert_eq!(reunion_page["title"], "무명 중반 재회");
    assert_eq!(reunion_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        reunion_page["visual"]["id"],
        "wuxia_mumyeong_midgame_reunion"
    );
    assert_eq!(reunion_page["visual"]["kind"], "rival_reunion_trace");
    assert_eq!(reunion_page["effect_cues"][0]["stable_terms"][2], "현악문");
    let reunion_action_ids: Vec<&str> = reunion_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        reunion_action_ids,
        vec![
            "choice:ask_why_seoharin_never_called_him_traitor",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "choice:point_out_the_copied_form_gap",
            "choice:keep_blades_low_and_watch_his_answer",
        ]
    );

    let reunion_result_json = apply_action_json(
        &post_orthodox_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:show_the_hyeonakmun_trace_without_accusing",
    )
    .expect("show Hyeonakmun trace action should serialize");
    let reunion_result: Value =
        serde_json::from_str(&reunion_result_json).expect("reunion action should parse");
    assert_eq!(
        reunion_result["encounter_id"],
        "wuxia_mumyeong_midgame_reunion"
    );
    assert!(reunion_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_midgame_reunion_resolved"));
    assert!(reunion_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_mirror_thread_deepened"));
    assert!(reunion_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "hyeonakmun_trace_shared_without_accusation"));
    assert!(reunion_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "boss_used_mumyeongs_wound"));
}

#[test]
fn json_boundary_reaches_wuxia_boss_first_appearance_through_preview_bundle() {
    let post_reunion_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
    ]);

    let boss_page_json = scene_page_json(&post_reunion_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("boss first appearance scene page should serialize");
    let boss_page: Value =
        serde_json::from_str(&boss_page_json).expect("boss page JSON should parse");
    assert_eq!(boss_page["mode"], "encounter");
    assert_eq!(boss_page["title"], "보스 첫 등장");
    assert_eq!(boss_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(boss_page["visual"]["id"], "wuxia_boss_first_appearance");
    assert_eq!(boss_page["visual"]["kind"], "boss_wall_pressure");
    assert_eq!(boss_page["effect_cues"][0]["stable_terms"][2], "청류문");
    let boss_action_ids: Vec<&str> = boss_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        boss_action_ids,
        vec![
            "choice:read_the_boss_flow_and_fail_to_move",
            "choice:pull_seo_harin_behind_broken_gate",
            "choice:watch_mumyeong_answer_the_boss",
            "choice:retreat_before_the_second_step",
        ]
    );

    let boss_result_json = apply_action_json(
        &post_reunion_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:watch_mumyeong_answer_the_boss",
    )
    .expect("watch Mumyeong answer boss action should serialize");
    let boss_result: Value =
        serde_json::from_str(&boss_result_json).expect("boss action should parse");
    assert_eq!(boss_result["encounter_id"], "wuxia_boss_first_appearance");
    assert!(boss_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "boss_first_appearance_resolved"));
    assert!(boss_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "boss_wall_thread_opened"));
    assert!(boss_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "black_serpent_core_pressure_opened"));
    assert!(boss_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "mumyeong_follows_power_that_saw_his_wound"));
    assert!(boss_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "boss_reads_people_not_forms"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_request_for_aid_through_preview_bundle() {
    let post_boss_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
    ]);

    let request_page_json = scene_page_json(&post_boss_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Mumyeong aid request scene page should serialize");
    let request_page: Value =
        serde_json::from_str(&request_page_json).expect("request page JSON should parse");
    assert_eq!(request_page["mode"], "encounter");
    assert_eq!(request_page["title"], "무명의 도움 요청");
    assert_eq!(request_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        request_page["visual"]["id"],
        "wuxia_mumyeong_request_for_aid"
    );
    assert_eq!(request_page["visual"]["kind"], "failed_aid_records");
    assert_eq!(request_page["effect_cues"][0]["stable_terms"][2], "정파");
    let request_action_ids: Vec<&str> = request_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        request_action_ids,
        vec![
            "choice:search_the_rejected_aid_letters",
            "choice:follow_old_inn_rumors_about_mumyeong",
            "choice:ask_seo_harin_what_help_never_came",
            "choice:keep_the_failed_aid_record_unshown",
        ]
    );

    let request_result_json = apply_action_json(
        &post_boss_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:search_the_rejected_aid_letters",
    )
    .expect("search rejected aid letters action should serialize");
    let request_result: Value =
        serde_json::from_str(&request_result_json).expect("request action should parse");
    assert_eq!(
        request_result["encounter_id"],
        "wuxia_mumyeong_request_for_aid"
    );
    assert!(request_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_request_for_aid_resolved"));
    assert!(request_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "orthodox_hypocrisy_thread_opened"));
    assert!(request_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "mumyeong_tried_to_save_qingliu"));
    assert!(request_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "orthodox_refusal_broke_mumyeong"));
    assert!(request_result["state"]["inventory"]
        .as_array()
        .expect("inventory should be an array")
        .iter()
        .any(|item| item == "rejected_aid_letter_fragment"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_awakening_through_preview_bundle() {
    let post_request_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
    ]);

    let awakening_page_json = scene_page_json(&post_request_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Mumyeong awakening scene page should serialize");
    let awakening_page: Value =
        serde_json::from_str(&awakening_page_json).expect("awakening page JSON should parse");
    assert_eq!(awakening_page["mode"], "encounter");
    assert_eq!(awakening_page["title"], "무명의 각성");
    assert_eq!(
        awakening_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(awakening_page["visual"]["id"], "wuxia_mumyeong_awakening");
    assert_eq!(awakening_page["visual"]["kind"], "anger_copy_bloom");
    assert_eq!(awakening_page["effect_cues"][0]["stable_terms"][2], "분노");
    let awakening_action_ids: Vec<&str> = awakening_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        awakening_action_ids,
        vec![
            "choice:compare_anger_to_copied_flow",
            "choice:trace_awakening_from_failed_aid",
            "choice:ask_what_the_copy_cost_him",
            "choice:stop_before_calling_it_salvation",
        ]
    );

    let awakening_result_json = apply_action_json(
        &post_request_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:compare_anger_to_copied_flow",
    )
    .expect("compare anger to copied flow action should serialize");
    let awakening_result: Value =
        serde_json::from_str(&awakening_result_json).expect("awakening action should parse");
    assert_eq!(awakening_result["encounter_id"], "wuxia_mumyeong_awakening");
    assert!(awakening_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_awakening_resolved"));
    assert!(awakening_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "copy_corruption_thread_opened"));
    assert!(awakening_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "mumyeong_copy_bloomed_from_anger"));
    assert!(awakening_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "copy_is_wound_not_growth"));
}

#[test]
fn json_boundary_reaches_wuxia_qingliu_attack_after_war_through_preview_bundle() {
    let post_awakening_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
    ]);

    let qingliu_page_json = scene_page_json(&post_awakening_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Qingliu attack trace scene page should serialize");
    let qingliu_page: Value =
        serde_json::from_str(&qingliu_page_json).expect("Qingliu page JSON should parse");
    assert_eq!(qingliu_page["mode"], "encounter");
    assert_eq!(qingliu_page["title"], "무너져가는 청류문 습격의 흔적");
    assert_eq!(qingliu_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(
        qingliu_page["visual"]["id"],
        "wuxia_qingliu_attack_after_war"
    );
    assert_eq!(qingliu_page["visual"]["kind"], "attack_trace_investigation");
    assert_eq!(
        qingliu_page["effect_cues"][0]["stable_terms"][2],
        "복호금쇄수"
    );
    let qingliu_action_ids: Vec<&str> = qingliu_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        qingliu_action_ids,
        vec![
            "choice:inspect_bokho_lock_scars",
            "choice:compare_hyeonakmun_trace_to_qingliu_wounds",
            "choice:ask_seo_harin_what_she_saw_afterward",
            "choice:stop_before_replaying_the_attack",
        ]
    );

    let qingliu_result_json = apply_action_json(
        &post_awakening_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:inspect_bokho_lock_scars",
    )
    .expect("inspect Bokho lock scars action should serialize");
    let qingliu_result: Value =
        serde_json::from_str(&qingliu_result_json).expect("Qingliu action should parse");
    assert_eq!(
        qingliu_result["encounter_id"],
        "wuxia_qingliu_attack_after_war"
    );
    assert!(qingliu_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "qingliu_attack_after_war_resolved"));
    assert!(qingliu_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "hyeonakmun_attack_thread_opened"));
    assert!(qingliu_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "bokho_geumsaesu_used_on_qingliu"));
    assert!(qingliu_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "full_flashback_still_unopened"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_destroys_orthodox_sect_through_preview_bundle() {
    let post_qingliu_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
    ]);

    let consequence_page_json = scene_page_json(&post_qingliu_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Hyeonakmun consequence trace scene page should serialize");
    let consequence_page: Value = serde_json::from_str(&consequence_page_json)
        .expect("Hyeonakmun consequence page JSON should parse");
    assert_eq!(consequence_page["mode"], "encounter");
    assert_eq!(consequence_page["title"], "비어 버린 현악문 산문");
    assert_eq!(
        consequence_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        consequence_page["visual"]["id"],
        "wuxia_mumyeong_destroys_orthodox_sect"
    );
    assert_eq!(
        consequence_page["visual"]["kind"],
        "hyeonakmun_empty_gate_record"
    );
    assert_eq!(
        consequence_page["effect_cues"][0]["stable_terms"][2],
        "무명"
    );
    let consequence_action_ids: Vec<&str> = consequence_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        consequence_action_ids,
        vec![
            "choice:read_hyeonakmun_empty_gate_record",
            "choice:trace_bokho_lock_to_mumyeong",
            "choice:ask_why_seoharin_never_heard_full_story",
            "choice:stop_before_counting_the_dead",
        ]
    );

    let consequence_result_json = apply_action_json(
        &post_qingliu_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:read_hyeonakmun_empty_gate_record",
    )
    .expect("read Hyeonakmun empty gate record action should serialize");
    let consequence_result: Value = serde_json::from_str(&consequence_result_json)
        .expect("Hyeonakmun consequence action should parse");
    assert_eq!(
        consequence_result["encounter_id"],
        "wuxia_mumyeong_destroys_orthodox_sect"
    );
    assert!(consequence_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "mumyeong_destroys_orthodox_sect_resolved"));
    assert!(consequence_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "hyeonakmun_destruction_thread_opened"));
    assert!(consequence_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "hyeonakmun_was_destroyed_after_qingliu_attack"));
    assert!(consequence_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "destruction_is_consequence_not_salvation"));
}

#[test]
fn json_boundary_reaches_wuxia_boss_recruits_mumyeong_through_preview_bundle() {
    let post_consequence_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
    ]);

    let recruit_page_json = scene_page_json(&post_consequence_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Boss recruitment trace scene page should serialize");
    let recruit_page: Value =
        serde_json::from_str(&recruit_page_json).expect("Boss recruitment page JSON should parse");
    assert_eq!(recruit_page["mode"], "encounter");
    assert_eq!(recruit_page["title"], "흑사방 보스의 스카웃 흔적");
    assert_eq!(recruit_page["location"]["id"], "cheongryu_outer_courtyard");
    assert_eq!(recruit_page["visual"]["id"], "wuxia_boss_recruits_mumyeong");
    assert_eq!(recruit_page["visual"]["kind"], "boss_recruitment_trace");
    assert_eq!(
        recruit_page["effect_cues"][0]["stable_terms"][0],
        "흑사방주"
    );
    let recruit_action_ids: Vec<&str> = recruit_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        recruit_action_ids,
        vec![
            "choice:trace_boss_offer_after_hyeonakmun",
            "choice:read_mumyeong_choice_without_excusing_it",
            "choice:search_black_serpent_recruitment_record",
            "choice:stop_before_following_him_into_black_serpent",
        ]
    );

    let recruit_result_json = apply_action_json(
        &post_consequence_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:trace_boss_offer_after_hyeonakmun",
    )
    .expect("trace boss recruitment offer action should serialize");
    let recruit_result: Value =
        serde_json::from_str(&recruit_result_json).expect("Boss recruitment action should parse");
    assert_eq!(
        recruit_result["encounter_id"],
        "wuxia_boss_recruits_mumyeong"
    );
    assert!(recruit_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "boss_recruits_mumyeong_resolved"));
    assert!(recruit_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array")
        .iter()
        .any(|flag| flag == "boss_recruitment_thread_opened"));
    assert!(recruit_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "boss_recruited_mumyeong_after_hyeonakmun"));
    assert!(recruit_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array")
        .iter()
        .any(|clue| clue == "recruitment_was_not_salvation"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_departure_truth_summary_through_preview_bundle() {
    let post_recruit_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
    ]);

    let truth_page_json = scene_page_json(&post_recruit_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Mumyeong departure truth summary scene page should serialize");
    let truth_page: Value = serde_json::from_str(&truth_page_json)
        .expect("Mumyeong departure truth summary page JSON should parse");
    assert_eq!(truth_page["mode"], "encounter");
    assert_eq!(truth_page["title"], "봉해 둔 이탈의 진실");
    assert_eq!(
        truth_page["visual"]["id"],
        "wuxia_mumyeong_departure_truth_summary"
    );
    assert_eq!(
        truth_page["visual"]["kind"],
        "sealed_departure_truth_summary"
    );
    assert_eq!(truth_page["effect_cues"][0]["stable_terms"][0], "무명");
    let truth_action_ids: Vec<&str> = truth_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        truth_action_ids,
        vec![
            "choice:assemble_departure_truth_without_delivering",
            "choice:compare_failed_aid_to_recruitment_offer",
            "choice:ask_seoharin_what_she_is_ready_to_hear",
            "choice:seal_truth_until_mumyeong_faces_it",
        ]
    );

    let truth_result_json = apply_action_json(
        &post_recruit_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:assemble_departure_truth_without_delivering",
    )
    .expect("assemble departure truth summary action should serialize");
    let truth_result: Value = serde_json::from_str(&truth_result_json)
        .expect("Mumyeong departure truth summary action should parse");
    assert_eq!(
        truth_result["encounter_id"],
        "wuxia_mumyeong_departure_truth_summary"
    );
    let flags = truth_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "mumyeong_departure_truth_summary_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "sealed_departure_truth_summary_prepared"));
    assert!(flags
        .iter()
        .any(|flag| flag == "truth_delivery_still_unopened"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = truth_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "departure_truth_can_be_understood_but_not_spoken_yet"));
    assert!(clues
        .iter()
        .any(|clue| clue == "seoharin_truth_delivery_requires_later_consent"));
}

#[test]
fn json_boundary_reaches_wuxia_seoharin_empty_place_through_preview_bundle() {
    let sealed_truth_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
    ]);

    let empty_place_page_json = scene_page_json(&sealed_truth_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Seo Harin empty-place scene page should serialize");
    let empty_place_page: Value = serde_json::from_str(&empty_place_page_json)
        .expect("Seo Harin empty-place page JSON should parse");
    assert_eq!(empty_place_page["mode"], "encounter");
    assert_eq!(empty_place_page["title"], "비워둔 자리");
    assert_eq!(
        empty_place_page["visual"]["id"],
        "wuxia_seoharin_empty_place"
    );
    assert_eq!(empty_place_page["visual"]["kind"], "empty_place_memory");
    assert_eq!(
        empty_place_page["effect_cues"][0]["stable_terms"][0],
        "서하린"
    );
    let empty_place_action_ids: Vec<&str> = empty_place_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        empty_place_action_ids,
        vec![
            "choice:ask_who_kept_the_empty_place",
            "choice:leave_the_place_unclaimed",
            "choice:set_down_the_work_notebook_briefly",
            "choice:step_back_without_naming_mumyeong",
        ]
    );

    let empty_place_result_json = apply_action_json(
        &sealed_truth_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:set_down_the_work_notebook_briefly",
    )
    .expect("set down notebook action should serialize");
    let empty_place_result: Value = serde_json::from_str(&empty_place_result_json)
        .expect("Seo Harin empty-place action should parse");
    assert_eq!(
        empty_place_result["encounter_id"],
        "wuxia_seoharin_empty_place"
    );
    let flags = empty_place_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "seoharin_empty_place_resolved"));
    assert!(flags.iter().any(|flag| flag == "seoharin_axis_opened"));
    assert!(flags.iter().any(|flag| flag == "empty_place_remembered"));
    assert!(flags
        .iter()
        .any(|flag| flag == "truth_delivery_still_unopened"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = empty_place_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "unpriced_wooden_sword_condition_seeded"));
    assert!(clues
        .iter()
        .any(|clue| clue == "empty_place_is_return_not_claim"));
    let items = empty_place_result["state"]["inventory"]
        .as_array()
        .expect("inventory should be an array");
    assert!(!items
        .iter()
        .any(|item| item == "item_unpriced_wooden_sword"));
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
fn json_boundary_reaches_wuxia_seoharin_left_meal_through_preview_bundle() {
    let empty_place_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
    ]);

    let left_meal_page_json = scene_page_json(&empty_place_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Seo Harin left-meal scene page should serialize");
    let left_meal_page: Value = serde_json::from_str(&left_meal_page_json)
        .expect("Seo Harin left-meal page JSON should parse");
    assert_eq!(left_meal_page["mode"], "encounter");
    assert_eq!(left_meal_page["title"], "남겨둔 밥");
    assert_eq!(left_meal_page["visual"]["id"], "wuxia_seoharin_left_meal");
    assert_eq!(left_meal_page["visual"]["kind"], "left_meal_memory");
    assert_eq!(
        left_meal_page["effect_cues"][0]["stable_terms"][1],
        "밥그릇"
    );
    let left_meal_action_ids: Vec<&str> = left_meal_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        left_meal_action_ids,
        vec![
            "choice:eat_the_left_meal_quietly",
            "choice:thank_seoharin_for_the_bowl",
            "choice:joke_about_who_ordered_extra_rice",
            "choice:pass_without_eating_the_meal",
        ]
    );

    let left_meal_result_json = apply_action_json(
        &empty_place_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:eat_the_left_meal_quietly",
    )
    .expect("eat left meal action should serialize");
    let left_meal_result: Value = serde_json::from_str(&left_meal_result_json)
        .expect("Seo Harin left-meal action should parse");
    assert_eq!(left_meal_result["encounter_id"], "wuxia_seoharin_left_meal");
    let flags = left_meal_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "seoharin_left_meal_resolved"));
    assert!(flags.iter().any(|flag| flag == "seoharin_axis_deepened"));
    assert!(flags.iter().any(|flag| flag == "qingliu_belonging_warmed"));
    assert!(flags
        .iter()
        .any(|flag| flag == "truth_delivery_still_unopened"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = left_meal_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "left_meal_was_kept_for_return"));
    assert!(clues.iter().any(|clue| clue == "belonging_is_daily_care"));
}

#[test]
fn json_boundary_reaches_wuxia_sado_final_phase_1_price_tag_through_preview_bundle() {
    let price_tag_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
    ]);

    let price_tag_page_json = scene_page_json(&price_tag_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Sado final phase 1 price-tag scene page should serialize");
    let price_tag_page: Value = serde_json::from_str(&price_tag_page_json)
        .expect("Sado final phase 1 price-tag page JSON should parse");
    assert_eq!(price_tag_page["mode"], "encounter");
    assert_eq!(price_tag_page["title"], "사도 최종전 1페이즈: 가격표");
    assert_eq!(
        price_tag_page["location"]["id"],
        "cheongryu_outer_courtyard"
    );
    assert_eq!(
        price_tag_page["visual"]["id"],
        "wuxia_sado_final_phase_1_price_tag"
    );
    assert_eq!(price_tag_page["visual"]["kind"], "final_phase_price_tag");
    assert_eq!(price_tag_page["effect_cues"][0]["stable_terms"][1], "장부");
    let price_tag_action_ids: Vec<&str> = price_tag_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        price_tag_action_ids,
        vec![
            "choice:approach_sado_before_the_ledger",
            "choice:burn_the_blackscale_ledger",
            "choice:secure_the_blackscale_ledger",
            "choice:ease_hostage_pressure_first",
        ]
    );

    let secure_ledger_result_json = apply_action_json(
        &price_tag_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:secure_the_blackscale_ledger",
    )
    .expect("secure blackscale ledger action should serialize");
    let secure_ledger_result: Value = serde_json::from_str(&secure_ledger_result_json)
        .expect("Sado final phase 1 price-tag action should parse");
    assert_eq!(
        secure_ledger_result["encounter_id"],
        "wuxia_sado_final_phase_1_price_tag"
    );
    assert_eq!(
        secure_ledger_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = secure_ledger_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "sado_final_phase_1_price_tag_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_state_routing_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_network_ledger_secured_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_evidence_strong_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_item_logs_blackscale_ledger_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "wuxia_sado_final_battle_started"));
    let clues = secure_ledger_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "item_blackscale_ledger_logged"));
    assert!(clues
        .iter()
        .any(|clue| clue == "black_serpent_network_structure_seen"));
}

#[test]
fn json_boundary_reaches_wuxia_sado_final_phase_2_weakpoint_control_through_preview_bundle() {
    let weakpoint_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
        "choice:secure_the_blackscale_ledger",
    ]);

    let weakpoint_page_json = scene_page_json(&weakpoint_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Sado final phase 2 weakpoint-control scene page should serialize");
    let weakpoint_page: Value = serde_json::from_str(&weakpoint_page_json)
        .expect("Sado final phase 2 weakpoint-control page JSON should parse");
    assert_eq!(weakpoint_page["mode"], "encounter");
    assert_eq!(weakpoint_page["title"], "사도 최종전 2페이즈: 약점 장악");
    assert_eq!(
        weakpoint_page["location"]["id"],
        "black_serpent_ledger_vault"
    );
    assert_eq!(
        weakpoint_page["visual"]["id"],
        "wuxia_sado_final_phase_2_weakpoint_control"
    );
    assert_eq!(
        weakpoint_page["visual"]["kind"],
        "final_phase_weakpoint_control"
    );
    assert_eq!(weakpoint_page["effect_cues"][0]["stable_terms"][3], "약점");
    let weakpoint_action_ids: Vec<&str> = weakpoint_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        weakpoint_action_ids,
        vec![
            "choice:respond_to_seoharin_pressure",
            "choice:return_flow_to_mumyeong",
            "choice:read_dangerous_cheongirok_sentence",
            "choice:focus_on_sado",
        ]
    );

    let return_flow_result_json = apply_action_json(
        &weakpoint_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:return_flow_to_mumyeong",
    )
    .expect("return flow to Mumyeong action should serialize");
    let return_flow_result: Value = serde_json::from_str(&return_flow_result_json)
        .expect("Sado final phase 2 weakpoint-control action should parse");
    assert_eq!(
        return_flow_result["encounter_id"],
        "wuxia_sado_final_phase_2_weakpoint_control"
    );
    assert_eq!(
        return_flow_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = return_flow_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "sado_final_phase_2_weakpoint_control_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_phase_2_weakpoint_control_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_mumyeong_salvation_partial_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_successor_route_suppressed_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_player_method_protected_as_person_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "wuxia_sado_final_battle_started"));
    let clues = return_flow_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues.iter().any(|clue| clue == "mumyeong_flow_is_not_tool"));
    assert!(clues.iter().any(|clue| clue == "successor_logic_wavers"));
}

#[test]
fn json_boundary_reaches_wuxia_sado_final_phase_3_outside_calculation_through_preview_bundle() {
    let outside_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
        "choice:secure_the_blackscale_ledger",
        "choice:return_flow_to_mumyeong",
    ]);

    let outside_page_json = scene_page_json(&outside_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("Sado final phase 3 outside-calculation scene page should serialize");
    let outside_page: Value = serde_json::from_str(&outside_page_json)
        .expect("Sado final phase 3 outside-calculation page JSON should parse");
    assert_eq!(outside_page["mode"], "encounter");
    assert_eq!(outside_page["title"], "사도 최종전 3페이즈: 계산식 밖");
    assert_eq!(outside_page["location"]["id"], "black_serpent_ledger_vault");
    assert_eq!(
        outside_page["visual"]["id"],
        "wuxia_sado_final_phase_3_outside_calculation"
    );
    assert_eq!(
        outside_page["visual"]["kind"],
        "final_phase_outside_calculation"
    );
    assert_eq!(outside_page["effect_cues"][0]["stable_terms"][0], "계산식");
    let outside_action_ids: Vec<&str> = outside_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        outside_action_ids,
        vec![
            "choice:remember_the_empty_place",
            "choice:let_mumyeong_choose_own_flow",
            "choice:endure_with_qingliu_will",
            "choice:point_to_blank_in_ledger",
            "choice:answer_with_sado_calculation",
        ]
    );

    let empty_place_result_json = apply_action_json(
        &outside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:remember_the_empty_place",
    )
    .expect("remember empty place action should serialize");
    let empty_place_result: Value = serde_json::from_str(&empty_place_result_json)
        .expect("Sado final phase 3 outside-calculation action should parse");
    assert_eq!(
        empty_place_result["encounter_id"],
        "wuxia_sado_final_phase_3_outside_calculation"
    );
    assert_eq!(
        empty_place_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = empty_place_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "sado_final_phase_3_outside_calculation_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_phase_3_outside_calculation_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_combat_result_battle_victory_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_boss_resolution_true_route_candidate_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_player_method_outside_calculation_confirmed_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "wuxia_sado_final_battle_started"));
    let clues = empty_place_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "empty_place_is_not_for_sale"));
    assert!(clues
        .iter()
        .any(|clue| clue == "sado_calculation_fails_to_price_waiting"));
}

#[test]
fn json_boundary_reaches_wuxia_boss_resolution_through_preview_bundle() {
    let outside_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
        "choice:secure_the_blackscale_ledger",
        "choice:return_flow_to_mumyeong",
    ]);

    let boss_result_json = apply_action_json(
        &outside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:remember_the_empty_place",
    )
    .expect("remember empty place action should serialize");
    let boss_result: Value =
        serde_json::from_str(&boss_result_json).expect("phase 3 action JSON should parse");
    let boss_state_json =
        serde_json::to_string(&boss_result["state"]).expect("boss state should stringify");
    let boss_page_json = scene_page_json(&boss_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("boss resolution scene page should serialize");
    let boss_page: Value =
        serde_json::from_str(&boss_page_json).expect("boss resolution page JSON should parse");
    assert_eq!(boss_page["mode"], "encounter");
    assert_eq!(boss_page["title"], "보스 결산");
    assert_eq!(boss_page["location"]["id"], "black_serpent_ledger_vault");
    assert_eq!(boss_page["visual"]["id"], "wuxia_boss_resolution");
    assert_eq!(boss_page["visual"]["kind"], "boss_resolution_seed");
    assert_eq!(boss_page["effect_cues"][0]["stable_terms"][0], "보스 결산");
    let boss_action_ids: Vec<&str> = boss_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        boss_action_ids,
        vec![
            "choice:confirm_true_route_outside_calculation",
            "choice:confirm_meaningful_victory_with_evidence",
            "choice:confirm_incomplete_victory_residue",
            "choice:confirm_mumyeong_unsaved_successor_risk",
            "choice:confirm_corrupted_victory",
        ]
    );

    let true_route_result_json = apply_action_json(
        &boss_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:confirm_true_route_outside_calculation",
    )
    .expect("true-route boss resolution action should serialize");
    let true_route_result: Value = serde_json::from_str(&true_route_result_json)
        .expect("boss resolution action JSON should parse");
    assert_eq!(true_route_result["encounter_id"], "wuxia_boss_resolution");
    assert_eq!(
        true_route_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = true_route_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags.iter().any(|flag| flag == "boss_resolution_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_boss_resolution_true_route_confirmed_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_result_priority_applied_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_epilogue_candidates_true_route_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "item_unpriced_wooden_sword"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = true_route_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "boss_resolution_true_route_requires_unpriced_things"));
    assert!(clues
        .iter()
        .any(|clue| clue == "broken_black_serpent_not_simple_happy_ending"));
}

#[test]
fn json_boundary_reaches_wuxia_mumyeong_resolution_after_boss_resolution() {
    let outside_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
        "choice:secure_the_blackscale_ledger",
        "choice:return_flow_to_mumyeong",
    ]);
    let boss_result_json = apply_action_json(
        &outside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:remember_the_empty_place",
    )
    .expect("remember empty place action should serialize");
    let boss_result: Value =
        serde_json::from_str(&boss_result_json).expect("phase 3 action JSON should parse");
    let boss_state_json =
        serde_json::to_string(&boss_result["state"]).expect("boss state should stringify");
    let mumyeong_result_json = apply_action_json(
        &boss_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:confirm_mumyeong_unsaved_successor_risk",
    )
    .expect("boss resolution successor-risk action should serialize");
    let mumyeong_result: Value =
        serde_json::from_str(&mumyeong_result_json).expect("boss resolution JSON should parse");
    let mumyeong_state_json =
        serde_json::to_string(&mumyeong_result["state"]).expect("mumyeong state should stringify");
    let mumyeong_page_json = scene_page_json(&mumyeong_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("mumyeong resolution scene page should serialize");
    let mumyeong_page: Value =
        serde_json::from_str(&mumyeong_page_json).expect("mumyeong page JSON should parse");

    assert_eq!(mumyeong_page["mode"], "encounter");
    assert_eq!(mumyeong_page["title"], "무명 결산");
    assert_eq!(
        mumyeong_page["location"]["id"],
        "black_serpent_ledger_vault"
    );
    assert_eq!(mumyeong_page["visual"]["id"], "wuxia_mumyeong_resolution");
    assert_eq!(mumyeong_page["visual"]["kind"], "mumyeong_resolution_seed");
    assert_eq!(mumyeong_page["effect_cues"][0]["stable_terms"][0], "무명");
    let mumyeong_action_ids: Vec<&str> = mumyeong_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        mumyeong_action_ids,
        vec![
            "choice:ask_mumyeong_for_own_flow",
            "choice:reveal_boss_used_mumyeongs_wound",
            "choice:leave_room_for_unsent_apology",
            "choice:let_stolen_forms_end",
            "choice:confirm_black_serpent_successor_risk",
            "choice:judge_with_sado_style_calculation",
        ]
    );

    let own_flow_result_json = apply_action_json(
        &mumyeong_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:ask_mumyeong_for_own_flow",
    )
    .expect("own-flow mumyeong resolution action should serialize");
    let own_flow_result: Value = serde_json::from_str(&own_flow_result_json)
        .expect("mumyeong resolution action JSON should parse");
    assert_eq!(own_flow_result["encounter_id"], "wuxia_mumyeong_resolution");
    assert_eq!(
        own_flow_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = own_flow_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "mumyeong_resolution_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_mumyeong_resolution_own_flow_salvation_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_mumyeong_successor_route_suppressed_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "item_unpriced_wooden_sword"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = own_flow_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "mumyeong_salvation_is_not_return_to_qingliu"));
    assert!(clues
        .iter()
        .any(|clue| clue == "own_flow_suppresses_successor_route"));
}

#[test]
fn json_boundary_reaches_wuxia_seoharin_qingliu_resolution_after_mumyeong_resolution() {
    let outside_state_json = wuxia_state_after_actions(&[
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
        "choice:listen_for_breath_mismatch",
        "choice:reconstruct_mumyeongs_sightline",
        "choice:show_the_hyeonakmun_trace_without_accusing",
        "choice:watch_mumyeong_answer_the_boss",
        "choice:search_the_rejected_aid_letters",
        "choice:compare_anger_to_copied_flow",
        "choice:inspect_bokho_lock_scars",
        "choice:read_hyeonakmun_empty_gate_record",
        "choice:trace_boss_offer_after_hyeonakmun",
        "choice:assemble_departure_truth_without_delivering",
        "choice:set_down_the_work_notebook_briefly",
        "choice:eat_the_left_meal_quietly",
        "choice:secure_the_blackscale_ledger",
        "choice:return_flow_to_mumyeong",
    ]);
    let phase_3_result_json = apply_action_json(
        &outside_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:remember_the_empty_place",
    )
    .expect("remember empty place action should serialize");
    let phase_3_result: Value =
        serde_json::from_str(&phase_3_result_json).expect("phase 3 action JSON should parse");
    let boss_state_json =
        serde_json::to_string(&phase_3_result["state"]).expect("boss state should stringify");
    let boss_result_json = apply_action_json(
        &boss_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:confirm_true_route_outside_calculation",
    )
    .expect("true-route boss resolution action should serialize");
    let boss_result: Value =
        serde_json::from_str(&boss_result_json).expect("boss resolution JSON should parse");
    let mumyeong_state_json =
        serde_json::to_string(&boss_result["state"]).expect("mumyeong state should stringify");
    let mumyeong_result_json = apply_action_json(
        &mumyeong_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:ask_mumyeong_for_own_flow",
    )
    .expect("own-flow mumyeong resolution action should serialize");
    let mumyeong_result: Value =
        serde_json::from_str(&mumyeong_result_json).expect("mumyeong resolution JSON should parse");
    let seoharin_state_json = serde_json::to_string(&mumyeong_result["state"])
        .expect("seoharin qingliu state should stringify");
    let seoharin_page_json = scene_page_json(&seoharin_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("seoharin qingliu resolution scene page should serialize");
    let seoharin_page: Value =
        serde_json::from_str(&seoharin_page_json).expect("seoharin qingliu page JSON should parse");

    assert_eq!(seoharin_page["mode"], "encounter");
    assert_eq!(seoharin_page["title"], "서하린·청류문 결산");
    assert_eq!(
        seoharin_page["location"]["id"],
        "black_serpent_ledger_vault"
    );
    assert_eq!(
        seoharin_page["visual"]["id"],
        "wuxia_seoharin_qingliu_resolution"
    );
    assert_eq!(
        seoharin_page["visual"]["kind"],
        "seoharin_qingliu_resolution_seed"
    );
    assert_eq!(seoharin_page["effect_cues"][0]["stable_terms"][0], "서하린");
    let action_ids: Vec<&str> = seoharin_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        action_ids,
        vec![
            "choice:leave_the_gate_unclosed",
            "choice:record_qingliu_rebuild_without_glory",
            "choice:keep_empty_place_for_return_or_absence",
            "choice:mark_qingliu_pressure_still_unresolved",
            "choice:close_the_gate_with_sado_logic",
        ]
    );

    let open_gate_result_json = apply_action_json(
        &seoharin_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:leave_the_gate_unclosed",
    )
    .expect("open-gate seoharin qingliu resolution action should serialize");
    let open_gate_result: Value = serde_json::from_str(&open_gate_result_json)
        .expect("seoharin qingliu resolution action JSON should parse");
    assert_eq!(
        open_gate_result["encounter_id"],
        "wuxia_seoharin_qingliu_resolution"
    );
    assert_eq!(
        open_gate_result["state"]["location_id"],
        "black_serpent_ledger_vault"
    );
    let flags = open_gate_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "seoharin_qingliu_resolution_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_epilogue_seoharin_open_gate_candidate_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_epilogue_qingliu_future_candidate_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "item_unpriced_wooden_sword"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = open_gate_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "open_gate_is_not_possession"));
    assert!(clues
        .iter()
        .any(|clue| clue == "qingliu_future_is_poor_but_flowing"));

    let unsaid_state_json = serde_json::to_string(&open_gate_result["state"])
        .expect("seoharin unsaid stay state should stringify");
    let unsaid_page_json = scene_page_json(&unsaid_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("seoharin unsaid stay scene page should serialize");
    let unsaid_page: Value =
        serde_json::from_str(&unsaid_page_json).expect("seoharin unsaid page JSON should parse");

    assert_eq!(unsaid_page["mode"], "encounter");
    assert_eq!(unsaid_page["title"], "가지 말라는 말");
    assert_eq!(unsaid_page["visual"]["id"], "wuxia_seoharin_unsaid_stay");
    assert_eq!(unsaid_page["visual"]["kind"], "seoharin_unsaid_stay_seed");
    assert_eq!(unsaid_page["effect_cues"][0]["stable_terms"][0], "서하린");
    let unsaid_action_ids: Vec<&str> = unsaid_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        unsaid_action_ids,
        vec![
            "choice:say_return_home_honestly",
            "choice:say_you_will_stay_with_qingliu",
            "choice:share_uncertainty_without_running",
            "choice:turn_away_from_the_empty_place",
        ]
    );

    let honest_return_result_json = apply_action_json(
        &unsaid_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:say_return_home_honestly",
    )
    .expect("honest return seoharin unsaid stay action should serialize");
    let honest_return_result: Value = serde_json::from_str(&honest_return_result_json)
        .expect("seoharin unsaid stay action JSON should parse");
    assert_eq!(
        honest_return_result["encounter_id"],
        "wuxia_seoharin_unsaid_stay"
    );
    let flags = honest_return_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "seoharin_unsaid_stay_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_return_settlement_contract_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_return_intent_honest_seeded"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_epilogue_return_absence_candidate_seeded"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "return_settlement_schema_opened"));
    let clues = honest_return_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "leaving_can_still_leave_a_place"));

    let cheongirok_state_json = serde_json::to_string(&honest_return_result["state"])
        .expect("cheongirok state should stringify");
    let cheongirok_page_json = scene_page_json(&cheongirok_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("cheongirok resolution scene page should serialize");
    let cheongirok_page: Value =
        serde_json::from_str(&cheongirok_page_json).expect("cheongirok page JSON should parse");

    assert_eq!(cheongirok_page["mode"], "encounter");
    assert_eq!(cheongirok_page["title"], "천기록 결산");
    assert_eq!(
        cheongirok_page["visual"]["id"],
        "wuxia_cheongirok_resolution"
    );
    assert_eq!(
        cheongirok_page["visual"]["kind"],
        "cheongirok_resolution_seed"
    );
    assert_eq!(
        cheongirok_page["effect_cues"][0]["stable_terms"][0],
        "천기록"
    );
    let cheongirok_action_ids: Vec<&str> = cheongirok_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        cheongirok_action_ids,
        vec![
            "choice:turn_the_last_page_without_question",
            "choice:leave_blank_as_unpriced_place",
            "choice:read_the_lines_that_align_like_ledger",
            "choice:close_record_before_it_becomes_answer",
            "choice:let_record_reflect_the_method",
        ]
    );

    let last_page_result_json = apply_action_json(
        &cheongirok_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:turn_the_last_page_without_question",
    )
    .expect("safe high-use cheongirok resolution action should serialize");
    let last_page_result: Value = serde_json::from_str(&last_page_result_json)
        .expect("cheongirok resolution action JSON should parse");
    assert_eq!(
        last_page_result["encounter_id"],
        "wuxia_cheongirok_resolution"
    );
    let flags = last_page_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "cheongirok_resolution_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_cheongirok_state_high_use_not_corruption_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "final_cheongirok_identity_revealed"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = last_page_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "record_does_not_answer_questions"));

    let aftermath_state_json = serde_json::to_string(&last_page_result["state"])
        .expect("aftermath state should stringify");
    let aftermath_page_json = scene_page_json(&aftermath_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("black serpent aftermath scene page should serialize");
    let aftermath_page: Value = serde_json::from_str(&aftermath_page_json)
        .expect("black serpent aftermath page JSON should parse");

    assert_eq!(aftermath_page["mode"], "encounter");
    assert_eq!(aftermath_page["title"], "흑사방 후일담 씨앗");
    assert_eq!(
        aftermath_page["visual"]["id"],
        "wuxia_black_serpent_aftermath"
    );
    assert_eq!(
        aftermath_page["visual"]["kind"],
        "black_serpent_aftermath_seed"
    );
    assert_eq!(
        aftermath_page["effect_cues"][0]["stable_terms"][0],
        "흑사방"
    );
    let aftermath_action_ids: Vec<&str> = aftermath_page["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| action["id"].as_str().expect("action id should be a string"))
        .collect();
    assert_eq!(
        aftermath_action_ids,
        vec![
            "choice:mark_broken_serpent_without_erasing_scars",
            "choice:fold_the_banner_without_calling_it_gone",
            "choice:send_ledger_to_alliance_and_watch_silence",
            "choice:listen_for_southern_market_debt_rumor",
            "choice:let_true_route_suppress_the_banner",
        ]
    );

    let broken_serpent_result_json = apply_action_json(
        &aftermath_state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:mark_broken_serpent_without_erasing_scars",
    )
    .expect("broken serpent aftermath action should serialize");
    let broken_serpent_result: Value = serde_json::from_str(&broken_serpent_result_json)
        .expect("black serpent aftermath action JSON should parse");
    assert_eq!(
        broken_serpent_result["encounter_id"],
        "wuxia_black_serpent_aftermath"
    );
    let flags = broken_serpent_result["state"]["flags"]
        .as_array()
        .expect("flags should be an array");
    assert!(flags
        .iter()
        .any(|flag| flag == "black_serpent_aftermath_resolved"));
    assert!(flags
        .iter()
        .any(|flag| flag == "final_epilogue_boss_broken_black_serpent_variant_ready_seeded"));
    assert!(!flags
        .iter()
        .any(|flag| flag == "final_epilogue_renderer_opened"));
    assert!(!flags.iter().any(|flag| flag == "told_seoharin_truth"));
    let clues = broken_serpent_result["state"]["clues"]
        .as_array()
        .expect("clues should be an array");
    assert!(clues
        .iter()
        .any(|clue| clue == "broken_serpent_still_leaves_network_scars"));

    let final_state_json = serde_json::to_string(&broken_serpent_result["state"])
        .expect("final epilogue state should stringify");
    let final_page_json = scene_page_json(&final_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("final epilogue scene page should serialize");
    let final_page: Value =
        serde_json::from_str(&final_page_json).expect("final epilogue page JSON should parse");

    assert_eq!(final_page["mode"], "ending");
    assert_eq!(final_page["title"], "이구학지 결산");
    assert_eq!(
        final_page["visual"]["id"],
        "ending:wuxia_final_epilogue_renderer_contract"
    );
    let body_blocks = final_page["body_blocks"]
        .as_array()
        .expect("final body blocks should be an array");
    assert!(body_blocks.iter().any(|block| {
        block["kind"] == "epilogue_result"
            && block["text"]
                .as_str()
                .expect("epilogue result text should be a string")
                .contains("final_result_key: true_route_victory")
    }));
    assert!(body_blocks.iter().any(|block| {
        block["kind"] == "epilogue_card"
            && block["text"]
                .as_str()
                .expect("epilogue card text should be a string")
                .contains("card_id: epilogue_boss_broken_black_serpent")
    }));
    assert!(body_blocks.iter().any(|block| {
        block["kind"] == "epilogue_card"
            && block["text"]
                .as_str()
                .expect("epilogue card text should be a string")
                .contains("card_id: epilogue_wuxia_returned_commute")
    }));
    assert!(final_page["actions"]
        .as_array()
        .expect("final page actions should be an array")
        .is_empty());
}

#[test]
fn json_boundary_outputs_wuxia_battle_loss_epilogue_bundle() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let mut state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    state["location_id"] = json!("black_serpent_ledger_vault");
    state["flags"] = json!([
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
        "final_epilogue_seoharin_open_gate_candidate_seeded"
    ]);
    let battle_loss_state_json =
        serde_json::to_string(&state).expect("battle-loss state should stringify");
    let final_page_json = scene_page_json(&battle_loss_state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("battle-loss final epilogue page should serialize");
    let final_page: Value =
        serde_json::from_str(&final_page_json).expect("final page JSON should parse");
    let body_blocks = final_page["body_blocks"]
        .as_array()
        .expect("final body blocks should be an array");
    let all_body_text = body_blocks
        .iter()
        .map(|block| {
            block["text"]
                .as_str()
                .expect("body block text should be a string")
        })
        .collect::<Vec<_>>()
        .join("\n---\n");

    assert_eq!(final_page["mode"], "ending");
    assert_eq!(final_page["title"], "이구학지 결산");
    assert!(all_body_text.contains("final_result_key: battle_loss"));
    assert!(all_body_text.contains("card_id: epilogue_boss_black_serpent_banner"));
    assert!(all_body_text.contains("variant: battle_loss_residue"));
    assert!(all_body_text.contains("card_id: epilogue_wuxia_southern_market_rumor"));
    assert!(all_body_text.contains("card_id: epilogue_mumyeong_black_serpent_new_scale"));
    assert!(all_body_text.contains("variant: battle_loss_successor_pressure"));
    assert!(all_body_text.contains("card_id: epilogue_seoharin_closed_gate"));
    assert!(all_body_text.contains("variant: battle_loss_or_corruption"));
    assert!(all_body_text.contains("card_id: epilogue_tianjilu_last_page"));
    assert!(all_body_text.contains("suppressed_by: battle_loss"));
    assert!(all_body_text.contains("card_id: epilogue_boss_broken_black_serpent"));
    assert!(all_body_text.contains("card_id: epilogue_seoharin_open_gate"));
    assert!(all_body_text.contains("card_id: epilogue_mumyeong_stolen_forms_stopped"));
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
