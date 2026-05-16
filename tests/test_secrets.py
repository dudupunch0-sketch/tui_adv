from pathlib import Path

from tui_adv.game.secrets import reveal_physical_hint


def test_public_secret_reveal_omits_final_hint_without_local_file(tmp_path: Path):
    reveal = reveal_physical_hint("real_note_001", local_path=tmp_path / "missing.yaml")

    assert reveal.id == "real_note_001"
    assert reveal.final_hint is None
    assert reveal.final_hint_available is False
    assert reveal.public_hint_steps[-1].endswith("완성된다.")


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
