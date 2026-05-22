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


def test_qa_smoke_runs_invalid_input_case_without_traceback():
    result = run_qa_smoke("--case", "invalid-input")

    assert result.returncode == 0
    assert "PASS invalid-input" in result.stdout
    assert "Traceback" not in result.stdout
    assert "Traceback" not in result.stderr
