from __future__ import annotations

from dataclasses import dataclass, replace

from tui_adv.game.achievements import Achievement, unlock_new_achievements
from tui_adv.game.encounters import ChoiceResolution, Encounter, select_encounter
from tui_adv.game.endings import Ending, evaluate_ending
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.state import GameState


@dataclass(frozen=True, slots=True)
class TurnAction:
    """A concrete action the player can take on the current turn."""

    id: str
    label: str
    kind: str
    target_id: str


@dataclass(frozen=True, slots=True)
class GameTurn:
    """The current playable prompt: encounter choices first, then movement."""

    state: GameState
    encounter: Encounter | None
    ending: Ending | None
    available_move_ids: tuple[str, ...]
    available_actions: tuple[TurnAction, ...]


@dataclass(frozen=True, slots=True)
class TurnActionResult:
    """A resolved action and the next playable turn."""

    action: TurnAction
    before_turn: GameTurn
    turn: GameTurn
    choice_resolution: ChoiceResolution | None = None
    unlocked_achievements: tuple[Achievement, ...] = ()


def build_game_turn(state: GameState) -> GameTurn:
    """Build the next prompt from state without mutating it."""

    ending = evaluate_ending(state)
    if ending is not None:
        return GameTurn(
            state=state,
            encounter=None,
            ending=ending,
            available_move_ids=(),
            available_actions=(),
        )

    encounter = select_encounter(state)
    if encounter is not None:
        actions = tuple(
            TurnAction(
                id=f"choice:{index}",
                label=choice.label,
                kind="choice",
                target_id=choice.id,
            )
            for index, choice in enumerate(encounter.available_choices(state), start=1)
        )
        return GameTurn(
            state=state,
            encounter=encounter,
            ending=None,
            available_move_ids=(),
            available_actions=actions,
        )

    move_ids = state.available_move_ids()
    actions = tuple(
        TurnAction(
            id=f"move:{location_id}",
            label=DEFAULT_LOCATIONS[location_id].name,
            kind="move",
            target_id=location_id,
        )
        for location_id in move_ids
    )
    return GameTurn(
        state=state,
        encounter=None,
        ending=None,
        available_move_ids=move_ids,
        available_actions=actions,
    )


def resolve_turn_action(turn: GameTurn, action_id: str) -> GameTurn:
    """Resolve an action id and return the next turn."""

    return resolve_turn_action_result(turn, action_id).turn


def resolve_turn_action_result(turn: GameTurn, action_id: str) -> TurnActionResult:
    """Resolve an action id and preserve metadata for CLI/TUI formatting."""

    action = _available_action_by_id(turn, action_id)
    if action.kind == "choice":
        if turn.encounter is None:
            raise ValueError(f"지금 실행할 수 없는 행동: {action_id}")
        resolution = turn.encounter.resolve_choice_result(action.target_id, turn.state)
        unlock_result = unlock_new_achievements(resolution.state)
        updated_resolution = replace(resolution, state=unlock_result.state)
        return TurnActionResult(
            action=action,
            before_turn=turn,
            turn=build_game_turn(unlock_result.state),
            choice_resolution=updated_resolution,
            unlocked_achievements=unlock_result.unlocked,
        )

    if action.kind == "move":
        moved = turn.state.move_to(action.target_id)
        unlock_result = unlock_new_achievements(moved)
        return TurnActionResult(
            action=action,
            before_turn=turn,
            turn=build_game_turn(unlock_result.state),
            unlocked_achievements=unlock_result.unlocked,
        )

    raise ValueError(f"알 수 없는 행동 유형: {action.kind}")


def _available_action_by_id(turn: GameTurn, action_id: str) -> TurnAction:
    for action in turn.available_actions:
        if action.id == action_id:
            return action
    raise ValueError(f"지금 실행할 수 없는 행동: {action_id}")
