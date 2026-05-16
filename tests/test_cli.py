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


def test_cli_new_game_actions_execute_multi_turn_route_and_print_ending():
    result = run_module(
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
        "move:server_room_front",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "== 턴 0 ==" in result.stdout
    assert "선택 실행: 메시지를 확인한다" in result.stdout
    assert "이동 실행: 개발팀 사무실" in result.stdout
    assert "이동 실행: 서버실 앞" in result.stdout
    assert "== 턴 5 ==" in result.stdout
    assert "엔딩: 사내 방송 장악" in result.stdout


def test_cli_escape_ending_smoke_prints_first_ending():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "emergency_stairs",
        "--flag",
        "escape_puzzle_ready",
        "--choice",
        "1",
    )

    assert result.returncode == 0
    assert "인카운터: 비상계단 공간 왜곡" in result.stdout
    assert "선택 실행: 반복되는 층수의 비밀을 풀고 문을 통과한다" in result.stdout
    assert "엔딩: 퇴근 성공" in result.stdout
    assert "공간 왜곡" in result.stdout


def test_cli_game_over_smoke_prints_spatial_failure():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "emergency_stairs",
        "--flag",
        "escape_puzzle_ready",
        "--choice",
        "2",
    )

    assert result.returncode == 0
    assert "선택 실행: 아래라고 믿고 계속 내려간다" in result.stdout
    assert "게임오버: 게임오버: 계단이 접혔다" in result.stdout
    assert "계단은 아래가 아니라 당신 안쪽" in result.stdout


def test_cli_tui_smoke_prints_textual_layout_snapshot():
    result = run_module("--tui-smoke", "--seed", "123")

    assert result.returncode == 0
    assert "escape from the office" in result.stdout
    assert "[LOCAL STATUS]" in result.stdout
    assert "[현재 인카운터]" in result.stdout
    assert "최근 로그" in result.stdout
    assert "1. 메시지를 확인한다" in result.stdout


def test_cli_version_prints_package_version():
    result = run_module("--version")

    assert result.returncode == 0
    assert result.stdout.strip().startswith("tui-adv ")
