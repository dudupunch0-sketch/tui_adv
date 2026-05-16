from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Mapping

from tui_adv.game.encounters import Conditions
from tui_adv.game.state import GameState


@dataclass(frozen=True, slots=True)
class Achievement:
    """A persistent route or discovery reward unlocked by game state."""

    id: str
    name: str
    description: str
    conditions: Conditions = field(default_factory=Conditions)
    hidden: bool = False


@dataclass(frozen=True, slots=True)
class AchievementUnlockResult:
    """State after achievement evaluation plus newly unlocked achievements."""

    state: GameState
    unlocked: tuple[Achievement, ...]


AchievementMap = Mapping[str, Achievement]


def load_runtime_default_achievements() -> dict[str, Achievement]:
    """Load runtime default achievements from packaged YAML content."""

    from tui_adv.game.content import load_default_achievements

    return load_default_achievements()


DEFAULT_ACHIEVEMENTS: dict[str, Achievement] = load_runtime_default_achievements()


def evaluate_new_achievements(
    state: GameState,
    achievements: AchievementMap | None = None,
) -> tuple[Achievement, ...]:
    """Return achievements whose conditions are satisfied and not yet unlocked."""

    achievement_map = DEFAULT_ACHIEVEMENTS if achievements is None else achievements
    unlocked_ids = set(state.unlocked_achievements)
    return tuple(
        achievement
        for achievement in achievement_map.values()
        if achievement.id not in unlocked_ids
        and achievement.conditions.is_satisfied_by(state)
    )


def unlock_new_achievements(
    state: GameState,
    achievements: AchievementMap | None = None,
) -> AchievementUnlockResult:
    """Record newly satisfied achievements without duplicating existing ids."""

    unlocked = evaluate_new_achievements(state, achievements)
    if not unlocked:
        return AchievementUnlockResult(state=state, unlocked=())

    unlocked_achievement_ids = list(state.unlocked_achievements)
    for achievement in unlocked:
        if achievement.id not in unlocked_achievement_ids:
            unlocked_achievement_ids.append(achievement.id)

    updated_state = replace(
        state,
        inventory=list(state.inventory),
        clues=list(state.clues),
        flags=list(state.flags),
        seen_encounters=list(state.seen_encounters),
        unlocked_achievements=unlocked_achievement_ids,
        log=list(state.log),
    )
    return AchievementUnlockResult(state=updated_state, unlocked=unlocked)


def format_unlocked_achievements(achievements: tuple[Achievement, ...]) -> str:
    """Render newly unlocked achievements for CLI/TUI logs."""

    lines: list[str] = []
    for achievement in achievements:
        if lines:
            lines.append("")
        lines.append(f"업적 달성: {achievement.name}")
        if achievement.description:
            lines.append(achievement.description)
    return "\n".join(lines)


def format_achievement_summary(
    state: GameState,
    achievements: AchievementMap | None = None,
) -> str:
    """Render the achievements already unlocked in a save/state."""

    if not state.unlocked_achievements:
        return ""
    achievement_map = DEFAULT_ACHIEVEMENTS if achievements is None else achievements
    lines = ["업적:"]
    for achievement_id in state.unlocked_achievements:
        achievement = achievement_map.get(achievement_id)
        name = achievement.name if achievement else achievement_id
        lines.append(f"- {name}")
    return "\n".join(lines)
