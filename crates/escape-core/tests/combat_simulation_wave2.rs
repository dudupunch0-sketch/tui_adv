use escape_core::*;
fn manifest() -> CombatManifest {
    CombatManifest {
        simulation_version: CombatSimulationVersion::new("v1").unwrap(),
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
    }
}
fn state() -> CombatState {
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
fn role() -> CombatRolePreset {
    CombatRolePreset {
        id: "r".into(),
        weights: CombatRoleWeights {
            preferred_distance: 5,
            aggression: 1,
            formation_maintenance: 0,
            pursuit_range: 0,
            protect_priority: 0,
            target_priority: 0,
            risk_tolerance: 0,
            ability_priority: 0,
        },
    }
}
fn participant(id: &str, side: CombatSide, x: i32) -> CombatSimulationParticipant {
    CombatSimulationParticipant {
        id: id.into(),
        side,
        position: CombatPosition { x, y: 0 },
        facing: CombatFacing { x: 1, y: 0 },
        speed_per_tick: 1,
        collision_radius: 1,
        attack_range: 2,
        support_range: 2,
        role_id: "r".into(),
        target_policy_id: None,
        active: true,
    }
}
fn input(parts: Vec<CombatSimulationParticipant>) -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: manifest(),
        state: state(),
        seed: 1,
        config: CombatSimulationConfig {
            tick_millis: 100,
            max_ticks: 3,
        },
        participants: parts,
        roles: vec![role()],
        policies: vec![],
    }
}
#[test]
fn config_geometry_and_facing_validate() {
    assert!(CombatSimulation::new(CombatSimulationInput {
        config: CombatSimulationConfig {
            tick_millis: 0,
            max_ticks: 1
        },
        ..input(vec![])
    })
    .is_err());
    assert!(CombatPosition { x: i32::MAX, y: 0 }
        .distance_squared(CombatPosition { x: i32::MIN, y: 0 })
        .is_err());
    assert!(CombatPosition { x: 0, y: 0 }
        .in_range(CombatPosition { x: 1, y: 0 }, -1)
        .is_err());
}
#[test]
fn active_limits_ignore_inactive() {
    let mut p = (0..5)
        .map(|i| participant(&format!("a{i}"), CombatSide::Ally, i))
        .collect::<Vec<_>>();
    p[4].active = false;
    p.push(participant("e", CombatSide::Enemy, 10));
    assert!(CombatSimulation::new(input(p)).is_ok());
}
#[test]
fn target_preference_falls_back_nearest() {
    let mut i = input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e1", CombatSide::Enemy, 4),
        participant("e2", CombatSide::Enemy, 2),
    ]);
    i.policies = vec![CombatTargetPolicy {
        id: "p".into(),
        preferences: vec![
            CombatTargetPreference {
                target_id: "e1".into(),
                priority: 2,
            },
            CombatTargetPreference {
                target_id: "e2".into(),
                priority: 1,
            },
        ],
        fallback: CombatTargetFallback::Nearest,
    }];
    i.participants[0].target_policy_id = Some("p".into());
    let s = CombatSimulation::new(i).unwrap();
    assert_eq!(
        s.select_target(&s.participants().next().unwrap()).unwrap(),
        Some("e1".into())
    );
}
#[test]
fn role_speed_produces_advance_and_hold() {
    let mut s = CombatSimulation::new(input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 10),
    ]))
    .unwrap();
    assert_eq!(
        s.advance_tick().unwrap().moves[0].mode,
        CombatMoveMode::Advance
    );
}
#[test]
fn snapshot_order_invariant_and_same_setup() {
    let a = input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 10),
    ]);
    let mut b = a.clone();
    b.participants.reverse();
    let mut sa = CombatSimulation::new(a).unwrap();
    let mut sb = CombatSimulation::new(b).unwrap();
    assert_eq!(sa.run_ticks(2).unwrap(), sb.run_ticks(2).unwrap());
}
#[test]
fn range_overlap_boundaries() {
    let p = CombatPosition { x: 0, y: 0 };
    assert!(p.in_range(CombatPosition { x: 2, y: 0 }, 2).unwrap());
    assert!(p.overlaps(CombatPosition { x: 1, y: 0 }, 1).unwrap());
}
#[test]
fn max_ticks_and_missing_refs_fail() {
    let mut s = CombatSimulation::new(input(vec![])).unwrap();
    assert!(s.run_ticks(4).is_err());
    let mut i = input(vec![participant("a", CombatSide::Ally, 0)]);
    i.participants[0].role_id = "missing".into();
    assert!(CombatSimulation::new(i).is_err());
}
#[test]
fn seed_and_version_bind_simulation_fingerprint() {
    let a = input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 10),
    ]);
    let mut b = a.clone();
    b.seed = 2;
    let sa = CombatSimulation::new(a).unwrap();
    let sb = CombatSimulation::new(b).unwrap();
    assert_ne!(sa.fingerprint().unwrap(), sb.fingerprint().unwrap());
}

#[test]
fn setup_fingerprint_and_structural_duplicates_are_deterministic_errors() {
    let a = input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 10),
    ]);
    let mut b = a.clone();
    b.participants.reverse();
    assert_eq!(
        CombatSimulation::new(a.clone()).unwrap().fingerprint(),
        CombatSimulation::new(b).unwrap().fingerprint()
    );
    let mut duplicate = a.clone();
    duplicate
        .participants
        .push(participant("a", CombatSide::Enemy, 20));
    assert!(matches!(
        CombatSimulation::new(duplicate),
        Err(CombatSimulationError::DuplicateId(_))
    ));
    let mut policy_input = a;
    policy_input.policies = vec![CombatTargetPolicy {
        id: "p".into(),
        preferences: vec![
            CombatTargetPreference {
                target_id: "e".into(),
                priority: 1,
            },
            CombatTargetPreference {
                target_id: "e".into(),
                priority: 0,
            },
        ],
        fallback: CombatTargetFallback::Nearest,
    }];
    policy_input.participants[0].target_policy_id = Some("p".into());
    assert!(matches!(
        CombatSimulation::new(policy_input),
        Err(CombatSimulationError::DuplicateId(_))
    ));
}

#[test]
fn state_changes_simulation_identity() {
    let a = input(vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 10),
    ]);
    let mut b = a.clone();
    b.state.manifest_fingerprint = "different".into();
    assert_ne!(
        CombatSimulation::new(a).unwrap().fingerprint(),
        CombatSimulation::new(b).unwrap().fingerprint()
    );
}
