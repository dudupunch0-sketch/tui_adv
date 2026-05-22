use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

pub const CONTENT_BUNDLE_SCHEMA_VERSION: u32 = 1;
pub const CONTENT_BUNDLE_KIND: &str = "tui_adv.content_bundle";

const PRIVATE_SECRET_FIELDS: &[&str] = &[
    "final_hint",
    "actual_ip_address",
    "office_location",
    "treasure_location",
];

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentBundle {
    pub schema_version: u32,
    pub kind: String,
    pub source: String,
    pub manifest: ContentManifest,
    pub content: ContentSections,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentManifest {
    pub schema_version: u32,
    pub source: String,
    pub counts: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentSections {
    pub locations: Vec<Value>,
    pub items: Vec<Value>,
    pub encounters: Vec<Value>,
    pub endings: Vec<Value>,
    pub achievements: Vec<Value>,
    pub secrets: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentBundleError {
    Json(String),
    UnsupportedSchemaVersion(u32),
    UnsupportedKind(String),
    PrivateSecretField { secret_id: String, field: String },
}

impl std::fmt::Display for ContentBundleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentBundleError::Json(message) => {
                write!(formatter, "invalid content bundle JSON: {message}")
            }
            ContentBundleError::UnsupportedSchemaVersion(version) => {
                write!(
                    formatter,
                    "unsupported content bundle schema version: {version}"
                )
            }
            ContentBundleError::UnsupportedKind(kind) => {
                write!(formatter, "unsupported content bundle kind: {kind}")
            }
            ContentBundleError::PrivateSecretField { secret_id, field } => {
                write!(
                    formatter,
                    "public secret {secret_id} has private-only field: {field}"
                )
            }
        }
    }
}

impl std::error::Error for ContentBundleError {}

pub fn load_content_bundle(json_text: &str) -> Result<ContentBundle, ContentBundleError> {
    let bundle: ContentBundle = serde_json::from_str(json_text)
        .map_err(|error| ContentBundleError::Json(error.to_string()))?;
    validate_content_bundle(&bundle)?;
    Ok(bundle)
}

pub fn validate_content_bundle(bundle: &ContentBundle) -> Result<(), ContentBundleError> {
    if bundle.schema_version != CONTENT_BUNDLE_SCHEMA_VERSION {
        return Err(ContentBundleError::UnsupportedSchemaVersion(
            bundle.schema_version,
        ));
    }
    if bundle.kind != CONTENT_BUNDLE_KIND {
        return Err(ContentBundleError::UnsupportedKind(bundle.kind.clone()));
    }
    validate_public_secret_fields(&bundle.content.secrets)
}

fn validate_public_secret_fields(secrets: &[Value]) -> Result<(), ContentBundleError> {
    for secret in secrets {
        let Some(secret_object) = secret.as_object() else {
            continue;
        };
        let secret_id = secret_object
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<missing>")
            .to_string();
        for private_field in PRIVATE_SECRET_FIELDS {
            if secret_object.contains_key(*private_field) {
                return Err(ContentBundleError::PrivateSecretField {
                    secret_id,
                    field: (*private_field).to_string(),
                });
            }
        }
    }
    Ok(())
}
