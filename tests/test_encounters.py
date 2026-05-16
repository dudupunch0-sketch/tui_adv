from dataclasses import replace

import pytest

from tui_adv.game.encounters import (
    DEFAULT_ENCOUNTERS,
    AbilityCheck,
    Choice,
    Conditions,
    Encounter,
    Outcome,
    eligible_encounters,
    select_encounter,
)
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


def test_pantry_coffee_machine_restores_focus_and_unlocks_reality_hint():
    encounter = DEFAULT_ENCOUNTERS["pantry_coffee_machine"]
    state = GameState.new(seed=1, location_id="pantry").with_player(
        PlayerState(sanity=50, hunger=30, thirst=20)
    )

    assert [choice.id for choice in encounter.available_choices(state)] == [
        "brew_coffee",
        "inspect_water_tank",
    ]

    caffeinated = encounter.resolve_choice("brew_coffee", state)

    assert caffeinated.player.sanity == 54
    assert caffeinated.player.hunger == 28
    assert caffeinated.player.thirst == 27
    assert caffeinated.log == ["커피는 아직 따뜻했고, 컵 바닥에는 작은 검은 점이 남았다."]

    hidden_state = replace(state, flags=["printer_secret_started"])
    assert [choice.id for choice in encounter.available_choices(hidden_state)] == [
        "brew_coffee",
        "inspect_water_tank",
        "look_behind_machine",
    ]

    revealed = encounter.resolve_choice("look_behind_machine", hidden_state)

    assert revealed.clues == ["reality_link_hint_2"]
    assert revealed.flags == [
        "printer_secret_started",
        "coffee_machine_back_panel",
        "pantry_hint_seen",
    ]
    assert revealed.log == ["커피머신 뒤쪽 패널에 복합기 출력물과 같은 표식이 있었다."]


def test_eligible_encounters_filter_current_state_and_seen_history():
    encounters = {
        "desk": Encounter(
            id="desk",
            title="자리 이벤트",
            body="책상 위 모니터가 깜빡인다.",
            conditions=Conditions(locations=("dev_desk",)),
            choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
        ),
        "printer": Encounter(
            id="printer",
            title="복합기 이벤트",
            body="복합기 쪽에서 종이 냄새가 난다.",
            conditions=Conditions(locations=("printer_area",)),
            choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
        ),
        "repeatable": Encounter(
            id="repeatable",
            title="반복 이벤트",
            body="슬랙 알림이 또 울린다.",
            conditions=Conditions(locations=("dev_desk",)),
            choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
            repeatable=True,
        ),
    }
    state = GameState.new(seed=1).with_seen_encounter("desk")

    assert [encounter.id for encounter in eligible_encounters(state, encounters)] == [
        "repeatable"
    ]


def test_select_encounter_uses_weights_after_eligibility_filtering():
    class FixedRng:
        def randrange(self, stop: int) -> int:
            assert stop == 4
            return 1

    light = Encounter(
        id="light",
        title="가벼운 이벤트",
        body="가벼운 이벤트",
        conditions=Conditions(locations=("dev_desk",)),
        choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
        weight=1,
    )
    heavy = Encounter(
        id="heavy",
        title="무거운 이벤트",
        body="무거운 이벤트",
        conditions=Conditions(locations=("dev_desk",)),
        choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
        weight=3,
    )
    wrong_location = Encounter(
        id="wrong_location",
        title="다른 장소 이벤트",
        body="다른 장소 이벤트",
        conditions=Conditions(locations=("printer_area",)),
        choices=(Choice(id="wait", label="기다린다", outcome=Outcome()),),
        weight=50,
    )

    selected = select_encounter(
        GameState.new(seed=1),
        {"light": light, "heavy": heavy, "wrong_location": wrong_location},
        rng=FixedRng(),
    )

    assert selected is heavy


def test_ability_gated_choices_expand_default_encounter_options():
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]
    baseline = GameState.new(seed=1)
    interface_ready = baseline.with_player(PlayerState().with_abilities(interface=4))

    assert "trace_packet_delay" not in [
        choice.id for choice in encounter.available_choices(baseline)
    ]
    assert "trace_packet_delay" in [
        choice.id for choice in encounter.available_choices(interface_ready)
    ]


def test_ability_check_choice_branches_into_success_or_failure_outcome():
    class FixedRolls:
        def __init__(self, rolls: tuple[int, int]) -> None:
            self._rolls = list(rolls)

        def randint(self, start: int, stop: int) -> int:
            assert (start, stop) == (1, 6)
            return self._rolls.pop(0)

    encounter = Encounter(
        id="test_skill_check",
        title="패킷의 유령",
        body="제한된 사내망 패킷이 같은 문장을 반복한다.",
        choices=(
            Choice(
                id="read_packet",
                label="[인터페이스] 패킷 지연을 읽는다",
                cost={"battery": 2},
                outcome=Outcome(log="휴대폰을 사내망에 붙였다."),
                check=AbilityCheck(
                    ability="interface",
                    difficulty=10,
                    success=Outcome(
                        add_clues=("packet_ghost_route",),
                        add_flags=("network_truth_hint",),
                        log="지연 시간 사이에서 숨은 라우팅을 찾았다.",
                    ),
                    failure=Outcome(
                        sanity=-5,
                        danger=1,
                        log="화면이 역으로 당신의 시선을 추적했다.",
                    ),
                ),
            ),
        ),
    )
    state = GameState.new(seed=1).with_player(PlayerState().with_abilities(interface=4))

    success = encounter.resolve_choice("read_packet", state, rng=FixedRolls((3, 3)))
    failure = encounter.resolve_choice("read_packet", state, rng=FixedRolls((1, 1)))

    assert success.player.battery == 98
    assert success.clues == ["packet_ghost_route"]
    assert success.flags == ["network_truth_hint"]
    assert success.log == [
        "휴대폰을 사내망에 붙였다.",
        "지연 시간 사이에서 숨은 라우팅을 찾았다.",
    ]

    assert failure.player.battery == 98
    assert failure.player.sanity == 95
    assert failure.danger == 1
    assert failure.clues == []
    assert failure.log == [
        "휴대폰을 사내망에 붙였다.",
        "화면이 역으로 당신의 시선을 추적했다.",
    ]
