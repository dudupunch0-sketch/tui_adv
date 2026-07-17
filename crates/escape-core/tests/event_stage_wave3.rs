//! Wave 3 staging and collapse-contract regression harness.
use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
    turn_view_from_content,
};

const BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

const WAVE3: [&str; 14] = [
    "wuxia_sado_final_phase_1_price_tag",
    "wuxia_cheonoe_analysis_thread_phase1_bridge",
    "wuxia_sado_final_phase_2_weakpoint_control",
    "wuxia_sado_final_phase_3_outside_calculation",
    "wuxia_sado_battle_loss_route_bridge",
    "wuxia_boss_resolution",
    "wuxia_mumyeong_resolution",
    "wuxia_seoharin_qingliu_resolution",
    "wuxia_seoharin_unsaid_stay",
    "wuxia_cheongirok_resolution",
    "wuxia_black_serpent_aftermath",
    "wuxia_return_modern_commute_scene",
    "wuxia_settlement_stay_scene",
    "wuxia_collapse_gate",
];

#[test]
fn wave3_entries_have_story_choice_results_and_illustration() {
    let bundle = load_content_bundle(BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    for id in WAVE3 {
        let encounter = index
            .encounter(id)
            .unwrap_or_else(|| panic!("missing {id}"));
        let event = encounter
            .event
            .as_ref()
            .expect("Wave 3 entry must be staged");
        assert_eq!(
            event.stages.first().map(|s| s.kind.as_str()),
            Some("story"),
            "{id}"
        );
        assert_eq!(
            event.stages.get(1).map(|s| s.kind.as_str()),
            Some("choice"),
            "{id}"
        );
        let illustrations: Vec<_> = event
            .stages
            .iter()
            .flat_map(|s| s.blocks.iter())
            .filter(|b| b.kind == "illustration")
            .collect();
        assert_eq!(illustrations.len(), 1, "{id}");
        assert!(
            illustrations[0]
                .alt
                .as_deref()
                .is_some_and(|alt| !alt.trim().is_empty()),
            "{id}"
        );
        let choice_stage = &event.stages[1];
        let mut staged: Vec<_> = choice_stage.choices.iter().map(|c| c.id.as_str()).collect();
        let mut legacy: Vec<_> = encounter.choices.iter().map(|c| c.id.as_str()).collect();
        staged.sort_unstable();
        legacy.sort_unstable();
        assert_eq!(staged, legacy, "{id} choice ids");
        assert!(
            encounter.choices.iter().all(|c| c.check.is_none()),
            "{id} must remain unchecked"
        );
        assert!(
            event
                .stages
                .iter()
                .all(|s| s.blocks.iter().all(|b| b.branch.is_none())),
            "{id} branch"
        );
        for choice in &encounter.choices {
            let result_id = format!("{}_result", choice.id);
            let result = event
                .stages
                .iter()
                .find(|s| s.id == result_id)
                .expect("result stage");
            let log = choice.outcome.log.as_deref().unwrap_or_default();
            assert!(
                result
                    .blocks
                    .iter()
                    .filter_map(|b| b.text.as_deref())
                    .any(|t| t.contains(log)),
                "{id}/{} log",
                choice.id
            );
        }
    }
}

#[test]
fn wave3_cheongirok_resolution_uses_document_surface() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let event = index
        .encounter("wuxia_cheongirok_resolution")
        .unwrap()
        .event
        .as_ref()
        .unwrap();
    assert!(event
        .stages
        .iter()
        .flat_map(|s| s.blocks.iter())
        .any(|b| b.kind == "document" || b.kind == "cheongirok"));
    assert!(event
        .stages
        .iter()
        .flat_map(|s| s.blocks.iter())
        .filter(|b| b.kind == "narration")
        .all(|b| b.speaker.as_deref() != Some("천기록")));
}

#[test]
fn collapse_gate_choice_order_is_revive_then_rest() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let event = index
        .encounter("wuxia_collapse_gate")
        .unwrap()
        .event
        .as_ref()
        .unwrap();
    let ids: Vec<_> = event.stages[1]
        .choices
        .iter()
        .map(|c| c.id.as_str())
        .collect();
    assert_eq!(ids, ["wuxia_collapse_revive", "wuxia_collapse_rest"]);
}

#[test]
fn wuxia_preview_has_full_51_event_coverage() {
    let bundle = load_content_bundle(BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    assert_eq!(index.encounters_len(), 51);
    assert_eq!(index.encounters().filter(|e| e.event.is_some()).count(), 51);
}

#[test]
fn collapse_does_not_preempt_an_active_staged_event_at_zero_health() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let mut state = new_game_from_content_at(91, &index, "cheongryu_outer_courtyard").unwrap();
    state.active_event_id = Some("wuxia_mumyeong_first_sighting".into());
    state.event_stage_index = 1;
    state.player.health = 0;
    let view = turn_view_from_content(&state, &index).unwrap();
    assert_eq!(
        view.encounter_id.as_deref(),
        Some("wuxia_mumyeong_first_sighting")
    );
    assert!(view.ending_id.is_none());
}

#[test]
fn collapse_revive_sets_health_and_used_flag_without_retriggering() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let mut state = new_game_from_content_at(92, &index, "cheongryu_outer_courtyard").unwrap();
    state.player.health = 0;
    let gate = turn_view_from_content(&state, &index).unwrap();
    assert_eq!(gate.encounter_id.as_deref(), Some("wuxia_collapse_gate"));
    let choice_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let revived = apply_action_from_content(&choice_state, &index, "choice:wuxia_collapse_revive")
        .unwrap()
        .state;
    assert_eq!(revived.player.health, 40);
    assert!(revived.flags.iter().any(|f| f == "second_wind_used"));
    let mut spent = revived;
    spent.player.health = 0;
    let next = turn_view_from_content(&spent, &index).unwrap();
    assert_ne!(next.encounter_id.as_deref(), Some("wuxia_collapse_gate"));
}

#[test]
fn collapse_rest_supersedes_result_with_death_ending() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let mut state = new_game_from_content_at(93, &index, "cheongryu_outer_courtyard").unwrap();
    state.player.health = 0;
    let choice_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let accepted = apply_action_from_content(&choice_state, &index, "choice:wuxia_collapse_rest")
        .unwrap()
        .state;
    assert!(accepted.flags.iter().any(|f| f == "accept_final_rest"));
    assert!(accepted.flags.iter().any(|f| f == "second_wind_used"));
    let view = turn_view_from_content(&accepted, &index).unwrap();
    assert_eq!(view.ending_id.as_deref(), Some("wuxia_death_rest"));
}

#[test]
fn black_serpent_aftermath_choice_supersedes_with_final_epilogue_ending() {
    let index = index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap();
    let mut state = new_game_from_content_at(94, &index, "black_serpent_ledger_vault").unwrap();
    state.flags = [
        "boss_resolution_resolved",
        "mumyeong_resolution_resolved",
        "seoharin_qingliu_resolution_resolved",
        "cheongirok_resolution_resolved",
        "final_result_priority_applied_seeded",
        "final_state_routing_seeded",
        "final_combat_result_battle_victory_seeded",
        "wuxia_ending_scene_resolved",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let choice_state = apply_action_from_content(&state, &index, "event:continue")
        .expect("aftermath story should advance to choice")
        .state;
    let result = apply_action_from_content(
        &choice_state,
        &index,
        "choice:mark_broken_serpent_without_erasing_scars",
    )
    .expect("aftermath choice must remain a known action");
    let view = turn_view_from_content(&result.state, &index).unwrap();
    assert_eq!(
        view.ending_id.as_deref(),
        Some("wuxia_final_epilogue_renderer_contract")
    );
}
