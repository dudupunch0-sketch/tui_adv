use escape_core::{load_content_bundle, ContentBundleError};

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
