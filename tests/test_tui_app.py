from dataclasses import replace

from tui_adv.game.loop import build_game_turn
from tui_adv.game.state import GameState
from tui_adv.tui.app import (
    build_tui_turn,
    render_tui_layout_snapshot,
    resolve_tui_action,
    resolve_tui_choice,
)


def test_tui_layout_snapshot_contains_status_encounter_choices_and_log_panel():
    turn = build_tui_turn(seed=123)

    snapshot = render_tui_layout_snapshot(turn)

    assert "[LOCAL STATUS]" in snapshot
    assert "인카운터: 퇴사자의 메신저" in snapshot
    assert "선택지" in snapshot
    assert "최근 로그" in snapshot
    assert "1. 메시지를 확인한다" in snapshot


def test_tui_choice_input_resolves_current_choice_and_ending():
    turn = build_tui_turn(
        seed=123,
        location_id="emergency_stairs",
        flags=("escape_puzzle_ready",),
    )

    resolved = resolve_tui_choice(turn, 1)

    assert resolved.choice_label == "반복되는 층수의 비밀을 풀고 문을 통과한다"
    assert resolved.ending is not None
    assert resolved.ending.id == "escape_commute"
    assert resolved.state.turn == 1


def test_tui_action_input_resolves_choice_then_movement_action():
    turn = build_tui_turn(seed=123)

    after_choice = resolve_tui_action(turn, 1)

    assert after_choice.action.id == "choice:1"
    assert after_choice.turn.state.turn == 1
    assert [action.id for action in after_choice.turn.available_actions] == ["move:dev_office"]
    movement_snapshot = render_tui_layout_snapshot(after_choice.turn)
    assert "[현재 행동]" in movement_snapshot
    assert "1. 개발팀 사무실" in movement_snapshot

    after_move = resolve_tui_action(after_choice.turn, 1)

    assert after_move.action.id == "move:dev_office"
    assert after_move.turn.state.location_id == "dev_office"
    assert after_move.turn.state.turn == 2


def test_tui_layout_snapshot_renders_inventory_and_clue_summary():
    state = replace(
        GameState.new(seed=123, location_id="pantry"),
        inventory=["crumpled_printout", "office_keycard", "flashlight"],
        clues=[
            "printer_ip_digits",
            "coffee_machine_back_panel",
            "server_log_fragment",
            "meeting_pattern_noticed",
        ],
    )
    turn = build_game_turn(state)

    snapshot = render_tui_layout_snapshot(turn)

    assert "[소지품]" in snapshot
    assert "- 구겨진 출력물 (crumpled_printout)" in snapshot
    assert "- office_keycard" in snapshot
    assert "- 손전등 (flashlight)" in snapshot
    assert "[단서]" in snapshot
    assert "- printer_ip_digits" in snapshot
    assert "- coffee_machine_back_panel" in snapshot
    assert "- server_log_fragment" in snapshot
    assert "+1 more" in snapshot
    assert "meeting_pattern_noticed" not in snapshot


def test_tui_layout_snapshot_renders_empty_inventory_and_clue_placeholders():
    turn = build_tui_turn(seed=123)

    snapshot = render_tui_layout_snapshot(turn)

    assert "[소지품]" in snapshot
    assert "- 없음" in snapshot
    assert "[단서]" in snapshot
    assert "- 아직 확보한 단서 없음" in snapshot


def test_tui_layout_snapshot_renders_hidden_reality_hint_ending_reward():
    state = replace(
        GameState.new(seed=123, location_id="pantry"),
        inventory=["crumpled_printout"],
        flags=["printer_secret_started", "pantry_hint_seen"],
    )
    turn = build_game_turn(state)

    snapshot = render_tui_layout_snapshot(turn)

    assert "[엔딩]" in snapshot
    assert "엔딩: 첫 번째 현실 연결 힌트" in snapshot
    assert "현실 연결 힌트: 첫 번째 현실 연결 힌트" in snapshot
    assert "숫자 합계: 33" in snapshot
    assert "[현재 인카운터]" not in snapshot


def test_tui_choice_input_rejects_out_of_range_index_without_mutating_state():
    turn = build_tui_turn(seed=123)

    try:
        resolve_tui_choice(turn, 99)
    except ValueError as exc:
        assert "선택지를 찾을 수 없다: 99" in str(exc)
    else:
        raise AssertionError("expected invalid TUI choice to fail")

    assert turn.state.turn == 0
