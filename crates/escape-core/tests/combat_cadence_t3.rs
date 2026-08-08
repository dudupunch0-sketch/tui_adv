// T3 (fable_combat_hex_t3_step1_2608080951.md, plan §6 WP4): pins the
// per-unit action-cadence gauge this slice adds. Every fixture in this file
// deliberately mirrors the pre-T3 shape of `combat_simulation_wave2.rs` /
// `combat_resolution_wave2.rs`'s own fixtures -- the point of these tests is
// to demonstrate the new cadence axis without disturbing the old one, so the
// "everything else" half of every fixture is copied straight from those
// files' conventions.

use escape_core::*;

// ---------------------------------------------------------------------
// Shared fixtures.
// ---------------------------------------------------------------------

fn manifest(combatant_ids: Vec<String>) -> CombatManifest {
    CombatManifest {
        simulation_version: CombatSimulationVersion::new(CURRENT_SIMULATION_VERSION).unwrap(),
        actual_seed: 1,
        world_state_fingerprint: "w".into(),
        applied_effects: vec![],
        suppressed_effects: vec![],
        combatant_ids,
        placement_ids: vec![],
        environment_ids: vec![],
        team_ids: vec![],
        rule_ids: vec![],
        public_info_ids: vec![],
    }
}

fn state(combatants: Vec<CombatantState>) -> CombatState {
    CombatState {
        battle_id: "b".into(),
        combatants,
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

fn combatant(id: &str, health: i32) -> CombatantState {
    CombatantState {
        id: id.into(),
        current_health: health,
        maximum_health: health,
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

#[allow(clippy::too_many_arguments)]
fn participant(
    id: &str,
    side: CombatSide,
    q: i32,
    role_id: &str,
    speed_per_tick: i32,
    move_speed_hundredths: Option<i64>,
) -> CombatSimulationParticipant {
    CombatSimulationParticipant {
        id: id.into(),
        side,
        position: HexCoord { q, r: 0 },
        facing: HexCoord { q: 1, r: 0 },
        speed_per_tick,
        move_speed_hundredths,
        collision_radius: 6,
        attack_range: 12,
        support_range: 2,
        role_id: role_id.into(),
        target_policy_id: None,
        active: true,
        occupies: vec![],
    }
}

fn sim_input(
    parts: Vec<CombatSimulationParticipant>,
    roles: Vec<CombatRolePreset>,
    max_ticks: u32,
) -> CombatSimulationInput {
    CombatSimulationInput {
        manifest: manifest(vec![]),
        state: state(vec![]),
        seed: 1,
        config: CombatSimulationConfig {
            tick_millis: 100,
            max_ticks,
        },
        participants: parts,
        roles,
        policies: vec![],
    }
}

/// A single mover (id "a", role "mover": preferred_distance 0, aggression 1,
/// so it always advances toward "e" while it acts at all) and a stationary
/// target (id "e", role "still": preferred_distance far larger than any
/// distance reached in these tests, aggression 0, so it never moves) far
/// enough apart that the mover never actually reaches "e" within the tick
/// counts used below -- isolating every assertion to the mover's own cadence.
fn mover_and_stationary_target(move_speed_hundredths: Option<i64>, max_ticks: u32) -> Simulation {
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, move_speed_hundredths);
    let e = participant("e", CombatSide::Enemy, 200, "still", 1, None);
    let roles = vec![role("mover", 0, 1), role("still", 999, 0)];
    Simulation(CombatSimulation::new(sim_input(vec![a, e], roles, max_ticks)).unwrap())
}

/// Thin wrapper so call sites read `sim.advance().0` for "a"'s displacement
/// this tick without repeating the lookup-by-id boilerplate everywhere.
struct Simulation(CombatSimulation);
impl Simulation {
    fn advance_displacement_of(&mut self, id: &str) -> i32 {
        let frame = self.0.advance_tick().unwrap();
        let intent = frame.moves.iter().find(|m| m.actor_id == id).unwrap();
        intent.to.q - intent.from.q
    }
}

fn defense(id: &str) -> CombatDefenseProfile {
    CombatDefenseProfile {
        combatant_id: id.into(),
        defense_hundredths: 0,
        balance_resistance_hundredths: 0,
    }
}

/// A resolution request with the given participants/attacks, an actor "a"
/// and target "e" (both tracked as combatants with generous health so
/// nothing dies mid-test unless a test wants it to), run for `max_ticks`.
fn resolution_request(
    participants: Vec<CombatSimulationParticipant>,
    attacks: Vec<CombatAttackDefinition>,
    max_ticks: u32,
) -> CombatResolutionRequest {
    let ids: Vec<String> = participants.iter().map(|p| p.id.clone()).collect();
    let roles = vec![role("mover", 0, 1), role("still", 999, 0)];
    CombatResolutionRequest {
        execution: CombatExecutionRequest {
            input: CombatSimulationInput {
                manifest: manifest(ids.clone()),
                // 1_000 -> 100_000 hundredths once `resolve()` scales it:
                // generous enough that nothing in this file's fixtures dies
                // mid-test, and round enough that
                // `existing_single_speed_combat_is_unchanged` can assert an
                // exact final value against it.
                state: state(ids.iter().map(|id| combatant(id, 1_000)).collect()),
                seed: 1,
                config: CombatSimulationConfig {
                    tick_millis: 100,
                    max_ticks,
                },
                participants,
                roles,
                policies: vec![],
            },
            mode: CombatRunMode::Actual,
            presentation: CombatPresentationSpeed::OneX,
            ticks: max_ticks,
        },
        attacks,
        defenses: ids.iter().map(|id| defense(id)).collect(),
        catalog: CombatEffectCatalog { effects: vec![] },
    }
}

#[allow(clippy::too_many_arguments)]
fn attack(
    id: &str,
    actor_id: &str,
    attack_speed_hundredths: Option<i64>,
) -> CombatAttackDefinition {
    CombatAttackDefinition {
        id: id.into(),
        actor_id: actor_id.into(),
        power_hundredths: 1200,
        ability_multiplier_hundredths: 100,
        accuracy_percent: 100,
        attack_range: 12,
        penetration_hundredths: 0,
        collision_balance_hundredths: 0,
        balance_power_hundredths: 0,
        attack_speed_hundredths,
        effects: vec![],
    }
}

// ---------------------------------------------------------------------
// §4-3 -- default and rejected speeds.
// ---------------------------------------------------------------------

#[test]
fn absent_speed_means_acting_every_tick() {
    let mut sim = mover_and_stationary_target(None, 4);
    for _ in 0..4 {
        assert_eq!(
            sim.advance_displacement_of("a"),
            1,
            "None must mean exactly today's behaviour: one action, every tick"
        );
    }
}

#[test]
fn zero_or_negative_speed_is_rejected() {
    for bad in [0i64, -1, -10_000] {
        let a = participant("a", CombatSide::Ally, 0, "mover", 1, Some(bad));
        let e = participant("e", CombatSide::Enemy, 200, "still", 1, None);
        let roles = vec![role("mover", 0, 1), role("still", 999, 0)];
        assert!(
            matches!(
                CombatSimulation::new(sim_input(vec![a, e], roles, 1)),
                Err(CombatSimulationError::InvalidParticipant(id)) if id == "a"
            ),
            "move_speed_hundredths = {bad} must be rejected, not treated as \"never acts\""
        );
    }
    for bad in [0i64, -1, -10_000] {
        let a = participant("a", CombatSide::Ally, 0, "mover", 1, None);
        let e = participant("e", CombatSide::Enemy, 5, "still", 1, None);
        let request = resolution_request(vec![a, e], vec![attack("atk", "a", Some(bad))], 1);
        assert!(
            matches!(
                resolve_combat(request),
                Err(CombatResolutionError::InvalidInput)
            ),
            "attack_speed_hundredths = {bad} must be rejected, not treated as \"never fires\""
        );
    }
}

// ---------------------------------------------------------------------
// §4-1 -- the gauge model itself.
// ---------------------------------------------------------------------

#[test]
fn half_speed_acts_every_other_tick() {
    // gauge sequence (threshold 10_000): 5000, 10000(-> act, 0), 5000,
    // 10000(-> act, 0), ... -- hold, move, hold, move.
    let mut sim = mover_and_stationary_target(Some(5_000), 4);
    let displacements: Vec<i32> = (0..4).map(|_| sim.advance_displacement_of("a")).collect();
    assert_eq!(displacements, vec![0, 1, 0, 1]);
}

#[test]
fn double_speed_acts_twice_in_one_tick() {
    // §4-1: a cadence gauge that crosses the threshold twice in one tick
    // must yield two actions that tick, not be clamped to one. With
    // speed_per_tick = 1 and move_speed_hundredths = 20_000, every tick's
    // gauge (0 + 20_000 = 20_000) crosses 10_000 exactly twice, so the
    // actor's single combined move intent this tick must cover 2 tiles, not
    // 1 -- clamping to "at most once per tick" would silently produce 1
    // here instead.
    let mut sim = mover_and_stationary_target(Some(20_000), 3);
    for _ in 0..3 {
        assert_eq!(
            sim.advance_displacement_of("a"),
            2,
            "a speed of 20_000 (double the 10_000 threshold) must move 2 tiles this tick, not be clamped to 1"
        );
    }
}

#[test]
fn a_non_integer_multiple_speed_drifts_deterministically() {
    // Hand-computed gauge trace for speed 6_600 against threshold 10_000,
    // starting from gauge 0 (§4-1's "don't invent a rounding rule" -- this
    // is just repeated addition and threshold-crossing, no floating point):
    //   tick 1: 0     + 6600 = 6600  (< 10000)            -> 0 actions, gauge 6600
    //   tick 2: 6600  + 6600 = 13200 (>= 10000 once)       -> 1 action,  gauge 3200
    //   tick 3: 3200  + 6600 = 9800  (< 10000)             -> 0 actions, gauge 9800
    //   tick 4: 9800  + 6600 = 16400 (>= 10000 once)       -> 1 action,  gauge 6400
    //   tick 5: 6400  + 6600 = 13000 (>= 10000 once)       -> 1 action,  gauge 3000
    //   tick 6: 3000  + 6600 = 9600  (< 10000)             -> 0 actions, gauge 9600
    //   tick 7: 9600  + 6600 = 16200 (>= 10000 once)       -> 1 action,  gauge 6200
    // This is not a fixed period (unlike half speed's strict alternation) --
    // that non-repeating drift is exactly the gauge model's advantage over a
    // fixed "every N ticks" interval scheme (plan §4-1's motivation).
    let expected = [0, 1, 0, 1, 1, 0, 1];
    let mut sim = mover_and_stationary_target(Some(6_600), 7);
    let displacements: Vec<i32> = (0..7).map(|_| sim.advance_displacement_of("a")).collect();
    assert_eq!(displacements, expected);

    // Determinism: an independent second run of the identical input produces
    // the identical sequence.
    let mut sim2 = mover_and_stationary_target(Some(6_600), 7);
    let displacements2: Vec<i32> = (0..7).map(|_| sim2.advance_displacement_of("a")).collect();
    assert_eq!(displacements, displacements2);
}

// ---------------------------------------------------------------------
// §4-2 -- movement and attack cadence are independent axes.
// ---------------------------------------------------------------------

#[test]
fn move_and_attack_cadences_are_independent() {
    // "a" moves at half speed (hold, move, hold, move, ...) but its one
    // attack fires every tick (attack_speed_hundredths: None) against a
    // stationary "e" that starts within range and stays within range no
    // matter how far "a" advances (attack_range/collision generous enough
    // to cover the whole starting distance). If the two cadences were
    // wrongly coupled, the attack would go silent on the ticks "a" doesn't
    // move; it must not.
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, Some(5_000));
    let e = participant("e", CombatSide::Enemy, 10, "still", 1, None);
    let request = resolution_request(vec![a, e], vec![attack("atk", "a", None)], 4);
    let result = resolve_combat(request).unwrap();
    assert_eq!(result.frames.len(), 4);
    for (tick, frame) in result.frames.iter().enumerate() {
        assert_eq!(
            frame.outcomes.len(),
            1,
            "tick {}: the attack must fire every tick regardless of the mover's own cadence",
            tick + 1
        );
    }
    // Confirm the mover's own cadence really did stay half-speed underneath
    // (i.e. this isn't vacuously true because the mover never held at all).
    let displacements: Vec<i32> = result
        .execution
        .frames
        .iter()
        .map(|frame| {
            let intent = frame.moves.iter().find(|m| m.actor_id == "a").unwrap();
            intent.to.q - intent.from.q
        })
        .collect();
    assert_eq!(displacements, vec![0, 1, 0, 1]);
}

// ---------------------------------------------------------------------
// §4-4 -- order independence.
// ---------------------------------------------------------------------

#[test]
fn who_acts_this_tick_is_decided_before_anyone_acts() {
    // Two attacks on the same actor with different cadences (fires every
    // tick vs. every other tick). Each attack's fire count this tick must
    // come only from its own tick-start gauge, never from the other
    // attack's processing -- reversing their order in the request must not
    // change a single outcome, log entry, or fingerprint.
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, None);
    let e = participant("e", CombatSide::Enemy, 5, "still", 1, None);
    let fast = attack("fast", "a", None);
    let slow = attack("slow", "a", Some(5_000));

    let forward = resolve_combat(resolution_request(
        vec![a.clone(), e.clone()],
        vec![fast.clone(), slow.clone()],
        4,
    ))
    .unwrap();
    let reversed = resolve_combat(resolution_request(vec![a, e], vec![slow, fast], 4)).unwrap();

    assert_eq!(forward.frames, reversed.frames);
    assert_eq!(forward.fingerprint, reversed.fingerprint);
}

#[test]
fn shuffled_participant_order_yields_identical_frames() {
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, Some(20_000));
    let e = participant("e", CombatSide::Enemy, 200, "still", 1, Some(5_000));
    let roles = vec![role("mover", 0, 1), role("still", 999, 0)];

    let forward = sim_input(vec![a.clone(), e.clone()], roles.clone(), 5);
    let mut reversed = forward.clone();
    reversed.participants.reverse();

    let mut sa = CombatSimulation::new(forward).unwrap();
    let mut sb = CombatSimulation::new(reversed).unwrap();
    assert_eq!(sa.run_ticks(5).unwrap(), sb.run_ticks(5).unwrap());
}

// ---------------------------------------------------------------------
// §4-5 -- no leak, including the multi-fire-per-tick path.
// ---------------------------------------------------------------------

#[test]
fn attack_speed_never_appears_in_any_log_or_view() {
    // attack_speed_hundredths = 30_000 fires 3 times in a single tick
    // (0 + 30_000 crosses the 10_000 threshold three times) -- the
    // multi-fire path WP2 added, not just the default single-fire path
    // `combat_spectator_wave3.rs`'s leak test already covers. Repeated
    // fires are allowed to be visible as repeated log entries/outcomes; the
    // raw speed number and the word "speed" are not allowed to appear
    // anywhere in the serialized output.
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, None);
    let e = participant("e", CombatSide::Enemy, 5, "still", 1, None);
    let request = resolution_request(
        vec![a.clone(), e.clone()],
        vec![attack("atk", "a", Some(30_000))],
        1,
    );
    let resolution = resolve_combat(request).unwrap();
    assert_eq!(
        resolution.frames[0].outcomes.len(),
        3,
        "sanity check: the multi-fire path actually fired 3 times this tick"
    );

    let resolution_json = serde_json::to_string(&resolution).unwrap();
    assert!(!resolution_json.contains("30000"));

    let view = spectate_combat(&CombatSpectatorRequest {
        resolution,
        participants: vec![a, e],
        catalog: CombatEffectCatalog { effects: vec![] },
    })
    .unwrap();
    let view_json = serde_json::to_string(&view).unwrap();
    assert!(!view_json.contains("30000"));
    assert!(!view_json.to_lowercase().contains("speed"));
}

// ---------------------------------------------------------------------
// Hard invariants 1/2 -- nothing existing moved.
// ---------------------------------------------------------------------

#[test]
fn speed_fields_are_absent_from_json_when_unset() {
    let p = participant("a", CombatSide::Ally, 0, "mover", 1, None);
    let p_value = serde_json::to_value(&p).unwrap();
    assert!(
        p_value.get("move_speed_hundredths").is_none(),
        "unset move_speed_hundredths must not appear as a JSON key at all: {p_value}"
    );

    let a = attack("atk", "a", None);
    let a_value = serde_json::to_value(&a).unwrap();
    assert!(
        a_value.get("attack_speed_hundredths").is_none(),
        "unset attack_speed_hundredths must not appear as a JSON key at all: {a_value}"
    );

    // The reverse must also hold: a JSON object that looks exactly like an
    // old (pre-T3) bundle -- no speed keys present at all -- must still
    // deserialize, defaulting both fields to None (additive-optional,
    // invariant 2).
    let mut old_shaped = p_value.clone();
    old_shaped.as_object_mut().unwrap().remove("occupies");
    let restored: CombatSimulationParticipant = serde_json::from_value(old_shaped).unwrap();
    assert_eq!(restored.move_speed_hundredths, None);

    // And when a speed *is* set, it round-trips as the literal value under
    // its own key -- confirming the field works, not just that it's absent.
    let mut fast = p;
    fast.move_speed_hundredths = Some(12_345);
    let fast_value = serde_json::to_value(&fast).unwrap();
    assert_eq!(fast_value["move_speed_hundredths"], 12_345);
}

#[test]
fn existing_single_speed_combat_is_unchanged() {
    // Both speeds unset (today's only configuration) must reproduce the
    // pre-T3 damage formula exactly: power 1200, ability multiplier 100,
    // zero defense/penetration ->
    //   pre = 1200 * 5 * 100 / 1200 = 500
    //   reduction = pre * 0 / (0 + 2000) = 0
    //   damage = 500, every tick, for 3 ticks.
    let a = participant("a", CombatSide::Ally, 0, "mover", 1, None);
    let e = participant("e", CombatSide::Enemy, 1, "still", 1, None);
    let request = resolution_request(vec![a, e], vec![attack("atk", "a", None)], 3);
    let resolution = resolve_combat(request).unwrap();
    assert_eq!(resolution.frames.len(), 3);
    for frame in &resolution.frames {
        assert_eq!(frame.outcomes.len(), 1);
        let outcome = &frame.outcomes[0];
        assert!(outcome.hit);
        assert_eq!(outcome.damage_hundredths, 500);
    }
    let final_health = resolution
        .state
        .combatants
        .iter()
        .find(|c| c.id == "e")
        .unwrap()
        .current_health_hundredths;
    assert_eq!(final_health, 100_000 - 3 * 500);
}
