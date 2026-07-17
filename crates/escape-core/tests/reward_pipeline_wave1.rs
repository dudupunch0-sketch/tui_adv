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

const CANONICAL_ROWS: [(&str, &str, &str, &str, &str, &str); 29] = [
    (
        "wuxia_cheongryu_first_night_shelter",
        "first_night_stay_guest",
        "title",
        "wuxia_title_guest_of_cheongryu",
        "immediate",
        "서하린이 정한 자리에 머물기",
    ),
    (
        "wuxia_cheongryu_first_night_shelter",
        "first_night_separate_boundary",
        "title",
        "wuxia_title_not_yet_disciple",
        "immediate",
        "방을 따로 달라고 하기",
    ),
    (
        "wuxia_cheongryu_first_night_shelter",
        "first_night_why_fidelity",
        "insight",
        "wuxia_insight_measure_fidelity",
        "immediate",
        "왜 자신을 받았는지 묻기",
    ),
    (
        "wuxia_cheongryu_first_night_shelter",
        "first_night_leave_name",
        "title",
        "wuxia_title_keeper_of_returning_name",
        "immediate",
        "내일 바로 나가겠다고 말하기",
    ),
    (
        "wuxia_cheongryu_first_breathing_lesson",
        "breath_copy_first_current",
        "insight",
        "wuxia_insight_first_current_breath",
        "check-success",
        "서하린의 호흡을 그대로 따라 하기",
    ),
    (
        "wuxia_cheongryu_first_breathing_lesson",
        "breath_own_pulse",
        "skill",
        "wuxia_skill_match_the_pulse",
        "check-success",
        "자신의 방식으로 관찰해 따라 하기",
    ),
    (
        "wuxia_cheongryu_first_breathing_lesson",
        "breath_ask_trust",
        "relationship",
        "relationship_person_seoharin_affection",
        "immediate",
        "왜 힘을 빼야 하는지 묻기",
    ),
    (
        "wuxia_cheongryu_first_breathing_lesson",
        "breath_stop_flow",
        "skill",
        "wuxia_skill_fallen_leaf_flow_step",
        "immediate",
        "아픈 척하지 않고 중단하기",
    ),
    (
        "wuxia_cheongryu_training_first_failure",
        "failure_no_excuse",
        "insight",
        "wuxia_insight_recording_defeat",
        "immediate",
        "변명하지 않고 복기하기",
    ),
    (
        "wuxia_cheongryu_training_first_failure",
        "failure_explain_gap",
        "skill",
        "wuxia_skill_record_the_gap",
        "immediate",
        "방금 본 동작을 설명하기",
    ),
    (
        "wuxia_cheongryu_training_first_failure",
        "failure_rematch_blade",
        "skill",
        "wuxia_skill_turning_blade",
        "check-success",
        "서하린에게 재대련 요청하기",
    ),
    (
        "wuxia_cheongryu_training_first_failure",
        "failure_escape_chore",
        "title",
        "wuxia_title_not_yet_disciple",
        "immediate",
        "다음 잡일로 도망가기",
    ),
    (
        "wuxia_cheongryu_medicine_errand",
        "medicine_alone_pouch",
        "item",
        "wuxia_item_modern_first_aid_pouch",
        "pending",
        "시장까지 혼자 다녀오기",
    ),
    (
        "wuxia_cheongryu_medicine_errand",
        "medicine_together_affection",
        "relationship",
        "relationship_person_seoharin_affection",
        "immediate",
        "서하린과 함께 가기",
    ),
    (
        "wuxia_cheongryu_medicine_errand",
        "medicine_badge_title",
        "title",
        "wuxia_title_badge_bearer",
        "immediate",
        "사원증·출근 물건을 교환 제안하기",
    ),
    (
        "wuxia_cheongryu_medicine_errand",
        "medicine_empty_ledger",
        "item",
        "wuxia_item_empty_medicine_ledger",
        "pending",
        "빈손으로 돌아와 부족을 알리기",
    ),
    (
        "wuxia_cheongryu_raid_omen",
        "omen_gate_register",
        "item",
        "wuxia_item_wet_gate_register",
        "pending",
        "산문을 직접 확인하기",
    ),
    (
        "wuxia_cheongryu_raid_omen",
        "omen_injured_talisman",
        "pendant",
        "wuxia_pendant_life_talisman",
        "immediate",
        "부상자와 약재를 점검하기",
    ),
    (
        "wuxia_cheongryu_raid_omen",
        "omen_archive_documents",
        "insight",
        "wuxia_insight_sort_like_documents",
        "immediate",
        "폐서고 기록을 찾기",
    ),
    (
        "wuxia_cheongryu_raid_omen",
        "omen_rest_threshold",
        "skill",
        "wuxia_skill_guard_the_threshold",
        "immediate",
        "서하린의 지시에 따라 휴식하기",
    ),
    (
        "wuxia_cheongryu_raid_omen",
        "omen_hidden_two_paths",
        "title",
        "wuxia_title_footprints_of_two_paths",
        "dormant",
        "산문 조사와 기록 조사 조건을 모두 충족",
    ),
    (
        "wuxia_cheongryu_gate_patrol_first_trouble",
        "patrol_follow_thread",
        "item",
        "wuxia_item_red_thread_fragment",
        "check-success",
        "소리를 따라가기",
    ),
    (
        "wuxia_cheongryu_gate_patrol_first_trouble",
        "patrol_report_faction",
        "relationship",
        "relationship_faction_cheongryu_affection",
        "immediate",
        "서하린에게 알리기",
    ),
    (
        "wuxia_cheongryu_gate_patrol_first_trouble",
        "patrol_ignore_retreat",
        "skill",
        "wuxia_skill_two_steps_back",
        "immediate",
        "아무 일 없는 척 순찰 계속하기",
    ),
    (
        "wuxia_cheongryu_gate_patrol_first_trouble",
        "patrol_fake_whistle",
        "item",
        "wuxia_item_cracked_whistle",
        "immediate",
        "가짜 순찰 신호를 남기기",
    ),
    (
        "wuxia_seoharin_hides_training_injury",
        "wrist_ask_read",
        "insight",
        "wuxia_insight_read_the_wrist",
        "immediate",
        "손목을 직접 묻기",
    ),
    (
        "wuxia_seoharin_hides_training_injury",
        "wrist_ointment_handkerchief",
        "pendant",
        "wuxia_pendant_seoharin_handkerchief",
        "pending",
        "약초 연고를 가져오기",
    ),
    (
        "wuxia_seoharin_hides_training_injury",
        "wrist_report_faction",
        "relationship",
        "relationship_faction_cheongryu_affection",
        "immediate",
        "장문인에게 알리기",
    ),
    (
        "wuxia_seoharin_hides_training_injury",
        "wrist_look_away_presence",
        "skill",
        "wuxia_skill_cut_the_presence",
        "immediate",
        "못 본 척하기",
    ),
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
fn wave1_adds_seven_staged_cards_and_all_29_mapping_rows() {
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
    let mut checks = 0;
    let mut pending = 0;
    let mut relationships = 0;
    for (encounter_id, choice_id, kind, reward_id, timing, label) in CANONICAL_ROWS {
        let event = index
            .encounter(encounter_id)
            .unwrap()
            .event
            .as_ref()
            .unwrap();
        let choice = index
            .encounters()
            .flat_map(|encounter| encounter.choices.iter())
            .find(|choice| choice.id == choice_id)
            .unwrap();
        assert_eq!(choice.label, label, "label for {choice_id}");
        assert!(event
            .stages
            .iter()
            .any(|stage| stage.choices.iter().any(|r| r.id == choice_id)));
        let outcome = match timing {
            "check-success" => {
                checks += 1;
                choice.check.as_ref().unwrap().success.clone()
            }
            _ => choice.outcome.clone(),
        };
        if timing == "pending" {
            pending += 1;
        }
        if kind == "relationship" {
            relationships += 1;
        }
        let has = match kind {
            "skill" => outcome.add_skills.contains(&reward_id.to_string()),
            "title" => outcome.add_titles.contains(&reward_id.to_string()),
            "insight" => outcome.add_insights.contains(&reward_id.to_string()),
            "item" | "pendant" => outcome.add_items.contains(&reward_id.to_string()),
            "relationship" => outcome.relationship_deltas.contains_key(reward_id),
            _ => false,
        };
        if timing == "pending" || timing == "dormant" {
            assert!(!has, "deferred reward must not be granted in {choice_id}");
        } else {
            assert!(has, "missing {kind} {reward_id} in {choice_id}");
        }
        if timing == "pending" {
            assert!(outcome
                .add_flags
                .iter()
                .any(|flag| flag == &format!("reward_pending_{reward_id}")));
        }
        if timing == "dormant" {
            assert!(!choice.outcome.add_titles.contains(&reward_id.to_string()));
            assert!(choice.check.is_none());
            assert!(!choice
                .outcome
                .add_flags
                .iter()
                .any(|flag| flag.starts_with("reward_pending_")));
        }
    }
    assert_eq!(checks, 4);
    assert_eq!(pending, 4);
    assert_eq!(relationships, 4);
    assert!(value["content"]["titles"]
        .as_array()
        .unwrap()
        .iter()
        .any(|title| title["id"] == "wuxia_title_footprints_of_two_paths"));
}

#[test]
fn duplicate_skill_and_title_rewards_are_ignored() {
    let index = index();
    let mut state = active_state(
        &index,
        "wuxia_cheongryu_first_night_shelter",
        &["cheongryu_apprentice_entry_resolved"],
    );
    state.skills.push("wuxia_skill_match_the_pulse".into());
    state.titles.push("wuxia_title_not_yet_disciple".into());
    let choice = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result =
        apply_action_from_content(&choice, &index, "choice:first_night_stay_guest").unwrap();
    assert_eq!(
        result
            .state
            .skills
            .iter()
            .filter(|id| id.as_str() == "wuxia_skill_match_the_pulse")
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
        &["cheongryu_apprentice_entry_resolved"],
    );
    let choice = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    let result =
        apply_action_from_content(&choice, &index, "choice:patrol_report_faction").unwrap();
    assert_eq!(
        result.state.relationships["relationship_faction_cheongryu_affection"],
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
                "wuxia_cheongryu_gate_patrol_first_trouble",
                &["cheongryu_apprentice_entry_resolved"],
            );
            s.seed = seed;
            s
        };
        let choice = apply_action_from_content(&state, &index, "event:continue")
            .unwrap()
            .state;
        let result =
            apply_action_from_content(&choice, &index, "choice:patrol_follow_thread").unwrap();
        let success = result
            .state
            .inventory
            .iter()
            .any(|id| id == "wuxia_item_red_thread_fragment");
        let failure = result
            .state
            .flags
            .iter()
            .any(|flag| flag == "patrol_follow_thread_check_failed");
        assert_ne!(success, failure);
        saw_success |= success;
        saw_failure |= failure;
        if saw_success && saw_failure {
            break;
        }
    }
    assert!(saw_success && saw_failure);
}
