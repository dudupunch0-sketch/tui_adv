from __future__ import annotations

import argparse

from tui_adv import __version__


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="tui-adv",
        description="escape from the office — CLI deprecated; use the Web player",
    )
    parser.add_argument("--version", action="store_true", help="print package version")
    args, _unknown = parser.parse_known_args(argv)

    if args.version:
        print(f"tui-adv {__version__}")
        return 0

    print(
        "이 CLI는 deprecated됐습니다.\n"
        "Web player를 사용하세요: https://github.com/dudupunch0-sketch/tui_adv"
    )
    return 0
