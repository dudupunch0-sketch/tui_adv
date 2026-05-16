from tui_adv.game.state import PlayerState
from tui_adv.tui.status import format_local_status


def test_format_local_status_uses_company_system_language():
    player = PlayerState(health=82, sanity=61, battery=34, hunger=43, thirst=58)

    rendered = format_local_status(player)

    assert "[LOCAL STATUS]" in rendered
    assert "신체" in rendered
    assert "집중" in rendered
    assert "단말기 전원: 34%" in rendered
    assert "최근 식수 기록" in rendered
    assert "체력    82" not in rendered
    assert "정신력  61" not in rendered
