use escape_core::{
    apply_action, index_content_bundle, load_content_bundle, load_state, new_game,
    new_game_from_content, new_game_from_content_at, save_state, turn_view, EffectCue,
    NewGameError,
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
