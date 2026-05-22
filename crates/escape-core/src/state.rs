use crate::content::ContentIndex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DEFAULT_START_LOCATION_ID: &str = "dev_desk";
pub const DEFAULT_DISASTER_TYPE: &str = "unknown_isolation";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewGameError {
    UnknownStartLocation(String),
}

impl std::fmt::Display for NewGameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewGameError::UnknownStartLocation(location_id) => {
                write!(formatter, "unknown start location: {location_id}")
            }
        }
    }
}

impl std::error::Error for NewGameError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub health: i32,
    pub sanity: i32,
    pub battery: i32,
    #[serde(default)]
    pub hunger: i32,
    #[serde(default)]
    pub thirst: i32,
    #[serde(default = "default_abilities")]
    pub abilities: BTreeMap<String, i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameHistoryEntry {
    pub kind: String,
    pub text: String,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub seed: u64,
    pub turn: u32,
    pub location_id: String,
    #[serde(default = "default_disaster_type")]
    pub disaster_type: String,
    pub danger: i32,
    pub player: PlayerState,
    #[serde(default)]
    pub inventory: Vec<String>,
    pub flags: Vec<String>,
    pub clues: Vec<String>,
    pub seen_encounters: Vec<String>,
    #[serde(default)]
    pub unlocked_achievements: Vec<String>,
    pub history: Vec<GameHistoryEntry>,
}

impl GameState {
    pub fn new_printer_scene(seed: u64) -> Self {
        Self {
            seed,
            turn: 0,
            location_id: "printer_area".to_string(),
            disaster_type: DEFAULT_DISASTER_TYPE.to_string(),
            danger: 0,
            player: PlayerState {
                health: 92,
                sanity: 67,
                battery: 41,
                hunger: 0,
                thirst: 0,
                abilities: default_abilities(),
            },
            inventory: Vec::new(),
            flags: Vec::new(),
            clues: Vec::new(),
            seen_encounters: Vec::new(),
            unlocked_achievements: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn new_from_content(seed: u64, content: &ContentIndex) -> Result<Self, NewGameError> {
        Self::new_from_content_at(seed, content, DEFAULT_START_LOCATION_ID)
    }

    pub fn new_from_content_at(
        seed: u64,
        content: &ContentIndex,
        start_location_id: &str,
    ) -> Result<Self, NewGameError> {
        if content.location(start_location_id).is_none() {
            return Err(NewGameError::UnknownStartLocation(
                start_location_id.to_string(),
            ));
        }

        Ok(Self {
            seed,
            turn: 0,
            location_id: start_location_id.to_string(),
            disaster_type: DEFAULT_DISASTER_TYPE.to_string(),
            danger: 0,
            player: PlayerState {
                health: 100,
                sanity: 100,
                battery: 100,
                hunger: 0,
                thirst: 0,
                abilities: default_abilities(),
            },
            inventory: Vec::new(),
            flags: Vec::new(),
            clues: Vec::new(),
            seen_encounters: Vec::new(),
            unlocked_achievements: Vec::new(),
            history: Vec::new(),
        })
    }

    pub(crate) fn add_flag_once(&mut self, flag: &str) {
        if !self.flags.iter().any(|existing| existing == flag) {
            self.flags.push(flag.to_string());
        }
    }

    pub(crate) fn remove_flag(&mut self, flag: &str) {
        self.flags.retain(|existing| existing != flag);
    }

    pub(crate) fn add_clue_once(&mut self, clue: &str) {
        if !self.clues.iter().any(|existing| existing == clue) {
            self.clues.push(clue.to_string());
        }
    }

    pub(crate) fn add_inventory_once(&mut self, item: &str) {
        if !self.inventory.iter().any(|existing| existing == item) {
            self.inventory.push(item.to_string());
        }
    }

    pub(crate) fn remove_inventory_item(&mut self, item: &str) {
        if let Some(index) = self.inventory.iter().position(|existing| existing == item) {
            self.inventory.remove(index);
        }
    }

    pub(crate) fn add_seen_encounter_once(&mut self, encounter_id: &str) {
        if !self
            .seen_encounters
            .iter()
            .any(|existing| existing == encounter_id)
        {
            self.seen_encounters.push(encounter_id.to_string());
        }
    }

    pub(crate) fn add_unlocked_achievement_once(&mut self, achievement_id: &str) -> bool {
        if self
            .unlocked_achievements
            .iter()
            .any(|existing| existing == achievement_id)
        {
            false
        } else {
            self.unlocked_achievements.push(achievement_id.to_string());
            true
        }
    }

    pub(crate) fn add_history_entry(&mut self, kind: &str, text: &str, source_id: Option<&str>) {
        self.history.push(GameHistoryEntry {
            kind: kind.to_string(),
            text: text.to_string(),
            source_id: source_id.map(str::to_string),
        });
    }
}

pub fn default_abilities() -> BTreeMap<String, i32> {
    [
        ("logic", 2),
        ("empathy", 2),
        ("volition", 2),
        ("composure", 2),
        ("interface", 2),
        ("physical", 2),
    ]
    .into_iter()
    .map(|(ability, value)| (ability.to_string(), value))
    .collect()
}

fn default_disaster_type() -> String {
    DEFAULT_DISASTER_TYPE.to_string()
}
