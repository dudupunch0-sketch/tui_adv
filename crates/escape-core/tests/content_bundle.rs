use escape_core::{
    index_content_bundle, load_content_bundle, ContentBundleError, ContentIndexError,
};
use serde_json::json;

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

#[test]
fn fixture_content_bundle_loads_counts_and_public_sections() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");

    assert_eq!(bundle.schema_version, 1);
    assert_eq!(bundle.kind, "tui_adv.content_bundle");
    assert_eq!(bundle.manifest.counts.get("locations"), Some(&16));
    assert_eq!(bundle.manifest.counts.get("encounters"), Some(&20));
    assert_eq!(bundle.content.locations.len(), 16);
    assert_eq!(bundle.content.encounters.len(), 20);
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
fn fixture_content_bundle_indexes_locations_and_encounters() {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");

    assert_eq!(index.locations_len(), 16);
    assert_eq!(index.encounters_len(), 20);

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
