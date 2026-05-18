from dataclasses import replace

import pytest

from tui_adv.game.save import load_game_state, save_game_state
from tui_adv.game.state import GameState, PlayerState


def test_save_game_state_round_trips_full_state(tmp_path):
    state = replace(
        GameState.new(seed=7, location_id="pantry"),
        turn=4,
        disaster_type="backrooms_shift",
        danger=3,
        player=PlayerState(
            health=72,
            sanity=61,
            battery=18,
            hunger=12,
            thirst=24,
            abilities={"logic": 5, "interface": 4},
        ),
        inventory=["crumpled_printout", "office_keycard"],
        clues=["printer_ip_digits"],
        flags=["printer_secret_started", "pantry_hint_seen"],
        seen_encounters=["printer_ip_label", "pantry_reality_hint"],
        unlocked_achievements=["first_signal_received", "reality_link_discovered"],
        log=["따뜻한 출력물을 접어 주머니에 넣었다.", "탕비실로 이동했다."],
    )
    save_path = tmp_path / "office-save.json"

    save_game_state(state, save_path)
    loaded = load_game_state(save_path)

    assert loaded == state


def test_load_game_state_rejects_unsupported_schema_version(tmp_path):
    save_path = tmp_path / "future-save.json"
    save_path.write_text('{"schema_version": 999, "state": {}}', encoding="utf-8")

    with pytest.raises(ValueError, match="지원하지 않는 저장 파일 버전"):
        load_game_state(save_path)


def test_load_game_state_rejects_non_mapping_json(tmp_path):
    save_path = tmp_path / "not-a-save.json"
    save_path.write_text("[]", encoding="utf-8")

    with pytest.raises(ValueError, match="저장 파일 루트는 객체여야 한다"):
        load_game_state(save_path)
