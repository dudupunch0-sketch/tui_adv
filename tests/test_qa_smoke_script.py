import os
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


def run_qa_smoke(*args: str) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["PYTHONPATH"] = str(REPO_ROOT / "src")
    return subprocess.run(
        [sys.executable, "scripts/qa_smoke.py", *args],
        cwd=REPO_ROOT,
        env=env,
        text=True,
        capture_output=True,
        check=False,
    )


def test_qa_smoke_lists_phase10_release_cases():
    result = run_qa_smoke("--list")

    assert result.returncode == 0
    assert "escape-ending" in result.stdout
    assert "failure-ending" in result.stdout
    assert "hidden-hint" in result.stdout
    assert "save-load" in result.stdout
    assert "invalid-input" in result.stdout
    assert "secret-scan" in result.stdout
    assert "new-game-10" in result.stdout
    assert "terminal-size" in result.stdout


def test_qa_smoke_runs_invalid_input_case_without_traceback():
    result = run_qa_smoke("--case", "invalid-input")

    assert result.returncode == 0
    assert "PASS invalid-input" in result.stdout
    assert "Traceback" not in result.stdout
    assert "Traceback" not in result.stderr


def test_qa_smoke_runs_new_game_batch_case():
    result = run_qa_smoke("--case", "new-game-10")

    assert result.returncode == 0
    assert "PASS new-game-10" in result.stdout


def test_qa_smoke_runs_terminal_size_case():
    result = run_qa_smoke("--case", "terminal-size")

    assert result.returncode == 0
    assert "PASS terminal-size" in result.stdout


def test_textual_qa_smoke_lists_manual_cases():
    env = os.environ.copy()
    env["PYTHONPATH"] = str(REPO_ROOT / "src")
    result = subprocess.run(
        [sys.executable, "scripts/textual_qa_smoke.py", "--list"],
        cwd=REPO_ROOT,
        env=env,
        text=True,
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert "start-save-slot" in result.stdout
