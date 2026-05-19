import pytest

from tui_adv.game.content import (
    DATA_DIR,
    load_default_achievements,
    load_default_encounters,
    load_default_endings,
    load_default_items,
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
    radio = encounters["server_room_radio"]
    cold_air_choice = next(
        choice for choice in radio.choices if choice.id == "follow_cold_air"
    )
    console = encounters["server_room_console"]

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
    assert cold_air_choice.outcome.destination_id == "server_room"
    assert console.conditions.locations == ("server_room",)
    assert console.choices[0].outcome.add_flags == (
        "network_admin_claimed",
        "internal_network_access",
    )


def test_default_endings_yaml_loads_escape_route_conditions():
    endings = load_default_endings()
    escape = endings["escape_commute"]
    truth = endings["truth_isolation_protocol"]
    conquest = endings["conquest_network_admin"]

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
    assert conquest.kind == "conquest"
    assert conquest.priority == 70
    assert conquest.conditions.locations == ("server_room",)
    assert conquest.conditions.required_flags == ("network_admin_claimed",)


def test_default_achievements_yaml_loads_truth_route_reward():
    achievements = load_default_achievements()
    truth = achievements["truth_protocol_understood"]
    network_admin = achievements["network_admin_claimed"]

    assert truth.name == "격리 프로토콜 독해"
    assert truth.conditions.required_flags == ("isolation_protocol_revealed",)
    assert network_admin.name == "사내망 관리자"
    assert network_admin.conditions.required_flags == ("network_admin_claimed",)


def test_default_items_yaml_loads_display_metadata():
    items = load_default_items()
    printout = items["crumpled_printout"]
    memo = items["ex_employee_memo"]

    assert DATA_DIR.joinpath("items.yaml").name == "items.yaml"
    assert printout.name == "구겨진 출력물"
    assert printout.kind == "clue"
    assert printout.tags == ("printer", "reality_link")
    assert memo.name == "퇴사자의 메모"
    assert memo.kind == "clue"


def test_default_locations_yaml_loads_connections_and_tags():
    locations = load_default_locations()
    security_room = locations["security_room"]
    server_room = locations["server_room"]
    supply_closet = locations["supply_closet"]
    elevator_hall = locations["elevator_hall"]
    rooftop = locations["rooftop"]

    assert DATA_DIR.joinpath("locations.yaml").name == "locations.yaml"
    assert security_room.name == "보안실"
    assert security_room.connections == ("hallway",)
    assert security_room.tags == ("security", "surveillance", "truth")
    assert "security_room" in locations["hallway"].connections
    assert server_room.name == "서버실 내부"
    assert server_room.connections == ("server_room_front",)
    assert "server_room" in locations["server_room_front"].connections
    assert supply_closet.name == "물품창고"
    assert supply_closet.connections == ("dev_office",)
    assert "supply_closet" in locations["dev_office"].connections
    assert elevator_hall.connections == ("hallway", "rooftop")
    assert "elevator_hall" in locations["hallway"].connections
    assert rooftop.tags == ("escape", "signal", "open_air")


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
    assert "supply_closet_cache" in encounters
    assert "elevator_nonexistent_floor" in encounters
    assert "rooftop_signal" in encounters
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
