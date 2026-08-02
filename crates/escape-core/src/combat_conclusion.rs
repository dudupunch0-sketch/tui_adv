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
    /// 피해가 하나도 없으면 `None` (발생하지 않은 항목은 숨긴다).
    #[serde(default)]
    pub top_damage_dealt_id: Option<String>,
    #[serde(default)]
    pub top_damage_taken_id: Option<String>,
    pub fingerprint: String,
}

/// 진영 전멸 조건: 그 진영의 활성 전투원 전원의 체력이 0이면 전멸이다.
///
/// `conclude`의 최종 결착 판정과 `combat_resolution::resolve`의 tick 단위 조기
/// 종료 판정(WP2/WP3, `fable_combat_early_conclusion_step1_2608022130.md`)이
/// 함께 쓰는 유일한 정의다. 규칙은 이 함수 하나에만 있다 — 두 곳에 같은
/// 조건을 베끼면 갈라진다.
pub fn side_all_defeated<'a>(
    side: impl IntoIterator<Item = &'a CombatSimulationParticipant>,
    current_health_hundredths: impl Fn(&str) -> i64,
) -> bool {
    side.into_iter()
        .all(|p| current_health_hundredths(p.id.as_str()) == 0)
}

pub fn conclude(
    request: CombatConclusionRequest,
) -> Result<CombatConclusionReport, CombatConclusionError> {
    let tick_millis = request
        .resolution
        .execution
        .provenance
        .as_ref()
        .map(|p| p.tick_millis)
        .filter(|millis| *millis > 0)
        .ok_or(CombatConclusionError::MissingProvenance)?;
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
    let health_of = |id: &str| state[id].current_health_hundredths;
    let allies_defeated = side_all_defeated(allies.iter().copied(), health_of);
    let enemies_defeated = side_all_defeated(enemies.iter().copied(), health_of);
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
        Some(t) => (u64::from(t) + 1) * u64::from(tick_millis),
        None => (request.resolution.frames.len() as u64) * u64::from(tick_millis),
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
    let mut first_defeated_at: BTreeMap<String, usize> = BTreeMap::new();
    for (idx, f) in request.resolution.frames.iter().enumerate() {
        for c in &f.combatants {
            if c.current_health_hundredths <= 0 && !first_defeated_at.contains_key(&c.id) {
                first_defeated_at.insert(c.id.clone(), idx);
            }
        }
    }
    let mut kills: BTreeMap<String, u32> = BTreeMap::new();
    for (target_id, idx) in &first_defeated_at {
        if let Some(f) = request.resolution.frames.get(*idx) {
            if let Some(o) = f
                .outcomes
                .iter()
                .rev()
                .find(|o| o.target_id == *target_id && o.hit && o.damage_hundredths > 0)
            {
                *kills.entry(o.actor_id.clone()).or_insert(0) += 1;
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
            kills: kills.get(id).copied().unwrap_or(0),
            incapacitated: health_of(id) <= 0,
        })
        .collect();
    let top_damage_dealt_id = top_id_by(&combatants, |c| c.damage_dealt_hundredths);
    let top_damage_taken_id = top_id_by(&combatants, |c| c.damage_taken_hundredths);
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
        top_damage_dealt_id,
        top_damage_taken_id,
        fingerprint: String::new(),
    };
    report.fingerprint = fingerprint(&report);
    Ok(report)
}

/// 최대값을 가진 id를 반환한다 (id 오름차순 입력 가정, 동점은 최초 등장 = 최소 id).
/// 최대값이 0 이하이면 발생하지 않은 항목이므로 `None`.
fn top_id_by(
    combatants: &[CombatCombatantReport],
    value: impl Fn(&CombatCombatantReport) -> i64,
) -> Option<String> {
    let mut best: Option<(&str, i64)> = None;
    for c in combatants {
        let v = value(c);
        if best.map_or(true, |(_, b)| v > b) {
            best = Some((c.id.as_str(), v));
        }
    }
    best.filter(|(_, v)| *v > 0).map(|(id, _)| id.to_string())
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
    /// `resolution.execution.provenance`가 없거나(구 JSON) `tick_millis`가 0이면
    /// `duration_millis`를 지어내지 않고 이 에러를 낸다.
    MissingProvenance,
}
impl std::fmt::Display for CombatConclusionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatConclusionError {}
