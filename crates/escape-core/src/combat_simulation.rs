use crate::combat_contract::ensure_supported_simulation_version;
use crate::{line, CombatManifest, CombatState, HexCoord, HexError, HexOccupancy, HexShape};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// T3 (fable_combat_hex_t3_step1_2608080951.md §4-1): the gauge threshold an
// action cadence must accumulate to before it fires. `10_000` reads as
// "100.00" in this crate's hundredths-fixed-point convention -- a speed of
// `10_000` per tick means the gauge crosses the threshold exactly once every
// tick, which is today's behaviour for every existing combatant and attack
// (none of them set a speed, so `Option::None` resolves to this value, §4-3).
// `combat_resolution.rs` shares this same threshold for its independent
// attack-speed gauge (§4-2's two-axis table); it reaches this constant via
// `crate::combat_simulation::ACTION_THRESHOLD_HUNDREDTHS` rather than a
// duplicated literal, so the two axes can never drift out of sync with each
// other by editing only one of them.
pub(crate) const ACTION_THRESHOLD_HUNDREDTHS: i64 = 10_000;

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
    // T3 §4-2: `speed_per_tick` is a *distance* per movement action (how
    // many tiles one movement judgment covers), unchanged by this slice.
    // `move_speed_hundredths` below is a *cadence* (how often a movement
    // judgment happens at all) -- a different axis entirely. The two names
    // now look confusable; renaming `speed_per_tick` is its own boundary
    // change this slice deliberately leaves alone (plan §4-2).
    pub speed_per_tick: i32,
    // T3 §4-1/§4-2/§4-3: hundredths-fixed-point movement-cadence gauge speed.
    // `None` means "act every tick" (`ACTION_THRESHOLD_HUNDREDTHS`), which is
    // every existing participant's behaviour before and after this slice --
    // no existing fixture or bundle sets this, so nothing changes for them.
    // `skip_serializing_if` drops the key from JSON entirely when unset, so
    // existing serialized bytes are untouched and no version bump is needed
    // (hard invariant 2). `Some(v)` with `v <= 0` is rejected as invalid
    // input in `CombatSimulation::new` -- never silently treated as "never
    // acts" (plan §4-3: don't invent a meaning for it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub move_speed_hundredths: Option<i64>,
    pub collision_radius: i32,
    pub attack_range: i32,
    pub support_range: i32,
    pub role_id: String,
    pub target_policy_id: Option<String>,
    pub active: bool,
    // T1-d §4-1: a plain offset list at the serialization boundary, not a
    // `HexShape` -- `HexShape` deliberately has no `Serialize`/`Deserialize`
    // (T1-a's choice), so this is converted to one only at validation time
    // (`participant_footprint`). An empty list means exactly one tile, the
    // anchor (`position`) itself -- every participant before this slice
    // implicitly meant that, and still does. `skip_serializing_if` makes the
    // key vanish entirely from JSON when empty, so every existing bundle and
    // fixture serializes to the exact same bytes as before (hard invariant
    // 2) -- no version bump is needed, or should be, for this change.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub occupies: Vec<HexCoord>,
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
    // T3 §4-1/§4-4: each active participant's accumulated movement-cadence
    // gauge, carried across ticks. This is pure runtime state, not part of
    // `CombatSimulationInput` -- it never crosses a JSON boundary, so adding
    // it here touches no serialized contract and needs no version bump.
    // Keyed by participant id in a `BTreeMap` so lookups (and, incidentally,
    // any iteration) are independent of `input.participants`'s original
    // order (invariant 4/§4-4).
    move_gauges: BTreeMap<String, i64>,
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
        // T1-c §4-2①: two active participants cannot start the simulation on
        // the same tile. This is the same `HexOccupancy` T1-d's large units
        // will use, but every slice through T1-c always hands it a
        // single-tile slice (§4-1: one tile, one unit here). `try_occupy`
        // already reports the exact tile on conflict; a dedicated error
        // variant (`DuplicateStartingPosition`) carries it rather than
        // folding this into `InvalidParticipant`/`DuplicateId`, which mean
        // something else.
        let mut starting_occupancy = HexOccupancy::new();
        for p in input.participants.iter().filter(|p| p.active) {
            ensure_id(&p.id)?;
            if p.speed_per_tick <= 0
                || p.collision_radius <= 0
                || p.attack_range <= 0
                || p.support_range <= 0
            {
                return Err(CombatSimulationError::InvalidParticipant(p.id.clone()));
            }
            // T3 §4-3: `Some(v)` with `v <= 0` is an input error, not "never
            // acts" -- a fabricated meaning the plan explicitly forbids
            // inventing. `None` (unset) is the only way to mean "every tick"
            // and is handled at the gauge, not here.
            if p.move_speed_hundredths.is_some_and(|v| v <= 0) {
                return Err(CombatSimulationError::InvalidParticipant(p.id.clone()));
            }
            // T1-b1 §4-2: facing must be one of the six hex neighbor
            // directions. The zero vector is not among them, so it keeps
            // being rejected automatically -- no special case needed.
            if !HexCoord::NEIGHBOR_DIRECTIONS.contains(&p.facing) {
                return Err(CombatSimulationError::InvalidFacing(p.id.clone()));
            }
            // T1-d §4-2/§4-3/§4-6: validates `p.occupies` (anchor-inclusion,
            // connectivity) and returns the actual tiles `p` occupies.
            // `try_occupy`'s all-or-nothing (`combat_hex.rs`) is what makes
            // "reject if the two footprints overlap by even one tile" come
            // for free here -- no separate overlap scan is needed.
            let footprint = participant_footprint(p)?;
            match starting_occupancy.try_occupy(&footprint, &p.id) {
                Ok(()) => {}
                Err(HexError::TileOccupied(tile)) => {
                    return Err(CombatSimulationError::DuplicateStartingPosition(tile));
                }
                Err(other) => return Err(CombatSimulationError::HexMath(other)),
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
        // T3 §4-1: every active participant starts at gauge 0, regardless of
        // its speed -- the first tick after construction is exactly one
        // gauge accumulation away from acting, same as any later tick.
        let move_gauges = participants.keys().map(|id| (id.clone(), 0i64)).collect();
        Ok(Self {
            input,
            tick: 0,
            participants,
            roles,
            policies,
            move_gauges,
        })
    }
    pub(crate) fn sync_active_from_health(
        &mut self,
        health_hundredths: &BTreeMap<String, i64>,
    ) -> Result<(), CombatSimulationError> {
        for participant in self.participants.values_mut() {
            if health_hundredths
                .get(&participant.id)
                .is_some_and(|health| *health <= 0)
            {
                participant.active = false;
            }
        }
        Ok(())
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
            // T1-d §4-4 site 1/5: target-preference distance. Precomputed
            // into a map before the comparator runs, because
            // `footprint_distance` is fallible (`HexError::Overflow`) and
            // `max_by`'s closure isn't -- `?` can't be used inside it. Both
            // of the two distance lookups the old comparator made inline
            // (`da` for `a`'s target, `db` for `b`'s target) now read from
            // this same footprint-based map instead of raw anchor
            // `.distance()`.
            let mut preference_distance: BTreeMap<&str, i64> = BTreeMap::new();
            for pref in policy.preferences.iter().filter(|p| valid(&p.target_id)) {
                let target = &self.participants[&pref.target_id];
                let distance = footprint_distance(
                    target.position,
                    &target.occupies,
                    actor.position,
                    &actor.occupies,
                )
                .map_err(CombatSimulationError::HexMath)?;
                preference_distance.insert(pref.target_id.as_str(), distance);
            }
            if let Some(target) = policy
                .preferences
                .iter()
                .filter(|p| valid(&p.target_id))
                .max_by(|a, b| {
                    a.priority
                        .cmp(&b.priority)
                        .then_with(|| {
                            let da = preference_distance[a.target_id.as_str()];
                            let db = preference_distance[b.target_id.as_str()];
                            db.cmp(&da)
                        })
                        .then_with(|| b.target_id.cmp(&a.target_id))
                })
            {
                return Ok(Some(target.target_id.clone()));
            }
        }
        // T1-d §4-4 site 2/5: nearest-target fallback. Same fallibility
        // reason as above -- distances are collected into a `Vec` first
        // (propagating `?` per candidate), then `min_by` runs over the
        // already-computed values.
        let mut nearest_candidates = Vec::new();
        for p in self.participants.values().filter(|p| valid(&p.id)) {
            let distance =
                footprint_distance(p.position, &p.occupies, actor.position, &actor.occupies)
                    .map_err(CombatSimulationError::HexMath)?;
            nearest_candidates.push((distance, p.id.clone()));
        }
        Ok(nearest_candidates
            .into_iter()
            .min_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
            .map(|(_, id)| id))
    }
    pub fn advance_tick(&mut self) -> Result<CombatTickFrame, CombatSimulationError> {
        if self.tick >= self.input.config.max_ticks {
            return Err(CombatSimulationError::MaxTicksExceeded);
        }
        let snapshot = self.participants.clone();
        // T1-c §4-4: occupancy is read from this tick-start snapshot and
        // never rebuilt mid-tick. See `occupancy_snapshot`'s doc comment for
        // why that is load-bearing, not incidental.
        let occupancy = occupancy_snapshot(&snapshot)?;
        // T3 §4-1/§4-4: how many times each active participant's movement
        // gauge crosses `ACTION_THRESHOLD_HUNDREDTHS` this tick, decided
        // from every participant's *tick-start* gauge value, in a pass that
        // completes before any participant's movement is applied below. Each
        // participant's gauge depends only on its own speed and its own
        // prior value -- never on another participant's position, gauge, or
        // this tick's moves -- so advancing it here regardless of the order
        // `snapshot` happens to iterate in can never leak processing order
        // into the result (invariant 4, §4-4). A speed of `20_000` crosses
        // the threshold twice in one tick and yields two actions, not one
        // (§4-1: a cadence must not be clamped to "at most once per tick");
        // a speed slower than the threshold yields zero actions on some
        // ticks.
        let mut move_actions: BTreeMap<String, i32> = BTreeMap::new();
        for actor in snapshot.values().filter(|actor| actor.active) {
            let speed = actor
                .move_speed_hundredths
                .unwrap_or(ACTION_THRESHOLD_HUNDREDTHS);
            let gauge = self.move_gauges.entry(actor.id.clone()).or_insert(0);
            *gauge = gauge
                .checked_add(speed)
                .ok_or(CombatSimulationError::Overflow)?;
            let mut actions = 0i32;
            while *gauge >= ACTION_THRESHOLD_HUNDREDTHS {
                actions += 1;
                *gauge -= ACTION_THRESHOLD_HUNDREDTHS;
            }
            move_actions.insert(actor.id.clone(), actions);
        }
        let mut moves = Vec::new();
        for actor in snapshot.values().filter(|actor| actor.active) {
            // T3 §4-2: target selection runs every tick unconditionally,
            // never gated on the movement-cadence gauge above. An attack's
            // own, independent attack-speed gauge (`combat_resolution.rs`)
            // reads its actor's `target_id` off this tick's intent, and must
            // not be starved of a target just because that actor's
            // *movement* gauge happened to be slow this same tick -- the two
            // cadences are separate axes (plan's two-gauge table) and must
            // not leak into each other.
            let target_id = self.select_target(actor)?;
            let target = target_id.as_ref().and_then(|id| snapshot.get(id));
            let role = &self.roles[&actor.role_id];
            let actions = move_actions[&actor.id];
            let (to, mode) = if actions == 0 {
                // T3 §4-1: the gauge did not cross the threshold this tick --
                // no movement judgment is acted on, regardless of what the
                // distance-based decision below would otherwise choose. This
                // reads identically to an ordinary "already at preferred
                // distance" hold in the frame (both are `Hold`/`to == from`),
                // which is exactly the allowed kind of visible-but-numberless
                // leak (§4-5): a viewer can see the piece didn't move, never
                // why or how fast.
                (actor.position, CombatMoveMode::Hold)
            } else {
                target
                    .map(|target| {
                        // T1-b1 §4-1/§4-3: hex distance is linear, not squared,
                        // so the old `d > preferred * preferred` comparison
                        // becomes a direct `d > preferred` -- no squaring on
                        // either side.
                        // T1-d §4-4 site 3/5: movement decision. Footprint
                        // distance replaces anchor distance so a large unit's
                        // preferred-distance judgement is based on how close its
                        // body actually is, not just its anchor.
                        let d = footprint_distance(
                            actor.position,
                            &actor.occupies,
                            target.position,
                            &target.occupies,
                        )
                        .map_err(CombatSimulationError::HexMath)?;
                        let preferred = i64::from(role.weights.preferred_distance.max(0));
                        // T3 §4-1: `actions` (>= 1 in this branch) folds
                        // multiple movement actions this tick into a single
                        // longer step along the same line, rather than
                        // multiple `CombatMoveIntent` entries -- the frame
                        // schema stays one intent per actor per tick (plan's
                        // "프레임 스키마를 바꾸지 마라" pattern, already used by
                        // T1-d for footprints). This is exactly equivalent to
                        // resolving the same number of individual steps in
                        // sequence: `occupancy` is the frozen tick-start
                        // snapshot for every one of those hypothetical
                        // sub-steps, so combining them into one longer walk
                        // along `line()` stops at the same tile either way.
                        let step = actor.speed_per_tick.saturating_mul(actions);
                        if d > preferred {
                            Ok((
                                step_toward(
                                    actor.position,
                                    target.position,
                                    step,
                                    &occupancy,
                                    &actor.occupies,
                                    &actor.id,
                                )?,
                                CombatMoveMode::Advance,
                            ))
                        } else if d < preferred && role.weights.aggression < 0 {
                            Ok((
                                step_away(
                                    actor.position,
                                    target.position,
                                    step,
                                    &occupancy,
                                    &actor.occupies,
                                    &actor.id,
                                )?,
                                CombatMoveMode::Retreat,
                            ))
                        } else {
                            Ok((actor.position, CombatMoveMode::Hold))
                        }
                    })
                    .transpose()?
                    .unwrap_or((actor.position, CombatMoveMode::Hold))
            };
            moves.push(CombatMoveIntent {
                actor_id: actor.id.clone(),
                target_id,
                from: actor.position,
                to,
                mode,
            });
        }
        moves.sort_by(|a, b| a.actor_id.cmp(&b.actor_id));
        // T1-c §4-2③/§4-3, broadened by T1-d §4-5: two participants whose
        // destination *footprints* (after the per-actor occupancy-aware path
        // truncation above) share even one tile both give up the move this
        // tick and hold at `from` instead. See `resolve_destination_contention`'s
        // doc comment for why neither one is allowed to win the tile.
        let occupies_by_actor: BTreeMap<&str, &[HexCoord]> = snapshot
            .values()
            .filter(|p| p.active)
            .map(|p| (p.id.as_str(), p.occupies.as_slice()))
            .collect();
        resolve_destination_contention(&mut moves, &occupies_by_actor)?;
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
// T1-c §4-2②: occupancy IS now enforced here, ending the pass-through defect
// noted above (T1-b1 recorded it, didn't fix it -- tile exclusivity was
// always T1-c's job, plan §6 T1 slice table). Both functions take the
// tick-start `HexOccupancy` snapshot and stop one tile short of whatever is
// occupied, rather than walking through it.

/// Moves `from` up to `step` tiles toward `to` along `line(from, to)`,
/// stopping at the last tile whose whole footprint is free if `occupancy`
/// blocks the rest of the path (T1-c §4-2②, footprint-wide since T1-d §4-5).
/// `occupies` is the mover's own offset list (empty means one tile) and
/// `mover_id` lets the scan ignore the mover's own current footprint (T1-d
/// §4-5's "자기 자신의 현재 타일은 자기를 막지 않는다" -- see
/// `first_free_tile_along`'s doc comment for why that matters for footprints
/// bigger than one tile). Returns `from` unchanged if the two tiles
/// coincide (no direction exists).
fn step_toward(
    from: HexCoord,
    to: HexCoord,
    step: i32,
    occupancy: &HexOccupancy,
    occupies: &[HexCoord],
    mover_id: &str,
) -> Result<HexCoord, CombatSimulationError> {
    if from == to {
        return Ok(from);
    }
    let path = line(from, to).map_err(CombatSimulationError::HexMath)?;
    let steps_available = path.len() - 1; // >= 1: `from != to` was just checked.
    let step_count = (step as usize).min(steps_available);
    first_free_tile_along(&path, step_count, occupancy, occupies, mover_id)
}

/// Moves `from` up to `step` tiles away from `to`, along the straight line
/// through `from` on the far side from `to`, stopping at the last tile whose
/// whole footprint is free if `occupancy` blocks the rest of the path (T1-c
/// §4-2②, applied to retreat too -- not just advance; footprint-wide since
/// T1-d §4-5). Implemented by reflecting `to` through `from` (a
/// cube-coordinate reflection, always a valid `HexCoord` at the same
/// distance from `from` as `to` is) and walking `line(from, mirror)` -- the
/// plan's "line()-based" retreat, since `line()` only ever interpolates
/// *between* two given tiles and never extrapolates past an endpoint on its
/// own. Returns `from` unchanged if the two tiles coincide (no direction
/// exists), matching `step_toward` and the old euclidean `step_away`.
fn step_away(
    from: HexCoord,
    to: HexCoord,
    step: i32,
    occupancy: &HexOccupancy,
    occupies: &[HexCoord],
    mover_id: &str,
) -> Result<HexCoord, CombatSimulationError> {
    if from == to {
        return Ok(from);
    }
    let mirror = reflect_through(from, to)?;
    let path = line(from, mirror).map_err(CombatSimulationError::HexMath)?;
    let steps_available = path.len() - 1; // >= 1: reflection preserves distance, which is >= 1 here.
    let step_count = (step as usize).min(steps_available);
    first_free_tile_along(&path, step_count, occupancy, occupies, mover_id)
}

/// Walks `path[1..=max_index]`, checking at each candidate anchor whether
/// the mover's *whole footprint* there (`footprint_tiles(candidate,
/// occupies)`, T1-d §4-5) is free -- stopping at the last candidate that
/// passes and returning `path[0]` unmoved if even the first candidate
/// (`path[1]`) fails.
///
/// **A tile occupied by `mover_id` itself counts as free (T1-d §4-5's
/// trap).** `path[0]` (the mover's own starting tile) was excluded from this
/// scan by range alone before T1-d, which was enough when every footprint
/// was exactly one tile -- a one-tile mover's footprint at any candidate
/// *other* than `path[0]` can never coincide with `path[0]` itself. That
/// stops being true once a footprint can be more than one tile: a large
/// unit's footprint at a candidate one step forward still overlaps most of
/// its own *current* footprint (still marked occupied under its own id in
/// this tick's frozen snapshot). Range-based exclusion no longer covers
/// that, so this checks occupancy by identity instead --
/// `occupancy.occupant_at(tile) == Some(mover_id)` reads as free, any other
/// occupant does not. Miss this and a large unit's every candidate step
/// re-collides with its own trailing tiles and it never moves at all, which
/// looks exactly like "correctly blocked by an enemy" until someone checks
/// why (`a_large_unit_does_not_block_itself_while_moving` pins this).
///
/// **Still otherwise deliberately conservative (T1-c §4-4), not a bug** --
/// pinned by `a_tile_vacated_this_tick_is_not_entered_this_tick`. `occupancy`
/// is always the tick-start snapshot (`advance_tick` builds it once via
/// `occupancy_snapshot` and never rebuilds it mid-tick, T1-d §4-5's "점유는
/// 여전히 tick 시작 스냅샷에서 읽는다"), so a tile some *other* unit is
/// vacating this very tick still reads as occupied here. This is the price
/// paid for order independence (invariant 4) -- see the historical T1-c
/// discussion this replaced for the full argument; nothing about that
/// argument changes for footprints.
fn first_free_tile_along(
    path: &[HexCoord],
    max_index: usize,
    occupancy: &HexOccupancy,
    occupies: &[HexCoord],
    mover_id: &str,
) -> Result<HexCoord, CombatSimulationError> {
    let mut last_free = path[0];
    for &candidate in &path[1..=max_index] {
        let footprint =
            footprint_tiles(candidate, occupies).map_err(CombatSimulationError::HexMath)?;
        let blocked = footprint.iter().any(|&tile| {
            occupancy
                .occupant_at(tile)
                .is_some_and(|occupant| occupant != mover_id)
        });
        if blocked {
            break;
        }
        last_free = candidate;
    }
    Ok(last_free)
}

/// Builds the tile-occupancy map for one tick, from a snapshot of
/// participants taken at tick start (T1-c §4-4). Every active participant
/// occupies its full footprint (T1-d §4-1: `p.occupies` placed at
/// `p.position`, or just `p.position` alone if `occupies` is empty) --
/// T1-c §4-1 fixed "one tile, one unit" for that slice; this is T1-d wiring
/// multi-tile footprints into the same snapshot.
///
/// This is built once per tick and handed around read-only; nothing in
/// `advance_tick` mutates it mid-tick. That is what makes occupancy checks
/// order-independent (invariant 4) -- see `first_free_tile_along`'s doc
/// comment for what that buys and what it costs.
///
/// `try_occupy` is expected to never fail: `CombatSimulation::new` already
/// rejects two active participants whose starting footprints overlap by even
/// one tile (`DuplicateStartingPosition`, T1-c §4-2①, broadened to
/// footprints by T1-d §4-6), and every tick's occupancy-aware movement
/// (§4-2②/③, footprint-aware since T1-d §4-5) keeps that property true
/// afterwards too. If it fails anyway, that invariant was violated somewhere
/// it shouldn't have been -- a real bug, reported as
/// `CombatSimulationError::OccupancyInvariantViolated` rather than swallowed
/// or panicked on, since nothing else in this crate panics on bad state and
/// a library that panics on bad input is worse than one that returns an
/// error for it.
fn occupancy_snapshot(
    participants: &BTreeMap<String, CombatSimulationParticipant>,
) -> Result<HexOccupancy, CombatSimulationError> {
    let mut occupancy = HexOccupancy::new();
    for p in participants.values().filter(|p| p.active) {
        let footprint = participant_footprint(p)?;
        occupancy
            .try_occupy(&footprint, &p.id)
            .map_err(|_| CombatSimulationError::OccupancyInvariantViolated(p.position))?;
    }
    Ok(occupancy)
}

/// T1-d §4-1/§4-2/§4-3: validates `p.occupies` and returns the tiles `p`
/// actually occupies at its current `position`.
///
/// An empty `occupies` means exactly one tile, `p.position` itself (§4-1;
/// every participant before this slice implicitly meant that, and still
/// does) -- the footprint-specific checks below are skipped entirely in that
/// case, since a single tile trivially satisfies all of them.
///
/// Non-empty `occupies` must: contain the `(0,0)` offset (§4-2 -- the anchor
/// must be one of the occupied tiles, or logs/targeting/spectator would
/// describe the participant as standing somewhere it doesn't stand); have no
/// duplicate offsets (`HexShape::new` rejects this on its own, surfaced here
/// as `CombatSimulationError::HexMath(HexError::DuplicateOffset(..))`); and
/// be hex-adjacency-connected as one blob (§4-3 -- two disconnected tiles
/// are two units, not one; this is a distinct structural rule from symmetry,
/// which is deliberately *not* enforced in code, per §4-3).
fn participant_footprint(
    p: &CombatSimulationParticipant,
) -> Result<Vec<HexCoord>, CombatSimulationError> {
    if p.occupies.is_empty() {
        return Ok(vec![p.position]);
    }
    let origin = HexCoord { q: 0, r: 0 };
    if !p.occupies.contains(&origin) {
        return Err(CombatSimulationError::FootprintMissingAnchor(p.id.clone()));
    }
    let shape = HexShape::new(p.occupies.clone()).map_err(CombatSimulationError::HexMath)?;
    if !footprint_is_connected(&p.occupies) {
        return Err(CombatSimulationError::DisconnectedFootprint(p.id.clone()));
    }
    shape
        .tiles_at(p.position)
        .map_err(CombatSimulationError::HexMath)
}

/// T1-d §4-3: are `offsets` a single hex-adjacency-connected blob? BFS from
/// `(0,0)`, which the caller (`participant_footprint`) has already confirmed
/// is present before calling this. Two offsets are adjacent by
/// `HexCoord::is_adjacent` -- the same primitive `combat_hex.rs` exposes for
/// exactly this kind of check. O(n^2) in footprint size, which is fine:
/// large units are a handful of tiles, not hundreds.
fn footprint_is_connected(offsets: &[HexCoord]) -> bool {
    let all: BTreeSet<HexCoord> = offsets.iter().copied().collect();
    let origin = HexCoord { q: 0, r: 0 };
    let mut visited: BTreeSet<HexCoord> = BTreeSet::from([origin]);
    let mut frontier = vec![origin];
    while let Some(current) = frontier.pop() {
        for &candidate in &all {
            if !visited.contains(&candidate) && current.is_adjacent(candidate) {
                visited.insert(candidate);
                frontier.push(candidate);
            }
        }
    }
    visited.len() == all.len()
}

/// T1-d §4-1: the tiles occupied by a footprint anchored at `anchor`. An
/// empty `occupies` means exactly one tile, `anchor` itself. A non-empty list
/// is placed as a `HexShape` at `anchor`.
///
/// Callers are expected to have already validated `occupies` (anchor
/// inclusion, connectivity, no duplicates) via
/// `CombatSimulation::new`/`participant_footprint` -- this function does not
/// re-check any of that, only hex-math (`anchor + offset` can overflow
/// `i32`, hence `Result`).
fn footprint_tiles(anchor: HexCoord, occupies: &[HexCoord]) -> Result<Vec<HexCoord>, HexError> {
    if occupies.is_empty() {
        return Ok(vec![anchor]);
    }
    HexShape::new(occupies.to_vec())?.tiles_at(anchor)
}

/// T1-d §4-4: the distance between two participants, defined as the minimum
/// hex distance between any tile of one's footprint and any tile of the
/// other's -- not anchor-to-anchor. Anchor distance stayed correct only for
/// single-tile participants; a large unit's anchor can be far away while its
/// body is already adjacent (or in range), and anchor distance alone would
/// wrongly read that as "out of range."
///
/// Both anchors are each participant's *current* tile (`position` in
/// `combat_simulation.rs`, or a frozen `CombatTickFrame` anchor in
/// `combat_resolution.rs`); `occupies` is each one's fixed offset list,
/// which never itself moves -- only the anchor does. That is what lets this
/// same function serve both call sites (this slice's five distance
/// measurement sites, plan §4-4: two in `select_target`'s target-preference
/// comparator, one in `select_target`'s nearest-target fallback, one in
/// `advance_tick`'s movement decision, and one in `combat_resolution.rs`'s
/// range/collision judgement).
///
/// For two single-tile participants (`occupies` empty on both sides) this
/// collapses to exactly `HexCoord::distance(a_anchor, b_anchor)`, since each
/// footprint reduces to the one-tile set `[anchor]` -- the minimum over a
/// single pair is that one distance (T1-d §5 invariant 1: no existing fight
/// changes).
pub fn footprint_distance(
    a_anchor: HexCoord,
    a_occupies: &[HexCoord],
    b_anchor: HexCoord,
    b_occupies: &[HexCoord],
) -> Result<i64, HexError> {
    let a_tiles = footprint_tiles(a_anchor, a_occupies)?;
    let b_tiles = footprint_tiles(b_anchor, b_occupies)?;
    Ok(a_tiles
        .iter()
        .flat_map(|&a| b_tiles.iter().map(move |&b| a.distance(b)))
        .min()
        .expect("footprint_tiles never returns an empty vec"))
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
/// T1-c §4-2③/§4-3, broadened to footprints by T1-d §4-5: if two (or more)
/// move intents computed above land on *destination footprints that share
/// even one tile*, none of them gets it -- all give up the move this tick
/// and hold at `from` instead. For single-tile participants this is exactly
/// the old "exact same destination tile" rule (a one-tile footprint can only
/// ever "share a tile" with another by being that same tile).
///
/// **Why nobody wins the tile (design rationale -- do not "fix" this by
/// adding a priority):** contention for the same free tile in the same tick
/// is routine, not an edge case, once two units approach each other or
/// converge on a flanking position. Breaking the tie would need some
/// deterministic order, and the only one available right now is id-alphabetic
/// order on `actor_id`. Using it would mean "alphabetically-earlier ids win
/// contested tiles" becomes an actual combat rule -- and that violates
/// invariant 4 (order independence), the same invariant
/// `simultaneous_mutual_defeat_is_independent_of_attack_definition_order`
/// pins elsewhere: outcomes must not depend on the arbitrary order
/// participants (or their ids) happen to be listed in.
///
/// "Both hold" is symmetric -- it doesn't care about id order at all -- and
/// it reads fine in-fiction too (two combatants stepping into each other's
/// way). It also deliberately does *not* invent a temporary priority scheme
/// (push, first-come, initiative, whatever): T2 owns real tile-assignment
/// resolution (move reservations, collision, pushing, adjacent substitution),
/// and anything invented here would just be dead weight T2 has to rip out
/// later. Refusing both is the only answer that commits to nothing.
///
/// `occupies_by_actor` supplies each actor's own fixed offset list (the
/// caller reads it from the tick-start snapshot); a missing entry is treated
/// as the single-tile default (`&[]`), which cannot actually happen since
/// `advance_tick` builds this map from the same snapshot as `moves`, but
/// doing it this way means a lookup miss degrades to "one tile" instead of
/// panicking. Grouping by destination tile with a `BTreeMap<HexCoord,
/// Vec<usize>>` (not a `HashMap`) keeps this itself deterministic, matching
/// the rest of `combat_hex`/`combat_simulation`'s no-`HashMap` convention.
fn resolve_destination_contention(
    moves: &mut [CombatMoveIntent],
    occupies_by_actor: &BTreeMap<&str, &[HexCoord]>,
) -> Result<(), CombatSimulationError> {
    let mut destination_footprints: Vec<Vec<HexCoord>> = Vec::with_capacity(moves.len());
    for m in moves.iter() {
        let occupies = occupies_by_actor
            .get(m.actor_id.as_str())
            .copied()
            .unwrap_or(&[]);
        destination_footprints
            .push(footprint_tiles(m.to, occupies).map_err(CombatSimulationError::HexMath)?);
    }
    let mut tile_claimants: BTreeMap<HexCoord, Vec<usize>> = BTreeMap::new();
    for (index, footprint) in destination_footprints.iter().enumerate() {
        for &tile in footprint {
            tile_claimants.entry(tile).or_default().push(index);
        }
    }
    let mut contended = vec![false; moves.len()];
    for claimants in tile_claimants.values() {
        if claimants.len() > 1 {
            for &index in claimants {
                contended[index] = true;
            }
        }
    }
    for (index, m) in moves.iter_mut().enumerate() {
        if contended[index] {
            m.to = m.from;
            m.mode = CombatMoveMode::Hold;
        }
    }
    Ok(())
}

/// T1-c §4-6: pure, unwired surround derivation. Reports which of `actor`'s
/// six hex neighbors are occupied by an active enemy (a participant on the
/// opposite `CombatSide`) -- the raw material T4's emergency-rescue
/// intervention (정본 결정 20, "포위된 아군") needs, produced here without
/// deciding anything about when to act on it.
///
/// The returned tiles are in `HexCoord::NEIGHBOR_DIRECTIONS` order (that
/// order is `combat_hex.rs`'s contract, inherited unchanged here), so a
/// caller that wants directions rather than a bare count still has them.
/// `.len()` on the result is the count; the tiles themselves are the
/// direction list -- "반환은 개수와 방향 목록 둘 다 쓸 수 있게 한다" (§4-6)
/// without needing two separate return values for it.
///
/// **No board-boundary concept exists in this codebase yet, so this counts
/// enemy occupation only.** A neighbor that would be blocked by terrain or a
/// board edge is not treated as "surrounding" -- that concept doesn't exist
/// here, and this function does not invent one just to make surround counts
/// look more complete.
///
/// **Deliberately defines no threshold.** "How many occupied neighbors count
/// as surrounded" is an intervention-detection rule, not a derived fact, and
/// it belongs to T4. A constant added here would be exactly the kind of
/// premature decision T4 would have to remove before it could pick its own.
///
/// **Not wired into anything.** Nothing in AI target selection, movement, or
/// intervention calls this yet (`surround_detection_is_not_wired_into_movement_or_targeting`
/// pins that absence) -- wiring it up is T4's job, T1-c only produces the
/// derived state.
pub fn surrounding_enemy_neighbors(
    actor: &CombatSimulationParticipant,
    participants: &[CombatSimulationParticipant],
) -> Result<Vec<HexCoord>, CombatSimulationError> {
    let neighbors = actor
        .position
        .neighbors()
        .map_err(CombatSimulationError::HexMath)?;
    let enemy_tiles: BTreeSet<HexCoord> = participants
        .iter()
        .filter(|p| p.active && p.side != actor.side)
        .map(|p| p.position)
        .collect();
    Ok(neighbors
        .into_iter()
        .filter(|neighbor| enemy_tiles.contains(neighbor))
        .collect())
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
    /// T1-c §4-2①: two active participants were configured to start on the
    /// same tile. Distinct from `DuplicateId` (which means the same
    /// combatant id appears twice) and from `InvalidParticipant` (which
    /// means one participant's own fields are internally inconsistent) --
    /// this is about two otherwise-valid participants colliding with each
    /// other. Carries the contested tile.
    DuplicateStartingPosition(HexCoord),
    /// `occupancy_snapshot` found two active participants sharing a tile
    /// when building a tick-start snapshot. This should be unreachable:
    /// `CombatSimulation::new` already rejects two active participants
    /// starting on the same tile (`DuplicateStartingPosition`, T1-c §4-2①),
    /// and every tick's occupancy-aware movement (§4-2②/③) keeps that
    /// property true afterwards. Reaching this variant means that
    /// construction-time invariant was violated somewhere it shouldn't have
    /// been -- a real bug, surfaced as a `Result` rather than a panic,
    /// since nothing else in this crate panics on bad state. Carries the
    /// contested tile.
    OccupancyInvariantViolated(HexCoord),
    /// T1-d §4-2: a participant's `occupies` is non-empty but does not
    /// include the `(0,0)` offset. The anchor (`position`) is "where this
    /// participant is" for logs, targeting, and spectator purposes -- an
    /// anchor outside the occupied tiles would mean reporting a combatant
    /// standing somewhere it doesn't actually stand. Carries the
    /// participant's id.
    FootprintMissingAnchor(String),
    /// T1-d §4-3: a participant's `occupies` tiles do not form a single
    /// hex-adjacency-connected blob. Two disconnected tiles describe two
    /// units, not one -- a distinct structural-validity failure from
    /// `HexError::DuplicateOffset` (which `HexShape::new` already rejects on
    /// its own) and unrelated to symmetry, which is deliberately left as a
    /// content-authoring guideline rather than a code-enforced rule (§4-3).
    /// Carries the participant's id.
    DisconnectedFootprint(String),
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
