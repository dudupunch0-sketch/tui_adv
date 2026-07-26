use escape_core::*;
fn input() -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: CombatManifest {
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
        },
        state: CombatState {
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
        },
        seed: 1,
        config: CombatSimulationConfig {
            tick_millis: 100,
            max_ticks: 3,
        },
        participants: vec![
            CombatSimulationParticipant {
                id: "ally".into(),
                side: CombatSide::Ally,
                position: CombatPosition { x: 0, y: 0 },
                facing: CombatFacing { x: 1, y: 0 },
                speed_per_tick: 1,
                collision_radius: 1,
                attack_range: 2,
                support_range: 2,
                role_id: "role".into(),
                target_policy_id: None,
                active: true,
            },
            CombatSimulationParticipant {
                id: "enemy".into(),
                side: CombatSide::Enemy,
                position: CombatPosition { x: 10, y: 0 },
                facing: CombatFacing { x: -1, y: 0 },
                speed_per_tick: 1,
                collision_radius: 1,
                attack_range: 2,
                support_range: 2,
                role_id: "role".into(),
                target_policy_id: None,
                active: true,
            },
        ],
        roles: vec![CombatRolePreset {
            id: "role".into(),
            weights: CombatRoleWeights {
                preferred_distance: 2,
                aggression: 1,
                formation_maintenance: 0,
                pursuit_range: 0,
                protect_priority: 0,
                target_priority: 0,
                risk_tolerance: 0,
                ability_priority: 0,
            },
        }],
        policies: vec![],
    }
}
fn request(mode: CombatRunMode, presentation: CombatPresentationSpeed) -> CombatExecutionRequest {
    CombatExecutionRequest {
        input: input(),
        mode,
        presentation,
        ticks: 2,
    }
}
#[test]
fn actual_retry_auto_fast_parity() {
    let a = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    for mode in [
        CombatRunMode::Retry,
        CombatRunMode::Auto,
        CombatRunMode::Fast,
    ] {
        let b = execute_combat(request(mode, CombatPresentationSpeed::OneX)).unwrap();
        assert_eq!(a.frames, b.frames);
        assert_eq!(a.full_log, b.full_log);
        assert_eq!(a.core_log, b.core_log);
        assert_eq!(a.fingerprint, b.fingerprint);
    }
}
#[test]
fn forecast_namespace_isolated_and_repeatable() {
    let a = execute_combat(request(
        CombatRunMode::Forecast,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    let b = execute_combat(request(
        CombatRunMode::Forecast,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    assert_ne!(
        a.effective_seed,
        request(CombatRunMode::Actual, CombatPresentationSpeed::OneX)
            .input
            .seed
    );
    assert_eq!(a, b);
}
#[test]
fn presentation_speed_does_not_change_result() {
    let a = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    for speed in [
        CombatPresentationSpeed::TwoX,
        CombatPresentationSpeed::Instant,
    ] {
        let b = execute_combat(request(CombatRunMode::Actual, speed)).unwrap();
        assert_eq!(a.frames, b.frames);
        assert_eq!(a.full_log, b.full_log);
    }
}
#[test]
fn zero_ticks_and_over_max_rejected() {
    assert!(matches!(
        execute_combat(CombatExecutionRequest {
            ticks: 0,
            ..request(CombatRunMode::Actual, CombatPresentationSpeed::OneX)
        }),
        Err(CombatExecutionError::ZeroTicks)
    ));
    assert!(execute_combat(CombatExecutionRequest {
        ticks: 4,
        ..request(CombatRunMode::Actual, CombatPresentationSpeed::OneX)
    })
    .is_err());
}
#[test]
fn core_log_filters_routine_and_full_log_is_stable() {
    let result = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    assert!(result
        .full_log
        .windows(2)
        .all(|pair| (pair[0].tick, pair[0].sequence) <= (pair[1].tick, pair[1].sequence)));
    assert!(result
        .core_log
        .iter()
        .all(|event| event.importance >= CombatLogImportance::Important));
}

#[test]
fn execution_fingerprint_binds_world_and_positions() {
    let a = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    let mut world = request(CombatRunMode::Actual, CombatPresentationSpeed::OneX);
    world.input.manifest.world_state_fingerprint = "changed".into();
    assert_ne!(a.fingerprint, execute_combat(world).unwrap().fingerprint);
    let mut position = request(CombatRunMode::Actual, CombatPresentationSpeed::OneX);
    position.input.participants[0].position.x = 1;
    assert_ne!(a.fingerprint, execute_combat(position).unwrap().fingerprint);
}
