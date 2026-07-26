use crate::CombatEffectCatalog;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatDetectionLevel {
    Undetected,
    Detected,
    Interpreted,
    Insightful,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatDetectionThresholds {
    pub detected: i32,
    pub interpreted: i32,
    pub insightful: i32,
}
impl CombatDetectionThresholds {
    pub fn validate(&self) -> Result<(), CombatOpportunityError> {
        if self.detected > self.interpreted || self.interpreted > self.insightful {
            Err(CombatOpportunityError::InvalidThresholds)
        } else {
            Ok(())
        }
    }
    pub fn level_for_score(
        &self,
        score: i32,
    ) -> Result<CombatDetectionLevel, CombatOpportunityError> {
        self.validate()?;
        Ok(if score < self.detected {
            CombatDetectionLevel::Undetected
        } else if score < self.interpreted {
            CombatDetectionLevel::Detected
        } else if score < self.insightful {
            CombatDetectionLevel::Interpreted
        } else {
            CombatDetectionLevel::Insightful
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityDefinition {
    pub id: String,
    pub trigger_tags: Vec<String>,
    pub required_condition_ids: Vec<String>,
    pub thresholds: CombatDetectionThresholds,
    pub expiry_tick: Option<u32>,
    pub dedupe: bool,
    pub scripted: bool,
    pub defeat_risk: bool,
    pub battlefield_impact: bool,
    pub unique_response: bool,
    pub tactical_priority: i32,
    pub free_alert_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityInstance {
    pub id: String,
    pub definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatObserver {
    pub id: String,
    pub detection_score: i32,
    pub capability_ids: Vec<String>,
    pub can_observe: bool,
    pub can_act: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResponseDefinition {
    pub id: String,
    pub opportunity_id: String,
    pub minimum_detection: CombatDetectionLevel,
    pub required_capability_ids: Vec<String>,
    pub required_condition_ids: Vec<String>,
    pub executor_selector: String,
    pub target_selector: String,
    pub cost_tags: Vec<String>,
    pub resolution_kind: String,
    pub success_effect_ids: Vec<String>,
    pub failure_effect_ids: Vec<String>,
    pub unique: bool,
    pub tactical_priority: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityCatalog {
    pub opportunities: Vec<CombatOpportunityDefinition>,
    pub responses: Vec<CombatResponseDefinition>,
    pub effect_catalog: CombatEffectCatalog,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatInterventionBudget {
    pub maximum: u8,
    pub consumed: u8,
}
impl CombatInterventionBudget {
    pub fn validate(&self) -> Result<(), CombatOpportunityError> {
        if self.maximum > 3 || self.consumed > self.maximum {
            Err(CombatOpportunityError::InvalidBudget)
        } else {
            Ok(())
        }
    }
    pub fn remaining(&self) -> Result<u8, CombatOpportunityError> {
        self.validate()?;
        Ok(self.maximum - self.consumed)
    }

    pub fn present(&mut self) -> Result<(), CombatOpportunityError> {
        self.validate()?;
        if self.consumed >= self.maximum {
            return Err(CombatOpportunityError::BudgetExhausted);
        }
        self.consumed += 1;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityContext {
    pub current_tick: u32,
    pub active_tag_ids: BTreeSet<String>,
    pub active_condition_ids: BTreeSet<String>,
    pub presented_instance_ids: BTreeSet<String>,
    pub observers: Vec<CombatObserver>,
    pub budget: CombatInterventionBudget,
    pub manifest_fingerprint: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatResponseOption {
    pub id: String,
    pub executor_id: Option<String>,
    pub success_effect_ids: Vec<String>,
    pub failure_effect_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityCandidate {
    pub instance_id: String,
    pub opportunity_id: String,
    pub observer_id: String,
    pub detection: CombatDetectionLevel,
    pub options: Vec<CombatResponseOption>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatOpportunityEvaluation {
    pub candidate: Option<CombatOpportunityCandidate>,
    pub free_alert_ids: Vec<String>,
    pub budget: CombatInterventionBudget,
}

impl CombatOpportunityEvaluation {
    pub fn canonical_json(&self) -> Result<String, CombatOpportunityError> {
        let mut value = self.clone();
        value.free_alert_ids.sort();
        if let Some(candidate) = &mut value.candidate {
            candidate.options.sort_by(|a, b| a.id.cmp(&b.id));
            for option in &mut candidate.options {
                option.success_effect_ids.sort();
                option.failure_effect_ids.sort();
            }
        }
        serde_json::to_string(&value)
            .map_err(|error| CombatOpportunityError::Serialization(error.to_string()))
    }

    pub fn fingerprint(&self) -> Result<String, CombatOpportunityError> {
        Ok(format!(
            "{:016x}",
            fnv1a_64(self.canonical_json()?.as_bytes())
        ))
    }
}

impl CombatOpportunityCatalog {
    pub fn validate(&self) -> Result<(), CombatOpportunityError> {
        let mut ids = BTreeSet::new();
        for opportunity in &self.opportunities {
            ensure_id(&opportunity.id)?;
            if !ids.insert(opportunity.id.clone()) {
                return Err(CombatOpportunityError::DuplicateId(opportunity.id.clone()));
            }
            opportunity.thresholds.validate()?;
            for id in opportunity
                .trigger_tags
                .iter()
                .chain(&opportunity.required_condition_ids)
            {
                ensure_id(id)?;
            }
            if let Some(id) = &opportunity.free_alert_id {
                ensure_id(id)?;
            }
        }
        let opportunity_ids: BTreeSet<_> =
            self.opportunities.iter().map(|o| o.id.as_str()).collect();
        self.effect_catalog
            .validate()
            .map_err(|error| CombatOpportunityError::InvalidEffectCatalog(format!("{error:?}")))?;
        let effect_ids: BTreeSet<_> = self
            .effect_catalog
            .effects
            .iter()
            .map(|effect| effect.id.as_str())
            .collect();
        ids.clear();
        for response in &self.responses {
            ensure_id(&response.id)?;
            if response.id == "no_intervention" {
                return Err(CombatOpportunityError::ReservedResponseId);
            }
            if !ids.insert(response.id.clone()) {
                return Err(CombatOpportunityError::DuplicateId(response.id.clone()));
            }
            if !opportunity_ids.contains(response.opportunity_id.as_str()) {
                return Err(CombatOpportunityError::UnknownOpportunity(
                    response.opportunity_id.clone(),
                ));
            }
            ensure_id(&response.executor_selector)?;
            ensure_id(&response.target_selector)?;
            ensure_id(&response.resolution_kind)?;
            if response.success_effect_ids.is_empty() || response.failure_effect_ids.is_empty() {
                return Err(CombatOpportunityError::MissingEffectBundle(
                    response.id.clone(),
                ));
            }
            for id in response
                .required_capability_ids
                .iter()
                .chain(&response.required_condition_ids)
                .chain(&response.cost_tags)
                .chain(&response.success_effect_ids)
                .chain(&response.failure_effect_ids)
            {
                ensure_id(id)?;
            }
            for id in response
                .success_effect_ids
                .iter()
                .chain(&response.failure_effect_ids)
            {
                if !effect_ids.contains(id.as_str()) {
                    return Err(CombatOpportunityError::UnknownEffect(id.clone()));
                }
            }
        }
        Ok(())
    }
    pub fn canonical_json(&self) -> Result<String, CombatOpportunityError> {
        self.validate()?;
        let mut value = self.clone();
        value.opportunities.sort_by(|a, b| a.id.cmp(&b.id));
        value.responses.sort_by(|a, b| a.id.cmp(&b.id));
        value.effect_catalog.effects = value
            .effect_catalog
            .canonical()
            .map_err(|error| CombatOpportunityError::InvalidEffectCatalog(format!("{error:?}")))?;
        serde_json::to_string(&value)
            .map_err(|e| CombatOpportunityError::Serialization(e.to_string()))
    }
    pub fn evaluate(
        &self,
        instances: &[CombatOpportunityInstance],
        context: &CombatOpportunityContext,
    ) -> Result<CombatOpportunityEvaluation, CombatOpportunityError> {
        self.validate()?;
        context.budget.validate()?;
        ensure_id(&context.manifest_fingerprint)?;
        let definitions: BTreeMap<_, _> = self
            .opportunities
            .iter()
            .map(|o| (o.id.as_str(), o))
            .collect();
        let mut candidates = Vec::new();
        let mut alerts = BTreeSet::new();
        let mut observer_ids = BTreeSet::new();
        for observer in &context.observers {
            ensure_id(&observer.id)?;
            if !observer_ids.insert(observer.id.clone()) {
                return Err(CombatOpportunityError::DuplicateId(observer.id.clone()));
            }
            for capability_id in &observer.capability_ids {
                ensure_id(capability_id)?;
            }
        }
        let mut instance_ids = BTreeSet::new();
        for instance in instances {
            ensure_id(&instance.id)?;
            if !instance_ids.insert(instance.id.clone()) {
                return Err(CombatOpportunityError::DuplicateId(instance.id.clone()));
            }
            let Some(definition) = definitions.get(instance.definition_id.as_str()) else {
                return Err(CombatOpportunityError::UnknownOpportunity(
                    instance.definition_id.clone(),
                ));
            };
            if context.presented_instance_ids.contains(&instance.id)
                || definition.dedupe && context.presented_instance_ids.contains(&instance.id)
            {
                continue;
            }
            if definition
                .expiry_tick
                .is_some_and(|tick| context.current_tick > tick)
            {
                continue;
            }
            if !definition
                .trigger_tags
                .iter()
                .all(|tag| context.active_tag_ids.contains(tag))
                || !definition
                    .required_condition_ids
                    .iter()
                    .all(|id| context.active_condition_ids.contains(id))
            {
                continue;
            }
            let observer = context
                .observers
                .iter()
                .filter(|o| o.can_observe)
                .filter_map(|o| {
                    definition
                        .thresholds
                        .level_for_score(o.detection_score)
                        .ok()
                        .map(|level| (o, level))
                })
                .filter(|(_, level)| *level != CombatDetectionLevel::Undetected)
                .max_by(|(a, la), (b, lb)| {
                    la.cmp(lb)
                        .then(a.detection_score.cmp(&b.detection_score))
                        .then_with(|| b.id.cmp(&a.id))
                });
            let Some((observer, detection)) = observer else {
                if let Some(alert) = &definition.free_alert_id {
                    alerts.insert(alert.clone());
                }
                continue;
            };
            let mut options = vec![CombatResponseOption {
                id: "no_intervention".into(),
                executor_id: None,
                success_effect_ids: vec![],
                failure_effect_ids: vec![],
            }];
            let mut actionable = self
                .responses
                .iter()
                .filter(|response| {
                    response.opportunity_id == definition.id
                        && detection >= response.minimum_detection
                        && response
                            .required_condition_ids
                            .iter()
                            .all(|id| context.active_condition_ids.contains(id))
                })
                .filter_map(|response| {
                    let executor = context
                        .observers
                        .iter()
                        .filter(|candidate| candidate.can_act)
                        .filter(|candidate| {
                            response_executor_matches(
                                &response.executor_selector,
                                observer.id.as_str(),
                                candidate.id.as_str(),
                            )
                        })
                        .find(|candidate| {
                            response
                                .required_capability_ids
                                .iter()
                                .all(|id| candidate.capability_ids.contains(id))
                        })?;
                    Some((response, executor))
                })
                .collect::<Vec<_>>();
            actionable.sort_by(|(a, _), (b, _)| {
                b.tactical_priority
                    .cmp(&a.tactical_priority)
                    .then(b.unique.cmp(&a.unique))
                    .then(a.id.cmp(&b.id))
            });
            for (response, executor) in actionable.into_iter().take(3) {
                options.push(CombatResponseOption {
                    id: response.id.clone(),
                    executor_id: Some(executor.id.clone()),
                    success_effect_ids: response.success_effect_ids.clone(),
                    failure_effect_ids: response.failure_effect_ids.clone(),
                });
            }
            options.sort_by(|a, b| a.id.cmp(&b.id));
            if options.len() > 4 {
                options.truncate(4);
            }
            if options.len() == 1 {
                if let Some(alert) = &definition.free_alert_id {
                    alerts.insert(alert.clone());
                }
                continue;
            }
            candidates.push((definition, instance, observer, detection, options));
        }
        let mut budget = context.budget.clone();
        if budget.remaining()? == 0 {
            for (definition, _, _, _, _) in &candidates {
                if let Some(alert) = &definition.free_alert_id {
                    alerts.insert(alert.clone());
                }
            }
            return Ok(CombatOpportunityEvaluation {
                candidate: None,
                free_alert_ids: alerts.into_iter().collect(),
                budget,
            });
        }
        let selected = candidates
            .into_iter()
            .max_by(|(a, ai, _, _, _), (b, bi, _, _, _)| {
                rank(a, ai)
                    .cmp(&rank(b, bi))
                    .then_with(|| bi.id.cmp(&ai.id))
            });
        let candidate = if let Some((definition, instance, observer, detection, options)) = selected
        {
            budget.present()?;
            Some(CombatOpportunityCandidate {
                instance_id: instance.id.clone(),
                opportunity_id: definition.id.clone(),
                observer_id: observer.id.clone(),
                detection,
                options,
            })
        } else {
            None
        };
        Ok(CombatOpportunityEvaluation {
            candidate,
            free_alert_ids: alerts.into_iter().collect(),
            budget,
        })
    }
}

fn rank(
    definition: &CombatOpportunityDefinition,
    _instance: &CombatOpportunityInstance,
) -> (bool, bool, bool, bool, i32, std::cmp::Reverse<Option<u32>>) {
    (
        definition.scripted,
        definition.defeat_risk,
        definition.battlefield_impact,
        definition.unique_response,
        definition.tactical_priority,
        std::cmp::Reverse(definition.expiry_tick),
    )
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatOpportunityError {
    EmptyId,
    DuplicateId(String),
    InvalidThresholds,
    InvalidBudget,
    BudgetExhausted,
    UnknownOpportunity(String),
    UnknownEffect(String),
    MissingEffectBundle(String),
    InvalidEffectCatalog(String),
    ReservedResponseId,
    Serialization(String),
}
impl fmt::Display for CombatOpportunityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatOpportunityError {}
fn ensure_id(id: &str) -> Result<(), CombatOpportunityError> {
    if id.trim().is_empty() {
        Err(CombatOpportunityError::EmptyId)
    } else {
        Ok(())
    }
}

fn response_executor_matches(selector: &str, observer_id: &str, executor_id: &str) -> bool {
    selector == "any"
        || (selector == "observer" && observer_id == executor_id)
        || selector == executor_id
}

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
