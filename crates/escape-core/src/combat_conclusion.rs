use crate::{CombatResolutionResult, CombatSide, CombatSimulationParticipant};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatTerminationPolicy {
    pub max_ticks: u32,
    pub conclude_on_max_ticks: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatConclusionRequest {
    pub resolution: CombatResolutionResult,
    pub participants: Vec<CombatSimulationParticipant>,
    pub policy: CombatTerminationPolicy,
    /// tick 한 칸의 길이(ms). `CombatResolutionResult`가 입력 `CombatSimulationConfig`를
    /// 보관하지 않으므로 호출자가 전달한다. 0은 `InvalidTickMillis`로 거부한다.
    pub tick_millis: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatConclusionOutcome {
    InProgress,
    AllyVictory,
    EnemyVictory,
    MutualDefeat,
    Stalemate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatConclusionReason {
    NoTerminalCondition,
    AllEnemiesDefeated,
    AllAlliesDefeated,
    BothSidesDefeated,
    MaxTicksReached,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatConclusionReport {
    pub resolution_fingerprint: String,
    pub outcome: CombatConclusionOutcome,
    pub reason: CombatConclusionReason,
    pub decisive_tick: Option<u32>,
    pub active_allies: u32,
    pub active_enemies: u32,
    pub survivor_ids: Vec<String>,
    pub defeated_ids: Vec<String>,
    pub removed_combat_effect_ids: Vec<String>,
    pub retained_effect_ids: Vec<String>,
    /// 결착까지의 전투 시간. tick 수 × tick_millis.
    #[serde(default)]
    pub duration_millis: u64,
    pub fingerprint: String,
}

pub fn conclude(
    request: CombatConclusionRequest,
) -> Result<CombatConclusionReport, CombatConclusionError> {
    if request.tick_millis == 0 {
        return Err(CombatConclusionError::InvalidTickMillis(
            request.tick_millis,
        ));
    }
    if request.policy.max_ticks == 0 {
        return Err(CombatConclusionError::InvalidPolicy);
    }
    let mut participants = BTreeMap::new();
    for participant in &request.participants {
        if participant.id.trim().is_empty()
            || participants
                .insert(participant.id.clone(), participant)
                .is_some()
        {
            return Err(CombatConclusionError::DuplicateParticipant(
                participant.id.clone(),
            ));
        }
    }
    let mut state = BTreeMap::new();
    for combatant in &request.resolution.state.combatants {
        if state.insert(combatant.id.clone(), combatant).is_some() {
            return Err(CombatConclusionError::DuplicateState(combatant.id.clone()));
        }
    }
    if participants.len() != state.len() || participants.keys().any(|id| !state.contains_key(id)) {
        return Err(CombatConclusionError::ParticipantStateMismatch);
    }
    let active: Vec<_> = participants
        .values()
        .filter(|p| p.active)
        .map(|p| *p)
        .collect();
    let allies: Vec<_> = active
        .iter()
        .filter(|p| p.side == CombatSide::Ally)
        .map(|p| *p)
        .collect();
    let enemies: Vec<_> = active
        .iter()
        .filter(|p| p.side == CombatSide::Enemy)
        .map(|p| *p)
        .collect();
    if allies.is_empty() || enemies.is_empty() {
        return Err(CombatConclusionError::EmptyActiveSide);
    }
    let last_tick = request.resolution.frames.last().map(|f| f.tick);
    if request
        .resolution
        .frames
        .iter()
        .any(|f| f.tick > request.policy.max_ticks)
    {
        return Err(CombatConclusionError::FrameExceedsPolicy);
    }
    let defeated = |side: &[&CombatSimulationParticipant]| {
        side.iter()
            .all(|p| state[p.id.as_str()].current_health_hundredths == 0)
    };
    let allies_defeated = defeated(&allies);
    let enemies_defeated = defeated(&enemies);
    let (outcome, reason) = if allies_defeated && enemies_defeated {
        (
            CombatConclusionOutcome::MutualDefeat,
            CombatConclusionReason::BothSidesDefeated,
        )
    } else if enemies_defeated {
        (
            CombatConclusionOutcome::AllyVictory,
            CombatConclusionReason::AllEnemiesDefeated,
        )
    } else if allies_defeated {
        (
            CombatConclusionOutcome::EnemyVictory,
            CombatConclusionReason::AllAlliesDefeated,
        )
    } else if last_tick == Some(request.policy.max_ticks) && request.policy.conclude_on_max_ticks {
        (
            CombatConclusionOutcome::Stalemate,
            CombatConclusionReason::MaxTicksReached,
        )
    } else {
        (
            CombatConclusionOutcome::InProgress,
            CombatConclusionReason::NoTerminalCondition,
        )
    };
    let terminal = !matches!(outcome, CombatConclusionOutcome::InProgress);
    let mut survivor_ids = Vec::new();
    let mut defeated_ids = Vec::new();
    for p in active {
        if state[p.id.as_str()].current_health_hundredths > 0 {
            survivor_ids.push(p.id.clone());
        } else {
            defeated_ids.push(p.id.clone());
        }
    }
    let mut removed = BTreeSet::new();
    let mut retained = BTreeSet::new();
    for effect in &request.resolution.state.active_effects {
        if effect.combat_only {
            removed.insert(effect.definition_id.clone());
        } else {
            retained.insert(effect.definition_id.clone());
        }
    }
    let decisive_tick = terminal.then_some(last_tick).flatten();
    let duration_millis = match decisive_tick {
        Some(t) => (u64::from(t) + 1) * u64::from(request.tick_millis),
        None => (request.resolution.frames.len() as u64) * u64::from(request.tick_millis),
    };
    let mut report = CombatConclusionReport {
        resolution_fingerprint: request.resolution.fingerprint.clone(),
        outcome,
        reason,
        decisive_tick,
        active_allies: allies.len() as u32,
        active_enemies: enemies.len() as u32,
        survivor_ids,
        defeated_ids,
        removed_combat_effect_ids: removed.into_iter().collect(),
        retained_effect_ids: retained.into_iter().collect(),
        duration_millis,
        fingerprint: String::new(),
    };
    report.fingerprint = fingerprint(&report);
    Ok(report)
}

fn fingerprint<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatConclusionError {
    InvalidPolicy,
    DuplicateParticipant(String),
    DuplicateState(String),
    ParticipantStateMismatch,
    EmptyActiveSide,
    FrameExceedsPolicy,
    InvalidTickMillis(u32),
}
impl std::fmt::Display for CombatConclusionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatConclusionError {}
