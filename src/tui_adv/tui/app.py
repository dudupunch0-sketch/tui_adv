from __future__ import annotations

from dataclasses import dataclass, replace

from tui_adv.game.encounters import ChoiceResolution
from tui_adv.game.endings import Ending, evaluate_ending, format_ending_summary
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.loop import (
    GameTurn,
    TurnActionResult,
    build_game_turn,
    resolve_turn_action_result,
)
from tui_adv.game.state import GameState
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn
from tui_adv.tui.status import format_local_status

TuiTurn = GameTurn


@dataclass(frozen=True, slots=True)
class TuiChoiceResult:
    choice_label: str
    resolution: ChoiceResolution
    state: GameState
    ending: Ending | None


def build_tui_turn(
    *,
    seed: int,
    location_id: str = "dev_desk",
    flags: tuple[str, ...] = (),
) -> GameTurn:
    if location_id not in DEFAULT_LOCATIONS:
        raise ValueError(f"알 수 없는 위치: {location_id}")
    state = replace(GameState.new(seed=seed, location_id=location_id), flags=list(flags))
    return build_game_turn(state)


def render_tui_layout_snapshot(turn: GameTurn) -> str:
    """Render the same panels the Textual shell mounts, without requiring Textual."""

    location = DEFAULT_LOCATIONS[turn.state.location_id]
    lines = [
        "escape from the office",
        "",
        "[위치]",
        f"{location.name} — {location.description}",
        "",
        format_local_status(turn.state.player),
        "",
    ]
    if turn.ending is not None:
        lines.extend(["[엔딩]", format_ending_summary(turn.ending)])
    elif turn.encounter is not None:
        lines.extend(["[현재 인카운터]", format_encounter_turn(turn.encounter, turn.state)])
    elif turn.available_actions:
        lines.extend(["[현재 행동]", "이동:"])
        for index, action in enumerate(turn.available_actions, start=1):
            lines.append(f"{index}. {action.label}")
    else:
        lines.extend(["[현재 행동]", "가능한 행동 없음"])

    lines.extend(["", "[최근 로그]"])
    if turn.state.log:
        lines.extend(f"- {entry}" for entry in turn.state.log[-5:])
    else:
        lines.append("- 아직 기록 없음")
    return "\n".join(lines)


def resolve_tui_action(turn: GameTurn, action_index: int) -> TurnActionResult:
    selected_index = action_index - 1
    if selected_index < 0 or selected_index >= len(turn.available_actions):
        raise ValueError(f"행동을 찾을 수 없다: {action_index}")
    action = turn.available_actions[selected_index]
    return resolve_turn_action_result(turn, action.id)


def resolve_tui_choice(turn: GameTurn, choice_index: int) -> TuiChoiceResult:
    if turn.encounter is None:
        raise ValueError("현재 인카운터가 없다")
    choices = turn.encounter.available_choices(turn.state)
    selected_index = choice_index - 1
    if selected_index < 0 or selected_index >= len(choices):
        raise ValueError(f"선택지를 찾을 수 없다: {choice_index}")
    choice = choices[selected_index]
    resolution = turn.encounter.resolve_choice_result(choice.id, turn.state)
    ending = evaluate_ending(resolution.state)
    return TuiChoiceResult(
        choice_label=choice.label,
        resolution=resolution,
        state=resolution.state,
        ending=ending,
    )


def run_textual_tui(
    *,
    seed: int,
    location_id: str = "dev_desk",
    flags: tuple[str, ...] = (),
) -> None:
    """Launch the interactive Textual shell when the optional dependency exists."""

    try:
        from textual.app import App, ComposeResult  # type: ignore[import-not-found]
        from textual.widgets import Footer, Header, Static  # type: ignore[import-not-found]
    except ModuleNotFoundError as exc:  # pragma: no cover - depends on local install.
        raise RuntimeError(
            "Textual이 설치되어 있지 않다. 패키지를 설치한 환경에서 --tui를 실행하라."
        ) from exc

    class OfficeEscapeApp(App[None]):  # pragma: no cover - interactive shell smoke only.
        CSS = """
        Screen { layout: vertical; }
        #game { height: 1fr; overflow-y: auto; padding: 1 2; }
        """

        def __init__(self) -> None:
            super().__init__()
            self.turn = build_tui_turn(seed=seed, location_id=location_id, flags=flags)

        def compose(self) -> ComposeResult:
            yield Header(show_clock=True)
            yield Static(render_tui_layout_snapshot(self.turn), id="game")
            yield Footer()

        def on_key(self, event) -> None:
            if self.turn.ending is not None:
                return
            if not event.key.isdecimal():
                return
            try:
                result = resolve_tui_action(self.turn, int(event.key))
            except ValueError as exc:
                self._append_message(str(exc))
                return
            log = [_format_tui_action_result(result)]
            next_state = replace(result.turn.state, log=[*result.turn.state.log, *log])
            self.turn = build_game_turn(next_state)
            self.query_one("#game", Static).update(render_tui_layout_snapshot(self.turn))

        def _append_message(self, message: str) -> None:
            state = replace(self.turn.state, log=[*self.turn.state.log, message])
            self.turn = build_game_turn(state)
            self.query_one("#game", Static).update(render_tui_layout_snapshot(self.turn))

    OfficeEscapeApp().run()


def _format_tui_action_result(result: TurnActionResult) -> str:
    if result.action.kind == "choice":
        lines = [f"선택 실행: {result.action.label}"]
        if result.choice_resolution is not None:
            lines.append(format_choice_resolution(result.choice_resolution))
        return "\n".join(lines)
    if result.action.kind == "move":
        return f"이동 실행: {result.action.label}"
    return f"행동 실행: {result.action.label}"
