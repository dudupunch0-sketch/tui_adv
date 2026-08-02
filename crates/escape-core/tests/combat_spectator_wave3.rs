use escape_core::*;

/// tick 한 칸의 길이(ms). 시뮬레이션 config에 설정하면 `execute()`가 provenance에
/// 옮기고, `spectate()`는 그 provenance에서 이 값을 읽는다. "시뮬레이션 시간 = 화면
/// 시간" 계약은 이제 request가 아니라 provenance를 통해 성립한다.
const SIM_TICK_MILLIS: u32 = 100;

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
                    tick_millis: SIM_TICK_MILLIS,
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

#[test]
fn spectate_is_deterministic_for_identical_input() {
    let request = two_way_spectator_request();
    let a = spectate_combat(&request).unwrap();
    let b = spectate_combat(&request).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.fingerprint, b.fingerprint);
    assert!(!a.fingerprint.is_empty());
}

#[test]
fn fingerprint_chains_the_resolution_fingerprint() {
    let mut low_seed = resolution_request();
    low_seed.execution.input.seed = 7;
    let mut high_seed = resolution_request();
    high_seed.execution.input.seed = 99;

    let res_a = resolve_combat(low_seed).unwrap();
    let res_b = resolve_combat(high_seed).unwrap();
    assert_ne!(res_a.fingerprint, res_b.fingerprint);

    let view_a = spectate_combat(&CombatSpectatorRequest {
        resolution: res_a.clone(),
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    })
    .unwrap();
    let view_b = spectate_combat(&CombatSpectatorRequest {
        resolution: res_b.clone(),
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    })
    .unwrap();

    assert_eq!(view_a.resolution_fingerprint, res_a.fingerprint);
    assert_eq!(view_b.resolution_fingerprint, res_b.fingerprint);
    assert_ne!(view_a.fingerprint, view_b.fingerprint);
}

#[test]
fn view_reports_the_tick_millis_from_provenance() {
    let view = spectate_combat(&spectator_request()).unwrap();
    assert_eq!(
        view.tick_millis, SIM_TICK_MILLIS,
        "the view must report the simulation tick length carried by execution provenance, not a placeholder"
    );
}

#[test]
fn view_reports_the_simulation_version_from_provenance() {
    let view = spectate_combat(&spectator_request()).unwrap();
    assert_eq!(
        view.simulation_version,
        manifest().simulation_version,
        "the view must report the simulation_version carried by execution provenance, not a placeholder"
    );
}

#[test]
fn missing_provenance_is_rejected() {
    let mut request = spectator_request();
    request.resolution.execution.provenance = None;
    assert_eq!(
        spectate_combat(&request),
        Err(CombatSpectatorError::MissingProvenance)
    );
}

/// "a" one-shots "e" for lethal damage but with zero balance impact, so only the
/// health snapshot reaches 0 and `Incapacitated` (not `BalanceBroken`) applies.
fn incapacitated_only_request() -> CombatResolutionRequest {
    let mut request = resolution_request();
    request.attacks[0].power_hundredths = 48_000;
    request.attacks[0].accuracy_percent = 100;
    request.attacks[0].balance_power_hundredths = 0;
    request.attacks[0].collision_balance_hundredths = 0;
    request
}

/// "a" guarantees a miss against "e" (accuracy 0) but still collides, and the
/// collision-balance penalty alone is large enough to zero "e"'s balance snapshot,
/// so only `BalanceBroken` (not `Incapacitated`, since a miss deals no damage) applies.
fn balance_broken_only_request() -> CombatResolutionRequest {
    let mut request = resolution_request();
    request.attacks[0].accuracy_percent = 0;
    request.attacks[0].collision_balance_hundredths = 20_000;
    request.attacks[0].balance_power_hundredths = 0;
    request
}

/// Stacks all five cue rules onto "e" in a single tick:
/// - `counter` (actor_id "e") gives "e" the `Attack` cue.
/// - `lethal` (actor_id "a", guaranteed hit, lethal damage, huge collision-balance)
///   gives "e" `Hit`, `Incapacitated`, and `BalanceBroken` all at once.
/// - `miss` (actor_id "a", guaranteed miss, still in range) gives "e" `Evade`.
fn all_cues_request() -> CombatResolutionRequest {
    let mut request = resolution_request();
    request.attacks[0].id = "lethal".into();
    request.attacks[0].power_hundredths = 48_000;
    request.attacks[0].accuracy_percent = 100;
    request.attacks[0].balance_power_hundredths = 0;
    request.attacks[0].collision_balance_hundredths = 20_000;

    let mut miss = request.attacks[0].clone();
    miss.id = "miss".into();
    miss.accuracy_percent = 0;
    miss.power_hundredths = 0;
    miss.collision_balance_hundredths = 0;
    miss.balance_power_hundredths = 0;
    request.attacks.push(miss);

    request.attacks.push(CombatAttackDefinition {
        id: "counter".into(),
        actor_id: "e".into(),
        power_hundredths: 100,
        ability_multiplier_hundredths: 100,
        accuracy_percent: 100,
        attack_range: 2,
        penetration_hundredths: 0,
        collision_balance_hundredths: 0,
        balance_power_hundredths: 0,
        effects: vec![],
    });
    request
}

fn spectator_request_for(resolution: CombatResolutionResult) -> CombatSpectatorRequest {
    CombatSpectatorRequest {
        resolution,
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    }
}

#[test]
fn incapacitated_cue_marks_a_combatant_whose_health_snapshot_hit_zero() {
    let resolution = resolve_combat(incapacitated_only_request()).unwrap();
    let view = spectate_combat(&spectator_request_for(resolution)).unwrap();
    let piece = view.frames[0].pieces.iter().find(|p| p.id == "e").unwrap();
    assert!(piece.cues.contains(&CombatSpectatorCue::Incapacitated));
    assert!(!piece.cues.contains(&CombatSpectatorCue::BalanceBroken));
}

#[test]
fn balance_broken_cue_marks_a_combatant_whose_balance_snapshot_hit_zero() {
    let resolution = resolve_combat(balance_broken_only_request()).unwrap();
    let view = spectate_combat(&spectator_request_for(resolution)).unwrap();
    let piece = view.frames[0].pieces.iter().find(|p| p.id == "e").unwrap();
    assert!(piece.cues.contains(&CombatSpectatorCue::BalanceBroken));
    assert!(!piece.cues.contains(&CombatSpectatorCue::Incapacitated));
}

#[test]
fn neither_state_cue_applies_when_health_and_balance_stay_above_zero() {
    let view = spectate_combat(&spectator_request()).unwrap();
    for piece in &view.frames[0].pieces {
        assert!(!piece.cues.contains(&CombatSpectatorCue::Incapacitated));
        assert!(!piece.cues.contains(&CombatSpectatorCue::BalanceBroken));
    }
}

#[test]
fn cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated() {
    let resolution = resolve_combat(all_cues_request()).unwrap();
    let view = spectate_combat(&spectator_request_for(resolution)).unwrap();
    let piece = view.frames[0].pieces.iter().find(|p| p.id == "e").unwrap();
    assert_eq!(
        piece.cues,
        vec![
            CombatSpectatorCue::Attack,
            CombatSpectatorCue::Hit,
            CombatSpectatorCue::Evade,
            CombatSpectatorCue::BalanceBroken,
            CombatSpectatorCue::Incapacitated,
        ]
    );
}

/// 조기 결착(`fable_combat_early_conclusion_step1_2608022130.md` I2) 뒤에는
/// `execution.frames`가 tick 상한까지 남아 있지만 `resolution.frames`는 결착
/// tick에서 끊긴다. 관전 화면은 판정된 범위만 보여야 한다 — 그러지 않으면
/// 결착 뒤에도 말이 계속 움직이고 보고서의 `decisive_tick`과 어긋난다.
#[test]
fn spectator_view_never_extends_past_the_last_resolved_tick() {
    // 여러 tick을 돌려야 이 규칙이 드러난다 — 1 tick 픽스처에서는 어떤 구현이든
    // 통과한다.
    let mut request = two_way_resolution_request();
    request.execution.ticks = 5;
    request.execution.input.config.max_ticks = 5;
    let resolution = resolve_combat(request).unwrap();
    assert_eq!(
        resolution.frames.len(),
        5,
        "fixture sanity: no terminal condition here, so all 5 ticks resolve"
    );

    // execution 쪽 프레임은 5개로 남겨 둔 채 resolution만 첫 tick으로 줄인다 —
    // 관전 화면이 어느 쪽을 시간 범위의 기준으로 삼는지 드러낸다.
    let first_tick = resolution.frames[0].tick;
    let mut truncated = resolution.clone();
    truncated.frames.retain(|frame| frame.tick == first_tick);
    assert_eq!(
        truncated.execution.frames.len(),
        5,
        "the execution pass must still carry every tick, or the test proves nothing"
    );

    let view = spectate_combat(&CombatSpectatorRequest {
        resolution: truncated,
        participants: participants(),
        catalog: CombatEffectCatalog { effects: vec![] },
    })
    .unwrap();

    assert_eq!(
        view.frames.len(),
        1,
        "the view must stop at the last resolved tick, not follow the execution pass"
    );
    assert!(
        view.frames.iter().all(|f| f.tick <= first_tick),
        "no spectator frame may exist after the last resolved tick"
    );
    assert!(
        view.full_log.iter().all(|e| e.tick <= first_tick),
        "no spectator log entry may exist after the last resolved tick"
    );
}

#[test]
fn empty_combatant_snapshot_yields_no_state_cues_and_no_error() {
    let mut resolution = resolve_combat(all_cues_request()).unwrap();
    for frame in &mut resolution.frames {
        frame.combatants = Vec::new();
    }
    let view = spectate_combat(&spectator_request_for(resolution)).unwrap();
    for piece in &view.frames[0].pieces {
        assert!(!piece.cues.contains(&CombatSpectatorCue::Incapacitated));
        assert!(!piece.cues.contains(&CombatSpectatorCue::BalanceBroken));
    }
}
