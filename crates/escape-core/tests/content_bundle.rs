use escape_core::{
    index_content_bundle, load_content_bundle, new_game_from_content_at, scene_page_from_content,
    ContentBundleError, ContentIndexError,
};
use serde_json::json;

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");
const WUXIA_PREVIEW_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");

#[test]
fn fixture_content_bundle_loads_counts_and_public_sections() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");

    assert_eq!(bundle.schema_version, 1);
    assert_eq!(bundle.kind, "tui_adv.content_bundle");
    assert_eq!(bundle.runtime, None);
    assert_eq!(bundle.manifest.counts.get("locations"), Some(&16));
    assert_eq!(bundle.manifest.counts.get("encounters"), Some(&21));
    assert_eq!(bundle.content.locations.len(), 16);
    assert_eq!(bundle.content.encounters.len(), 21);
    assert_eq!(bundle.content.secrets.len(), 3);
}

#[test]
fn content_bundle_rejects_private_secret_fields() {
    let unsafe_bundle = r#"
{
  "schema_version": 1,
  "kind": "tui_adv.content_bundle",
  "source": "test",
  "manifest": {
    "schema_version": 1,
    "source": "test",
    "counts": {}
  },
  "content": {
    "locations": [],
    "items": [],
    "encounters": [],
    "endings": [],
    "achievements": [],
    "secrets": [
      {
        "id": "unsafe",
        "title": "unsafe",
        "final_hint": "do not publish"
      }
    ]
  }
}
"#;

    let error = load_content_bundle(unsafe_bundle).expect_err("private field should be rejected");
    assert_eq!(
        error,
        ContentBundleError::PrivateSecretField {
            secret_id: "unsafe".to_string(),
            field: "final_hint".to_string(),
        }
    );
}

#[test]
fn content_bundle_rejects_wrong_kind() {
    let wrong_kind = CONTENT_BUNDLE.replace(
        "\"kind\": \"tui_adv.content_bundle\"",
        "\"kind\": \"other.bundle\"",
    );

    let error = load_content_bundle(&wrong_kind).expect_err("wrong kind should be rejected");
    assert_eq!(
        error,
        ContentBundleError::UnsupportedKind("other.bundle".to_string())
    );
}

#[test]
fn content_bundle_loads_optional_storypack_preview_runtime_metadata() {
    let preview_bundle = r#"
{
  "schema_version": 1,
  "kind": "tui_adv.content_bundle",
  "source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml",
  "runtime": {
    "runtime_mode": "storypack_preview",
    "world_id": "wuxia_jianghu",
    "storypack_id": "wuxia_jianghu_pack",
    "default_location": "wuxia_commute_rift"
  },
  "manifest": {"schema_version": 1, "source": "preview", "counts": {}},
  "content": {
    "locations": [],
    "items": [],
    "encounters": [],
    "endings": [],
    "achievements": [],
    "secrets": []
  }
}
"#;

    let bundle = load_content_bundle(preview_bundle).expect("preview bundle should load");
    let runtime = bundle
        .runtime
        .expect("preview runtime metadata should load");
    assert_eq!(runtime.runtime_mode, "storypack_preview");
    assert_eq!(runtime.world_id, "wuxia_jianghu");
    assert_eq!(runtime.storypack_id, "wuxia_jianghu_pack");
    assert_eq!(runtime.default_location, "wuxia_commute_rift");
}

#[test]
fn wuxia_scene_page_uses_storybook_chapter_label() {
    let bundle =
        load_content_bundle(WUXIA_PREVIEW_BUNDLE).expect("wuxia preview bundle should load");
    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    let state = new_game_from_content_at(123, &index, "wuxia_commute_rift")
        .expect("wuxia content-backed game should start");

    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    assert_eq!(page.location.id, "wuxia_commute_rift");
    assert_eq!(page.chapter_label, "천기록 1쪽");
}

#[test]
fn fixture_content_bundle_indexes_locations_and_encounters() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");

    assert_eq!(index.locations_len(), 16);
    assert_eq!(index.encounters_len(), 21);

    let dev_desk = index.location("dev_desk").expect("dev_desk location");
    assert_eq!(dev_desk.name, "내 자리");
    assert_eq!(dev_desk.connections, vec!["dev_office"]);

    let printer = index
        .encounter("printer_prints_alone")
        .expect("printer encounter");
    assert_eq!(printer.title, "복합기가 혼자 출력한다");
    assert_eq!(printer.conditions.locations, vec!["printer_area"]);
    assert_eq!(printer.choices.len(), 3);

    let read_printout = printer
        .choices
        .iter()
        .find(|choice| choice.id == "read_printout")
        .expect("read_printout choice");
    assert_eq!(read_printout.label, "출력물을 읽는다");
    assert_eq!(read_printout.cost.get("sanity"), Some(&3));
    assert_eq!(
        read_printout.outcome.add_clues,
        vec!["future_choice_printout"]
    );

    let presentation = printer
        .presentation
        .as_ref()
        .expect("printer encounter should carry presentation metadata");
    assert_eq!(presentation.visual_id.as_deref(), Some("printer_anomaly"));
    assert_eq!(presentation.layout.as_deref(), Some("anomaly_object"));
    assert_eq!(presentation.speaker.as_deref(), Some("시스템 복합기"));
    assert_eq!(presentation.effect_cues.len(), 1);
    assert_eq!(presentation.effect_cues[0].kind, "glyph_anomaly");
    assert_eq!(presentation.effect_cues[0].source, "copier_output");
    assert!((presentation.effect_cues[0].intensity - 0.72).abs() < f32::EPSILON);
    assert_eq!(
        presentation.effect_cues[0].stable_terms,
        vec!["비상계단", "토너", "접힌 방향"]
    );
    assert_eq!(
        presentation.effect_cues[0].distortion,
        "reflow_then_stabilize"
    );

    let combat = index
        .encounter("supply_closet_auto_brawl")
        .expect("schema-less combat prototype encounter");
    assert_eq!(combat.title, "물품창고 자동 난투");
    assert_eq!(combat.conditions.locations, vec!["supply_closet"]);
    assert_eq!(
        combat.conditions.required_flags,
        vec!["supply_scuffle_started"]
    );
    assert_eq!(
        combat.conditions.forbidden_flags,
        vec!["supply_scuffle_resolved"]
    );
    assert_eq!(
        combat
            .presentation
            .as_ref()
            .expect("combat encounter should carry presentation")
            .layout
            .as_deref(),
        Some("combat_intervention")
    );
    assert_eq!(combat.choices.len(), 3);
}

#[test]
fn preview_fixture_indexes_wuxia_first_fight() {
    let bundle =
        load_content_bundle(WUXIA_PREVIEW_BUNDLE).expect("wuxia preview bundle should load");
    let runtime = bundle.runtime.as_ref().expect("preview runtime metadata");
    assert_eq!(runtime.runtime_mode, "storypack_preview");
    assert_eq!(runtime.world_id, "wuxia_jianghu");
    assert_eq!(runtime.storypack_id, "wuxia_jianghu_pack");
    assert_eq!(runtime.default_location, "wuxia_commute_rift");
    assert_eq!(bundle.manifest.counts.get("locations"), Some(&5));
    assert_eq!(bundle.manifest.counts.get("items"), Some(&4));
    assert_eq!(bundle.manifest.counts.get("encounters"), Some(&30));
    assert_eq!(bundle.manifest.counts.get("achievements"), Some(&2));

    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    assert_eq!(index.locations_len(), 5);
    assert_eq!(index.encounters_len(), 30);

    let market = index
        .location("jianghu_market_street")
        .expect("market street location");
    assert_eq!(market.connections, vec!["jianghu_roadside"]);

    let fight = index
        .encounter("wuxia_heuksa_bang_first_fight")
        .expect("first fight encounter");
    assert_eq!(fight.title, "흑사방 첫 난투");
    assert_eq!(fight.conditions.locations, vec!["jianghu_market_street"]);
    assert_eq!(
        fight.conditions.required_flags,
        vec!["wuxia_arrival_hidden"]
    );
    assert_eq!(
        fight.conditions.forbidden_flags,
        vec!["heuksa_bang_first_fight_resolved"]
    );
    assert_eq!(
        fight
            .presentation
            .as_ref()
            .expect("first fight presentation")
            .layout
            .as_deref(),
        Some("combat_intervention")
    );
    assert_eq!(fight.choices.len(), 5);
    let fallback = fight
        .choices
        .iter()
        .find(|choice| choice.id == "run_toward_open_street")
        .expect("fallback retreat choice");
    assert_eq!(fallback.outcome.resources.get("health"), Some(&-3));
    assert_eq!(
        fallback.outcome.add_clues,
        vec!["violence_is_real", "open_street_escape_route"]
    );

    let fragment = index
        .encounter("wuxia_cheonggi_record_first_fragment")
        .expect("first cheonggi fragment encounter");
    assert_eq!(fragment.title, "천기록 첫 편린");
    assert_eq!(fragment.conditions.locations, vec!["jianghu_market_street"]);
    assert_eq!(
        fragment.conditions.required_flags,
        vec!["heuksa_bang_first_fight_resolved"]
    );
    assert_eq!(
        fragment.conditions.forbidden_flags,
        vec!["cheonggi_record_first_fragment_resolved"]
    );
    assert_eq!(
        fragment
            .presentation
            .as_ref()
            .expect("first fragment presentation")
            .layout
            .as_deref(),
        Some("cheonggi_record")
    );
    assert_eq!(fragment.choices.len(), 4);
    let fallback_fragment = fragment
        .choices
        .iter()
        .find(|choice| choice.id == "close_notebook_without_choice")
        .expect("fallback notebook choice");
    assert_eq!(
        fallback_fragment.outcome.add_flags,
        vec![
            "cheonggi_record_awakened",
            "first_fragment_seen",
            "cheonggi_record_first_fragment_resolved",
            "cheonggi_record_caution"
        ]
    );

    let courtyard = index
        .location("cheongryu_outer_courtyard")
        .expect("cheongryu outer courtyard location");
    assert_eq!(courtyard.name, "청류문 외곽 마당");
    assert_eq!(
        courtyard.connections,
        vec!["jianghu_market_street", "black_serpent_ledger_vault"]
    );

    let ledger_vault = index
        .location("black_serpent_ledger_vault")
        .expect("black serpent ledger vault location");
    assert_eq!(ledger_vault.name, "흑사방 장부고");
    assert_eq!(ledger_vault.connections, vec!["cheongryu_outer_courtyard"]);

    let rescue = index
        .encounter("wuxia_seo_harin_rescue")
        .expect("seo harin rescue encounter");
    assert_eq!(rescue.title, "서하린의 개입");
    assert_eq!(rescue.conditions.locations, vec!["jianghu_market_street"]);
    assert_eq!(
        rescue.conditions.required_flags,
        vec![
            "heuksa_bang_first_fight_resolved",
            "cheonggi_record_first_fragment_resolved"
        ]
    );
    assert_eq!(
        rescue.conditions.forbidden_flags,
        vec!["seo_harin_rescue_resolved"]
    );
    assert_eq!(
        rescue
            .presentation
            .as_ref()
            .expect("rescue presentation")
            .layout
            .as_deref(),
        Some("rescue_and_investigation")
    );
    assert_eq!(rescue.choices.len(), 5);
    let plain_truth = rescue
        .choices
        .iter()
        .find(|choice| choice.id == "tell_plain_truth")
        .expect("plain truth fallback choice");
    assert_eq!(
        plain_truth.outcome.add_flags,
        vec![
            "seo_harin_rescue_resolved",
            "seo_harin_intervened",
            "taken_under_watch",
            "outsider_claim_recorded",
            "truthful_outsider_claim"
        ]
    );
    assert_eq!(
        plain_truth.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let apprentice = index
        .encounter("wuxia_cheongryu_apprentice_entry")
        .expect("cheongryu apprentice entry encounter");
    assert_eq!(apprentice.title, "청류문 임시 수습생 등록");
    assert_eq!(
        apprentice.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        apprentice.conditions.required_flags,
        vec!["seo_harin_rescue_resolved", "taken_under_watch"]
    );
    assert_eq!(
        apprentice.conditions.forbidden_flags,
        vec!["cheongryu_apprentice_entry_resolved"]
    );
    assert_eq!(
        apprentice
            .presentation
            .as_ref()
            .expect("apprentice presentation")
            .layout
            .as_deref(),
        Some("cheongryu_apprenticeship")
    );
    assert_eq!(apprentice.choices.len(), 4);
    let trial = apprentice
        .choices
        .iter()
        .find(|choice| choice.id == "accept_three_month_trial")
        .expect("three month trial fallback choice");
    assert_eq!(
        trial.outcome.add_flags,
        vec![
            "cheongryu_apprentice_entry_resolved",
            "cheongryu_trial_started",
            "seo_harin_mentor_thread",
            "sect_debt_accepted",
            "chore_training_open"
        ]
    );
    assert_eq!(trial.outcome.add_items, vec!["work_chore_token"]);
    assert_eq!(
        trial.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let sparring = index
        .encounter("wuxia_cheongryu_chore_sparring")
        .expect("cheongryu chore sparring encounter");
    assert_eq!(sparring.title, "청류문 장작 마당 첫 겨루기");
    assert_eq!(
        sparring.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        sparring.conditions.required_flags,
        vec![
            "cheongryu_apprentice_entry_resolved",
            "cheongryu_trial_started",
            "cheonggi_record_awakened",
            "first_fragment_seen"
        ]
    );
    assert_eq!(
        sparring.conditions.forbidden_flags,
        vec!["cheongryu_chore_sparring_resolved"]
    );
    let sparring_presentation = sparring
        .presentation
        .as_ref()
        .expect("sparring presentation");
    assert_eq!(
        sparring_presentation.layout.as_deref(),
        Some("combat_intervention")
    );
    assert_eq!(
        sparring_presentation.effect_cues[0].stable_terms,
        vec!["균형", "호흡", "장작"]
    );
    assert_eq!(sparring.choices.len(), 4);
    let fallback_sparring = sparring
        .choices
        .iter()
        .find(|choice| choice.id == "step_back_with_firewood")
        .expect("step back fallback choice");
    assert_eq!(
        fallback_sparring.outcome.add_flags,
        vec![
            "cheongryu_chore_sparring_resolved",
            "chore_sparring_completed",
            "balance_training_noticed",
            "office_combat_model_reused"
        ]
    );
    assert_eq!(
        fallback_sparring.outcome.add_clues,
        vec![
            "balance_matters_more_than_force",
            "office_items_can_translate_to_training"
        ]
    );
    assert_eq!(
        fallback_sparring.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let raid = index
        .encounter("wuxia_cheongryu_raid_route_split")
        .expect("cheongryu raid route split encounter");
    assert_eq!(raid.title, "청류문 습격과 갈라지는 길");
    assert_eq!(raid.conditions.locations, vec!["cheongryu_outer_courtyard"]);
    assert_eq!(
        raid.conditions.required_flags,
        vec![
            "cheongryu_apprentice_entry_resolved",
            "cheongryu_trial_started",
            "cheonggi_record_awakened",
            "first_fragment_seen",
            "cheongryu_chore_sparring_resolved"
        ]
    );
    assert_eq!(
        raid.conditions.forbidden_flags,
        vec!["cheongryu_raid_route_split_resolved"]
    );
    let raid_presentation = raid.presentation.as_ref().expect("raid presentation");
    assert_eq!(
        raid_presentation.layout.as_deref(),
        Some("raid_route_pressure")
    );
    assert_eq!(
        raid_presentation.effect_cues[0].stable_terms,
        vec!["청류문", "백도맹", "천기록"]
    );
    assert_eq!(raid.choices.len(), 4);
    let fallback_raid = raid
        .choices
        .iter()
        .find(|choice| choice.id == "evacuate_the_wounded_first")
        .expect("evacuate wounded fallback choice");
    assert_eq!(
        fallback_raid.outcome.add_flags,
        vec![
            "cheongryu_raid_route_split_resolved",
            "cheongryu_raid_survived",
            "route_commitment_pressure",
            "route_commitment_deferred",
            "wounded_saved_flag",
            "seo_harin_survived_raid"
        ]
    );
    assert_eq!(
        fallback_raid.outcome.add_clues,
        vec![
            "saving_people_delays_route_choice",
            "blood_moon_targets_cheonggi_record"
        ]
    );
    assert_eq!(
        fallback_raid.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let wounded_fallback = index
        .encounter("wuxia_cheongryu_raid_wounded_fallback")
        .expect("cheongryu raid wounded fallback encounter");
    assert_eq!(wounded_fallback.title, "부상자 피난처와 미뤄진 선택");
    assert_eq!(
        wounded_fallback.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        wounded_fallback.conditions.required_flags,
        vec![
            "cheongryu_raid_route_split_resolved",
            "route_commitment_deferred",
            "wounded_saved_flag",
            "cheongryu_raid_survived"
        ]
    );
    assert_eq!(
        wounded_fallback.conditions.forbidden_flags,
        vec!["cheongryu_raid_wounded_fallback_resolved"]
    );
    let wounded_presentation = wounded_fallback
        .presentation
        .as_ref()
        .expect("wounded fallback presentation");
    assert_eq!(
        wounded_presentation.layout.as_deref(),
        Some("wounded_fallback_route_pressure")
    );
    assert_eq!(
        wounded_presentation.effect_cues[0].stable_terms,
        vec!["부상자", "백도맹", "천기각"]
    );
    assert_eq!(wounded_fallback.choices.len(), 4);
    let stabilize = wounded_fallback
        .choices
        .iter()
        .find(|choice| choice.id == "stabilize_wounded_until_dawn")
        .expect("stabilize wounded fallback choice");
    assert_eq!(
        stabilize.outcome.add_flags,
        vec![
            "cheongryu_raid_wounded_fallback_resolved",
            "deferred_route_reopened",
            "route_commitment_deferred",
            "wounded_shelter_stabilized",
            "survivor_roll_call_complete",
            "route_delay_cost_recorded"
        ]
    );
    assert_eq!(
        stabilize.outcome.add_clues,
        vec![
            "saving_people_changed_witnesses",
            "deferred_choice_is_still_choice"
        ]
    );
    assert_eq!(
        stabilize.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let baekdo_debt = index
        .encounter("wuxia_baekdo_medicine_debt")
        .expect("baekdo medicine debt route opener encounter");
    assert_eq!(baekdo_debt.title, "백도맹 약상자와 청류문의 채무");
    assert_eq!(
        baekdo_debt.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        baekdo_debt.conditions.required_flags,
        vec!["righteous_route_started", "cheongryu_rebuild_thread"]
    );
    assert_eq!(
        baekdo_debt.conditions.forbidden_flags,
        vec!["baekdo_medicine_debt_resolved"]
    );
    let baekdo_presentation = baekdo_debt
        .presentation
        .as_ref()
        .expect("baekdo route opener presentation");
    assert_eq!(
        baekdo_presentation.layout.as_deref(),
        Some("righteous_route_opener")
    );
    assert_eq!(baekdo_presentation.speaker.as_deref(), Some("남궁서윤"));
    assert_eq!(
        baekdo_presentation.effect_cues[0].stable_terms,
        vec!["약상자", "백도맹", "채무"]
    );
    assert_eq!(baekdo_debt.choices.len(), 4);
    let accept_debt = baekdo_debt
        .choices
        .iter()
        .find(|choice| choice.id == "accept_medicine_with_written_debt")
        .expect("fallback written debt choice");
    assert_eq!(
        accept_debt.outcome.add_flags,
        vec![
            "baekdo_medicine_debt_resolved",
            "righteous_route_opened",
            "route_opener_resolved",
            "white_path_debt_recorded",
            "cheongryu_rebuild_supplies_secured",
            "namgung_seoyun_notice"
        ]
    );
    assert_eq!(
        accept_debt.outcome.add_clues,
        vec![
            "medicine_has_banner",
            "white_path_help_has_price",
            "qingliu_survival_needs_outside_help"
        ]
    );
    assert_eq!(
        accept_debt.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let black_heaven = index
        .encounter("wuxia_black_heaven_escape_price")
        .expect("black heaven route opener encounter");
    assert_eq!(black_heaven.title, "흑천련 탈출로의 값");
    assert_eq!(
        black_heaven.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        black_heaven.conditions.required_flags,
        vec!["sapa_route_started", "dowol_debt"]
    );
    assert_eq!(
        black_heaven.conditions.forbidden_flags,
        vec!["black_heaven_escape_price_resolved"]
    );
    let black_heaven_presentation = black_heaven
        .presentation
        .as_ref()
        .expect("black heaven route opener presentation");
    assert_eq!(
        black_heaven_presentation.layout.as_deref(),
        Some("sapa_route_opener")
    );
    assert_eq!(black_heaven_presentation.speaker.as_deref(), Some("도월"));
    assert_eq!(
        black_heaven_presentation.effect_cues[0].stable_terms,
        vec!["탈출로", "흑천련", "값"]
    );
    assert_eq!(black_heaven.choices.len(), 4);
    let accept_marker = black_heaven
        .choices
        .iter()
        .find(|choice| choice.id == "accept_dowol_marker_for_safehouse")
        .expect("fallback black heaven marker choice");
    assert_eq!(
        accept_marker.outcome.add_flags,
        vec![
            "black_heaven_escape_price_resolved",
            "sapa_route_opened",
            "route_opener_resolved",
            "black_heaven_safehouse_marked",
            "market_route_debt_recorded"
        ]
    );
    assert_eq!(
        accept_marker.outcome.add_clues,
        vec![
            "black_heaven_help_marks_debt",
            "survival_bargain_is_not_loyalty"
        ]
    );
    assert_eq!(
        accept_marker.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let heavenly_archive = index
        .encounter("wuxia_heavenly_archive_previous_outsiders")
        .expect("heavenly archive route opener encounter");
    assert_eq!(heavenly_archive.title, "천기각 이전 이방인 기록");
    assert_eq!(
        heavenly_archive.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        heavenly_archive.conditions.required_flags,
        vec!["cheonggi_return_route_started", "cheonggi_record_targeted"]
    );
    assert_eq!(
        heavenly_archive.conditions.forbidden_flags,
        vec!["heavenly_archive_previous_outsiders_resolved"]
    );
    let heavenly_archive_presentation = heavenly_archive
        .presentation
        .as_ref()
        .expect("heavenly archive route opener presentation");
    assert_eq!(
        heavenly_archive_presentation.layout.as_deref(),
        Some("cheonggi_return_opener")
    );
    assert_eq!(
        heavenly_archive_presentation.speaker.as_deref(),
        Some("연소하")
    );
    assert_eq!(
        heavenly_archive_presentation.effect_cues[0].stable_terms,
        vec!["천기각", "이방인", "균열"]
    );
    assert_eq!(heavenly_archive.choices.len(), 4);
    let read_margins = heavenly_archive
        .choices
        .iter()
        .find(|choice| choice.id == "read_previous_outsider_margins")
        .expect("fallback previous outsider margins choice");
    assert_eq!(
        read_margins.outcome.add_flags,
        vec![
            "heavenly_archive_previous_outsiders_resolved",
            "cheonggi_return_route_opened",
            "route_opener_resolved",
            "previous_outsiders_record_seen"
        ]
    );
    assert_eq!(
        read_margins.outcome.add_clues,
        vec![
            "archive_has_other_outsiders",
            "return_clue_is_not_return_method"
        ]
    );
    assert_eq!(
        read_margins.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let wounded_shelter = index
        .encounter("wuxia_wounded_shelter_dawn_offers")
        .expect("wounded shelter dawn offers encounter");
    assert_eq!(wounded_shelter.title, "부상자 피난처의 새벽 제안");
    assert_eq!(
        wounded_shelter.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        wounded_shelter.conditions.required_flags,
        vec![
            "cheongryu_raid_wounded_fallback_resolved",
            "route_commitment_deferred",
            "deferred_route_reopened",
            "wounded_shelter_stabilized"
        ]
    );
    assert_eq!(
        wounded_shelter.conditions.forbidden_flags,
        vec!["wounded_shelter_dawn_offers_resolved"]
    );
    let wounded_shelter_presentation = wounded_shelter
        .presentation
        .as_ref()
        .expect("wounded shelter dawn offers presentation");
    assert_eq!(
        wounded_shelter_presentation.layout.as_deref(),
        Some("deferred_route_offer")
    );
    assert_eq!(
        wounded_shelter_presentation.speaker.as_deref(),
        Some("서하린")
    );
    assert_eq!(
        wounded_shelter_presentation.effect_cues[0].stable_terms,
        vec!["새벽", "부상자", "제안"]
    );
    assert_eq!(wounded_shelter.choices.len(), 4);
    let keep_shelter = wounded_shelter
        .choices
        .iter()
        .find(|choice| choice.id == "keep_wounded_shelter_until_noon")
        .expect("fallback wounded shelter choice");
    assert_eq!(
        keep_shelter.outcome.add_flags,
        vec![
            "wounded_shelter_dawn_offers_resolved",
            "route_commitment_reopened",
            "wounded_shelter_until_noon",
            "deferred_offer_debt_recorded"
        ]
    );
    assert_eq!(
        keep_shelter.outcome.add_clues,
        vec![
            "saving_people_changed_witnesses",
            "care_is_not_route_escape",
            "dawn_shelter_keeps_names"
        ]
    );
    assert_eq!(
        keep_shelter.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let mumyeong = index
        .encounter("wuxia_mumyeong_first_sighting")
        .expect("mumyeong first sighting encounter");
    assert_eq!(mumyeong.title, "무명 첫 목격");
    assert_eq!(
        mumyeong.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        mumyeong.conditions.required_flags,
        vec![
            "route_opener_resolved",
            "cheongryu_raid_survived",
            "cheongryu_trial_started",
            "first_fragment_seen"
        ]
    );
    assert_eq!(
        mumyeong.conditions.forbidden_flags,
        vec!["mumyeong_first_sighting_resolved"]
    );
    let mumyeong_presentation = mumyeong
        .presentation
        .as_ref()
        .expect("mumyeong first sighting presentation");
    assert_eq!(
        mumyeong_presentation.layout.as_deref(),
        Some("midgame_rival_sighting")
    );
    assert_eq!(mumyeong_presentation.speaker.as_deref(), Some("서하린"));
    assert_eq!(
        mumyeong_presentation.effect_cues[0].stable_terms,
        vec!["무명", "청류문", "흑사방"]
    );
    assert_eq!(mumyeong.choices.len(), 4);
    let observe = mumyeong
        .choices
        .iter()
        .find(|choice| choice.id == "watch_the_stolen_qingliu_flow")
        .expect("fallback stolen qingliu flow choice");
    assert_eq!(
        observe.outcome.add_flags,
        vec![
            "mumyeong_first_sighting_resolved",
            "midgame_continuity_started",
            "mumyeong_shadow_seen",
            "copied_qingliu_flow_noted"
        ]
    );
    assert_eq!(
        observe.outcome.add_clues,
        vec!["mumyeong_exists", "copied_flow_is_not_qingliu"]
    );
    assert_eq!(
        observe.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let confrontation = index
        .encounter("wuxia_mumyeong_first_confrontation")
        .expect("mumyeong first confrontation encounter");
    assert_eq!(confrontation.title, "무명 첫 대치");
    assert_eq!(
        confrontation.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        confrontation.conditions.required_flags,
        vec![
            "mumyeong_first_sighting_resolved",
            "midgame_continuity_started",
            "cheongryu_raid_survived",
            "first_fragment_seen"
        ]
    );
    assert_eq!(
        confrontation.conditions.forbidden_flags,
        vec!["mumyeong_first_confrontation_resolved"]
    );
    let confrontation_presentation = confrontation
        .presentation
        .as_ref()
        .expect("mumyeong first confrontation presentation");
    assert_eq!(
        confrontation_presentation.layout.as_deref(),
        Some("rival_first_confrontation")
    );
    assert_eq!(confrontation_presentation.speaker.as_deref(), Some("무명"));
    assert_eq!(
        confrontation_presentation.effect_cues[0].stable_terms,
        vec!["무명", "서하린", "청류문"]
    );
    assert_eq!(confrontation.choices.len(), 5);
    let endure = confrontation
        .choices
        .iter()
        .find(|choice| choice.id == "endure_until_copy_flow_breaks")
        .expect("endure until copy flow breaks choice");
    assert_eq!(
        endure.outcome.add_flags,
        vec![
            "mumyeong_first_confrontation_resolved",
            "mumyeong_rival_thread_opened",
            "copied_flow_weakness_noted"
        ]
    );
    assert_eq!(
        endure.outcome.add_clues,
        vec!["copy_style_has_gap", "copied_flow_is_not_qingliu"]
    );
    assert_eq!(
        endure.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let copy_style = index
        .encounter("wuxia_mumyeong_copy_style_reveal")
        .expect("mumyeong copy style reveal encounter");
    assert_eq!(copy_style.title, "무명의 카피 무공 공개");
    assert_eq!(
        copy_style.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        copy_style.conditions.required_flags,
        vec![
            "mumyeong_first_confrontation_resolved",
            "mumyeong_rival_thread_opened",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        copy_style.conditions.forbidden_flags,
        vec!["mumyeong_copy_style_reveal_resolved"]
    );
    let copy_style_presentation = copy_style
        .presentation
        .as_ref()
        .expect("mumyeong copy style presentation");
    assert_eq!(
        copy_style_presentation.layout.as_deref(),
        Some("copy_style_analysis")
    );
    assert_eq!(copy_style_presentation.speaker.as_deref(), Some("서하린"));
    assert_eq!(
        copy_style_presentation.effect_cues[0].stable_terms,
        vec!["무명", "청류안", "천기록"]
    );
    assert_eq!(copy_style.choices.len(), 4);
    let breath = copy_style
        .choices
        .iter()
        .find(|choice| choice.id == "listen_for_breath_mismatch")
        .expect("listen for breath mismatch choice");
    assert_eq!(
        breath.outcome.add_flags,
        vec![
            "mumyeong_copy_style_reveal_resolved",
            "copy_style_hint_recorded",
            "copied_breath_mismatch_noted"
        ]
    );
    assert_eq!(
        breath.outcome.add_clues,
        vec!["breath_mismatch_marks_copy", "understanding_is_not_copying"]
    );
    assert_eq!(
        breath.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let orthodox_style = index
        .encounter("wuxia_mumyeong_reads_orthodox_style")
        .expect("mumyeong orthodox style trace encounter");
    assert_eq!(orthodox_style.title, "무명의 정파 무공 간파");
    assert_eq!(
        orthodox_style.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        orthodox_style.conditions.required_flags,
        vec![
            "mumyeong_copy_style_reveal_resolved",
            "copy_style_hint_recorded",
            "midgame_continuity_started",
            "first_fragment_seen"
        ]
    );
    assert_eq!(
        orthodox_style.conditions.forbidden_flags,
        vec!["mumyeong_reads_orthodox_style_resolved"]
    );
    let orthodox_presentation = orthodox_style
        .presentation
        .as_ref()
        .expect("mumyeong orthodox style presentation");
    assert_eq!(
        orthodox_presentation.layout.as_deref(),
        Some("orthodox_style_trace")
    );
    assert_eq!(orthodox_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        orthodox_presentation.effect_cues[0].stable_terms,
        vec!["현악문", "복호금쇄수", "무명"]
    );
    assert_eq!(orthodox_style.choices.len(), 4);
    let reconstruct = orthodox_style
        .choices
        .iter()
        .find(|choice| choice.id == "reconstruct_mumyeongs_sightline")
        .expect("reconstruct mumyeong sightline choice");
    assert_eq!(
        reconstruct.outcome.add_flags,
        vec![
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "mumyeong_sightline_reconstructed"
        ]
    );
    assert_eq!(
        reconstruct.outcome.add_clues,
        vec![
            "bokho_geumsaesu_name_recorded",
            "departure_truth_still_incomplete"
        ]
    );
    assert_eq!(
        reconstruct.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let midgame_reunion = index
        .encounter("wuxia_mumyeong_midgame_reunion")
        .expect("mumyeong midgame reunion encounter");
    assert_eq!(midgame_reunion.title, "무명 중반 재회");
    assert_eq!(
        midgame_reunion.conditions.required_flags,
        vec![
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "mumyeong_first_confrontation_resolved",
            "mumyeong_rival_thread_opened"
        ]
    );

    let boss = index
        .encounter("wuxia_boss_first_appearance")
        .expect("boss first appearance encounter");
    assert_eq!(boss.title, "보스 첫 등장");
    assert_eq!(
        boss.conditions.required_flags,
        vec![
            "mumyeong_midgame_reunion_resolved",
            "mumyeong_mirror_thread_deepened",
            "cheongryu_raid_survived",
            "midgame_continuity_started"
        ]
    );

    let request_for_aid = index
        .encounter("wuxia_mumyeong_request_for_aid")
        .expect("Mumyeong aid request encounter");
    assert_eq!(request_for_aid.title, "무명의 도움 요청");
    assert_eq!(
        request_for_aid.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        request_for_aid.conditions.required_flags,
        vec![
            "boss_first_appearance_resolved",
            "boss_wall_thread_opened",
            "black_serpent_core_pressure_opened",
            "mumyeong_mirror_thread_deepened",
            "orthodox_style_trace_recorded",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        request_for_aid.conditions.forbidden_flags,
        vec!["mumyeong_request_for_aid_resolved"]
    );
    let request_presentation = request_for_aid
        .presentation
        .as_ref()
        .expect("Mumyeong aid request presentation");
    assert_eq!(
        request_presentation.layout.as_deref(),
        Some("failed_aid_records")
    );
    assert_eq!(request_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        request_presentation.effect_cues[0].stable_terms,
        vec!["무명", "청류문", "정파"]
    );
    assert_eq!(request_for_aid.choices.len(), 4);
    let rejected_letters = request_for_aid
        .choices
        .iter()
        .find(|choice| choice.id == "search_the_rejected_aid_letters")
        .expect("rejected aid letters choice");
    assert_eq!(
        rejected_letters.outcome.add_flags,
        vec![
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "rejected_aid_letters_read"
        ]
    );
    assert_eq!(
        rejected_letters.outcome.add_clues,
        vec![
            "mumyeong_tried_to_save_qingliu",
            "orthodox_refusal_broke_mumyeong"
        ]
    );
    assert_eq!(
        rejected_letters.outcome.add_items,
        vec!["rejected_aid_letter_fragment"]
    );
    assert_eq!(
        rejected_letters.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let awakening = index
        .encounter("wuxia_mumyeong_awakening")
        .expect("Mumyeong awakening encounter");
    assert_eq!(awakening.title, "무명의 각성");
    assert_eq!(
        awakening.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        awakening.conditions.required_flags,
        vec![
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "mumyeong_copy_style_reveal_resolved",
            "copy_style_hint_recorded",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        awakening.conditions.forbidden_flags,
        vec!["mumyeong_awakening_resolved"]
    );
    let awakening_presentation = awakening
        .presentation
        .as_ref()
        .expect("Mumyeong awakening presentation");
    assert_eq!(
        awakening_presentation.layout.as_deref(),
        Some("anger_copy_bloom")
    );
    assert_eq!(awakening_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        awakening_presentation.effect_cues[0].stable_terms,
        vec!["무명", "카피", "분노"]
    );
    assert_eq!(awakening.choices.len(), 4);
    let compare = awakening
        .choices
        .iter()
        .find(|choice| choice.id == "compare_anger_to_copied_flow")
        .expect("compare anger to copied flow choice");
    assert_eq!(
        compare.outcome.add_flags,
        vec![
            "mumyeong_awakening_resolved",
            "mumyeong_awakening_thread_opened",
            "copy_corruption_thread_opened"
        ]
    );
    assert_eq!(
        compare.outcome.add_clues,
        vec![
            "mumyeong_copy_bloomed_from_anger",
            "copy_is_wound_not_growth"
        ]
    );
    assert_eq!(
        compare.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let qingliu_attack = index
        .encounter("wuxia_qingliu_attack_after_war")
        .expect("Qingliu attack trace encounter");
    assert_eq!(qingliu_attack.title, "무너져가는 청류문 습격의 흔적");
    assert_eq!(
        qingliu_attack.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        qingliu_attack.conditions.required_flags,
        vec![
            "mumyeong_awakening_resolved",
            "mumyeong_awakening_thread_opened",
            "copy_corruption_thread_opened",
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        qingliu_attack.conditions.forbidden_flags,
        vec!["qingliu_attack_after_war_resolved"]
    );
    let qingliu_presentation = qingliu_attack
        .presentation
        .as_ref()
        .expect("Qingliu attack trace presentation");
    assert_eq!(
        qingliu_presentation.layout.as_deref(),
        Some("attack_trace_investigation")
    );
    assert_eq!(qingliu_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        qingliu_presentation.effect_cues[0].stable_terms,
        vec!["청류문", "현악문", "복호금쇄수"]
    );
    assert_eq!(qingliu_attack.choices.len(), 4);
    let lock_scars = qingliu_attack
        .choices
        .iter()
        .find(|choice| choice.id == "inspect_bokho_lock_scars")
        .expect("inspect Bokho lock scars choice");
    assert_eq!(
        lock_scars.outcome.add_flags,
        vec![
            "qingliu_attack_after_war_resolved",
            "qingliu_attack_trace_confirmed",
            "hyeonakmun_attack_thread_opened"
        ]
    );
    assert_eq!(
        lock_scars.outcome.add_clues,
        vec![
            "bokho_geumsaesu_used_on_qingliu",
            "full_flashback_still_unopened"
        ]
    );
    assert_eq!(
        lock_scars.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let destroys_orthodox = index
        .encounter("wuxia_mumyeong_destroys_orthodox_sect")
        .expect("Mumyeong destroys orthodox sect consequence trace encounter");
    assert_eq!(destroys_orthodox.title, "비어 버린 현악문 산문");
    assert_eq!(
        destroys_orthodox.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        destroys_orthodox.conditions.required_flags,
        vec![
            "qingliu_attack_after_war_resolved",
            "qingliu_attack_trace_confirmed",
            "hyeonakmun_attack_thread_opened",
            "mumyeong_awakening_resolved",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        destroys_orthodox.conditions.forbidden_flags,
        vec!["mumyeong_destroys_orthodox_sect_resolved"]
    );
    let destroys_presentation = destroys_orthodox
        .presentation
        .as_ref()
        .expect("Hyeonakmun consequence trace presentation");
    assert_eq!(
        destroys_presentation.layout.as_deref(),
        Some("hyeonakmun_empty_gate_record")
    );
    assert_eq!(destroys_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        destroys_presentation.effect_cues[0].stable_terms,
        vec!["현악문", "복호금쇄수", "무명"]
    );
    assert_eq!(destroys_orthodox.choices.len(), 4);
    let read_record = destroys_orthodox
        .choices
        .iter()
        .find(|choice| choice.id == "read_hyeonakmun_empty_gate_record")
        .expect("read Hyeonakmun empty gate record choice");
    assert_eq!(
        read_record.outcome.add_flags,
        vec![
            "mumyeong_destroys_orthodox_sect_resolved",
            "hyeonakmun_destruction_thread_opened",
            "departure_truth_thread_deepened"
        ]
    );
    assert_eq!(
        read_record.outcome.add_clues,
        vec![
            "hyeonakmun_was_destroyed_after_qingliu_attack",
            "destruction_is_consequence_not_salvation"
        ]
    );
    assert_eq!(
        read_record.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let boss_recruit = index
        .encounter("wuxia_boss_recruits_mumyeong")
        .expect("Boss recruits Mumyeong trace encounter");
    assert_eq!(boss_recruit.title, "흑사방 보스의 스카웃 흔적");
    assert_eq!(
        boss_recruit.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        boss_recruit.conditions.required_flags,
        vec![
            "mumyeong_destroys_orthodox_sect_resolved",
            "hyeonakmun_destruction_thread_opened",
            "departure_truth_thread_deepened",
            "boss_first_appearance_resolved",
            "boss_wall_thread_opened",
            "black_serpent_core_pressure_opened",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        boss_recruit.conditions.forbidden_flags,
        vec!["boss_recruits_mumyeong_resolved"]
    );
    let recruit_presentation = boss_recruit
        .presentation
        .as_ref()
        .expect("Boss recruitment trace presentation");
    assert_eq!(
        recruit_presentation.layout.as_deref(),
        Some("boss_recruitment_trace")
    );
    assert_eq!(recruit_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        recruit_presentation.effect_cues[0].stable_terms,
        vec!["흑사방주", "무명", "현악문"]
    );
    assert_eq!(boss_recruit.choices.len(), 4);
    let trace_offer = boss_recruit
        .choices
        .iter()
        .find(|choice| choice.id == "trace_boss_offer_after_hyeonakmun")
        .expect("trace boss offer choice");
    assert_eq!(
        trace_offer.outcome.add_flags,
        vec![
            "boss_recruits_mumyeong_resolved",
            "boss_recruitment_thread_opened",
            "boss_saw_mumyeongs_wound"
        ]
    );
    assert_eq!(
        trace_offer.outcome.add_clues,
        vec![
            "boss_recruited_mumyeong_after_hyeonakmun",
            "recruitment_was_not_salvation"
        ]
    );
    assert_eq!(
        trace_offer.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let departure_truth = index
        .encounter("wuxia_mumyeong_departure_truth_summary")
        .expect("Mumyeong departure truth summary encounter");
    assert_eq!(departure_truth.title, "봉해 둔 이탈의 진실");
    assert_eq!(
        departure_truth.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        departure_truth.conditions.required_flags,
        vec![
            "boss_recruits_mumyeong_resolved",
            "boss_recruitment_thread_opened",
            "mumyeong_destroys_orthodox_sect_resolved",
            "hyeonakmun_destruction_thread_opened",
            "departure_truth_thread_deepened",
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "mumyeong_awakening_resolved",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        departure_truth.conditions.forbidden_flags,
        vec!["mumyeong_departure_truth_summary_resolved"]
    );
    let departure_presentation = departure_truth
        .presentation
        .as_ref()
        .expect("Mumyeong departure truth summary presentation");
    assert_eq!(
        departure_presentation.layout.as_deref(),
        Some("sealed_departure_truth_summary")
    );
    assert_eq!(departure_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        departure_presentation.effect_cues[0].stable_terms,
        vec!["무명", "서하린", "현악문", "흑사방주"]
    );
    assert_eq!(departure_truth.choices.len(), 4);
    let assemble_truth = departure_truth
        .choices
        .iter()
        .find(|choice| choice.id == "assemble_departure_truth_without_delivering")
        .expect("assemble departure truth choice");
    assert_eq!(
        assemble_truth.outcome.add_flags,
        vec![
            "mumyeong_departure_truth_summary_resolved",
            "sealed_departure_truth_summary_prepared",
            "truth_delivery_still_unopened"
        ]
    );
    assert_eq!(
        assemble_truth.outcome.add_clues,
        vec![
            "departure_truth_can_be_understood_but_not_spoken_yet",
            "seoharin_truth_delivery_requires_later_consent"
        ]
    );
    assert_eq!(
        assemble_truth.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let empty_place = index
        .encounter("wuxia_seoharin_empty_place")
        .expect("Seo Harin empty-place encounter");
    assert_eq!(empty_place.title, "비워둔 자리");
    assert_eq!(
        empty_place.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        empty_place.conditions.required_flags,
        vec![
            "mumyeong_departure_truth_summary_resolved",
            "sealed_departure_truth_summary_prepared",
            "truth_delivery_still_unopened",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        empty_place.conditions.forbidden_flags,
        vec!["seoharin_empty_place_resolved"]
    );
    let empty_place_presentation = empty_place
        .presentation
        .as_ref()
        .expect("Seo Harin empty-place presentation");
    assert_eq!(
        empty_place_presentation.layout.as_deref(),
        Some("empty_place_memory")
    );
    assert_eq!(empty_place_presentation.speaker.as_deref(), Some("서하린"));
    assert_eq!(
        empty_place_presentation.effect_cues[0].stable_terms,
        vec!["서하린", "무명", "청류문", "목검"]
    );
    assert_eq!(empty_place.choices.len(), 4);
    let ask_empty_place = empty_place
        .choices
        .iter()
        .find(|choice| choice.id == "ask_who_kept_the_empty_place")
        .expect("ask who kept the empty place choice");
    assert_eq!(
        ask_empty_place.outcome.add_flags,
        vec![
            "seoharin_empty_place_resolved",
            "seoharin_axis_opened",
            "empty_place_remembered",
            "truth_delivery_still_unopened"
        ]
    );
    assert_eq!(
        ask_empty_place.outcome.add_clues,
        vec![
            "seoharin_remembers_without_possessing",
            "mumyeong_place_still_unclaimed"
        ]
    );
    assert!(ask_empty_place.outcome.add_items.is_empty());
    assert_eq!(
        ask_empty_place.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let left_meal = index
        .encounter("wuxia_seoharin_left_meal")
        .expect("Seo Harin left-meal encounter");
    assert_eq!(left_meal.title, "남겨둔 밥");
    assert_eq!(
        left_meal.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        left_meal.conditions.required_flags,
        vec![
            "seoharin_empty_place_resolved",
            "seoharin_axis_opened",
            "empty_place_remembered",
            "truth_delivery_still_unopened",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        left_meal.conditions.forbidden_flags,
        vec!["seoharin_left_meal_resolved"]
    );
    let left_meal_presentation = left_meal
        .presentation
        .as_ref()
        .expect("Seo Harin left-meal presentation");
    assert_eq!(
        left_meal_presentation.layout.as_deref(),
        Some("left_meal_memory")
    );
    assert_eq!(left_meal_presentation.speaker.as_deref(), Some("서하린"));
    assert_eq!(
        left_meal_presentation.effect_cues[0].stable_terms,
        vec!["서하린", "밥그릇", "청류문", "귀환"]
    );
    assert_eq!(left_meal.choices.len(), 4);
    let eat_left_meal = left_meal
        .choices
        .iter()
        .find(|choice| choice.id == "eat_the_left_meal_quietly")
        .expect("eat the left meal choice");
    assert_eq!(
        eat_left_meal.outcome.add_flags,
        vec![
            "seoharin_left_meal_resolved",
            "seoharin_axis_deepened",
            "qingliu_belonging_warmed",
            "truth_delivery_still_unopened"
        ]
    );
    assert_eq!(
        eat_left_meal.outcome.add_clues,
        vec!["left_meal_was_kept_for_return", "belonging_is_daily_care"]
    );
    assert!(eat_left_meal.outcome.add_items.is_empty());
    assert_eq!(
        eat_left_meal.outcome.destination_id.as_deref(),
        Some("cheongryu_outer_courtyard")
    );

    let price_tag = index
        .encounter("wuxia_sado_final_phase_1_price_tag")
        .expect("Sado final phase 1 price-tag encounter");
    assert_eq!(price_tag.title, "사도 최종전 1페이즈: 가격표");
    assert_eq!(
        price_tag.conditions.locations,
        vec!["cheongryu_outer_courtyard"]
    );
    assert_eq!(
        price_tag.conditions.required_flags,
        vec![
            "seoharin_left_meal_resolved",
            "seoharin_empty_place_resolved",
            "seoharin_axis_opened",
            "empty_place_remembered",
            "truth_delivery_still_unopened",
            "boss_recruits_mumyeong_resolved",
            "boss_recruitment_thread_opened",
            "boss_first_appearance_resolved",
            "black_serpent_core_pressure_opened",
            "sealed_departure_truth_summary_prepared",
            "midgame_continuity_started"
        ]
    );
    assert_eq!(
        price_tag.conditions.forbidden_flags,
        vec!["sado_final_phase_1_price_tag_resolved"]
    );
    let price_tag_presentation = price_tag
        .presentation
        .as_ref()
        .expect("Sado final phase 1 price-tag presentation");
    assert_eq!(
        price_tag_presentation.layout.as_deref(),
        Some("final_phase_price_tag")
    );
    assert_eq!(price_tag_presentation.speaker.as_deref(), Some("흑사방주"));
    assert_eq!(
        price_tag_presentation.effect_cues[0].stable_terms,
        vec!["흑사방주", "장부", "빚", "청류문"]
    );
    assert_eq!(price_tag.choices.len(), 4);
    let secure_ledger = price_tag
        .choices
        .iter()
        .find(|choice| choice.id == "secure_the_blackscale_ledger")
        .expect("secure blackscale ledger choice");
    assert_eq!(
        secure_ledger.outcome.add_flags,
        vec![
            "sado_final_phase_1_price_tag_resolved",
            "final_state_routing_seeded",
            "final_price_tag_ledger_secured",
            "final_network_ledger_secured_seeded",
            "final_evidence_strong_seeded",
            "final_item_logs_blackscale_ledger_seeded"
        ]
    );
    assert_eq!(
        secure_ledger.outcome.add_clues,
        vec![
            "item_blackscale_ledger_logged",
            "black_serpent_network_structure_seen",
            "alliance_silence_accountability_seeded"
        ]
    );
    assert!(secure_ledger.outcome.add_items.is_empty());
    assert_eq!(
        secure_ledger.outcome.destination_id.as_deref(),
        Some("black_serpent_ledger_vault")
    );

    let weakpoint = index
        .encounter("wuxia_sado_final_phase_2_weakpoint_control")
        .expect("Sado final phase 2 weakpoint-control encounter");
    assert_eq!(weakpoint.title, "사도 최종전 2페이즈: 약점 장악");
    assert_eq!(
        weakpoint.conditions.locations,
        vec!["black_serpent_ledger_vault"]
    );
    assert_eq!(
        weakpoint.conditions.required_flags,
        vec![
            "sado_final_phase_1_price_tag_resolved",
            "final_state_routing_seeded"
        ]
    );
    assert_eq!(
        weakpoint.conditions.forbidden_flags,
        vec!["sado_final_phase_2_weakpoint_control_resolved"]
    );
    let weakpoint_presentation = weakpoint
        .presentation
        .as_ref()
        .expect("Sado final phase 2 weakpoint-control presentation");
    assert_eq!(
        weakpoint_presentation.layout.as_deref(),
        Some("final_phase_weakpoint_control")
    );
    assert_eq!(weakpoint_presentation.speaker.as_deref(), Some("흑사방주"));
    assert_eq!(
        weakpoint_presentation.effect_cues[0].stable_terms,
        vec!["서하린", "무명", "천기록", "약점"]
    );
    assert_eq!(weakpoint.choices.len(), 4);
    let return_flow = weakpoint
        .choices
        .iter()
        .find(|choice| choice.id == "return_flow_to_mumyeong")
        .expect("return flow to Mumyeong choice");
    assert_eq!(
        return_flow.outcome.add_flags,
        vec![
            "sado_final_phase_2_weakpoint_control_resolved",
            "final_phase_2_weakpoint_control_resolved",
            "final_mumyeong_salvation_partial_seeded",
            "final_successor_route_suppressed_seeded",
            "final_own_flow_choice_opened_seeded",
            "final_player_method_protected_as_person_seeded"
        ]
    );
    assert_eq!(
        return_flow.outcome.add_clues,
        vec![
            "mumyeong_flow_is_not_tool",
            "successor_logic_wavers",
            "stolen_form_can_stop"
        ]
    );
    assert!(return_flow.outcome.add_items.is_empty());
    assert_eq!(
        return_flow.outcome.destination_id.as_deref(),
        Some("black_serpent_ledger_vault")
    );

    let outside_calculation = index
        .encounter("wuxia_sado_final_phase_3_outside_calculation")
        .expect("Sado final phase 3 outside-calculation encounter");
    assert_eq!(outside_calculation.title, "사도 최종전 3페이즈: 계산식 밖");
    assert_eq!(
        outside_calculation.conditions.locations,
        vec!["black_serpent_ledger_vault"]
    );
    assert_eq!(
        outside_calculation.conditions.required_flags,
        vec![
            "sado_final_phase_2_weakpoint_control_resolved",
            "final_phase_2_weakpoint_control_resolved",
            "final_state_routing_seeded"
        ]
    );
    assert_eq!(
        outside_calculation.conditions.forbidden_flags,
        vec!["sado_final_phase_3_outside_calculation_resolved"]
    );
    let outside_presentation = outside_calculation
        .presentation
        .as_ref()
        .expect("Sado final phase 3 outside-calculation presentation");
    assert_eq!(
        outside_presentation.layout.as_deref(),
        Some("final_phase_outside_calculation")
    );
    assert_eq!(outside_presentation.speaker.as_deref(), Some("흑사방주"));
    assert_eq!(
        outside_presentation.effect_cues[0].stable_terms,
        vec!["계산식", "서하린", "무명", "목검"]
    );
    assert_eq!(outside_calculation.choices.len(), 5);
    let empty_place = outside_calculation
        .choices
        .iter()
        .find(|choice| choice.id == "remember_the_empty_place")
        .expect("remember the empty place choice");
    assert_eq!(
        empty_place.outcome.add_flags,
        vec![
            "sado_final_phase_3_outside_calculation_resolved",
            "final_phase_3_outside_calculation_resolved",
            "final_combat_result_battle_victory_seeded",
            "final_boss_resolution_true_route_candidate_seeded",
            "final_seoharin_axis_high_preserved_seeded",
            "final_unpriced_wooden_sword_condition_raised_seeded",
            "final_player_method_outside_calculation_confirmed_seeded"
        ]
    );
    assert_eq!(
        empty_place.outcome.add_clues,
        vec![
            "empty_place_is_not_for_sale",
            "unpriced_wooden_sword_condition_rises",
            "sado_calculation_fails_to_price_waiting"
        ]
    );
    assert!(empty_place.outcome.add_items.is_empty());
    assert_eq!(
        empty_place.outcome.destination_id.as_deref(),
        Some("black_serpent_ledger_vault")
    );

    let boss_resolution = index
        .encounter("wuxia_boss_resolution")
        .expect("boss resolution encounter");
    assert_eq!(boss_resolution.title, "보스 결산");
    assert_eq!(
        boss_resolution.conditions.locations,
        vec!["black_serpent_ledger_vault"]
    );
    assert_eq!(
        boss_resolution.conditions.required_flags,
        vec![
            "sado_final_phase_3_outside_calculation_resolved",
            "final_phase_3_outside_calculation_resolved",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded"
        ]
    );
    assert_eq!(
        boss_resolution.conditions.forbidden_flags,
        vec!["boss_resolution_resolved"]
    );
    let boss_presentation = boss_resolution
        .presentation
        .as_ref()
        .expect("boss resolution presentation");
    assert_eq!(
        boss_presentation.layout.as_deref(),
        Some("boss_resolution_seed")
    );
    assert_eq!(boss_presentation.speaker.as_deref(), Some("천기록"));
    assert_eq!(
        boss_presentation.effect_cues[0].stable_terms,
        vec!["보스 결산", "흑사방", "무명", "무림맹"]
    );
    assert_eq!(boss_resolution.choices.len(), 5);
    let true_route = boss_resolution
        .choices
        .iter()
        .find(|choice| choice.id == "confirm_true_route_outside_calculation")
        .expect("true-route boss resolution choice");
    assert_eq!(
        true_route.outcome.add_flags,
        vec![
            "boss_resolution_resolved",
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_result_priority_applied_seeded",
            "final_epilogue_candidates_true_route_seeded",
            "final_broken_black_serpent_epilogue_candidate_seeded",
            "final_seoharin_open_gate_candidate_seeded",
            "final_mumyeong_second_wooden_sword_candidate_seeded",
            "final_qingliu_future_high_candidate_seeded"
        ]
    );
    assert_eq!(
        true_route.outcome.add_clues,
        vec![
            "boss_resolution_true_route_requires_unpriced_things",
            "broken_black_serpent_not_simple_happy_ending",
            "open_gate_suppresses_closed_gate_candidate"
        ]
    );
    assert!(true_route.outcome.add_items.is_empty());
    assert_eq!(
        true_route.outcome.destination_id.as_deref(),
        Some("black_serpent_ledger_vault")
    );
}

#[test]
fn content_index_rejects_duplicate_location_ids() {
    let mut bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let duplicate = bundle.content.locations[0].clone();
    bundle.content.locations.push(duplicate);

    let error = index_content_bundle(&bundle).expect_err("duplicate id should be rejected");
    assert_eq!(
        error,
        ContentIndexError::DuplicateId {
            section: "locations".to_string(),
            id: "dev_desk".to_string(),
        }
    );
}

#[test]
fn content_index_rejects_unknown_encounter_location_references() {
    let mut bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    bundle.content.encounters[0]["conditions"]["locations"] = json!(["missing_floor"]);

    let error = index_content_bundle(&bundle).expect_err("unknown location should be rejected");
    assert_eq!(
        error,
        ContentIndexError::UnknownEncounterLocation {
            encounter_id: "ex_employee_messenger".to_string(),
            location_id: "missing_floor".to_string(),
        }
    );
}
