#!/usr/bin/env python3
"""Phase 10 QA/release smoke checks for escape from the office."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

REPO_ROOT = Path(__file__).resolve().parents[1]
PYTHON = sys.executable


@dataclass(frozen=True, slots=True)
class CommandCase:
    name: str
    args: tuple[str, ...]
    must_contain: tuple[str, ...]
    expected_returncode: int = 0
    stream: str = "stdout"


def _module_args(*args: str) -> tuple[str, ...]:
    return (PYTHON, "-m", "tui_adv", *args)


COMMAND_CASES: tuple[CommandCase, ...] = (
    CommandCase(
        name="escape-ending",
        args=_module_args(
            "--new",
            "--seed",
            "123",
            "--action",
            "choice:1",
            "--action",
            "move:dev_office",
            "--action",
            "move:hallway",
            "--action",
            "move:emergency_stairs",
            "--action",
            "choice:1",
            "--action",
            "choice:1",
        ),
        must_contain=("엔딩: 퇴근 성공",),
    ),
    CommandCase(
        name="failure-ending",
        args=_module_args(
            "--new",
            "--seed",
            "123",
            "--action",
            "choice:1",
            "--action",
            "move:dev_office",
            "--action",
            "move:hallway",
            "--action",
            "move:emergency_stairs",
            "--action",
            "choice:1",
            "--action",
            "choice:2",
        ),
        must_contain=("게임오버: 게임오버: 계단이 접혔다",),
    ),
    CommandCase(
        name="hidden-hint",
        args=_module_args(
            "--new",
            "--seed",
            "123",
            "--location",
            "printer_area",
            "--action",
            "choice:2",
            "--action",
            "move:pantry",
            "--action",
            "choice:3",
        ),
        must_contain=("엔딩: 첫 번째 현실 연결 힌트", "현실 연결 힌트:"),
    ),
    CommandCase(
        name="invalid-input",
        args=_module_args("--new", "--seed", "123", "--choice", "99"),
        expected_returncode=2,
        stream="stderr",
        must_contain=("선택지를 찾을 수 없다: 99",),
    ),
)

SPECIAL_CASES: dict[str, Callable[[], None]] = {}
CASE_ORDER = tuple(case.name for case in COMMAND_CASES) + ("save-load", "secret-scan")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run Phase 10 QA/release smoke checks")
    parser.add_argument("--list", action="store_true", help="list available QA cases")
    parser.add_argument(
        "--case",
        action="append",
        choices=CASE_ORDER,
        help="run only a named QA case; may repeat",
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
        else:
            print(f"PASS {case_name}")
    return 1 if failures else 0


def run_case(case_name: str) -> None:
    for case in COMMAND_CASES:
        if case.name == case_name:
            _run_command_case(case)
            return
    if case_name == "save-load":
        _run_save_load_case()
        return
    if case_name == "secret-scan":
        _run_secret_scan_case()
        return
    raise AssertionError(f"unknown case: {case_name}")


def _run_command_case(case: CommandCase) -> None:
    result = _run(case.args)
    if result.returncode != case.expected_returncode:
        raise AssertionError(
            f"exit {result.returncode}, expected {case.expected_returncode}\n"
            f"stdout:\n{_tail(result.stdout)}\nstderr:\n{_tail(result.stderr)}"
        )
    haystack = result.stdout if case.stream == "stdout" else result.stderr
    for expected in case.must_contain:
        if expected not in haystack:
            raise AssertionError(
                f"missing {expected!r} in {case.stream}\n"
                f"stdout:\n{_tail(result.stdout)}\nstderr:\n{_tail(result.stderr)}"
            )
    if "Traceback" in result.stdout or "Traceback" in result.stderr:
        raise AssertionError("unexpected traceback")


def _run_save_load_case() -> None:
    with tempfile.TemporaryDirectory(prefix="tui-adv-qa-") as tmp_dir:
        first_save = Path(tmp_dir) / "qa-save.json"
        second_save = Path(tmp_dir) / "qa-save-continued.json"
        first = _run(
            _module_args(
                "--new",
                "--seed",
                "123",
                "--action",
                "choice:1",
                "--save",
                str(first_save),
            )
        )
        _assert_success(first, "initial save")
        if not first_save.exists():
            raise AssertionError("initial save file was not written")
        second = _run(
            _module_args(
                "--load",
                str(first_save),
                "--action",
                "move:dev_office",
                "--save",
                str(second_save),
            )
        )
        _assert_success(second, "continued save")
        if not second_save.exists():
            raise AssertionError("continued save file was not written")
        if "위치: 개발팀 사무실" not in second.stdout:
            raise AssertionError("loaded route did not continue into dev_office")


def _run_secret_scan_case() -> None:
    tracked = _run(("git", "ls-files"))
    _assert_success(tracked, "git ls-files")
    forbidden_paths = [
        path
        for path in tracked.stdout.splitlines()
        if path.startswith("private/") or ".local." in Path(path).name
    ]
    if forbidden_paths:
        raise AssertionError(f"tracked private/local files: {forbidden_paths}")
    bundle_check = _run(
        (
            PYTHON,
            "scripts/export_web_data.py",
            "--check",
            "--bundle",
            "crates/escape-core/fixtures/content/content.bundle.json",
        )
    )
    _assert_success(bundle_check, "public export/bundle check")


def _assert_success(result: subprocess.CompletedProcess[str], label: str) -> None:
    if result.returncode != 0:
        raise AssertionError(
            f"{label} failed with exit {result.returncode}\n"
            f"stdout:\n{_tail(result.stdout)}\nstderr:\n{_tail(result.stderr)}"
        )
    if "Traceback" in result.stdout or "Traceback" in result.stderr:
        raise AssertionError(f"{label} printed traceback")


def _run(args: tuple[str, ...]) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["PYTHONPATH"] = str(REPO_ROOT / "src")
    return subprocess.run(
        args,
        cwd=REPO_ROOT,
        env=env,
        text=True,
        capture_output=True,
        check=False,
    )


def _tail(text: str, *, line_count: int = 20) -> str:
    lines = text.splitlines()
    return "\n".join(lines[-line_count:])


if __name__ == "__main__":
    raise SystemExit(main())
