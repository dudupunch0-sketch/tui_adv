use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const CONTENT_BUNDLE_SCHEMA_VERSION: u32 = 1;
pub const CONTENT_BUNDLE_KIND: &str = "tui_adv.content_bundle";

const PRIVATE_SECRET_FIELDS: &[&str] = &[
    "final_hint",
    "actual_ip_address",
    "office_location",
    "treasure_location",
];
const RESOURCE_KEYS: &[&str] = &["health", "sanity", "battery", "hunger", "thirst"];

pub type ResourceMap = BTreeMap<String, i32>;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentIndexError {
    InvalidSectionItem {
        section: String,
        id: Option<String>,
        message: String,
    },
    DuplicateId {
        section: String,
        id: String,
    },
    UnknownLocationConnection {
        location_id: String,
        connected_location_id: String,
    },
    UnknownEncounterLocation {
        encounter_id: String,
        location_id: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentIndex {
    locations: BTreeMap<String, LocationDef>,
    encounters: BTreeMap<String, EncounterDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LocationDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub connections: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub danger: i32,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Eq)]
pub struct ContentConditions {
    #[serde(default)]
    pub locations: Vec<String>,
    #[serde(default)]
    pub required_flags: Vec<String>,
    #[serde(default)]
    pub forbidden_flags: Vec<String>,
    #[serde(default)]
    pub required_clues: Vec<String>,
    #[serde(default)]
    pub min_resources: ResourceMap,
    #[serde(default)]
    pub min_abilities: ResourceMap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncounterDef {
    pub id: String,
    pub title: String,
    pub body: String,
    pub conditions: ContentConditions,
    pub choices: Vec<ChoiceDef>,
    pub weight: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceDef {
    pub id: String,
    pub label: String,
    pub conditions: ContentConditions,
    pub cost: ResourceMap,
    pub outcome: OutcomeDef,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Eq)]
pub struct OutcomeDef {
    #[serde(default)]
    pub log: Option<String>,
    #[serde(default)]
    pub add_flags: Vec<String>,
    #[serde(default)]
    pub add_clues: Vec<String>,
    #[serde(default)]
    pub add_items: Vec<String>,
    #[serde(default)]
    pub destination_id: Option<String>,
    #[serde(default)]
    pub danger: i32,
    #[serde(default)]
    pub resources: ResourceMap,
}

#[derive(Clone, Debug, Deserialize)]
struct RawEncounterDef {
    id: String,
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    conditions: ContentConditions,
    #[serde(default)]
    choices: Vec<RawChoiceDef>,
    #[serde(default = "default_encounter_weight")]
    weight: u32,
}

#[derive(Clone, Debug, Deserialize)]
struct RawChoiceDef {
    id: String,
    label: String,
    #[serde(default)]
    conditions: ContentConditions,
    #[serde(default)]
    cost: ResourceMap,
    #[serde(default)]
    outcome: Value,
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

impl std::fmt::Display for ContentIndexError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentIndexError::InvalidSectionItem {
                section,
                id,
                message,
            } => match id {
                Some(id) => write!(formatter, "invalid {section} item {id}: {message}"),
                None => write!(formatter, "invalid {section} item: {message}"),
            },
            ContentIndexError::DuplicateId { section, id } => {
                write!(formatter, "duplicate {section} id: {id}")
            }
            ContentIndexError::UnknownLocationConnection {
                location_id,
                connected_location_id,
            } => write!(
                formatter,
                "location {location_id} references unknown connection: {connected_location_id}"
            ),
            ContentIndexError::UnknownEncounterLocation {
                encounter_id,
                location_id,
            } => write!(
                formatter,
                "encounter {encounter_id} references unknown location: {location_id}"
            ),
        }
    }
}

impl std::error::Error for ContentIndexError {}

impl ContentIndex {
    pub fn locations_len(&self) -> usize {
        self.locations.len()
    }

    pub fn encounters_len(&self) -> usize {
        self.encounters.len()
    }

    pub fn location(&self, id: &str) -> Option<&LocationDef> {
        self.locations.get(id)
    }

    pub fn encounter(&self, id: &str) -> Option<&EncounterDef> {
        self.encounters.get(id)
    }

    pub fn encounters(&self) -> impl Iterator<Item = &EncounterDef> {
        self.encounters.values()
    }
}

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

pub fn index_content_bundle(bundle: &ContentBundle) -> Result<ContentIndex, ContentIndexError> {
    let mut locations = BTreeMap::new();
    for location_value in &bundle.content.locations {
        let location: LocationDef = parse_section_value("locations", location_value)?;
        insert_unique("locations", &mut locations, location.id.clone(), location)?;
    }

    validate_location_connections(&locations)?;

    let location_ids: BTreeSet<&str> = locations.keys().map(String::as_str).collect();
    let mut encounters = BTreeMap::new();
    for encounter_value in &bundle.content.encounters {
        let encounter = parse_encounter(encounter_value)?;
        validate_encounter_locations(&encounter, &location_ids)?;
        insert_unique(
            "encounters",
            &mut encounters,
            encounter.id.clone(),
            encounter,
        )?;
    }

    Ok(ContentIndex {
        locations,
        encounters,
    })
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

fn parse_section_value<T>(section: &str, value: &Value) -> Result<T, ContentIndexError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(value.clone()).map_err(|error| ContentIndexError::InvalidSectionItem {
        section: section.to_string(),
        id: value_id(value),
        message: error.to_string(),
    })
}

fn parse_encounter(value: &Value) -> Result<EncounterDef, ContentIndexError> {
    let raw: RawEncounterDef = parse_section_value("encounters", value)?;
    let choices = raw
        .choices
        .into_iter()
        .map(parse_choice)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(EncounterDef {
        id: raw.id,
        title: raw.title,
        body: raw.body,
        conditions: raw.conditions,
        choices,
        weight: raw.weight,
    })
}

fn parse_choice(raw: RawChoiceDef) -> Result<ChoiceDef, ContentIndexError> {
    Ok(ChoiceDef {
        id: raw.id,
        label: raw.label,
        conditions: raw.conditions,
        cost: raw.cost,
        outcome: parse_outcome(&raw.outcome)?,
    })
}

fn parse_outcome(value: &Value) -> Result<OutcomeDef, ContentIndexError> {
    if value.is_null() {
        return Ok(OutcomeDef::default());
    }

    let mut outcome: OutcomeDef = serde_json::from_value(value.clone()).map_err(|error| {
        ContentIndexError::InvalidSectionItem {
            section: "encounter outcomes".to_string(),
            id: value_id(value),
            message: error.to_string(),
        }
    })?;

    if let Some(object) = value.as_object() {
        for resource_key in RESOURCE_KEYS {
            if let Some(resource_delta) = object.get(*resource_key).and_then(Value::as_i64) {
                outcome
                    .resources
                    .insert((*resource_key).to_string(), resource_delta as i32);
            }
        }
    }

    Ok(outcome)
}

fn insert_unique<T>(
    section: &str,
    map: &mut BTreeMap<String, T>,
    id: String,
    value: T,
) -> Result<(), ContentIndexError> {
    if map.contains_key(&id) {
        return Err(ContentIndexError::DuplicateId {
            section: section.to_string(),
            id,
        });
    }
    map.insert(id, value);
    Ok(())
}

fn validate_location_connections(
    locations: &BTreeMap<String, LocationDef>,
) -> Result<(), ContentIndexError> {
    for location in locations.values() {
        for connected_location_id in &location.connections {
            if !locations.contains_key(connected_location_id) {
                return Err(ContentIndexError::UnknownLocationConnection {
                    location_id: location.id.clone(),
                    connected_location_id: connected_location_id.clone(),
                });
            }
        }
    }
    Ok(())
}

fn validate_encounter_locations(
    encounter: &EncounterDef,
    location_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    for location_id in &encounter.conditions.locations {
        if !location_ids.contains(location_id.as_str()) {
            return Err(ContentIndexError::UnknownEncounterLocation {
                encounter_id: encounter.id.clone(),
                location_id: location_id.clone(),
            });
        }
    }
    Ok(())
}

fn value_id(value: &Value) -> Option<String> {
    value
        .as_object()
        .and_then(|object| object.get("id"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn default_encounter_weight() -> u32 {
    1
}
