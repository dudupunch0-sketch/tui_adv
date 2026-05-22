use crate::content::ContentIndex;

pub const DEFAULT_START_LOCATION_ID: &str = "dev_desk";

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerState {
    pub health: i32,
    pub sanity: i32,
    pub battery: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub seed: u64,
    pub turn: u32,
    pub location_id: String,
    pub player: PlayerState,
    pub flags: Vec<String>,
    pub clues: Vec<String>,
    pub seen_encounters: Vec<String>,
}

impl GameState {
    pub fn new_printer_scene(seed: u64) -> Self {
        Self {
            seed,
            turn: 0,
            location_id: "printer_area".to_string(),
            player: PlayerState {
                health: 92,
                sanity: 67,
                battery: 41,
            },
            flags: Vec::new(),
            clues: Vec::new(),
            seen_encounters: Vec::new(),
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
            player: PlayerState {
                health: 100,
                sanity: 100,
                battery: 100,
            },
            flags: Vec::new(),
            clues: Vec::new(),
            seen_encounters: Vec::new(),
        })
    }

    pub(crate) fn add_flag_once(&mut self, flag: &str) {
        if !self.flags.iter().any(|existing| existing == flag) {
            self.flags.push(flag.to_string());
        }
    }

    pub(crate) fn add_clue_once(&mut self, clue: &str) {
        if !self.clues.iter().any(|existing| existing == clue) {
            self.clues.push(clue.to_string());
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
}
