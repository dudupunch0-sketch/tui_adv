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

/// 캐릭터 한 명의 전투 기록. 정본 13의 "캐릭터별 상세 기록".
/// 치유량은 파이프라인에 회복 개념이 없어 아직 필드를 두지 않는다.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatCombatantReport {
    pub id: String,
    pub damage_dealt_hundredths: i64,
    pub damage_taken_hundredths: i64,
    pub kills: u32,
    pub incapacitated: bool,
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
    /// id 오름차순.
    #[serde(default)]
    pub combatants: Vec<CombatCombatantReport>,
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
    let mut damage_dealt: BTreeMap<String, i64> = BTreeMap::new();
    let mut damage_taken: BTreeMap<String, i64> = BTreeMap::new();
    for f in &request.resolution.frames {
        for o in &f.outcomes {
            if o.hit && o.damage_hundredths > 0 {
                *damage_dealt.entry(o.actor_id.clone()).or_insert(0) += o.damage_hundredths;
                *damage_taken.entry(o.target_id.clone()).or_insert(0) += o.damage_hundredths;
            }
        }
    }
    let last_frame_snapshot = request
        .resolution
        .frames
        .last()
        .map(|f| &f.combatants)
        .filter(|c| !c.is_empty());
    let health_of = |id: &str| -> i64 {
        if let Some(snapshot) = last_frame_snapshot {
            if let Some(c) = snapshot.iter().find(|c| c.id.as_str() == id) {
                return c.current_health_hundredths;
            }
        }
        state
            .get(id)
            .map(|c| c.current_health_hundredths)
            .unwrap_or(0)
    };
    let combatants: Vec<CombatCombatantReport> = participants
        .keys()
        .map(|id| CombatCombatantReport {
            id: id.clone(),
            damage_dealt_hundredths: damage_dealt.get(id).copied().unwrap_or(0),
            damage_taken_hundredths: damage_taken.get(id).copied().unwrap_or(0),
            kills: 0,
            incapacitated: health_of(id) <= 0,
        })
        .collect();
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
        combatants,
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
