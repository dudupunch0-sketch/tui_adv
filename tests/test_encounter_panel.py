from tui_adv.game.encounters import DEFAULT_ENCOUNTERS
from tui_adv.game.state import GameState, PlayerState
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn


class FixedRolls:
    def __init__(self, rolls: tuple[int, int]) -> None:
        self._rolls = list(rolls)

    def randint(self, start: int, stop: int) -> int:
        assert (start, stop) == (1, 6)
        return self._rolls.pop(0)


def test_format_encounter_turn_lists_available_choices_and_check_details():
    state = GameState.new(seed=1).with_player(
        PlayerState().with_abilities(interface=4)
    )
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    rendered = format_encounter_turn(encounter, state)

    assert "인카운터: 퇴사자의 메신저" in rendered
    assert "퇴사한 전임자에게서 사내 메신저가 도착했다." in rendered
    assert "1. 메시지를 확인한다" in rendered
    assert "4. [인터페이스] 알림 지연 시간을 역추적한다" in rendered
    assert "비용: 배터리 -2" in rendered
    assert "판정: 2d6 + interface >= 10" in rendered


def test_format_encounter_turn_lists_unavailable_choices_with_reasons():
    state = GameState.new(seed=1)
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    rendered = format_encounter_turn(encounter, state)

    assert "잠긴 선택지:" in rendered
    assert "- [잠김] [인터페이스] 알림 지연 시간을 역추적한다" in rendered
    assert "이유: 능력 부족: interface 2/4" in rendered


def test_format_encounter_turn_keeps_available_choice_numbers_compact_when_middle_choices_are_locked():
    state = GameState.new(seed=1, location_id="pantry")
    encounter = DEFAULT_ENCOUNTERS["pantry_coffee_machine"]

    rendered = format_encounter_turn(encounter, state)

    assert "1. 커피를 뽑는다" in rendered
    assert "2. 물통을 확인한다" in rendered
    assert "3. 밀봉된 생수 한 병을 챙긴다" in rendered
    assert "5. 밀봉된 생수 한 병을 챙긴다" not in rendered
    assert "- [잠김] 점검 라벨의 표식을 토너 표식과 맞춰 본다" in rendered
    assert "- [잠김] 커피머신 뒤를 본다" in rendered


def test_format_encounter_turn_distorts_choice_labels_when_sanity_is_low():
    state = GameState.new(seed=1).with_player(PlayerState(sanity=30))
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    rendered = format_encounter_turn(encounter, state)

    assert "집중도가 흔들려 선택지가 부분적으로 왜곡된다" in rendered
    assert "1. 메시▒를 확▒한다" in rendered
    assert "1. 메시지를 확인한다" not in rendered


def test_format_choice_resolution_summarizes_check_result_logs_and_resource_delta():
    state = GameState.new(seed=1).with_player(
        PlayerState().with_abilities(interface=4)
    )
    encounter = DEFAULT_ENCOUNTERS["ex_employee_messenger"]

    resolution = encounter.resolve_choice_result(
        "trace_packet_delay",
        state,
        rng=FixedRolls((3, 3)),
    )
    rendered = format_choice_resolution(resolution)

    assert "[판정] interface 3+3+4 = 10 / 난이도 10: 성공" in rendered
    assert "알림 패킷을 조심스럽게 붙잡았다." in rendered
    assert "지연 시간 사이에서 숨은 라우팅을 찾았다." in rendered
    assert "배터리: 100 -> 98" in rendered
