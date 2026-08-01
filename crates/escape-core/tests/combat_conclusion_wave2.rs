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
