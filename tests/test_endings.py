from dataclasses import replace

from tui_adv.game.encounters import DEFAULT_ENCOUNTERS
from tui_adv.game.endings import evaluate_ending, format_ending_summary
from tui_adv.game.state import GameState, PlayerState


def test_spatial_exit_puzzle_success_triggers_first_escape_ending():
    state = replace(
        GameState.new(seed=1, location_id="emergency_stairs"),
        flags=["escape_puzzle_ready"],
    )
    encounter = DEFAULT_ENCOUNTERS["spatial_exit_puzzle"]

    escaped = encounter.resolve_choice("solve_distorted_floor", state)
    ending = evaluate_ending(escaped)

    assert ending is not None
    assert ending.id == "escape_commute"
    assert ending.kind == "escape"
    assert "공간 왜곡" in ending.text
    assert "퇴근" in format_ending_summary(ending)


def test_spatial_exit_puzzle_failure_triggers_game_over():
    state = replace(
        GameState.new(seed=1, location_id="emergency_stairs"),
        flags=["escape_puzzle_ready"],
    )
    encounter = DEFAULT_ENCOUNTERS["spatial_exit_puzzle"]

    failed = encounter.resolve_choice("walk_down_wrong_stairs", state)
    ending = evaluate_ending(failed)

    assert ending is not None
    assert ending.id == "game_over_spatial_collapse"
    assert ending.kind == "failure"
    assert "게임오버" in format_ending_summary(ending)


def test_resource_failure_takes_priority_over_escape_flags():
    state = replace(
        GameState.new(seed=1, location_id="emergency_stairs").with_player(
            PlayerState(health=0)
        ),
        flags=["escape_route_completed"],
    )

    ending = evaluate_ending(state)

    assert ending is not None
    assert ending.id == "game_over_health_depleted"
    assert ending.kind == "failure"
