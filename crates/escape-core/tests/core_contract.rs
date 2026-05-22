use escape_core::{
    apply_action, apply_action_from_content, index_content_bundle, load_content_bundle, load_state,
    new_game, new_game_from_content, new_game_from_content_at, save_state, turn_view,
    turn_view_from_content, ContentTurnError, EffectCue, NewGameError,
};

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

#[test]
fn printer_scene_turn_view_exposes_renderer_safe_actions_and_glyph_cue() {
    let state = new_game(123);

    let view = turn_view(&state);

    assert_eq!(state.seed, 123);
    assert_eq!(view.location_id, "printer_area");
    assert_eq!(view.encounter_id.as_deref(), Some("printer_prints_alone"));
    assert!(view.body.contains("복합기"));
    assert_eq!(view.actions.len(), 3);
    assert_eq!(view.actions[0].id, "choice:wait_for_output");
    assert_eq!(view.actions[0].label, "출력물이 안정될 때까지 기다린다");
    assert_eq!(view.actions[1].id, "choice:inspect_toner");
    assert_eq!(view.actions[2].id, "choice:record_stable_terms");

    let [EffectCue::GlyphAnomaly(cue)] = view.effect_cues.as_slice() else {
        panic!(
            "expected exactly one GlyphAnomaly cue, got {:?}",
            view.effect_cues
        );
    };
    assert_eq!(cue.source, "copier_output");
    assert_eq!(cue.intensity, 72);
    assert_eq!(cue.distortion, "reflow_then_stabilize");
    assert_eq!(
        cue.stable_terms,
        vec![
            "비상계단".to_string(),
            "토너".to_string(),
            "접힌 방향".to_string()
        ]
    );
}

#[test]
fn printer_choice_returns_action_result_and_save_roundtrip() {
    let state = new_game(123);

    let result =
        apply_action(&state, "choice:wait_for_output").expect("printer action should resolve");

    assert_eq!(result.encounter_id, "printer_prints_alone");
    assert_eq!(result.action_id, "choice:wait_for_output");
    assert_eq!(result.state.turn, 1);
    assert_eq!(result.state.player.sanity, 65);
    assert!(result
        .state
        .flags
        .contains(&"printer_secret_started".to_string()));
    assert!(result
        .state
        .clues
        .contains(&"copier_stable_terms".to_string()));
    assert!(result
        .logs
        .iter()
        .any(|line| line.contains("비상계단") && line.contains("토너")));

    let envelope = save_state(&result.state);
    assert_eq!(envelope.schema_version, 1);
    assert_eq!(
        load_state(&envelope).expect("save envelope should restore"),
        result.state
    );
}

#[test]
fn content_backed_new_game_starts_at_indexed_default_location() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");

    let state = new_game_from_content(123, &index).expect("content-backed game should start");

    assert_eq!(state.seed, 123);
    assert_eq!(state.turn, 0);
    assert_eq!(state.location_id, "dev_desk");
    assert!(index.location(&state.location_id).is_some());
    assert_eq!(state.player.health, 100);
    assert_eq!(state.player.sanity, 100);
    assert_eq!(state.player.battery, 100);
    assert!(state.flags.is_empty());
    assert!(state.clues.is_empty());
    assert!(state.seen_encounters.is_empty());
}

#[test]
fn content_backed_new_game_rejects_unknown_custom_start_location() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");

    let error = new_game_from_content_at(123, &index, "missing_floor")
        .expect_err("unknown start location should be rejected");

    assert_eq!(
        error,
        NewGameError::UnknownStartLocation("missing_floor".to_string())
    );
}

#[test]
fn content_backed_turn_view_renders_start_encounter_choices() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");

    let view = turn_view_from_content(&state, &index).expect("content-backed turn should render");

    assert_eq!(view.location_id, "dev_desk");
    assert_eq!(view.encounter_id.as_deref(), Some("ex_employee_messenger"));
    assert_eq!(view.title, "퇴사자의 메신저");
    assert!(view.body.contains("사내 메신저"));
    assert_eq!(
        view.actions
            .iter()
            .map(|action| action.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "choice:check_message",
            "choice:ignore_phone",
            "choice:search_ex_employee",
        ]
    );
    assert_eq!(
        view.actions[0].cost_summary.as_deref(),
        Some("배터리 -3, 정신력 -2")
    );
    assert!(!view
        .actions
        .iter()
        .any(|action| action.id == "choice:trace_packet_delay"));
    assert_eq!(view.blocked_actions.len(), 1);
    assert_eq!(view.blocked_actions[0].id, "choice:trace_packet_delay");
    assert_eq!(
        view.blocked_actions[0].reasons,
        vec!["능력 조건 미충족: interface >= 4".to_string()]
    );
}

#[test]
fn content_backed_turn_view_rejects_unknown_state_location() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let mut state = new_game_from_content(123, &index).expect("content-backed game should start");
    state.location_id = "missing_floor".to_string();

    let error = turn_view_from_content(&state, &index)
        .expect_err("unknown state location should be rejected");

    assert_eq!(
        error,
        ContentTurnError::UnknownStateLocation("missing_floor".to_string())
    );
}

#[test]
fn content_backed_action_applies_cost_outcome_and_logs() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");

    let result = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("content-backed action should resolve");

    assert_eq!(result.encounter_id, "ex_employee_messenger");
    assert_eq!(result.action_id, "choice:check_message");
    assert_eq!(result.state.turn, 1);
    assert_eq!(result.state.location_id, "dev_desk");
    assert_eq!(result.state.player.health, 100);
    assert_eq!(result.state.player.sanity, 98);
    assert_eq!(result.state.player.battery, 97);
    assert!(result
        .state
        .clues
        .contains(&"ex_employee_contacted".to_string()));
    assert!(result
        .state
        .seen_encounters
        .contains(&"ex_employee_messenger".to_string()));
    assert_eq!(result.logs, vec!["퇴사자의 메시지를 확인했다.".to_string()]);
    assert!(result.effect_cues.is_empty());
}

#[test]
fn content_backed_turn_loop_exposes_movement_after_seen_encounter() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let result = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("content-backed action should resolve");

    let next_view =
        turn_view_from_content(&result.state, &index).expect("next content turn should render");

    assert_eq!(next_view.location_id, "dev_desk");
    assert_eq!(next_view.encounter_id, None);
    assert_eq!(next_view.title, "내 자리");
    assert_eq!(
        next_view
            .actions
            .iter()
            .map(|action| (action.id.as_str(), action.label.as_str()))
            .collect::<Vec<_>>(),
        vec![("move:dev_office", "개발팀 사무실")]
    );
}

#[test]
fn content_backed_movement_action_changes_location_and_logs() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let after_choice = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("content-backed action should resolve");

    let result = apply_action_from_content(&after_choice.state, &index, "move:dev_office")
        .expect("content-backed movement should resolve");

    assert_eq!(result.encounter_id, "movement");
    assert_eq!(result.action_id, "move:dev_office");
    assert_eq!(result.state.turn, 2);
    assert_eq!(result.state.location_id, "dev_office");
    assert_eq!(result.logs, vec!["개발팀 사무실로 이동했다.".to_string()]);
    assert!(result.effect_cues.is_empty());
}

#[test]
fn content_backed_movement_action_accumulates_destination_danger() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let mut state = new_game_from_content_at(123, &index, "dev_office")
        .expect("content-backed game should start at office");
    state.seen_encounters = index
        .encounters()
        .map(|encounter| encounter.id.clone())
        .collect();

    let result = apply_action_from_content(&state, &index, "move:hallway")
        .expect("content-backed movement should resolve");

    assert_eq!(state.danger, 0);
    assert_eq!(result.state.location_id, "hallway");
    assert_eq!(result.state.danger, 1);
}

#[test]
fn content_backed_action_applies_destination_and_flags() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content_at(123, &index, "server_room_front")
        .expect("content-backed game should start at server door");

    let result = apply_action_from_content(&state, &index, "choice:follow_cold_air")
        .expect("content-backed destination action should resolve");

    assert_eq!(result.encounter_id, "server_room_radio");
    assert_eq!(result.state.turn, 1);
    assert_eq!(result.state.location_id, "server_room");
    assert_eq!(result.state.player.sanity, 98);
    assert!(result
        .state
        .flags
        .contains(&"server_room_entered".to_string()));
    assert_eq!(
        result.logs,
        vec!["서버실 문은 열리지 않았지만, 당신은 이미 문 안쪽에 서 있었다.".to_string()]
    );
}
