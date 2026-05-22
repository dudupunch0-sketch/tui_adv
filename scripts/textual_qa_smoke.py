#!/usr/bin/env python3
"""Reproducible Textual smoke for the checklist's actual TUI QA item."""

from __future__ import annotations

import argparse
import asyncio
import os
import sys
import tempfile
from pathlib import Path
from typing import Callable

REPO_ROOT = Path(__file__).resolve().parents[1]
SRC_ROOT = REPO_ROOT / "src"
if str(SRC_ROOT) not in sys.path:
    sys.path.insert(0, str(SRC_ROOT))

CASE_ORDER = ("start-save-slot",)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run actual Textual TUI QA smoke checks")
    parser.add_argument("--list", action="store_true", help="list available Textual QA cases")
    parser.add_argument(
        "--case",
        action="append",
        choices=CASE_ORDER,
        help="run only a named Textual QA case; may repeat",
    )
    args = parser.parse_args(argv)

    if args.list:
        for name in CASE_ORDER:
            print(name)
        return 0

    selected_cases = tuple(args.case or CASE_ORDER)
    failures: list[str] = []
    for case_name in selected_cases:
        try:
            run_case(case_name)
        except AssertionError as exc:
            print(f"FAIL {case_name}: {exc}")
            failures.append(case_name)
        except RuntimeError as exc:
            print(f"FAIL {case_name}: {exc}")
            failures.append(case_name)
        else:
            print(f"PASS {case_name}")
    return 1 if failures else 0


def run_case(case_name: str) -> None:
    cases: dict[str, Callable[[], None]] = {
        "start-save-slot": _run_start_save_slot_case,
    }
    cases[case_name]()


def _run_start_save_slot_case() -> None:
    _require_textual()
    asyncio.run(_run_load_flow())
    asyncio.run(_run_delete_flow())


async def _run_load_flow() -> None:
    from tui_adv.game.save import save_game_state
    from tui_adv.game.state import GameState
    from tui_adv.tui.app import build_office_escape_app

    with tempfile.TemporaryDirectory(prefix="tui-adv-textual-load-") as tmp_dir:
        tmp_path = Path(tmp_dir)
        old_save = tmp_path / "old.json"
        recent_save = tmp_path / "recent.json"
        save_game_state(GameState.new(seed=123, location_id="lobby"), old_save)
        save_game_state(GameState.new(seed=123, location_id="pantry"), recent_save)
        os.utime(old_save, (1_700_000_000, 1_700_000_000))
        os.utime(recent_save, (1_700_000_010, 1_700_000_010))

        app = build_office_escape_app(seed=123, save_path=tmp_path / "autosave.json")
        async with app.run_test() as pilot:
            await pilot.pause()
            controls = _widget_text(pilot.app, "panel-controls")
            if "숫자: 저장 파일 불러오기" not in controls:
                raise AssertionError("start screen did not show load prompt")
            if "1. recent.json — 턴 0 / 탕비실" not in controls:
                raise AssertionError("start screen did not list most recent save first")
            if not pilot.app.selecting_save_slot:
                raise AssertionError("app did not enter save-slot selection mode")

            await pilot.press("1")
            await pilot.pause()
            if pilot.app.selecting_save_slot:
                raise AssertionError("slot load did not leave selection mode")
            if pilot.app.turn.state.location_id != "pantry":
                raise AssertionError("slot load did not restore the selected save state")
            if Path(pilot.app.save_path).name != "recent.json":
                raise AssertionError("slot load did not switch save path to selected slot")


async def _run_delete_flow() -> None:
    from tui_adv.game.save import save_game_state
    from tui_adv.game.state import GameState
    from tui_adv.tui.app import build_office_escape_app

    with tempfile.TemporaryDirectory(prefix="tui-adv-textual-delete-") as tmp_dir:
        tmp_path = Path(tmp_dir)
        old_save = tmp_path / "old.json"
        recent_save = tmp_path / "recent.json"
        save_game_state(GameState.new(seed=123, location_id="lobby"), old_save)
        save_game_state(GameState.new(seed=123, location_id="pantry"), recent_save)
        os.utime(old_save, (1_700_000_000, 1_700_000_000))
        os.utime(recent_save, (1_700_000_010, 1_700_000_010))

        app = build_office_escape_app(seed=123, save_path=tmp_path / "autosave.json")
        async with app.run_test() as pilot:
            await pilot.pause()
            await pilot.press("d")
            await pilot.pause()
            controls = _widget_text(pilot.app, "panel-controls")
            if "숫자: 저장 파일 삭제" not in controls:
                raise AssertionError("delete mode did not show delete prompt")

            await pilot.press("1")
            await pilot.pause()
            if recent_save.exists():
                raise AssertionError("delete mode did not remove selected save")
            if not old_save.exists():
                raise AssertionError("delete mode removed the wrong save")
            if pilot.app.deleting_save_slot:
                raise AssertionError("delete mode did not reset after deletion")
            log_text = _widget_text(pilot.app, "panel-log")
            if "저장 슬롯 삭제: recent.json" not in log_text:
                raise AssertionError("delete mode did not append deletion log")

            await pilot.press("n")
            await pilot.pause()
            if pilot.app.selecting_save_slot:
                raise AssertionError("new-game key did not leave save-slot selection mode")


def _widget_text(app, widget_id: str) -> str:
    from textual.widgets import Static  # type: ignore[import-not-found]

    widget = app.query_one(f"#{widget_id}", Static)
    renderable = widget.renderable
    return getattr(renderable, "plain", str(renderable))


def _require_textual() -> None:
    try:
        import textual  # type: ignore[import-not-found]  # noqa: F401
    except ModuleNotFoundError as exc:
        raise RuntimeError(
            "Textual dependency is required. Install the project deps or run in a venv with textual>=0.85."
        ) from exc


if __name__ == "__main__":
    raise SystemExit(main())
