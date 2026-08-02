use escape_core::*;

fn request() -> CombatResolutionRequest {
    let manifest = CombatManifest {
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
    };
    let c = |id: &str| CombatantState {
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
    };
    let p = |id: &str, side: CombatSide| CombatSimulationParticipant {
        id: id.into(),
        side,
        position: CombatPosition { x: 0, y: 0 },
        facing: CombatFacing { x: 1, y: 0 },
        speed_per_tick: 1,
        collision_radius: 1,
        attack_range: 2,
        support_range: 2,
        role_id: "r".into(),
        target_policy_id: None,
        active: true,
    };
    CombatResolutionRequest {
        execution: CombatExecutionRequest {
            input: CombatSimulationInput {
                manifest,
                state: CombatState {
                    battle_id: "b".into(),
                    combatants: vec![c("a"), c("e")],
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
                participants: vec![p("a", CombatSide::Ally), p("e", CombatSide::Enemy)],
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

#[test]
fn resolution_is_repeatable_and_clamps_damage() {
    let a = resolve_combat(request()).unwrap();
    let b = resolve_combat(request()).unwrap();
    assert_eq!(a, b);
    assert_eq!(
        a.state
            .combatants
            .iter()
            .find(|c| c.id == "e")
            .unwrap()
            .current_health_hundredths,
        9500
    );
    assert!(a
        .full_log
        .iter()
        .any(|e| e.tag == CombatResolutionLogTag::DamageApplied));
}

#[test]
fn fixed_point_formula_and_invalid_effect_are_data_driven() {
    let mut r = request();
    r.attacks[0].effects.push(CombatAttackEffect {
        effect_id: "missing".into(),
        chance_percent: 100,
    });
    assert!(matches!(
        resolve_combat(r),
        Err(CombatResolutionError::UnknownEffect(_))
    ));
}

#[test]
fn collision_balance_applies_even_when_accuracy_misses() {
    let mut r = request();
    r.attacks[0].accuracy_percent = 0;
    let result = resolve_combat(r).unwrap();
    let target = result
        .state
        .combatants
        .iter()
        .find(|c| c.id == "e")
        .unwrap();
    assert_eq!(target.balance_hundredths, 9900);
    assert!(!result.frames[0].outcomes[0].hit);
    assert_eq!(result.frames[0].outcomes[0].balance_delta_hundredths, -100);
}

#[test]
fn effect_stacking_preserves_catalog_phase_and_persistent_lifetime() {
    let mut r = request();
    r.catalog.effects = vec![CombatEffectDefinition {
        id: "mark".into(),
        source: "skill".into(),
        category: CombatEffectCategory::State,
        target_selector: "target".into(),
        parameters: Default::default(),
        conditions: vec![],
        phase: EffectPhase::CombatStart,
        lifetime: EffectLifetime::Persistent,
        stacking: EffectStacking::Unique,
        stacking_group: "mark".into(),
        stacking_cap: None,
        priority: 5,
        visibility: EffectVisibility::Public,
        tags: vec![],
    }];
    r.attacks[0].effects = vec![CombatAttackEffect {
        effect_id: "mark".into(),
        chance_percent: 100,
    }];
    let mut duplicate = r.attacks[0].clone();
    duplicate.id = "slash_again".into();
    r.attacks.push(duplicate);
    let result = resolve_combat(r).unwrap();
    assert_eq!(result.state.active_effects.len(), 1);
    let active = &result.state.active_effects[0];
    assert!(!active.combat_only);
    assert_eq!(active.phase, EffectPhase::CombatStart);
    assert_eq!(result.state.suppressed_effect_ids, vec!["mark"]);
    assert!(result
        .full_log
        .iter()
        .any(|event| event.tag == CombatResolutionLogTag::EffectSuppressed));
}

#[test]
fn resolution_is_mode_and_presentation_invariant() {
    let baseline = resolve_combat(request()).unwrap();
    for mode in [
        CombatRunMode::Retry,
        CombatRunMode::Auto,
        CombatRunMode::Fast,
    ] {
        let mut r = request();
        r.execution.mode = mode;
        let result = resolve_combat(r).unwrap();
        assert_eq!(result.frames, baseline.frames);
        assert_eq!(result.state, baseline.state);
        assert_eq!(result.full_log, baseline.full_log);
        assert_eq!(result.core_log, baseline.core_log);
        assert_eq!(result.fingerprint, baseline.fingerprint);
    }
    for presentation in [
        CombatPresentationSpeed::TwoX,
        CombatPresentationSpeed::Instant,
    ] {
        let mut r = request();
        r.execution.presentation = presentation;
        let result = resolve_combat(r).unwrap();
        assert_eq!(result.frames, baseline.frames);
        assert_eq!(result.state, baseline.state);
        assert_eq!(result.full_log, baseline.full_log);
    }
}

#[test]
fn forecast_resolution_uses_a_separate_repeatable_namespace() {
    let mut r = request();
    r.execution.mode = CombatRunMode::Forecast;
    let first = resolve_combat(r.clone()).unwrap();
    let second = resolve_combat(r).unwrap();
    assert_eq!(first, second);
    assert_ne!(
        first.execution.effective_seed,
        request().execution.input.seed
    );
    assert_eq!(
        first.execution.namespace,
        CombatRngNamespace::ForecastEnsemble
    );
}

#[test]
fn attack_input_order_and_missing_target_state_are_deterministic_errors() {
    let mut ordered = request();
    let mut second = ordered.attacks[0].clone();
    second.id = "jab".into();
    ordered.attacks.push(second);
    let mut reordered = ordered.clone();
    reordered.attacks.reverse();
    assert_eq!(
        resolve_combat(ordered).unwrap(),
        resolve_combat(reordered).unwrap()
    );

    let mut missing = request();
    missing
        .execution
        .input
        .state
        .combatants
        .retain(|combatant| combatant.id == "a");
    missing
        .defenses
        .retain(|defense| defense.combatant_id == "a");
    assert!(matches!(
        resolve_combat(missing),
        Err(CombatResolutionError::InvalidInput)
    ));
}

#[test]
fn health_damage_clamps_to_zero_without_treating_lethal_damage_as_overflow() {
    let mut r = request();
    r.attacks[0].power_hundredths = 48_000;
    let result = resolve_combat(r).unwrap();
    assert_eq!(
        result
            .state
            .combatants
            .iter()
            .find(|c| c.id == "e")
            .unwrap()
            .current_health_hundredths,
        0
    );
}

#[test]
fn accuracy_range_penetration_and_overflow_are_explicit() {
    let mut mid = request();
    mid.attacks[0].accuracy_percent = 50;
    let mid_result = resolve_combat(mid).unwrap();
    let mid_outcome = &mid_result.frames[0].outcomes[0];
    assert_eq!(mid_outcome.hit, mid_outcome.roll_percent < 50);

    let mut far = request();
    far.execution.input.participants[1].position.x = 10;
    let far_result = resolve_combat(far).unwrap();
    let far_outcome = &far_result.frames[0].outcomes[0];
    assert!(!far_outcome.collision);
    assert!(!far_outcome.in_range);
    assert!(!far_outcome.hit);
    assert_eq!(
        far_result
            .state
            .combatants
            .iter()
            .find(|c| c.id == "e")
            .unwrap()
            .current_health_hundredths,
        10_000
    );

    let mut defended = request();
    defended.defenses[1].defense_hundredths = 500;
    let defended_result = resolve_combat(defended).unwrap();
    assert_eq!(defended_result.frames[0].outcomes[0].damage_hundredths, 400);

    let mut penetrated = request();
    penetrated.defenses[1].defense_hundredths = 500;
    penetrated.attacks[0].penetration_hundredths = 500;
    let penetrated_result = resolve_combat(penetrated).unwrap();
    assert_eq!(
        penetrated_result.frames[0].outcomes[0].damage_hundredths,
        500
    );

    let mut overflow = request();
    overflow.attacks[0].power_hundredths = i64::MAX;
    assert!(matches!(
        resolve_combat(overflow),
        Err(CombatResolutionError::Overflow)
    ));
}

#[test]
fn effect_stacking_policies_are_deterministic() {
    let definition = |id: &str, group: &str, stacking: EffectStacking, cap: Option<u32>| {
        CombatEffectDefinition {
            id: id.into(),
            source: "test".into(),
            category: CombatEffectCategory::State,
            target_selector: "target".into(),
            parameters: Default::default(),
            conditions: vec![],
            phase: EffectPhase::DuringCombat,
            lifetime: EffectLifetime::UntilCombatSettlement,
            stacking,
            stacking_group: group.into(),
            stacking_cap: cap,
            priority: 1,
            visibility: EffectVisibility::Public,
            tags: vec![],
        }
    };
    let mut r = request();
    r.catalog.effects = vec![
        definition("cap", "cap", EffectStacking::StackCount, Some(1)),
        definition(
            "independent",
            "independent",
            EffectStacking::Independent,
            None,
        ),
        definition("replace_a", "replace", EffectStacking::Replace, None),
        definition("replace_b", "replace", EffectStacking::Replace, None),
    ];
    r.attacks[0].effects = vec![
        CombatAttackEffect {
            effect_id: "cap".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "independent".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "replace_a".into(),
            chance_percent: 100,
        },
        CombatAttackEffect {
            effect_id: "replace_b".into(),
            chance_percent: 100,
        },
    ];
    let mut duplicate = r.attacks[0].clone();
    duplicate.id = "second".into();
    r.attacks.push(duplicate);
    let result = resolve_combat(r).unwrap();
    assert_eq!(
        result
            .state
            .active_effects
            .iter()
            .filter(|effect| effect.stacking_group == "cap")
            .count(),
        1
    );
    assert_eq!(
        result
            .state
            .active_effects
            .iter()
            .filter(|effect| effect.stacking_group == "independent")
            .count(),
        2
    );
    assert_eq!(
        result
            .state
            .active_effects
            .iter()
            .filter(|effect| effect.stacking_group == "replace")
            .map(|effect| effect.definition_id.as_str())
            .collect::<Vec<_>>(),
        vec!["replace_b"]
    );
}

#[test]
fn resolution_logs_are_stable_and_core_filtered() {
    let result = resolve_combat(request()).unwrap();
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
fn frame_snapshot_is_id_sorted_and_covers_every_combatant() {
    let result = resolve_combat(request()).unwrap();
    for frame in &result.frames {
        let ids: Vec<&str> = frame.combatants.iter().map(|c| c.id.as_str()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted, "frame.combatants must already be id-ascending");
        assert_eq!(
            ids,
            vec!["a", "e"],
            "frame.combatants must cover every combatant"
        );
    }
}

#[test]
fn frame_snapshot_reflects_the_tick_damage_from_its_own_outcomes() {
    let result = resolve_combat(request()).unwrap();
    let frame = &result.frames[0];
    let outcome = &frame.outcomes[0];
    let snapshot_health = frame
        .combatants
        .iter()
        .find(|c| c.id == outcome.target_id)
        .unwrap()
        .current_health_hundredths;
    assert_eq!(snapshot_health, 10_000 - outcome.damage_hundredths);
}

#[test]
fn last_frame_snapshot_matches_final_state_combatants() {
    let result = resolve_combat(request()).unwrap();
    let last_frame = result.frames.last().unwrap();
    assert_eq!(last_frame.combatants, result.state.combatants);
}

#[test]
fn combatants_field_is_additive_optional_for_deserialization() {
    let json = r#"{"tick":1,"outcomes":[],"fingerprint":"f"}"#;
    let frame: CombatResolutionFrame = serde_json::from_str(json).unwrap();
    assert_eq!(frame.combatants, Vec::new());
}

#[test]
fn frame_snapshots_are_deterministic_across_identical_runs() {
    let a = resolve_combat(request()).unwrap();
    let b = resolve_combat(request()).unwrap();
    assert_eq!(a.frames, b.frames);
    assert_eq!(a.fingerprint, b.fingerprint);
}

// ---------------------------------------------------------------------
// WP2 (fable_combat_early_conclusion_step1_2608022130.md, I1): an
// incapacitated combatant (tick-start health <= 0) neither attacks nor is
// attacked. Both attacks below reuse `request()`'s co-located a/e pair so
// collision + range are trivially satisfied; only health and attack ids
// change per test.
// ---------------------------------------------------------------------

/// A mirror of `request()`'s "slash" attack, but authored for the opposite
/// actor so both sides can strike each other within a single tick.
fn mirrored_attack(id: &str, actor_id: &str) -> CombatAttackDefinition {
    CombatAttackDefinition {
        id: id.into(),
        actor_id: actor_id.into(),
        power_hundredths: 1200,
        ability_multiplier_hundredths: 100,
        accuracy_percent: 100,
        attack_range: 2,
        penetration_hundredths: 0,
        collision_balance_hundredths: 100,
        balance_power_hundredths: 500,
        effects: vec![],
    }
}

/// 여러 tick을 돌리는 픽스처. `request()`는 1 tick만 돌도록 되어 있다.
fn set_ticks(r: &mut CombatResolutionRequest, ticks: u32) {
    r.execution.ticks = ticks;
    r.execution.input.config.max_ticks = ticks;
}

fn set_current_health(r: &mut CombatResolutionRequest, id: &str, current_health: i32) {
    let combatant = r
        .execution
        .input
        .state
        .combatants
        .iter_mut()
        .find(|c| c.id == id)
        .expect("combatant must exist in the fixture");
    combatant.current_health = current_health;
}

#[test]
fn incapacitated_actor_and_incapacitated_target_are_both_skipped() {
    // "e" starts already at 0 health (tick-start snapshot). Both attacks
    // reference it: "a"'s attack has an incapacitated *target*, and "e"'s own
    // attack has an incapacitated *actor*. Neither may create an outcome,
    // roll, or log entry (I1).
    let mut r = request();
    r.attacks[0].id = "ally_strike".into();
    r.attacks.push(mirrored_attack("challenger_strike", "e"));
    set_current_health(&mut r, "e", 0);

    let result = resolve_combat(r).unwrap();
    assert!(
        result.frames[0].outcomes.is_empty(),
        "an already-incapacitated actor/target must not produce any attack outcome"
    );
    assert!(
        result.full_log.is_empty(),
        "a skipped attack must not create any log entry"
    );
    assert_eq!(
        result
            .state
            .combatants
            .iter()
            .find(|c| c.id == "a")
            .unwrap()
            .current_health_hundredths,
        10_000,
        "the untouched ally must keep full health since the only attack against it never ran"
    );
}

#[test]
fn simultaneous_mutual_defeat_is_independent_of_attack_definition_order() {
    // Both combatants start alive with just enough health that a single hit
    // (500 hundredths) is lethal. Both attack each other in the same tick.
    // The attack_map is a BTreeMap keyed by attack id, so whichever attack
    // sorts first is *processed* first -- but I1 requires the incapacitation
    // check to use the tick's *starting* health snapshot, not the
    // partially-mutated live map. So both orderings below must still produce
    // a mutual knockout (both combatants at 0, both attacks landed), proving
    // the result does not depend on attack processing order.
    let build = |ally_attack_id: &str, enemy_attack_id: &str| {
        let mut r = request();
        r.attacks[0].id = ally_attack_id.into();
        r.attacks.push(mirrored_attack(enemy_attack_id, "e"));
        set_current_health(&mut r, "a", 1);
        set_current_health(&mut r, "e", 1);
        r
    };

    // Ally's attack id sorts first alphabetically.
    let ally_first = resolve_combat(build("a_ally_strike", "z_challenger_strike")).unwrap();
    // Enemy's attack id now sorts first -- processing order is flipped.
    let enemy_first = resolve_combat(build("z_ally_strike", "a_challenger_strike")).unwrap();

    for result in [&ally_first, &enemy_first] {
        assert_eq!(
            result.frames[0].outcomes.len(),
            2,
            "both attacks must land in the same tick even though one actor is knocked out \
             mid-tick by the other's attack"
        );
        assert!(result.frames[0].outcomes.iter().all(|o| o.hit));
        for id in ["a", "e"] {
            assert_eq!(
                result
                    .state
                    .combatants
                    .iter()
                    .find(|c| c.id == id)
                    .unwrap()
                    .current_health_hundredths,
                0,
                "mutual knockout: both combatants must end the tick at 0 health"
            );
        }
    }
    assert_eq!(
        ally_first.state, enemy_first.state,
        "final state must be identical regardless of attack definition order"
    );
}

// ---------------------------------------------------------------------
// WP3 (같은 플랜, I2): 결착 tick 이후를 시뮬레이션하지 않는다. 결착 조건은
// `conclude`와 공유하는 `side_all_defeated` 하나뿐이다. 한 명중은 500
// hundredths(= 체력 5)이므로 체력 15는 정확히 3타에 쓰러진다.
// ---------------------------------------------------------------------

#[test]
fn simulation_stops_at_the_tick_that_concludes_the_fight() {
    let mut r = request();
    set_ticks(&mut r, 6);
    set_current_health(&mut r, "e", 15);

    let result = resolve_combat(r).unwrap();
    assert_eq!(
        result.frames.len(),
        3,
        "enemy dies on the third hit, so tick 3 must be the last simulated tick"
    );
    let last_tick = result.frames.last().unwrap().tick;
    assert!(
        result.frames.iter().all(|f| f.tick <= last_tick),
        "no frame may exist after the concluding tick"
    );
    assert!(
        result.full_log.iter().all(|e| e.tick <= last_tick),
        "no log entry may exist after the concluding tick"
    );
}

#[test]
fn the_concluding_tick_frame_is_included_with_its_outcomes() {
    let mut r = request();
    set_ticks(&mut r, 6);
    set_current_health(&mut r, "e", 15);

    let result = resolve_combat(r).unwrap();
    let last = result.frames.last().unwrap();
    assert!(
        !last.outcomes.is_empty(),
        "the tick where the fight concluded must still show what happened in it"
    );
    assert_eq!(
        last.combatants
            .iter()
            .find(|c| c.id == "e")
            .unwrap()
            .current_health_hundredths,
        0,
        "the concluding tick's snapshot must show the knockout that ended it"
    );
}

#[test]
fn a_fight_with_no_conclusion_runs_every_tick() {
    let mut r = request();
    set_ticks(&mut r, 6);
    // 체력 100은 6타(30)로 죽지 않는다 -> 결착 없음.
    let result = resolve_combat(r).unwrap();
    assert_eq!(
        result.frames.len(),
        6,
        "without a terminal condition the resolver must keep going to the tick limit"
    );
}

#[test]
fn simultaneous_mutual_defeat_stops_at_that_tick() {
    let mut r = request();
    set_ticks(&mut r, 6);
    r.attacks[0].id = "a_ally_strike".into();
    r.attacks.push(mirrored_attack("z_challenger_strike", "e"));
    set_current_health(&mut r, "a", 15);
    set_current_health(&mut r, "e", 15);

    let result = resolve_combat(r).unwrap();
    assert_eq!(
        result.frames.len(),
        3,
        "both sides wiped on the same tick must stop the simulation on that tick"
    );
    for id in ["a", "e"] {
        assert_eq!(
            result
                .state
                .combatants
                .iter()
                .find(|c| c.id == id)
                .unwrap()
                .current_health_hundredths,
            0
        );
    }
}
