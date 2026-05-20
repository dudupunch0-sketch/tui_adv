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


def format_pressure_warnings(player: PlayerState) -> str:
    """Render active resource pressure warnings for the TUI side panel."""

    warnings: list[str] = []
    if player.health < 40:
        warnings.append("신체 반응: 사내 의료 챗봇이 응답을 재시도한다.")
    if player.should_distort_choices:
        warnings.append("집중도: 선택지 왜곡이 시작된다.")
    if player.battery <= 15:
        warnings.append("단말기 전원: 제한된 인터넷 접속도 곧 끊길 수 있다.")
    if player.should_trigger_thirst_hallucination:
        warnings.append("최근 식수 기록: 정수기 환청이 가까운 위치를 가리킨다.")
    if player.hunger >= 80:
        warnings.append("영양 상태: 공복 한계로 장기 행동이 불안정하다.")
    if not warnings:
        return ""
    return "\n".join(["[압박 경고]", *(f"- {warning}" for warning in warnings)])
