use escape_core::*;
use std::collections::BTreeMap;

fn manifest() -> CombatManifest {
    CombatManifest {
        simulation_version: CombatSimulationVersion::new("wave1").unwrap(),
        actual_seed: 7,
        world_state_fingerprint: "world:7".into(),
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
fn combatant() -> CombatantState {
    CombatantState {
        id: "player".into(),
        current_health: 10,
        maximum_health: 10,
        current_breath: 10,
        maximum_breath: 10,
        balance: 10,
        maximum_balance: 10,
        fear: 0,
        anger: 0,
        posture: Posture::Neutral,
        weapon_control: WeaponControl::Stable,
        relationship_refs: vec!["ally".into()],
        environment_refs: vec!["market".into()],
        team_refs: vec!["solo".into()],
        persistent_status_ids: vec!["tired".into()],
        combat_effect_ids: vec![],
    }
}
fn effect(
    id: &str,
    phase: EffectPhase,
    priority: i32,
    lifetime: EffectLifetime,
) -> CombatEffectDefinition {
    CombatEffectDefinition {
        id: id.into(),
        source: "test".into(),
        category: CombatEffectCategory::State,
        target_selector: "player".into(),
        parameters: BTreeMap::new(),
        conditions: vec![],
        phase,
        lifetime,
        stacking: EffectStacking::Unique,
        stacking_group: id.into(),
        stacking_cap: None,
        priority,
        visibility: EffectVisibility::Public,
        tags: vec![],
    }
}

#[test]
fn catalog_validates_and_canonicalizes_phase_priority_id() {
    let catalog = CombatEffectCatalog {
        effects: vec![
            effect(
                "z",
                EffectPhase::DuringPlacement,
                1,
                EffectLifetime::UntilCombatSettlement,
            ),
            effect(
                "a",
                EffectPhase::BeforePlacement,
                9,
                EffectLifetime::Persistent,
            ),
            effect(
                "b",
                EffectPhase::DuringPlacement,
                1,
                EffectLifetime::UntilCombatSettlement,
            ),
        ],
    };
    let canonical = catalog.canonical().unwrap();
    assert_eq!(
        canonical.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(),
        vec!["a", "b", "z"]
    );
    assert!(catalog.canonical_json().unwrap().contains("\"id\":\"a\""));
}

#[test]
fn projection_applies_candidates_and_preserves_suppression_reasons() {
    let mut conditional = effect(
        "conditional",
        EffectPhase::DuringCombat,
        0,
        EffectLifetime::UntilCombatSettlement,
    );
    conditional.conditions = vec!["opening_visible".into()];
    let catalog = CombatEffectCatalog {
        effects: vec![
            effect(
                "persistent",
                EffectPhase::BeforePlacement,
                0,
                EffectLifetime::Persistent,
            ),
            effect(
                "combat_only",
                EffectPhase::DuringCombat,
                0,
                EffectLifetime::UntilCombatSettlement,
            ),
            conditional,
        ],
    };
    let input = CombatPreCombatInput {
        manifest: manifest(),
        battle_id: "battle:1".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![PersistentCombatStatus {
            id: "tired".into(),
            source: "world".into(),
        }],
        candidate_effect_ids: vec![
            "persistent".into(),
            "combat_only".into(),
            "conditional".into(),
        ],
        active_condition_ids: vec![],
        environment_refs: vec!["market".into()],
        team_refs: vec!["solo".into()],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
    };
    let projection = CombatInitialStateProjection::project(&input, &catalog).unwrap();
    assert_eq!(projection.applied_effects.len(), 2);
    assert!(projection
        .suppressed_effects
        .iter()
        .any(|e| e.id == "conditional" && e.reason == "condition_not_met"));
    assert_eq!(projection.manifest.applied_effects.len(), 2);
    assert_eq!(projection.manifest.team_ids, vec!["solo"]);
}

#[test]
fn conclusion_removes_combat_only_effects_without_promoting_them() {
    let state = CombatState {
        battle_id: "battle:1".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![PersistentCombatStatus {
            id: "tired".into(),
            source: "world".into(),
        }],
        active_effects: vec![
            CombatEffectInstance {
                definition_id: "temporary".into(),
                source: "test".into(),
                combat_only: true,
                target_selector: "player".into(),
                parameters: BTreeMap::new(),
                phase: EffectPhase::DuringCombat,
                lifetime: EffectLifetime::UntilCombatSettlement,
                stacking_group: "temporary".into(),
            },
            CombatEffectInstance {
                definition_id: "lasting".into(),
                source: "test".into(),
                combat_only: false,
                target_selector: "player".into(),
                parameters: BTreeMap::new(),
                phase: EffectPhase::CombatSettlement,
                lifetime: EffectLifetime::Persistent,
                stacking_group: "lasting".into(),
            },
        ],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
        manifest_fingerprint: "fingerprint".into(),
    };
    let conclusion = state.conclude().unwrap();
    assert_eq!(conclusion.removed_combat_effect_ids, vec!["temporary"]);
    assert_eq!(
        conclusion
            .retained_effects
            .iter()
            .map(|e| e.definition_id.as_str())
            .collect::<Vec<_>>(),
        vec!["lasting"]
    );
    assert_eq!(conclusion.persistent_statuses[0].id, "tired");
}

#[test]
fn invalid_health_duplicate_ids_and_zero_exchange_lifetime_fail() {
    let mut invalid = combatant();
    invalid.current_health = 11;
    let state = CombatState {
        battle_id: "battle".into(),
        combatants: vec![invalid],
        persistent_statuses: vec![],
        active_effects: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
        manifest_fingerprint: "fp".into(),
    };
    assert!(state.validate().is_err());
    let bad = CombatEffectCatalog {
        effects: vec![effect(
            "bad",
            EffectPhase::DuringCombat,
            0,
            EffectLifetime::Seconds(0),
        )],
    };
    assert!(bad.validate().is_err());
}

#[test]
fn state_and_catalog_serde_round_trip() {
    let state = CombatState {
        battle_id: "battle".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![],
        active_effects: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
        manifest_fingerprint: "fp".into(),
    };
    let decoded: CombatState =
        serde_json::from_str(&serde_json::to_string(&state).unwrap()).unwrap();
    assert_eq!(state, decoded);
}

#[test]
fn projection_rejects_unknown_candidates_and_exposes_stable_identity() {
    let catalog = CombatEffectCatalog {
        effects: vec![effect(
            "known",
            EffectPhase::CombatStart,
            0,
            EffectLifetime::Instant,
        )],
    };
    let mut input = CombatPreCombatInput {
        manifest: manifest(),
        battle_id: "battle".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![],
        candidate_effect_ids: vec!["unknown".into()],
        active_condition_ids: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
    };
    assert!(matches!(
        CombatInitialStateProjection::project(&input, &catalog),
        Err(CombatStateError::UnknownEffect(_))
    ));
    input.candidate_effect_ids = vec!["known".into()];
    let projection = CombatInitialStateProjection::project(&input, &catalog).unwrap();
    assert_eq!(
        projection.final_state_fingerprint,
        projection.state.fingerprint().unwrap()
    );
    assert_eq!(
        projection.fingerprint().unwrap(),
        projection.fingerprint().unwrap()
    );
    let mut reordered = input.clone();
    reordered.combatants[0].relationship_refs.reverse();
    reordered.combatants[0].environment_refs.reverse();
    reordered.combatants[0].team_refs.reverse();
    assert_eq!(
        projection.fingerprint(),
        CombatInitialStateProjection::project(&reordered, &catalog)
            .unwrap()
            .fingerprint()
    );
}

#[test]
fn projection_suppresses_unique_group_conflicts_deterministically() {
    let mut second = effect(
        "second",
        EffectPhase::DuringCombat,
        1,
        EffectLifetime::Instant,
    );
    second.stacking_group = "shared".into();
    let mut first = effect(
        "first",
        EffectPhase::DuringCombat,
        0,
        EffectLifetime::Instant,
    );
    first.stacking_group = "shared".into();
    let catalog = CombatEffectCatalog {
        effects: vec![second, first],
    };
    let input = CombatPreCombatInput {
        manifest: manifest(),
        battle_id: "battle".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![],
        candidate_effect_ids: vec!["second".into(), "first".into()],
        active_condition_ids: vec![],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
    };
    let projection = CombatInitialStateProjection::project(&input, &catalog).unwrap();
    assert_eq!(
        projection
            .applied_effects
            .iter()
            .map(|e| e.id.as_str())
            .collect::<Vec<_>>(),
        vec!["first"]
    );
    assert!(projection
        .suppressed_effects
        .iter()
        .any(|e| e.id == "second" && e.reason.contains("first")));
}

#[test]
fn state_fingerprint_ignores_combat_only_tie_order() {
    let mut state = CombatState {
        battle_id: "battle".into(),
        combatants: vec![combatant()],
        persistent_statuses: vec![],
        active_effects: vec![
            CombatEffectInstance {
                definition_id: "same".into(),
                source: "source".into(),
                combat_only: true,
                target_selector: "player".into(),
                parameters: BTreeMap::new(),
                phase: EffectPhase::DuringCombat,
                lifetime: EffectLifetime::Instant,
                stacking_group: "same".into(),
            },
            CombatEffectInstance {
                definition_id: "same".into(),
                source: "source".into(),
                combat_only: false,
                target_selector: "player".into(),
                parameters: BTreeMap::new(),
                phase: EffectPhase::DuringCombat,
                lifetime: EffectLifetime::Instant,
                stacking_group: "same".into(),
            },
        ],
        environment_refs: vec![],
        team_refs: vec![],
        team_formations: vec![],
        relationships: vec![],
        environment_states: vec![],
        manifest_fingerprint: "fp".into(),
    };
    let fingerprint = state.fingerprint().unwrap();
    state.active_effects.reverse();
    assert_eq!(fingerprint, state.fingerprint().unwrap());
}
