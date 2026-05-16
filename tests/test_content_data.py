import pytest

from tui_adv.game.content import (
    DATA_DIR,
    load_default_achievements,
    load_default_encounters,
    load_default_endings,
    load_default_locations,
    load_locations,
    validate_public_content,
)
from tui_adv.game.encounters import DEFAULT_ENCOUNTERS, select_encounter
from tui_adv.game.endings import DEFAULT_ENDINGS, evaluate_ending
from tui_adv.game.state import GameState


def test_default_encounter_yaml_loads_choice_conditions_and_checks():
    encounters = load_default_encounters()
    messenger = encounters["ex_employee_messenger"]
    trace_choice = next(
        choice for choice in messenger.choices if choice.id == "trace_packet_delay"
    )
    meeting = encounters["meeting_room_all_hands"]
    cctv = encounters["security_room_delayed_cctv"]

    assert DATA_DIR.joinpath("encounters.yaml").name == "encounters.yaml"
    assert messenger.title == "퇴사자의 메신저"
    assert messenger.conditions.locations == ("dev_desk",)
    assert trace_choice.conditions.min_abilities == {"interface": 4}
    assert trace_choice.check is not None
    assert trace_choice.check.success.add_flags == ("network_truth_hint",)
    assert meeting.conditions.required_flags == ("truth_route_started",)
    assert meeting.choices[0].outcome.add_flags == ("impossible_meeting_saved",)
    assert cctv.conditions.required_flags == ("impossible_meeting_saved",)
    assert cctv.choices[0].outcome.add_clues == ("server_log_fragment",)


def test_default_endings_yaml_loads_escape_route_conditions():
    endings = load_default_endings()
    escape = endings["escape_commute"]
    truth = endings["truth_isolation_protocol"]

    assert escape.kind == "escape"
    assert escape.priority == 60
    assert escape.conditions.locations == ("emergency_stairs",)
    assert escape.conditions.required_flags == ("escape_route_completed",)
    assert "공간 왜곡" in escape.text
    assert truth.kind == "truth"
    assert truth.priority == 70
    assert truth.conditions.required_items == ("ex_employee_memo",)
    assert truth.conditions.required_clues == (
        "meeting_pattern_noticed",
        "server_log_fragment",
    )


def test_default_achievements_yaml_loads_truth_route_reward():
    achievements = load_default_achievements()
    truth = achievements["truth_protocol_understood"]

    assert truth.name == "격리 프로토콜 독해"
    assert truth.conditions.required_flags == ("isolation_protocol_revealed",)


def test_default_locations_yaml_loads_connections_and_tags():
    locations = load_default_locations()
    security_room = locations["security_room"]

    assert DATA_DIR.joinpath("locations.yaml").name == "locations.yaml"
    assert security_room.name == "보안실"
    assert security_room.connections == ("hallway",)
    assert security_room.tags == ("security", "surveillance", "truth")
    assert "security_room" in locations["hallway"].connections


def test_locations_yaml_rejects_unknown_connections(tmp_path):
    path = tmp_path / "locations.yaml"
    path.write_text(
        """
locations:
  - id: lonely_room
    name: 고립된 방
    description: 문 하나만 보인다.
    connections: [missing_room]
""".strip(),
        encoding="utf-8",
    )

    with pytest.raises(
        ValueError, match="location lonely_room references unknown connection: missing_room"
    ):
        load_locations(path)


def test_public_yaml_content_references_are_valid_and_private_safe():
    assert validate_public_content() == []


def test_runtime_default_encounters_are_loaded_from_yaml_content():
    encounters = load_default_encounters()

    assert "server_room_radio" in encounters
    assert "server_room_radio" in DEFAULT_ENCOUNTERS
    state = GameState.new(seed=123, location_id="server_room_front")

    selected = select_encounter(state)

    assert selected is not None
    assert selected.id == "server_room_radio"


def test_runtime_default_endings_are_loaded_from_yaml_content():
    endings = load_default_endings()

    assert "conquest_broadcast_channel" in endings
    assert "conquest_broadcast_channel" in DEFAULT_ENDINGS
    state = GameState.new(seed=123, location_id="server_room_front")
    state.flags.append("server_room_broadcast_controlled")

    ending = evaluate_ending(state)

    assert ending is not None
    assert ending.id == "conquest_broadcast_channel"
