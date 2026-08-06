use escape_core::{
    CombatContractError, CombatEffectRef, CombatManifest, CombatRngNamespace, CombatSimulation,
    CombatSimulationConfig, CombatSimulationError, CombatSimulationInput, CombatSimulationVersion,
    CombatState, SuppressedCombatEffect, CURRENT_SIMULATION_VERSION,
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

// ---------------------------------------------------------------------
// T0: simulation_version enforcement.
// ---------------------------------------------------------------------

fn minimal_state() -> CombatState {
    CombatState {
        battle_id: "b".into(),
        combatants: vec![],
        persistent_statuses: vec![],
        active_effects: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
        manifest_fingerprint: "fp".into(),
    }
}

/// A minimal, otherwise-valid `CombatSimulationInput` with `version` as its
/// `simulation_version` -- everything else is fixed so only the version
/// varies between calls.
fn sim_input(version: &str) -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: CombatManifest {
            simulation_version: CombatSimulationVersion::new(version).unwrap(),
            actual_seed: 1,
            world_state_fingerprint: "w".into(),
            applied_effects: vec![],
            suppressed_effects: vec![],
            combatant_ids: vec![],
            placement_ids: vec![],
            environment_ids: vec![],
            team_ids: vec![],
            rule_ids: vec![],
            public_info_ids: vec![],
        },
        state: minimal_state(),
        seed: 1,
        config: CombatSimulationConfig {
            tick_millis: 100,
            max_ticks: 3,
        },
        participants: vec![],
        roles: vec![],
        policies: vec![],
    }
}

#[test]
fn current_simulation_version_is_accepted() {
    assert!(CombatSimulation::new(sim_input(CURRENT_SIMULATION_VERSION)).is_ok());
}

#[test]
fn unsupported_simulation_version_is_rejected_at_simulation_entry() {
    let error = CombatSimulation::new(sim_input("v9")).unwrap_err();
    match error {
        CombatSimulationError::UnsupportedSimulationVersion(received) => {
            assert_eq!(received, "v9");
        }
        other => panic!("expected UnsupportedSimulationVersion, got {other:?}"),
    }
}

#[test]
fn error_message_names_both_the_received_and_the_expected_version() {
    let error = CombatContractError::UnsupportedSimulationVersion("v9".to_string());
    let message = error.to_string();
    assert!(
        message.contains("v9"),
        "message must name the received version: {message}"
    );
    assert!(
        message.contains(CURRENT_SIMULATION_VERSION),
        "message must name the expected version: {message}"
    );
}

/// Invariant 4: a record carrying any old (or invented) version string must
/// keep deserializing without error. Enforcement lives at simulation entry
/// and index time -- never on the serde path.
#[test]
fn unknown_version_json_still_deserializes_without_error() {
    let mut m = manifest(false);
    m.simulation_version = CombatSimulationVersion::new("some_ancient_version_9000").unwrap();
    let json = serde_json::to_string(&m).unwrap();
    let decoded: CombatManifest =
        serde_json::from_str(&json).expect("archived version strings must still deserialize");
    assert_eq!(
        decoded.simulation_version.as_str(),
        "some_ancient_version_9000"
    );
}

/// Adding the version gate to `CombatSimulation::new()` must not perturb the
/// fingerprint formula it guards: the same input, built twice, must keep
/// producing the same manifest fingerprint and the same simulation
/// fingerprint (mirrors the existing repeatability tests in
/// `combat_simulation_wave2.rs`, scoped here to the version gate itself).
#[test]
fn version_enforcement_does_not_change_any_fingerprint() {
    let a = sim_input(CURRENT_SIMULATION_VERSION);
    let b = sim_input(CURRENT_SIMULATION_VERSION);
    assert_eq!(
        a.manifest.fingerprint().unwrap(),
        b.manifest.fingerprint().unwrap()
    );
    let sim_a = CombatSimulation::new(a).unwrap();
    let sim_b = CombatSimulation::new(b).unwrap();
    assert_eq!(sim_a.fingerprint().unwrap(), sim_b.fingerprint().unwrap());
}
