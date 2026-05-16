from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

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

DEFAULT_ENDINGS: dict[str, Ending] = {
    "game_over_spatial_collapse": Ending(
        id="game_over_spatial_collapse",
        name="게임오버: 계단이 접혔다",
        kind="failure",
        priority=100,
        conditions=Conditions(required_flags=("spatial_exit_failed",)),
        text="공간 왜곡의 규칙을 틀렸다. 계단은 아래가 아니라 당신 안쪽으로 접혔다.",
    ),
    "escape_commute": Ending(
        id="escape_commute",
        name="퇴근 성공",
        kind="escape",
        priority=60,
        conditions=Conditions(
            locations=("emergency_stairs",),
            required_flags=("escape_route_completed",),
            min_resources={"health": 1, "sanity": 1},
        ),
        text=(
            "공간 왜곡 속에서 반복되는 층수를 맞춰 풀었다. "
            "비상문 너머의 평범한 밤공기가 당신을 퇴근시켰다."
        ),
    ),
}


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


def format_ending_summary(ending: Ending) -> str:
    """Render an ending in a CLI/TUI-friendly Korean block."""

    label = "게임오버" if ending.kind == "failure" else "엔딩"
    return "\n".join([f"{label}: {ending.name}", ending.text])
