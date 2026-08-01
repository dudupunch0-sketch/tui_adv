use escape_core::*;

fn manifest() -> CombatManifest {
    CombatManifest {
        simulation_version: CombatSimulationVersion::new("v1").unwrap(),
        actual_seed: 7,
        world_state_fingerprint: "w".into(),
        applied_effects: vec![],
        suppressed_effects: vec![],
        combatant_ids: vec!["a".into(), "e".into()],
        placement_ids: vec![],
        environment_ids: vec![],
        team_ids: vec![],
        rule_ids: vec![],
        public_info_ids: vec![],
    }
}

fn combatant(id: &str) -> CombatantState {
    CombatantState {
        id: id.into(),
        current_health: 100,
        maximum_health: 100,
        current_breath: 1,
        maximum_breath: 1,
        balance: 100,
        maximum_balance: 100,
        fear: 0,
        anger: 0,
        posture: Posture::Neutral,
        weapon_control: WeaponControl::Stable,
        relationship_refs: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        persistent_status_ids: vec![],
        combat_effect_ids: vec![],
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

fn participants() -> Vec<CombatSimulationParticipant> {
    vec![
        participant("a", CombatSide::Ally, 0),
        participant("e", CombatSide::Enemy, 0),
    ]
}

fn resolution_request() -> CombatResolutionRequest {
    CombatResolutionRequest {
        execution: CombatExecutionRequest {
            input: CombatSimulationInput {
                manifest: manifest(),
                state: CombatState {
                    battle_id: "b".into(),
                    combatants: vec![combatant("a"), combatant("e")],
                    persistent_statuses: vec![],
                    active_effects: vec![],
                    environment_refs: vec![],
                    team_refs: vec![],
                    team_formations: vec![],
                    relationships: vec![],
                    environment_states: vec![],
                    manifest_fingerprint: "fp".into(),
                },
                seed: 7,
                config: CombatSimulationConfig {
                    tick_millis: 100,
                    max_ticks: 1,
                },
                participants: participants(),
                roles: vec![CombatRolePreset {
                    id: "r".into(),
                    weights: CombatRoleWeights {
                        preferred_distance: 1,
                        aggression: 1,
                        formation_maintenance: 0,
                        pursuit_range: 1,
                        protect_priority: 0,
                        target_priority: 0,
                        risk_tolerance: 0,
                        ability_priority: 0,
                    },
                }],
                policies: vec![],
            },
            mode: CombatRunMode::Actual,
            presentation: CombatPresentationSpeed::OneX,
            ticks: 1,
        },
        attacks: vec![CombatAttackDefinition {
            id: "slash".into(),
            actor_id: "a".into(),
            power_hundredths: 1200,
            ability_multiplier_hundredths: 100,
            accuracy_percent: 100,
            attack_range: 2,
            penetration_hundredths: 0,
            collision_balance_hundredths: 100,
            balance_power_hundredths: 500,
            effects: vec![],
        }],
        defenses: vec![
            CombatDefenseProfile {
                combatant_id: "a".into(),
                defense_hundredths: 0,
                balance_resistance_hundredths: 0,
            },
            CombatDefenseProfile {
                combatant_id: "e".into(),
                defense_hundredths: 0,
                balance_resistance_hundredths: 0,
            },
        ],
        catalog: CombatEffectCatalog { effects: vec![] },
    }
}

fn spectator_request() -> CombatSpectatorRequest {
    let resolution = resolve_combat(resolution_request()).unwrap();
    CombatSpectatorRequest {
        resolution,
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    }
}

#[test]
fn frame_positions_facing_side_and_active_match_input() {
    let request = spectator_request();
    let view = spectate_combat(&request).unwrap();
    assert_eq!(view.frames.len(), 1);
    let frame = &view.frames[0];
    assert_eq!(frame.tick, 1);
    for piece in &frame.pieces {
        let expected = request
            .participants
            .iter()
            .find(|p| p.id == piece.id)
            .unwrap();
        assert_eq!(piece.side, expected.side);
        assert_eq!(piece.facing, expected.facing);
        assert_eq!(piece.active, expected.active);
        let tick_position = request.resolution.execution.frames[0]
            .positions
            .get(&piece.id)
            .unwrap();
        assert_eq!(piece.position, *tick_position);
    }
}

#[test]
fn unknown_participant_is_rejected() {
    let mut request = spectator_request();
    request.participants.retain(|p| p.id != "e");
    assert_eq!(
        spectate_combat(&request),
        Err(CombatSpectatorError::UnknownParticipant("e".into()))
    );
}

#[test]
fn participant_input_order_does_not_affect_view() {
    let mut reordered = spectator_request();
    reordered.participants.reverse();
    assert_eq!(
        spectate_combat(&spectator_request()).unwrap(),
        spectate_combat(&reordered).unwrap()
    );
}
