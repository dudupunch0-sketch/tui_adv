from tui_adv.game.content import (
    DATA_DIR,
    load_default_encounters,
    load_default_endings,
    validate_public_content,
)


def test_default_encounter_yaml_loads_choice_conditions_and_checks():
    encounters = load_default_encounters()
    messenger = encounters["ex_employee_messenger"]
    trace_choice = next(
        choice for choice in messenger.choices if choice.id == "trace_packet_delay"
    )

    assert DATA_DIR.joinpath("encounters.yaml").name == "encounters.yaml"
    assert messenger.title == "퇴사자의 메신저"
    assert messenger.conditions.locations == ("dev_desk",)
    assert trace_choice.conditions.min_abilities == {"interface": 4}
    assert trace_choice.check is not None
    assert trace_choice.check.success.add_flags == ("network_truth_hint",)


def test_default_endings_yaml_loads_escape_route_conditions():
    endings = load_default_endings()
    escape = endings["escape_commute"]

    assert escape.kind == "escape"
    assert escape.priority == 60
    assert escape.conditions.locations == ("emergency_stairs",)
    assert escape.conditions.required_flags == ("escape_route_completed",)
    assert "공간 왜곡" in escape.text


def test_public_yaml_content_references_are_valid_and_private_safe():
    assert validate_public_content() == []
