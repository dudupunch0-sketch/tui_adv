use escape_core::{
    index_content_bundle, load_content_bundle, ContentBundleError, ContentIndexError,
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
    assert_eq!(bundle.manifest.counts.get("locations"), Some(&4));
    assert_eq!(bundle.manifest.counts.get("items"), Some(&3));
    assert_eq!(bundle.manifest.counts.get("encounters"), Some(&10));
    assert_eq!(bundle.manifest.counts.get("achievements"), Some(&2));

    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    assert_eq!(index.locations_len(), 4);
    assert_eq!(index.encounters_len(), 10);

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
    assert_eq!(courtyard.connections, vec!["jianghu_market_street"]);

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
