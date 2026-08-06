use crate::combat_contract::ensure_supported_simulation_version;
use crate::{CombatManifest, CombatState};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSimulationConfig {
    pub tick_millis: u32,
    pub max_ticks: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatSide {
    Ally,
    Enemy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatPosition {
    pub x: i32,
    pub y: i32,
}
impl CombatPosition {
    pub fn distance_squared(self, other: Self) -> Result<i64, CombatSimulationError> {
        let dx = i64::from(self.x) - i64::from(other.x);
        let dy = i64::from(self.y) - i64::from(other.y);
        dx.checked_mul(dx)
            .and_then(|x| dy.checked_mul(dy).and_then(|y| x.checked_add(y)))
            .ok_or(CombatSimulationError::Overflow)
    }
    pub fn in_range(self, other: Self, range: i32) -> Result<bool, CombatSimulationError> {
        if range < 0 {
            return Err(CombatSimulationError::InvalidRange);
        }
        Ok(self.distance_squared(other)? <= i64::from(range).pow(2))
    }
    pub fn overlaps(self, other: Self, radius: i32) -> Result<bool, CombatSimulationError> {
        self.in_range(other, radius)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatFacing {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatRoleWeights {
    pub preferred_distance: i32,
    pub aggression: i32,
    pub formation_maintenance: i32,
    pub pursuit_range: i32,
    pub protect_priority: i32,
    pub target_priority: i32,
    pub risk_tolerance: i32,
    pub ability_priority: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatRolePreset {
    pub id: String,
    pub weights: CombatRoleWeights,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatTargetPreference {
    pub target_id: String,
    pub priority: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatTargetFallback {
    Nearest,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatTargetPolicy {
    pub id: String,
    pub preferences: Vec<CombatTargetPreference>,
    pub fallback: CombatTargetFallback,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSimulationParticipant {
    pub id: String,
    pub side: CombatSide,
    pub position: CombatPosition,
    pub facing: CombatFacing,
    pub speed_per_tick: i32,
    pub collision_radius: i32,
    pub attack_range: i32,
    pub support_range: i32,
    pub role_id: String,
    pub target_policy_id: Option<String>,
    pub active: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatMoveIntent {
    pub actor_id: String,
    pub target_id: Option<String>,
    pub from: CombatPosition,
    pub to: CombatPosition,
    pub mode: CombatMoveMode,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatMoveMode {
    Hold,
    Advance,
    Retreat,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatTickFrame {
    pub tick: u32,
    pub moves: Vec<CombatMoveIntent>,
    pub positions: BTreeMap<String, CombatPosition>,
    pub fingerprint: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSimulationInput {
    pub manifest: CombatManifest,
    pub state: CombatState,
    pub seed: u64,
    pub config: CombatSimulationConfig,
    pub participants: Vec<CombatSimulationParticipant>,
    pub roles: Vec<CombatRolePreset>,
    pub policies: Vec<CombatTargetPolicy>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatSimulation {
    input: CombatSimulationInput,
    tick: u32,
    participants: BTreeMap<String, CombatSimulationParticipant>,
    roles: BTreeMap<String, CombatRolePreset>,
    policies: BTreeMap<String, CombatTargetPolicy>,
}
impl CombatSimulation {
    pub fn participants(&self) -> impl Iterator<Item = &CombatSimulationParticipant> {
        self.participants.values()
    }
    pub fn fingerprint(&self) -> Result<String, CombatSimulationError> {
        let mut participants = self.input.participants.clone();
        participants.sort_by(|a, b| a.id.cmp(&b.id));
        let mut roles = self.input.roles.clone();
        roles.sort_by(|a, b| a.id.cmp(&b.id));
        let mut policies = self.input.policies.clone();
        policies.sort_by(|a, b| a.id.cmp(&b.id));
        let manifest = self
            .input
            .manifest
            .canonical_json()
            .map_err(|_| CombatSimulationError::InvalidReference)?;
        let state = self
            .input
            .state
            .canonical_json()
            .map_err(|_| CombatSimulationError::InvalidReference)?;
        fingerprint(&(
            self.input.seed,
            self.input.manifest.simulation_version.as_str(),
            &self.input.config,
            manifest,
            state,
            &participants,
            &roles,
            &policies,
        ))
    }
    pub fn new(input: CombatSimulationInput) -> Result<Self, CombatSimulationError> {
        if input.config.tick_millis == 0 || input.config.max_ticks == 0 {
            return Err(CombatSimulationError::InvalidConfig);
        }
        ensure_supported_simulation_version(&input.manifest.simulation_version).map_err(|_| {
            CombatSimulationError::UnsupportedSimulationVersion(
                input.manifest.simulation_version.as_str().to_string(),
            )
        })?;
        input
            .manifest
            .validate()
            .map_err(|_| CombatSimulationError::InvalidReference)?;
        input
            .state
            .validate()
            .map_err(|_| CombatSimulationError::InvalidReference)?;
        let mut all_ids = BTreeSet::new();
        for p in &input.participants {
            ensure_id(&p.id)?;
            if !all_ids.insert(p.id.clone()) {
                return Err(CombatSimulationError::DuplicateId(p.id.clone()));
            }
        }
        let mut participants = BTreeMap::new();
        for p in input.participants.iter().filter(|p| p.active) {
            ensure_id(&p.id)?;
            if p.speed_per_tick <= 0
                || p.collision_radius <= 0
                || p.attack_range <= 0
                || p.support_range <= 0
            {
                return Err(CombatSimulationError::InvalidParticipant(p.id.clone()));
            }
            if p.facing.x == 0 && p.facing.y == 0 {
                return Err(CombatSimulationError::InvalidFacing(p.id.clone()));
            }
            if participants.insert(p.id.clone(), p.clone()).is_some() {
                return Err(CombatSimulationError::DuplicateId(p.id.clone()));
            }
        }
        let allies = participants
            .values()
            .filter(|p| p.side == CombatSide::Ally)
            .count();
        let enemies = participants
            .values()
            .filter(|p| p.side == CombatSide::Enemy)
            .count();
        if allies > 4 || enemies > 8 {
            return Err(CombatSimulationError::ParticipantLimit);
        }
        let mut roles = BTreeMap::new();
        for role in &input.roles {
            ensure_id(&role.id)?;
            if role.weights.preferred_distance < 0 || role.weights.pursuit_range < 0 {
                return Err(CombatSimulationError::InvalidRole(role.id.clone()));
            }
            if roles.insert(role.id.clone(), role.clone()).is_some() {
                return Err(CombatSimulationError::DuplicateId(role.id.clone()));
            }
        }
        let mut policies = BTreeMap::new();
        for policy in &input.policies {
            ensure_id(&policy.id)?;
            if policies.insert(policy.id.clone(), policy.clone()).is_some() {
                return Err(CombatSimulationError::DuplicateId(policy.id.clone()));
            }
            let mut pref_ids = BTreeSet::new();
            for pref in &policy.preferences {
                ensure_id(&pref.target_id)?;
                if !pref_ids.insert(pref.target_id.clone()) {
                    return Err(CombatSimulationError::DuplicateId(pref.target_id.clone()));
                }
            }
        }
        for p in participants.values() {
            if !roles.contains_key(&p.role_id) {
                return Err(CombatSimulationError::MissingReference(p.role_id.clone()));
            }
            if let Some(id) = &p.target_policy_id {
                if !policies.contains_key(id) {
                    return Err(CombatSimulationError::MissingReference(id.clone()));
                }
            }
        }
        Ok(Self {
            input,
            tick: 0,
            participants,
            roles,
            policies,
        })
    }
    pub fn select_target(
        &self,
        actor: &CombatSimulationParticipant,
    ) -> Result<Option<String>, CombatSimulationError> {
        let policy = actor
            .target_policy_id
            .as_ref()
            .and_then(|id| self.policies.get(id));
        let valid = |id: &str| {
            self.participants
                .get(id)
                .is_some_and(|p| p.active && p.side != actor.side)
        };
        if let Some(policy) = policy {
            if let Some(target) = policy
                .preferences
                .iter()
                .filter(|p| valid(&p.target_id))
                .max_by(|a, b| {
                    a.priority
                        .cmp(&b.priority)
                        .then_with(|| {
                            let da = self.participants[&a.target_id]
                                .position
                                .distance_squared(actor.position)
                                .unwrap_or(i64::MAX);
                            let db = self.participants[&b.target_id]
                                .position
                                .distance_squared(actor.position)
                                .unwrap_or(i64::MAX);
                            db.cmp(&da)
                        })
                        .then_with(|| b.target_id.cmp(&a.target_id))
                })
            {
                return Ok(Some(target.target_id.clone()));
            }
        }
        Ok(self
            .participants
            .values()
            .filter(|p| valid(&p.id))
            .filter_map(|p| {
                p.position
                    .distance_squared(actor.position)
                    .ok()
                    .map(|d| (d, p.id.clone()))
            })
            .min_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
            .map(|(_, id)| id))
    }
    pub fn advance_tick(&mut self) -> Result<CombatTickFrame, CombatSimulationError> {
        if self.tick >= self.input.config.max_ticks {
            return Err(CombatSimulationError::MaxTicksExceeded);
        }
        let snapshot = self.participants.clone();
        let mut moves = Vec::new();
        for actor in snapshot.values() {
            let target_id = self.select_target(actor)?;
            let target = target_id.as_ref().and_then(|id| snapshot.get(id));
            let role = &self.roles[&actor.role_id];
            let (to, mode) = target
                .map(|target| {
                    let d = actor
                        .position
                        .distance_squared(target.position)
                        .unwrap_or(i64::MAX);
                    let preferred = i64::from(role.weights.preferred_distance.max(0));
                    let step = actor.speed_per_tick;
                    if d > preferred * preferred {
                        Ok((
                            step_toward(actor.position, target.position, step)?,
                            CombatMoveMode::Advance,
                        ))
                    } else if d < preferred * preferred && role.weights.aggression < 0 {
                        Ok((
                            step_away(actor.position, target.position, step)?,
                            CombatMoveMode::Retreat,
                        ))
                    } else {
                        Ok((actor.position, CombatMoveMode::Hold))
                    }
                })
                .transpose()?
                .unwrap_or((actor.position, CombatMoveMode::Hold));
            moves.push(CombatMoveIntent {
                actor_id: actor.id.clone(),
                target_id,
                from: actor.position,
                to,
                mode,
            });
        }
        moves.sort_by(|a, b| a.actor_id.cmp(&b.actor_id));
        for intent in &moves {
            if let Some(p) = self.participants.get_mut(&intent.actor_id) {
                p.position = intent.to;
            }
        }
        self.tick += 1;
        let positions = self
            .participants
            .values()
            .map(|p| (p.id.clone(), p.position))
            .collect();
        let mut frame = CombatTickFrame {
            tick: self.tick,
            moves,
            positions,
            fingerprint: String::new(),
        };
        frame.fingerprint = fingerprint(&frame)?;
        Ok(frame)
    }
    pub fn run_ticks(&mut self, count: u32) -> Result<Vec<CombatTickFrame>, CombatSimulationError> {
        if count > self.input.config.max_ticks.saturating_sub(self.tick) {
            return Err(CombatSimulationError::MaxTicksExceeded);
        }
        (0..count).map(|_| self.advance_tick()).collect()
    }
}
fn step_toward(
    from: CombatPosition,
    to: CombatPosition,
    step: i32,
) -> Result<CombatPosition, CombatSimulationError> {
    let dx =
        to.x.checked_sub(from.x)
            .ok_or(CombatSimulationError::Overflow)?;
    let dy =
        to.y.checked_sub(from.y)
            .ok_or(CombatSimulationError::Overflow)?;
    let ax = dx.checked_abs().ok_or(CombatSimulationError::Overflow)?;
    let ay = dy.checked_abs().ok_or(CombatSimulationError::Overflow)?;
    let dominant = ax.max(ay);
    if dominant == 0 {
        return Ok(from);
    }
    let x_step = step
        .checked_mul(ax)
        .ok_or(CombatSimulationError::Overflow)?
        .checked_div(dominant)
        .ok_or(CombatSimulationError::Overflow)?;
    let y_step = step
        .checked_mul(ay)
        .ok_or(CombatSimulationError::Overflow)?
        .checked_div(dominant)
        .ok_or(CombatSimulationError::Overflow)?;
    Ok(CombatPosition {
        x: from
            .x
            .checked_add(
                dx.signum()
                    .checked_mul(x_step)
                    .ok_or(CombatSimulationError::Overflow)?,
            )
            .ok_or(CombatSimulationError::Overflow)?,
        y: from
            .y
            .checked_add(
                dy.signum()
                    .checked_mul(y_step)
                    .ok_or(CombatSimulationError::Overflow)?,
            )
            .ok_or(CombatSimulationError::Overflow)?,
    })
}
fn step_away(
    from: CombatPosition,
    to: CombatPosition,
    step: i32,
) -> Result<CombatPosition, CombatSimulationError> {
    let dx = from
        .x
        .checked_sub(to.x)
        .ok_or(CombatSimulationError::Overflow)?;
    let dy = from
        .y
        .checked_sub(to.y)
        .ok_or(CombatSimulationError::Overflow)?;
    let ax = dx.checked_abs().ok_or(CombatSimulationError::Overflow)?;
    let ay = dy.checked_abs().ok_or(CombatSimulationError::Overflow)?;
    let dominant = ax.max(ay);
    if dominant == 0 {
        return Ok(from);
    }
    let x_step = step
        .checked_mul(ax)
        .ok_or(CombatSimulationError::Overflow)?
        .checked_div(dominant)
        .ok_or(CombatSimulationError::Overflow)?;
    let y_step = step
        .checked_mul(ay)
        .ok_or(CombatSimulationError::Overflow)?
        .checked_div(dominant)
        .ok_or(CombatSimulationError::Overflow)?;
    let target = CombatPosition {
        x: from
            .x
            .checked_add(
                dx.signum()
                    .checked_mul(x_step)
                    .ok_or(CombatSimulationError::Overflow)?,
            )
            .ok_or(CombatSimulationError::Overflow)?,
        y: from
            .y
            .checked_add(
                dy.signum()
                    .checked_mul(y_step)
                    .ok_or(CombatSimulationError::Overflow)?,
            )
            .ok_or(CombatSimulationError::Overflow)?,
    };
    Ok(target)
}
fn fingerprint<T: Serialize>(value: &T) -> Result<String, CombatSimulationError> {
    serde_json::to_string(value)
        .map(|s| format!("{:016x}", fnv(s.as_bytes())))
        .map_err(|_| CombatSimulationError::Serialization)
}
fn fnv(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
fn ensure_id(id: &str) -> Result<(), CombatSimulationError> {
    if id.trim().is_empty() {
        Err(CombatSimulationError::EmptyId)
    } else {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatSimulationError {
    EmptyId,
    InvalidConfig,
    InvalidParticipant(String),
    InvalidRole(String),
    InvalidFacing(String),
    InvalidRange,
    Overflow,
    ParticipantLimit,
    DuplicateId(String),
    MissingReference(String),
    InvalidReference,
    MaxTicksExceeded,
    Serialization,
    /// `input.manifest.simulation_version` is not the one this build
    /// implements. Dedicated variant so the cause is visible at a glance,
    /// rather than folded into `InvalidReference` (T0 §4-3).
    UnsupportedSimulationVersion(String),
}
impl std::fmt::Display for CombatSimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatSimulationError {}
