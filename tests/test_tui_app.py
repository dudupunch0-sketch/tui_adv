from tui_adv.tui.app import build_tui_turn, render_tui_layout_snapshot, resolve_tui_choice


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


def test_tui_choice_input_rejects_out_of_range_index_without_mutating_state():
    turn = build_tui_turn(seed=123)

    try:
        resolve_tui_choice(turn, 99)
    except ValueError as exc:
        assert "선택지를 찾을 수 없다: 99" in str(exc)
    else:
        raise AssertionError("expected invalid TUI choice to fail")

    assert turn.state.turn == 0
