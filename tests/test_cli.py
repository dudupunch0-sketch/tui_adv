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


def test_cli_new_game_smoke_prints_initial_status_and_seeded_encounter():
    result = run_module("--new", "--seed", "123")

    assert result.returncode == 0
    assert "escape from the office" in result.stdout
    assert "위치: 내 자리" in result.stdout
    assert "재난: unknown_isolation" in result.stdout
    assert "[LOCAL STATUS]" in result.stdout
    assert "인카운터: 퇴사자의 메신저" in result.stdout
    assert "1. 메시지를 확인한다" in result.stdout


def test_cli_new_game_choice_index_executes_one_turn_and_prints_result():
    result = run_module("--new", "--seed", "123", "--choice", "1")

    assert result.returncode == 0
    assert "선택 실행: 메시지를 확인한다" in result.stdout
    assert "결과:" in result.stdout
    assert "퇴사자의 메시지를 확인했다." in result.stdout
    assert "배터리: 100 -> 97" in result.stdout
    assert "정신력: 100 -> 98" in result.stdout
    assert "허기: 0 -> 1" in result.stdout
    assert "갈증: 0 -> 2" in result.stdout
    assert "턴: 1" in result.stdout


def test_cli_new_game_rejects_invalid_choice_without_traceback():
    result = run_module("--new", "--seed", "123", "--choice", "99")

    assert result.returncode == 2
    assert "선택지를 찾을 수 없다: 99" in result.stderr
    assert "Traceback" not in result.stderr


def test_cli_version_prints_package_version():
    result = run_module("--version")

    assert result.returncode == 0
    assert result.stdout.strip().startswith("tui-adv ")
