import os
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


def run_module(*args: str) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["PYTHONPATH"] = str(REPO_ROOT / "src")
    return subprocess.run(
        [sys.executable, "-m", "tui_adv", *args],
        cwd=REPO_ROOT,
        env=env,
        text=True,
        capture_output=True,
        check=False,
    )


def test_cli_new_game_smoke_prints_initial_status():
    result = run_module("--new", "--seed", "123")

    assert result.returncode == 0
    assert "escape from the office" in result.stdout
    assert "위치: 내 자리" in result.stdout
    assert "재난: unknown_isolation" in result.stdout
    assert "[LOCAL STATUS]" in result.stdout


def test_cli_version_prints_package_version():
    result = run_module("--version")

    assert result.returncode == 0
    assert result.stdout.strip().startswith("tui-adv ")
