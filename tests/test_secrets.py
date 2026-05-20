from pathlib import Path

from tui_adv.game.secrets import reveal_physical_hint, sum_ip_address_digits


def test_ip_address_digit_sum_ignores_dots():
    assert sum_ip_address_digits("192.168.0.42") == 33


def test_public_secret_reveal_omits_final_hint_without_local_file(tmp_path: Path):
    reveal = reveal_physical_hint("real_note_001", local_path=tmp_path / "missing.yaml")

    assert reveal.id == "real_note_001"
    assert reveal.final_hint is None
    assert reveal.final_hint_available is False
    assert reveal.public_hint_steps[-1].endswith("완성된다.")
    assert reveal.puzzle_prompt == "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다."
    assert reveal.puzzle_ip_address == "192.168.0.42"
    assert reveal.puzzle_answer == 33


def test_second_public_secret_uses_maintenance_label_placeholder(tmp_path: Path):
    reveal = reveal_physical_hint("real_note_002", local_path=tmp_path / "missing.yaml")

    assert reveal.id == "real_note_002"
    assert reveal.final_hint_available is False
    assert reveal.title == "두 번째 현실 연결 힌트"
    assert reveal.puzzle_prompt == "커피머신 점검 라벨의 IP 주소 숫자들을 모두 더한다."
    assert reveal.puzzle_ip_address == "10.30.0.45"
    assert reveal.puzzle_answer == 13


def test_local_secret_file_merges_safety_checked_final_hint(tmp_path: Path):
    local_file = tmp_path / "secrets.local.yaml"
    local_file.write_text(
        """
secrets:
  - id: real_note_001
    final_hint: 로컬에서만 보이는 마지막 위치 확인 문장
    safety_checked: true
    notes: 공용 공간, 위험 없음, 개인 물건 아님.
""".strip(),
        encoding="utf-8",
    )

    reveal = reveal_physical_hint("real_note_001", local_path=local_file)

    assert reveal.final_hint_available is True
    assert reveal.final_hint == "로컬에서만 보이는 마지막 위치 확인 문장"


def test_local_secret_can_override_placeholder_ip_digit_sum(tmp_path: Path):
    local_file = tmp_path / "secrets.local.yaml"
    local_file.write_text(
        """
secrets:
  - id: real_note_001
    final_hint: 로컬에서만 보이는 마지막 위치 확인 문장
    actual_ip_address: 10.20.30.40
    safety_checked: true
    notes: 실제 IP 주소는 로컬 비공개 파일에만 둔다.
""".strip(),
        encoding="utf-8",
    )

    reveal = reveal_physical_hint("real_note_001", local_path=local_file)

    assert reveal.puzzle_ip_address == "10.20.30.40"
    assert reveal.puzzle_answer == 10


def test_local_secret_without_safety_check_is_not_revealed(tmp_path: Path):
    local_file = tmp_path / "secrets.local.yaml"
    local_file.write_text(
        """
secrets:
  - id: real_note_001
    final_hint: 안전 확인 없이 표시되면 안 되는 문장
    safety_checked: false
""".strip(),
        encoding="utf-8",
    )

    reveal = reveal_physical_hint("real_note_001", local_path=local_file)

    assert reveal.final_hint_available is False
    assert reveal.final_hint is None
