from dataclasses import replace

from tui_adv.game.loop import build_game_turn, resolve_turn_action, resolve_turn_action_result
from tui_adv.game.state import GameState, PlayerState


def test_game_turn_shows_encounter_first_then_moves_after_seen_encounter():
    initial_turn = build_game_turn(GameState.new(seed=123))

    assert initial_turn.encounter is not None
    assert initial_turn.encounter.id == "ex_employee_messenger"
    assert initial_turn.available_move_ids == ()
    assert [action.id for action in initial_turn.available_actions] == [
        "choice:1",
        "choice:2",
        "choice:3",
    ]

    after_choice = resolve_turn_action(initial_turn, "choice:1")

    assert after_choice.state.turn == 1
    assert after_choice.encounter is None
    assert after_choice.available_move_ids == ("dev_office",)
    assert [action.id for action in after_choice.available_actions] == ["move:dev_office"]


def test_multi_turn_actions_can_reach_conquest_ending_through_movement_and_encounter():
    turn = build_game_turn(GameState.new(seed=123))

    for action in (
        "choice:1",
        "move:dev_office",
        "move:hallway",
        "move:server_room_front",
        "choice:1",
    ):
        turn = resolve_turn_action(turn, action)

    assert turn.state.turn == 5
    assert turn.state.location_id == "server_room_front"
    assert turn.ending is not None
    assert turn.ending.id == "conquest_broadcast_channel"
    assert turn.available_actions == ()


def test_invalid_turn_action_is_rejected_without_mutating_state():
    turn = build_game_turn(GameState.new(seed=123))

    try:
        resolve_turn_action(turn, "move:hallway")
    except ValueError as exc:
        assert "지금 실행할 수 없는 행동: move:hallway" in str(exc)
    else:
        raise AssertionError("expected invalid action to fail")

    assert turn.state.turn == 0
    assert turn.state.location_id == "dev_desk"
    assert turn.state.log == []


def test_game_turn_allows_using_inventory_item_between_moves():
    state = replace(
        GameState.new(seed=123, location_id="pantry"),
        player=PlayerState(thirst=70),
        inventory=["bottled_water"],
        seen_encounters=["pantry_coffee_machine"],
    )
    turn = build_game_turn(state)

    assert "use:bottled_water" in [action.id for action in turn.available_actions]

    result = resolve_turn_action_result(turn, "use:bottled_water")

    assert result.action.kind == "item"
    assert result.turn.state.turn == 1
    assert result.turn.state.inventory == []
    assert result.turn.state.player.thirst == 37
    assert "생수" in result.turn.state.log[-1]


def test_high_thirst_pantry_turn_surfaces_water_dispenser_hallucination():
    state = replace(
        GameState.new(seed=123, location_id="pantry").with_player(PlayerState(thirst=70)),
        seen_encounters=["pantry_coffee_machine"],
    )
    turn = build_game_turn(state)

    assert turn.encounter is not None
    assert turn.encounter.id == "strange_water_dispenser"

    result = resolve_turn_action_result(turn, "choice:1")

    assert result.choice_resolution is not None
    assert result.turn.state.player.thirst == 47
    assert result.turn.state.player.sanity == 92
    assert "thirst_hallucination_seen" in result.turn.state.flags


def test_security_floor_mismatch_can_unlock_lockdown_conquest_route():
    turn = build_game_turn(GameState.new(seed=123, location_id="elevator_hall"))

    for action in (
        "choice:2",
        "choice:1",
        "move:hallway",
        "move:server_room_front",
        "choice:4",
        "choice:3",
    ):
        turn = resolve_turn_action(turn, action)

    assert turn.state.location_id == "server_room"
    assert "security_override_badge" in turn.state.inventory
    assert "security_override_unlocked" in turn.state.flags
    assert "security_lockdown_claimed" in turn.state.flags
    assert turn.ending is not None
    assert turn.ending.id == "conquest_security_lockdown"


def test_parking_lot_key_and_ramp_sequence_reaches_escape_ending():
    turn = build_game_turn(GameState.new(seed=123))

    for action in (
        "choice:1",
        "move:dev_office",
        "move:hallway",
        "move:parking_lot",
        "choice:1",
        "choice:1",
    ):
        turn = resolve_turn_action(turn, action)

    assert turn.state.location_id == "parking_lot"
    assert "parking_key_fob" in turn.state.inventory
    assert "parking_key_found" in turn.state.flags
    assert "parking_ramp_opened" in turn.state.flags
    assert "parking_exit_route" in turn.state.clues
    assert turn.ending is not None
    assert turn.ending.id == "escape_parking_lot"


def test_forcing_elevator_doors_returns_through_security_room_with_clue():
    turn = build_game_turn(GameState.new(seed=123, location_id="elevator_hall"))

    result = resolve_turn_action_result(turn, "choice:2")

    assert result.choice_resolution is not None
    assert result.turn.state.location_id == "security_room"
    assert "security_floor_misalignment" in result.turn.state.clues
    assert "elevator_returned_wrong_floor" in result.turn.state.flags
