use crate::{
    execute_combat, CombatEffectCatalog, CombatEffectDefinition, CombatEffectInstance,
    CombatExecutionError, CombatExecutionRequest, CombatExecutionResult, CombatLogImportance,
    CombatRngNamespace, CombatSimulationError, EffectLifetime, EffectStacking,
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
    for frame in &execution.frames {
        let mut outcomes = Vec::new();
        let mut sequence = 0;
        for attack in attack_map.values() {
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
            let collision = frame.positions[&actor.id]
                .overlaps(
                    frame.positions[target_id],
                    actor
                        .collision_radius
                        .checked_add(target.collision_radius)
                        .ok_or(CombatResolutionError::Overflow)?,
                )
                .map_err(CombatResolutionError::Simulation)?;
            let in_range = frame.positions[&actor.id]
                .in_range(frame.positions[target_id], attack.attack_range)
                .map_err(CombatResolutionError::Simulation)?;
            let roll_value = roll(
                execution.effective_seed,
                execution.namespace,
                frame.tick,
                &attack.id,
                &actor.id,
                target_id,
                0,
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
                            .fold(1u64, |a, b| a.wrapping_add(u64::from(*b))),
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
        let fp = fingerprint(&(frame.tick, &outcomes));
        frames.push(CombatResolutionFrame {
            tick: frame.tick,
            outcomes,
            fingerprint: fp,
        });
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
