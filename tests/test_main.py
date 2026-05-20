import os

from tui_adv.game.save import save_game_state
from tui_adv.game.state import GameState
import tui_adv.main as main_module


def test_tui_mode_accepts_loaded_state_and_save_path(monkeypatch, tmp_path):
    load_path = tmp_path / "loaded-save.json"
    autosave_path = tmp_path / "autosave.json"
    loaded_state = GameState.new(seed=77, location_id="pantry")
    save_game_state(loaded_state, load_path)
    captured_kwargs = {}

    def fake_run_textual_tui(**kwargs):
        captured_kwargs.update(kwargs)

    monkeypatch.setattr(main_module, "run_textual_tui", fake_run_textual_tui)

    result = main_module.main(
        ["--tui", "--load", str(load_path), "--save", str(autosave_path)]
    )

    assert result == 0
    assert captured_kwargs["initial_state"] == loaded_state
    assert captured_kwargs["save_path"] == str(autosave_path)


def test_tui_smoke_save_path_renders_discovered_save_slots(capsys, tmp_path):
    old_save = tmp_path / "old.json"
    broken_save = tmp_path / "broken.json"
    autosave_path = tmp_path / "autosave.json"
    save_game_state(GameState.new(seed=77, location_id="lobby"), old_save)
    broken_save.write_text("not json", encoding="utf-8")

    result = main_module.main(["--tui-smoke", "--seed", "123", "--save", str(autosave_path)])

    output = capsys.readouterr().out
    assert result == 0
    assert "[저장 파일 목록]" in output
    assert "old.json — 턴 0 / 로비" in output
    assert "broken.json — 읽기 실패" in output
    assert f"저장: {autosave_path}" in output
    assert autosave_path.exists()


def test_tui_smoke_can_delete_numbered_save_slot_before_rendering(capsys, tmp_path):
    old_save = tmp_path / "old.json"
    recent_save = tmp_path / "recent.json"
    autosave_path = tmp_path / "autosave.save"
    save_game_state(GameState.new(seed=77, location_id="pantry"), old_save)
    save_game_state(GameState.new(seed=77, location_id="lobby"), recent_save)
    os.utime(old_save, (1_700_000_000, 1_700_000_000))
    os.utime(recent_save, (1_700_000_010, 1_700_000_010))

    result = main_module.main(
        [
            "--tui-smoke",
            "--seed",
            "123",
            "--save",
            str(autosave_path),
            "--delete-save-slot",
            "1",
        ]
    )

    output = capsys.readouterr().out
    assert result == 0
    assert "저장 슬롯 삭제: recent.json" in output
    assert "recent.json — 턴 0 / 로비" not in output
    assert "old.json — 턴 0 / 탕비실" in output
    assert not recent_save.exists()


def test_third_reality_hint_route_cli_smoke(capsys):
    result = main_module.main(
        [
            "--new",
            "--seed",
            "123",
            "--location",
            "printer_area",
            "--action",
            "choice:1",
            "--action",
            "move:dev_office",
            "--action",
            "move:meeting_room",
            "--action",
            "choice:1",
        ]
    )

    output = capsys.readouterr().out
    assert result == 0
    assert "세 번째 현실 연결 힌트" in output
    assert "화이트보드" in output
    assert "숫자 합계: 30" in output
