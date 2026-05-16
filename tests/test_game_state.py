from tui_adv.game.state import GameState, PlayerState


def test_player_state_defaults_match_initial_design():
    player = PlayerState()

    assert player.health == 100
    assert player.sanity == 100
    assert player.battery == 100
    assert player.hunger == 0
    assert player.thirst == 0


def test_player_state_delta_clamps_resource_bounds():
    player = PlayerState(health=5, sanity=95, battery=50, hunger=98, thirst=1)

    updated = player.apply_delta(
        health=-10,
        sanity=20,
        battery=-60,
        hunger=10,
        thirst=-5,
    )

    assert updated.health == 0
    assert updated.sanity == 100
    assert updated.battery == 0
    assert updated.hunger == 100
    assert updated.thirst == 0


def test_new_game_uses_seeded_unknown_isolation_start_state():
    state = GameState.new(seed=12345)

    assert state.seed == 12345
    assert state.turn == 0
    assert state.location_id == "dev_desk"
    assert state.disaster_type == "unknown_isolation"
    assert state.danger == 0
    assert state.player == PlayerState()
    assert state.inventory == []
    assert state.clues == []
    assert state.flags == []
    assert state.log == []


def test_advance_turn_applies_basic_survival_pressure():
    state = GameState.new(seed=1)

    next_state = state.advance_turn()

    assert next_state.turn == 1
    assert next_state.player.hunger == 1
    assert next_state.player.thirst == 2
    assert state.turn == 0
    assert state.player.hunger == 0
    assert state.player.thirst == 0


def test_failure_reason_reports_immediate_resource_collapse():
    assert PlayerState(health=0).failure_reason == "health_depleted"
    assert PlayerState(sanity=0).failure_reason == "sanity_depleted"
    assert PlayerState(health=0, sanity=0).failure_reason == "health_depleted"
    assert GameState.new(seed=1).failure_reason is None


def test_hunger_and_thirst_limits_apply_damage_during_turn_pressure():
    state = GameState.new(
        seed=1,
    )
    state = state.with_player(
        PlayerState(health=20, sanity=20, hunger=99, thirst=98)
    )

    next_state = state.advance_turn()

    assert next_state.player.hunger == 100
    assert next_state.player.thirst == 100
    assert next_state.player.health == 14
    assert next_state.player.sanity == 18
    assert next_state.failure_reason is None


def test_low_resources_expose_choice_rule_hooks():
    stable = PlayerState(sanity=40, thirst=59, battery=1)
    distorted = PlayerState(sanity=39, thirst=60, battery=0)

    assert stable.should_distort_choices is False
    assert stable.should_trigger_thirst_hallucination is False
    assert stable.can_spend_battery(1) is True
    assert stable.can_spend_battery(2) is False

    assert distorted.should_distort_choices is True
    assert distorted.should_trigger_thirst_hallucination is True
    assert distorted.can_spend_battery(1) is False
