from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Mapping

from tui_adv.game.state import GameState, PlayerState

SAVE_SCHEMA_VERSION = 1


def save_game_state(state: GameState, path: str | Path) -> None:
    """Write a game state to a stable JSON save file."""

    save_path = Path(path)
    save_path.parent.mkdir(parents=True, exist_ok=True)
    save_path.write_text(
        json.dumps(_state_to_save_data(state), ensure_ascii=False, indent=2, sort_keys=True)
        + "\n",
        encoding="utf-8",
    )


def load_game_state(path: str | Path) -> GameState:
    """Read a game state from a JSON save file."""

    raw_data = json.loads(Path(path).read_text(encoding="utf-8"))
    if not isinstance(raw_data, Mapping):
        raise ValueError("저장 파일 형식이 올바르지 않다")
    return _state_from_save_data(raw_data)


def _state_to_save_data(state: GameState) -> dict[str, Any]:
    return {
        "schema_version": SAVE_SCHEMA_VERSION,
        "state": {
            "seed": state.seed,
            "turn": state.turn,
            "location_id": state.location_id,
            "disaster_type": state.disaster_type,
            "danger": state.danger,
            "player": {
                "health": state.player.health,
                "sanity": state.player.sanity,
                "battery": state.player.battery,
                "hunger": state.player.hunger,
                "thirst": state.player.thirst,
                "abilities": dict(state.player.abilities),
            },
            "inventory": list(state.inventory),
            "clues": list(state.clues),
            "flags": list(state.flags),
            "seen_encounters": list(state.seen_encounters),
            "log": list(state.log),
        },
    }


def _state_from_save_data(data: Mapping[str, Any]) -> GameState:
    version = data.get("schema_version")
    if version != SAVE_SCHEMA_VERSION:
        raise ValueError(f"지원하지 않는 저장 파일 버전: {version}")
    state_data = data.get("state")
    if not isinstance(state_data, Mapping):
        raise ValueError("저장 파일에 state 객체가 없다")
    player_data = state_data.get("player")
    if not isinstance(player_data, Mapping):
        raise ValueError("저장 파일에 player 객체가 없다")
    abilities = player_data.get("abilities", {})
    if not isinstance(abilities, Mapping):
        raise ValueError("저장 파일의 abilities 객체가 올바르지 않다")
    return GameState(
        seed=int(state_data["seed"]),
        turn=int(state_data["turn"]),
        location_id=str(state_data["location_id"]),
        disaster_type=str(state_data["disaster_type"]),
        danger=int(state_data["danger"]),
        player=PlayerState(
            health=int(player_data["health"]),
            sanity=int(player_data["sanity"]),
            battery=int(player_data["battery"]),
            hunger=int(player_data["hunger"]),
            thirst=int(player_data["thirst"]),
            abilities={str(key): int(value) for key, value in abilities.items()},
        ),
        inventory=_string_list(state_data.get("inventory", [])),
        clues=_string_list(state_data.get("clues", [])),
        flags=_string_list(state_data.get("flags", [])),
        seen_encounters=_string_list(state_data.get("seen_encounters", [])),
        log=_string_list(state_data.get("log", [])),
    )


def _string_list(value: Any) -> list[str]:
    if not isinstance(value, list):
        raise ValueError("저장 파일의 목록 필드가 올바르지 않다")
    return [str(item) for item in value]
