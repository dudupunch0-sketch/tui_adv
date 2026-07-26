use crate::{
    CombatRngNamespace, CombatSimulation, CombatSimulationError, CombatSimulationInput,
    CombatTickFrame,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatRunMode {
    Actual,
    Forecast,
    Retry,
    Auto,
    Fast,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatPresentationSpeed {
    OneX,
    TwoX,
    Instant,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatLogImportance {
    Routine,
    Important,
    Decisive,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatLogTag {
    MoveIntent,
    TargetSelection,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatLogEvent {
    pub tick: u32,
    pub sequence: u32,
    pub tag: CombatLogTag,
    pub importance: CombatLogImportance,
    pub actor_id: String,
    pub target_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatExecutionRequest {
    pub input: CombatSimulationInput,
    pub mode: CombatRunMode,
    pub presentation: CombatPresentationSpeed,
    pub ticks: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatExecutionResult {
    pub mode: CombatRunMode,
    pub presentation: CombatPresentationSpeed,
    pub effective_seed: u64,
    pub namespace: CombatRngNamespace,
    pub frames: Vec<CombatTickFrame>,
    pub full_log: Vec<CombatLogEvent>,
    pub core_log: Vec<CombatLogEvent>,
    pub fingerprint: String,
}

pub fn execute(
    request: CombatExecutionRequest,
) -> Result<CombatExecutionResult, CombatExecutionError> {
    if request.ticks == 0 {
        return Err(CombatExecutionError::ZeroTicks);
    }
    let namespace = if request.mode == CombatRunMode::Forecast {
        CombatRngNamespace::ForecastEnsemble
    } else {
        CombatRngNamespace::ActualCombat
    };
    let effective_seed = if namespace == CombatRngNamespace::ForecastEnsemble {
        request
            .input
            .manifest
            .derived_seed(namespace)
            .map_err(|_| CombatExecutionError::InvalidInput)?
    } else {
        request.input.seed
    };
    let mut input = request.input.clone();
    input.seed = effective_seed;
    let mut simulation = CombatSimulation::new(input)?;
    let setup_fingerprint = simulation.fingerprint()?;
    let frames = simulation.run_ticks(request.ticks)?;
    let mut full_log = Vec::new();
    for frame in &frames {
        let mut sequence = 0u32;
        for intent in &frame.moves {
            full_log.push(CombatLogEvent {
                tick: frame.tick,
                sequence,
                tag: CombatLogTag::TargetSelection,
                importance: CombatLogImportance::Routine,
                actor_id: intent.actor_id.clone(),
                target_id: intent.target_id.clone(),
            });
            sequence += 1;
            full_log.push(CombatLogEvent {
                tick: frame.tick,
                sequence,
                tag: CombatLogTag::MoveIntent,
                importance: match intent.mode {
                    crate::CombatMoveMode::Hold => CombatLogImportance::Routine,
                    crate::CombatMoveMode::Advance => CombatLogImportance::Important,
                    crate::CombatMoveMode::Retreat => CombatLogImportance::Decisive,
                },
                actor_id: intent.actor_id.clone(),
                target_id: intent.target_id.clone(),
            });
            sequence += 1;
        }
    }
    let core_log = full_log
        .iter()
        .filter(|event| event.importance >= CombatLogImportance::Important)
        .cloned()
        .collect();
    let fingerprint = stable_fingerprint(&(
        namespace,
        effective_seed,
        setup_fingerprint,
        &frames,
        &full_log,
    ));
    Ok(CombatExecutionResult {
        mode: request.mode,
        presentation: request.presentation,
        effective_seed,
        namespace,
        frames,
        full_log,
        core_log,
        fingerprint,
    })
}

fn stable_fingerprint<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    format!("{:016x}", fnv(&bytes))
}
fn fnv(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatExecutionError {
    ZeroTicks,
    InvalidInput,
    Simulation(CombatSimulationError),
}
impl From<CombatSimulationError> for CombatExecutionError {
    fn from(error: CombatSimulationError) -> Self {
        Self::Simulation(error)
    }
}
impl std::fmt::Display for CombatExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatExecutionError {}
