//! T1-d (`fable_combat_hex_t1d_step1_2608072234.md`) WP4: the minimum test
//! set the plan names (§6 WP4 table). Each test pins one piece of §4's
//! design directly, not just "the suite stays green" -- see each test's own
//! comment for which plan clause it fixes.
//!
//! Helper builders below mirror `combat_occupancy_t1c.rs`'s shape (this is a
//! separate integration-test binary, so nothing can be shared directly),
//! extended with an `occupies` parameter this slice's participants need.

use escape_core::*;

fn manifest() -> CombatManifest {
    CombatManifest {
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
    }
}

fn state() -> CombatState {
    CombatState {
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
    }
}

fn role(id: &str, preferred_distance: i32, aggression: i32) -> CombatRolePreset {
    CombatRolePreset {
        id: id.into(),
        weights: CombatRoleWeights {
            preferred_distance,
            aggression,
            formation_maintenance: 0,
            pursuit_range: 0,
            protect_priority: 0,
            target_priority: 0,
            risk_tolerance: 0,
            ability_priority: 0,
        },
    }
}

fn participant(
    id: &str,
    side: CombatSide,
    position: HexCoord,
    role_id: &str,
    speed_per_tick: i32,
    occupies: Vec<HexCoord>,
) -> CombatSimulationParticipant {
    CombatSimulationParticipant {
        id: id.into(),
        side,
        position,
        facing: HexCoord { q: 1, r: 0 },
        speed_per_tick,
        collision_radius: 1,
        attack_range: 2,
        support_range: 2,
        role_id: role_id.into(),
        target_policy_id: None,
        active: true,
        occupies,
    }
}

fn input(
    parts: Vec<CombatSimulationParticipant>,
    roles: Vec<CombatRolePreset>,
    policies: Vec<CombatTargetPolicy>,
    max_ticks: u32,
) -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: manifest(),
        state: state(),
        seed: 1,
        config: CombatSimulationConfig {
            tick_millis: 100,
            max_ticks,
        },
        participants: parts,
        roles,
        policies,
    }
}

fn hex(q: i32, r: i32) -> HexCoord {
    HexCoord { q, r }
}

/// Builds a minimal two-combatant `CombatResolutionRequest` for testing the
/// attack-resolution distance check (plan §4-4 site 5) in isolation. Both
/// combatants use a "stay" role with a very high `preferred_distance` and
/// zero aggression, so neither moves this tick and `frame.positions` stays
/// exactly at `a_position`/`e_position` -- isolating the assertion to the
/// resolution distance check itself, not movement.
fn resolution_request(
    a_position: HexCoord,
    a_occupies: Vec<HexCoord>,
    e_position: HexCoord,
    attack_range: i32,
) -> CombatResolutionRequest {
    let manifest = CombatManifest {
        simulation_version: CombatSimulationVersion::new(CURRENT_SIMULATION_VERSION).unwrap(),
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
                participants: vec![
                    CombatSimulationParticipant {
                        id: "a".into(),
                        side: CombatSide::Ally,
                        position: a_position,
                        facing: HexCoord { q: 1, r: 0 },
                        speed_per_tick: 1,
                        collision_radius: 1,
                        attack_range,
                        support_range: 2,
                        role_id: "stay".into(),
                        target_policy_id: None,
                        active: true,
                        occupies: a_occupies,
                    },
                    CombatSimulationParticipant {
                        id: "e".into(),
                        side: CombatSide::Enemy,
                        position: e_position,
                        facing: HexCoord { q: -1, r: 0 },
                        speed_per_tick: 1,
                        collision_radius: 1,
                        attack_range,
                        support_range: 2,
                        role_id: "stay".into(),
                        target_policy_id: None,
                        active: true,
                        occupies: vec![],
                    },
                ],
                roles: vec![CombatRolePreset {
                    id: "stay".into(),
                    weights: CombatRoleWeights {
                        preferred_distance: 1000,
                        aggression: 0,
                        formation_maintenance: 0,
                        pursuit_range: 0,
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
            attack_range,
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

// ---------------------------------------------------------------------
// §4-1 -- an empty `occupies` means exactly one tile, the anchor.
// ---------------------------------------------------------------------

#[test]
fn an_empty_occupies_list_means_a_single_tile_at_the_anchor() {
    // "a" has empty `occupies` and sits at (0,0). A second participant must
    // be free to occupy any of "a"'s six neighbors -- if the empty list
    // secretly meant something bigger than one tile, at least one neighbor
    // would be rejected as overlapping.
    for &neighbor in &HexCoord::NEIGHBOR_DIRECTIONS {
        let a = participant("a", CombatSide::Ally, hex(0, 0), "r", 1, vec![]);
        let b = participant("b", CombatSide::Enemy, neighbor, "r", 1, vec![]);
        assert!(
            CombatSimulation::new(input(vec![a, b], vec![role("r", 0, 0)], vec![], 3)).is_ok(),
            "neighbor {neighbor:?} must be free to occupy when \"a\"'s occupies is empty"
        );
    }
    // But the exact same tile as "a" itself must still be rejected --
    // confirming the single tile really is (0,0) itself, not merely "none
    // of the neighbors."
    let a = participant("a", CombatSide::Ally, hex(0, 0), "r", 1, vec![]);
    let same_tile = participant("b", CombatSide::Enemy, hex(0, 0), "r", 1, vec![]);
    assert!(matches!(
        CombatSimulation::new(input(vec![a, same_tile], vec![role("r", 0, 0)], vec![], 3)),
        Err(CombatSimulationError::DuplicateStartingPosition(pos)) if pos == hex(0, 0)
    ));
}

// ---------------------------------------------------------------------
// §4-2 -- the anchor must be one of the occupied tiles.
// ---------------------------------------------------------------------

#[test]
fn occupies_without_the_origin_offset_is_rejected() {
    // Two adjacent offsets, neither of which is (0,0).
    let a = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "r",
        1,
        vec![hex(1, 0), hex(1, -1)],
    );
    let result = CombatSimulation::new(input(vec![a], vec![role("r", 0, 0)], vec![], 3));
    assert!(matches!(
        result,
        Err(CombatSimulationError::FootprintMissingAnchor(id)) if id == "a"
    ));
}

// ---------------------------------------------------------------------
// §4-3 -- occupied tiles must form one connected blob.
// ---------------------------------------------------------------------

#[test]
fn a_disconnected_footprint_is_rejected() {
    // (0,0) and (5,0) are far apart -- two separate blobs, not one shape.
    let a = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "r",
        1,
        vec![hex(0, 0), hex(5, 0)],
    );
    let result = CombatSimulation::new(input(vec![a], vec![role("r", 0, 0)], vec![], 3));
    assert!(matches!(
        result,
        Err(CombatSimulationError::DisconnectedFootprint(id)) if id == "a"
    ));
}

#[test]
fn a_duplicate_offset_is_rejected() {
    let a = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "r",
        1,
        vec![hex(0, 0), hex(1, 0), hex(1, 0)],
    );
    let result = CombatSimulation::new(input(vec![a], vec![role("r", 0, 0)], vec![], 3));
    assert!(matches!(
        result,
        Err(CombatSimulationError::HexMath(HexError::DuplicateOffset(tile))) if tile == hex(1, 0)
    ));
}

// ---------------------------------------------------------------------
// §4-6 -- two footprints overlapping by even one tile cannot both start.
// ---------------------------------------------------------------------

#[test]
fn two_large_units_with_overlapping_footprints_cannot_both_start() {
    // "a" is a 2-tile unit anchored at (0,0) covering (0,0) and (1,0). "b"
    // is a 2-tile unit anchored at (1,0) itself, covering (1,0) and (2,0) --
    // footprints share tile (1,0) even though the two anchors differ.
    let a = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "r",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let b = participant(
        "b",
        CombatSide::Enemy,
        hex(1, 0),
        "r",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let result = CombatSimulation::new(input(vec![a, b], vec![role("r", 0, 0)], vec![], 3));
    assert!(matches!(
        result,
        Err(CombatSimulationError::DuplicateStartingPosition(pos)) if pos == hex(1, 0)
    ));
}

// ---------------------------------------------------------------------
// §4-4 -- distance is footprint-to-footprint minimum, not anchor-to-anchor.
// ---------------------------------------------------------------------

#[test]
fn distance_is_measured_from_the_nearest_occupied_tile() {
    // "a" is a 3-tile line reaching from (0,0) to (2,0). "b" is a single
    // tile at (4,0).
    let a_anchor = hex(0, 0);
    let a_occupies = vec![hex(0, 0), hex(1, 0), hex(2, 0)];
    let b_anchor = hex(4, 0);
    let distance = footprint_distance(a_anchor, &a_occupies, b_anchor, &[]).unwrap();
    // The nearest tile of "a"'s footprint to "b" is (2,0), distance 2.
    assert_eq!(distance, 2);
    // Sanity check: anchor-to-anchor distance would have been 4 -- confirms
    // this isn't accidentally still measuring from the anchor.
    assert_eq!(a_anchor.distance(b_anchor), 4);
}

#[test]
fn a_large_unit_in_range_by_its_body_but_not_its_anchor_can_attack() {
    // "a" is a 4-tile line from (0,0) to (3,0); "e" sits at (4,0), one tile
    // from "a"'s nearest occupied tile (3,0) but four tiles from "a"'s
    // anchor (0,0). collision_radius (1 each, reach 2) and attack_range (1)
    // both only cover the footprint distance (1), not the anchor distance
    // (4) -- this is exactly why plan §4-4 exists: a large unit's body, not
    // just its reported anchor, must be able to reach an attack.
    let request = resolution_request(
        hex(0, 0),
        vec![hex(0, 0), hex(1, 0), hex(2, 0), hex(3, 0)],
        hex(4, 0),
        1,
    );
    let result = resolve_combat(request).unwrap();
    let outcome = &result.frames[0].outcomes[0];
    assert!(
        outcome.in_range,
        "a's footprint reaches to (3,0), one tile from e at (4,0) -- must be in range even \
         though the anchor (0,0) is four tiles away"
    );
    assert!(
        outcome.collision,
        "the collision/melee-reach check uses the same footprint distance"
    );
    assert!(
        outcome.hit,
        "100% accuracy plus in-range plus collision must hit"
    );
}

// ---------------------------------------------------------------------
// §4-5 -- movement blocks on any footprint tile, and a mover never blocks
// itself.
// ---------------------------------------------------------------------

#[test]
fn a_large_unit_stops_when_any_footprint_tile_would_be_blocked() {
    // "a" is a 2-tile unit (occupies (0,0) and (1,0) relative to its
    // anchor) advancing at speed 5 toward an enemy far to the right.
    // "blocker" (same side as "a", so it is never a valid attack target)
    // sits at (3,0). At candidate anchor (2,0), "a"'s footprint would be
    // {(2,0), (3,0)} -- (3,0) collides with "blocker" even though (2,0)
    // itself, the anchor tile, is empty. "a" must stop one step earlier, at
    // anchor (1,0) (footprint {(1,0), (2,0)}), not at (2,0).
    let a = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "advance",
        5,
        vec![hex(0, 0), hex(1, 0)],
    );
    let blocker = participant("blocker", CombatSide::Ally, hex(3, 0), "hold", 1, vec![]);
    let e = participant("e", CombatSide::Enemy, hex(10, 0), "hold", 1, vec![]);
    let mut sim = CombatSimulation::new(input(
        vec![a, blocker, e],
        vec![role("advance", 0, 1), role("hold", 1000, 0)],
        vec![],
        3,
    ))
    .unwrap();
    let frame = sim.advance_tick().unwrap();
    let mover = frame.moves.iter().find(|m| m.actor_id == "a").unwrap();
    assert_eq!(
        mover.to,
        hex(1, 0),
        "must stop before its trailing tile (3,0) would collide with the blocker, even though \
         the anchor tile (2,0) is itself empty"
    );
    assert_ne!(
        mover.to,
        hex(2, 0),
        "(2,0) is the old single-tile-anchor-only result -- ignoring that the footprint's other \
         tile (3,0) is occupied"
    );
}

#[test]
fn a_large_unit_does_not_block_itself_while_moving() {
    // "mover" is a 2-tile unit (occupies (0,0) and (1,0)) advancing one step
    // at a time toward a distant enemy, with nothing else on the board. At
    // its very first step (anchor (1,0), footprint {(1,0), (2,0)}), tile
    // (1,0) is part of "mover"'s own *current* footprint and is still
    // marked occupied (under "mover"'s own id) in this tick's frozen
    // snapshot. If that self-occupied tile were not excluded, "mover" would
    // read its own trailing tile as "blocked" and never move at all --
    // indistinguishable from a legitimately blocked unit unless someone
    // checks why.
    let mover = participant(
        "mover",
        CombatSide::Ally,
        hex(0, 0),
        "advance",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let target = participant("target", CombatSide::Enemy, hex(10, 0), "hold", 1, vec![]);
    let mut sim = CombatSimulation::new(input(
        vec![mover, target],
        vec![role("advance", 0, 1), role("hold", 1000, 0)],
        vec![],
        3,
    ))
    .unwrap();
    let frame = sim.advance_tick().unwrap();
    let mover_move = frame.moves.iter().find(|m| m.actor_id == "mover").unwrap();
    assert_eq!(
        mover_move.to,
        hex(1, 0),
        "a large unit's own trailing footprint tile must not block its own advance -- without \
         self-exclusion this would incorrectly stay at (0,0)"
    );
    assert_eq!(mover_move.mode, CombatMoveMode::Advance);
}

#[test]
fn overlapping_destinations_make_both_large_units_hold() {
    // "mover_a" (2-tile, anchor (0,0), footprint {(0,0),(1,0)}) advances
    // toward a far-right target; "mover_b" (2-tile, anchor (4,0), footprint
    // {(3,0),(4,0)}) advances toward a far-left target. Neither's own path
    // is blocked by tick-start occupancy (their target tiles are all
    // currently vacant), but their *computed destination footprints* --
    // {(1,0),(2,0)} for mover_a and {(2,0),(3,0)} for mover_b -- share tile
    // (2,0) even though the two anchors land on different tiles ((1,0) vs
    // (3,0)). Both must give up the move and hold, exactly as two
    // single-tile units converging on the same tile would (T1-c §4-2③),
    // generalized to "share a tile" instead of "are the same tile."
    let mover_a = participant(
        "mover_a",
        CombatSide::Ally,
        hex(0, 0),
        "advance",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let mover_b = participant(
        "mover_b",
        CombatSide::Enemy,
        hex(4, 0),
        "advance",
        1,
        vec![hex(0, 0), hex(-1, 0)],
    );
    let target_a = participant("target_a", CombatSide::Enemy, hex(20, 0), "stay", 1, vec![]);
    let target_b = participant("target_b", CombatSide::Ally, hex(-20, 0), "stay", 1, vec![]);
    let policy_a = CombatTargetPolicy {
        id: "policy_a".into(),
        preferences: vec![CombatTargetPreference {
            target_id: "target_a".into(),
            priority: 1,
        }],
        fallback: CombatTargetFallback::Nearest,
    };
    let policy_b = CombatTargetPolicy {
        id: "policy_b".into(),
        preferences: vec![CombatTargetPreference {
            target_id: "target_b".into(),
            priority: 1,
        }],
        fallback: CombatTargetFallback::Nearest,
    };
    let mut mover_a = mover_a;
    mover_a.target_policy_id = Some("policy_a".into());
    let mut mover_b = mover_b;
    mover_b.target_policy_id = Some("policy_b".into());

    let mut sim = CombatSimulation::new(input(
        vec![mover_a, mover_b, target_a, target_b],
        vec![role("advance", 0, 1), role("stay", 1000, 0)],
        vec![policy_a, policy_b],
        3,
    ))
    .unwrap();
    let frame = sim.advance_tick().unwrap();

    let a_move = frame
        .moves
        .iter()
        .find(|m| m.actor_id == "mover_a")
        .unwrap();
    let b_move = frame
        .moves
        .iter()
        .find(|m| m.actor_id == "mover_b")
        .unwrap();
    assert_eq!(
        a_move.to,
        hex(0, 0),
        "mover_a's destination footprint would have overlapped mover_b's at (2,0) -- it must hold"
    );
    assert_eq!(
        b_move.to,
        hex(4, 0),
        "mover_b's destination footprint would have overlapped mover_a's at (2,0) -- it must hold"
    );
    assert_eq!(a_move.mode, CombatMoveMode::Hold);
    assert_eq!(b_move.mode, CombatMoveMode::Hold);
}

// ---------------------------------------------------------------------
// Hard invariant 1 -- single-tile behavior is completely unchanged.
// ---------------------------------------------------------------------

#[test]
fn single_tile_units_behave_exactly_as_before() {
    // The same scenario as T1-c's
    // `a_unit_stops_before_an_occupied_tile_instead_of_passing_through`,
    // reproduced here with `occupies` explicitly present-but-empty on every
    // participant, pinning that single-tile behavior is untouched by this
    // slice.
    let a = participant("a", CombatSide::Ally, hex(0, 0), "r", 3, vec![]);
    let blocker = participant("blocker", CombatSide::Ally, hex(2, 0), "r", 1, vec![]);
    let e = participant("e", CombatSide::Enemy, hex(10, 0), "r", 1, vec![]);
    let mut sim =
        CombatSimulation::new(input(vec![a, blocker, e], vec![role("r", 0, 1)], vec![], 3))
            .unwrap();
    let frame = sim.advance_tick().unwrap();
    let mover = frame.moves.iter().find(|m| m.actor_id == "a").unwrap();
    assert_eq!(
        mover.to,
        hex(1, 0),
        "single-tile movement/occupancy behavior must be bit-for-bit identical to T1-c"
    );
}

// ---------------------------------------------------------------------
// Hard invariant 2 -- occupies never appears in JSON when empty.
// ---------------------------------------------------------------------

#[test]
fn occupies_is_absent_from_json_when_empty() {
    let empty = participant("a", CombatSide::Ally, hex(0, 0), "r", 1, vec![]);
    let json = serde_json::to_string(&empty).unwrap();
    assert!(
        !json.contains("occupies"),
        "empty occupies must not appear in JSON at all -- got: {json}"
    );

    let large = participant(
        "a",
        CombatSide::Ally,
        hex(0, 0),
        "r",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let json_large = serde_json::to_string(&large).unwrap();
    assert!(
        json_large.contains("\"occupies\""),
        "non-empty occupies must appear in JSON -- got: {json_large}"
    );
}

// ---------------------------------------------------------------------
// Invariant 4 -- order independence holds for large-unit footprints too.
// ---------------------------------------------------------------------

#[test]
fn shuffled_participant_order_yields_identical_frames() {
    let build = |order: Vec<CombatSimulationParticipant>| {
        CombatSimulation::new(input(
            order,
            vec![role("advance", 0, 1), role("hold", 0, 0)],
            vec![],
            10,
        ))
        .unwrap()
    };
    let center = participant("center", CombatSide::Enemy, hex(0, 0), "hold", 1, vec![]);
    let large = participant(
        "large",
        CombatSide::Ally,
        hex(0, -6),
        "advance",
        1,
        vec![hex(0, 0), hex(1, 0)],
    );
    let n2 = participant("n2", CombatSide::Ally, hex(6, -6), "advance", 1, vec![]);
    let n3 = participant("n3", CombatSide::Ally, hex(6, 0), "advance", 1, vec![]);
    let n4 = participant("n4", CombatSide::Ally, hex(-6, 6), "advance", 1, vec![]);

    let forward = vec![
        center.clone(),
        large.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
    ];
    let mut shuffled = vec![n3, large, center, n4, n2];
    shuffled.reverse();

    let mut sim_forward = build(forward);
    let mut sim_shuffled = build(shuffled);
    assert_eq!(
        sim_forward.run_ticks(10).unwrap(),
        sim_shuffled.run_ticks(10).unwrap(),
        "input order must not change any tick's resolved frame, including footprint-based path \
         blocking and destination-contention decisions"
    );
}
