use escape_core::*;
use serde_json::json;
use std::collections::BTreeMap;
fn strategy() -> CombatStrategyModifier {
    CombatStrategyModifier {
        scope: StrategyScope::Role {
            role_id: "frontline".into(),
        },
        duration: StrategyDuration::NextSegment,
        operations: vec![StrategyOperation::SetTargetingRule {
            rule_id: "combat.strategy.targeting.v1.attackers_of".into(),
        }],
    }
}
fn special() -> CombatSpecialEffect {
    CombatSpecialEffect {
        formula_id: "combat.formula.v1.fixed_chance".into(),
        formula_parameters: BTreeMap::from([("chance_percent".into(), json!(50))]),
        executor_selector_id: "combat.selector.executor.v1.observer".into(),
        target_selector_id: "combat.selector.target.v1.selected_target".into(),
        success: CombatEffectBranch {
            effect_ids: vec!["heal".into()],
            outcome_actions: vec![CombatOutcomeAction::SetFlag {
                flag_id: "revived".into(),
            }],
        },
        failure: CombatEffectBranch {
            effect_ids: vec![],
            outcome_actions: vec![CombatOutcomeAction::CreateLootEntitlement {
                item_id: "token".into(),
                source_selector_id: "combat.selector.target.v1.executor_self".into(),
                claim_policy: CombatClaimPolicy::DefaultTerminalPolicy,
            }],
        },
    }
}
#[test]
fn payload_kinds_roundtrip() {
    for p in [
        CombatInterventionPayload {
            strategy_modifier: Some(strategy()),
            special_effect: None,
        },
        CombatInterventionPayload {
            strategy_modifier: None,
            special_effect: Some(special()),
        },
        CombatInterventionPayload {
            strategy_modifier: Some(strategy()),
            special_effect: Some(special()),
        },
    ] {
        p.validate().unwrap();
        assert_eq!(
            serde_json::from_value::<CombatInterventionPayload>(serde_json::to_value(&p).unwrap())
                .unwrap(),
            p
        )
    }
}
#[test]
fn empty_and_unknown_fail() {
    assert!(matches!(
        (CombatInterventionPayload {
            strategy_modifier: None,
            special_effect: None
        })
        .validate(),
        Err(CombatInterventionError::EmptyPayload)
    ));
    assert!(serde_json::from_value::<CombatInterventionPayload>(
        json!({"strategy_modifier":null,"extra":true})
    )
    .is_err());
    assert!(
        serde_json::from_value::<StrategyScope>(json!({"kind":"all_allies","extra":true})).is_err()
    );
    assert!(serde_json::from_value::<CombatOutcomeAction>(
        json!({"kind":"grant_item","item_id":"x","extra":true})
    )
    .is_err())
}
#[test]
fn registry_and_aliases() {
    for x in [
        "combat.selector.executor.v1.observer",
        "combat.selector.executor.v1.any_capable",
    ] {
        let mut e = special();
        e.executor_selector_id = x.into();
        e.validate().unwrap()
    }
    for x in [
        "combat.selector.target.v1.executor_self",
        "combat.selector.target.v1.selected_target",
        "combat.selector.target.v1.nearest_active_enemy",
        "combat.selector.target.v1.lowest_health_active_ally",
        "combat.selector.target.v1.surrounded_active_ally",
        "combat.selector.target.v1.all_active_allies",
    ] {
        let mut e = special();
        e.target_selector_id = x.into();
        e.validate().unwrap()
    }
    let mut e = special();
    e.target_selector_id = "target".into();
    assert!(matches!(e.validate(),Err(CombatInterventionError::LegacyAlias(x))if x=="target"));
    let mut e = special();
    e.formula_id = "unknown".into();
    assert!(matches!(
        e.validate(),
        Err(CombatInterventionError::UnknownCanonicalId(_))
    ))
}
#[test]
fn fixed_chance_and_typed_validation() {
    for p in [
        BTreeMap::new(),
        BTreeMap::from([
            ("chance_percent".into(), json!(50)),
            ("extra".into(), json!(1)),
        ]),
        BTreeMap::from([("chance_percent".into(), json!("50"))]),
        BTreeMap::from([("chance_percent".into(), json!(-1))]),
        BTreeMap::from([("chance_percent".into(), json!(101))]),
        BTreeMap::from([("chance_percent".into(), json!(50.0))]),
    ] {
        let mut e = special();
        e.formula_parameters = p;
        assert!(e.validate().is_err())
    }
    let mut e = strategy();
    e.operations.clear();
    assert!(matches!(
        e.validate(),
        Err(CombatInterventionError::EmptyOperations)
    ));
    let mut e = special();
    e.success.effect_ids = vec!["x".into(), "x".into()];
    assert!(matches!(
        e.validate(),
        Err(CombatInterventionError::DuplicateId(_))
    ));
    let mut e = special();
    e.failure.outcome_actions = vec![CombatOutcomeAction::GrantItem { item_id: "".into() }];
    assert!(matches!(
        e.validate(),
        Err(CombatInterventionError::EmptyField("item_id"))
    ))
}
#[test]
fn strict_nested_and_empty_id_validation() {
    let mut effect = special();
    let raw = serde_json::to_value(&effect).unwrap();
    let mut obj = raw.as_object().unwrap().clone();
    obj.insert("extra".into(), json!(true));
    assert!(serde_json::from_value::<CombatSpecialEffect>(serde_json::Value::Object(obj)).is_err());

    assert!(serde_json::from_value::<StrategyOperation>(
        json!({"kind":"set_target_policy","policy_id":"x","extra":true})
    )
    .is_err());
    assert!(serde_json::from_value::<CombatClaimPolicy>(json!("sometimes")).is_err());

    let mut s = strategy();
    s.scope = StrategyScope::Combatants {
        combatant_selector_ids: vec![],
    };
    assert!(s.validate().is_err());
    s.scope = StrategyScope::Combatants {
        combatant_selector_ids: vec![
            "combat.selector.target.v1.executor_self".into(),
            "combat.selector.target.v1.executor_self".into(),
        ],
    };
    assert!(s.validate().is_err());

    let mut s = strategy();
    s.operations = vec![StrategyOperation::SetTargetingRule {
        rule_id: "unknown".into(),
    }];
    assert!(s.validate().is_err());

    let mut e = special();
    e.success.effect_ids = vec!["".into()];
    assert!(e.validate().is_err());
    e.success.effect_ids.clear();
    e.success.outcome_actions = vec![CombatOutcomeAction::SetFlag { flag_id: "".into() }];
    assert!(e.validate().is_err());
}

#[test]
fn integer_boundaries_are_accepted_but_float_is_rejected() {
    for chance in [0, 50, 100] {
        let mut e = special();
        e.formula_parameters = BTreeMap::from([("chance_percent".into(), json!(chance))]);
        e.validate().unwrap();
    }
    let mut e = special();
    e.formula_parameters = BTreeMap::from([("chance_percent".into(), json!(50.0))]);
    assert!(e.validate().is_err());
}

#[test]
fn all_legacy_aliases_are_rejected() {
    for alias in ["self", "target", "observer", "opponent", "any"] {
        let mut e = special();
        e.executor_selector_id = alias.into();
        assert!(
            matches!(e.validate(), Err(CombatInterventionError::LegacyAlias(id)) if id == alias)
        );
        let mut e = special();
        e.target_selector_id = alias.into();
        assert!(
            matches!(e.validate(), Err(CombatInterventionError::LegacyAlias(id)) if id == alias)
        );
    }
    let mut e = special();
    e.formula_id = "self".into();
    assert!(matches!(
        e.validate(),
        Err(CombatInterventionError::LegacyAlias(id)) if id == "self"
    ));
}
