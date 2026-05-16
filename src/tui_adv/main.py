from __future__ import annotations

import argparse

from tui_adv import __version__
from tui_adv.game.state import GameState
from tui_adv.tui.status import format_local_status

LOCATION_LABELS = {
    "desk": "내 자리",
}


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="tui-adv",
        description="escape from the office command line entry point",
    )
    parser.add_argument("--version", action="store_true", help="print package version")
    parser.add_argument("--new", action="store_true", help="start a new smoke-test game")
    parser.add_argument("--seed", type=int, default=0, help="random seed for a new game")
    return parser


def render_new_game_smoke(state: GameState) -> str:
    location = LOCATION_LABELS.get(state.location_id, state.location_id)
    return "\n".join(
        [
            "escape from the office",
            f"위치: {location}",
            f"재난: {state.disaster_type}",
            format_local_status(state.player),
        ]
    )


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.version:
        print(f"tui-adv {__version__}")
        return 0

    if args.new:
        state = GameState.new(seed=args.seed)
        print(render_new_game_smoke(state))
        return 0

    parser.print_help()
    return 0
