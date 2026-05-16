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


def test_hidden_reality_hint_ending_includes_public_secret_summary(tmp_path):
    state = replace(
        GameState.new(seed=1, location_id="pantry"),
        inventory=["crumpled_printout"],
        flags=["printer_secret_started", "pantry_hint_seen"],
    )

    ending = evaluate_ending(state)

    assert ending is not None
    assert ending.id == "hidden_reality_hint_001"
    assert ending.kind == "hidden"
    assert ending.local_hint_id == "real_note_001"
    summary = format_ending_summary(ending, local_hint_path=tmp_path / "missing.yaml")
    assert "현실 연결 힌트: 첫 번째 현실 연결 힌트" in summary
    assert "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다." in summary
    assert "IP 주소: 192.168.0.42" in summary
    assert "숫자 합계: 33" in summary
    assert "로컬 비공개 파일" in summary


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
