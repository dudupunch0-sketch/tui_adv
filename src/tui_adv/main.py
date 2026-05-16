from __future__ import annotations

import argparse

from tui_adv import __version__
from tui_adv.game.encounters import Choice, select_encounter
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.state import GameState
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn
from tui_adv.tui.status import format_local_status


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="tui-adv",
        description="escape from the office command line entry point",
    )
    parser.add_argument("--version", action="store_true", help="print package version")
    parser.add_argument("--new", action="store_true", help="start a new smoke-test game")
    parser.add_argument("--seed", type=int, default=0, help="random seed for a new game")
    parser.add_argument(
        "--choice",
        help="execute an available choice by 1-based index during --new smoke",
    )
    return parser


def render_new_game_smoke(state: GameState, choice_argument: str | None = None) -> str:
    location_info = DEFAULT_LOCATIONS.get(state.location_id)
    location = location_info.name if location_info else state.location_id
    lines = [
        "escape from the office",
        f"위치: {location}",
        f"재난: {state.disaster_type}",
        format_local_status(state.player),
    ]
    encounter = select_encounter(state)
    if encounter is None:
        return "\n".join(lines)

    lines.extend(["", format_encounter_turn(encounter, state)])
    if choice_argument is not None:
        choice = _choice_from_argument(encounter.available_choices(state), choice_argument)
        resolution = encounter.resolve_choice_result(choice.id, state)
        lines.extend(
            [
                "",
                f"선택 실행: {choice.label}",
                format_choice_resolution(resolution),
                "",
                f"턴: {resolution.state.turn}",
                format_local_status(resolution.state.player),
            ]
        )
    return "\n".join(lines)


def _choice_from_argument(choices: tuple[Choice, ...], argument: str) -> Choice:
    if argument.isdecimal():
        index = int(argument) - 1
        if 0 <= index < len(choices):
            return choices[index]
    raise ValueError(f"선택지를 찾을 수 없다: {argument}")


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.version:
        print(f"tui-adv {__version__}")
        return 0

    if args.new:
        state = GameState.new(seed=args.seed)
        try:
            print(render_new_game_smoke(state, choice_argument=args.choice))
        except ValueError as exc:
            parser.error(str(exc))
        return 0

    if args.choice:
        parser.error("--choice requires --new")

    parser.print_help()
    return 0
