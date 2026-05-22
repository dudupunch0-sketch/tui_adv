use escape_wasm::{
    apply_action_json, load_state_json, new_game_json, save_state_json, scene_page_json,
};
use serde_json::Value;

const CONTENT_BUNDLE: &str = include_str!("../../escape-core/fixtures/content/content.bundle.json");

#[test]
fn json_boundary_creates_scene_page_applies_action_and_roundtrips_save() {
    let state_json = new_game_json(123, CONTENT_BUNDLE).expect("new game should serialize");
    let state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    assert_eq!(state["seed"], 123);
    assert_eq!(state["location_id"], "dev_desk");
    assert_eq!(state["turn"], 0);

    let page_json =
        scene_page_json(&state_json, CONTENT_BUNDLE).expect("scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "퇴사자의 메신저");
    assert_eq!(page["actions"][0]["id"], "choice:check_message");

    let result_json = apply_action_json(&state_json, CONTENT_BUNDLE, "choice:check_message")
        .expect("action result should serialize");
    let result: Value =
        serde_json::from_str(&result_json).expect("action result JSON should parse");
    assert_eq!(result["encounter_id"], "ex_employee_messenger");
    assert_eq!(result["action_id"], "choice:check_message");
    assert_eq!(result["state"]["turn"], 1);
    assert_eq!(result["state"]["player"]["battery"], 97);
    assert_eq!(result["logs"][0], "퇴사자의 메시지를 확인했다.");

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
