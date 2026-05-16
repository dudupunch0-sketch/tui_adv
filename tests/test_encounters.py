import pytest

from tui_adv.game.encounters import DEFAULT_ENCOUNTERS, Choice, Conditions, Encounter, Outcome
from tui_adv.game.state import GameState, PlayerState


def test_default_encounter_is_eligible_by_location_and_seen_history():
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    assert encounter.is_eligible(GameState.new(seed=1)) is True
    assert encounter.is_eligible(GameState.new(seed=1, location_id="dev_office")) is False
    assert encounter.is_eligible(
        GameState.new(seed=1, location_id="dev_desk").with_seen_encounter(
            "ex_employee_messenger"
        )
    ) is False


def test_available_choices_respect_costs_flags_items_and_resource_conditions():
    encounter = DEFAULT_ENCOUNTERS["printer_prints_alone"]
    low_sanity = GameState.new(seed=1, location_id="printer_area").with_player(
        PlayerState(sanity=39)
    )

    assert [choice.id for choice in encounter.available_choices(low_sanity)] == [
        "read_printout",
        "take_printout",
    ]

    ready = low_sanity.with_player(PlayerState(sanity=40))
    assert [choice.id for choice in encounter.available_choices(ready)] == [
        "read_printout",
        "take_printout",
        "check_toner",
    ]

    battery_starved = GameState.new(seed=1).with_player(PlayerState(battery=2))
    messenger = DEFAULT_ENCOUNTERS["ex_employee_messenger"]
    assert [choice.id for choice in messenger.available_choices(battery_starved)] == [
        "ignore_phone",
    ]


def test_resolve_choice_applies_costs_outcome_seen_history_log_and_turn_pressure():
    state = GameState.new(seed=1)
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    next_state = encounter.resolve_choice("check_message", state)

    assert next_state.turn == 1
    assert next_state.player.battery == 97
    assert next_state.player.sanity == 98
    assert next_state.player.hunger == 1
    assert next_state.player.thirst == 2
    assert next_state.clues == ["ex_employee_contacted"]
    assert next_state.seen_encounters == ["ex_employee_messenger"]
    assert next_state.log == ["퇴사자의 메시지를 확인했다."]
    assert state.turn == 0
    assert state.clues == []


def test_unavailable_choice_is_rejected_without_mutation():
    state = GameState.new(seed=1).with_player(PlayerState(battery=7))
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    with pytest.raises(ValueError, match="choice search_ex_employee is not available"):
        encounter.resolve_choice("search_ex_employee", state)

    assert state.turn == 0
    assert state.player.battery == 7
    assert state.flags == []


def test_choice_outcome_can_update_inventory_flags_danger_and_location():
    encounter = Encounter(
        id="test_forced_move",
        title="테스트 이동",
        body="테스트용 인카운터",
        conditions=Conditions(locations=("dev_desk",)),
        choices=(
            Choice(
                id="take_badge",
                label="사원증을 챙겨 복도로 나간다",
                outcome=Outcome(
                    add_items=("employee_badge",),
                    add_flags=("left_desk_after_badge",),
                    destination_id="dev_office",
                    danger=1,
                    log="사원증을 챙기고 자리에서 벗어났다.",
                ),
            ),
        ),
    )

    next_state = encounter.resolve_choice("take_badge", GameState.new(seed=1))

    assert next_state.location_id == "dev_office"
    assert next_state.inventory == ["employee_badge"]
    assert next_state.flags == ["left_desk_after_badge"]
    assert next_state.danger == 1
    assert next_state.turn == 1
    assert next_state.log == ["사원증을 챙기고 자리에서 벗어났다."]
