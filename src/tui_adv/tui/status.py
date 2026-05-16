from __future__ import annotations

from tui_adv.game.state import PlayerState


def _health_label(value: int) -> str:
    if value >= 70:
        return "정상 범위"
    if value >= 40:
        return "관찰 필요"
    if value > 0:
        return "위험"
    return "응답 없음"


def _sanity_label(value: int) -> str:
    if value >= 70:
        return "안정"
    if value >= 40:
        return "불안정"
    if value > 0:
        return "붕괴 위험"
    return "응답 없음"


def _water_label(value: int) -> str:
    if value < 30:
        return "최근 기록 있음"
    if value < 60:
        return "부족"
    if value < 80:
        return "없음"
    return "탈수 의심"


def _nutrition_label(value: int) -> str:
    if value < 30:
        return "정상"
    if value < 60:
        return "부족"
    if value < 80:
        return "위험"
    return "공복 한계"


def format_local_status(player: PlayerState) -> str:
    """Render resources as an in-world local diagnostic block."""

    return "\n".join(
        [
            "[LOCAL STATUS]",
            f"신체 반응: {_health_label(player.health)}",
            f"집중도: {_sanity_label(player.sanity)}",
            f"단말기 전원: {player.battery}%",
            f"최근 식수 기록: {_water_label(player.thirst)}",
            f"영양 상태: {_nutrition_label(player.hunger)}",
        ]
    )
