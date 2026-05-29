import os
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


def run_module(*args: str, input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["PYTHONPATH"] = str(REPO_ROOT / "src")
    return subprocess.run(
        [sys.executable, "-m", "tui_adv", *args],
        cwd=REPO_ROOT,
        env=env,
        input=input_text,
        text=True,
        capture_output=True,
        check=False,
    )


def test_cli_play_mode_accepts_numbered_input_and_quit():
    result = run_module("--play", "--seed", "123", input_text="1\nq\n")

    assert result.returncode == 0
    assert "escape from the office - 직접 플레이" in result.stdout
    assert "입력: 번호 또는 action id" in result.stdout
    assert "선택 실행: 메시지를 확인한다" in result.stdout
    assert "게임을 종료한다" in result.stdout
    assert "Traceback" not in result.stderr


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


def test_cli_load_rejects_invalid_save_without_traceback(tmp_path):
    save_path = tmp_path / "future-save.json"
    save_path.write_text('{"schema_version": 999, "state": {}}', encoding="utf-8")

    result = run_module("--load", str(save_path))

    assert result.returncode == 2
    assert "지원하지 않는 저장 파일 버전: 999" in result.stderr
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


def test_cli_server_room_conquest_route_prints_network_admin_ending():
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
        "choice:3",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "선택 실행: 문틈의 찬 공기를 따라 안쪽으로 들어간다" in result.stdout
    assert "위치: 서버실 내부" in result.stdout
    assert "선택 실행: 관리자 콘솔에 격리 규칙을 덮어쓴다" in result.stdout
    assert "업적 달성: 사내망 관리자" in result.stdout
    assert "엔딩: 사내망 관리자 권한" in result.stdout


def test_cli_truth_route_prints_protocol_ending_and_achievement():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--action",
        "choice:3",
        "--action",
        "move:dev_office",
        "--action",
        "move:meeting_room",
        "--action",
        "choice:1",
        "--action",
        "move:dev_office",
        "--action",
        "move:hallway",
        "--action",
        "move:security_room",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "선택 실행: 전임자의 이름을 사내망에서 검색한다" in result.stdout
    assert "선택 실행: 회의록을 저장하고 패턴을 표시한다" in result.stdout
    assert "선택 실행: 지연된 CCTV 화면을 되감는다" in result.stdout
    assert "업적 달성: 격리 프로토콜 독해" in result.stdout
    assert "엔딩: 격리 프로토콜의 진실" in result.stdout


def test_cli_reality_hint_route_prints_copier_ip_digit_sum():
    result = run_module(
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
    )

    assert result.returncode == 0
    assert "선택 실행: 출력물을 챙긴다" in result.stdout
    assert "선택 실행: 커피머신 뒤를 본다" in result.stdout
    assert "엔딩: 첫 번째 현실 연결 힌트" in result.stdout
    assert "현실 연결 힌트: 첫 번째 현실 연결 힌트" in result.stdout
    assert "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다." in result.stdout
    assert "IP 주소:" in result.stdout
    assert "숫자 합계:" in result.stdout


def test_cli_escape_route_is_playable_without_manual_flag_injection():
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
        "move:emergency_stairs",
        "--action",
        "choice:1",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "인카운터: 비상계단 문틈 표식" in result.stdout
    assert "선택 실행: 계단문 틈의 숨소리와 층수 표시를 맞춘다" in result.stdout
    assert "위치: 비상계단" in result.stdout
    assert "선택 실행: 반복되는 층수의 비밀을 풀고 문을 통과한다" in result.stdout
    assert "엔딩: 퇴근 성공" in result.stdout


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


def test_cli_can_take_and_use_bottled_water_from_pantry():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "pantry",
        "--action",
        "choice:3",
        "--action",
        "use:bottled_water",
    )

    assert result.returncode == 0
    assert "선택 실행: 밀봉된 생수 한 병을 챙긴다" in result.stdout
    assert "아이템 사용: 생수" in result.stdout
    assert "생수를 마셨다." in result.stdout
    assert "== 턴 2 ==" in result.stdout


def test_cli_supply_closet_cache_can_collect_and_use_power_bank():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--action",
        "choice:1",
        "--action",
        "move:dev_office",
        "--action",
        "move:supply_closet",
        "--action",
        "choice:2",
        "--action",
        "use:power_bank",
    )

    assert result.returncode == 0
    assert "이동 실행: 물품창고" in result.stdout
    assert "인카운터: 물품창고 비상 보급함" in result.stdout
    assert "선택 실행: 보조배터리를 챙긴다" in result.stdout
    assert "아이템 사용: 보조배터리" in result.stdout
    assert "배터리: 97 -> 100" in result.stdout


def test_cli_schema_less_combat_prototype_runs_through_existing_choice_actions():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--action",
        "choice:1",
        "--action",
        "move:dev_office",
        "--action",
        "move:supply_closet",
        "--action",
        "choice:4",
        "--action",
        "choice:2",
    )

    assert result.returncode == 0
    assert "인카운터: 물품창고 비상 보급함" in result.stdout
    assert "선택 실행: 잠긴 물품 카트를 끌어 복도 쪽으로 세운다" in result.stdout
    assert "인카운터: 물품창고 자동 난투" in result.stdout
    assert "선택 실행: 캐비닛 손잡이에 카트를 걸어 거리를 만든다" in result.stdout
    assert "상대의 균형이 선반 쪽으로 밀렸다" in result.stdout


def test_cli_rooftop_signal_route_prints_escape_ending_and_achievement():
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
        "move:elevator_hall",
        "--action",
        "choice:1",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "이동 실행: 엘리베이터 홀" in result.stdout
    assert "선택 실행: 존재하지 않는 R층 버튼을 누른다" in result.stdout
    assert "위치: 옥상" in result.stdout
    assert "선택 실행: 제한된 외부 신호를 짧게 송신한다" in result.stdout
    assert "업적 달성: 외부 신호 송신" in result.stdout
    assert "엔딩: 옥상 외부 신호" in result.stdout


def test_cli_parking_lot_escape_route_prints_ending_and_achievement():
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
        "move:parking_lot",
        "--action",
        "choice:1",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "이동 실행: 지하주차장" in result.stdout
    assert "인카운터: 지하주차장의 시동음" in result.stdout
    assert "선택 실행: 켜져 있는 차의 키태그를 찾는다" in result.stdout
    assert "선택 실행: 주차장 차단기를 임시 개방한다" in result.stdout
    assert "업적 달성: 지하주차장 탈출자" in result.stdout
    assert "엔딩: 지하주차장 탈출" in result.stdout


def test_cli_lobby_gate_escape_route_prints_ending_and_achievement():
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
        "move:lobby",
        "--action",
        "choice:1",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "이동 실행: 로비" in result.stdout
    assert "인카운터: 무인 로비 안내 키오스크" in result.stdout
    assert "선택 실행: 방문증 프린터를 깨운다" in result.stdout
    assert "선택 실행: 방문증 바코드를 출구 게이트에 읽힌다" in result.stdout
    assert "업적 달성: 로비 게이트 통과자" in result.stdout
    assert "엔딩: 로비 회전문 탈출" in result.stdout


def test_cli_executive_approval_route_prints_conquest_ending_and_achievement():
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
        "move:lobby",
        "--action",
        "choice:2",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "이동 실행: 로비" in result.stdout
    assert "선택 실행: 대표실 호출 버튼을 길게 누른다" in result.stdout
    assert "위치: 대표실" in result.stdout
    assert "선택 실행: 대표 승인란에 내 이름을 입력한다" in result.stdout
    assert "업적 달성: 대표 승인권자" in result.stdout
    assert "엔딩: 대표 승인권 장악" in result.stdout


def test_cli_tui_smoke_actions_can_print_executive_conquest_ending():
    result = run_module(
        "--tui-smoke",
        "--seed",
        "123",
        "--action",
        "choice:1",
        "--action",
        "move:dev_office",
        "--action",
        "move:hallway",
        "--action",
        "move:lobby",
        "--action",
        "choice:2",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "[엔딩]" in result.stdout
    assert "엔딩: 대표 승인권 장악" in result.stdout
    assert "대표 승인" in result.stdout


def test_cli_resource_preload_can_trigger_thirst_hallucination_route():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "pantry",
        "--resource",
        "thirst=70",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "인카운터: 정수기의 이상한 물" in result.stdout
    assert "선택 실행: 물을 마신다" in result.stdout
    assert "정신력: 100 -> 92" in result.stdout
    assert "갈증: 70 -> 47" in result.stdout


def test_cli_security_override_route_prints_lockdown_conquest_ending():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "elevator_hall",
        "--action",
        "choice:2",
        "--action",
        "choice:1",
        "--action",
        "move:hallway",
        "--action",
        "move:server_room_front",
        "--action",
        "choice:4",
        "--action",
        "choice:3",
    )

    assert result.returncode == 0
    assert "인카운터: 어긋난 층수의 보안 콘솔" in result.stdout
    assert "선택 실행: 보안실 층수 로그에서 서버실 우회권한을 뽑는다" in result.stdout
    assert "선택 실행: 보안실 우회권한으로 서버실 문을 연다" in result.stdout
    assert "선택 실행: 출입 로그와 격리 규칙을 함께 잠근다" in result.stdout
    assert "엔딩: 보안 격리 권한 장악" in result.stdout


def test_cli_elevator_force_door_route_prints_security_room_bypass():
    result = run_module(
        "--new",
        "--seed",
        "123",
        "--location",
        "elevator_hall",
        "--action",
        "choice:2",
    )

    assert result.returncode == 0
    assert "선택 실행: 문틈을 벌려 현재 층으로 돌아온다" in result.stdout
    assert "위치: 보안실" in result.stdout
    assert "보안실 모니터" in result.stdout


def test_cli_tui_smoke_resource_preload_shows_distorted_low_sanity_choices():
    result = run_module(
        "--tui-smoke",
        "--seed",
        "123",
        "--resource",
        "sanity=30",
    )

    assert result.returncode == 0
    assert "집중도가 흔들려 선택지가 부분적으로 왜곡된다" in result.stdout
    assert "1. 메시▒를 확▒한다" in result.stdout


def test_cli_tui_smoke_prints_textual_layout_snapshot():
    result = run_module("--tui-smoke", "--seed", "123")

    assert result.returncode == 0
    assert "escape from the office" in result.stdout
    assert "[LOCAL STATUS]" in result.stdout
    assert "[현재 인카운터]" in result.stdout
    assert "최근 로그" in result.stdout
    assert "1. 메시지를 확인한다" in result.stdout


def test_cli_tui_smoke_actions_can_print_hidden_reality_hint_ending():
    result = run_module(
        "--tui-smoke",
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
    )

    assert result.returncode == 0
    assert "[엔딩]" in result.stdout
    assert "엔딩: 첫 번째 현실 연결 힌트" in result.stdout
    assert "현실 연결 힌트: 첫 번째 현실 연결 힌트" in result.stdout
    assert "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다." in result.stdout
    assert "숫자 합계: 33" in result.stdout


def test_cli_tui_smoke_actions_can_print_rooftop_signal_ending():
    result = run_module(
        "--tui-smoke",
        "--seed",
        "123",
        "--action",
        "choice:1",
        "--action",
        "move:dev_office",
        "--action",
        "move:hallway",
        "--action",
        "move:elevator_hall",
        "--action",
        "choice:1",
        "--action",
        "choice:1",
    )

    assert result.returncode == 0
    assert "[엔딩]" in result.stdout
    assert "엔딩: 옥상 외부 신호" in result.stdout
    assert "옥상" in result.stdout


def test_cli_can_save_new_smoke_state_and_load_it_for_later_actions(tmp_path):
    save_path = tmp_path / "office-save.json"

    saved = run_module(
        "--new",
        "--seed",
        "123",
        "--action",
        "choice:1",
        "--save",
        str(save_path),
    )

    assert saved.returncode == 0
    assert "업적 달성: 첫 신호 확인" in saved.stdout
    assert save_path.exists()
    assert f"저장: {save_path}" in saved.stdout

    loaded = run_module("--load", str(save_path), "--action", "move:dev_office")

    assert loaded.returncode == 0
    assert "== 턴 1 ==" in loaded.stdout
    assert "이동 실행: 개발팀 사무실" in loaded.stdout
    assert "== 턴 2 ==" in loaded.stdout
    assert "위치: 개발팀 사무실" in loaded.stdout
    assert "첫 신호 확인" in loaded.stdout


def test_cli_version_prints_package_version():
    result = run_module("--version")

    assert result.returncode == 0
    assert result.stdout.strip().startswith("tui-adv ")
