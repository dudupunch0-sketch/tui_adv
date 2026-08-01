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

/// "a" attacks "e" with a guaranteed hit (accuracy 100) and "e" counters "a" with a
/// guaranteed miss (accuracy 0) while still colliding/in range, so a single tick
/// exercises all three cue rules at once: Attack (both actors), Hit (e is struck by
/// a's hit), Evade (a is missed by e's in-range attack).
fn two_way_resolution_request() -> CombatResolutionRequest {
    let mut request = resolution_request();
    request.attacks.push(CombatAttackDefinition {
        id: "counter".into(),
        actor_id: "e".into(),
        power_hundredths: 1200,
        ability_multiplier_hundredths: 100,
        accuracy_percent: 0,
        attack_range: 2,
        penetration_hundredths: 0,
        collision_balance_hundredths: 100,
        balance_power_hundredths: 500,
        effects: vec![],
    });
    request
}

fn two_way_spectator_request() -> CombatSpectatorRequest {
    let resolution = resolve_combat(two_way_resolution_request()).unwrap();
    CombatSpectatorRequest {
        resolution,
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    }
}

fn effect_def(id: &str, group: &str, visibility: EffectVisibility) -> CombatEffectDefinition {
    CombatEffectDefinition {
        id: id.into(),
        source: "skill".into(),
        category: CombatEffectCategory::State,
        target_selector: "target".into(),
        parameters: Default::default(),
        conditions: vec![],
        phase: EffectPhase::DuringCombat,
        lifetime: EffectLifetime::UntilCombatSettlement,
        stacking: EffectStacking::Unique,
        stacking_group: group.into(),
        stacking_cap: None,
        priority: 1,
        visibility,
        tags: vec![],
    }
}

/// "a" hits "e" (accuracy 100, guaranteed) applying four effects at once:
/// - `buff_public` (Public) should pass through unmasked.
/// - `buff_hidden` (Hidden) should be masked.
/// - `buff_conditional` (Conditional) should be masked.
/// - `buff_unregistered` exists in the *resolution's* catalog (so `resolve_combat`
///   accepts it) but is deliberately left out of the *spectator's* catalog, to
///   exercise the "unknown to spectator catalog -> masked" safe default.
fn leak_resolution_request() -> CombatResolutionRequest {
    let mut request = resolution_request();
    request.catalog.effects = vec![
        effect_def("buff_public", "public", EffectVisibility::Public),
        effect_def("buff_hidden", "hidden", EffectVisibility::Hidden),
        effect_def(
            "buff_conditional",
            "conditional",
            EffectVisibility::Conditional,
        ),
        effect_def(
            "buff_unregistered",
            "unregistered",
            EffectVisibility::Public,
        ),
    ];
    request.attacks[0].effects = vec![
        CombatAttackEffect {
            effect_id: "buff_public".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "buff_hidden".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "buff_conditional".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "buff_unregistered".into(),
            chance_percent: 100,
        },
    ];
    request
}

fn leak_spectator_request() -> CombatSpectatorRequest {
    let resolution = resolve_combat(leak_resolution_request()).unwrap();
    CombatSpectatorRequest {
        resolution,
        participants: participants(),
        // Deliberately omits `buff_unregistered` to simulate an id the spectator's
        // catalog does not recognize.
        catalog: CombatEffectCatalog {
            effects: vec![
                effect_def("buff_public", "public", EffectVisibility::Public),
                effect_def("buff_hidden", "hidden", EffectVisibility::Hidden),
                effect_def(
                    "buff_conditional",
                    "conditional",
                    EffectVisibility::Conditional,
                ),
            ],
        },
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

#[test]
fn attack_hit_and_evade_cues_follow_the_three_rules_only() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    let frame = &view.frames[0];
    let piece = |id: &str| frame.pieces.iter().find(|p| p.id == id).unwrap();
    assert_eq!(
        piece("a").cues,
        vec![CombatSpectatorCue::Attack, CombatSpectatorCue::Evade]
    );
    assert_eq!(
        piece("e").cues,
        vec![CombatSpectatorCue::Attack, CombatSpectatorCue::Hit]
    );
}

#[test]
fn cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    for piece in &view.frames[0].pieces {
        let mut sorted = piece.cues.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(piece.cues, sorted, "cues must already be sorted+deduped");
    }
}

#[test]
fn log_entries_use_registered_template_ids_not_free_sentences() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    assert!(view
        .full_log
        .iter()
        .any(|e| e.template_id == "combat.log.move_intent"));
    assert!(view
        .full_log
        .iter()
        .any(|e| e.template_id == "combat.log.target_selection"));
    assert!(view
        .full_log
        .iter()
        .any(|e| e.template_id == "combat.log.collision"));
    let damage = view
        .full_log
        .iter()
        .find(|e| e.template_id == "combat.log.damage_applied")
        .expect("damage_applied entry expected");
    assert!(damage.value_hundredths.unwrap() > 0);
    assert_eq!(damage.target_id.as_deref(), Some("e"));
}

#[test]
fn full_log_is_ordered_by_tick_then_sequence() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    assert!(view
        .full_log
        .windows(2)
        .all(|pair| (pair[0].tick, pair[0].sequence) <= (pair[1].tick, pair[1].sequence)));
}

#[test]
fn core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    assert!(view
        .core_log
        .iter()
        .all(|entry| entry.importance >= CombatLogImportance::Important));
    // core_log preserves the relative order it had inside full_log.
    let mut cursor = 0usize;
    for entry in &view.core_log {
        let found = view.full_log[cursor..]
            .iter()
            .position(|candidate| candidate == entry)
            .expect("core_log entry must appear in full_log at or after the cursor");
        cursor += found + 1;
    }
}

#[test]
fn attack_roll_and_effect_suppressed_never_leak_into_any_log() {
    let request = two_way_spectator_request();
    let view = spectate_combat(&request).unwrap();
    for entry in view.full_log.iter().chain(view.core_log.iter()) {
        assert_ne!(entry.template_id, "combat.log.attack_roll");
        assert_ne!(entry.template_id, "combat.log.effect_suppressed");
    }
}

#[test]
fn hidden_conditional_and_unregistered_effect_ids_are_masked() {
    let request = leak_spectator_request();
    let view = spectate_combat(&request).unwrap();
    let effect_entries: Vec<_> = view
        .full_log
        .iter()
        .filter(|e| {
            e.template_id == "combat.log.effect_applied"
                || e.template_id == "combat.log.effect_applied_hidden"
        })
        .collect();
    assert_eq!(effect_entries.len(), 4);

    let public = effect_entries
        .iter()
        .find(|e| e.effect_id.as_deref() == Some("buff_public"))
        .expect("public effect id must remain visible");
    assert_eq!(public.template_id, "combat.log.effect_applied");

    let masked_count = effect_entries
        .iter()
        .filter(|e| e.template_id == "combat.log.effect_applied_hidden" && e.effect_id.is_none())
        .count();
    assert_eq!(
        masked_count, 3,
        "buff_hidden, buff_conditional, and buff_unregistered must all be masked"
    );
}
