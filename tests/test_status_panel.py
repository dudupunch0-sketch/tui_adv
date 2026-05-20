from tui_adv.game.state import PlayerState
from tui_adv.tui.status import format_local_status, format_pressure_warnings


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


def test_format_pressure_warnings_lists_low_resource_triggers():
    rendered = format_pressure_warnings(
        PlayerState(health=75, sanity=30, battery=12, hunger=82, thirst=70)
    )

    assert "[압박 경고]" in rendered
    assert "선택지 왜곡" in rendered
    assert "단말기 전원" in rendered
    assert "정수기 환청" in rendered
    assert "영양 상태" in rendered
