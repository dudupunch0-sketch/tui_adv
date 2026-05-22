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
    """Render the current encounter, playable choices, and blocked-choice reasons."""

    lines = [f"인카운터: {encounter.title}", encounter.body, "", "선택지:"]
    available_choices: list[Choice] = []
    unavailable_choices: list[tuple[Choice, tuple[str, ...]]] = []
    for choice in encounter.choices:
        reasons = choice.unavailable_reasons(state)
        if reasons:
            unavailable_choices.append((choice, reasons))
        else:
            available_choices.append(choice)

    if not available_choices:
        lines.append("(가능한 선택지가 없다)")
    if state.player.should_distort_choices and available_choices:
        lines.append("(집중도가 흔들려 선택지가 부분적으로 왜곡된다)")

    for index, choice in enumerate(available_choices, start=1):
        lines.append(f"{index}. {_format_choice_label(choice, state)}")
        detail = _format_choice_detail(choice)
        if detail:
            lines.append(f"   {detail}")

    if unavailable_choices:
        lines.extend(["", "잠긴 선택지:"])
        for choice, reasons in unavailable_choices:
            lines.append(f"- [잠김] {choice.label}")
            reason_text = ", ".join(
                _format_unavailable_reason(reason, state) for reason in reasons
            )
            lines.append(f"   이유: {reason_text}")
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


def _format_unavailable_reason(reason: str, state: GameState) -> str:
    if reason == "location":
        return "현재 위치 조건 불일치"
    if reason == "disaster_type":
        return "재난 유형 조건 불일치"
    if reason.startswith("missing_item:"):
        return f"필요 아이템 없음: {reason.split(':', 1)[1]}"
    if reason.startswith("missing_clue:"):
        return f"필요 단서 없음: {reason.split(':', 1)[1]}"
    if reason.startswith("missing_flag:"):
        return f"필요 플래그 없음: {reason.split(':', 1)[1]}"
    if reason.startswith("forbidden_flag:"):
        return f"이미 발생한 플래그: {reason.split(':', 1)[1]}"
    if "+" in reason and reason.endswith(">100"):
        resource_name, amount_text = reason[:-4].split("+", 1)
        label = _RESOURCE_LABELS.get(resource_name, resource_name)
        current_value = getattr(state.player, resource_name, 0)
        return f"{label} 한계 초과: {current_value}+{amount_text}>100"
    if "<" in reason:
        name, required_text = reason.split("<", 1)
        try:
            required = int(required_text)
        except ValueError:
            return reason
        if name in _RESOURCE_LABELS:
            label = _RESOURCE_LABELS[name]
            current_value = getattr(state.player, name)
            return f"{label} 부족: {current_value}/{required}"
        return f"능력 부족: {name} {state.player.ability(name)}/{required}"
    if ">" in reason:
        name, maximum_text = reason.split(">", 1)
        try:
            maximum = int(maximum_text)
        except ValueError:
            return reason
        label = _RESOURCE_LABELS.get(name, name)
        current_value = getattr(state.player, name, 0)
        return f"{label} 조건 초과: {current_value}/{maximum}"
    return reason


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
