from __future__ import annotations

import argparse
from dataclasses import replace
from pathlib import Path

from tui_adv import __version__
from tui_adv.game.achievements import (
    format_achievement_summary,
    format_unlocked_achievements,
    unlock_new_achievements,
)
from tui_adv.game.encounters import Choice, select_encounter
from tui_adv.game.endings import evaluate_ending, format_ending_summary
from tui_adv.game.items import ItemUseResult
from tui_adv.game.locations import DEFAULT_LOCATIONS
from tui_adv.game.loop import (
    GameTurn,
    TurnActionResult,
    build_game_turn,
    resolve_turn_action_result,
)
from tui_adv.game.save import load_game_state, save_game_state
from tui_adv.game.state import GameState
from tui_adv.tui.app import (
    delete_tui_save_slot,
    discover_save_slots,
    render_tui_layout_snapshot,
    run_textual_tui,
)
from tui_adv.tui.encounter import format_choice_resolution, format_encounter_turn
from tui_adv.tui.status import format_local_status


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="tui-adv",
        description="escape from the office command line entry point",
    )
    parser.add_argument("--version", action="store_true", help="print package version")
    parser.add_argument("--new", action="store_true", help="start a new smoke-test game")
    parser.add_argument("--seed", type=int, default=0, help="random seed for a new game")
    parser.add_argument(
        "--choice",
        help="execute an available choice by 1-based index during --new smoke",
    )
    parser.add_argument(
        "--action",
        action="append",
        default=[],
        help="execute a scripted multi-turn action during --new smoke, e.g. choice:1 or move:dev_office; may repeat",
    )
    parser.add_argument(
        "--location",
        default="dev_desk",
        help="starting location id for --new smoke",
    )
    parser.add_argument(
        "--flag",
        action="append",
        default=[],
        help="preload a state flag for deterministic smoke paths; may repeat",
    )
    parser.add_argument(
        "--resource",
        action="append",
        default=[],
        help="preload a resource as name=value for deterministic smoke paths; may repeat",
    )
    parser.add_argument("--save", help="write the final smoke game state to a JSON file")
    parser.add_argument("--load", help="load a JSON smoke game state instead of starting new")
    parser.add_argument("--tui", action="store_true", help="launch the interactive Textual TUI")
    parser.add_argument(
        "--tui-smoke",
        action="store_true",
        help="print the Textual layout model without launching an interactive screen",
    )
    parser.add_argument(
        "--delete-save-slot",
        type=int,
        help="delete a numbered TUI save slot during --tui-smoke --save rendering",
    )
    return parser


def render_new_game_smoke(state: GameState, choice_argument: str | None = None) -> str:
    output, _ = render_new_game_smoke_result(state, choice_argument=choice_argument)
    return output


def render_new_game_smoke_result(
    state: GameState,
    choice_argument: str | None = None,
) -> tuple[str, GameState]:
    location_info = DEFAULT_LOCATIONS.get(state.location_id)
    location = location_info.name if location_info else state.location_id
    final_state = state
    lines = [
        "escape from the office",
        f"위치: {location}",
        f"재난: {state.disaster_type}",
        format_local_status(state.player),
    ]
    encounter = select_encounter(state)
    if encounter is None:
        return "\n".join(lines), final_state

    lines.extend(["", format_encounter_turn(encounter, state)])
    if choice_argument is not None:
        choice = _choice_from_argument(encounter.available_choices(state), choice_argument)
        resolution = encounter.resolve_choice_result(choice.id, state)
        unlock_result = unlock_new_achievements(resolution.state)
        resolution = replace(resolution, state=unlock_result.state)
        final_state = resolution.state
        lines.extend(
            [
                "",
                f"선택 실행: {choice.label}",
                format_choice_resolution(resolution),
                "",
                f"턴: {resolution.state.turn}",
                format_local_status(resolution.state.player),
            ]
        )
        achievement_text = format_unlocked_achievements(unlock_result.unlocked)
        if achievement_text:
            lines.extend(["", achievement_text])
        ending = evaluate_ending(resolution.state)
        if ending is not None:
            lines.extend(["", format_ending_summary(ending)])
    return "\n".join(lines), final_state


def render_scripted_game_smoke(state: GameState, action_arguments: list[str]) -> str:
    output, _ = render_scripted_game_smoke_result(state, action_arguments)
    return output


def render_scripted_game_smoke_result(
    state: GameState,
    action_arguments: list[str],
) -> tuple[str, GameState]:
    turn = build_game_turn(state)
    lines = ["escape from the office", "", _format_game_turn(turn)]
    for action_argument in action_arguments:
        result = resolve_turn_action_result(turn, action_argument)
        lines.extend(["", _format_turn_action_result(result), "", _format_game_turn(result.turn)])
        turn = result.turn
        if turn.ending is not None:
            break
    return "\n".join(lines), turn.state


def render_loaded_game_smoke(state: GameState, action_arguments: list[str]) -> tuple[str, GameState]:
    if action_arguments:
        return render_scripted_game_smoke_result(state, action_arguments)
    turn = build_game_turn(state)
    return "\n".join(["escape from the office", "", _format_game_turn(turn)]), turn.state


def _format_game_turn(turn: GameTurn) -> str:
    location_info = DEFAULT_LOCATIONS.get(turn.state.location_id)
    location = location_info.name if location_info else turn.state.location_id
    lines = [
        f"== 턴 {turn.state.turn} ==",
        f"위치: {location}",
        f"재난: {turn.state.disaster_type}",
        format_local_status(turn.state.player),
    ]
    achievement_summary = format_achievement_summary(turn.state)
    if achievement_summary:
        lines.extend(["", achievement_summary])
    if turn.ending is not None:
        lines.extend(["", format_ending_summary(turn.ending)])
    elif turn.encounter is not None:
        lines.extend(["", format_encounter_turn(turn.encounter, turn.state)])
        _append_action_group(lines, turn, kind="item", title="소지품 사용:")
    elif turn.available_actions:
        _append_action_group(lines, turn, kind="move", title="이동:")
        _append_action_group(lines, turn, kind="item", title="소지품 사용:")
    else:
        lines.extend(["", "가능한 행동 없음"])
    return "\n".join(lines)


def _append_action_group(
    lines: list[str],
    turn: GameTurn,
    *,
    kind: str,
    title: str,
) -> None:
    actions = [action for action in turn.available_actions if action.kind == kind]
    if not actions:
        return
    lines.extend(["", title])
    for action in actions:
        lines.append(f"- {action.id} {action.label}")


def _format_turn_action_result(result: TurnActionResult) -> str:
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
            lines.extend(_format_item_resource_deltas(result.item_use_result))
    else:
        lines = [f"행동 실행: {result.action.label}"]

    achievement_text = format_unlocked_achievements(result.unlocked_achievements)
    if achievement_text:
        lines.extend(["", achievement_text])
    return "\n".join(lines)


def _format_item_resource_deltas(result: ItemUseResult) -> list[str]:
    labels = {
        "health": "체력",
        "sanity": "정신력",
        "battery": "배터리",
        "hunger": "허기",
        "thirst": "갈증",
    }
    before = result.before_state.player
    after = result.state.player
    lines: list[str] = []
    for resource_name, label in labels.items():
        before_value = getattr(before, resource_name)
        after_value = getattr(after, resource_name)
        if before_value != after_value:
            lines.append(f"{label}: {before_value} -> {after_value}")
    return lines


def _choice_from_argument(choices: tuple[Choice, ...], argument: str) -> Choice:
    if argument.isdecimal():
        index = int(argument) - 1
        if 0 <= index < len(choices):
            return choices[index]
    raise ValueError(f"선택지를 찾을 수 없다: {argument}")


_RESOURCE_OVERRIDE_NAMES = {"health", "sanity", "battery", "hunger", "thirst"}


def _new_state_from_args(args: argparse.Namespace) -> GameState:
    state = replace(
        GameState.new(seed=args.seed, location_id=args.location),
        flags=list(args.flag),
    )
    return _apply_resource_overrides(state, args.resource)


def _apply_resource_overrides(
    state: GameState,
    resource_arguments: list[str],
) -> GameState:
    if not resource_arguments:
        return state
    updates: dict[str, int] = {}
    for argument in resource_arguments:
        if "=" not in argument:
            raise ValueError(f"자원 설정은 name=value 형식이어야 한다: {argument}")
        resource_name, value_text = argument.split("=", 1)
        resource_name = resource_name.strip()
        if resource_name not in _RESOURCE_OVERRIDE_NAMES:
            raise ValueError(f"알 수 없는 자원: {resource_name}")
        try:
            updates[resource_name] = int(value_text)
        except ValueError:
            raise ValueError(f"자원 값은 정수여야 한다: {argument}") from None
    return state.with_player(replace(state.player, **updates))


def _print_output_and_maybe_save(
    output: str,
    state: GameState,
    save_path: str | None,
) -> None:
    print(output)
    if save_path:
        save_game_state(state, save_path)
        print(f"\n저장: {save_path}")


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.version:
        print(f"tui-adv {__version__}")
        return 0

    if args.location not in DEFAULT_LOCATIONS:
        parser.error(f"알 수 없는 위치: {args.location}")
    if args.new and args.load:
        parser.error("--new와 --load은 함께 사용할 수 없다")
    if args.resource and args.load:
        parser.error("--resource는 --load와 함께 사용할 수 없다")
    if args.resource and not (args.new or args.tui_smoke):
        parser.error("--resource requires --new or --tui-smoke")
    if args.delete_save_slot is not None and not args.tui_smoke:
        parser.error("--delete-save-slot requires --tui-smoke")

    if args.tui_smoke:
        if args.load:
            try:
                loaded_state = load_game_state(args.load)
            except ValueError as exc:
                parser.error(str(exc))
            turn = build_game_turn(loaded_state)
        else:
            try:
                state = _new_state_from_args(args)
            except ValueError as exc:
                parser.error(str(exc))
            turn = build_game_turn(state)
        try:
            for action_argument in args.action:
                result = resolve_turn_action_result(turn, action_argument)
                turn = result.turn
                if turn.ending is not None:
                    break
        except ValueError as exc:
            parser.error(str(exc))
        save_slots = discover_save_slots(Path(args.save).parent) if args.save else None
        if args.delete_save_slot is not None:
            if save_slots is None:
                parser.error("--delete-save-slot requires --tui-smoke --save")
            try:
                deleted_path = delete_tui_save_slot(save_slots, args.delete_save_slot)
            except ValueError as exc:
                parser.error(str(exc))
            turn = build_game_turn(
                replace(
                    turn.state,
                    log=[*turn.state.log, f"저장 슬롯 삭제: {deleted_path.name}"],
                )
            )
            save_slots = discover_save_slots(deleted_path.parent)
        _print_output_and_maybe_save(
            render_tui_layout_snapshot(
                turn,
                save_path=args.save,
                save_slots=save_slots,
            ),
            turn.state,
            args.save,
        )
        return 0

    if args.tui:
        initial_state = None
        if args.load:
            try:
                initial_state = load_game_state(args.load)
            except ValueError as exc:
                parser.error(str(exc))
        try:
            run_textual_tui(
                seed=args.seed,
                location_id=args.location,
                flags=tuple(args.flag),
                initial_state=initial_state,
                save_path=args.save,
            )
        except RuntimeError as exc:
            parser.error(str(exc))
        return 0

    if args.load:
        if args.choice:
            parser.error("--choice requires --new")
        try:
            state = load_game_state(args.load)
        except ValueError as exc:
            parser.error(str(exc))
        try:
            output, final_state = render_loaded_game_smoke(state, args.action)
        except ValueError as exc:
            parser.error(str(exc))
        _print_output_and_maybe_save(output, final_state, args.save)
        return 0

    if args.new:
        if args.choice and args.action:
            parser.error("--choice와 --action은 함께 사용할 수 없다")
        try:
            state = _new_state_from_args(args)
            if args.action:
                output, final_state = render_scripted_game_smoke_result(state, args.action)
            else:
                output, final_state = render_new_game_smoke_result(
                    state,
                    choice_argument=args.choice,
                )
        except ValueError as exc:
            parser.error(str(exc))
        _print_output_and_maybe_save(output, final_state, args.save)
        return 0

    if args.choice:
        parser.error("--choice requires --new")
    if args.action:
        parser.error("--action requires --new")
    if args.save:
        parser.error("--save requires --new, --load, or --tui-smoke")

    parser.print_help()
    return 0
