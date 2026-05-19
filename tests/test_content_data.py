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
from tui_adv.game.state import GameState, PlayerState


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
    security_override = encounters["security_room_floor_mismatch_console"]
    water = encounters["strange_water_dispenser"]

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
    assert security_override.conditions.locations == ("security_room",)
    assert security_override.conditions.required_clues == (
        "security_floor_misalignment",
    )
    assert security_override.conditions.required_flags == (
        "elevator_returned_wrong_floor",
    )
    assert security_override.choices[0].outcome.add_items == (
        "security_override_badge",
    )
    assert water.conditions.locations == ("pantry",)
    assert water.conditions.min_resources == {"thirst": 60}
    assert water.conditions.forbidden_flags == ("thirst_hallucination_seen",)
    assert water.choices[0].outcome.add_flags == ("thirst_hallucination_seen",)


def test_default_endings_yaml_loads_escape_route_conditions():
    endings = load_default_endings()
    escape = endings["escape_commute"]
    truth = endings["truth_isolation_protocol"]
    conquest = endings["conquest_network_admin"]
    security_conquest = endings["conquest_security_lockdown"]

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
    assert security_conquest.kind == "conquest"
    assert security_conquest.priority > conquest.priority
    assert security_conquest.conditions.required_flags == (
        "security_lockdown_claimed",
    )


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
    security_badge = items["security_override_badge"]
    parking_key = items["parking_key_fob"]

    assert DATA_DIR.joinpath("items.yaml").name == "items.yaml"
    assert printout.name == "구겨진 출력물"
    assert printout.kind == "clue"
    assert printout.tags == ("printer", "reality_link")
    assert memo.name == "퇴사자의 메모"
    assert memo.kind == "clue"
    assert security_badge.name == "보안실 우회권한"
    assert security_badge.kind == "key"
    assert parking_key.name == "지하주차장 키태그"
    assert parking_key.kind == "key"


def test_default_locations_yaml_loads_connections_and_tags():
    locations = load_default_locations()
    security_room = locations["security_room"]
    server_room = locations["server_room"]
    supply_closet = locations["supply_closet"]
    elevator_hall = locations["elevator_hall"]
    rooftop = locations["rooftop"]
    parking_lot = locations["parking_lot"]

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
    assert parking_lot.name == "지하주차장"
    assert parking_lot.connections == ("hallway",)
    assert "parking_lot" in locations["hallway"].connections
    assert parking_lot.tags == ("escape", "parking", "dark")


def test_default_parking_lot_escape_content_is_wired():
    encounters = load_default_encounters()
    endings = load_default_endings()
    achievements = load_default_achievements()
    ignition = encounters["parking_ignition"]
    ramp = encounters["parking_exit_ramp"]
    parking_escape = endings["escape_parking_lot"]
    achievement = achievements["parking_lot_escape_driver"]

    assert ignition.conditions.locations == ("parking_lot",)
    assert ignition.choices[0].outcome.add_items == ("parking_key_fob",)
    assert ignition.choices[0].outcome.add_flags == ("parking_key_found",)
    assert ramp.conditions.locations == ("parking_lot",)
    assert ramp.conditions.required_items == ("parking_key_fob",)
    assert ramp.conditions.required_flags == ("parking_key_found",)
    assert ramp.choices[0].outcome.add_clues == ("parking_exit_route",)
    assert ramp.choices[0].outcome.add_flags == ("parking_ramp_opened",)
    assert parking_escape.kind == "escape"
    assert parking_escape.conditions.locations == ("parking_lot",)
    assert parking_escape.conditions.required_items == ("parking_key_fob",)
    assert parking_escape.conditions.required_clues == ("parking_exit_route",)
    assert parking_escape.conditions.required_flags == ("parking_ramp_opened",)
    assert achievement.conditions.required_flags == ("parking_ramp_opened",)


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
    assert "strange_water_dispenser" in encounters
    assert "security_room_floor_mismatch_console" in encounters
    assert "server_room_radio" in DEFAULT_ENCOUNTERS
    state = GameState.new(seed=123, location_id="server_room_front")

    selected = select_encounter(state)

    assert selected is not None
    assert selected.id == "server_room_radio"


def test_strange_water_dispenser_only_appears_when_thirst_is_high():
    water = DEFAULT_ENCOUNTERS["strange_water_dispenser"]
    stable = GameState.new(seed=123, location_id="pantry").with_player(
        PlayerState(thirst=59)
    )
    thirsty = GameState.new(seed=123, location_id="pantry").with_player(
        PlayerState(thirst=60)
    )

    assert water.is_eligible(stable) is False
    assert water.is_eligible(thirsty) is True


def test_runtime_default_endings_are_loaded_from_yaml_content():
    endings = load_default_endings()

    assert "conquest_broadcast_channel" in endings
    assert "conquest_security_lockdown" in endings
    assert "conquest_broadcast_channel" in DEFAULT_ENDINGS
    state = GameState.new(seed=123, location_id="server_room_front")
    state.flags.append("server_room_broadcast_controlled")

    ending = evaluate_ending(state)

    assert ending is not None
    assert ending.id == "conquest_broadcast_channel"
