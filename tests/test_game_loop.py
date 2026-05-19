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
