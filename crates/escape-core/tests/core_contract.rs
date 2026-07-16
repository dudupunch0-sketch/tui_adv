use escape_core::{
    ability_check_success_percent, apply_action, apply_action_from_content, index_content_bundle,
    load_content_bundle, load_state, new_game, new_game_from_content, new_game_from_content_at,
    save_state, scene_page_from_content, turn_view, turn_view_from_content, ContentTurnError,
    EffectCue, NewGameError, SaveEnvelope, SceneMode,
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

#[test]
fn test_character_summary_serialization_shape() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    let page = scene_page_from_content(&state, &index).expect("build scene page should succeed");

    assert!(page.character_summary.is_some());
    let summary = page.character_summary.as_ref().unwrap();
    assert_eq!(summary.name, "당신");
    assert_eq!(summary.title_label, None);
    assert_eq!(summary.title_description, None);
    assert_eq!(summary.abilities.len(), 6);
    assert_eq!(summary.abilities[0].id, "logic");
    assert_eq!(summary.abilities[1].id, "empathy");

    let serialized = serde_json::to_string(&page).unwrap();
    assert!(!serialized.contains("\"title_label\""));
    assert!(!serialized.contains("\"progression\""));
}

#[test]
fn test_character_summary_with_trait() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "protagonist_name": "당가인"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": [
                {
                    "id": "sword_master",
                    "name": "검호",
                    "description": "검의 달인"
                }
            ]
        }
    }"#;
    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let mut state = new_game_from_content(123, &index).expect("test game should start");
    state.trait_id = Some("sword_master".to_string());

    let page = scene_page_from_content(&state, &index).expect("build scene page should succeed");
    assert!(page.character_summary.is_some());
    let summary = page.character_summary.as_ref().unwrap();
    assert_eq!(summary.name, "당가인");
    assert_eq!(summary.title_label, Some("검호".to_string()));
    assert_eq!(summary.title_description, Some("검의 달인".to_string()));

    let serialized = serde_json::to_string(&page).unwrap();
    assert!(serialized.contains("\"title_label\":\"검호\""));
    assert!(serialized.contains("\"title_description\":\"검의 달인\""));
}

#[test]
fn test_old_save_compat() {
    let old_save_json = r#"{
        "schema_version": 1,
        "state": {
            "seed": 123,
            "turn": 4,
            "location_id": "dev_desk",
            "disaster_type": "fire",
            "danger": 2,
            "player": {
                "health": 85,
                "sanity": 90,
                "battery": 75,
                "hunger": 10,
                "thirst": 5,
                "abilities": {
                    "logic": 3,
                    "empathy": 2,
                    "volition": 2,
                    "composure": 2,
                    "interface": 2,
                    "physical": 3
                }
            },
            "inventory": ["clue_item"],
            "flags": ["server_room_entered"],
            "clues": [],
            "seen_encounters": [],
            "unlocked_achievements": [],
            "history": []
        }
    }"#;

    let envelope: SaveEnvelope =
        serde_json::from_str(old_save_json).expect("should deserialize old save envelope");
    let state = load_state(&envelope).expect("should load old save state successfully");

    assert_eq!(state.seed, 123);
    assert_eq!(state.trait_id, None);
    assert_eq!(state.experience, 0);
}

#[test]
fn leveling_points_train_without_advancing_turn_and_respect_cap() {
    let mut value: serde_json::Value =
        serde_json::from_str(CONTENT_BUNDLE).expect("fixture bundle should parse");
    value["runtime"] = json!({
        "runtime_mode": "content",
        "world_id": "office",
        "storypack_id": "office",
        "default_location": "dev_desk",
        "leveling": {"thresholds": [10, 20]}
    });
    let bundle_json = serde_json::to_string(&value).expect("bundle should serialize");
    let bundle = load_content_bundle(&bundle_json).expect("leveling bundle should load");
    let index = index_content_bundle(&bundle).expect("leveling bundle should index");
    let mut state = new_game_from_content(123, &index).expect("game should start");
    state.experience = 10;
    let page = scene_page_from_content(&state, &index).expect("page should render");
    assert_eq!(page.character_summary.unwrap().stat_points, 1);

    let result = apply_action_from_content(&state, &index, "train:composure")
        .expect("training should resolve");
    assert_eq!(result.state.turn, state.turn);
    assert_eq!(result.state.danger, state.danger);
    assert_eq!(result.state.player.abilities["composure"], 3);
    assert_eq!(result.state.spent_stat_points, 1);
    assert_eq!(result.logs, vec!["+ 평정 수련 1"]);

    let mut capped = result.state.clone();
    capped.experience = 20;
    capped.player.abilities.insert("logic".to_string(), 5);
    assert!(apply_action_from_content(&capped, &index, "train:logic").is_err());
}

#[test]
fn insights_add_once_and_raise_check_total_without_changing_dice() {
    let mut value: serde_json::Value =
        serde_json::from_str(CONTENT_BUNDLE).expect("fixture bundle should parse");
    value["content"]["insights"] = json!([{
        "id": "steady_breath",
        "name": "고른 호흡",
        "description": "흔들릴수록 호흡을 세어 판정의 바닥을 붙든다.",
        "check_bonus": {"ability": "logic", "bonus": 1}
    }]);
    let encounters = value["content"]["encounters"]
        .as_array_mut()
        .expect("encounters should be an array");
    let messenger = encounters
        .iter_mut()
        .find(|encounter| encounter["id"] == "ex_employee_messenger")
        .expect("messenger encounter should exist");
    messenger["choices"][0]["outcome"]["add_insights"] = json!(["steady_breath"]);

    let bundle_json = serde_json::to_string(&value).expect("bundle should serialize");
    let bundle = load_content_bundle(&bundle_json).expect("insight bundle should load");
    let index = index_content_bundle(&bundle).expect("insight bundle should index");
    let state = new_game_from_content(123, &index).expect("game should start");
    let baseline = escape_core::resolve_ability_check(&state, "logic", 9);
    let mut gifted = state.clone();
    gifted.insights.push("steady_breath".to_string());
    let boosted = escape_core::resolve_ability_check_with_content(&gifted, &index, "logic", 9);
    assert_eq!(baseline.dice, boosted.dice);
    assert_eq!(boosted.insight_bonus, 1);
    assert_eq!(boosted.total, baseline.total + 1);

    let result = apply_action_from_content(&state, &index, "choice:check_message")
        .expect("insight outcome should resolve");
    assert_eq!(result.state.insights, vec!["steady_breath"]);
    assert!(result.logs.iter().any(|line| line == "+ 기연: 고른 호흡"));
    let page = scene_page_from_content(&result.state, &index).expect("page should render");
    assert_eq!(page.insights[0].effect_text, "논리 판정 +1");
}

#[test]
fn test_ability_check_success_percent() {
    // need <= 2
    assert_eq!(ability_check_success_percent(0, 2), 100.0);
    assert_eq!(ability_check_success_percent(10, 5), 100.0);

    // need > 12
    assert_eq!(ability_check_success_percent(0, 13), 0.0);

    // need = 7
    // P(2d6 >= 7) = 21 / 36 = 58.333... -> 58.3%
    assert_eq!(ability_check_success_percent(0, 7), 58.3);

    // need = 12
    // P(2d6 >= 12) = 1 / 36 = 2.777... -> 2.8%
    assert_eq!(ability_check_success_percent(0, 12), 2.8);

    // need = 3
    // P(2d6 >= 3) = 35 / 36 = 97.222... -> 97.2%
    assert_eq!(ability_check_success_percent(0, 3), 97.2);
}

#[test]
fn test_delta_logs_and_trait_change() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [
                {
                    "id": "wood_sword",
                    "name": "목검",
                    "usable": false,
                    "use_effects": {}
                }
            ],
            "encounters": [
                {
                    "id": "test_encounter",
                    "weight": 1,
                    "conditions": {
                        "locations": ["dev_desk"]
                    },
                    "title": "테스트",
                    "body": "테스트 바디",
                    "choices": [
                        {
                            "id": "test_choice",
                            "label": "선택",
                            "cost": {},
                            "outcome": {
                                "log": "기본 서사 로그.",
                                "resources": {
                                    "health": 10,
                                    "sanity": -5
                                },
                                "add_items": ["wood_sword"],
                                "set_trait": "sword_master",
                                "experience": 15
                            }
                        }
                    ]
                }
            ],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": [
                {
                    "id": "sword_master",
                    "name": "검호",
                    "description": "검의 달인"
                },
                {
                    "id": "beginner",
                    "name": "초심자",
                    "description": "초심자"
                }
            ]
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let mut state = new_game_from_content(123, &index).expect("test game should start");

    // Set initial values
    state.trait_id = Some("beginner".to_string());
    state.player.health = 80;
    state.player.sanity = 80;

    let result = apply_action_from_content(&state, &index, "choice:test_choice")
        .expect("action should resolve");

    assert_eq!(
        result.logs,
        vec![
            "기본 서사 로그.".to_string(),
            "+ 체력 10".to_string(),
            "- 정신력 5".to_string(),
            "+ 목검".to_string(),
            "- 특성: 초심자".to_string(),
            "+ 특성: 검호".to_string(),
            "+ 경험 15".to_string(),
        ]
    );

    assert_eq!(result.state.player.health, 90);
    assert_eq!(result.state.player.sanity, 75);
    assert_eq!(result.state.trait_id, Some("sword_master".to_string()));
    assert_eq!(result.state.experience, 15);
}

#[test]
fn test_min_experience_condition_matching() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [
                {
                    "id": "test_encounter",
                    "weight": 1,
                    "conditions": {
                        "locations": ["dev_desk"],
                        "min_experience": 10
                    },
                    "title": "테스트",
                    "body": "테스트 바디",
                    "choices": [
                        {
                            "id": "test_choice",
                            "label": "선택",
                            "cost": {},
                            "outcome": {
                                "log": "완료"
                            }
                        }
                    ]
                }
            ],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let mut state = new_game_from_content(123, &index).expect("test game should start");

    state.experience = 0;
    let view = turn_view_from_content(&state, &index).expect("should render turn view");
    assert_eq!(view.encounter_id, None);

    state.experience = 10;
    let view = turn_view_from_content(&state, &index).expect("should render turn view");
    assert_eq!(view.encounter_id, Some("test_encounter".to_string()));
}

#[test]
fn test_experience_delta_negative_cap() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [
                {
                    "id": "test_encounter",
                    "weight": 1,
                    "conditions": {
                        "locations": ["dev_desk"]
                    },
                    "title": "테스트",
                    "body": "테스트 바디",
                    "choices": [
                        {
                            "id": "test_choice",
                            "label": "선택",
                            "cost": {},
                            "outcome": {
                                "log": "완료",
                                "experience": -50
                            }
                        }
                    ]
                }
            ],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let mut state = new_game_from_content(123, &index).expect("test game should start");
    state.experience = 10;

    let result = apply_action_from_content(&state, &index, "choice:test_choice")
        .expect("action should resolve");

    assert_eq!(result.state.experience, 0);
    assert_eq!(result.logs[1], "- 경험 50");
}

#[test]
fn test_progression_metadata_visibility() {
    let test_bundle_json_with_prog = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "progression": {
                "experience_target": 100,
                "label": "단계"
            }
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json_with_prog).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let state = new_game_from_content(123, &index).expect("test game should start");
    let page = scene_page_from_content(&state, &index).expect("build scene page should succeed");

    assert!(page.progression.is_some());
    let prog = page.progression.as_ref().unwrap();
    assert_eq!(prog.experience, 0);
    assert_eq!(prog.target, 100);
    assert_eq!(prog.label, "단계");

    let serialized = serde_json::to_string(&page).unwrap();
    assert!(
        serialized.contains("\"progression\":{\"experience\":0,\"target\":100,\"label\":\"단계\"}")
    );
}

#[test]
fn content_backed_scene_page_carries_content_labels() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [
                {
                    "id": "iron_sword",
                    "name": "철검",
                    "description": "평범한 철검이다."
                }
            ],
            "encounters": [],
            "endings": [],
            "achievements": [
                {
                    "id": "first_kill",
                    "name": "첫 번째 승리",
                    "description": "첫 전투에서 이겼다."
                }
            ],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let mut state = new_game_from_content(123, &index).expect("test game should start");

    // empty case
    let page_empty =
        scene_page_from_content(&state, &index).expect("build scene page should succeed");
    assert!(page_empty.content_labels.is_none());

    // inventory & achievement populated
    state.inventory.push("iron_sword".to_string());
    state.unlocked_achievements.push("first_kill".to_string());
    state.inventory.push("missing_item".to_string());

    let page = scene_page_from_content(&state, &index).expect("build scene page should succeed");
    assert!(page.content_labels.is_some());
    let labels = page.content_labels.as_ref().unwrap();

    assert_eq!(labels.items.len(), 1);
    assert_eq!(labels.items[0].id, "iron_sword");
    assert_eq!(labels.items[0].label, "철검");

    assert_eq!(labels.achievements.len(), 1);
    assert_eq!(labels.achievements[0].id, "first_kill");
    assert_eq!(labels.achievements[0].label, "첫 번째 승리");

    let serialized = serde_json::to_string(&page).unwrap();
    assert!(serialized.contains("\"content_labels\":{"));
    assert!(serialized.contains("\"id\":\"iron_sword\",\"label\":\"철검\""));
}

#[test]
fn content_backed_scene_page_content_labels_includes_achievement_unlocked_this_turn() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [
                {
                    "id": "trigger_encounter",
                    "title": "발동 인카운터",
                    "body": "발동 바디",
                    "choices": [
                        {
                            "id": "earn_flag",
                            "label": "플래그를 얻는다",
                            "outcome": { "add_flags": ["earned_it"] }
                        }
                    ]
                }
            ],
            "endings": [],
            "achievements": [
                {
                    "id": "first_kill",
                    "name": "첫 번째 승리",
                    "description": "첫 전투에서 이겼다.",
                    "conditions": { "required_flags": ["earned_it"] }
                }
            ],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let state = new_game_from_content(123, &index).expect("test game should start");

    let before_page =
        scene_page_from_content(&state, &index).expect("build scene page should succeed");
    assert!(before_page.content_labels.is_none());

    let action_result =
        apply_action_from_content(&state, &index, "choice:earn_flag").expect("action should apply");
    assert_eq!(
        action_result.newly_unlocked_achievements,
        vec!["first_kill".to_string()]
    );

    let after_page = scene_page_from_content(&action_result.state, &index)
        .expect("build scene page should succeed");
    let labels = after_page
        .content_labels
        .as_ref()
        .expect("content_labels should be populated once an achievement unlocks");
    assert_eq!(labels.achievements.len(), 1);
    assert_eq!(labels.achievements[0].id, "first_kill");
    assert_eq!(labels.achievements[0].label, "첫 번째 승리");
}

#[test]
fn test_check_resolution_lifecycle_and_regression() {
    let test_bundle_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": {
            "schema_version": 1,
            "source": "test",
            "counts": {}
        },
        "content": {
            "locations": [
                {
                    "id": "dev_desk",
                    "name": "내 자리",
                    "description": "내 개발 자리.",
                    "connections": ["other_desk"]
                },
                {
                    "id": "other_desk",
                    "name": "다른 자리",
                    "description": "다른 개발 자리.",
                    "connections": []
                }
            ],
            "items": [],
            "encounters": [
                {
                    "id": "test_encounter",
                    "title": "테스트 인카운터",
                    "body": "테스트 바디",
                    "choices": [
                        {
                            "id": "checked_choice",
                            "label": "체크 선택지",
                            "check": {
                                "ability": "logic",
                                "difficulty": 9,
                                "success": { "log": "성공!" },
                                "failure": { "log": "실패!" }
                            }
                        },
                        {
                            "id": "normal_choice",
                            "label": "일반 선택지",
                            "outcome": { "log": "일반 선택" }
                        }
                    ]
                }
            ],
            "endings": [],
            "achievements": [],
            "secrets": [],
            "traits": []
        }
    }"#;

    let bundle = load_content_bundle(test_bundle_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let state = new_game_from_content(123, &index).expect("test game should start");

    // 1. Regression test for hashing (same seed/turn/ability/difficulty => same dice)
    let res = escape_core::resolve_ability_check(&state, "logic", 9);
    assert_eq!(res.dice, (1, 1));
    assert_eq!(res.ability_value, 2);
    assert_eq!(res.total, 4);
    assert_eq!(res.success, false);

    // 2. Lifecycle: not set at start
    assert!(state.last_check.is_none());

    // 3. Set on check resolution
    let action_res = apply_action_from_content(&state, &index, "choice:checked_choice").unwrap();
    let next_state = action_res.state;

    assert!(next_state.last_check.is_some());
    let last_check = next_state.last_check.as_ref().unwrap();
    assert_eq!(last_check.dice, (1, 1));
    assert_eq!(last_check.ability_id, "logic");
    assert_eq!(last_check.success, false);

    // ScenePage check_result mapping check
    let page = scene_page_from_content(&next_state, &index).unwrap();
    assert!(page.check_result.is_some());
    assert_eq!(page.check_result.unwrap().dice, (1, 1));

    // 4. Cleared on next action (movement)
    let move_res = apply_action_from_content(&next_state, &index, "move:other_desk").unwrap();
    assert!(move_res.state.last_check.is_none());

    // 5. absent-field save loads
    let save_json_no_check = r#"{
        "schema_version": 1,
        "state": {
            "seed": 123,
            "turn": 0,
            "location_id": "dev_desk",
            "disaster_type": "unknown_isolation",
            "danger": 0,
            "player": {
                "health": 100,
                "sanity": 100,
                "battery": 100,
                "hunger": 0,
                "thirst": 0,
                "abilities": {}
            },
            "inventory": [],
            "flags": [],
            "clues": [],
            "seen_encounters": [],
            "unlocked_achievements": [],
            "history": []
        }
    }"#;
    let envelope: escape_core::SaveEnvelope = serde_json::from_str(save_json_no_check).unwrap();
    assert!(envelope.state.last_check.is_none());
}

#[test]
fn test_collapse_gate_lifecycle_and_validation() {
    // 1. validation test: invalid resource_id
    let invalid_res_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "collapse": {
                "encounter_id": "wuxia_collapse_gate",
                "resource_id": "sanity",
                "used_flag": "second_wind_used"
            }
        },
        "manifest": { "schema_version": 1, "source": "test", "counts": {} },
        "content": {
            "locations": [{"id": "dev_desk", "name": "내 자리", "description": "내 개발 자리.", "connections": []}],
            "items": [],
            "encounters": [{"id": "wuxia_collapse_gate", "title": "붕괴", "body": "붕괴", "choices": []}],
            "endings": [], "achievements": [], "secrets": [], "traits": []
        }
    }"#;
    let bundle = load_content_bundle(invalid_res_json).unwrap();
    let err = index_content_bundle(&bundle).unwrap_err();
    assert!(format!("{err:?}").contains("unsupported collapse resource_id"));

    // 2. validation test: empty used_flag
    let empty_flag_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "collapse": {
                "encounter_id": "wuxia_collapse_gate",
                "resource_id": "health",
                "used_flag": ""
            }
        },
        "manifest": { "schema_version": 1, "source": "test", "counts": {} },
        "content": {
            "locations": [{"id": "dev_desk", "name": "내 자리", "description": "내 개발 자리.", "connections": []}],
            "items": [],
            "encounters": [{"id": "wuxia_collapse_gate", "title": "붕괴", "body": "붕괴", "choices": []}],
            "endings": [], "achievements": [], "secrets": [], "traits": []
        }
    }"#;
    let bundle = load_content_bundle(empty_flag_json).unwrap();
    let err = index_content_bundle(&bundle).unwrap_err();
    assert!(format!("{err:?}").contains("collapse used_flag cannot be empty"));

    // 3. validation test: missing collapse encounter_id
    let missing_enc_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "collapse": {
                "encounter_id": "missing_collapse_gate",
                "resource_id": "health",
                "used_flag": "second_wind_used"
            }
        },
        "manifest": { "schema_version": 1, "source": "test", "counts": {} },
        "content": {
            "locations": [{"id": "dev_desk", "name": "내 자리", "description": "내 개발 자리.", "connections": []}],
            "items": [],
            "encounters": [{"id": "wuxia_collapse_gate", "title": "붕괴", "body": "붕괴", "choices": []}],
            "endings": [], "achievements": [], "secrets": [], "traits": []
        }
    }"#;
    let bundle = load_content_bundle(missing_enc_json).unwrap();
    let err = index_content_bundle(&bundle).unwrap_err();
    assert!(format!("{err:?}").contains("missing_collapse_gate' not found in encounters"));

    // 4. normal behavior: collapse trigger
    let valid_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk",
            "collapse": {
                "encounter_id": "wuxia_collapse_gate",
                "resource_id": "health",
                "used_flag": "second_wind_used"
            }
        },
        "manifest": { "schema_version": 1, "source": "test", "counts": {} },
        "content": {
            "locations": [{"id": "dev_desk", "name": "내 자리", "description": "내 개발 자리.", "connections": []}],
            "items": [],
            "encounters": [
                {
                    "id": "wuxia_collapse_gate",
                    "title": "붕괴 게이트",
                    "body": "안식을 취할 것인가?",
                    "choices": [
                        {
                            "id": "revive",
                            "label": "기사회생",
                            "outcome": {
                                "resources": { "health": 40 },
                                "add_flags": ["second_wind_used"]
                            }
                        },
                        {
                            "id": "accept_death",
                            "label": "안식",
                            "outcome": {
                                "add_flags": ["accept_final_rest", "second_wind_used"]
                            }
                        }
                    ]
                },
                {
                    "id": "normal_enc",
                    "title": "일반 인카운터",
                    "body": "일반 바디",
                    "choices": [
                        {
                            "id": "lose_health",
                            "label": "체력 감소",
                            "outcome": {
                                "resources": { "health": -120 }
                            }
                        }
                    ]
                }
            ],
            "endings": [
                {
                    "id": "death_ending",
                    "kind": "death",
                    "name": "사망 엔딩",
                    "text": "당신은 죽었습니다.",
                    "priority": 100,
                    "conditions": {
                        "required_flags": ["accept_final_rest"]
                    }
                }
            ],
            "achievements": [], "secrets": [], "traits": []
        }
    }"#;

    let bundle = load_content_bundle(valid_json).expect("test bundle should load");
    let index = index_content_bundle(&bundle).expect("test bundle should index");
    let state = new_game_from_content(123, &index).expect("test game should start");

    assert_eq!(state.player.health, 100);

    let action_res = apply_action_from_content(&state, &index, "choice:lose_health").unwrap();
    let next_state = action_res.state;

    assert!(next_state.player.health <= 0);

    let turn_view = escape_core::turn_view_from_content(&next_state, &index).unwrap();
    assert_eq!(
        turn_view.encounter_id.as_deref(),
        Some("wuxia_collapse_gate")
    );
    assert!(turn_view.ending_id.is_none());

    let revive_res = apply_action_from_content(&next_state, &index, "choice:revive").unwrap();
    let revived_state = revive_res.state;
    assert_eq!(revived_state.player.health, 40);
    assert!(revived_state.flags.iter().any(|f| f == "second_wind_used"));

    let mut recollapse_state = revived_state.clone();
    recollapse_state.player.health = -10;
    let recollapse_view = escape_core::turn_view_from_content(&recollapse_state, &index).unwrap();
    assert_ne!(
        recollapse_view.encounter_id.as_deref(),
        Some("wuxia_collapse_gate")
    );

    let accept_res = apply_action_from_content(&next_state, &index, "choice:accept_death").unwrap();
    let accepted_state = accept_res.state;
    assert!(accepted_state
        .flags
        .iter()
        .any(|f| f == "accept_final_rest"));

    let ending_view = escape_core::turn_view_from_content(&accepted_state, &index).unwrap();
    assert_eq!(ending_view.ending_id.as_deref(), Some("death_ending"));
}

#[test]
fn test_check_ability_id_validation_rejects_unknown_ability() {
    let invalid_ability_json = r#"{
        "schema_version": 1,
        "kind": "tui_adv.content_bundle",
        "source": "test",
        "runtime": {
            "runtime_mode": "content",
            "world_id": "test_world",
            "storypack_id": "test_pack",
            "default_location": "dev_desk"
        },
        "manifest": { "schema_version": 1, "source": "test", "counts": {} },
        "content": {
            "locations": [{"id": "dev_desk", "name": "내 자리", "description": "내 개발 자리.", "connections": []}],
            "items": [],
            "encounters": [
                {
                    "id": "bad_check_encounter",
                    "title": "잘못된 판정",
                    "body": "잘못된 능력치 판정.",
                    "choices": [
                        {
                            "id": "bad_check_choice",
                            "label": "잘못된 판정 선택",
                            "outcome": {},
                            "check": {
                                "ability": "dexterity",
                                "difficulty": 8,
                                "success": {},
                                "failure": {}
                            }
                        }
                    ]
                }
            ],
            "endings": [], "achievements": [], "secrets": [], "traits": []
        }
    }"#;
    let bundle = load_content_bundle(invalid_ability_json).unwrap();
    let err = index_content_bundle(&bundle).unwrap_err();
    let message = format!("{err:?}");
    assert!(message.contains("unknown check ability id: 'dexterity'"));
    assert!(message.contains("logic"));
    assert!(message.contains("empathy"));
    assert!(message.contains("volition"));
    assert!(message.contains("composure"));
    assert!(message.contains("interface"));
    assert!(message.contains("physical"));
}
