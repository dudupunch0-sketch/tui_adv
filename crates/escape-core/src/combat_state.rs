use crate::{CombatContractError, CombatManifest};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatantState {
    pub id: String,
    pub current_health: i32,
    pub maximum_health: i32,
    pub current_breath: i32,
    pub maximum_breath: i32,
    pub balance: i32,
    pub maximum_balance: i32,
    pub fear: i32,
    pub anger: i32,
    pub posture: Posture,
    pub weapon_control: WeaponControl,
    #[serde(default)]
    pub relationship_refs: Vec<String>,
    #[serde(default)]
    pub environment_refs: Vec<String>,
    #[serde(default)]
    pub team_refs: Vec<String>,
    #[serde(default)]
    pub persistent_status_ids: Vec<String>,
    #[serde(default)]
    pub combat_effect_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    Neutral,
    Attack,
    Defense,
    Evasion,
    Grappling,
    Down,
    Rising,
    Staggered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeaponControl {
    Stable,
    Unstable,
    Held,
    Pinned,
    Dropped,
    Broken,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentCombatStatus {
    pub id: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatEffectInstance {
    pub definition_id: String,
    pub source: String,
    pub combat_only: bool,
    pub target_selector: String,
    pub parameters: BTreeMap<String, i64>,
    pub phase: EffectPhase,
    pub lifetime: EffectLifetime,
    pub stacking_group: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatState {
    pub battle_id: String,
    pub combatants: Vec<CombatantState>,
    #[serde(default)]
    pub persistent_statuses: Vec<PersistentCombatStatus>,
    #[serde(default)]
    pub active_effects: Vec<CombatEffectInstance>,
    #[serde(default)]
    pub environment_refs: Vec<String>,
    #[serde(default)]
    pub team_refs: Vec<String>,
    #[serde(default)]
    pub team_formations: Vec<TeamFormationState>,
    #[serde(default)]
    pub relationships: Vec<RelationshipState>,
    #[serde(default)]
    pub environment_states: Vec<EnvironmentState>,
    pub manifest_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamFormationState {
    pub team_id: String,
    pub formation_id: String,
    pub cohesion: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipState {
    pub id: String,
    pub value: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub id: String,
    pub distance: i32,
    pub pressure: i32,
    pub visibility: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatConclusion {
    pub persistent_statuses: Vec<PersistentCombatStatus>,
    pub retained_effects: Vec<CombatEffectInstance>,
    pub removed_combat_effect_ids: Vec<String>,
}

impl CombatState {
    pub fn canonical_json(&self) -> Result<String, CombatStateError> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical.combatants.sort_by(|a, b| a.id.cmp(&b.id));
        canonical
            .persistent_statuses
            .sort_by(|a, b| a.id.cmp(&b.id));
        canonical.active_effects.sort_by(|a, b| {
            a.definition_id
                .cmp(&b.definition_id)
                .then(a.source.cmp(&b.source))
                .then(a.target_selector.cmp(&b.target_selector))
                .then(a.phase.cmp(&b.phase))
                .then(a.lifetime.cmp(&b.lifetime))
                .then(a.stacking_group.cmp(&b.stacking_group))
                .then(a.parameters.cmp(&b.parameters))
                .then(a.combat_only.cmp(&b.combat_only))
        });
        for combatant in &mut canonical.combatants {
            combatant.relationship_refs.sort();
            combatant.environment_refs.sort();
            combatant.team_refs.sort();
            combatant.persistent_status_ids.sort();
            combatant.combat_effect_ids.sort();
        }
        canonical.environment_refs.sort();
        canonical.team_refs.sort();
        canonical.team_formations.sort_by(|a, b| {
            a.team_id
                .cmp(&b.team_id)
                .then(a.formation_id.cmp(&b.formation_id))
                .then(a.cohesion.cmp(&b.cohesion))
        });
        canonical
            .relationships
            .sort_by(|a, b| a.id.cmp(&b.id).then(a.value.cmp(&b.value)));
        canonical.environment_states.sort_by(|a, b| {
            a.id.cmp(&b.id)
                .then(a.distance.cmp(&b.distance))
                .then(a.pressure.cmp(&b.pressure))
                .then(a.visibility.cmp(&b.visibility))
        });
        serde_json::to_string(&canonical)
            .map_err(|e| CombatStateError::Serialization(e.to_string()))
    }
    pub fn fingerprint(&self) -> Result<String, CombatStateError> {
        Ok(format!(
            "{:016x}",
            fnv1a_64(self.canonical_json()?.as_bytes())
        ))
    }
    pub fn validate(&self) -> Result<(), CombatStateError> {
        ensure_id(&self.battle_id)?;
        ensure_id(&self.manifest_fingerprint)?;
        let mut seen = std::collections::BTreeSet::new();
        for combatant in &self.combatants {
            ensure_id(&combatant.id)?;
            if !seen.insert(&combatant.id) {
                return Err(CombatStateError::DuplicateId(combatant.id.clone()));
            }
            if invalid_range(combatant.current_health, combatant.maximum_health)
                || invalid_range(combatant.current_breath, combatant.maximum_breath)
                || invalid_range(combatant.balance, combatant.maximum_balance)
                || combatant.fear < 0
                || combatant.anger < 0
            {
                return Err(CombatStateError::InvalidHealth(combatant.id.clone()));
            }
            for id in combatant
                .relationship_refs
                .iter()
                .chain(&combatant.environment_refs)
                .chain(&combatant.team_refs)
                .chain(&combatant.persistent_status_ids)
                .chain(&combatant.combat_effect_ids)
            {
                ensure_id(id)?;
            }
        }
        for formation in &self.team_formations {
            ensure_id(&formation.team_id)?;
            ensure_id(&formation.formation_id)?;
            if formation.cohesion < 0 {
                return Err(CombatStateError::InvalidValue(formation.team_id.clone()));
            }
        }
        for relation in &self.relationships {
            ensure_id(&relation.id)?;
        }
        for environment in &self.environment_states {
            ensure_id(&environment.id)?;
            if environment.distance < 0 || environment.pressure < 0 || environment.visibility < 0 {
                return Err(CombatStateError::InvalidValue(environment.id.clone()));
            }
        }
        for status in &self.persistent_statuses {
            ensure_id(&status.id)?;
            ensure_id(&status.source)?;
        }
        for effect in &self.active_effects {
            ensure_id(&effect.definition_id)?;
            ensure_id(&effect.source)?;
            ensure_id(&effect.target_selector)?;
            ensure_id(&effect.stacking_group)?;
        }
        Ok(())
    }

    pub fn conclude(&self) -> Result<CombatConclusion, CombatStateError> {
        self.validate()?;
        let mut retained_effects = Vec::new();
        let mut removed_combat_effect_ids = Vec::new();
        for effect in &self.active_effects {
            if effect.combat_only {
                removed_combat_effect_ids.push(effect.definition_id.clone());
            } else {
                retained_effects.push(effect.clone());
            }
        }
        Ok(CombatConclusion {
            persistent_statuses: self.persistent_statuses.clone(),
            retained_effects,
            removed_combat_effect_ids,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatEffectCategory {
    State,
    Ai,
    Space,
    Information,
    ConditionalRule,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectPhase {
    BeforePlacement,
    DuringPlacement,
    AfterPlacementConfirmed,
    CombatStart,
    DuringCombat,
    CombatSettlement,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectLifetime {
    Instant,
    Seconds(u32),
    UntilConditionClears,
    Uses(u32),
    UntilCombatSettlement,
    SourceLinked,
    Persistent,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectStacking {
    Unique,
    Strongest,
    AdditiveWithCap,
    StackCount,
    DurationRefresh,
    Independent,
    Replace,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectVisibility {
    Public,
    Hidden,
    Conditional,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatEffectDefinition {
    pub id: String,
    pub source: String,
    pub category: CombatEffectCategory,
    pub target_selector: String,
    pub parameters: BTreeMap<String, i64>,
    #[serde(default)]
    pub conditions: Vec<String>,
    pub phase: EffectPhase,
    pub lifetime: EffectLifetime,
    pub stacking: EffectStacking,
    pub stacking_group: String,
    pub stacking_cap: Option<u32>,
    pub priority: i32,
    pub visibility: EffectVisibility,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatEffectCatalog {
    pub effects: Vec<CombatEffectDefinition>,
}
impl CombatEffectCatalog {
    pub fn validate(&self) -> Result<(), CombatStateError> {
        let mut seen = std::collections::BTreeSet::new();
        for effect in &self.effects {
            ensure_id(&effect.id)?;
            ensure_id(&effect.source)?;
            ensure_id(&effect.target_selector)?;
            if !seen.insert(&effect.id) {
                return Err(CombatStateError::DuplicateId(effect.id.clone()));
            }
            for condition in &effect.conditions {
                ensure_id(condition)?;
            }
            for tag in &effect.tags {
                ensure_id(tag)?;
            }
            if let EffectLifetime::Seconds(count) | EffectLifetime::Uses(count) = effect.lifetime {
                if count == 0 {
                    return Err(CombatStateError::InvalidLifetime(effect.id.clone()));
                }
            }
            ensure_id(&effect.stacking_group)?;
            if matches!(effect.stacking, EffectStacking::AdditiveWithCap)
                && effect.stacking_cap.unwrap_or(0) == 0
            {
                return Err(CombatStateError::InvalidStacking(effect.id.clone()));
            }
        }
        Ok(())
    }
    pub fn canonical(&self) -> Result<Vec<CombatEffectDefinition>, CombatStateError> {
        self.validate()?;
        let mut effects = self.effects.clone();
        for effect in &mut effects {
            effect.conditions.sort();
            effect.tags.sort();
        }
        effects.sort_by(|a, b| {
            a.phase
                .cmp(&b.phase)
                .then(a.priority.cmp(&b.priority))
                .then(a.id.cmp(&b.id))
        });
        Ok(effects)
    }
    pub fn canonical_json(&self) -> Result<String, CombatStateError> {
        serde_json::to_string(&self.canonical()?)
            .map_err(|e| CombatStateError::Serialization(e.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatPreCombatInput {
    pub manifest: CombatManifest,
    pub battle_id: String,
    pub combatants: Vec<CombatantState>,
    #[serde(default)]
    pub persistent_statuses: Vec<PersistentCombatStatus>,
    #[serde(default)]
    pub candidate_effect_ids: Vec<String>,
    #[serde(default)]
    pub active_condition_ids: Vec<String>,
    #[serde(default)]
    pub environment_refs: Vec<String>,
    #[serde(default)]
    pub team_refs: Vec<String>,
    #[serde(default)]
    pub team_formations: Vec<TeamFormationState>,
    #[serde(default)]
    pub relationships: Vec<RelationshipState>,
    #[serde(default)]
    pub environment_states: Vec<EnvironmentState>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatEffectDecision {
    pub id: String,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatInitialStateProjection {
    pub state: CombatState,
    pub applied_effects: Vec<CombatEffectDecision>,
    pub suppressed_effects: Vec<CombatEffectDecision>,
    pub manifest: CombatManifest,
    pub final_state_fingerprint: String,
}

impl CombatInitialStateProjection {
    pub fn canonical_json(&self) -> Result<String, CombatStateError> {
        let mut applied = self.applied_effects.clone();
        let mut suppressed = self.suppressed_effects.clone();
        applied.sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
        suppressed.sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
        let value = serde_json::json!({
            "state": serde_json::from_str::<serde_json::Value>(&self.state.canonical_json()?).map_err(|e| CombatStateError::Serialization(e.to_string()))?,
            "applied_effects": applied,
            "suppressed_effects": suppressed,
            "manifest": serde_json::from_str::<serde_json::Value>(&self.manifest.canonical_json().map_err(CombatStateError::Contract)?).map_err(|e| CombatStateError::Serialization(e.to_string()))?,
            "final_state_fingerprint": self.final_state_fingerprint,
        });
        serde_json::to_string(&value).map_err(|e| CombatStateError::Serialization(e.to_string()))
    }
    pub fn fingerprint(&self) -> Result<String, CombatStateError> {
        Ok(format!(
            "{:016x}",
            fnv1a_64(self.canonical_json()?.as_bytes())
        ))
    }

    pub fn project(
        input: &CombatPreCombatInput,
        catalog: &CombatEffectCatalog,
    ) -> Result<Self, CombatStateError> {
        input
            .manifest
            .validate()
            .map_err(CombatStateError::Contract)?;
        catalog.validate()?;
        ensure_id(&input.battle_id)?;
        let candidates: std::collections::BTreeSet<_> = input.candidate_effect_ids.iter().collect();
        let conditions: std::collections::BTreeSet<_> = input.active_condition_ids.iter().collect();
        let catalog_ids: std::collections::BTreeSet<_> = catalog
            .effects
            .iter()
            .map(|effect| effect.id.as_str())
            .collect();
        for candidate in &input.candidate_effect_ids {
            if !catalog_ids.contains(candidate.as_str()) {
                return Err(CombatStateError::UnknownEffect(candidate.clone()));
            }
        }
        let mut applied_effects = Vec::new();
        let mut suppressed_effects = Vec::new();
        let mut active_effects = Vec::new();
        let mut selected_groups = std::collections::BTreeMap::new();
        for effect in catalog.canonical()? {
            if !candidates.contains(&effect.id) {
                suppressed_effects.push(CombatEffectDecision {
                    id: effect.id,
                    reason: "not_candidate".into(),
                });
                continue;
            }
            if effect
                .conditions
                .iter()
                .any(|condition| !conditions.contains(condition))
            {
                suppressed_effects.push(CombatEffectDecision {
                    id: effect.id,
                    reason: "condition_not_met".into(),
                });
                continue;
            }
            if matches!(
                effect.stacking,
                EffectStacking::Unique | EffectStacking::Replace
            ) {
                if let Some(winner) = selected_groups.get(&effect.stacking_group) {
                    suppressed_effects.push(CombatEffectDecision {
                        id: effect.id,
                        reason: format!("stacking_conflict:{winner}"),
                    });
                    continue;
                }
                selected_groups.insert(effect.stacking_group.clone(), effect.id.clone());
            }
            let combat_only = !matches!(effect.lifetime, EffectLifetime::Persistent);
            applied_effects.push(CombatEffectDecision {
                id: effect.id.clone(),
                reason: "applied".into(),
            });
            active_effects.push(CombatEffectInstance {
                definition_id: effect.id,
                source: effect.source,
                combat_only,
                target_selector: effect.target_selector,
                parameters: effect.parameters,
                phase: effect.phase,
                lifetime: effect.lifetime,
                stacking_group: effect.stacking_group,
            });
        }
        let fingerprint = input
            .manifest
            .fingerprint()
            .map_err(CombatStateError::Contract)?;
        let state = CombatState {
            battle_id: input.battle_id.clone(),
            combatants: input.combatants.clone(),
            persistent_statuses: input.persistent_statuses.clone(),
            active_effects,
            environment_refs: input.environment_refs.clone(),
            team_refs: input.team_refs.clone(),
            team_formations: input.team_formations.clone(),
            relationships: input.relationships.clone(),
            environment_states: input.environment_states.clone(),
            manifest_fingerprint: fingerprint,
        };
        state.validate()?;
        let mut manifest = input.manifest.clone();
        manifest.applied_effects = applied_effects
            .iter()
            .map(|e| crate::CombatEffectRef {
                id: e.id.clone(),
                reason: e.reason.clone(),
            })
            .collect();
        manifest.suppressed_effects = suppressed_effects
            .iter()
            .map(|e| crate::SuppressedCombatEffect {
                id: e.id.clone(),
                reason: e.reason.clone(),
            })
            .collect();
        manifest.combatant_ids = state.combatants.iter().map(|c| c.id.clone()).collect();
        manifest.environment_ids = state.environment_refs.clone();
        manifest.team_ids = state.team_refs.clone();
        let final_state_fingerprint = state.fingerprint()?;
        Ok(Self {
            state,
            applied_effects,
            suppressed_effects,
            manifest,
            final_state_fingerprint,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatStateError {
    EmptyId,
    DuplicateId(String),
    InvalidHealth(String),
    InvalidLifetime(String),
    InvalidStacking(String),
    InvalidValue(String),
    UnknownEffect(String),
    Serialization(String),
    Contract(CombatContractError),
}
impl fmt::Display for CombatStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatStateError {}
fn ensure_id(id: &str) -> Result<(), CombatStateError> {
    if id.trim().is_empty() {
        Err(CombatStateError::EmptyId)
    } else {
        Ok(())
    }
}

fn invalid_range(current: i32, maximum: i32) -> bool {
    maximum < 0 || current < 0 || current > maximum
}
fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
