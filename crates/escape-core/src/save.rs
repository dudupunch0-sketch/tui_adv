use crate::combat_runtime::CombatRuntimeCheckpoint;
use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub const SAVE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveEnvelope {
    pub schema_version: u32,
    pub state: GameState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combat_checkpoint: Option<CombatRuntimeCheckpoint>,
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
        combat_checkpoint: None,
    }
}

pub fn load_state(envelope: &SaveEnvelope) -> Result<GameState, SaveError> {
    if envelope.schema_version != SAVE_SCHEMA_VERSION {
        return Err(SaveError::UnsupportedSchemaVersion(envelope.schema_version));
    }

    Ok(envelope.state.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::new_game;

    #[test]
    fn save_state_omits_empty_combat_checkpoint_and_old_json_loads() {
        let envelope = save_state(&new_game(7));
        let value = serde_json::to_value(&envelope).unwrap();
        assert!(value.get("combat_checkpoint").is_none());

        let old_json = serde_json::json!({
            "schema_version": SAVE_SCHEMA_VERSION,
            "state": serde_json::to_value(&envelope.state).unwrap(),
        });
        let loaded: SaveEnvelope = serde_json::from_value(old_json).unwrap();
        assert_eq!(loaded.combat_checkpoint, None);
    }
}
