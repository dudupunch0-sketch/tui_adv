from __future__ import annotations

from dataclasses import dataclass, replace
from pathlib import Path

from tui_adv.game.achievements import (
    format_achievement_summary,
    format_unlocked_achievements,
    unlock_new_achievements,
)
from tui_adv.game.encounters import ChoiceResolution
from tui_adv.game.endings import Ending, evaluate_ending, format_ending_summary
from tui_adv.game.items import DEFAULT_ITEMS
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.loop import (
    GameTurn,
    TurnAction,
    TurnActionResult,
    build_game_turn,
    resolve_turn_action_result,
)
from tui_adv.game.save import load_game_state, save_game_state
from tui_adv.game.state import GameState
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn
from tui_adv.tui.status import format_local_status, format_pressure_warnings

TuiTurn = GameTurn
_MOVE_SHORTCUT_KEYS = tuple("adfghjklzxcvbrtyuop")
_RESERVED_TUI_KEYS = {"s", "q", "n", "i", "l", "?"}


@dataclass(frozen=True, slots=True)
class TuiChoiceResult:
    choice_label: str
    resolution: ChoiceResolution
    state: GameState
    ending: Ending | None


@dataclass(frozen=True, slots=True)
class SaveSlot:
    path: Path
    modified_time: float
    turn: int | None = None
    location_id: str | None = None
    location_name: str | None = None
    error: str | None = None


def discover_save_slots(directory: str | Path, *, limit: int = 5) -> tuple[SaveSlot, ...]:
    """Return recent JSON save files for the TUI start/save panel."""

    if limit <= 0:
        return ()
    save_dir = Path(directory)
    if not save_dir.exists() or not save_dir.is_dir():
        return ()

    candidates: list[tuple[Path, float]] = []
    for path in save_dir.glob("*.json"):
        try:
            if not path.is_file():
                continue
            modified_time = path.stat().st_mtime
        except OSError:
            continue
        candidates.append((path, modified_time))

    slots: list[SaveSlot] = []
    for path, modified_time in sorted(candidates, key=lambda item: item[1], reverse=True):
        try:
            state = load_game_state(path)
        except (OSError, ValueError, KeyError, TypeError, OverflowError) as exc:
            slots.append(SaveSlot(path=path, modified_time=modified_time, error=str(exc)))
        else:
            location = DEFAULT_LOCATIONS.get(state.location_id)
            slots.append(
                SaveSlot(
                    path=path,
                    modified_time=modified_time,
                    turn=state.turn,
                    location_id=state.location_id,
                    location_name=location.name if location else state.location_id,
                )
            )
        if len(slots) >= limit:
            break
    return tuple(slots)


def build_tui_turn(
    *,
    seed: int,
    location_id: str = "dev_desk",
    flags: tuple[str, ...] = (),
) -> GameTurn:
    if location_id not in DEFAULT_LOCATIONS:
        raise ValueError(f"알 수 없는 위치: {location_id}")
    state = replace(GameState.new(seed=seed, location_id=location_id), flags=list(flags))
    return build_game_turn(state)


def render_tui_layout_snapshot(
    turn: GameTurn,
    *,
    save_path: str | Path | None = None,
    save_slots: tuple[SaveSlot, ...] | None = None,
    start_mode: bool = False,
    detail_panel: str | None = None,
    delete_slot_mode: bool = False,
) -> str:
    """Render the same panels the Textual shell mounts, without requiring Textual."""

    location = DEFAULT_LOCATIONS[turn.state.location_id]
    lines = [
        "escape from the office",
        "",
        "[위치]",
        f"{location.name} — {location.description}",
        "",
        format_local_status(turn.state.player),
    ]
    if pressure_warnings := format_pressure_warnings(turn.state.player):
        lines.extend(["", pressure_warnings])
    lines.append("")
    if save_path is not None:
        lines.extend(
            [
                "[저장]",
                f"저장 파일: {Path(save_path)}",
                "s: 저장 / q: 종료",
                "",
            ]
        )
    if save_slots is not None:
        if start_mode:
            start_prompt = (
                "숫자: 저장 파일 삭제 / n: 새 게임"
                if delete_slot_mode
                else "숫자: 저장 파일 불러오기 / n: 새 게임 / d: 저장 파일 삭제 모드"
            )
            lines.extend(
                [
                    "[시작]",
                    start_prompt,
                    "",
                ]
            )
        lines.extend(_format_save_slot_panel(save_slots))
        lines.append("")
    lines.extend(_format_help_panel(turn, start_mode=start_mode, delete_slot_mode=delete_slot_mode))
    lines.append("")
    if achievement_summary := format_achievement_summary(turn.state):
        lines.extend([achievement_summary, ""])
    lines.extend(_format_inventory_and_clues(turn.state))
    if detail_lines := _format_detail_panel(turn, detail_panel):
        lines.extend(["", *detail_lines])
    lines.append("")
    if turn.ending is not None:
        lines.extend(["[엔딩]", format_ending_summary(turn.ending)])
    elif turn.encounter is not None:
        lines.extend(["[현재 인카운터]", format_encounter_turn(turn.encounter, turn.state)])
        _append_numbered_action_group(lines, turn, kind="item", title="소지품 사용")
    elif turn.available_actions:
        lines.append("[현재 행동]")
        _append_numbered_action_group(lines, turn, kind="move", title="이동")
        _append_numbered_action_group(lines, turn, kind="item", title="소지품 사용")
    else:
        lines.extend(["[현재 행동]", "가능한 행동 없음"])

    lines.extend(["", "[최근 로그]"])
    if turn.state.log:
        lines.extend(f"- {entry}" for entry in turn.state.log[-5:])
    else:
        lines.append("- 아직 기록 없음")
    return "\n".join(lines)


def _format_save_slot_panel(save_slots: tuple[SaveSlot, ...]) -> list[str]:
    lines = ["[저장 파일 목록]"]
    if not save_slots:
        lines.append("- 저장 파일 없음")
        return lines
    for index, slot in enumerate(save_slots, start=1):
        if slot.error is not None:
            lines.append(f"{index}. {slot.path.name} — 읽기 실패")
        else:
            lines.append(f"{index}. {slot.path.name} — 턴 {slot.turn} / {slot.location_name}")
    return lines


def _format_help_panel(
    turn: GameTurn,
    *,
    start_mode: bool = False,
    delete_slot_mode: bool = False,
) -> list[str]:
    lines = ["[도움말]"]
    if start_mode:
        if delete_slot_mode:
            lines.append("숫자: 저장 파일 삭제 / n: 새 게임")
        else:
            lines.append("숫자: 저장 파일 불러오기 / n: 새 게임 / d: 저장 파일 삭제 모드")
    else:
        lines.append("숫자: 현재 선택/행동 실행")
    lines.append("?: 도움말 / i: 소지품/단서 / l: 최근 로그 / s: 저장 / q: 종료")
    shortcuts = movement_shortcuts_for_turn(turn)
    if shortcuts:
        shortcut_text = " / ".join(
            f"{key}={action.label}" for key, action in shortcuts.items()
        )
        lines.append(f"이동 단축키: {shortcut_text}")
    return lines


def movement_shortcuts_for_turn(turn: GameTurn) -> dict[str, TurnAction]:
    move_actions = [action for action in turn.available_actions if action.kind == "move"]
    shortcuts: dict[str, TurnAction] = {}
    shortcut_keys = (key for key in _MOVE_SHORTCUT_KEYS if key not in _RESERVED_TUI_KEYS)
    for action in move_actions:
        try:
            key = next(shortcut_keys)
        except StopIteration:
            break
        shortcuts[key] = action
    return shortcuts


def _append_numbered_action_group(
    lines: list[str],
    turn: GameTurn,
    *,
    kind: str,
    title: str,
) -> None:
    indexed_actions = [
        (index, action)
        for index, action in enumerate(turn.available_actions, start=1)
        if action.kind == kind
    ]
    if not indexed_actions:
        return
    lines.append(f"{title}:")
    for index, action in indexed_actions:
        lines.append(f"{index}. {action.label}")


def _format_inventory_and_clues(state: GameState) -> list[str]:
    lines = ["[소지품]"]
    lines.extend(
        _format_limited_list(
            state.inventory,
            empty="없음",
            limit=5,
            formatter=_format_inventory_item,
        )
    )
    lines.append("")
    lines.append("[단서]")
    lines.extend(_format_limited_list(state.clues, empty="아직 확보한 단서 없음", limit=3))
    return lines


def _format_limited_list(
    values: list[str],
    *,
    empty: str,
    limit: int,
    formatter=None,
) -> list[str]:
    if not values:
        return [f"- {empty}"]
    visible = values[:limit]
    format_value = formatter or (lambda value: value)
    lines = [f"- {format_value(value)}" for value in visible]
    remaining_count = len(values) - len(visible)
    if remaining_count > 0:
        lines.append(f"+{remaining_count} more")
    return lines


def _format_inventory_item(item_id: str) -> str:
    item = DEFAULT_ITEMS.get(item_id)
    if item is None:
        return item_id
    return f"{item.name} ({item.id})"


def _format_detail_panel(turn: GameTurn, detail_panel: str | None) -> list[str]:
    if detail_panel is None:
        return []
    if detail_panel == "help":
        return _format_help_detail_panel(turn)
    if detail_panel == "inventory":
        return _format_inventory_detail_panel(turn.state)
    if detail_panel == "log":
        return _format_log_detail_panel(turn.state)
    raise ValueError(f"알 수 없는 상세 패널: {detail_panel}")


def _format_help_detail_panel(turn: GameTurn) -> list[str]:
    lines = [
        "[상세 도움말]",
        "- 숫자: 현재 선택지, 이동, 소지품 사용을 실행한다.",
        "- i: 소지품/단서 상세",
        "- l: 최근 로그 상세",
        "- ?: 이 도움말 다시 보기",
        "- s: 현재 저장 경로에 저장",
        "- q: 종료",
    ]
    shortcuts = movement_shortcuts_for_turn(turn)
    if shortcuts:
        shortcut_text = " / ".join(
            f"{key}={action.label}" for key, action in shortcuts.items()
        )
        lines.append(f"- 이동 단축키: {shortcut_text}")
    return lines


def _format_inventory_detail_panel(state: GameState) -> list[str]:
    lines = ["[상세 소지품]"]
    if state.inventory:
        lines.extend(_format_inventory_item_detail(item_id) for item_id in state.inventory)
    else:
        lines.append("- 없음")
    lines.extend(["", "[상세 단서]"])
    if state.clues:
        lines.extend(f"- {clue}" for clue in state.clues)
    else:
        lines.append("- 아직 확보한 단서 없음")
    return lines


def _format_inventory_item_detail(item_id: str) -> str:
    item = DEFAULT_ITEMS.get(item_id)
    if item is None:
        return f"- {item_id}: 등록되지 않은 물품"
    return f"- {item.name} ({item.id}): {item.description}"


def _format_log_detail_panel(state: GameState) -> list[str]:
    lines = ["[상세 로그]"]
    if not state.log:
        lines.append("- 아직 기록 없음")
        return lines
    lines.extend(f"{index}. {entry}" for index, entry in enumerate(state.log, start=1))
    return lines


def resolve_tui_save_slot(save_slots: tuple[SaveSlot, ...], slot_index: int) -> GameState:
    selected_index = slot_index - 1
    if selected_index < 0 or selected_index >= len(save_slots):
        raise ValueError(f"저장 슬롯을 찾을 수 없다: {slot_index}")
    slot = save_slots[selected_index]
    if slot.error is not None:
        raise ValueError(f"저장 슬롯을 읽을 수 없다: {slot.path.name}")
    return load_game_state(slot.path)


def delete_tui_save_slot(save_slots: tuple[SaveSlot, ...], slot_index: int) -> Path:
    selected_index = slot_index - 1
    if selected_index < 0 or selected_index >= len(save_slots):
        raise ValueError(f"저장 슬롯을 찾을 수 없다: {slot_index}")
    path = save_slots[selected_index].path
    try:
        path.unlink()
    except OSError as exc:
        raise ValueError(f"저장 슬롯을 삭제할 수 없다: {path.name}") from exc
    return path


def resolve_tui_action(turn: GameTurn, action_index: int) -> TurnActionResult:
    selected_index = action_index - 1
    if selected_index < 0 or selected_index >= len(turn.available_actions):
        raise ValueError(f"행동을 찾을 수 없다: {action_index}")
    action = turn.available_actions[selected_index]
    return resolve_turn_action_result(turn, action.id)


def resolve_tui_key(turn: GameTurn, key: str) -> TurnActionResult:
    if key.isdecimal():
        return resolve_tui_action(turn, int(key))
    shortcuts = movement_shortcuts_for_turn(turn)
    action = shortcuts.get(key.lower())
    if action is None:
        raise ValueError(f"행동 단축키를 찾을 수 없다: {key}")
    return resolve_turn_action_result(turn, action.id)


def resolve_tui_choice(turn: GameTurn, choice_index: int) -> TuiChoiceResult:
    if turn.encounter is None:
        raise ValueError("현재 인카운터가 없다")
    choices = turn.encounter.available_choices(turn.state)
    selected_index = choice_index - 1
    if selected_index < 0 or selected_index >= len(choices):
        raise ValueError(f"선택지를 찾을 수 없다: {choice_index}")
    choice = choices[selected_index]
    resolution = turn.encounter.resolve_choice_result(choice.id, turn.state)
    unlock_result = unlock_new_achievements(resolution.state)
    resolution = replace(resolution, state=unlock_result.state)
    ending = evaluate_ending(resolution.state)
    return TuiChoiceResult(
        choice_label=choice.label,
        resolution=resolution,
        state=resolution.state,
        ending=ending,
    )


def save_tui_turn_state(
    turn: GameTurn,
    save_path: str | Path,
    *,
    message_prefix: str = "저장",
) -> GameTurn:
    saved_path = Path(save_path)
    saved_state = replace(
        turn.state,
        inventory=list(turn.state.inventory),
        clues=list(turn.state.clues),
        flags=list(turn.state.flags),
        seen_encounters=list(turn.state.seen_encounters),
        unlocked_achievements=list(turn.state.unlocked_achievements),
        log=[*turn.state.log, f"{message_prefix}: {saved_path}"],
    )
    save_game_state(saved_state, saved_path)
    return build_game_turn(saved_state)


def _save_slots_for_path(save_path: str | Path | None) -> tuple[SaveSlot, ...] | None:
    if save_path is None:
        return None
    return discover_save_slots(Path(save_path).parent)


def run_textual_tui(
    *,
    seed: int,
    location_id: str = "dev_desk",
    flags: tuple[str, ...] = (),
    initial_state: GameState | None = None,
    save_path: str | Path | None = None,
) -> None:
    """Launch the interactive Textual shell when the optional dependency exists."""

    try:
        from textual.app import App, ComposeResult  # type: ignore[import-not-found]
        from textual.widgets import Footer, Header, Static  # type: ignore[import-not-found]
    except ModuleNotFoundError as exc:  # pragma: no cover - depends on local install.
        raise RuntimeError(
            "Textual이 설치되어 있지 않다. 패키지를 설치한 환경에서 --tui를 실행하라."
        ) from exc

    class OfficeEscapeApp(App[None]):  # pragma: no cover - interactive shell smoke only.
        BINDINGS = [("s", "save_game", "저장"), ("q", "quit", "종료")]
        CSS = """
        Screen { layout: vertical; }
        #game { height: 1fr; overflow-y: auto; padding: 1 2; }
        """

        def __init__(self) -> None:
            super().__init__()
            self.save_path = save_path
            self.save_slots = _save_slots_for_path(save_path) or ()
            self.selecting_save_slot = initial_state is None and bool(self.save_slots)
            self.deleting_save_slot = False
            self.detail_panel: str | None = None
            self.turn = (
                build_game_turn(initial_state)
                if initial_state is not None
                else build_tui_turn(seed=seed, location_id=location_id, flags=flags)
            )

        def compose(self) -> ComposeResult:
            yield Header(show_clock=True)
            yield Static(
                render_tui_layout_snapshot(
                    self.turn,
                    save_path=self.save_path,
                    save_slots=self.save_slots,
                    start_mode=self.selecting_save_slot,
                    detail_panel=self.detail_panel,
                    delete_slot_mode=self.deleting_save_slot,
                ),
                id="game",
            )
            yield Footer()

        def on_key(self, event) -> None:
            if self.selecting_save_slot:
                if event.key == "n":
                    self.selecting_save_slot = False
                    self.deleting_save_slot = False
                    self._refresh_game_panel()
                    return
                if event.key == "d":
                    self.deleting_save_slot = True
                    self._refresh_game_panel()
                    return
                if not event.key.isdecimal():
                    return
                try:
                    slot_index = int(event.key)
                    if self.deleting_save_slot:
                        deleted_path = delete_tui_save_slot(self.save_slots, slot_index)
                        self.save_slots = discover_save_slots(deleted_path.parent)
                        self.deleting_save_slot = False
                        self._append_message(f"저장 슬롯 삭제: {deleted_path.name}")
                        return
                    loaded_state = resolve_tui_save_slot(self.save_slots, slot_index)
                except ValueError as exc:
                    self._append_message(str(exc))
                    return
                self.save_path = self.save_slots[slot_index - 1].path
                self.turn = build_game_turn(loaded_state)
                self.selecting_save_slot = False
                self.deleting_save_slot = False
                self._refresh_game_panel()
                return
            if self.turn.ending is not None:
                return
            if event.key in {"?", "question_mark"}:
                self.detail_panel = "help"
                self._refresh_game_panel()
                return
            if event.key == "i":
                self.detail_panel = "inventory"
                self._refresh_game_panel()
                return
            if event.key == "l":
                self.detail_panel = "log"
                self._refresh_game_panel()
                return
            if event.key in _RESERVED_TUI_KEYS:
                return
            try:
                result = resolve_tui_key(self.turn, event.key)
            except ValueError as exc:
                self._append_message(str(exc))
                return
            log = [_format_tui_action_result(result)]
            next_state = replace(result.turn.state, log=[*result.turn.state.log, *log])
            self.turn = build_game_turn(next_state)
            self.detail_panel = None
            if self.save_path is not None:
                self.turn = save_tui_turn_state(
                    self.turn,
                    self.save_path,
                    message_prefix="자동 저장",
                )
                self.save_slots = _save_slots_for_path(self.save_path) or ()
            self._refresh_game_panel()

        def action_save_game(self) -> None:
            if self.save_path is None:
                self._append_message("저장 파일 경로가 설정되지 않았다.")
                return
            self.turn = save_tui_turn_state(self.turn, self.save_path)
            self.save_slots = _save_slots_for_path(self.save_path) or ()
            self._refresh_game_panel()

        def _append_message(self, message: str) -> None:
            state = replace(self.turn.state, log=[*self.turn.state.log, message])
            self.turn = build_game_turn(state)
            self._refresh_game_panel()

        def _refresh_game_panel(self) -> None:
            self.query_one("#game", Static).update(
                render_tui_layout_snapshot(
                    self.turn,
                    save_path=self.save_path,
                    save_slots=self.save_slots,
                    start_mode=self.selecting_save_slot,
                    detail_panel=self.detail_panel,
                    delete_slot_mode=self.deleting_save_slot,
                )
            )

    OfficeEscapeApp().run()


def _format_tui_action_result(result: TurnActionResult) -> str:
    if result.action.kind == "choice":
        lines = [f"선택 실행: {result.action.label}"]
        if result.choice_resolution is not None:
            lines.append(format_choice_resolution(result.choice_resolution))
    elif result.action.kind == "move":
        lines = [f"이동 실행: {result.action.label}"]
    elif result.action.kind == "item":
        lines = [f"아이템 사용: {result.action.label}"]
        if result.item_use_result is not None:
            lines.extend(result.item_use_result.new_logs)
    else:
        lines = [f"행동 실행: {result.action.label}"]

    achievement_text = format_unlocked_achievements(result.unlocked_achievements)
    if achievement_text:
        lines.extend(["", achievement_text])
    return "\n".join(lines)
