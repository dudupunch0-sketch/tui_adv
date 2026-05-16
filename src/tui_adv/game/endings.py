from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Mapping

from tui_adv.game.encounters import Conditions
from tui_adv.game.state import GameState


@dataclass(frozen=True, slots=True)
class Ending:
    """A resolved ending candidate for the current game state."""

    id: str
    name: str
    kind: str
    priority: int
    text: str
    conditions: Conditions = Conditions()
    local_hint_id: str | None = None


_FAILURE_ENDINGS: dict[str, Ending] = {
    "health_depleted": Ending(
        id="game_over_health_depleted",
        name="게임오버: 신체 반응 없음",
        kind="failure",
        priority=1000,
        text="몸이 먼저 퇴근을 포기했다. LOCAL STATUS가 더 이상 갱신되지 않는다.",
    ),
    "sanity_depleted": Ending(
        id="game_over_sanity_depleted",
        name="게임오버: 집중도 붕괴",
        kind="failure",
        priority=1000,
        text="마지막으로 본 사내 공지는 당신의 이름으로 이미 퇴근 처리되어 있었다.",
    ),
}

def load_runtime_default_endings() -> dict[str, Ending]:
    """Load runtime default route endings from packaged YAML content."""

    from tui_adv.game.content import load_default_endings

    return load_default_endings()


DEFAULT_ENDINGS: dict[str, Ending] = load_runtime_default_endings()

def evaluate_ending(
    state: GameState,
    endings: Mapping[str, Ending] | None = None,
) -> Ending | None:
    """Return the highest-priority ending currently satisfied by state."""

    if state.failure_reason is not None:
        return _FAILURE_ENDINGS[state.failure_reason]

    ending_map = DEFAULT_ENDINGS if endings is None else endings
    candidates = [
        ending
        for ending in ending_map.values()
        if ending.conditions.is_satisfied_by(state)
    ]
    if not candidates:
        return None
    return max(candidates, key=lambda ending: ending.priority)


def format_ending_summary(
    ending: Ending,
    *,
    local_hint_path: Path | Any | None = None,
) -> str:
    """Render an ending in a CLI/TUI-friendly Korean block."""

    label = "게임오버" if ending.kind == "failure" else "엔딩"
    lines = [f"{label}: {ending.name}", ending.text]
    if ending.local_hint_id:
        lines.extend(["", _format_physical_hint_reveal(ending.local_hint_id, local_hint_path)])
    return "\n".join(lines)


def _format_physical_hint_reveal(
    local_hint_id: str,
    local_hint_path: Path | Any | None,
) -> str:
    from tui_adv.game.secrets import reveal_physical_hint

    reveal = reveal_physical_hint(local_hint_id, local_path=local_hint_path)
    lines = [
        f"현실 연결 힌트: {reveal.title}",
        reveal.reward_text,
        "공개 단서:",
    ]
    lines.extend(f"- {step}" for step in reveal.public_hint_steps)
    if reveal.puzzle_prompt:
        lines.append(f"퍼즐: {reveal.puzzle_prompt}")
    if reveal.puzzle_ip_address:
        lines.append(f"IP 주소: {reveal.puzzle_ip_address}")
    if reveal.puzzle_answer is not None:
        lines.append(f"숫자 합계: {reveal.puzzle_answer}")
    if reveal.final_hint_available:
        lines.append(f"최종 힌트: {reveal.final_hint}")
    else:
        lines.append("최종 힌트: 로컬 비공개 파일이 있을 때만 표시된다.")
    return "\n".join(lines)
