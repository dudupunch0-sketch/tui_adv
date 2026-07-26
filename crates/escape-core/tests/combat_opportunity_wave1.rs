use escape_core::*;
use std::collections::BTreeSet;

fn catalog() -> CombatOpportunityCatalog {
    let definition = |id: &str| CombatEffectDefinition {
        id: id.into(),
        source: "test".into(),
        category: CombatEffectCategory::State,
        target_selector: "player".into(),
        parameters: Default::default(),
        conditions: vec![],
        phase: EffectPhase::DuringCombat,
        lifetime: EffectLifetime::Instant,
        stacking: EffectStacking::Unique,
        stacking_group: id.into(),
        stacking_cap: None,
        priority: 0,
        visibility: EffectVisibility::Public,
        tags: vec![],
    };
    CombatOpportunityCatalog {
        opportunities: vec![CombatOpportunityDefinition {
            id: "opening".into(),
            trigger_tags: vec!["clinch".into()],
            required_condition_ids: vec![],
            thresholds: CombatDetectionThresholds {
                detected: 1,
                interpreted: 3,
                insightful: 5,
            },
            expiry_tick: Some(10),
            dedupe: true,
            scripted: true,
            defeat_risk: true,
            battlefield_impact: true,
            unique_response: true,
            tactical_priority: 2,
            free_alert_id: Some("alert".into()),
        }],
        responses: vec![CombatResponseDefinition {
            id: "break_grip".into(),
            opportunity_id: "opening".into(),
            minimum_detection: CombatDetectionLevel::Detected,
            required_capability_ids: vec!["grip".into()],
            required_condition_ids: vec![],
            executor_selector: "observer".into(),
            target_selector: "opponent".into(),
            cost_tags: vec![],
            resolution_kind: "choice".into(),
            success_effect_ids: vec!["break_success".into()],
            failure_effect_ids: vec!["break_failure".into()],
            unique: true,
            tactical_priority: 1,
        }],
        effect_catalog: CombatEffectCatalog {
            effects: vec![definition("break_success"), definition("break_failure")],
        },
    }
}
fn context(budget: CombatInterventionBudget) -> CombatOpportunityContext {
    CombatOpportunityContext {
        current_tick: 1,
        active_tag_ids: ["clinch".into()].into_iter().collect(),
        active_condition_ids: BTreeSet::new(),
        presented_instance_ids: BTreeSet::new(),
        observers: vec![CombatObserver {
            id: "p1".into(),
            detection_score: 5,
            capability_ids: vec!["grip".into()],
            can_observe: true,
            can_act: true,
        }],
        budget,
        manifest_fingerprint: "fp".into(),
    }
}
#[test]
fn thresholds_and_undetected_ladder() {
    let t = CombatDetectionThresholds {
        detected: 1,
        interpreted: 3,
        insightful: 5,
    };
    assert_eq!(
        t.level_for_score(0).unwrap(),
        CombatDetectionLevel::Undetected
    );
    assert_eq!(
        t.level_for_score(5).unwrap(),
        CombatDetectionLevel::Insightful
    );
    assert!(CombatDetectionThresholds {
        detected: 3,
        interpreted: 2,
        insightful: 5
    }
    .validate()
    .is_err());
}
#[test]
fn evaluation_presents_and_consumes_budget_with_options() {
    let result = catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into(),
            }],
            &context(CombatInterventionBudget {
                maximum: 1,
                consumed: 0,
            }),
        )
        .unwrap();
    assert_eq!(result.budget.consumed, 1);
    let candidate = result.candidate.unwrap();
    assert!(candidate
        .options
        .iter()
        .any(|option| option.id == "no_intervention"));
    assert!(candidate
        .options
        .iter()
        .any(|option| option.id == "break_grip"));
}
#[test]
fn zero_budget_returns_alert_without_candidate() {
    let result = catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into(),
            }],
            &context(CombatInterventionBudget {
                maximum: 0,
                consumed: 0,
            }),
        )
        .unwrap();
    assert!(result.candidate.is_none());
    assert_eq!(result.free_alert_ids, vec!["alert"]);
}

#[test]
fn no_op_only_response_is_not_presented_and_observer_may_not_act() {
    let mut c = context(CombatInterventionBudget {
        maximum: 1,
        consumed: 0,
    });
    c.observers[0].capability_ids.clear();
    assert!(catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into()
            }],
            &c
        )
        .unwrap()
        .candidate
        .is_none());
    c.observers[0].capability_ids.push("grip".into());
    c.observers[0].can_act = false;
    let result = catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into(),
            }],
            &c,
        )
        .unwrap();
    assert!(result.candidate.is_none());
    assert_eq!(result.free_alert_ids, vec!["alert"]);
}
#[test]
fn dedupe_and_expiry_filter() {
    let mut c = context(CombatInterventionBudget {
        maximum: 3,
        consumed: 0,
    });
    c.presented_instance_ids.insert("i1".into());
    assert!(catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into()
            }],
            &c
        )
        .unwrap()
        .candidate
        .is_none());
    c.presented_instance_ids.clear();
    c.current_tick = 11;
    assert!(catalog()
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into()
            }],
            &c
        )
        .unwrap()
        .candidate
        .is_none());
}
#[test]
fn validation_rejects_unknown_effect_and_missing_bundle() {
    let mut c = catalog();
    c.effect_catalog.effects.clear();
    assert!(matches!(
        c.validate(),
        Err(CombatOpportunityError::UnknownEffect(_))
    ));
    let mut c = catalog();
    c.responses[0].failure_effect_ids.clear();
    assert!(matches!(
        c.validate(),
        Err(CombatOpportunityError::MissingEffectBundle(_))
    ));
}
#[test]
fn canonical_catalog_ignores_definition_input_order() {
    let a = catalog();
    let mut b = catalog();
    b.opportunities.reverse();
    b.responses.reverse();
    assert_eq!(a.canonical_json().unwrap(), b.canonical_json().unwrap());
    b.effect_catalog.effects.reverse();
    assert_eq!(a.canonical_json().unwrap(), b.canonical_json().unwrap());
}

#[test]
fn evaluation_fingerprint_is_input_order_invariant() {
    let instances = vec![CombatOpportunityInstance {
        id: "i1".into(),
        definition_id: "opening".into(),
    }];
    let first = catalog()
        .evaluate(
            &instances,
            &context(CombatInterventionBudget {
                maximum: 1,
                consumed: 0,
            }),
        )
        .unwrap();
    let mut reordered_context = context(CombatInterventionBudget {
        maximum: 1,
        consumed: 0,
    });
    reordered_context.observers.reverse();
    let second = catalog().evaluate(&instances, &reordered_context).unwrap();
    assert_eq!(
        first.canonical_json().unwrap(),
        second.canonical_json().unwrap()
    );
    assert_eq!(first.fingerprint(), second.fingerprint());
}

#[test]
fn observer_and_executor_can_be_different() {
    let mut c = context(CombatInterventionBudget {
        maximum: 1,
        consumed: 0,
    });
    c.observers.push(CombatObserver {
        id: "p2".into(),
        detection_score: 1,
        capability_ids: vec!["grip".into()],
        can_observe: true,
        can_act: true,
    });
    c.observers[0].can_act = false;
    let mut cat = catalog();
    cat.responses[0].executor_selector = "any".into();
    let candidate = cat
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into(),
            }],
            &c,
        )
        .unwrap()
        .candidate
        .unwrap();
    assert_eq!(candidate.observer_id, "p1");
    assert_eq!(
        candidate
            .options
            .iter()
            .find(|option| option.id == "break_grip")
            .and_then(|option| option.executor_id.as_deref()),
        Some("p2")
    );
}

#[test]
fn budget_three_consumes_last_slot_and_rejects_fourth() {
    let instance = [CombatOpportunityInstance {
        id: "i1".into(),
        definition_id: "opening".into(),
    }];
    let last_slot = catalog()
        .evaluate(
            &instance,
            &context(CombatInterventionBudget {
                maximum: 3,
                consumed: 2,
            }),
        )
        .unwrap();
    assert!(last_slot.candidate.is_some());
    assert_eq!(last_slot.budget.consumed, 3);

    let exhausted = catalog()
        .evaluate(
            &instance,
            &context(CombatInterventionBudget {
                maximum: 3,
                consumed: 3,
            }),
        )
        .unwrap();
    assert!(exhausted.candidate.is_none());
    assert_eq!(exhausted.budget.remaining().unwrap(), 0);

    let mut budget = CombatInterventionBudget {
        maximum: 1,
        consumed: 1,
    };
    assert!(matches!(
        budget.present(),
        Err(CombatOpportunityError::BudgetExhausted)
    ));
}

#[test]
fn candidate_tie_break_prefers_scripted_then_stable_id() {
    let mut scripted_first = catalog();
    let mut other = scripted_first.opportunities[0].clone();
    other.id = "other".into();
    other.scripted = false;
    scripted_first.opportunities.push(other);
    let mut other_response = scripted_first.responses[0].clone();
    other_response.id = "other_response".into();
    other_response.opportunity_id = "other".into();
    scripted_first.responses.push(other_response);
    let result = scripted_first
        .evaluate(
            &[
                CombatOpportunityInstance {
                    id: "i2".into(),
                    definition_id: "other".into(),
                },
                CombatOpportunityInstance {
                    id: "i1".into(),
                    definition_id: "opening".into(),
                },
            ],
            &context(CombatInterventionBudget {
                maximum: 1,
                consumed: 0,
            }),
        )
        .unwrap();
    assert_eq!(result.candidate.unwrap().opportunity_id, "opening");
}

#[test]
fn response_options_are_capped_without_dropping_no_intervention() {
    let mut c = catalog();
    for index in 0..5 {
        let mut response = c.responses[0].clone();
        response.id = format!("extra_{index}");
        response.tactical_priority = 0;
        c.responses.push(response);
    }
    let candidate = c
        .evaluate(
            &[CombatOpportunityInstance {
                id: "i1".into(),
                definition_id: "opening".into(),
            }],
            &context(CombatInterventionBudget {
                maximum: 1,
                consumed: 0,
            }),
        )
        .unwrap()
        .candidate
        .unwrap();
    assert_eq!(candidate.options.len(), 4);
    assert!(candidate
        .options
        .iter()
        .any(|option| option.id == "no_intervention"));
}
