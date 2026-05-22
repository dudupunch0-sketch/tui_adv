from pathlib import Path


def test_checklist_tracks_completed_terminal_ux_slices():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    assert "- [x] 위험도 변화 규칙 적용" in checklist
    assert "- [x] 선택 불가 선택지 이유 표시" in checklist


def test_readme_next_work_does_not_list_completed_unavailable_reason_slice():
    readme = Path("README.md").read_text(encoding="utf-8")
    next_work = readme.split("## 다음 작업 후보", 1)[1]

    assert "선택 불가 선택지의 이유 표시" not in next_work
    assert "색상 테마" in next_work
