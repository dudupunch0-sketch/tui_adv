use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content,
    new_game_from_content_at, scene_page_from_content, ContentIndex, GameState, SceneMode,
};

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

fn content() -> ContentIndex {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    index_content_bundle(&bundle).expect("content bundle should index")
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
