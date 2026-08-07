use escape_core::*;
fn input() -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: CombatManifest {
            simulation_version: CombatSimulationVersion::new(CURRENT_SIMULATION_VERSION).unwrap(),
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
                position: HexCoord { q: 0, r: 0 },
                facing: HexCoord { q: 1, r: 0 },
                speed_per_tick: 1,
                collision_radius: 1,
                attack_range: 2,
                support_range: 2,
                role_id: "role".into(),
                target_policy_id: None,
                active: true,
                // T1-d (fable_combat_hex_t1d_step1_2608072234.md): new field
                // on `CombatSimulationParticipant`. Empty = single tile at
                // the anchor, exactly this fixture's pre-T1-d meaning --
                // mechanical fix to keep this file compiling, not a scope
                // change; see the T1-d step2 report for why this file
                // needed touching despite being outside that slice's
                // original ownership list.
                occupies: vec![],
            },
            CombatSimulationParticipant {
                id: "enemy".into(),
                side: CombatSide::Enemy,
                position: HexCoord { q: 10, r: 0 },
                facing: HexCoord { q: -1, r: 0 },
                speed_per_tick: 1,
                collision_radius: 1,
                attack_range: 2,
                support_range: 2,
                role_id: "role".into(),
                target_policy_id: None,
                active: true,
                occupies: vec![],
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
    position.input.participants[0].position.q = 1;
    assert_ne!(a.fingerprint, execute_combat(position).unwrap().fingerprint);
}

/// 정본 03 "전투 기록에는 version을 저장한다"의 구현 확인: `execute()`가 채우는
/// provenance가 입력 manifest의 simulation_version, config의 tick_millis,
/// manifest fingerprint와 정확히 일치해야 한다.
#[test]
fn provenance_matches_input_manifest_version_tick_millis_and_fingerprint() {
    let req = request(CombatRunMode::Actual, CombatPresentationSpeed::OneX);
    let expected_version = req.input.manifest.simulation_version.clone();
    let expected_tick_millis = req.input.config.tick_millis;
    let expected_manifest_fingerprint = req.input.manifest.fingerprint().unwrap();
    let result = execute_combat(req).unwrap();
    let provenance = result
        .provenance
        .expect("execute() always fills provenance");
    assert_eq!(provenance.simulation_version, expected_version);
    assert_eq!(provenance.tick_millis, expected_tick_millis);
    assert_eq!(
        provenance.manifest_fingerprint,
        expected_manifest_fingerprint
    );
}

/// `Forecast` 모드는 seed만 파생될 뿐, provenance는 입력을 그대로 반영해야 한다.
#[test]
fn forecast_mode_reports_the_same_provenance_as_input() {
    let req = request(CombatRunMode::Forecast, CombatPresentationSpeed::OneX);
    let expected_version = req.input.manifest.simulation_version.clone();
    let expected_tick_millis = req.input.config.tick_millis;
    let result = execute_combat(req).unwrap();
    let provenance = result
        .provenance
        .expect("execute() always fills provenance");
    assert_eq!(provenance.simulation_version, expected_version);
    assert_eq!(provenance.tick_millis, expected_tick_millis);
}

/// provenance 필드가 없는(구 JSON) 결과는 역직렬화 시 `None`이어야 하고, 이것은
/// 에러가 아니라 additive-optional 계약의 정상 경로다 (invariant 4).
#[test]
fn deserializing_result_json_without_provenance_field_yields_none() {
    let a = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    let mut value = serde_json::to_value(&a).unwrap();
    value.as_object_mut().unwrap().remove("provenance");
    let result: CombatExecutionResult = serde_json::from_value(value).unwrap();
    assert_eq!(result.provenance, None);
}

/// 같은 입력을 두 번 실행하면 provenance도 동일해야 한다 (결정론).
#[test]
fn same_input_executed_twice_yields_identical_provenance() {
    let a = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    let b = execute_combat(request(
        CombatRunMode::Actual,
        CombatPresentationSpeed::OneX,
    ))
    .unwrap();
    assert_eq!(a.provenance, b.provenance);
}
