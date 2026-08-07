use crate::combat_contract::ensure_supported_simulation_version;
use crate::{line, CombatManifest, CombatState, HexCoord};
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
// T1-b1 (fable_combat_hex_t1b1_step1_2608071921.md §4-1): `CombatPosition{x,y}`
// and `CombatFacing{x,y}` are gone, not renamed. Both roles are now played by
// `HexCoord{q,r}` (axial, from the frozen `combat_hex` module) --
// `Serialize`/`Deserialize`/`Ord`/`Hash` on that type already cover everything
// these two used to need. The old euclidean helpers have no 1:1 method
// replacement on `HexCoord`; call sites below inline the plan's mapping
// table instead (`distance_squared` -> `HexCoord::distance`, `in_range` ->
// `a.distance(b) <= i64::from(range)`, `overlaps` -> deleted, see
// `combat_resolution.rs`).
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
    pub position: HexCoord,
    pub facing: HexCoord,
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
    pub from: HexCoord,
    pub to: HexCoord,
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
    pub positions: BTreeMap<String, HexCoord>,
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
            // T1-b1 §4-2: facing must be one of the six hex neighbor
            // directions. The zero vector is not among them, so it keeps
            // being rejected automatically -- no special case needed.
            if !HexCoord::NEIGHBOR_DIRECTIONS.contains(&p.facing) {
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
                            // `HexCoord::distance` is total (no invalid
                            // input exists), so no `unwrap_or` fallback is
                            // needed anymore (T1-b1 §4-1).
                            let da = self.participants[&a.target_id]
                                .position
                                .distance(actor.position);
                            let db = self.participants[&b.target_id]
                                .position
                                .distance(actor.position);
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
            .map(|p| (p.position.distance(actor.position), p.id.clone()))
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
                    // T1-b1 §4-1/§4-3: hex distance is linear, not squared,
                    // so the old `d > preferred * preferred` comparison
                    // becomes a direct `d > preferred` -- no squaring on
                    // either side.
                    let d = actor.position.distance(target.position);
                    let preferred = i64::from(role.weights.preferred_distance.max(0));
                    let step = actor.speed_per_tick;
                    if d > preferred {
                        Ok((
                            step_toward(actor.position, target.position, step)?,
                            CombatMoveMode::Advance,
                        ))
                    } else if d < preferred && role.weights.aggression < 0 {
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
// T1-b1 §4-3: `speed_per_tick` now means "tiles per tick", not "coordinate
// units per tick". Both functions below walk the straight-line path
// `combat_hex::line()` gives between two tiles, taking at most
// `speed_per_tick` steps along it (never overshooting the far end -- "최대
// speed_per_tick 타일만큼" means *up to*, not *exactly*). The old
// dominant-axis fractional-step decomposition is gone; there is nothing
// left to divide by an axis magnitude.
//
// Occupancy is intentionally NOT enforced here. Two participants can end a
// tick on the same tile and walk through each other exactly as the old
// euclidean code allowed -- this is the pre-existing behaviour, not a
// regression introduced by this slice. Tile exclusivity is T1-c's job
// (`fable_combat_hex_t1b1_step1_2608071921.md` §8, plan §6 T1 slice table).

/// Moves `from` up to `step` tiles toward `to` along `line(from, to)`.
/// Returns `from` unchanged if the two tiles coincide (no direction exists).
fn step_toward(from: HexCoord, to: HexCoord, step: i32) -> Result<HexCoord, CombatSimulationError> {
    if from == to {
        return Ok(from);
    }
    let path = line(from, to).map_err(CombatSimulationError::HexMath)?;
    let steps_available = path.len() - 1; // >= 1: `from != to` was just checked.
    let step_count = (step as usize).min(steps_available);
    Ok(path[step_count])
}

/// Moves `from` up to `step` tiles away from `to`, along the straight line
/// through `from` on the far side from `to`. Implemented by reflecting `to`
/// through `from` (a cube-coordinate reflection, always a valid `HexCoord` at
/// the same distance from `from` as `to` is) and walking `line(from,
/// mirror)` -- the plan's "line()-based" retreat, since `line()` only ever
/// interpolates *between* two given tiles and never extrapolates past an
/// endpoint on its own. Returns `from` unchanged if the two tiles coincide
/// (no direction exists), matching `step_toward` and the old euclidean
/// `step_away`.
fn step_away(from: HexCoord, to: HexCoord, step: i32) -> Result<HexCoord, CombatSimulationError> {
    if from == to {
        return Ok(from);
    }
    let mirror = reflect_through(from, to)?;
    let path = line(from, mirror).map_err(CombatSimulationError::HexMath)?;
    let steps_available = path.len() - 1; // >= 1: reflection preserves distance, which is >= 1 here.
    let step_count = (step as usize).min(steps_available);
    Ok(path[step_count])
}

/// Reflects `other` through `anchor`: the point the same distance from
/// `anchor` as `other`, in exactly the opposite direction. Axial coordinates
/// are cube coordinates with the redundant third axis dropped, and cube
/// reflection is linear component-wise, so `2*anchor - other` on `q`/`r`
/// alone is exact (no separate `x+y+z=0` fix-up is needed, unlike
/// `combat_hex::line`'s rounding). Checked in `i64` first since `2*anchor.q`
/// can exceed `i32` before the subtraction brings it back in range.
fn reflect_through(anchor: HexCoord, other: HexCoord) -> Result<HexCoord, CombatSimulationError> {
    let q = i64::from(anchor.q)
        .checked_mul(2)
        .and_then(|doubled| doubled.checked_sub(i64::from(other.q)))
        .ok_or(CombatSimulationError::Overflow)?;
    let r = i64::from(anchor.r)
        .checked_mul(2)
        .and_then(|doubled| doubled.checked_sub(i64::from(other.r)))
        .ok_or(CombatSimulationError::Overflow)?;
    Ok(HexCoord {
        q: i32::try_from(q).map_err(|_| CombatSimulationError::Overflow)?,
        r: i32::try_from(r).map_err(|_| CombatSimulationError::Overflow)?,
    })
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
    /// `combat_hex::line()` rejected a movement path (T1-b1 §4-3). In
    /// practice this can only be `HexError::PathTooLong`, since the board is
    /// nowhere near `combat_hex::MAX_LINE_LENGTH` and individual coordinates
    /// along a line cannot overflow `i32` (see that module's own docs) --
    /// the variant is still generic over `HexError` rather than hand-picking
    /// one case, so it does not silently swallow a future new variant.
    HexMath(crate::HexError),
}
impl std::fmt::Display for CombatSimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatSimulationError {}
