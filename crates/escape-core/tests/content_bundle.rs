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
    assert_eq!(bundle.manifest.counts.get("locations"), Some(&3));
    assert_eq!(bundle.manifest.counts.get("encounters"), Some(&2));

    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    assert_eq!(index.locations_len(), 3);
    assert_eq!(index.encounters_len(), 2);

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
