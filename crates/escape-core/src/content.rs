use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const CONTENT_BUNDLE_SCHEMA_VERSION: u32 = 1;
pub const CONTENT_BUNDLE_KIND: &str = "tui_adv.content_bundle";

// Mirrors: scripts/export_web_data.py PRIVATE_SECRET_FIELDS, web/src/security/publicSecretGuard.ts
const PRIVATE_SECRET_FIELDS: &[&str] = &[
    "final_hint",
    "actual_ip_address",
    "office_location",
    "treasure_location",
];
use crate::resources::RESOURCE_IDS;

const RESOURCE_KEYS: &[&str] = &RESOURCE_IDS;

pub type ResourceMap = BTreeMap<String, i32>;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentBundle {
    pub schema_version: u32,
    pub kind: String,
    pub source: String,
    #[serde(default)]
    pub runtime: Option<RuntimeMetadata>,
    pub manifest: ContentManifest,
    pub content: ContentSections,
}

impl ContentBundle {
    /// Start location declared by the bundle's runtime metadata,
    /// falling back to the engine default.
    pub fn start_location_id(&self) -> &str {
        self.runtime
            .as_ref()
            .map(|runtime| runtime.default_location.as_str())
            .filter(|location_id| !location_id.is_empty())
            .unwrap_or(crate::state::DEFAULT_START_LOCATION_ID)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RuntimeMetadata {
    pub runtime_mode: String,
    pub world_id: String,
    pub storypack_id: String,
    pub default_location: String,
    #[serde(default)]
    pub protagonist_name: Option<String>,
    #[serde(default)]
    pub progression: Option<ProgressionMetadata>,
    #[serde(default)]
    pub collapse: Option<CollapseMetadata>,
    #[serde(default)]
    pub leveling: Option<LevelingMetadata>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CollapseMetadata {
    pub encounter_id: String,
    pub resource_id: String,
    pub used_flag: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ProgressionMetadata {
    pub experience_target: u32,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LevelingMetadata {
    pub thresholds: Vec<u32>,
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
    #[serde(default)]
    pub traits: Vec<Value>,
    #[serde(default)]
    pub insights: Vec<Value>,
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
    UnknownEndingLocation {
        ending_id: String,
        location_id: String,
    },
    UnknownTrait {
        encounter_id: String,
        trait_id: String,
    },
    InvalidEvent {
        encounter_id: String,
        message: String,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TraitDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct InsightDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub check_bonus: Option<CheckBonusDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CheckBonusDef {
    pub ability: String,
    pub bonus: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentIndex {
    locations: BTreeMap<String, LocationDef>,
    items: BTreeMap<String, ItemDef>,
    encounters: BTreeMap<String, EncounterDef>,
    endings: BTreeMap<String, EndingDef>,
    achievements: BTreeMap<String, AchievementDef>,
    secrets: BTreeMap<String, PublicSecretDef>,
    traits: BTreeMap<String, TraitDef>,
    insights: BTreeMap<String, InsightDef>,
    pub runtime: Option<RuntimeMetadata>,
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
    pub disaster_types: Vec<String>,
    #[serde(default)]
    pub required_items: Vec<String>,
    #[serde(default)]
    pub required_flags: Vec<String>,
    #[serde(default)]
    pub forbidden_flags: Vec<String>,
    #[serde(default)]
    pub required_clues: Vec<String>,
    #[serde(default)]
    pub min_resources: ResourceMap,
    #[serde(default)]
    pub max_resources: ResourceMap,
    #[serde(default)]
    pub min_abilities: ResourceMap,
    #[serde(default)]
    pub min_experience: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, rename = "type")]
    pub item_type: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub usable: bool,
    #[serde(default)]
    pub use_effects: ResourceMap,
    #[serde(default)]
    pub use_log: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncounterDef {
    pub id: String,
    pub title: String,
    pub body: String,
    pub presentation: Option<PresentationDef>,
    pub event: Option<EventDef>,
    pub conditions: ContentConditions,
    pub choices: Vec<ChoiceDef>,
    pub repeatable: bool,
    pub weight: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct EventDef {
    pub stages: Vec<EventStageDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct EventStageDef {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub blocks: Vec<ContentBlockDef>,
    #[serde(default)]
    pub choices: Vec<EventChoiceRef>,
    #[serde(default)]
    pub next_stage_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentBlockDef {
    pub kind: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default)]
    pub visual_id: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
    #[serde(default)]
    pub placeholder: bool,
    #[serde(default)]
    pub branch: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct EventChoiceRef {
    pub id: String,
    #[serde(default)]
    pub next_stage_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
pub struct PresentationDef {
    #[serde(default)]
    pub visual_id: Option<String>,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default)]
    pub layout: Option<String>,
    #[serde(default)]
    pub effect_cues: Vec<PresentationEffectCue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PresentationEffectCue {
    pub kind: String,
    pub source: String,
    #[serde(default)]
    pub intensity: f32,
    #[serde(default)]
    pub stable_terms: Vec<String>,
    #[serde(default)]
    pub distortion: String,
    #[serde(default)]
    pub duration_hint_ms: Option<u32>,
    #[serde(default)]
    pub fallback_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceDef {
    pub id: String,
    pub label: String,
    pub conditions: ContentConditions,
    pub cost: ResourceMap,
    pub outcome: OutcomeDef,
    pub check: Option<AbilityCheckDef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AbilityCheckDef {
    pub ability: String,
    pub difficulty: i32,
    #[serde(default)]
    pub success: OutcomeDef,
    #[serde(default)]
    pub failure: OutcomeDef,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Eq)]
pub struct OutcomeDef {
    #[serde(default)]
    pub log: Option<String>,
    #[serde(default)]
    pub add_flags: Vec<String>,
    #[serde(default)]
    pub remove_flags: Vec<String>,
    #[serde(default)]
    pub add_clues: Vec<String>,
    #[serde(default)]
    pub add_items: Vec<String>,
    #[serde(default)]
    pub remove_items: Vec<String>,
    #[serde(default)]
    pub destination_id: Option<String>,
    #[serde(default)]
    pub danger: i32,
    #[serde(default)]
    pub resources: ResourceMap,
    #[serde(default)]
    pub set_trait: Option<String>,
    #[serde(default)]
    pub experience: Option<i32>,
    #[serde(default)]
    pub add_insights: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct EndingDef {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub conditions: ContentConditions,
    #[serde(default)]
    pub local_hint_id: Option<String>,
    #[serde(default)]
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AchievementDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub conditions: ContentConditions,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PublicSecretDef {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub unlock_flags: Vec<String>,
    #[serde(default)]
    pub public_hint_steps: Vec<String>,
    #[serde(default)]
    pub puzzle_prompt: Option<String>,
    #[serde(default)]
    pub placeholder_ip_address: Option<String>,
    #[serde(default)]
    pub final_hint_policy: Option<String>,
    #[serde(default)]
    pub reward_text: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct RawEncounterDef {
    id: String,
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    presentation: Option<PresentationDef>,
    #[serde(default)]
    event: Option<EventDef>,
    #[serde(default)]
    conditions: ContentConditions,
    #[serde(default)]
    choices: Vec<RawChoiceDef>,
    #[serde(default)]
    repeatable: bool,
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
    #[serde(default)]
    check: Option<RawAbilityCheckDef>,
}

#[derive(Clone, Debug, Deserialize)]
struct RawAbilityCheckDef {
    ability: String,
    difficulty: i32,
    #[serde(default)]
    success: Value,
    #[serde(default)]
    failure: Value,
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
            ContentIndexError::UnknownEndingLocation {
                ending_id,
                location_id,
            } => write!(
                formatter,
                "ending {ending_id} references unknown location: {location_id}"
            ),
            ContentIndexError::UnknownTrait {
                encounter_id,
                trait_id,
            } => write!(
                formatter,
                "encounter {encounter_id} references unknown trait: {trait_id}"
            ),
            ContentIndexError::InvalidEvent {
                encounter_id,
                message,
            } => {
                write!(
                    formatter,
                    "invalid event in encounter {encounter_id}: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ContentIndexError {}

impl ContentIndex {
    pub fn locations_len(&self) -> usize {
        self.locations.len()
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn encounters_len(&self) -> usize {
        self.encounters.len()
    }

    pub fn endings_len(&self) -> usize {
        self.endings.len()
    }

    pub fn achievements_len(&self) -> usize {
        self.achievements.len()
    }

    pub fn location(&self, id: &str) -> Option<&LocationDef> {
        self.locations.get(id)
    }

    pub fn item(&self, id: &str) -> Option<&ItemDef> {
        self.items.get(id)
    }

    pub fn encounter(&self, id: &str) -> Option<&EncounterDef> {
        self.encounters.get(id)
    }

    pub fn ending(&self, id: &str) -> Option<&EndingDef> {
        self.endings.get(id)
    }

    pub fn secret(&self, id: &str) -> Option<&PublicSecretDef> {
        self.secrets.get(id)
    }

    pub fn encounters(&self) -> impl Iterator<Item = &EncounterDef> {
        self.encounters.values()
    }

    pub fn items(&self) -> impl Iterator<Item = &ItemDef> {
        self.items.values()
    }

    pub fn endings(&self) -> impl Iterator<Item = &EndingDef> {
        self.endings.values()
    }

    pub fn achievements(&self) -> impl Iterator<Item = &AchievementDef> {
        self.achievements.values()
    }

    pub fn achievement(&self, id: &str) -> Option<&AchievementDef> {
        self.achievements.get(id)
    }

    pub fn traits_len(&self) -> usize {
        self.traits.len()
    }

    pub fn trait_def(&self, id: &str) -> Option<&TraitDef> {
        self.traits.get(id)
    }

    pub fn traits(&self) -> impl Iterator<Item = &TraitDef> {
        self.traits.values()
    }

    pub fn insight(&self, id: &str) -> Option<&InsightDef> {
        self.insights.get(id)
    }

    pub fn insights(&self) -> impl Iterator<Item = &InsightDef> {
        self.insights.values()
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

    let mut items = BTreeMap::new();
    for item_value in &bundle.content.items {
        let item: ItemDef = parse_section_value("items", item_value)?;
        insert_unique("items", &mut items, item.id.clone(), item)?;
    }

    let mut traits = BTreeMap::new();
    for trait_value in &bundle.content.traits {
        let trait_def: TraitDef = parse_section_value("traits", trait_value)?;
        insert_unique("traits", &mut traits, trait_def.id.clone(), trait_def)?;
    }
    let trait_ids: BTreeSet<&str> = traits.keys().map(String::as_str).collect();

    let mut insights = BTreeMap::new();
    for insight_value in &bundle.content.insights {
        let insight: InsightDef = parse_section_value("insights", insight_value)?;
        if let Some(check_bonus) = &insight.check_bonus {
            if !VALID_ABILITY_IDS.contains(&check_bonus.ability.as_str()) {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "insights".to_string(),
                    id: Some(insight.id.clone()),
                    message: format!("unknown check bonus ability id: {}", check_bonus.ability),
                });
            }
            if !(1..=2).contains(&check_bonus.bonus) {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "insights".to_string(),
                    id: Some(insight.id.clone()),
                    message: "check bonus must be between 1 and 2".to_string(),
                });
            }
        }
        insert_unique("insights", &mut insights, insight.id.clone(), insight)?;
    }
    let insight_ids: BTreeSet<&str> = insights.keys().map(String::as_str).collect();

    let mut encounters = BTreeMap::new();
    for encounter_value in &bundle.content.encounters {
        let encounter = parse_encounter(encounter_value)?;
        validate_event(&encounter)?;
        validate_encounter_locations(&encounter, &location_ids)?;
        validate_encounter_traits(&encounter, &trait_ids)?;
        validate_encounter_insights(&encounter, &insight_ids)?;
        insert_unique(
            "encounters",
            &mut encounters,
            encounter.id.clone(),
            encounter,
        )?;
    }

    let mut endings = BTreeMap::new();
    for ending_value in &bundle.content.endings {
        let ending: EndingDef = parse_section_value("endings", ending_value)?;
        validate_ending_locations(&ending, &location_ids)?;
        insert_unique("endings", &mut endings, ending.id.clone(), ending)?;
    }

    let mut achievements = BTreeMap::new();
    for achievement_value in &bundle.content.achievements {
        let achievement: AchievementDef = parse_section_value("achievements", achievement_value)?;
        insert_unique(
            "achievements",
            &mut achievements,
            achievement.id.clone(),
            achievement,
        )?;
    }

    let mut secrets = BTreeMap::new();
    for secret_value in &bundle.content.secrets {
        let secret: PublicSecretDef = parse_section_value("secrets", secret_value)?;
        insert_unique("secrets", &mut secrets, secret.id.clone(), secret)?;
    }

    if let Some(runtime) = &bundle.runtime {
        if let Some(leveling) = &runtime.leveling {
            if leveling.thresholds.is_empty()
                || leveling
                    .thresholds
                    .windows(2)
                    .any(|window| window[0] >= window[1])
            {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "runtime.leveling".to_string(),
                    id: None,
                    message: "thresholds must be non-empty and strictly increasing".to_string(),
                });
            }
        }
        if let Some(collapse) = &runtime.collapse {
            if collapse.resource_id != "health" {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "runtime.collapse".to_string(),
                    id: None,
                    message: format!(
                        "unsupported collapse resource_id: '{}', only 'health' is supported",
                        collapse.resource_id
                    ),
                });
            }
            if collapse.used_flag.is_empty() {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "runtime.collapse".to_string(),
                    id: None,
                    message: "collapse used_flag cannot be empty".to_string(),
                });
            }
            if !encounters.contains_key(&collapse.encounter_id) {
                return Err(ContentIndexError::InvalidSectionItem {
                    section: "runtime.collapse".to_string(),
                    id: None,
                    message: format!(
                        "collapse encounter_id '{}' not found in encounters",
                        collapse.encounter_id
                    ),
                });
            }
        }
    }

    Ok(ContentIndex {
        locations,
        items,
        encounters,
        endings,
        achievements,
        secrets,
        traits,
        insights,
        runtime: bundle.runtime.clone(),
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
        presentation: raw.presentation,
        event: raw.event,
        conditions: raw.conditions,
        choices,
        repeatable: raw.repeatable,
        weight: raw.weight,
    })
}

fn validate_event(encounter: &EncounterDef) -> Result<(), ContentIndexError> {
    let Some(event) = &encounter.event else {
        return Ok(());
    };
    let fail = |message| ContentIndexError::InvalidEvent {
        encounter_id: encounter.id.clone(),
        message,
    };
    if event.stages.is_empty() {
        return Err(fail("stages cannot be empty".into()));
    }
    let ids: BTreeSet<&str> = event.stages.iter().map(|s| s.id.as_str()).collect();
    if ids.len() != event.stages.len() || ids.contains("") {
        return Err(fail("stage ids must be non-empty and unique".into()));
    }
    if !event.stages.iter().any(|s| s.kind == "story")
        || !event.stages.iter().any(|s| s.kind == "choice")
    {
        return Err(fail(
            "requires at least one story stage and one choice stage".into(),
        ));
    }
    let illustrations = event
        .stages
        .iter()
        .flat_map(|s| &s.blocks)
        .filter(|b| b.kind == "illustration")
        .count();
    if illustrations == 0 {
        return Err(fail(
            "requires at least one illustration block (use a placeholder until art exists)".into(),
        ));
    }
    if illustrations > 3 {
        return Err(fail("supports at most three illustration blocks".into()));
    }
    const BLOCK_KINDS: [&str; 7] = [
        "narration",
        "dialogue",
        "illustration",
        "document",
        "system",
        "cheongirok",
        "result_summary",
    ];
    for (index, stage) in event.stages.iter().enumerate() {
        if !matches!(stage.kind.as_str(), "story" | "choice" | "result") {
            return Err(fail(format!(
                "stage {} has unknown kind '{}'",
                stage.id, stage.kind
            )));
        }
        if stage.kind == "choice" {
            if stage.choices.is_empty() {
                return Err(fail(format!("choice stage {} has no choices", stage.id)));
            }
            if event.stages.get(index + 1).map(|s| s.kind.as_str()) != Some("result") {
                return Err(fail(format!(
                    "choice stage {} must be immediately followed by a result stage",
                    stage.id
                )));
            }
            for choice_ref in &stage.choices {
                if !encounter.choices.iter().any(|c| c.id == choice_ref.id) {
                    return Err(fail(format!(
                        "choice stage {} references unknown encounter choice {}",
                        stage.id, choice_ref.id
                    )));
                }
                if let Some(target) = &choice_ref.next_stage_id {
                    if !ids.contains(target.as_str()) {
                        return Err(fail(format!(
                            "choice {} references unknown next stage {}",
                            choice_ref.id, target
                        )));
                    }
                }
            }
        } else if !stage.choices.is_empty() {
            return Err(fail(format!(
                "non-choice stage {} cannot define choices",
                stage.id
            )));
        }
        if let Some(target) = &stage.next_stage_id {
            if !ids.contains(target.as_str()) {
                return Err(fail(format!(
                    "stage {} references unknown next stage {}",
                    stage.id, target
                )));
            }
        }
        for block in &stage.blocks {
            if !BLOCK_KINDS.contains(&block.kind.as_str()) {
                return Err(fail(format!(
                    "stage {} has unknown block kind '{}'",
                    stage.id, block.kind
                )));
            }
            if let Some(branch) = &block.branch {
                if !matches!(branch.as_str(), "success" | "failure") {
                    return Err(fail(format!(
                        "block in stage {} has unknown branch '{}'; expected success or failure",
                        stage.id, branch
                    )));
                }
            }
            if block.kind == "illustration"
                && (block.visual_id.as_deref().is_none_or(str::is_empty)
                    || block.alt.as_deref().is_none_or(str::is_empty))
            {
                return Err(fail(format!(
                    "illustration block in stage {} requires non-empty visual_id and alt",
                    stage.id
                )));
            }
            if block.kind == "illustration" {
                if block.visual_id.as_deref().unwrap_or("").is_empty()
                    || block.alt.as_deref().unwrap_or("").is_empty()
                {
                    return Err(fail(format!(
                        "illustration in stage {} requires visual_id and alt",
                        stage.id
                    )));
                }
            } else if block.text.as_deref().unwrap_or("").is_empty() {
                return Err(fail(format!(
                    "{} block in stage {} requires text",
                    block.kind, stage.id
                )));
            }
            if block.kind == "dialogue" && block.speaker.as_deref().unwrap_or("").is_empty() {
                return Err(fail(format!(
                    "dialogue in stage {} requires speaker",
                    stage.id
                )));
            }
        }
    }
    Ok(())
}

fn parse_choice(raw: RawChoiceDef) -> Result<ChoiceDef, ContentIndexError> {
    Ok(ChoiceDef {
        id: raw.id,
        label: raw.label,
        conditions: raw.conditions,
        cost: raw.cost,
        outcome: parse_outcome(&raw.outcome)?,
        check: raw.check.map(parse_check).transpose()?,
    })
}

/// The fixed set of valid ability ids. Any `check.ability` outside this set
/// is a content authoring error caught at bundle-indexing time.
const VALID_ABILITY_IDS: [&str; 6] = [
    "logic",
    "empathy",
    "volition",
    "composure",
    "interface",
    "physical",
];

fn parse_check(raw: RawAbilityCheckDef) -> Result<AbilityCheckDef, ContentIndexError> {
    if !VALID_ABILITY_IDS.contains(&raw.ability.as_str()) {
        return Err(ContentIndexError::InvalidSectionItem {
            section: "encounter choices".to_string(),
            id: None,
            message: format!(
                "unknown check ability id: '{}', valid ability ids are: {}",
                raw.ability,
                VALID_ABILITY_IDS.join(", ")
            ),
        });
    }
    Ok(AbilityCheckDef {
        ability: raw.ability,
        difficulty: raw.difficulty,
        success: parse_outcome(&raw.success)?,
        failure: parse_outcome(&raw.failure)?,
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

fn validate_ending_locations(
    ending: &EndingDef,
    location_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    for location_id in &ending.conditions.locations {
        if !location_ids.contains(location_id.as_str()) {
            return Err(ContentIndexError::UnknownEndingLocation {
                ending_id: ending.id.clone(),
                location_id: location_id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_encounter_traits(
    encounter: &EncounterDef,
    trait_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    for choice in &encounter.choices {
        validate_outcome_traits(&encounter.id, &choice.outcome, trait_ids)?;
        if let Some(check) = &choice.check {
            validate_outcome_traits(&encounter.id, &check.success, trait_ids)?;
            validate_outcome_traits(&encounter.id, &check.failure, trait_ids)?;
        }
    }
    Ok(())
}

fn validate_encounter_insights(
    encounter: &EncounterDef,
    insight_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    for choice in &encounter.choices {
        validate_outcome_insights(&encounter.id, &choice.outcome, insight_ids)?;
        if let Some(check) = &choice.check {
            validate_outcome_insights(&encounter.id, &check.success, insight_ids)?;
            validate_outcome_insights(&encounter.id, &check.failure, insight_ids)?;
        }
    }
    Ok(())
}

fn validate_outcome_insights(
    encounter_id: &str,
    outcome: &OutcomeDef,
    insight_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    if let Some(insight_id) = outcome
        .add_insights
        .iter()
        .find(|insight_id| !insight_ids.contains(insight_id.as_str()))
    {
        return Err(ContentIndexError::InvalidSectionItem {
            section: "encounter outcomes".to_string(),
            id: Some(encounter_id.to_string()),
            message: format!("unknown insight id: {insight_id}"),
        });
    }
    Ok(())
}

fn validate_outcome_traits(
    encounter_id: &str,
    outcome: &OutcomeDef,
    trait_ids: &BTreeSet<&str>,
) -> Result<(), ContentIndexError> {
    if let Some(ref trait_id) = outcome.set_trait {
        if !trait_ids.contains(trait_id.as_str()) {
            return Err(ContentIndexError::UnknownTrait {
                encounter_id: encounter_id.to_string(),
                trait_id: trait_id.clone(),
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
