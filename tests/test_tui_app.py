import os
from dataclasses import replace

from tui_adv.game.loop import GameTurn, TurnAction, build_game_turn
from tui_adv.game.save import load_game_state, save_game_state
from tui_adv.game.state import GameState
from tui_adv.tui.app import (
    build_tui_turn,
    delete_tui_save_slot,
    discover_save_slots,
    movement_shortcuts_for_turn,
    render_tui_layout_snapshot,
    resolve_tui_action,
    resolve_tui_choice,
    resolve_tui_key,
    resolve_tui_save_slot,
    save_tui_turn_state,
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


def test_tui_layout_snapshot_renders_help_panel_and_movement_shortcuts():
    turn = build_tui_turn(seed=123)
    after_choice = resolve_tui_action(turn, 1)

    snapshot = render_tui_layout_snapshot(after_choice.turn)

    assert "[도움말]" in snapshot
    assert "숫자: 현재 선택/행동 실행" in snapshot
    assert "?: 도움말" in snapshot
    assert "i: 소지품/단서" in snapshot
    assert "l: 최근 로그" in snapshot
    assert "이동 단축키: a=개발팀 사무실" in snapshot


def test_tui_detail_help_panel_explains_non_gameplay_keys():
    turn = build_tui_turn(seed=123)
    after_choice = resolve_tui_action(turn, 1)

    snapshot = render_tui_layout_snapshot(after_choice.turn, detail_panel="help")

    assert "[상세 도움말]" in snapshot
    assert "숫자: 현재 선택지, 이동, 소지품 사용을 실행한다." in snapshot
    assert "i: 소지품/단서 상세" in snapshot
    assert "l: 최근 로그 상세" in snapshot
    assert "s: 현재 저장 경로에 저장" in snapshot
    assert "q: 종료" in snapshot


def test_tui_letter_key_resolves_movement_shortcut():
    turn = build_tui_turn(seed=123)
    after_choice = resolve_tui_action(turn, 1)

    shortcuts = movement_shortcuts_for_turn(after_choice.turn)
    resolved = resolve_tui_key(after_choice.turn, "a")

    assert shortcuts["a"].id == "move:dev_office"
    assert resolved.action.id == "move:dev_office"
    assert resolved.turn.state.location_id == "dev_office"


def test_tui_movement_shortcuts_skip_reserved_keys_without_dropping_actions():
    actions = tuple(
        TurnAction(
            id=f"move:room_{index}",
            label=f"room {index}",
            kind="move",
            target_id=f"room_{index}",
        )
        for index in range(9)
    )
    turn = GameTurn(
        state=GameState.new(seed=123),
        encounter=None,
        ending=None,
        available_move_ids=tuple(action.target_id for action in actions),
        available_actions=actions,
    )

    shortcuts = movement_shortcuts_for_turn(turn)

    assert "l" not in shortcuts
    assert list(shortcuts)[:9] == ["a", "d", "f", "g", "h", "j", "k", "z", "x"]
    assert shortcuts["z"].id == "move:room_7"
    assert shortcuts["x"].id == "move:room_8"


def test_tui_unknown_letter_key_rejects_without_mutating_state():
    turn = build_tui_turn(seed=123)
    after_choice = resolve_tui_action(turn, 1)

    try:
        resolve_tui_key(after_choice.turn, "?")
    except ValueError as exc:
        assert "행동 단축키를 찾을 수 없다: ?" in str(exc)
    else:
        raise AssertionError("expected unknown TUI shortcut to fail")

    assert after_choice.turn.state.location_id == "dev_desk"


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


def test_tui_inventory_detail_panel_renders_all_items_clues_and_item_descriptions():
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

    snapshot = render_tui_layout_snapshot(turn, detail_panel="inventory")

    assert "[상세 소지품]" in snapshot
    detail_section = snapshot.split("[상세 소지품]", 1)[1]
    assert "구겨진 출력물 (crumpled_printout):" in detail_section
    assert "작은 손전등. 회사 지급품은 아니고 누군가의 개인 물건 같다." in detail_section
    assert "[상세 단서]" in detail_section
    assert "- meeting_pattern_noticed" in detail_section
    assert "+1 more" not in detail_section


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


def test_tui_layout_snapshot_renders_save_controls_when_path_is_configured(tmp_path):
    turn = build_tui_turn(seed=123)
    save_path = tmp_path / "office-save.json"

    snapshot = render_tui_layout_snapshot(turn, save_path=save_path)

    assert "[저장]" in snapshot
    assert f"저장 파일: {save_path}" in snapshot
    assert "s: 저장" in snapshot
    assert "q: 종료" in snapshot


def test_tui_save_slot_discovery_sorts_recent_json_saves(tmp_path):
    old_save = tmp_path / "old.json"
    recent_save = tmp_path / "recent.json"
    broken_save = tmp_path / "broken.json"
    save_game_state(GameState.new(seed=123, location_id="pantry"), old_save)
    save_game_state(GameState.new(seed=123, location_id="lobby"), recent_save)
    broken_save.write_text("not json", encoding="utf-8")
    old_time = 1_700_000_000
    recent_time = old_time + 10
    broken_time = old_time + 20
    os.utime(old_save, (old_time, old_time))
    os.utime(recent_save, (recent_time, recent_time))
    os.utime(broken_save, (broken_time, broken_time))

    slots = discover_save_slots(tmp_path)

    assert [slot.path.name for slot in slots] == ["broken.json", "recent.json", "old.json"]
    assert slots[0].error is not None
    assert slots[1].turn == 0
    assert slots[1].location_name == "로비"


def test_tui_save_slot_discovery_marks_schema_invalid_json_as_unreadable(tmp_path):
    invalid_save = tmp_path / "invalid-schema.json"
    invalid_save.write_text('{"schema_version": 1, "state": {}}', encoding="utf-8")

    slots = discover_save_slots(tmp_path)

    assert len(slots) == 1
    assert slots[0].path.name == "invalid-schema.json"
    assert slots[0].error is not None


def test_tui_save_slot_discovery_marks_overflowing_json_number_as_unreadable(tmp_path):
    invalid_save = tmp_path / "overflowing-number.json"
    invalid_save.write_text(
        """
{
  "schema_version": 1,
  "state": {
    "seed": 1e10000,
    "turn": 0,
    "location_id": "lobby",
    "disaster_type": "unknown_isolation",
    "danger": 0,
    "player": {
      "health": 100,
      "sanity": 100,
      "battery": 100,
      "hunger": 0,
      "thirst": 0,
      "abilities": {}
    }
  }
}
""",
        encoding="utf-8",
    )

    slots = discover_save_slots(tmp_path)

    assert len(slots) == 1
    assert slots[0].path.name == "overflowing-number.json"
    assert slots[0].error is not None


def test_tui_save_slot_discovery_limit_applies_to_broken_slots(tmp_path):
    base_time = 1_700_000_000
    for index in range(5):
        path = tmp_path / f"broken-{index}.json"
        path.write_text("not json", encoding="utf-8")
        slot_time = base_time + index
        os.utime(path, (slot_time, slot_time))

    slots = discover_save_slots(tmp_path, limit=3)

    assert [slot.path.name for slot in slots] == [
        "broken-4.json",
        "broken-3.json",
        "broken-2.json",
    ]
    assert all(slot.error is not None for slot in slots)


def test_tui_layout_snapshot_renders_save_slot_list(tmp_path):
    save_path = tmp_path / "office-save.json"
    save_game_state(GameState.new(seed=123, location_id="lobby"), save_path)
    turn = build_tui_turn(seed=123)

    snapshot = render_tui_layout_snapshot(
        turn,
        save_slots=discover_save_slots(tmp_path),
    )

    assert "[저장 파일 목록]" in snapshot
    assert "1. office-save.json — 턴 0 / 로비" in snapshot


def test_tui_start_snapshot_prompts_numbered_save_slot_loading(tmp_path):
    save_path = tmp_path / "office-save.json"
    save_game_state(GameState.new(seed=123, location_id="lobby"), save_path)
    turn = build_tui_turn(seed=123)

    snapshot = render_tui_layout_snapshot(
        turn,
        save_path=tmp_path / "autosave.json",
        save_slots=discover_save_slots(tmp_path),
        start_mode=True,
    )

    assert "[시작]" in snapshot
    assert "숫자: 저장 파일 불러오기" in snapshot
    assert "n: 새 게임" in snapshot
    assert "d: 저장 파일 삭제 모드" in snapshot
    assert "1. office-save.json — 턴 0 / 로비" in snapshot


def test_tui_start_snapshot_prompts_save_slot_deletion_when_delete_mode_is_active(tmp_path):
    save_path = tmp_path / "office-save.json"
    save_game_state(GameState.new(seed=123, location_id="lobby"), save_path)
    turn = build_tui_turn(seed=123)

    snapshot = render_tui_layout_snapshot(
        turn,
        save_path=tmp_path / "autosave.json",
        save_slots=discover_save_slots(tmp_path),
        start_mode=True,
        delete_slot_mode=True,
    )

    assert "[시작]" in snapshot
    assert "숫자: 저장 파일 삭제" in snapshot
    assert "숫자: 저장 파일 불러오기" not in snapshot
    assert "d: 저장 파일 삭제 모드" not in snapshot
    assert "n: 새 게임" in snapshot
    assert "1. office-save.json — 턴 0 / 로비" in snapshot


def test_tui_save_slot_selection_loads_numbered_save(tmp_path):
    save_path = tmp_path / "office-save.json"
    save_game_state(GameState.new(seed=123, location_id="lobby"), save_path)
    slots = discover_save_slots(tmp_path)

    loaded_state = resolve_tui_save_slot(slots, 1)

    assert loaded_state.location_id == "lobby"


def test_tui_save_slot_deletion_removes_selected_file_and_keeps_other_slots(tmp_path):
    old_save = tmp_path / "old.json"
    recent_save = tmp_path / "recent.json"
    save_game_state(GameState.new(seed=123, location_id="pantry"), old_save)
    save_game_state(GameState.new(seed=123, location_id="lobby"), recent_save)
    os.utime(old_save, (1_700_000_000, 1_700_000_000))
    os.utime(recent_save, (1_700_000_010, 1_700_000_010))
    slots = discover_save_slots(tmp_path)

    deleted_path = delete_tui_save_slot(slots, 1)

    assert deleted_path == recent_save
    assert not recent_save.exists()
    assert old_save.exists()
    assert [slot.path.name for slot in discover_save_slots(tmp_path)] == ["old.json"]


def test_tui_save_slot_selection_rejects_broken_slot(tmp_path):
    broken_save = tmp_path / "broken.json"
    broken_save.write_text("not json", encoding="utf-8")
    slots = discover_save_slots(tmp_path)

    try:
        resolve_tui_save_slot(slots, 1)
    except ValueError as exc:
        assert "저장 슬롯을 읽을 수 없다: broken.json" in str(exc)
    else:
        raise AssertionError("expected broken save slot selection to fail")


def test_tui_layout_snapshot_renders_pressure_warning_panel():
    state = GameState.new(seed=123).with_player(
        replace(GameState.new(seed=123).player, sanity=30, battery=12, hunger=82, thirst=70)
    )
    turn = build_game_turn(state)

    snapshot = render_tui_layout_snapshot(turn)

    assert "[압박 경고]" in snapshot
    assert "선택지 왜곡" in snapshot
    assert "단말기 전원" in snapshot
    assert "정수기 환청" in snapshot
    assert "영양 상태" in snapshot


def test_tui_log_detail_panel_renders_more_than_recent_five_entries():
    state = replace(
        GameState.new(seed=123),
        log=[f"log-{index}" for index in range(7)],
    )
    turn = build_game_turn(state)

    summary_snapshot = render_tui_layout_snapshot(turn)
    detail_snapshot = render_tui_layout_snapshot(turn, detail_panel="log")

    assert "- log-0" not in summary_snapshot
    assert "- log-6" in summary_snapshot
    assert "[상세 로그]" in detail_snapshot
    assert "1. log-0" in detail_snapshot
    assert "7. log-6" in detail_snapshot


def test_tui_save_turn_state_writes_file_and_appends_log(tmp_path):
    turn = build_tui_turn(seed=123)
    save_path = tmp_path / "office-save.json"

    saved_turn = save_tui_turn_state(turn, save_path)

    loaded_state = load_game_state(save_path)
    assert saved_turn.state.log[-1] == f"저장: {save_path}"
    assert loaded_state == saved_turn.state


def test_tui_choice_input_rejects_out_of_range_index_without_mutating_state():
    turn = build_tui_turn(seed=123)

    try:
        resolve_tui_choice(turn, 99)
    except ValueError as exc:
        assert "선택지를 찾을 수 없다: 99" in str(exc)
    else:
        raise AssertionError("expected invalid TUI choice to fail")

    assert turn.state.turn == 0
