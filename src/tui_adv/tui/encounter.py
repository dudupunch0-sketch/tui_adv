from __future__ import annotations

from tui_adv.game.encounters import CheckResult, Choice, ChoiceResolution, Encounter
from tui_adv.game.state import GameState, PlayerState

_RESOURCE_LABELS = {
    "health": "체력",
    "sanity": "정신력",
    "battery": "배터리",
    "hunger": "허기",
    "thirst": "갈증",
}


def format_encounter_turn(encounter: Encounter, state: GameState) -> str:
    """Render the current encounter and selectable choices."""

    lines = [f"인카운터: {encounter.title}", encounter.body, "", "선택지:"]
    choices = encounter.available_choices(state)
    if not choices:
        lines.append("(가능한 선택지가 없다)")
        return "\n".join(lines)
    if state.player.should_distort_choices:
        lines.append("(집중도가 흔들려 선택지가 부분적으로 왜곡된다)")

    for index, choice in enumerate(choices, start=1):
        lines.append(f"{index}. {_format_choice_label(choice, state)}")
        detail = _format_choice_detail(choice)
        if detail:
            lines.append(f"   {detail}")
    return "\n".join(lines)


def format_choice_resolution(resolution: ChoiceResolution) -> str:
    """Render logs, check metadata, and resource deltas after a choice."""

    lines = ["결과:"]
    if resolution.check_result is not None:
        lines.append(_format_check_result(resolution.check_result))

    if resolution.new_logs:
        lines.append("로그:")
        lines.extend(f"- {entry}" for entry in resolution.new_logs)

    deltas = _format_resource_deltas(resolution.before_state.player, resolution.state.player)
    if deltas:
        lines.append("변화:")
        lines.extend(f"- {delta}" for delta in deltas)

    return "\n".join(lines)


def _format_choice_label(choice: Choice, state: GameState) -> str:
    if not state.player.should_distort_choices:
        return choice.label
    return _distort_text(choice.label)


def _distort_text(text: str) -> str:
    distorted: list[str] = []
    visible_index = 0
    for character in text:
        if character.isspace():
            distorted.append(character)
            continue
        visible_index += 1
        if visible_index % 3 == 0:
            distorted.append("▒")
        else:
            distorted.append(character)
    return "".join(distorted)


def _format_choice_detail(choice: Choice) -> str:
    details: list[str] = []
    if choice.cost:
        details.append("비용: " + ", ".join(_format_cost(name, amount) for name, amount in choice.cost.items()))
    if choice.check is not None:
        details.append(f"판정: 2d6 + {choice.check.ability} >= {choice.check.difficulty}")
    return " / ".join(details)


def _format_cost(resource_name: str, amount: int) -> str:
    label = _RESOURCE_LABELS.get(resource_name, resource_name)
    sign = "+" if resource_name in {"hunger", "thirst"} else "-"
    return f"{label} {sign}{amount}"


def _format_check_result(result: CheckResult) -> str:
    first, second = result.rolls
    status = "성공" if result.succeeded else "실패"
    return (
        f"[판정] {result.ability} {first}+{second}+{result.ability_score} "
        f"= {result.total} / 난이도 {result.difficulty}: {status}"
    )


def _format_resource_deltas(before: PlayerState, after: PlayerState) -> tuple[str, ...]:
    deltas: list[str] = []
    for field_name, label in _RESOURCE_LABELS.items():
        before_value = getattr(before, field_name)
        after_value = getattr(after, field_name)
        if before_value != after_value:
            deltas.append(f"{label}: {before_value} -> {after_value}")
    return tuple(deltas)
