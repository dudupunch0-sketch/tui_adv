from __future__ import annotations

from dataclasses import dataclass, replace

from tui_adv.game.encounters import ChoiceResolution, Encounter, select_encounter
from tui_adv.game.endings import Ending, evaluate_ending, format_ending_summary
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.state import GameState
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn
from tui_adv.tui.status import format_local_status


@dataclass(frozen=True, slots=True)
class TuiTurn:
    state: GameState
    encounter: Encounter | None


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
) -> TuiTurn:
    if location_id not in DEFAULT_LOCATIONS:
        raise ValueError(f"알 수 없는 위치: {location_id}")
    state = replace(GameState.new(seed=seed, location_id=location_id), flags=list(flags))
    return TuiTurn(state=state, encounter=select_encounter(state))


def render_tui_layout_snapshot(turn: TuiTurn) -> str:
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
        "[현재 인카운터]",
    ]
    if turn.encounter is None:
        lines.append("인카운터 없음")
    else:
        lines.append(format_encounter_turn(turn.encounter, turn.state))
    lines.extend(["", "[최근 로그]"])
    if turn.state.log:
        lines.extend(f"- {entry}" for entry in turn.state.log[-5:])
    else:
        lines.append("- 아직 기록 없음")
    return "\n".join(lines)


def resolve_tui_choice(turn: TuiTurn, choice_index: int) -> TuiChoiceResult:
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
            self.ending: Ending | None = None

        def compose(self) -> ComposeResult:
            yield Header(show_clock=True)
            yield Static(render_tui_layout_snapshot(self.turn), id="game")
            yield Footer()

        def on_key(self, event) -> None:
            if self.ending is not None:
                return
            if not event.key.isdecimal():
                return
            try:
                result = resolve_tui_choice(self.turn, int(event.key))
            except ValueError as exc:
                self._append_message(str(exc))
                return
            self.ending = result.ending
            log = [
                f"선택 실행: {result.choice_label}",
                format_choice_resolution(result.resolution),
            ]
            if result.ending is not None:
                log.append(format_ending_summary(result.ending))
            next_state = replace(result.state, log=[*result.state.log, *log])
            self.turn = TuiTurn(state=next_state, encounter=select_encounter(next_state))
            self.query_one("#game", Static).update(render_tui_layout_snapshot(self.turn))

        def _append_message(self, message: str) -> None:
            state = replace(self.turn.state, log=[*self.turn.state.log, message])
            self.turn = TuiTurn(state=state, encounter=self.turn.encounter)
            self.query_one("#game", Static).update(render_tui_layout_snapshot(self.turn))

    OfficeEscapeApp().run()
