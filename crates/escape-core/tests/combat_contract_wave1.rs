use escape_core::{
    CombatEffectRef, CombatManifest, CombatRngNamespace, CombatSimulationVersion,
    SuppressedCombatEffect,
};
fn manifest(reverse: bool) -> CombatManifest {
    let mut combatant_ids = vec!["player".into(), "wuxia_guard".into()];
    let mut environment_ids = vec!["market_street".into(), "narrow_lane".into()];
    if reverse {
        combatant_ids.reverse();
        environment_ids.reverse();
    }
    CombatManifest {
        simulation_version: CombatSimulationVersion::new("wave1").unwrap(),
        actual_seed: 42,
        world_state_fingerprint: "state:abc".into(),
        applied_effects: vec![CombatEffectRef {
            id: "fog".into(),
            reason: "visible_signal".into(),
        }],
        suppressed_effects: vec![SuppressedCombatEffect {
            id: "ambush_bonus".into(),
            reason: "not_visible".into(),
        }],
        combatant_ids,
        placement_ids: vec!["lane_entry".into()],
        environment_ids,
        team_ids: vec!["team:solo".into()],
        rule_ids: vec!["opening_exchange".into()],
        public_info_ids: vec!["heard_footsteps".into()],
    }
}
#[test]
fn canonical_fingerprint_ignores_input_order() {
    assert_eq!(manifest(false).fingerprint(), manifest(true).fingerprint());
    assert_eq!(
        manifest(false).canonical_json(),
        manifest(true).canonical_json()
    );
}

#[test]
fn canonical_fingerprint_ignores_same_effect_id_reason_order() {
    let mut first = manifest(false);
    first.applied_effects = vec![
        CombatEffectRef {
            id: "fog".into(),
            reason: "z_reason".into(),
        },
        CombatEffectRef {
            id: "fog".into(),
            reason: "a_reason".into(),
        },
    ];
    let mut second = first.clone();
    second.applied_effects.reverse();
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.canonical_json(), second.canonical_json());
}
#[test]
fn namespaces_produce_separate_derived_seeds() {
    let m = manifest(false);
    assert_ne!(
        m.derived_seed(CombatRngNamespace::ActualCombat),
        m.derived_seed(CombatRngNamespace::ForecastEnsemble)
    );
}
#[test]
fn invalid_required_values_fail_explicitly() {
    assert!(CombatSimulationVersion::new("  ").is_err());
    let mut m = manifest(false);
    m.world_state_fingerprint.clear();
    assert!(m.validate().is_err());
    m.world_state_fingerprint = "state:abc".into();
    m.combatant_ids = vec![" ".into()];
    assert!(m.validate().is_err());
}
#[test]
fn serde_round_trip_preserves_contract_identity() {
    let m = manifest(false);
    let json = serde_json::to_string(&m).unwrap();
    let decoded: CombatManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(m, decoded);
    assert_eq!(m.fingerprint(), decoded.fingerprint());
}
