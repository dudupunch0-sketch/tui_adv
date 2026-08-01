use escape_core::*;
use std::collections::BTreeMap;

fn participant(id: &str, side: CombatSide, active: bool) -> CombatSimulationParticipant {
    CombatSimulationParticipant {
        id: id.into(),
        side,
        position: CombatPosition { x: 0, y: 0 },
        facing: CombatFacing { x: 1, y: 0 },
        speed_per_tick: 1,
        collision_radius: 1,
        attack_range: 1,
        support_range: 1,
        role_id: "r".into(),
        target_policy_id: None,
        active,
    }
}
fn resolution(ally: i64, enemy: i64, tick: u32) -> CombatResolutionResult {
    let state = CombatResolutionState {
        combatants: vec![
            CombatResolutionCombatant {
                id: "a".into(),
                current_health_hundredths: ally,
                maximum_health_hundredths: 100,
                balance_hundredths: 0,
                maximum_balance_hundredths: 100,
            },
            CombatResolutionCombatant {
                id: "e".into(),
                current_health_hundredths: enemy,
                maximum_health_hundredths: 100,
                balance_hundredths: 0,
                maximum_balance_hundredths: 100,
            },
        ],
        active_effects: vec![],
        applied_effect_ids: vec![],
        suppressed_effect_ids: vec![],
    };
    CombatResolutionResult {
        execution: CombatExecutionResult {
            mode: CombatRunMode::Actual,
            presentation: CombatPresentationSpeed::OneX,
            effective_seed: 1,
            namespace: CombatRngNamespace::ActualCombat,
            frames: vec![],
            full_log: vec![],
            core_log: vec![],
            provenance: None,
            fingerprint: "res".into(),
        },
        frames: vec![CombatResolutionFrame {
            tick,
            outcomes: vec![],
            combatants: vec![],
            fingerprint: "f".into(),
        }],
        state,
        full_log: vec![],
        core_log: vec![],
        fingerprint: "res".into(),
    }
}
fn attack_outcome(actor: &str, target: &str, hit: bool, damage: i64) -> CombatAttackOutcome {
    CombatAttackOutcome {
        attack_id: format!("{actor}->{target}"),
        actor_id: actor.into(),
        target_id: target.into(),
        collision: true,
        in_range: true,
        roll_percent: 1,
        hit,
        damage_hundredths: damage,
        balance_delta_hundredths: 0,
        applied_effect_ids: vec![],
        suppressed_effect_ids: vec![],
    }
}
fn health_snapshot(id: &str, hp: i64) -> CombatResolutionCombatant {
    CombatResolutionCombatant {
        id: id.into(),
        current_health_hundredths: hp,
        maximum_health_hundredths: 100,
        balance_hundredths: 0,
        maximum_balance_hundredths: 100,
    }
}
fn frame(
    tick: u32,
    outcomes: Vec<CombatAttackOutcome>,
    combatants: Vec<CombatResolutionCombatant>,
) -> CombatResolutionFrame {
    CombatResolutionFrame {
        tick,
        outcomes,
        combatants,
        fingerprint: format!("f{tick}"),
    }
}
fn multi_resolution(
    frames: Vec<CombatResolutionFrame>,
    final_state: Vec<CombatResolutionCombatant>,
) -> CombatResolutionResult {
    CombatResolutionResult {
        execution: CombatExecutionResult {
            mode: CombatRunMode::Actual,
            presentation: CombatPresentationSpeed::OneX,
            effective_seed: 1,
            namespace: CombatRngNamespace::ActualCombat,
            frames: vec![],
            full_log: vec![],
            core_log: vec![],
            provenance: None,
            fingerprint: "res".into(),
        },
        frames,
        state: CombatResolutionState {
            combatants: final_state,
            active_effects: vec![],
            applied_effect_ids: vec![],
            suppressed_effect_ids: vec![],
        },
        full_log: vec![],
        core_log: vec![],
        fingerprint: "res".into(),
    }
}
fn eval(a: i64, e: i64, tick: u32, close: bool) -> CombatConclusionReport {
    conclude_combat(CombatConclusionRequest {
        resolution: resolution(a, e, tick),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 3,
            conclude_on_max_ticks: close,
        },
        tick_millis: 100,
    })
    .unwrap()
}

#[test]
fn all_outcomes_and_mutual_precedence_are_stable() {
    assert_eq!(
        eval(10, 0, 1, false).outcome,
        CombatConclusionOutcome::AllyVictory
    );
    assert_eq!(
        eval(0, 10, 1, false).outcome,
        CombatConclusionOutcome::EnemyVictory
    );
    assert_eq!(
        eval(0, 0, 1, false).reason,
        CombatConclusionReason::BothSidesDefeated
    );
    assert_eq!(
        eval(10, 10, 3, true).outcome,
        CombatConclusionOutcome::Stalemate
    );
    assert_eq!(
        eval(10, 10, 1, false).outcome,
        CombatConclusionOutcome::InProgress
    );
}

#[test]
fn permutation_and_invalid_inputs_are_deterministic() {
    let mut r = CombatConclusionRequest {
        resolution: resolution(10, 0, 1),
        participants: vec![
            participant("e", CombatSide::Enemy, true),
            participant("a", CombatSide::Ally, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 3,
            conclude_on_max_ticks: false,
        },
        tick_millis: 100,
    };
    let a = conclude_combat(r.clone()).unwrap();
    r.participants.reverse();
    assert_eq!(a, conclude_combat(r).unwrap());
    let bad = CombatConclusionRequest {
        resolution: resolution(10, 0, 1),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("a", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 3,
            conclude_on_max_ticks: false,
        },
        tick_millis: 100,
    };
    assert!(matches!(
        conclude_combat(bad),
        Err(CombatConclusionError::DuplicateParticipant(_))
    ));
}

#[test]
fn cleanup_is_split_without_persistent_promotion() {
    let mut r = CombatConclusionRequest {
        resolution: resolution(10, 0, 1),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 3,
            conclude_on_max_ticks: false,
        },
        tick_millis: 100,
    };
    r.resolution.state.active_effects = vec![
        CombatEffectInstance {
            definition_id: "combat_buff".into(),
            source: "skill".into(),
            combat_only: true,
            target_selector: "a".into(),
            parameters: BTreeMap::new(),
            phase: EffectPhase::DuringCombat,
            lifetime: EffectLifetime::UntilCombatSettlement,
            stacking_group: "buff".into(),
        },
        CombatEffectInstance {
            definition_id: "persistent_status".into(),
            source: "world".into(),
            combat_only: false,
            target_selector: "a".into(),
            parameters: BTreeMap::new(),
            phase: EffectPhase::CombatSettlement,
            lifetime: EffectLifetime::Persistent,
            stacking_group: "status".into(),
        },
    ];
    let report = conclude_combat(r).unwrap();
    assert_eq!(report.removed_combat_effect_ids, vec!["combat_buff"]);
    assert_eq!(report.retained_effect_ids, vec!["persistent_status"]);
}

#[test]
fn policy_and_active_side_validation_are_explicit() {
    let mut r = CombatConclusionRequest {
        resolution: resolution(10, 10, 1),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 0,
            conclude_on_max_ticks: true,
        },
        tick_millis: 100,
    };
    assert!(matches!(
        conclude_combat(r.clone()),
        Err(CombatConclusionError::InvalidPolicy)
    ));
    r.policy.max_ticks = 1;
    r.resolution.frames[0].tick = 2;
    assert!(matches!(
        conclude_combat(r),
        Err(CombatConclusionError::FrameExceedsPolicy)
    ));
    let empty_enemy = CombatConclusionRequest {
        resolution: resolution(10, 10, 1),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, false),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 1,
            conclude_on_max_ticks: false,
        },
        tick_millis: 100,
    };
    assert!(matches!(
        conclude_combat(empty_enemy),
        Err(CombatConclusionError::EmptyActiveSide)
    ));
}

#[test]
fn zero_tick_millis_is_rejected() {
    let r = CombatConclusionRequest {
        resolution: resolution(10, 0, 1),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 3,
            conclude_on_max_ticks: false,
        },
        tick_millis: 0,
    };
    assert!(matches!(
        conclude_combat(r),
        Err(CombatConclusionError::InvalidTickMillis(0))
    ));
}

#[test]
fn duration_millis_uses_decisive_tick_plus_one_when_terminal() {
    // eval(10, 0, 1, false) reaches AllyVictory at tick 1 with tick_millis = 100.
    let report = eval(10, 0, 1, false);
    assert_eq!(report.decisive_tick, Some(1));
    assert_eq!(report.duration_millis, (1 + 1) * 100);
}

#[test]
fn duration_millis_uses_frame_count_when_not_terminal() {
    // eval(10, 10, 1, false) stays InProgress: no decisive tick, one frame recorded.
    let report = eval(10, 10, 1, false);
    assert_eq!(report.decisive_tick, None);
    assert_eq!(report.duration_millis, 1 * 100);
}

#[test]
fn combatants_report_sums_damage_and_marks_incapacitated() {
    let frames = vec![
        frame(
            0,
            vec![
                attack_outcome("a", "e", true, 20),
                attack_outcome("e", "a", true, 15),
                attack_outcome("c", "e", false, 0),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 100),
                health_snapshot("e", 80),
            ],
        ),
        frame(
            1,
            vec![
                attack_outcome("e", "c", true, 10),
                attack_outcome("a", "e", true, 5),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 90),
                health_snapshot("e", 75),
            ],
        ),
    ];
    let final_state = vec![
        health_snapshot("a", 85),
        health_snapshot("c", 90),
        health_snapshot("e", 75),
    ];
    let request = CombatConclusionRequest {
        resolution: multi_resolution(frames, final_state),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("c", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 5,
            conclude_on_max_ticks: false,
        },
        tick_millis: 50,
    };
    let report = conclude_combat(request).unwrap();
    assert_eq!(
        report
            .combatants
            .iter()
            .map(|c| c.id.clone())
            .collect::<Vec<_>>(),
        vec!["a".to_string(), "c".to_string(), "e".to_string()]
    );
    let by_id: BTreeMap<_, _> = report
        .combatants
        .iter()
        .map(|c| (c.id.clone(), c))
        .collect();
    assert_eq!(by_id["a"].damage_dealt_hundredths, 25);
    assert_eq!(by_id["a"].damage_taken_hundredths, 15);
    assert_eq!(by_id["a"].kills, 0);
    assert!(!by_id["a"].incapacitated);
    assert_eq!(by_id["c"].damage_dealt_hundredths, 0);
    assert_eq!(by_id["c"].damage_taken_hundredths, 10);
    assert_eq!(by_id["c"].kills, 0);
    assert!(!by_id["c"].incapacitated);
    assert_eq!(by_id["e"].damage_dealt_hundredths, 25);
    assert_eq!(by_id["e"].damage_taken_hundredths, 25);
    assert_eq!(by_id["e"].kills, 0);
    assert!(!by_id["e"].incapacitated);
}

#[test]
fn kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick() {
    // "d" starts already at 0 with no lethal outcome in that tick: nobody is credited.
    // "e" first reaches <= 0 at tick 1, where two hits land; the LAST one ("a") gets the kill,
    // not "c" even though c also damaged e in the same tick.
    let frames = vec![
        frame(
            0,
            vec![attack_outcome("a", "e", true, 30)],
            vec![
                health_snapshot("a", 100),
                health_snapshot("c", 100),
                health_snapshot("d", 0),
                health_snapshot("e", 70),
            ],
        ),
        frame(
            1,
            vec![
                attack_outcome("c", "e", true, 40),
                attack_outcome("a", "e", true, 35),
            ],
            vec![
                health_snapshot("a", 100),
                health_snapshot("c", 100),
                health_snapshot("d", 0),
                health_snapshot("e", -5),
            ],
        ),
    ];
    let final_state = vec![
        health_snapshot("a", 100),
        health_snapshot("c", 100),
        health_snapshot("d", 0),
        health_snapshot("e", -5),
    ];
    let request = CombatConclusionRequest {
        resolution: multi_resolution(frames, final_state),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("c", CombatSide::Ally, true),
            participant("d", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 5,
            conclude_on_max_ticks: false,
        },
        tick_millis: 50,
    };
    let report = conclude_combat(request).unwrap();
    let by_id: BTreeMap<_, _> = report
        .combatants
        .iter()
        .map(|c| (c.id.clone(), c))
        .collect();
    assert_eq!(by_id["a"].kills, 1);
    assert_eq!(by_id["c"].kills, 0);
    assert_eq!(by_id["d"].kills, 0);
    assert_eq!(by_id["e"].kills, 0);
    assert!(by_id["d"].incapacitated);
    assert!(by_id["e"].incapacitated);
    assert!(!by_id["a"].incapacitated);
    assert!(!by_id["c"].incapacitated);
}

#[test]
fn top_damage_highlights_hidden_when_no_damage_occurs() {
    let frames = vec![frame(
        0,
        vec![attack_outcome("a", "e", false, 0)],
        vec![health_snapshot("a", 100), health_snapshot("e", 100)],
    )];
    let final_state = vec![health_snapshot("a", 100), health_snapshot("e", 100)];
    let request = CombatConclusionRequest {
        resolution: multi_resolution(frames, final_state),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 5,
            conclude_on_max_ticks: false,
        },
        tick_millis: 50,
    };
    let report = conclude_combat(request).unwrap();
    assert_eq!(report.top_damage_dealt_id, None);
    assert_eq!(report.top_damage_taken_id, None);
}

#[test]
fn top_damage_highlights_pick_max_with_lowest_id_tie_break() {
    let frames = vec![
        frame(
            0,
            vec![
                attack_outcome("a", "e", true, 20),
                attack_outcome("e", "a", true, 15),
                attack_outcome("c", "e", false, 0),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 100),
                health_snapshot("e", 80),
            ],
        ),
        frame(
            1,
            vec![
                attack_outcome("e", "c", true, 10),
                attack_outcome("a", "e", true, 5),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 90),
                health_snapshot("e", 75),
            ],
        ),
    ];
    let final_state = vec![
        health_snapshot("a", 85),
        health_snapshot("c", 90),
        health_snapshot("e", 75),
    ];
    let request = CombatConclusionRequest {
        resolution: multi_resolution(frames, final_state),
        participants: vec![
            participant("a", CombatSide::Ally, true),
            participant("c", CombatSide::Ally, true),
            participant("e", CombatSide::Enemy, true),
        ],
        policy: CombatTerminationPolicy {
            max_ticks: 5,
            conclude_on_max_ticks: false,
        },
        tick_millis: 50,
    };
    let report = conclude_combat(request).unwrap();
    // damage_dealt: a=25, c=0, e=25 -> tie, lowest id "a" wins.
    assert_eq!(report.top_damage_dealt_id, Some("a".to_string()));
    // damage_taken: a=15, c=10, e=25 -> unique max "e".
    assert_eq!(report.top_damage_taken_id, Some("e".to_string()));
}

fn sample_multi_participants() -> Vec<CombatSimulationParticipant> {
    vec![
        participant("a", CombatSide::Ally, true),
        participant("c", CombatSide::Ally, true),
        participant("e", CombatSide::Enemy, true),
    ]
}
fn sample_multi_resolution() -> CombatResolutionResult {
    let frames = vec![
        frame(
            0,
            vec![
                attack_outcome("a", "e", true, 20),
                attack_outcome("e", "a", true, 15),
                attack_outcome("c", "e", false, 0),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 100),
                health_snapshot("e", 80),
            ],
        ),
        frame(
            1,
            vec![
                attack_outcome("e", "c", true, 10),
                attack_outcome("a", "e", true, 5),
            ],
            vec![
                health_snapshot("a", 85),
                health_snapshot("c", 90),
                health_snapshot("e", 75),
            ],
        ),
    ];
    let final_state = vec![
        health_snapshot("a", 85),
        health_snapshot("c", 90),
        health_snapshot("e", 75),
    ];
    multi_resolution(frames, final_state)
}
fn sample_multi_request(participants: Vec<CombatSimulationParticipant>) -> CombatConclusionRequest {
    CombatConclusionRequest {
        resolution: sample_multi_resolution(),
        participants,
        policy: CombatTerminationPolicy {
            max_ticks: 5,
            conclude_on_max_ticks: false,
        },
        tick_millis: 50,
    }
}

#[test]
fn same_input_conclude_twice_yields_identical_report_and_fingerprint() {
    let request = sample_multi_request(sample_multi_participants());
    let a = conclude_combat(request.clone()).unwrap();
    let b = conclude_combat(request).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.fingerprint, b.fingerprint);
}

#[test]
fn shuffled_participant_order_yields_identical_report() {
    let baseline = conclude_combat(sample_multi_request(sample_multi_participants())).unwrap();
    let mut shuffled_participants = sample_multi_participants();
    shuffled_participants.reverse();
    let shuffled = conclude_combat(sample_multi_request(shuffled_participants)).unwrap();
    assert_eq!(baseline, shuffled);
}

#[test]
fn deserializing_report_json_without_new_fields_uses_defaults() {
    let json = r#"{
        "resolution_fingerprint": "r",
        "outcome": "ally_victory",
        "reason": "all_enemies_defeated",
        "decisive_tick": 1,
        "active_allies": 1,
        "active_enemies": 1,
        "survivor_ids": ["a"],
        "defeated_ids": ["e"],
        "removed_combat_effect_ids": [],
        "retained_effect_ids": [],
        "fingerprint": "fp"
    }"#;
    let report: CombatConclusionReport = serde_json::from_str(json).unwrap();
    assert_eq!(report.duration_millis, 0);
    assert!(report.combatants.is_empty());
    assert_eq!(report.top_damage_dealt_id, None);
    assert_eq!(report.top_damage_taken_id, None);
}
