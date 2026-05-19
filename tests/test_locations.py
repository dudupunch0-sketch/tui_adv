import pytest

from tui_adv.game.locations import DEFAULT_LOCATIONS, Location
from tui_adv.game.state import GameState


def test_default_office_map_contains_start_and_core_connections():
    start = DEFAULT_LOCATIONS["dev_desk"]
    office = DEFAULT_LOCATIONS["dev_office"]
    hallway = DEFAULT_LOCATIONS["hallway"]

    assert start == Location(
        id="dev_desk",
        name="내 자리",
        description="당신의 모니터는 아직 켜져 있다.",
        connections=("dev_office",),
        tags=("office", "start", "messenger", "personal"),
        danger=0,
    )
    assert office.connections == (
        "dev_desk",
        "hallway",
        "meeting_room",
        "printer_area",
        "supply_closet",
    )
    assert hallway.connections == (
        "dev_office",
        "server_room_front",
        "emergency_stairs",
        "security_room",
        "elevator_hall",
        "parking_lot",
    )


def test_new_game_starts_at_dev_desk_and_exposes_available_moves():
    state = GameState.new(seed=7)

    assert state.location_id == "dev_desk"
    assert state.available_move_ids() == ("dev_office",)


def test_move_to_connected_location_advances_turn_and_records_log():
    state = GameState.new(seed=7)

    moved = state.move_to("dev_office")

    assert moved.location_id == "dev_office"
    assert moved.turn == 1
    assert moved.player.hunger == 1
    assert moved.player.thirst == 2
    assert moved.log == ["개발팀 사무실로 이동했다."]
    assert state.location_id == "dev_desk"
    assert state.turn == 0
    assert state.log == []


def test_move_to_location_applies_destination_danger_without_mutating_source():
    state = GameState.new(seed=7, location_id="dev_office")

    moved = state.move_to("hallway")

    assert moved.location_id == "hallway"
    assert moved.danger == 1
    assert state.danger == 0


def test_move_to_unconnected_location_is_rejected_without_mutation():
    state = GameState.new(seed=7)

    with pytest.raises(ValueError, match="cannot move from dev_desk to hallway"):
        state.move_to("hallway")

    assert state.location_id == "dev_desk"
    assert state.turn == 0
    assert state.log == []


def test_runtime_default_locations_are_loaded_from_yaml_content():
    assert "security_room" in DEFAULT_LOCATIONS
    state = GameState.new(seed=7, location_id="hallway")

    assert "security_room" in state.available_move_ids()
    moved = state.move_to("security_room")

    assert moved.location_id == "security_room"
    assert moved.log == ["보안실로 이동했다."]
