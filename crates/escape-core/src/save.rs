use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub const SAVE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveEnvelope {
    pub schema_version: u32,
    pub state: GameState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveError {
    UnsupportedSchemaVersion(u32),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::UnsupportedSchemaVersion(version) => {
                write!(formatter, "unsupported save schema version: {version}")
            }
        }
    }
}

impl std::error::Error for SaveError {}

pub fn save_state(state: &GameState) -> SaveEnvelope {
    SaveEnvelope {
        schema_version: SAVE_SCHEMA_VERSION,
        state: state.clone(),
    }
}

pub fn load_state(envelope: &SaveEnvelope) -> Result<GameState, SaveError> {
    if envelope.schema_version != SAVE_SCHEMA_VERSION {
        return Err(SaveError::UnsupportedSchemaVersion(envelope.schema_version));
    }

    Ok(envelope.state.clone())
}
