//! Wave 3 Step 2a: `EncounterCombatDef` schema, index-time validation (WP-2),
//! and the systemic combat producer (WP-3), plus the additive-optional /
//! determinism regression suite (WP-4).
//!
//! Fixture policy: `crates/escape-core/fixtures/content/content.bundle.json`
//! is never modified. Every test loads it fresh with `load_content_bundle`
//! and injects a minimal combat definition into one existing encounter's
//! `serde_json::Value` before re-indexing, the same technique
//! `tests/event_stage.rs` already uses for event stages.

use escape_core::{
    index_content_bundle, load_content_bundle, new_game_from_content_at, scene_page_from_content,
    turn_view_from_content, CombatConclusionOutcome, CombatConclusionReason, ContentBundle,
    ContentIndexError, ContentTurnError,
};
use serde_json::{json, Value};

const BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");
const ENCOUNTER_ID: &str = "printer_prints_alone";
const LOCATION_ID: &str = "printer_area";

/// Wave 3 Step 2b: real authoring regression suite. Unlike the rest of this
/// file (which injects a synthetic combat definition into a small unrelated
/// fixture), these tests load the actual wuxia storypack-preview bundle and
/// exercise the one real systemic combat encounter authored in
/// `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
/// (`fable_combat_wave3_step2b_2608021228.md`).
const WUXIA_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");
const SPECTATOR_BOUT_ID: &str = "wuxia_combat_spectator_preview_bout";
const SPECTATOR_LOCATION_ID: &str = "cheongryu_outer_courtyard";
const SPECTATOR_GATE_FLAG: &str = "combat_spectator_preview_unlocked";

/// A minimal, internally-consistent systemic combat definition: two
/// combatants (one per side), one attack each, matching defenses, and a
/// one-effect catalog. `manifest.actual_seed` is intentionally an arbitrary
/// authoring placeholder -- the producer must never use it directly
/// (invariant 1 / WP-4 test 6).
fn valid_combat_json() -> Value {
    json!({
        "kind": "systemic",
        "intervention_budget": 1,
        "manifest": {
            // Kept as a literal: this manifest lives inside a JSON string blob
            // built with `json!()`, so it cannot reference the Rust constant
            // directly. Must match `escape_core::CURRENT_SIMULATION_VERSION`.
            "simulation_version": "v3",
            "actual_seed": 999,
            "world_state_fingerprint": "wsf-1",
            "applied_effects": [],
            "suppressed_effects": [],
            "combatant_ids": ["ally_1", "enemy_1"],
            "placement_ids": [],
            "environment_ids": [],
            "team_ids": [],
            "rule_ids": [],
            "public_info_ids": []
        },
        "state": {
            "battle_id": "battle_1",
            "combatants": [
                {
                    "id": "ally_1", "current_health": 100, "maximum_health": 100,
                    "current_breath": 50, "maximum_breath": 50,
                    "balance": 100, "maximum_balance": 100,
                    "fear": 0, "anger": 0,
                    "posture": "neutral", "weapon_control": "stable"
                },
                {
                    "id": "enemy_1", "current_health": 100, "maximum_health": 100,
                    "current_breath": 50, "maximum_breath": 50,
                    "balance": 100, "maximum_balance": 100,
                    "fear": 0, "anger": 0,
                    "posture": "neutral", "weapon_control": "stable"
                }
            ],
            "manifest_fingerprint": "state-fp-1"
        },
        "config": {"tick_millis": 100, "max_ticks": 5},
        "participants": [
            {
                "id": "ally_1", "side": "ally",
                "position": {"q": 0, "r": 0}, "facing": {"q": 1, "r": 0},
                "speed_per_tick": 1, "collision_radius": 5,
                "attack_range": 10, "support_range": 10,
                "role_id": "role_ally", "target_policy_id": null, "active": true
            },
            {
                "id": "enemy_1", "side": "enemy",
                "position": {"q": 5, "r": 0}, "facing": {"q": -1, "r": 0},
                "speed_per_tick": 1, "collision_radius": 5,
                "attack_range": 10, "support_range": 10,
                "role_id": "role_enemy", "target_policy_id": null, "active": true
            }
        ],
        "roles": [
            {
                "id": "role_ally",
                "weights": {
                    "preferred_distance": 0, "aggression": 1, "formation_maintenance": 0,
                    "pursuit_range": 10, "protect_priority": 0, "target_priority": 0,
                    "risk_tolerance": 0, "ability_priority": 0
                }
            },
            {
                "id": "role_enemy",
                "weights": {
                    "preferred_distance": 0, "aggression": 1, "formation_maintenance": 0,
                    "pursuit_range": 10, "protect_priority": 0, "target_priority": 0,
                    "risk_tolerance": 0, "ability_priority": 0
                }
            }
        ],
        "policies": [],
        "attacks": [
            {
                "id": "atk_ally", "actor_id": "ally_1",
                "power_hundredths": 1000, "ability_multiplier_hundredths": 100,
                "accuracy_percent": 100, "attack_range": 10,
                "penetration_hundredths": 0, "collision_balance_hundredths": 0,
                "balance_power_hundredths": 0,
                "effects": [{"effect_id": "burn", "chance_percent": 100}]
            },
            {
                "id": "atk_enemy", "actor_id": "enemy_1",
                "power_hundredths": 800, "ability_multiplier_hundredths": 100,
                "accuracy_percent": 100, "attack_range": 10,
                "penetration_hundredths": 0, "collision_balance_hundredths": 0,
                "balance_power_hundredths": 0,
                "effects": []
            }
        ],
        "defenses": [
            {"combatant_id": "ally_1", "defense_hundredths": 0, "balance_resistance_hundredths": 0},
            {"combatant_id": "enemy_1", "defense_hundredths": 0, "balance_resistance_hundredths": 0}
        ],
        "effect_catalog": {
            "effects": [
                {
                    "id": "burn", "source": "atk_ally", "category": "state",
                    "target_selector": "target", "parameters": {}, "conditions": [],
                    "phase": "during_combat", "lifetime": "Persistent",
                    "stacking": "unique", "stacking_group": "burn_group",
                    "stacking_cap": null, "priority": 0, "visibility": "public", "tags": []
                }
            ]
        },
        "ticks": 3,
        "termination": {"max_ticks": 3, "conclude_on_max_ticks": true}
    })
}

/// Loads the shared fixture fresh and injects a combat definition (built from
/// `valid_combat_json()` and then `mutate`d) into `ENCOUNTER_ID`. The fixture
/// file on disk is never written to.
fn bundle_with_combat(mutate: impl FnOnce(&mut Value)) -> ContentBundle {
    let mut bundle = load_content_bundle(BUNDLE).expect("fixture bundle should load");
    let mut combat = valid_combat_json();
    mutate(&mut combat);
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == ENCOUNTER_ID)
        .expect("fixture must contain the target encounter");
    encounter["combat"] = combat;
    bundle
}

fn expect_combat_error(bundle: &ContentBundle) -> ContentIndexError {
    match index_content_bundle(bundle) {
        Ok(_) => panic!("expected combat validation to reject this bundle"),
        Err(error) => error,
    }
}

// ---------------------------------------------------------------------
// WP-2: index-time validation (11 hard-error rules, 정본 12).
// ---------------------------------------------------------------------

#[test]
fn valid_systemic_combat_indexes_without_error() {
    let bundle = bundle_with_combat(|_| {});
    index_content_bundle(&bundle).expect("a well-formed systemic combat should index cleanly");
}

#[test]
fn rule1_intervention_budget_over_three_is_rejected() {
    let bundle = bundle_with_combat(|combat| combat["intervention_budget"] = json!(4));
    let error = expect_combat_error(&bundle);
    assert!(matches!(
        error,
        ContentIndexError::InvalidEncounterCombat { .. }
    ));
    assert!(error.to_string().contains(ENCOUNTER_ID));
}

#[test]
fn rule2_mixed_kind_is_rejected_and_names_the_encounter() {
    let bundle = bundle_with_combat(|combat| combat["kind"] = json!("mixed"));
    let error = expect_combat_error(&bundle);
    let message = error.to_string();
    assert!(message.contains(ENCOUNTER_ID));
    assert!(message.contains("2b/2c"));
}

#[test]
fn rule2_scripted_kind_is_rejected_and_names_the_encounter() {
    let bundle = bundle_with_combat(|combat| combat["kind"] = json!("scripted"));
    let error = expect_combat_error(&bundle);
    let message = error.to_string();
    assert!(message.contains(ENCOUNTER_ID));
    assert!(message.contains("2b/2c"));
}

#[test]
fn rule3_zero_tick_millis_is_rejected() {
    let bundle = bundle_with_combat(|combat| combat["config"]["tick_millis"] = json!(0));
    expect_combat_error(&bundle);
}

#[test]
fn rule4_zero_ticks_is_rejected() {
    let bundle = bundle_with_combat(|combat| combat["ticks"] = json!(0));
    expect_combat_error(&bundle);
}

#[test]
fn rule4_ticks_exceeding_max_ticks_is_rejected() {
    let bundle = bundle_with_combat(|combat| combat["ticks"] = json!(999));
    expect_combat_error(&bundle);
}

#[test]
fn rule5_attack_actor_id_not_in_combatants_is_rejected() {
    let bundle = bundle_with_combat(|combat| combat["attacks"][0]["actor_id"] = json!("ghost"));
    expect_combat_error(&bundle);
}

#[test]
fn rule6_defense_combatant_id_not_in_combatants_is_rejected() {
    let bundle =
        bundle_with_combat(|combat| combat["defenses"][0]["combatant_id"] = json!("ghost"));
    expect_combat_error(&bundle);
}

#[test]
fn rule7_combatant_missing_a_defense_profile_is_rejected() {
    let bundle = bundle_with_combat(|combat| {
        combat["defenses"] = json!([
            {"combatant_id": "ally_1", "defense_hundredths": 0, "balance_resistance_hundredths": 0}
        ]);
    });
    expect_combat_error(&bundle);
}

#[test]
fn rule8_participant_id_set_mismatch_is_rejected() {
    let bundle = bundle_with_combat(|combat| {
        combat["participants"][0]["id"] = json!("someone_else");
    });
    expect_combat_error(&bundle);
}

#[test]
fn rule9_invalid_effect_catalog_is_rejected() {
    let bundle = bundle_with_combat(|combat| {
        // Empty effect id fails `CombatEffectCatalog::validate`'s `ensure_id`.
        combat["effect_catalog"]["effects"][0]["id"] = json!("");
    });
    expect_combat_error(&bundle);
}

#[test]
fn rule10_invalid_manifest_is_rejected() {
    let bundle = bundle_with_combat(|combat| {
        combat["manifest"]["world_state_fingerprint"] = json!("");
    });
    expect_combat_error(&bundle);
}

#[test]
fn rule11_attack_references_unknown_effect_id_is_rejected() {
    let bundle = bundle_with_combat(|combat| {
        combat["attacks"][0]["effects"] =
            json!([{"effect_id": "nonexistent", "chance_percent": 100}]);
    });
    expect_combat_error(&bundle);
}

/// T0 rule 12: an encounter declaring a `simulation_version` this build
/// doesn't implement is a hard index-time error, named with the encounter id
/// like every other rule in this function.
#[test]
fn unsupported_simulation_version_is_rejected_at_index_time() {
    let bundle =
        bundle_with_combat(|combat| combat["manifest"]["simulation_version"] = json!("v9"));
    let error = expect_combat_error(&bundle);
    let message = error.to_string();
    assert!(message.contains(ENCOUNTER_ID));
    assert!(message.contains("v9"));
    assert!(message.contains("v3"));
}

/// T1-b1 WP6 (§4-4): `v2` was the current version before this slice's bump
/// and is a well-formed version string, not a typo like `v9` above -- this
/// is what actually proves T0's enforcement catches a missed bump, rather
/// than just proving it catches nonsense input.
#[test]
fn v2_authoring_is_rejected_after_the_bump() {
    let bundle =
        bundle_with_combat(|combat| combat["manifest"]["simulation_version"] = json!("v2"));
    let error = expect_combat_error(&bundle);
    let message = error.to_string();
    assert!(message.contains(ENCOUNTER_ID));
    assert!(message.contains("v2"));
    assert!(message.contains("v3"));
}

// ---------------------------------------------------------------------
// Additive-optional proof (also WP-4 minimal case 13).
// ---------------------------------------------------------------------

#[test]
fn bundle_without_any_combat_field_still_indexes() {
    let bundle = load_content_bundle(BUNDLE).expect("fixture bundle should load");
    index_content_bundle(&bundle).expect("bundles with no combat authoring must still index");
}

// ---------------------------------------------------------------------
// WP-3/WP-4: systemic combat producer.
// ---------------------------------------------------------------------

/// WP-4 case 2 & 3: a systemic combat producer fills `ScenePage.combat` with a
/// non-empty spectator view and a conclusion report.
#[test]
fn systemic_combat_producer_fills_scene_page_combat() {
    let bundle = bundle_with_combat(|_| {});
    let index = index_content_bundle(&bundle).expect("bundle should index");
    let state =
        new_game_from_content_at(7, &index, LOCATION_ID).expect("content-backed game should start");
    let page = scene_page_from_content(&state, &index).expect("scene page should render");

    let combat = page
        .combat
        .expect("systemic combat should fill ScenePage.combat");
    assert!(
        !combat.view.frames.is_empty(),
        "spectator view should have at least one tick frame"
    );
    let report = combat
        .report
        .expect("concluded combat should carry a report");
    assert!(report.duration_millis > 0);
    assert_eq!(report.combatants.len(), 2);
}

/// WP-4 case 4: determinism -- same state + same bundle -> identical
/// `ScenePage.combat` on repeated calls.
#[test]
fn systemic_combat_producer_is_deterministic_for_the_same_state() {
    let bundle = bundle_with_combat(|_| {});
    let index = index_content_bundle(&bundle).expect("bundle should index");
    let state = new_game_from_content_at(42, &index, LOCATION_ID).expect("game should start");

    let first = scene_page_from_content(&state, &index).expect("first render should succeed");
    let second = scene_page_from_content(&state, &index).expect("second render should succeed");
    assert_eq!(first.combat, second.combat);
}

/// WP-4 case 5: a different run seed produces a different combat fingerprint.
#[test]
fn systemic_combat_producer_seed_changes_with_run_seed() {
    let bundle = bundle_with_combat(|_| {});
    let index = index_content_bundle(&bundle).expect("bundle should index");

    let state_a = new_game_from_content_at(1, &index, LOCATION_ID).expect("game should start");
    let state_b = new_game_from_content_at(2, &index, LOCATION_ID).expect("game should start");

    let page_a = scene_page_from_content(&state_a, &index).expect("render should succeed");
    let page_b = scene_page_from_content(&state_b, &index).expect("render should succeed");

    let fingerprint_a = page_a.combat.unwrap().view.fingerprint;
    let fingerprint_b = page_b.combat.unwrap().view.fingerprint;
    assert_ne!(
        fingerprint_a, fingerprint_b,
        "different run seeds must produce different combat fingerprints"
    );
}

/// WP-4 case 6 (invariant 1 proof): changing the *authoring* manifest's
/// `actual_seed` must not change the actual combat outcome, because the
/// producer always overwrites it with a run-derived value.
#[test]
fn systemic_combat_producer_result_is_independent_of_authoring_actual_seed() {
    let bundle_a = bundle_with_combat(|combat| combat["manifest"]["actual_seed"] = json!(1));
    let bundle_b = bundle_with_combat(|combat| combat["manifest"]["actual_seed"] = json!(999_999));

    let index_a = index_content_bundle(&bundle_a).expect("bundle should index");
    let index_b = index_content_bundle(&bundle_b).expect("bundle should index");

    let state_a = new_game_from_content_at(123, &index_a, LOCATION_ID).expect("game should start");
    let state_b = new_game_from_content_at(123, &index_b, LOCATION_ID).expect("game should start");

    let page_a = scene_page_from_content(&state_a, &index_a).expect("render should succeed");
    let page_b = scene_page_from_content(&state_b, &index_b).expect("render should succeed");

    assert_eq!(
        page_a.combat.unwrap().view.fingerprint,
        page_b.combat.unwrap().view.fingerprint,
        "authoring actual_seed must not influence the real combat seed"
    );
}

/// A combat-less encounter must still round-trip to `combat: None` with no
/// `"combat"` JSON key (Step 1c boundary contract; also WP-4 case 1).
#[test]
fn encounter_without_combat_still_yields_no_combat_key_in_json() {
    let bundle = load_content_bundle(BUNDLE).expect("fixture bundle should load");
    let index = index_content_bundle(&bundle).expect("bundle should index");
    let state = new_game_from_content_at(5, &index, LOCATION_ID).expect("game should start");
    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    assert!(page.combat.is_none());

    let value = serde_json::to_value(&page).expect("ScenePage should serialize");
    assert!(value.as_object().unwrap().get("combat").is_none());
}

/// WP-4 regression: a `ScenePage` filled by the *real* systemic combat
/// producer (not the synthetic `CombatSpectatorPage` used in
/// `scene_page_combat_boundary.rs`) still round-trips losslessly through
/// serde, and the `"combat"` key is present with a non-empty view.
#[test]
fn systemic_combat_scene_page_round_trips_through_serde() {
    let bundle = bundle_with_combat(|_| {});
    let index = index_content_bundle(&bundle).expect("bundle should index");
    let state = new_game_from_content_at(9, &index, LOCATION_ID).expect("game should start");
    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    assert!(page.combat.is_some());

    let json = serde_json::to_string(&page).expect("ScenePage should serialize to string");
    let value: Value = serde_json::from_str(&json).expect("serialized JSON should parse");
    assert!(
        value.as_object().unwrap().get("combat").is_some(),
        "combat key must appear once ScenePage.combat is Some"
    );

    let restored: escape_core::ScenePage =
        serde_json::from_str(&json).expect("ScenePage should deserialize");
    assert_eq!(restored, page);
}

#[test]
fn combat_producer_failures_report_as_their_own_error_variant() {
    // The producer path is defensive (index-time validation rejects the kinds
    // that would reach it), but the variant is user-visible through Display on
    // the terminal, so pin the shape and the message here.
    let error = ContentTurnError::CombatProducer {
        encounter_id: "printer_prints_alone".to_string(),
        reason: "seed derivation failed".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "combat producer failed for encounter 'printer_prints_alone': seed derivation failed"
    );
    assert_ne!(
        error,
        ContentTurnError::UnknownStateLocation("printer_prints_alone".to_string()),
        "a combat failure must not masquerade as an unknown location"
    );
}

// ---------------------------------------------------------------------
// Wave 3 Step 2b: the real `wuxia_combat_spectator_preview_bout` encounter,
// authored in the wuxia storypack-preview bundle behind the
// `combat_spectator_preview_unlocked` gate flag (invariant 8 -- no ordinary
// play path may ever set it, so these tests set it directly on the state).
// ---------------------------------------------------------------------

/// WP-3 case 1: without the gate flag, the encounter is unreachable -- some
/// other (combat-less) encounter is current, and `ScenePage.combat` is `None`.
#[test]
fn spectator_preview_bout_is_unreachable_without_the_gate_flag() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let state = new_game_from_content_at(1, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");

    let view = turn_view_from_content(&state, &index).expect("turn view should render");
    assert_ne!(
        view.encounter_id.as_deref(),
        Some(SPECTATOR_BOUT_ID),
        "the gated encounter must not be selectable without its flag"
    );

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    assert!(
        page.combat.is_none(),
        "no other encounter in the bundle authors combat, so ScenePage.combat must be None here"
    );
}

/// WP-3 case 2 & 3: setting the gate flag selects the encounter and fills
/// `ScenePage.combat` with a non-empty spectator view and a report.
#[test]
fn gate_flag_selects_the_bout_and_fills_scene_page_combat() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(2, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let view = turn_view_from_content(&state, &index).expect("turn view should render");
    assert_eq!(
        view.encounter_id.as_deref(),
        Some(SPECTATOR_BOUT_ID),
        "the gate flag must make this the current encounter"
    );

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("gated systemic combat should fill ScenePage.combat");
    assert!(
        !combat.view.frames.is_empty(),
        "spectator view should have at least one tick frame"
    );
    assert!(
        combat.report.is_some(),
        "a concluded combat should carry a report"
    );
}

/// WP-3 case 4: the report carries exactly the two authored combatants, each
/// with non-negative damage totals.
#[test]
fn report_covers_both_combatants_with_non_negative_damage_totals() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(3, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let report = page
        .combat
        .expect("gated systemic combat should fill ScenePage.combat")
        .report
        .expect("concluded combat should carry a report");

    assert_eq!(
        report.combatants.len(),
        2,
        "exactly two authored combatants"
    );
    for combatant in &report.combatants {
        assert!(combatant.damage_dealt_hundredths >= 0);
        assert!(combatant.damage_taken_hundredths >= 0);
    }
}

/// WP-3 case 5 (정본 검산): the first landed hit deals exactly 1333
/// hundredths of damage. Both authored attacks use the canonical 정본 11
/// standard combatant numbers (power 40 / ability multiplier 1.0 / defense 5),
/// so `damage(attack, defense)` in `combat_resolution.rs` must yield
/// `pre = 4000 * 5 * 100 / 1200 = 1666`, `reduction = 1666 * 500 / 2500 = 333`,
/// `damage = 1666 - 333 = 1333`. This single assertion pins both the
/// authoring numbers and the resolver formula together.
#[test]
fn wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(4, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("gated systemic combat should fill ScenePage.combat");

    let first_hit = combat
        .view
        .full_log
        .iter()
        .find(|entry| entry.template_id == "combat.log.damage_applied")
        .expect("at least one landed hit is expected between two standard combatants");
    assert_eq!(
        first_hit.value_hundredths,
        Some(1333),
        "정본 11 표준 전투원 대 표준 전투원 첫 명중 피해는 1333 hundredths여야 한다"
    );
}

/// WP-3 case 6: calling the producer twice for the same state yields a
/// completely identical `ScenePage.combat`.
#[test]
fn gated_combat_is_deterministic_for_the_same_state() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(5, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let first = scene_page_from_content(&state, &index).expect("first render should succeed");
    let second = scene_page_from_content(&state, &index).expect("second render should succeed");
    assert_eq!(first.combat, second.combat);
}

/// WP-3 case 7 (invariant 6): the encounter has a staged `event` (Story ->
/// Choice -> per-choice Result), like every other wuxia preview encounter.
#[test]
fn spectator_preview_bout_has_a_staged_event() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let encounter = index
        .encounter(SPECTATOR_BOUT_ID)
        .expect("the gated encounter must exist in the bundle");
    let event = encounter
        .event
        .as_ref()
        .expect("Wave 3 Step 2b entry must be staged");
    assert_eq!(event.stages.first().map(|s| s.kind.as_str()), Some("story"));
    assert_eq!(event.stages.get(1).map(|s| s.kind.as_str()), Some("choice"));
    let illustrations: Vec<_> = event
        .stages
        .iter()
        .flat_map(|s| s.blocks.iter())
        .filter(|b| b.kind == "illustration")
        .collect();
    assert_eq!(illustrations.len(), 1);
    assert!(illustrations[0]
        .alt
        .as_deref()
        .is_some_and(|alt| !alt.trim().is_empty()));
}

/// T1-b1 WP6 (§4-6, the plan's own pre-registered prediction): both
/// combatants sit on `r = 0`, where hex distance collapses to `|dq|` and
/// equals the old euclidean distance exactly, so swapping `{x,y}` for
/// `{q,r}` was predicted to leave this bout's behaviour completely
/// unchanged. This test turns that prediction into a fixed regression --
/// exactly the tick count, conclusion, and per-hit damage confirmed by a
/// direct pre/post-change comparison (see step2 report §4). If any of these
/// values ever moves, that means the coordinate swap changed real combat
/// behaviour and the fix belongs in analysis, not in this assertion.
#[test]
fn authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(4, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("gated systemic combat should fill ScenePage.combat");
    let report = combat
        .report
        .as_ref()
        .expect("concluded combat should carry a report");

    assert_eq!(
        combat.view.frames.len(),
        8,
        "the bout must still take exactly 8 ticks to conclude"
    );
    assert_eq!(report.decisive_tick, Some(8));
    assert_eq!(report.outcome, CombatConclusionOutcome::MutualDefeat);
    assert_eq!(report.reason, CombatConclusionReason::BothSidesDefeated);

    let damage_entries: Vec<_> = combat
        .view
        .full_log
        .iter()
        .filter(|e| e.template_id == "combat.log.damage_applied")
        .collect();
    assert_eq!(
        damage_entries.len(),
        16,
        "16 landed hits total (2 per tick x 8 ticks)"
    );
    assert!(
        damage_entries
            .iter()
            .all(|e| e.value_hundredths == Some(1333)),
        "every landed hit must still deal exactly 1333 hundredths"
    );
}

/// T1-c (`fable_combat_hex_t1c_step1_2608072138.md` §6): the test above
/// deliberately never asserts positions -- it only pins conclusion and
/// damage, which the plan predicted (and this confirms) stay identical
/// whether or not occupancy is enforced, since both combatants remain
/// within `attack_range: 10` either way. This is the assertion that
/// actually checks occupancy did something: before T1-c, the two
/// combatants walked through each other and swapped which side of the
/// board they were on tick to tick (the recorded defect the plan opens
/// with, violating canon 09's "ally on the left / enemy on the right").
/// With occupancy enforced they must never cross, and never share a tile.
#[test]
fn authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile() {
    let index = index_content_bundle(&load_content_bundle(WUXIA_BUNDLE).unwrap())
        .expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(4, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("gated systemic combat should fill ScenePage.combat");

    for frame in &combat.view.frames {
        let ally = frame
            .pieces
            .iter()
            .find(|p| p.id == "wuxia_spectator_bout_ally")
            .expect("ally piece must be present every frame");
        let challenger = frame
            .pieces
            .iter()
            .find(|p| p.id == "wuxia_spectator_bout_challenger")
            .expect("challenger piece must be present every frame");
        assert_ne!(
            ally.position, challenger.position,
            "tick {}: the two combatants must never occupy the same tile",
            frame.tick
        );
        assert!(
            ally.position.q < challenger.position.q,
            "tick {}: the ally (started at q=0) must stay left of the challenger \
             (started at q=5) -- got ally={:?}, challenger={:?}",
            frame.tick,
            ally.position,
            challenger.position
        );
    }
}
