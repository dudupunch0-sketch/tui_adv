use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
    scene_page_from_content, ContentIndexError, SaveEnvelope,
};
use serde_json::json;

const BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

fn event_bundle() -> escape_core::ContentBundle {
    let mut bundle = load_content_bundle(BUNDLE).unwrap();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["choices"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|choice| choice["id"] == "read_printout")
        .unwrap()["check"] = json!({
        "ability": "logic",
        "difficulty": 7,
        "success": {"log": "인쇄 순서를 읽었다."},
        "failure": {"log": "인쇄 순서를 놓쳤다."}
    });
    encounter["event"] = json!({"stages": [
        {"id":"opening","kind":"story","blocks":[
            {"kind":"narration","text":"첫 문단"},
            {"kind":"illustration","visual_id":"printer-event.png","alt":"출력 중인 복합기","placeholder":true},
            {"kind":"dialogue","speaker":"동료","text":"저것 좀 봐."}
        ]},
        {"id":"decision","kind":"choice","blocks":[{"kind":"system","text":"무엇을 할까?"}],
         "choices":[{"id":"read_printout","next_stage_id":"closing"}]},
        {"id":"result","kind":"result","blocks":[
            {"kind":"result_summary","text":"종이가 멈췄다."},
            {"kind":"narration","text":"성공 분기 결과","branch":"success"},
            {"kind":"narration","text":"실패 분기 결과","branch":"failure"}
        ]},
        {"id":"closing","kind":"story","blocks":[{"kind":"document","text":"출력 시각: 내일"}]}
    ]});
    bundle
}

#[test]
fn ordered_event_stages_progress_and_serialize_as_content_stream() {
    let index = index_content_bundle(&event_bundle()).unwrap();
    let state = new_game_from_content_at(7, &index, "printer_area").unwrap();
    let opening = scene_page_from_content(&state, &index).unwrap();
    assert_eq!(
        opening
            .content_stream
            .iter()
            .map(|item| item.kind.as_str())
            .collect::<Vec<_>>(),
        ["narration", "illustration", "dialogue", "continue"]
    );
    assert!(opening.content_stream[1].placeholder);

    let decision_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    assert_eq!(
        decision_state.active_event_id.as_deref(),
        Some("printer_prints_alone")
    );
    assert_eq!(decision_state.event_stage_index, 1);
    let decision = scene_page_from_content(&decision_state, &index).unwrap();
    assert_eq!(decision.content_stream.last().unwrap().kind, "choice");
    assert_eq!(
        decision.content_stream.last().unwrap().actions[0].id,
        "choice:read_printout"
    );
    let unauthorized = apply_action_from_content(&decision_state, &index, "choice:check_toner")
        .expect_err("definition-pool choices outside the current stage must stay unavailable");
    assert!(unauthorized.to_string().contains("unknown action id"));
    assert_eq!(decision.content_stream.last().unwrap().actions.len(), 1);
    assert!(apply_action_from_content(&decision_state, &index, "choice:take_printout").is_err());

    let result_state = apply_action_from_content(&decision_state, &index, "choice:read_printout")
        .unwrap()
        .state;
    assert!(result_state.last_check.is_some());
    assert_eq!(result_state.event_stage_index, 2);
    assert_eq!(result_state.event_next_stage_id.as_deref(), Some("closing"));
    assert_eq!(
        scene_page_from_content(&result_state, &index)
            .unwrap()
            .content_stream[0]
            .kind,
        "result_summary"
    );

    let closing_state = apply_action_from_content(&result_state, &index, "event:continue")
        .unwrap()
        .state;
    assert!(
        closing_state.last_check.is_none(),
        "event:continue must clear the checked choice result"
    );
    assert_eq!(closing_state.event_stage_index, 3);
    let closing_page = scene_page_from_content(&closing_state, &index).unwrap();
    assert_eq!(closing_page.content_stream[0].kind, "document");
    assert!(closing_page.check_result.is_none());
    let done = apply_action_from_content(&closing_state, &index, "event:continue")
        .unwrap()
        .state;
    assert_eq!(done.active_event_id, None);
    assert!(done
        .seen_encounters
        .contains(&"printer_prints_alone".to_string()));
}

#[test]
fn event_validation_rejects_choice_without_immediate_result() {
    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"]
        .as_array_mut()
        .unwrap()
        .remove(2);
    let error = index_content_bundle(&bundle).unwrap_err();
    assert!(matches!(error, ContentIndexError::InvalidEvent { .. }));
    assert!(error.to_string().contains("immediately followed"));
}

#[test]
fn event_validation_requires_an_illustration_slot_with_accessible_metadata() {
    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"][0]["blocks"] =
        json!([{"kind":"narration","text":"그림은 아직 준비되지 않았다."}]);
    let missing_slot = index_content_bundle(&bundle).unwrap_err();
    assert!(missing_slot
        .to_string()
        .contains("requires at least one illustration block"));

    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"][0]["blocks"][1]["alt"] = json!("");
    let missing_alt = index_content_bundle(&bundle).unwrap_err();
    assert!(missing_alt.to_string().contains("visual_id and alt"));
}

#[test]
fn old_save_json_defaults_event_cursor_fields() {
    let index = index_content_bundle(&event_bundle()).unwrap();
    let state = new_game_from_content_at(7, &index, "printer_area").unwrap();
    let mut value = serde_json::to_value(SaveEnvelope {
        schema_version: 1,
        state,
    })
    .unwrap();
    let object = value["state"].as_object_mut().unwrap();
    object.remove("active_event_id");
    object.remove("event_stage_index");
    object.remove("event_next_stage_id");
    let loaded: SaveEnvelope = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.state.active_event_id, None);
    assert_eq!(loaded.state.event_stage_index, 0);
}

#[test]
fn result_stage_blocks_follow_success_and_failure_check_branches() {
    let index = index_content_bundle(&event_bundle()).unwrap();
    let mut seen_success = false;
    let mut seen_failure = false;

    for seed in 1..=512 {
        let state = new_game_from_content_at(seed, &index, "printer_area").unwrap();
        let decision_state = apply_action_from_content(&state, &index, "event:continue")
            .unwrap()
            .state;
        let result_state =
            apply_action_from_content(&decision_state, &index, "choice:read_printout")
                .unwrap()
                .state;
        let success = result_state.last_check.as_ref().unwrap().success;
        let page = scene_page_from_content(&result_state, &index).unwrap();
        let texts = page
            .content_stream
            .iter()
            .filter_map(|item| item.text.as_deref())
            .collect::<Vec<_>>();
        assert!(texts.contains(&"종이가 멈췄다."));
        if success {
            assert!(texts.contains(&"성공 분기 결과"));
            assert!(!texts.contains(&"실패 분기 결과"));
            seen_success = true;
        } else {
            assert!(texts.contains(&"실패 분기 결과"));
            assert!(!texts.contains(&"성공 분기 결과"));
            seen_failure = true;
        }
        if seen_success && seen_failure {
            break;
        }
    }

    assert!(seen_success, "test seeds should cover a successful check");
    assert!(seen_failure, "test seeds should cover a failed check");
}

#[test]
fn event_validation_rejects_unknown_result_branch() {
    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"][2]["blocks"][1]["branch"] = json!("sucess");

    let error = index_content_bundle(&bundle).unwrap_err();
    assert!(matches!(error, ContentIndexError::InvalidEvent { .. }));
    assert!(error.to_string().contains("unknown branch"));
}

#[test]
fn legacy_bundle_without_branch_field_still_loads_and_indexes() {
    let bundle = load_content_bundle(BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    assert!(index.encounters_len() > 0);
}

#[test]
fn direct_result_target_uses_result_cursor_and_ends_without_fallthrough() {
    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"][1]["choices"][0]["next_stage_id"] = json!("result");

    let index = index_content_bundle(&bundle).unwrap();
    let state = new_game_from_content_at(7, &index, "printer_area").unwrap();
    let decision_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result_state = apply_action_from_content(&decision_state, &index, "choice:read_printout")
        .unwrap()
        .state;
    assert_eq!(result_state.event_stage_index, 2);
    assert_eq!(result_state.event_next_stage_id.as_deref(), Some("result"));
    let done = apply_action_from_content(&result_state, &index, "event:continue")
        .unwrap()
        .state;
    assert_eq!(done.active_event_id, None);
}

#[test]
fn direct_result_target_honors_result_next_stage_id() {
    let mut bundle = event_bundle();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"]["stages"][1]["choices"][0]["next_stage_id"] = json!("result");
    encounter["event"]["stages"][2]["next_stage_id"] = json!("closing");

    let index = index_content_bundle(&bundle).unwrap();
    let state = new_game_from_content_at(7, &index, "printer_area").unwrap();
    let decision_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result_state = apply_action_from_content(&decision_state, &index, "choice:read_printout")
        .unwrap()
        .state;
    let closing_state = apply_action_from_content(&result_state, &index, "event:continue")
        .unwrap()
        .state;
    assert_eq!(
        closing_state.active_event_id.as_deref(),
        Some("printer_prints_alone")
    );
    assert_eq!(closing_state.event_stage_index, 3);
}
