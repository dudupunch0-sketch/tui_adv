use crate::combat_simulation::ACTION_THRESHOLD_HUNDREDTHS;
use crate::{
    execute_combat, footprint_distance, side_all_defeated, CombatEffectCatalog,
    CombatEffectDefinition, CombatEffectInstance, CombatExecutionError, CombatExecutionRequest,
    CombatExecutionResult, CombatLogImportance, CombatRngNamespace, CombatSide,
    CombatSimulationError, CombatSimulationParticipant, EffectLifetime, EffectStacking,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatAttackEffect {
    pub effect_id: String,
    pub chance_percent: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatAttackDefinition {
    pub id: String,
    pub actor_id: String,
    pub power_hundredths: i64,
    pub ability_multiplier_hundredths: i64,
    pub accuracy_percent: u8,
    pub attack_range: i32,
    pub penetration_hundredths: i64,
    pub collision_balance_hundredths: i64,
    pub balance_power_hundredths: i64,
    // T3 (fable_combat_hex_t3_step1_2608080951.md §4-1/§4-2/§4-3):
    // hundredths-fixed-point attack-cadence gauge speed, independent of the
    // actor's movement cadence -- one combatant can carry several attacks,
    // each with its own rhythm (plan's two-gauge table). `None` means "fire
    // every tick" (`ACTION_THRESHOLD_HUNDREDTHS`), which is every existing
    // attack's behaviour before and after this slice. `skip_serializing_if`
    // drops the key from JSON when unset, so no existing bundle or fixture's
    // bytes change and no version bump is needed (hard invariant 2).
    // `Some(v)` with `v <= 0` is rejected in `validate_inputs` rather than
    // treated as "never fires" (§4-3: don't invent a meaning for it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack_speed_hundredths: Option<i64>,
    #[serde(default)]
    pub effects: Vec<CombatAttackEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatDefenseProfile {
    pub combatant_id: String,
    pub defense_hundredths: i64,
    pub balance_resistance_hundredths: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionRequest {
    pub execution: CombatExecutionRequest,
    pub attacks: Vec<CombatAttackDefinition>,
    pub defenses: Vec<CombatDefenseProfile>,
    pub catalog: CombatEffectCatalog,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionCombatant {
    pub id: String,
    pub current_health_hundredths: i64,
    pub maximum_health_hundredths: i64,
    pub balance_hundredths: i64,
    pub maximum_balance_hundredths: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionState {
    pub combatants: Vec<CombatResolutionCombatant>,
    pub active_effects: Vec<CombatEffectInstance>,
    pub applied_effect_ids: Vec<String>,
    pub suppressed_effect_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatAttackOutcome {
    pub attack_id: String,
    pub actor_id: String,
    pub target_id: String,
    pub collision: bool,
    pub in_range: bool,
    pub roll_percent: u8,
    pub hit: bool,
    pub damage_hundredths: i64,
    pub balance_delta_hundredths: i64,
    pub applied_effect_ids: Vec<String>,
    pub suppressed_effect_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionFrame {
    pub tick: u32,
    pub outcomes: Vec<CombatAttackOutcome>,
    /// 이 tick의 모든 outcome을 적용한 뒤의 전투원 상태. id 오름차순.
    /// `CombatResolutionState`(전투 종료 후 최종 상태)와 달리 tick 단위 기록이며,
    /// 관전 연출이 균형 붕괴·전투불능 시점을 알 수 있게 한다.
    ///
    /// fingerprint 범위 주의 (2026-08-02 실측):
    /// - 이 struct의 `fingerprint`는 `(tick, outcomes)`만 해싱하므로 이 필드를 포함하지 않는다.
    /// - 반면 `CombatResolutionResult.fingerprint`는 `frames`를 직렬화해 해싱하므로
    ///   이 필드가 값에 섞인다. 즉 frame에 필드를 추가하면 result/conclusion/spectator
    ///   fingerprint 값이 바뀐다. 아직 save·JSON boundary에 노출된 적이 없어 호환 문제는
    ///   없지만, Wave 3 Step 1c에서 WASM/ScenePage로 내보내기 전에 이 안정성 계약을 확정해야 한다.
    #[serde(default)]
    pub combatants: Vec<CombatResolutionCombatant>,
    pub fingerprint: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatResolutionLogTag {
    Collision,
    AttackRoll,
    DamageApplied,
    EffectApplied,
    EffectSuppressed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionLogEvent {
    pub tick: u32,
    pub sequence: u32,
    pub tag: CombatResolutionLogTag,
    pub importance: CombatLogImportance,
    pub attack_id: String,
    pub actor_id: String,
    pub target_id: String,
    pub value_hundredths: i64,
    pub effect_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResolutionResult {
    pub execution: CombatExecutionResult,
    pub frames: Vec<CombatResolutionFrame>,
    pub state: CombatResolutionState,
    pub full_log: Vec<CombatResolutionLogEvent>,
    pub core_log: Vec<CombatResolutionLogEvent>,
    pub fingerprint: String,
}

pub fn resolve(
    request: CombatResolutionRequest,
) -> Result<CombatResolutionResult, CombatResolutionError> {
    let execution =
        execute_combat(request.execution.clone()).map_err(CombatResolutionError::Execution)?;
    request
        .catalog
        .validate()
        .map_err(CombatResolutionError::State)?;
    let participants: BTreeMap<_, _> = request
        .execution
        .input
        .participants
        .iter()
        .map(|p| (p.id.clone(), p))
        .collect();
    let mut combatants = BTreeMap::new();
    for c in &request.execution.input.state.combatants {
        combatants.insert(
            c.id.clone(),
            CombatResolutionCombatant {
                id: c.id.clone(),
                current_health_hundredths: i64::from(c.current_health)
                    .checked_mul(100)
                    .ok_or(CombatResolutionError::Overflow)?,
                maximum_health_hundredths: i64::from(c.maximum_health)
                    .checked_mul(100)
                    .ok_or(CombatResolutionError::Overflow)?,
                balance_hundredths: i64::from(c.balance)
                    .checked_mul(100)
                    .ok_or(CombatResolutionError::Overflow)?,
                maximum_balance_hundredths: i64::from(c.maximum_balance)
                    .checked_mul(100)
                    .ok_or(CombatResolutionError::Overflow)?,
            },
        );
    }
    validate_inputs(&request, &participants, &combatants)?;
    let defenses: BTreeMap<_, _> = request
        .defenses
        .iter()
        .map(|d| (d.combatant_id.clone(), d))
        .collect();
    let attack_map: BTreeMap<_, _> = request.attacks.iter().map(|a| (a.id.clone(), a)).collect();
    let catalog: BTreeMap<_, _> = request
        .catalog
        .effects
        .iter()
        .map(|e| (e.id.clone(), e))
        .collect();
    let mut active_effects = request.execution.input.state.active_effects.clone();
    active_effects.sort_by(|a, b| {
        a.definition_id
            .cmp(&b.definition_id)
            .then(a.target_selector.cmp(&b.target_selector))
            .then(a.source.cmp(&b.source))
            .then(a.stacking_group.cmp(&b.stacking_group))
    });
    let mut applied = Vec::new();
    let mut suppressed = Vec::new();
    let mut frames = Vec::new();
    let mut full_log = Vec::new();

    // I2: 결착 tick 이후를 시뮬레이션하지 않는다. 판정 조건은 새로 만들지 않고
    // `conclude`가 쓰는 것과 같은 `side_all_defeated`를 쓴다.
    //
    // 활성 전투원이 없는 진영이 있으면 조기 종료를 판정하지 않는다 —
    // `all()`은 빈 집합에서 true이므로 첫 tick에 곧바로 결착으로 읽힌다.
    // 그 입력은 `conclude`가 `EmptyActiveSide`로 거부하는 몫이며, resolver가
    // 조용히 다르게 처리하지 않는다.
    let active_allies: Vec<&CombatSimulationParticipant> = participants
        .values()
        .copied()
        .filter(|p| p.active && p.side == CombatSide::Ally)
        .collect();
    let active_enemies: Vec<&CombatSimulationParticipant> = participants
        .values()
        .copied()
        .filter(|p| p.active && p.side == CombatSide::Enemy)
        .collect();
    let early_conclusion_is_decidable = !active_allies.is_empty() && !active_enemies.is_empty();

    // T3 §4-1/§4-2/§4-4: each attack definition's own, independent
    // attack-cadence gauge, carried across ticks. Keyed by attack id in a
    // `BTreeMap` (not tied to actor or target) because one actor can own
    // several attacks, each with its own rhythm.
    let mut attack_gauges: BTreeMap<&str, i64> =
        attack_map.keys().map(|id| (id.as_str(), 0i64)).collect();

    for frame in &execution.frames {
        // I1 (fable_combat_early_conclusion_step1_2608022130.md): whether an
        // actor/target is incapacitated is decided from THIS tick's starting
        // health, snapshotted once before any attack in the tick is applied.
        // Using the live `combatants` map instead would make the result
        // depend on `attack_map`'s (BTreeMap-by-id) processing order: an
        // actor killed earlier in the same tick would wrongly lose its own
        // already-in-flight attack, breaking simultaneous mutual knockouts
        // and order independence (I5, pinned by
        // `simultaneous_mutual_defeat_is_independent_of_attack_definition_order`).
        let health_snapshot: BTreeMap<String, i64> = combatants
            .iter()
            .map(|(id, c)| (id.clone(), c.current_health_hundredths))
            .collect();
        // T3 §4-1/§4-4: how many times each attack's cadence gauge crosses
        // `ACTION_THRESHOLD_HUNDREDTHS` this tick, decided from every
        // attack's *tick-start* gauge value in a pass that completes before
        // any attack in this tick is resolved. Each attack's gauge depends
        // only on its own speed and its own prior value -- never on another
        // attack's outcome, actor, or target -- so advancing it here in
        // `attack_map`'s fixed (id-sorted) order can never leak processing
        // order into the result (invariant 4, §4-4). A speed of `20_000`
        // crosses the threshold twice in one tick and fires twice, not once
        // (§4-1: don't clamp a cadence to "at most once per tick").
        let mut attack_fires: BTreeMap<&str, u32> = BTreeMap::new();
        for attack in attack_map.values() {
            let speed = attack
                .attack_speed_hundredths
                .unwrap_or(ACTION_THRESHOLD_HUNDREDTHS);
            let gauge = attack_gauges.get_mut(attack.id.as_str()).unwrap();
            *gauge = gauge
                .checked_add(speed)
                .ok_or(CombatResolutionError::Overflow)?;
            let mut fires = 0u32;
            while *gauge >= ACTION_THRESHOLD_HUNDREDTHS {
                fires += 1;
                *gauge -= ACTION_THRESHOLD_HUNDREDTHS;
            }
            attack_fires.insert(attack.id.as_str(), fires);
        }
        let mut outcomes = Vec::new();
        let mut sequence = 0;
        for attack in attack_map.values() {
            // T3 §4-1: this attack's cadence gauge did not cross the
            // threshold this tick -- it does not fire at all, independent of
            // whether it has a valid actor/target/move-intent this tick.
            let fires = attack_fires[attack.id.as_str()];
            if fires == 0 {
                continue;
            }
            let Some(intent) = frame.moves.iter().find(|m| m.actor_id == attack.actor_id) else {
                continue;
            };
            let Some(target_id) = &intent.target_id else {
                continue;
            };
            let (Some(actor), Some(target)) = (
                participants.get(&attack.actor_id),
                participants.get(target_id),
            ) else {
                continue;
            };
            if actor.side == target.side || !actor.active || !target.active {
                continue;
            }
            // I1: an incapacitated actor does not attack, and an
            // incapacitated target is not attacked -- judged from this
            // tick's starting health snapshot (see comment above the loop).
            // No roll, no log, no outcome is created for either case. A
            // missing snapshot entry is NOT treated as incapacitated (rule
            // 4, never fabricate a value): `validate_inputs` already
            // guarantees the actor is a tracked combatant, and a target with
            // no tracked state must still fall through to the existing
            // `InvalidInput` error below rather than being silently skipped.
            let actor_incapacitated = health_snapshot
                .get(&attack.actor_id)
                .is_some_and(|h| *h <= 0);
            let target_incapacitated = health_snapshot.get(target_id).is_some_and(|h| *h <= 0);
            if actor_incapacitated || target_incapacitated {
                continue;
            }
            // T3 §4-1: `fires` (>= 1 here) repeats this attack's resolution
            // that many times within this one tick, each producing its own
            // outcome and log entries -- a fast attack's extra actions are
            // not folded into a single bigger hit the way movement folds
            // extra actions into a longer step (§4-1's "don't clamp to one
            // per tick" applies just the same to attacks, and an attack's
            // per-fire roll is what makes each fire an independent judgment,
            // unlike movement's deterministic distance walk). `fire_index`
            // (0-based) is mixed into the roll `stream` below purely to keep
            // repeated fires from rolling identically against the same
            // tick/attack/actor/target -- deterministic, not new randomness
            // (invariant/§5 rule 6: no RNG call added). `fire_index == 0`
            // reproduces the exact stream value used before this slice, so
            // the default one-fire-per-tick path is byte-for-byte unchanged.
            for fire_index in 0..fires {
                // T1-b1 §4-1/§4-5: `CombatPosition::overlaps`/`in_range` no
                // longer exist -- `HexCoord` only offers `distance`. Both
                // predicates are now the plan's replacement formula
                // (`a.distance(b) <= i64::from(range)`) applied to different
                // thresholds. `collision_radius` keeps its old "combined melee
                // reach" meaning; only the metric under it moved from euclidean
                // to hex distance.
                //
                // T1-d §4-4 site 5/5: footprint distance, not anchor distance.
                // `combat_resolution.rs` only has each tick's frozen anchor
                // (`frame.positions`), not a live `position` field, but
                // `actor`/`target` (from `request.execution.input.participants`)
                // still carry their fixed `occupies` offset list -- the plan's
                // "프레임 스키마를 바꾸지 마라" (§4-4), satisfied by reading the
                // shape from the participant and the anchor from the frame
                // instead of adding a footprint field to the frame itself.
                let distance = footprint_distance(
                    frame.positions[&actor.id],
                    &actor.occupies,
                    frame.positions[target_id],
                    &target.occupies,
                )
                .map_err(|e| {
                    CombatResolutionError::Simulation(CombatSimulationError::HexMath(e))
                })?;
                let collision_reach =
                    i64::from(actor.collision_radius) + i64::from(target.collision_radius);
                let collision = distance <= collision_reach;
                let in_range = distance <= i64::from(attack.attack_range);
                // T3 §4-1: `fire_index` (0-based) is folded into the roll
                // stream so repeated fires of the same attack, in the same
                // tick, against the same target, don't roll identically --
                // deterministic, not new randomness. `fire_index == 0` is
                // exactly the stream value used before this slice.
                let roll_value = roll(
                    execution.effective_seed,
                    execution.namespace,
                    frame.tick,
                    &attack.id,
                    &actor.id,
                    target_id,
                    fire_index as u64,
                );
                let mut outcome = CombatAttackOutcome {
                    attack_id: attack.id.clone(),
                    actor_id: actor.id.clone(),
                    target_id: target_id.clone(),
                    collision,
                    in_range,
                    roll_percent: roll_value,
                    hit: false,
                    damage_hundredths: 0,
                    balance_delta_hundredths: 0,
                    applied_effect_ids: Vec::new(),
                    suppressed_effect_ids: Vec::new(),
                };
                full_log.push(log(
                    frame.tick,
                    sequence,
                    CombatResolutionLogTag::Collision,
                    CombatLogImportance::Routine,
                    attack,
                    target_id,
                    i64::from(collision),
                    None,
                ));
                sequence += 1;
                full_log.push(log(
                    frame.tick,
                    sequence,
                    CombatResolutionLogTag::AttackRoll,
                    CombatLogImportance::Important,
                    attack,
                    target_id,
                    i64::from(roll_value),
                    None,
                ));
                sequence += 1;
                outcome.hit = collision
                    && in_range
                    && (attack.accuracy_percent == 100
                        || (attack.accuracy_percent > 0 && roll_value < attack.accuracy_percent));
                if collision {
                    outcome.balance_delta_hundredths = outcome
                        .balance_delta_hundredths
                        .checked_sub(attack.collision_balance_hundredths)
                        .ok_or(CombatResolutionError::Overflow)?;
                }
                if outcome.hit {
                    let Some(defense) = defenses.get(target_id).copied() else {
                        return Err(CombatResolutionError::InvalidInput);
                    };
                    outcome.damage_hundredths = damage(attack, defense)?;
                    outcome.balance_delta_hundredths = outcome
                        .balance_delta_hundredths
                        .checked_sub(
                            attack
                                .balance_power_hundredths
                                .checked_sub(defense.balance_resistance_hundredths)
                                .unwrap_or(0),
                        )
                        .ok_or(CombatResolutionError::Overflow)?;
                    let target_state = combatants
                        .get_mut(target_id)
                        .ok_or(CombatResolutionError::InvalidInput)?;
                    target_state.current_health_hundredths = target_state
                        .current_health_hundredths
                        .saturating_sub(outcome.damage_hundredths)
                        .max(0);
                }
                let target_state = combatants
                    .get_mut(target_id)
                    .ok_or(CombatResolutionError::InvalidInput)?;
                target_state.balance_hundredths = target_state
                    .balance_hundredths
                    .checked_add(outcome.balance_delta_hundredths)
                    .ok_or(CombatResolutionError::Overflow)?
                    .clamp(0, target_state.maximum_balance_hundredths);
                if outcome.hit {
                    full_log.push(log(
                        frame.tick,
                        sequence,
                        CombatResolutionLogTag::DamageApplied,
                        CombatLogImportance::Decisive,
                        attack,
                        target_id,
                        outcome.damage_hundredths,
                        None,
                    ));
                    sequence += 1;
                    let mut effects = attack.effects.clone();
                    effects.sort_by(|a, b| {
                        a.effect_id
                            .cmp(&b.effect_id)
                            .then(a.chance_percent.cmp(&b.chance_percent))
                    });
                    for effect in &effects {
                        let effect_roll = roll(
                            execution.effective_seed,
                            execution.namespace,
                            frame.tick,
                            &attack.id,
                            &actor.id,
                            target_id,
                            effect
                                .effect_id
                                .as_bytes()
                                .iter()
                                .fold(1u64 + fire_index as u64, |a, b| {
                                    a.wrapping_add(u64::from(*b))
                                }),
                        );
                        if effect.chance_percent == 0
                            || (effect.chance_percent < 100 && effect_roll >= effect.chance_percent)
                        {
                            outcome.suppressed_effect_ids.push(effect.effect_id.clone());
                            suppressed.push(effect.effect_id.clone());
                            full_log.push(log(
                                frame.tick,
                                sequence,
                                CombatResolutionLogTag::EffectSuppressed,
                                CombatLogImportance::Important,
                                attack,
                                target_id,
                                i64::from(effect_roll),
                                Some(effect.effect_id.clone()),
                            ));
                            sequence += 1;
                            continue;
                        }
                        let def = catalog.get(&effect.effect_id).unwrap();
                        if apply_effect(&mut active_effects, def, target_id, &catalog) {
                            outcome.applied_effect_ids.push(effect.effect_id.clone());
                            applied.push(effect.effect_id.clone());
                            full_log.push(log(
                                frame.tick,
                                sequence,
                                CombatResolutionLogTag::EffectApplied,
                                CombatLogImportance::Important,
                                attack,
                                target_id,
                                i64::from(effect_roll),
                                Some(effect.effect_id.clone()),
                            ));
                            sequence += 1;
                        } else {
                            outcome.suppressed_effect_ids.push(effect.effect_id.clone());
                            suppressed.push(effect.effect_id.clone());
                            full_log.push(log(
                                frame.tick,
                                sequence,
                                CombatResolutionLogTag::EffectSuppressed,
                                CombatLogImportance::Important,
                                attack,
                                target_id,
                                i64::from(effect_roll),
                                Some(effect.effect_id.clone()),
                            ));
                            sequence += 1;
                        }
                    }
                }
                outcomes.push(outcome);
            }
        }
        let fp = fingerprint(&(frame.tick, &outcomes));
        // `combatants` is a `BTreeMap`, so this iterates id ascending already;
        // clone (not `into_values()`) because the map keeps accumulating across ticks.
        let tick_combatants = combatants.values().cloned().collect();
        frames.push(CombatResolutionFrame {
            tick: frame.tick,
            outcomes,
            combatants: tick_combatants,
            fingerprint: fp,
        });

        // 결착 tick의 프레임은 남기고 그 뒤를 돌지 않는다 (정본 03: 결착 시
        // 정리가 일어나므로 결착 이후를 계속 시뮬레이션할 근거가 없다).
        if early_conclusion_is_decidable {
            // 추적되지 않는 id는 "전멸"로 읽지 않는다 — 없는 상태를 결착
            // 근거로 삼지 않는다. `validate_inputs`가 참가자와 상태의 일치를
            // 이미 보장하므로 이 분기는 도달하지 않으며, 따라서 테스트로
            // 고정할 수 없다. 방어용으로만 둔다 (`mutate_ec` M5는 잡히지
            // 않는다 — 도달 불가라서 그렇다).
            const UNTRACKED_IS_NOT_DEFEATED: i64 = i64::MAX;
            let health_of = |id: &str| {
                combatants
                    .get(id)
                    .map_or(UNTRACKED_IS_NOT_DEFEATED, |c| c.current_health_hundredths)
            };
            let allies_defeated = side_all_defeated(active_allies.iter().copied(), &health_of);
            let enemies_defeated = side_all_defeated(active_enemies.iter().copied(), &health_of);
            if allies_defeated || enemies_defeated {
                break;
            }
        }
    }
    let state = CombatResolutionState {
        combatants: combatants.into_values().collect(),
        active_effects,
        applied_effect_ids: applied,
        suppressed_effect_ids: suppressed,
    };
    let core_log = full_log
        .iter()
        .filter(|e| e.importance >= CombatLogImportance::Important)
        .cloned()
        .collect();
    let fp = fingerprint(&(execution.fingerprint.clone(), &frames, &state, &full_log));
    Ok(CombatResolutionResult {
        execution,
        frames,
        state,
        full_log,
        core_log,
        fingerprint: fp,
    })
}

fn validate_inputs(
    r: &CombatResolutionRequest,
    participants: &BTreeMap<String, &crate::CombatSimulationParticipant>,
    combatants: &BTreeMap<String, CombatResolutionCombatant>,
) -> Result<(), CombatResolutionError> {
    let mut ids = BTreeSet::new();
    for a in &r.attacks {
        if a.id.trim().is_empty()
            || !ids.insert(&a.id)
            || !participants.contains_key(&a.actor_id)
            || !combatants.contains_key(&a.actor_id)
            || a.power_hundredths < 0
            || a.ability_multiplier_hundredths < 0
            || a.accuracy_percent > 100
            || a.attack_range < 0
            || a.penetration_hundredths < 0
            || a.collision_balance_hundredths < 0
            || a.balance_power_hundredths < 0
            // T3 §4-3: `Some(v)` with `v <= 0` is an input error, not "never
            // fires" -- a fabricated meaning the plan explicitly forbids
            // inventing. `None` (unset) is the only way to mean "every tick".
            || a.attack_speed_hundredths.is_some_and(|v| v <= 0)
        {
            return Err(CombatResolutionError::InvalidInput);
        }
        for e in &a.effects {
            if e.chance_percent > 100 || !r.catalog.effects.iter().any(|d| d.id == e.effect_id) {
                return Err(CombatResolutionError::UnknownEffect(e.effect_id.clone()));
            }
        }
    }
    let mut defenses = BTreeSet::new();
    for d in &r.defenses {
        if !combatants.contains_key(&d.combatant_id)
            || d.defense_hundredths < 0
            || d.balance_resistance_hundredths < 0
            || !defenses.insert(&d.combatant_id)
        {
            return Err(CombatResolutionError::InvalidInput);
        }
    }
    for id in participants.keys() {
        if combatants.contains_key(id) && !defenses.contains(id) {
            return Err(CombatResolutionError::InvalidInput);
        }
    }
    Ok(())
}
fn damage(
    a: &CombatAttackDefinition,
    d: &CombatDefenseProfile,
) -> Result<i64, CombatResolutionError> {
    let pre = a
        .power_hundredths
        .checked_mul(5)
        .and_then(|v| v.checked_mul(a.ability_multiplier_hundredths))
        .ok_or(CombatResolutionError::Overflow)?
        .checked_div(1200)
        .ok_or(CombatResolutionError::Overflow)?;
    let effective = (d.defense_hundredths - a.penetration_hundredths).max(0);
    let reduction = pre
        .checked_mul(effective)
        .ok_or(CombatResolutionError::Overflow)?
        .checked_div(
            effective
                .checked_add(2000)
                .ok_or(CombatResolutionError::Overflow)?,
        )
        .ok_or(CombatResolutionError::Overflow)?;
    Ok(pre.saturating_sub(reduction).max(0))
}
fn apply_effect(
    active: &mut Vec<CombatEffectInstance>,
    def: &CombatEffectDefinition,
    target: &str,
    catalog: &BTreeMap<String, &CombatEffectDefinition>,
) -> bool {
    let same: Vec<usize> = active
        .iter()
        .enumerate()
        .filter(|(_, e)| e.target_selector == target && e.stacking_group == def.stacking_group)
        .map(|(i, _)| i)
        .collect();
    let allowed = match def.stacking {
        EffectStacking::Unique => same.is_empty(),
        EffectStacking::Replace | EffectStacking::DurationRefresh => {
            for i in same.iter().rev().copied() {
                active.remove(i);
            }
            true
        }
        EffectStacking::Strongest => {
            let should_replace = same.first().is_none_or(|i| {
                let current = &active[*i];
                let current_priority = catalog
                    .get(&current.definition_id)
                    .map(|definition| definition.priority)
                    .unwrap_or(i32::MIN);
                def.priority > current_priority
                    || (def.priority == current_priority && def.id < current.definition_id)
            });
            if should_replace {
                for i in same.iter().rev().copied() {
                    active.remove(i);
                }
            }
            should_replace
        }
        EffectStacking::AdditiveWithCap => def
            .stacking_cap
            .is_some_and(|cap| same.len() < cap as usize),
        EffectStacking::StackCount => def
            .stacking_cap
            .map(|cap| same.len() < cap as usize)
            .unwrap_or(true),
        EffectStacking::Independent => true,
    };
    if !allowed {
        return false;
    }
    active.push(CombatEffectInstance {
        definition_id: def.id.clone(),
        source: def.source.clone(),
        combat_only: !matches!(def.lifetime, EffectLifetime::Persistent),
        target_selector: target.into(),
        parameters: def.parameters.clone(),
        phase: def.phase,
        lifetime: def.lifetime.clone(),
        stacking_group: def.stacking_group.clone(),
    });
    true
}
fn log(
    tick: u32,
    sequence: u32,
    tag: CombatResolutionLogTag,
    importance: CombatLogImportance,
    a: &CombatAttackDefinition,
    target: &str,
    value: i64,
    effect_id: Option<String>,
) -> CombatResolutionLogEvent {
    CombatResolutionLogEvent {
        tick,
        sequence,
        tag,
        importance,
        attack_id: a.id.clone(),
        actor_id: a.actor_id.clone(),
        target_id: target.into(),
        value_hundredths: value,
        effect_id,
    }
}
fn roll(
    seed: u64,
    namespace: CombatRngNamespace,
    tick: u32,
    attack: &str,
    actor: &str,
    target: &str,
    stream: u64,
) -> u8 {
    (fnv(format!(
        "{}:{}:{}:{}:{}:{}:{}",
        seed,
        namespace.as_str(),
        tick,
        attack,
        actor,
        target,
        stream
    )
    .as_bytes())
        % 100) as u8
}
fn fingerprint<T: Serialize>(value: &T) -> String {
    format!(
        "{:016x}",
        fnv(&serde_json::to_vec(value).unwrap_or_default())
    )
}
fn fnv(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatResolutionError {
    Execution(CombatExecutionError),
    State(crate::CombatStateError),
    Simulation(CombatSimulationError),
    InvalidInput,
    UnknownEffect(String),
    Overflow,
}
impl std::fmt::Display for CombatResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatResolutionError {}

#[allow(dead_code)]
pub(crate) struct CombatResolutionStepper {
    pub(crate) execution: CombatExecutionResult,
    pub(crate) participants: BTreeMap<String, CombatSimulationParticipant>,
    pub(crate) combatants: BTreeMap<String, CombatResolutionCombatant>,
    pub(crate) defenses: BTreeMap<String, CombatDefenseProfile>,
    pub(crate) attacks: BTreeMap<String, CombatAttackDefinition>,
    pub(crate) catalog: BTreeMap<String, CombatEffectDefinition>,
    pub(crate) active_effects: Vec<CombatEffectInstance>,
    pub(crate) attack_gauges: BTreeMap<String, i64>,
    pub(crate) applied: Vec<String>,
    pub(crate) suppressed: Vec<String>,
    pub(crate) full_log: Vec<CombatResolutionLogEvent>,
}
impl CombatResolutionStepper {
    pub(crate) fn new(
        request: &CombatResolutionRequest,
        execution: CombatExecutionResult,
    ) -> Result<Self, CombatResolutionError> {
        request
            .catalog
            .validate()
            .map_err(CombatResolutionError::State)?;
        let participants = request
            .execution
            .input
            .participants
            .iter()
            .map(|p| (p.id.clone(), p.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut combatants = BTreeMap::new();
        for c in &request.execution.input.state.combatants {
            combatants.insert(
                c.id.clone(),
                CombatResolutionCombatant {
                    id: c.id.clone(),
                    current_health_hundredths: i64::from(c.current_health)
                        .checked_mul(100)
                        .ok_or(CombatResolutionError::Overflow)?,
                    maximum_health_hundredths: i64::from(c.maximum_health)
                        .checked_mul(100)
                        .ok_or(CombatResolutionError::Overflow)?,
                    balance_hundredths: i64::from(c.balance)
                        .checked_mul(100)
                        .ok_or(CombatResolutionError::Overflow)?,
                    maximum_balance_hundredths: i64::from(c.maximum_balance)
                        .checked_mul(100)
                        .ok_or(CombatResolutionError::Overflow)?,
                },
            );
        }
        let refs = participants.iter().map(|(id, p)| (id.clone(), p)).collect();
        validate_inputs(request, &refs, &combatants)?;
        let defenses = request
            .defenses
            .iter()
            .map(|d| (d.combatant_id.clone(), d.clone()))
            .collect();
        let attacks = request
            .attacks
            .iter()
            .map(|a| (a.id.clone(), a.clone()))
            .collect::<BTreeMap<_, _>>();
        let catalog = request
            .catalog
            .effects
            .iter()
            .map(|e| (e.id.clone(), e.clone()))
            .collect();
        let mut active_effects = request.execution.input.state.active_effects.clone();
        active_effects.sort_by(|a, b| {
            a.definition_id
                .cmp(&b.definition_id)
                .then(a.target_selector.cmp(&b.target_selector))
                .then(a.source.cmp(&b.source))
                .then(a.stacking_group.cmp(&b.stacking_group))
        });
        let attack_gauges = attacks.keys().map(|id| (id.clone(), 0)).collect();
        Ok(Self {
            execution,
            participants,
            combatants,
            defenses,
            attacks,
            catalog,
            active_effects,
            attack_gauges,
            applied: vec![],
            suppressed: vec![],
            full_log: vec![],
        })
    }
}

impl CombatResolutionStepper {
    pub(crate) fn step(
        &mut self,
        frame: &crate::CombatTickFrame,
    ) -> Result<CombatResolutionFrame, CombatResolutionError> {
        let _health_snapshot: BTreeMap<String, i64> = self
            .combatants
            .iter()
            .map(|(id, combatant)| (id.clone(), combatant.current_health_hundredths))
            .collect();
        let mut _attack_fires = BTreeMap::new();
        for attack in self.attacks.values() {
            let speed = attack
                .attack_speed_hundredths
                .unwrap_or(ACTION_THRESHOLD_HUNDREDTHS);
            let gauge = self
                .attack_gauges
                .get_mut(&attack.id)
                .ok_or(CombatResolutionError::InvalidInput)?;
            *gauge = gauge
                .checked_add(speed)
                .ok_or(CombatResolutionError::Overflow)?;
            let mut fires = 0u32;
            while *gauge >= ACTION_THRESHOLD_HUNDREDTHS {
                fires += 1;
                *gauge -= ACTION_THRESHOLD_HUNDREDTHS;
            }
            _attack_fires.insert(attack.id.clone(), fires);
        }
        let outcomes = Vec::new();
        let combatants = self.combatants.values().cloned().collect();
        let fingerprint = fingerprint(&(frame.tick, &outcomes));
        Ok(CombatResolutionFrame {
            tick: frame.tick,
            outcomes,
            combatants,
            fingerprint,
        })
    }
}
