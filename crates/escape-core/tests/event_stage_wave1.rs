use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
    scene_page_from_content,
};

const WUXIA_PREVIEW_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

fn staged_entry_state(
    index: &escape_core::ContentIndex,
    encounter_id: &str,
    location_id: &str,
    flags: &[&str],
    seed: u64,
) -> escape_core::GameState {
    let mut state = new_game_from_content_at(seed, index, location_id).unwrap();
    state.active_event_id = Some(encounter_id.to_string());
    state.flags = flags.iter().map(|flag| (*flag).to_string()).collect();
    state
}

#[test]
fn wave1_entries_expose_staged_streams_and_preserve_choice_ids() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let entries = [
        (
            "wuxia_heuksa_bang_first_fight",
            "jianghu_market_street",
            &["wuxia_arrival_hidden"][..],
        ),
        (
            "wuxia_cheongryu_apprentice_entry",
            "cheongryu_outer_courtyard",
            &["seo_harin_rescue_resolved", "taken_under_watch"][..],
        ),
        (
            "wuxia_cheongryu_chore_sparring",
            "cheongryu_outer_courtyard",
            &[
                "cheongryu_apprentice_entry_resolved",
                "cheongryu_trial_started",
                "cheonggi_record_awakened",
                "first_fragment_seen",
            ][..],
        ),
        (
            "wuxia_cheongryu_raid_route_split",
            "cheongryu_outer_courtyard",
            &[
                "cheongryu_apprentice_entry_resolved",
                "cheongryu_trial_started",
                "cheonggi_record_awakened",
                "first_fragment_seen",
                "cheongryu_chore_sparring_resolved",
            ][..],
        ),
        (
            "wuxia_cheongryu_raid_wounded_fallback",
            "cheongryu_outer_courtyard",
            &[
                "cheongryu_raid_route_split_resolved",
                "route_commitment_deferred",
                "wounded_saved_flag",
                "cheongryu_raid_survived",
            ][..],
        ),
        (
            "wuxia_baekdo_medicine_debt",
            "cheongryu_outer_courtyard",
            &["righteous_route_started", "cheongryu_rebuild_thread"][..],
        ),
        (
            "wuxia_black_heaven_escape_price",
            "cheongryu_outer_courtyard",
            &["sapa_route_started", "dowol_debt"][..],
        ),
        (
            "wuxia_heavenly_archive_previous_outsiders",
            "cheongryu_outer_courtyard",
            &["cheonggi_return_route_started", "cheonggi_record_targeted"][..],
        ),
        (
            "wuxia_wounded_shelter_dawn_offers",
            "cheongryu_outer_courtyard",
            &[
                "cheongryu_raid_wounded_fallback_resolved",
                "route_commitment_deferred",
                "deferred_route_reopened",
                "wounded_shelter_stabilized",
            ][..],
        ),
        (
            "wuxia_mumyeong_first_sighting",
            "cheongryu_outer_courtyard",
            &[
                "route_opener_resolved",
                "cheongryu_raid_survived",
                "cheongryu_trial_started",
                "first_fragment_seen",
            ][..],
        ),
    ];

    for (encounter_id, location_id, flags) in entries {
        let state = staged_entry_state(&index, encounter_id, location_id, flags, 17);
        let encounter = index.encounter(encounter_id).unwrap();
        let event = encounter
            .event
            .as_ref()
            .expect("wave-1 encounter must be staged");
        assert_eq!(event.stages[0].kind, "story", "{encounter_id}");
        assert_eq!(event.stages[1].kind, "choice", "{encounter_id}");
        assert!(
            event
                .stages
                .iter()
                .flat_map(|stage| stage.blocks.iter())
                .any(|block| block.kind == "illustration"),
            "{encounter_id}"
        );

        let page = scene_page_from_content(&state, &index).unwrap();
        assert!(!page.content_stream.is_empty(), "{encounter_id}");
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
fn heuksa_checked_result_streams_keep_only_the_matching_branch() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let mut success_text = None;
    let mut failure_text = None;

    for seed in 1..=256 {
        let state = staged_entry_state(
            &index,
            "wuxia_heuksa_bang_first_fight",
            "jianghu_market_street",
            &["wuxia_arrival_hidden"],
            seed,
        );
        let choice_state = apply_action_from_content(&state, &index, "event:continue")
            .unwrap()
            .state;
        let result_state =
            apply_action_from_content(&choice_state, &index, "choice:run_toward_open_street")
                .unwrap()
                .state;
        let page = scene_page_from_content(&result_state, &index).unwrap();
        let narration: Vec<_> = page
            .content_stream
            .iter()
            .filter_map(|item| item.text.clone())
            .collect();
        if result_state.last_check.as_ref().unwrap().success {
            success_text = Some(narration);
        } else {
            failure_text = Some(narration);
        }
        if success_text.is_some() && failure_text.is_some() {
            break;
        }
    }

    let success = success_text.expect("seed sweep should find a successful check");
    let failure = failure_text.expect("seed sweep should find a failed check");
    assert!(success
        .iter()
        .any(|text| text.contains("민첩하게 큰길로 물러섰다")));
    assert!(!success
        .iter()
        .any(|text| text.contains("비틀거리다 몽둥이에 쓸리며")));
    assert!(failure
        .iter()
        .any(|text| text.contains("비틀거리다 몽둥이에 쓸리며")));
    assert!(!failure
        .iter()
        .any(|text| text.contains("민첩하게 큰길로 물러섰다")));
}

#[test]
fn apprentice_entry_plain_conversion_keeps_story_choice_result_order() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let state = staged_entry_state(
        &index,
        "wuxia_cheongryu_apprentice_entry",
        "cheongryu_outer_courtyard",
        &["seo_harin_rescue_resolved", "taken_under_watch"],
        23,
    );
    let opening = scene_page_from_content(&state, &index).unwrap();
    assert_eq!(opening.content_stream[0].kind, "narration");
    assert!(opening
        .content_stream
        .iter()
        .any(|item| item.kind == "illustration"));
    assert!(opening
        .content_stream
        .iter()
        .any(|item| item.kind == "dialogue"));

    let choice_state = apply_action_from_content(&state, &index, "event:continue")
        .unwrap()
        .state;
    assert_eq!(choice_state.event_stage_index, 1);
    let choice_page = scene_page_from_content(&choice_state, &index).unwrap();
    assert_eq!(choice_page.content_stream.last().unwrap().kind, "choice");
    assert_eq!(choice_page.content_stream.last().unwrap().actions.len(), 4);

    let result_state =
        apply_action_from_content(&choice_state, &index, "choice:accept_three_month_trial")
            .unwrap()
            .state;
    assert_eq!(result_state.event_stage_index, 2);
    let result_page = scene_page_from_content(&result_state, &index).unwrap();
    assert_eq!(result_page.content_stream[0].kind, "result_summary");
    assert!(result_page
        .content_stream
        .iter()
        .any(|item| item.text.as_deref() == Some("보호는 소속의 문을 열지만 공짜가 아니다. 잡일은 벌이 아니라 청류문의 흐름을 배우는 첫 수련이 된다.")));
}
