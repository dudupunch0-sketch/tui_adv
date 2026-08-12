use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const EXECUTOR: &[&str] = &[
    "combat.selector.executor.v1.observer",
    "combat.selector.executor.v1.any_capable",
];
const TARGET: &[&str] = &[
    "combat.selector.target.v1.executor_self",
    "combat.selector.target.v1.selected_target",
    "combat.selector.target.v1.nearest_active_enemy",
    "combat.selector.target.v1.lowest_health_active_ally",
    "combat.selector.target.v1.surrounded_active_ally",
    "combat.selector.target.v1.all_active_allies",
];
const RULE: &[&str] = &[
    "combat.strategy.targeting.v1.attackers_of",
    "combat.strategy.targeting.v1.rearmost_active_enemy",
    "combat.strategy.targeting.v1.focus_resolved_target",
];
const WEIGHTS: &[&str] = &[
    "preferred_distance",
    "aggression",
    "formation_maintenance",
    "pursuit_range",
    "protect_priority",
    "target_priority",
    "risk_tolerance",
    "ability_priority",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatInterventionError {
    EmptyPayload,
    EmptyOperations,
    EmptyField(&'static str),
    InvalidField(String),
    DuplicateId(String),
    UnknownCanonicalId(String),
    LegacyAlias(String),
}
impl std::fmt::Display for CombatInterventionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatInterventionError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StrategyScope {
    AllAllies {},
    Role { role_id: String },
    Combatants { combatant_selector_ids: Vec<String> },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyDuration {
    UntilReplaced,
    NextSegment,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StrategyOperation {
    SetRoleWeight { field: String, value: i32 },
    SetTargetingRule { rule_id: String },
    SetTargetPolicy { policy_id: String },
    ClearOverride { field: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatStrategyModifier {
    pub scope: StrategyScope,
    pub duration: StrategyDuration,
    pub operations: Vec<StrategyOperation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatEffectBranch {
    pub effect_ids: Vec<String>,
    pub outcome_actions: Vec<CombatOutcomeAction>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatClaimPolicy {
    DefaultTerminalPolicy,
    AlwaysClaimable,
    NeverClaimable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CombatOutcomeAction {
    SetFlag {
        flag_id: String,
    },
    CreateLootEntitlement {
        item_id: String,
        source_selector_id: String,
        claim_policy: CombatClaimPolicy,
    },
    GrantItem {
        item_id: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatSpecialEffect {
    pub formula_id: String,
    pub formula_parameters: BTreeMap<String, serde_json::Value>,
    pub executor_selector_id: String,
    pub target_selector_id: String,
    pub success: CombatEffectBranch,
    pub failure: CombatEffectBranch,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatInterventionPayload {
    pub strategy_modifier: Option<CombatStrategyModifier>,
    pub special_effect: Option<CombatSpecialEffect>,
}

impl CombatInterventionPayload {
    pub fn validate(&self) -> Result<(), CombatInterventionError> {
        if self.strategy_modifier.is_none() && self.special_effect.is_none() {
            return Err(CombatInterventionError::EmptyPayload);
        }
        if let Some(v) = &self.strategy_modifier {
            v.validate()?
        }
        if let Some(v) = &self.special_effect {
            v.validate()?
        }
        Ok(())
    }
}
impl CombatStrategyModifier {
    pub fn validate(&self) -> Result<(), CombatInterventionError> {
        match &self.scope {
            StrategyScope::AllAllies {} => {}
            StrategyScope::Role { role_id } => nonempty(role_id, "role_id")?,
            StrategyScope::Combatants {
                combatant_selector_ids,
            } => {
                if combatant_selector_ids.is_empty() {
                    return Err(CombatInterventionError::EmptyField(
                        "combatant_selector_ids",
                    ));
                }
                unique(combatant_selector_ids)?;
                for id in combatant_selector_ids {
                    registry(TARGET, id)?
                }
            }
        }
        if self.operations.is_empty() {
            return Err(CombatInterventionError::EmptyOperations);
        }
        for op in &self.operations {
            match op {
                StrategyOperation::SetRoleWeight { field, .. } => {
                    if !WEIGHTS.contains(&field.as_str()) {
                        return Err(CombatInterventionError::InvalidField(field.clone()));
                    }
                }
                StrategyOperation::SetTargetingRule { rule_id } => registry(RULE, rule_id)?,
                StrategyOperation::SetTargetPolicy { policy_id } => {
                    nonempty(policy_id, "policy_id")?
                }
                StrategyOperation::ClearOverride { field } => {
                    if !["role_weight", "targeting_rule", "target_policy"].contains(&field.as_str())
                    {
                        return Err(CombatInterventionError::InvalidField(field.clone()));
                    }
                }
            }
        }
        Ok(())
    }
}
impl CombatSpecialEffect {
    pub fn validate(&self) -> Result<(), CombatInterventionError> {
        registry(&["combat.formula.v1.fixed_chance"], &self.formula_id)?;
        if self.formula_parameters.len() != 1
            || !self.formula_parameters.contains_key("chance_percent")
        {
            return Err(CombatInterventionError::InvalidField(
                "chance_percent".into(),
            ));
        }
        let n = self.formula_parameters["chance_percent"]
            .as_i64()
            .ok_or_else(|| CombatInterventionError::InvalidField("chance_percent".into()))?;
        if !(0..=100).contains(&n) {
            return Err(CombatInterventionError::InvalidField(
                "chance_percent".into(),
            ));
        }
        registry(EXECUTOR, &self.executor_selector_id)?;
        registry(TARGET, &self.target_selector_id)?;
        self.success.validate()?;
        self.failure.validate()?;
        Ok(())
    }
}
impl CombatEffectBranch {
    pub fn validate(&self) -> Result<(), CombatInterventionError> {
        unique(&self.effect_ids)?;
        for id in &self.effect_ids {
            nonempty(id, "effect_id")?
        }
        for a in &self.outcome_actions {
            a.validate()?
        }
        Ok(())
    }
}
impl CombatOutcomeAction {
    pub fn validate(&self) -> Result<(), CombatInterventionError> {
        match self {
            Self::SetFlag { flag_id } => nonempty(flag_id, "flag_id"),
            Self::CreateLootEntitlement {
                item_id,
                source_selector_id,
                ..
            } => {
                nonempty(item_id, "item_id")?;
                registry(TARGET, source_selector_id)
            }
            Self::GrantItem { item_id } => nonempty(item_id, "item_id"),
        }
    }
}
fn nonempty(v: &str, f: &'static str) -> Result<(), CombatInterventionError> {
    if v.trim().is_empty() {
        Err(CombatInterventionError::EmptyField(f))
    } else {
        Ok(())
    }
}
fn unique(ids: &[String]) -> Result<(), CombatInterventionError> {
    let mut s = BTreeSet::new();
    for id in ids {
        if !s.insert(id) {
            return Err(CombatInterventionError::DuplicateId(id.clone()));
        }
    }
    Ok(())
}
fn registry(ids: &[&str], id: &str) -> Result<(), CombatInterventionError> {
    if ["self", "target", "observer", "opponent", "any"].contains(&id) {
        Err(CombatInterventionError::LegacyAlias(id.into()))
    } else if ids.contains(&id) {
        Ok(())
    } else {
        Err(CombatInterventionError::UnknownCanonicalId(id.into()))
    }
}
