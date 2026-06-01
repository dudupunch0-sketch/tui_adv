use escape_wasm::{
    apply_action_json, load_state_json, new_game_json, save_state_json, scene_page_json,
};
use serde_json::Value;

const CONTENT_BUNDLE: &str = include_str!("../../escape-core/fixtures/content/content.bundle.json");

const WUXIA_PREVIEW_BUNDLE: &str = include_str!(
    "../../escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json"
);

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
