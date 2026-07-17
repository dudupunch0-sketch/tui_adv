//! Reward Pipeline Wave 1 guards: additive state, staged cards, and mapping coverage.
use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
};
use serde_json::Value;

const BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

const NEW_ENCOUNTERS: [&str; 7] = [
    "wuxia_cheongryu_first_night_shelter",
    "wuxia_cheongryu_first_breathing_lesson",
    "wuxia_cheongryu_training_first_failure",
    "wuxia_cheongryu_medicine_errand",
    "wuxia_cheongryu_raid_omen",
    "wuxia_cheongryu_gate_patrol_first_trouble",
    "wuxia_seoharin_hides_training_injury",
];

const REWARD_IDS: [&str; 24] = [
    "match_the_pulse",
    "fallen_leaf_flow_step",
    "record_the_gap",
    "turning_blade",
    "guard_the_threshold",
    "two_steps_back",
    "cut_the_presence",
    "not_yet_disciple",
    "guest_of_cheongryu",
    "keeper_of_returning_name",
    "badge_bearer",
    "footprints_of_two_paths",
    "first_current_breath",
    "recording_defeat",
    "measure_fidelity",
    "sort_like_documents",
    "read_the_wrist",
    "modern_first_aid_pouch",
    "empty_medicine_ledger",
    "cracked_whistle",
    "red_thread_fragment",
    "wet_gate_register",
    "life_talisman",
    "seoharin_handkerchief",
];

fn index() -> escape_core::ContentIndex {
    index_content_bundle(&load_content_bundle(BUNDLE).unwrap()).unwrap()
}

fn active_state(
    index: &escape_core::ContentIndex,
    encounter_id: &str,
    flags: &[&str],
) -> escape_core::GameState {
    let mut state = new_game_from_content_at(19, index, "cheongryu_outer_courtyard").unwrap();
    state.active_event_id = Some(encounter_id.to_string());
    state.flags = flags.iter().map(|flag| (*flag).to_string()).collect();
    state
}

#[test]
fn wave1_adds_seven_staged_cards_and_all_30_mapping_rows() {
    let value: Value = serde_json::from_str(BUNDLE).unwrap();
    let index = index();
    assert_eq!(index.encounters_len(), 51);
    for id in NEW_ENCOUNTERS {
        let event = index.encounter(id).unwrap().event.as_ref().unwrap();
        assert_eq!(event.stages[0].kind, "story");
        assert_eq!(event.stages[1].kind, "choice");
        assert!(event.stages.iter().any(|stage| stage
            .blocks
            .iter()
            .any(|block| block.kind == "illustration")));
    }
    assert_eq!(value["content"]["skills"].as_array().unwrap().len(), 7);
    assert_eq!(value["content"]["titles"].as_array().unwrap().len(), 5);
    for id in REWARD_IDS {
        assert!(value.to_string().contains(id), "missing reward {id}");
    }
    let mapping_markers = value.to_string().matches("reward_pending_").count()
        + value.to_string().matches("add_skills").count()
        + value.to_string().matches("add_titles").count()
        + value.to_string().matches("add_insights").count()
        + value.to_string().matches("add_items").count()
        + value.to_string().matches("relationship_deltas").count();
    assert!(
        mapping_markers >= 30,
        "expected 30 mapping markers, got {mapping_markers}"
    );
}

#[test]
fn duplicate_skill_and_title_rewards_are_ignored() {
    let index = index();
    let mut state = active_state(
        &index,
        "wuxia_cheongryu_first_night_shelter",
        &["cheongryu_apprentice_entry_resolved"],
    );
    state.skills.push("match_the_pulse".into());
    state.titles.push("not_yet_disciple".into());
    let choice = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result =
        apply_action_from_content(&choice, &index, "choice:listen_to_the_roof_rain").unwrap();
    assert_eq!(
        result
            .state
            .skills
            .iter()
            .filter(|id| id.as_str() == "match_the_pulse")
            .count(),
        1
    );
    assert!(!result.logs.iter().any(|line| line.contains("+ 스킬")));
}

#[test]
fn relationship_deltas_accumulate_without_exposing_values_in_logs() {
    let index = index();
    let state = active_state(
        &index,
        "wuxia_cheongryu_gate_patrol_first_trouble",
        &["wuxia_cheongryu_raid_omen_resolved"],
    );
    let choice = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result = apply_action_from_content(&choice, &index, "choice:call_for_seoharin").unwrap();
    assert_eq!(
        result.state.relationships["relationship_person_seoharin_affection"],
        1
    );
    assert!(result
        .logs
        .iter()
        .any(|line| line.contains("관계 방향 기록")));
    assert!(!result.logs.iter().any(|line| line.contains("= 1")));
}

#[test]
fn check_success_and_failure_mapping_remain_exclusive() {
    let index = index();
    let mut saw_success = false;
    let mut saw_failure = false;
    for seed in 1..=128 {
        let state = {
            let mut s = active_state(
                &index,
                "wuxia_cheongryu_medicine_errand",
                &["wuxia_cheongryu_training_first_failure_resolved"],
            );
            s.seed = seed;
            s
        };
        let choice = apply_action_from_content(&state, &index, "event:continue")
            .unwrap()
            .state;
        let result =
            apply_action_from_content(&choice, &index, "choice:carry_the_medicine_without_turning")
                .unwrap();
        let success = result
            .state
            .flags
            .iter()
            .any(|flag| flag == "carry_the_medicine_without_turning_check_success");
        let failure = result
            .state
            .flags
            .iter()
            .any(|flag| flag == "carry_the_medicine_without_turning_check_failure");
        assert_ne!(success, failure);
        saw_success |= success;
        saw_failure |= failure;
        if saw_success && saw_failure {
            break;
        }
    }
    assert!(saw_success && saw_failure);
}
