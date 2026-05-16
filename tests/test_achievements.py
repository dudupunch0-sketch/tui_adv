from dataclasses import replace

from tui_adv.game.achievements import (
    Achievement,
    DEFAULT_ACHIEVEMENTS,
    format_unlocked_achievements,
    unlock_new_achievements,
)
from tui_adv.game.encounters import Conditions
from tui_adv.game.loop import build_game_turn, resolve_turn_action_result
from tui_adv.game.state import GameState


def test_unlock_new_achievements_records_matching_ids_once():
    achievement = Achievement(
        id="first_signal_received",
        name="첫 신호 확인",
        description="퇴사자의 첫 메시지를 확인했다.",
        conditions=Conditions(required_clues=("ex_employee_contacted",)),
    )
    state = replace(GameState.new(seed=123), clues=["ex_employee_contacted"])

    first = unlock_new_achievements(
        state,
        achievements={achievement.id: achievement},
    )
    second = unlock_new_achievements(
        first.state,
        achievements={achievement.id: achievement},
    )

    assert first.state.unlocked_achievements == ["first_signal_received"]
    assert first.unlocked == (achievement,)
    assert second.state.unlocked_achievements == ["first_signal_received"]
    assert second.unlocked == ()


def test_scripted_choice_unlocks_first_signal_achievement():
    turn = build_game_turn(GameState.new(seed=123))

    result = resolve_turn_action_result(turn, "choice:1")

    assert "first_signal_received" in result.turn.state.unlocked_achievements
    assert [achievement.id for achievement in result.unlocked_achievements] == [
        "first_signal_received"
    ]


def test_default_achievements_include_reality_conquest_and_truth_route_rewards():
    assert DEFAULT_ACHIEVEMENTS["first_signal_received"].conditions.required_clues == (
        "ex_employee_contacted",
    )
    assert DEFAULT_ACHIEVEMENTS[
        "reality_link_discovered"
    ].conditions.required_flags == ("pantry_hint_seen",)
    assert DEFAULT_ACHIEVEMENTS[
        "broadcast_channel_captured"
    ].conditions.required_flags == ("server_room_broadcast_controlled",)
    assert DEFAULT_ACHIEVEMENTS[
        "truth_protocol_understood"
    ].conditions.required_flags == ("isolation_protocol_revealed",)


def test_format_unlocked_achievements_renders_korean_unlock_lines():
    achievement = Achievement(
        id="first_signal_received",
        name="첫 신호 확인",
        description="퇴사자의 첫 메시지를 확인했다.",
    )

    assert format_unlocked_achievements((achievement,)) == (
        "업적 달성: 첫 신호 확인\n퇴사자의 첫 메시지를 확인했다."
    )
    assert format_unlocked_achievements(()) == ""
