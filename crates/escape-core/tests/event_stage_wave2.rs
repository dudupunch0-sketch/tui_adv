use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
    scene_page_from_content,
};

const WUXIA_PREVIEW_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

const WAVE2_ENCOUNTERS: &[&str] = &[
    "wuxia_mumyeong_first_confrontation",
    "wuxia_mumyeong_copy_style_reveal",
    "wuxia_mumyeong_reads_orthodox_style",
    "wuxia_mumyeong_midgame_reunion",
    "wuxia_boss_first_appearance",
    "wuxia_mumyeong_request_for_aid",
    "wuxia_mumyeong_awakening",
    "wuxia_qingliu_attack_after_war",
    "wuxia_mumyeong_destroys_orthodox_sect",
    "wuxia_boss_recruits_mumyeong",
    "wuxia_mumyeong_departure_truth_summary",
    "wuxia_cheonggi_record_writing_sense",
    "wuxia_cheonoe_pyeonrin_first_reward",
    "wuxia_cheonoe_pyeonrin_second_reward",
    "wuxia_seoharin_empty_place",
    "wuxia_seoharin_left_meal",
];

fn staged_entry_state(
    index: &escape_core::ContentIndex,
    encounter_id: &str,
) -> escape_core::GameState {
    let mut state = new_game_from_content_at(17, index, "cheongryu_outer_courtyard").unwrap();
    state.active_event_id = Some(encounter_id.to_string());
    state
}

#[test]
fn wave2_entries_expose_ordered_streams_and_preserve_choice_ids() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();

    for encounter_id in WAVE2_ENCOUNTERS {
        let state = staged_entry_state(&index, encounter_id);
        let encounter = index.encounter(encounter_id).unwrap();
        let event = encounter
            .event
            .as_ref()
            .expect("wave-2 encounter must be staged");
        assert_eq!(
            event.stages.first().unwrap().kind,
            "story",
            "{encounter_id}"
        );
        assert_eq!(
            event.stages.get(1).unwrap().kind,
            "choice",
            "{encounter_id}"
        );
        assert!(
            event
                .stages
                .iter()
                .flat_map(|stage| stage.blocks.iter())
                .any(|block| block.kind == "illustration"),
            "{encounter_id}"
        );

        let opening = scene_page_from_content(&state, &index).unwrap();
        assert!(!opening.content_stream.is_empty(), "{encounter_id}");
        let choice_state = apply_action_from_content(&state, &index, "event:continue")
            .unwrap()
            .state;
        let choice_page = scene_page_from_content(&choice_state, &index).unwrap();
        let stream_choice = choice_page
            .content_stream
            .iter()
            .find(|item| item.kind == "choice")
            .expect("choice stage should expose an ordered choice item");
        let mut staged_ids: Vec<_> = stream_choice
            .actions
            .iter()
            .map(|action| action.id.strip_prefix("choice:").unwrap())
            .collect();
        let mut legacy_ids: Vec<_> = encounter
            .choices
            .iter()
            .map(|choice| choice.id.as_str())
            .collect();
        staged_ids.sort_unstable();
        legacy_ids.sort_unstable();
        assert_eq!(staged_ids, legacy_ids, "{encounter_id}");
    }
}

#[test]
fn cheonggi_record_writing_sense_uses_document_blocks() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let state = staged_entry_state(&index, "wuxia_cheonggi_record_writing_sense");
    let opening = scene_page_from_content(&state, &index).unwrap();
    assert!(
        opening
            .content_stream
            .iter()
            .any(|item| item.kind == "document"),
        "record-writing event must expose a document block"
    );
}

#[test]
fn wave2_has_no_checked_choices_to_branch() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    assert!(WAVE2_ENCOUNTERS.iter().all(|encounter_id| {
        index
            .encounter(encounter_id)
            .unwrap()
            .choices
            .iter()
            .all(|choice| choice.check.is_none())
    }));
}
