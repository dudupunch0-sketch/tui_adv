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
