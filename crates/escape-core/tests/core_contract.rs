use escape_core::{
    apply_action, apply_action_from_content, index_content_bundle, load_content_bundle, load_state,
    new_game, new_game_from_content, new_game_from_content_at, save_state, scene_page_from_content,
    turn_view, turn_view_from_content, ContentTurnError, EffectCue, NewGameError, SceneMode,
};

use serde_json::json;

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

    let value = serde_json::to_value(&result).expect("ActionResult should serialize");
    assert_eq!(value["effect_cues"][0]["kind"], json!("glyph_anomaly"));
    assert_eq!(value["effect_cues"][0]["source"], json!("copier_output"));
    assert_eq!(value["effect_cues"][0]["intensity"], json!(0.72));
    assert_eq!(
        value["effect_cues"][0]["stable_terms"][0],
        json!("비상계단")
    );
    assert!(value["effect_cues"][0].get("GlyphAnomaly").is_none());

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
fn content_backed_scene_page_renders_renderer_safe_encounter_contract() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");

    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    assert_eq!(page.mode, SceneMode::Encounter);
    assert_eq!(page.title, "퇴사자의 메신저");
    assert_eq!(page.location.id, "dev_desk");
    assert_eq!(page.location.name, "내 자리");
    assert_eq!(page.chapter_label, "격리 0턴");
    assert_eq!(page.status_summary.turn, 0);
    assert_eq!(page.status_summary.danger, 0);
    assert_eq!(
        page.status_summary
            .resources
            .iter()
            .map(|resource| (
                resource.id.as_str(),
                resource.label.as_str(),
                resource.band.as_str(),
                resource.text.as_str(),
                resource.value
            ))
            .collect::<Vec<_>>(),
        vec![
            ("health", "신체 반응", "normal", "정상 범위", 100),
            ("sanity", "집중도", "normal", "안정", 100),
            ("battery", "단말기 전원", "normal", "100%", 100),
            ("hunger", "허기", "normal", "버틸 만함", 0),
            ("thirst", "갈증", "normal", "버틸 만함", 0),
        ]
    );
    assert!(page.status_summary.warnings.is_empty());
    assert_eq!(page.body_blocks.len(), 1);
    assert_eq!(page.body_blocks[0].kind, "narration");
    assert_eq!(
        page.body_blocks[0].source_id.as_deref(),
        Some("ex_employee_messenger")
    );
    assert!(page.body_blocks[0].text.contains("사내 메신저"));
    assert_eq!(page.visual.id, "encounter:ex_employee_messenger");
    assert_eq!(page.visual.kind, "encounter");
    assert_eq!(
        page.visual.source_id.as_deref(),
        Some("ex_employee_messenger")
    );
    assert_eq!(
        page.actions
            .iter()
            .map(|action| (
                action.id.as_str(),
                action.kind.as_str(),
                action.label.as_str(),
                action.cost_text.as_deref()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                "choice:check_message",
                "choice",
                "메시지를 확인한다",
                Some("배터리 -3, 정신력 -2")
            ),
            (
                "choice:ignore_phone",
                "choice",
                "무시하고 휴대폰을 엎어둔다",
                None
            ),
            (
                "choice:search_ex_employee",
                "choice",
                "전임자의 이름을 사내망에서 검색한다",
                Some("배터리 -8")
            ),
        ]
    );
    assert_eq!(page.blocked_actions.len(), 1);
    assert_eq!(page.blocked_actions[0].id, "choice:trace_packet_delay");
    assert_eq!(page.blocked_actions[0].kind, "choice");
    assert_eq!(
        page.blocked_actions[0].reasons,
        vec!["능력 조건 미충족: interface >= 4".to_string()]
    );
    assert!(page.dialogue_entries.is_empty());
    assert!(page.history_entries.is_empty());
    assert!(page.inventory_summary.items.is_empty());
    assert_eq!(page.inventory_summary.overflow_count, 0);
    assert!(page.achievement_summary.unlocked.is_empty());
    assert!(page.achievement_summary.newly_unlocked.is_empty());
    assert!(page.pressure_cues.is_empty());
    assert!(page.effect_cues.is_empty());
}

#[test]
fn content_backed_scene_page_renders_renderer_safe_movement_contract() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let after_choice = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("content-backed action should resolve");

    let page =
        scene_page_from_content(&after_choice.state, &index).expect("scene page should render");

    assert_eq!(page.mode, SceneMode::Movement);
    assert_eq!(page.title, "내 자리");
    assert_eq!(page.location.id, "dev_desk");
    assert_eq!(page.chapter_label, "격리 1턴");
    assert_eq!(page.status_summary.turn, 1);
    assert_eq!(page.visual.id, "location:dev_desk");
    assert_eq!(page.visual.kind, "location");
    assert_eq!(page.visual.source_id.as_deref(), Some("dev_desk"));
    assert_eq!(page.body_blocks[0].source_id.as_deref(), Some("dev_desk"));
    assert_eq!(
        page.status_summary
            .resources
            .iter()
            .map(|resource| (resource.id.as_str(), resource.value))
            .collect::<Vec<_>>(),
        vec![
            ("health", 100),
            ("sanity", 98),
            ("battery", 97),
            ("hunger", 1),
            ("thirst", 2),
        ]
    );
    assert_eq!(page.actions.len(), 1);
    assert_eq!(page.actions[0].id, "move:dev_office");
    assert_eq!(page.actions[0].kind, "move");
    assert_eq!(page.actions[0].label, "개발팀 사무실");
    assert_eq!(page.actions[0].cost_text, None);
    assert!(page.blocked_actions.is_empty());
}

#[test]
fn content_backed_scene_page_serializes_to_documented_json_shape() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    let value = serde_json::to_value(&page).expect("ScenePage should serialize");

    assert_eq!(value["mode"], json!("encounter"));
    assert_eq!(value["title"], json!("퇴사자의 메신저"));
    assert_eq!(value["location"]["id"], json!("dev_desk"));
    assert_eq!(value["chapter_label"], json!("격리 0턴"));
    assert_eq!(
        value["status_summary"]["resources"][0]["id"],
        json!("health")
    );
    assert_eq!(value["body_blocks"][0]["kind"], json!("narration"));
    assert_eq!(
        value["visual"]["id"],
        json!("encounter:ex_employee_messenger")
    );
    assert_eq!(value["actions"][0]["id"], json!("choice:check_message"));
    assert_eq!(value["actions"][0]["kind"], json!("choice"));
    assert_eq!(
        value["actions"][0]["cost_text"],
        json!("배터리 -3, 정신력 -2")
    );
    assert_eq!(
        value["blocked_actions"][0]["reasons"][0],
        json!("능력 조건 미충족: interface >= 4")
    );
    assert_eq!(value["history_entries"], json!([]));
    assert_eq!(value["pressure_cues"], json!([]));
    assert_eq!(value["effect_cues"], json!([]));
}

#[test]
fn content_backed_scene_page_emits_pressure_cues_from_core_thresholds() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let mut state = new_game_from_content(123, &index).expect("content-backed game should start");
    state.player.sanity = 30;
    state.player.battery = 20;

    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    assert_eq!(
        page.pressure_cues
            .iter()
            .map(|cue| (
                cue.kind.as_str(),
                cue.severity.as_str(),
                cue.resource_id.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("low_sanity", "warning", "sanity"),
            ("low_battery", "warning", "battery"),
        ]
    );
    assert_eq!(
        page.status_summary.warnings,
        vec![
            "집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.".to_string(),
            "단말기 전원이 낮습니다. 전력 행동이 제한될 수 있습니다.".to_string(),
        ]
    );
}

#[test]
fn content_backed_scene_page_carries_recent_action_logs_as_history() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let after_choice = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("content-backed action should resolve");

    let page =
        scene_page_from_content(&after_choice.state, &index).expect("scene page should render");

    assert_eq!(page.history_entries.len(), 1);
    assert_eq!(page.history_entries[0].kind, "action");
    assert_eq!(page.history_entries[0].text, "퇴사자의 메시지를 확인했다.");
    assert_eq!(
        page.history_entries[0].source_id.as_deref(),
        Some("ex_employee_messenger")
    );
}

#[test]
fn content_backed_scene_page_exposes_presentation_visual_and_effect_cues() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content_at(123, &index, "printer_area")
        .expect("content-backed game should start at printer area");

    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    assert_eq!(page.mode, SceneMode::Encounter);
    assert_eq!(page.title, "복합기가 혼자 출력한다");
    assert_eq!(page.visual.id, "printer_anomaly");
    assert_eq!(page.visual.kind, "anomaly_object");
    assert_eq!(page.dialogue_entries.len(), 1);
    assert_eq!(page.dialogue_entries[0].speaker, "시스템 복합기");
    assert_eq!(
        page.dialogue_entries[0].source_id.as_deref(),
        Some("printer_prints_alone")
    );
    assert_eq!(page.effect_cues.len(), 1);
    assert_eq!(page.effect_cues[0].kind, "glyph_anomaly");
    assert_eq!(page.effect_cues[0].source, "copier_output");
    assert!((page.effect_cues[0].intensity - 0.72).abs() < f32::EPSILON);
    assert_eq!(
        page.effect_cues[0].stable_terms,
        vec!["비상계단", "토너", "접힌 방향"]
    );

    let value = serde_json::to_value(&page).expect("ScenePage should serialize");
    assert_eq!(value["effect_cues"][0]["kind"], json!("glyph_anomaly"));
    assert_eq!(
        value["effect_cues"][0]["stable_terms"][0],
        json!("비상계단")
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
