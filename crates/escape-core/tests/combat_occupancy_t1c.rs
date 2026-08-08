//! T1-c (`fable_combat_hex_t1c_step1_2608072138.md`) WP5: the minimum test
//! set the plan names. Each test pins one piece of §4's design directly, not
//! just "the suite stays green" -- see each test's own comment for which
//! plan clause it fixes.
//!
//! Helper builders below intentionally mirror `combat_simulation_wave2.rs`'s
//! shape (this is a separate integration-test binary, so nothing can be
//! shared directly), but this file's `participant()` takes a full `HexCoord`
//! instead of just a `q` offset -- most of these tests need `r != 0` to set
//! up real hex adjacency, which `combat_simulation_wave2.rs`'s bare-`q`
//! helper never needed.

use escape_core::*;
use std::collections::BTreeSet;

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

/// `preferred_distance`/`aggression` are the only weights this slice's
/// movement logic reads (T1-b1); the other five stay at 0, matching the
/// plan §4 "이미 예약되어 있는 자리" table (unread until later tracks).
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
) -> CombatSimulationParticipant {
    CombatSimulationParticipant {
        id: id.into(),
        side,
        position,
        facing: HexCoord { q: 1, r: 0 },
        speed_per_tick,
        // T3 (fable_combat_hex_t3_step1_2608080951.md §4-3): new field on
        // `CombatSimulationParticipant`. `None` means "act every tick",
        // exactly this fixture's pre-T3 behaviour -- mechanical fix to keep
        // this file compiling, not a scope change.
        move_speed_hundredths: None,
        collision_radius: 1,
        attack_range: 2,
        support_range: 2,
        role_id: role_id.into(),
        target_policy_id: None,
        active: true,
        // T1-d (fable_combat_hex_t1d_step1_2608072234.md): new field on
        // `CombatSimulationParticipant`. Empty = single tile at the anchor,
        // exactly this fixture's pre-T1-d meaning -- mechanical fix to keep
        // this file compiling, not a scope change; see the T1-d step2
        // report for why this file needed touching despite being outside
        // that slice's original ownership list.
        occupies: vec![],
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

// ---------------------------------------------------------------------
// §4-2① -- initial placement.
// ---------------------------------------------------------------------
//
// This check was implemented, then reverted, then re-landed once ownership
// was expanded mid-slice -- see the step2 report's "WP1" section for the
// full account. It first broke 42 previously-passing tests in
// `crates/escape-core/tests/combat_resolution_wave2.rs` and
// `crates/escape-core/tests/combat_spectator_wave3.rs`, both outside this
// slice's original ownership list: both files' two-participant fixtures
// defaulted both combatants to the same starting tile (`HexCoord{q:0,r:0}`),
// because those suites only ever exercised attack resolution and spectator
// log formatting, never spatial validity. Those fixtures were always
// invalid input; nothing checked it until this slice added the check. The
// coordinator verified the finding, expanded ownership to cover both files,
// and confirmed moving the second participant one tile over doesn't change
// any of those 42 tests' expected values (collision_radius sums to 2,
// attack_range is 2, and hex distance 1 satisfies both exactly as distance
// 0 did). Both fixtures now separate their two participants by one tile,
// and the check below ships as designed.

#[test]
fn two_participants_cannot_start_on_the_same_tile() {
    let a = participant("a", CombatSide::Ally, hex(0, 0), "r", 1);
    let e = participant("e", CombatSide::Enemy, hex(0, 0), "r", 1);
    let result = CombatSimulation::new(input(vec![a, e], vec![role("r", 0, 1)], vec![], 3));
    assert!(matches!(
        result,
        Err(CombatSimulationError::DuplicateStartingPosition(pos)) if pos == hex(0, 0)
    ));
}

// ---------------------------------------------------------------------
// Slice-wide core invariant.
// ---------------------------------------------------------------------

#[test]
fn no_two_units_share_a_tile_at_any_tick() {
    // Four allies converge on one central enemy from the four "cardinal"
    // neighbor directions available in axial hex space, all at speed 1, so
    // every tick has real contention to resolve (path blocking, then
    // destination contention on whatever's left). If occupancy enforcement
    // has any gap, this is the kind of scenario that would surface it.
    let center_enemy = participant("center", CombatSide::Enemy, hex(0, 0), "hold", 1);
    let attackers = vec![
        participant("n1", CombatSide::Ally, hex(0, -6), "advance", 1),
        participant("n2", CombatSide::Ally, hex(6, -6), "advance", 1),
        participant("n3", CombatSide::Ally, hex(6, 0), "advance", 1),
        participant("n4", CombatSide::Ally, hex(-6, 6), "advance", 1),
    ];
    let mut parts = attackers;
    parts.push(center_enemy);
    let roles = vec![role("advance", 0, 1), role("hold", 0, 0)];
    let mut sim = CombatSimulation::new(input(parts, roles, vec![], 10)).unwrap();
    for _ in 0..10 {
        let frame = sim.advance_tick().unwrap();
        let mut seen = BTreeSet::new();
        for (id, pos) in &frame.positions {
            assert!(
                seen.insert(*pos),
                "tile {pos:?} is occupied by more than one participant at tick {} (id {id})",
                frame.tick
            );
        }
    }
}

// ---------------------------------------------------------------------
// §4-2② -- path blocking seals the pass-through defect.
// ---------------------------------------------------------------------

#[test]
fn a_unit_stops_before_an_occupied_tile_instead_of_passing_through() {
    // "a" (speed 3) advances toward its only enemy "e" at q=10, but ally
    // "blocker" sits at q=2, directly on the path. Before T1-c, "a" would
    // land at q=3 (the pass-through defect this slice seals); with
    // occupancy enforced it must stop at q=1, the last free tile before the
    // blocker.
    let a = participant("a", CombatSide::Ally, hex(0, 0), "r", 3);
    let blocker = participant("blocker", CombatSide::Ally, hex(2, 0), "r", 1);
    let e = participant("e", CombatSide::Enemy, hex(10, 0), "r", 1);
    let mut sim =
        CombatSimulation::new(input(vec![a, blocker, e], vec![role("r", 0, 1)], vec![], 3))
            .unwrap();
    let frame = sim.advance_tick().unwrap();
    let mover = frame.moves.iter().find(|m| m.actor_id == "a").unwrap();
    assert_eq!(
        mover.to,
        hex(1, 0),
        "must stop one tile short of the blocker, not pass through it"
    );
    assert_ne!(
        mover.to,
        hex(3, 0),
        "q=3 is the old pass-through result -- walking straight past the blocker at q=2"
    );
}

// ---------------------------------------------------------------------
// §4-2③/§4-3 -- destination contention: both hold, no priority invented.
// ---------------------------------------------------------------------

#[test]
fn two_units_targeting_the_same_tile_both_hold() {
    // Two allies approach tile (1,0) from different starting tiles and
    // different directions this same tick -- (1,0) is free in the tick-start
    // snapshot, so neither is blocked by §4-2②. Both must give up the move
    // (§4-2③) rather than one of them winning by id order.
    let mover_a = participant("mover_a", CombatSide::Ally, hex(0, 0), "advance", 1);
    let mover_b = participant("mover_b", CombatSide::Ally, hex(2, -1), "advance", 1);
    // Explicit target policies pin exactly which tile each mover's first
    // step lands on, independent of "nearest enemy" fallback ambiguity.
    let target_a = participant("target_a", CombatSide::Enemy, hex(10, 0), "hold", 1);
    let target_b = participant("target_b", CombatSide::Enemy, hex(-8, 9), "hold", 1);
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
        vec![role("advance", 0, 1), role("hold", 0, 0)],
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
    // Sanity check: both really were headed for the same tile before
    // contention resolution would have been moot.
    assert_eq!(
        a_move.to,
        hex(0, 0),
        "mover_a must have given up its move and held"
    );
    assert_eq!(
        b_move.to,
        hex(2, -1),
        "mover_b must have given up its move and held"
    );
    assert_eq!(a_move.mode, CombatMoveMode::Hold);
    assert_eq!(b_move.mode, CombatMoveMode::Hold);
}

// ---------------------------------------------------------------------
// Invariant 1 -- order independence.
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
    let center = participant("center", CombatSide::Enemy, hex(0, 0), "hold", 1);
    let n1 = participant("n1", CombatSide::Ally, hex(0, -6), "advance", 1);
    let n2 = participant("n2", CombatSide::Ally, hex(6, -6), "advance", 1);
    let n3 = participant("n3", CombatSide::Ally, hex(6, 0), "advance", 1);
    let n4 = participant("n4", CombatSide::Ally, hex(-6, 6), "advance", 1);

    let forward = vec![
        center.clone(),
        n1.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
    ];
    let mut shuffled = vec![n3, n1, center, n4, n2];
    // A second, differently-shuffled order, so this isn't just testing one
    // arbitrary reversal.
    shuffled.reverse();

    let mut sim_forward = build(forward);
    let mut sim_shuffled = build(shuffled);
    assert_eq!(
        sim_forward.run_ticks(10).unwrap(),
        sim_shuffled.run_ticks(10).unwrap(),
        "input order must not change any tick's resolved frame, including occupancy \
         path-blocking and destination-contention decisions"
    );
}

// ---------------------------------------------------------------------
// §4-4 -- occupancy is read from the tick-start snapshot: deliberately
// conservative, pinned as intent (not a bug).
// ---------------------------------------------------------------------

#[test]
fn a_tile_vacated_this_tick_is_not_entered_this_tick() {
    // "leader" (enemy) sits at (1,0) and retreats away from "follower"
    // (ally) at (0,0) this tick, vacating (1,0). "follower" simultaneously
    // wants to close the entire gap onto "leader"'s tile (preferred_distance
    // 0). Both moves are computed from the same tick-start snapshot, in
    // which (1,0) still reads as occupied -- so "follower" must NOT move
    // into (1,0) this tick, even though by the tick's end "leader" isn't
    // there anymore.
    let leader = participant("leader", CombatSide::Enemy, hex(1, 0), "retreat", 1);
    let follower = participant("follower", CombatSide::Ally, hex(0, 0), "advance", 1);
    let mut sim = CombatSimulation::new(input(
        vec![leader, follower],
        // preferred_distance=5 with the leader-follower gap at 1 makes
        // d < preferred true, so the retreat branch (aggression < 0) fires.
        vec![role("retreat", 5, -1), role("advance", 0, 1)],
        vec![],
        3,
    ))
    .unwrap();
    let frame = sim.advance_tick().unwrap();

    let leader_move = frame.moves.iter().find(|m| m.actor_id == "leader").unwrap();
    let follower_move = frame
        .moves
        .iter()
        .find(|m| m.actor_id == "follower")
        .unwrap();
    assert_eq!(
        leader_move.to,
        hex(2, 0),
        "leader should have retreated, vacating (1,0)"
    );
    assert_eq!(
        follower_move.to,
        hex(0, 0),
        "follower must not enter (1,0) this tick even though leader is vacating it -- \
         occupancy is read from the tick-start snapshot (§4-4), not the in-progress state"
    );
}

// ---------------------------------------------------------------------
// §4-2② applies to retreat too, not just advance.
// ---------------------------------------------------------------------

#[test]
fn retreat_is_blocked_by_occupancy_too() {
    // "r" retreats away from "aggressor" (which sits far to its left), so it
    // would normally step further right. "blocker" (same side as "r", so it
    // is never a valid attack target and can't shift which enemy "r"
    // targets) sits immediately to its right and must stop the retreat
    // exactly like an occupied tile would stop an advance.
    let aggressor = participant("aggressor", CombatSide::Enemy, hex(0, 0), "hold", 1);
    let retreater = participant("r", CombatSide::Ally, hex(5, 0), "retreat", 2);
    let blocker = participant("blocker", CombatSide::Ally, hex(6, 0), "hold", 1);
    let mut sim = CombatSimulation::new(input(
        vec![aggressor, retreater, blocker],
        // distance(aggressor, r) = 5 < preferred_distance=10, aggression<0
        // => retreat branch.
        vec![role("retreat", 10, -1), role("hold", 0, 0)],
        vec![],
        3,
    ))
    .unwrap();
    let frame = sim.advance_tick().unwrap();
    let r_move = frame.moves.iter().find(|m| m.actor_id == "r").unwrap();
    assert_eq!(
        r_move.to,
        hex(5, 0),
        "retreat must stop before the occupied tile at (6,0), not pass through it to (7,0)"
    );
}

// ---------------------------------------------------------------------
// §4-6 -- surround detection: pure derivation, no threshold, no wiring.
// ---------------------------------------------------------------------

#[test]
fn surround_count_reports_enemy_occupied_neighbors_only() {
    let actor = participant("actor", CombatSide::Ally, hex(0, 0), "r", 1);
    // Two of the six neighbors are enemy-occupied, one is ally-occupied
    // (must be excluded), one is enemy but inactive (must be excluded), and
    // the remaining two are empty.
    let enemy_1 = participant("enemy_1", CombatSide::Enemy, hex(1, 0), "r", 1); // neighbor 0
    let enemy_2 = participant("enemy_2", CombatSide::Enemy, hex(0, -1), "r", 1); // neighbor 2
    let ally_neighbor = participant("ally_neighbor", CombatSide::Ally, hex(-1, 0), "r", 1); // neighbor 3
    let mut inactive_enemy = participant("inactive_enemy", CombatSide::Enemy, hex(0, 1), "r", 1); // neighbor 5
    inactive_enemy.active = false;

    let participants = vec![
        actor.clone(),
        enemy_1,
        enemy_2,
        ally_neighbor,
        inactive_enemy,
    ];
    let result = surrounding_enemy_neighbors(&actor, &participants).unwrap();
    assert_eq!(
        result,
        vec![hex(1, 0), hex(0, -1)],
        "only the two active-enemy-occupied neighbors count, in NEIGHBOR_DIRECTIONS order"
    );
    assert_eq!(result.len(), 2, "count is just the list length");
}

#[test]
fn surround_detection_is_not_wired_into_movement_or_targeting() {
    // Give "x" the exact same explicit target ("t", far away) in both
    // setups, so target selection is pinned identically either way. The
    // only difference between the two setups is that in the second, five of
    // "x"'s six neighbor tiles are filled with enemies (about as surrounded
    // as a single unit can be). If surround detection were wired into
    // movement or targeting anywhere, that difference would change "x"'s
    // computed move intent. It must not.
    let build_and_advance = |extra_enemies: Vec<CombatSimulationParticipant>| {
        let mut x = participant("x", CombatSide::Ally, hex(0, 0), "advance", 1);
        x.target_policy_id = Some("policy".into());
        let t = participant("t", CombatSide::Enemy, hex(10, 0), "hold", 1);
        let policy = CombatTargetPolicy {
            id: "policy".into(),
            preferences: vec![CombatTargetPreference {
                target_id: "t".into(),
                priority: 1,
            }],
            fallback: CombatTargetFallback::Nearest,
        };
        let mut parts = vec![x, t];
        parts.extend(extra_enemies);
        let mut sim = CombatSimulation::new(input(
            parts,
            vec![role("advance", 0, 1), role("hold", 0, 0)],
            vec![policy],
            3,
        ))
        .unwrap();
        let frame = sim.advance_tick().unwrap();
        frame.moves.into_iter().find(|m| m.actor_id == "x").unwrap()
    };

    let lonely = build_and_advance(vec![]);
    // Every neighbor direction *except* (1,0) -- the tile "x" actually
    // steps onto -- so occupancy itself can't be the reason the two cases
    // might differ; only surround-derived wiring could do that, and there
    // is none.
    let surrounded = build_and_advance(vec![
        participant("s1", CombatSide::Enemy, hex(1, -1), "hold", 1),
        participant("s2", CombatSide::Enemy, hex(0, -1), "hold", 1),
        participant("s3", CombatSide::Enemy, hex(-1, 0), "hold", 1),
        participant("s4", CombatSide::Enemy, hex(-1, 1), "hold", 1),
        participant("s5", CombatSide::Enemy, hex(0, 1), "hold", 1),
    ]);

    assert_eq!(
        lonely.to, surrounded.to,
        "being surrounded must not change x's computed destination -- surround detection isn't \
         wired into movement"
    );
    assert_eq!(lonely.mode, surrounded.mode);
    assert_eq!(
        lonely.target_id, surrounded.target_id,
        "being surrounded must not change x's selected target either"
    );
}
