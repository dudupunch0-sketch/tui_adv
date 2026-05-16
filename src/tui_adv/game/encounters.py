from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Mapping

from tui_adv.game.state import GameState

_RESOURCE_NAMES = {"health", "sanity", "battery", "hunger", "thirst"}
_GOOD_RESOURCES = {"health", "sanity", "battery"}
_PRESSURE_RESOURCES = {"hunger", "thirst"}


def _resource_value(state: GameState, resource_name: str) -> int:
    if resource_name not in _RESOURCE_NAMES:
        raise ValueError(f"unknown resource: {resource_name}")
    return getattr(state.player, resource_name)


@dataclass(frozen=True, slots=True)
class Conditions:
    """Eligibility gates for encounters and choices."""

    locations: tuple[str, ...] = ()
    disaster_types: tuple[str, ...] = ()
    required_items: tuple[str, ...] = ()
    required_flags: tuple[str, ...] = ()
    forbidden_flags: tuple[str, ...] = ()
    min_resources: Mapping[str, int] = field(default_factory=dict)
    max_resources: Mapping[str, int] = field(default_factory=dict)

    def unavailable_reasons(self, state: GameState) -> tuple[str, ...]:
        reasons: list[str] = []
        if self.locations and state.location_id not in self.locations:
            reasons.append("location")
        if self.disaster_types and state.disaster_type not in self.disaster_types:
            reasons.append("disaster_type")
        for item_id in self.required_items:
            if item_id not in state.inventory:
                reasons.append(f"missing_item:{item_id}")
        for flag_id in self.required_flags:
            if flag_id not in state.flags:
                reasons.append(f"missing_flag:{flag_id}")
        for flag_id in self.forbidden_flags:
            if flag_id in state.flags:
                reasons.append(f"forbidden_flag:{flag_id}")
        for resource_name, minimum in self.min_resources.items():
            if _resource_value(state, resource_name) < minimum:
                reasons.append(f"{resource_name}<{minimum}")
        for resource_name, maximum in self.max_resources.items():
            if _resource_value(state, resource_name) > maximum:
                reasons.append(f"{resource_name}>{maximum}")
        return tuple(reasons)

    def is_satisfied_by(self, state: GameState) -> bool:
        return not self.unavailable_reasons(state)


@dataclass(frozen=True, slots=True)
class Outcome:
    """Effects applied after a choice is selected."""

    health: int = 0
    sanity: int = 0
    battery: int = 0
    hunger: int = 0
    thirst: int = 0
    add_items: tuple[str, ...] = ()
    remove_items: tuple[str, ...] = ()
    add_clues: tuple[str, ...] = ()
    add_flags: tuple[str, ...] = ()
    remove_flags: tuple[str, ...] = ()
    destination_id: str | None = None
    danger: int = 0
    log: str = ""


@dataclass(frozen=True, slots=True)
class Choice:
    """A selectable action within an encounter."""

    id: str
    label: str
    outcome: Outcome
    conditions: Conditions = field(default_factory=Conditions)
    cost: Mapping[str, int] = field(default_factory=dict)

    def unavailable_reasons(self, state: GameState) -> tuple[str, ...]:
        reasons = list(self.conditions.unavailable_reasons(state))
        for resource_name, amount in self.cost.items():
            current_value = _resource_value(state, resource_name)
            if resource_name in _GOOD_RESOURCES and current_value < amount:
                reasons.append(f"{resource_name}<{amount}")
            if resource_name in _PRESSURE_RESOURCES and current_value + amount > 100:
                reasons.append(f"{resource_name}+{amount}>100")
        return tuple(reasons)

    def is_available(self, state: GameState) -> bool:
        return not self.unavailable_reasons(state)

    def apply(self, state: GameState, *, encounter_id: str) -> GameState:
        player = state.player.apply_delta(
            health=self.outcome.health - self.cost.get("health", 0),
            sanity=self.outcome.sanity - self.cost.get("sanity", 0),
            battery=self.outcome.battery - self.cost.get("battery", 0),
            hunger=self.outcome.hunger + self.cost.get("hunger", 0),
            thirst=self.outcome.thirst + self.cost.get("thirst", 0),
        )
        inventory = _remove_all(state.inventory, self.outcome.remove_items)
        inventory = _append_unique(inventory, self.outcome.add_items)
        flags = _remove_all(state.flags, self.outcome.remove_flags)
        flags = _append_unique(flags, self.outcome.add_flags)
        clues = _append_unique(state.clues, self.outcome.add_clues)
        seen_encounters = _append_unique(state.seen_encounters, (encounter_id,))
        log = [*state.log]
        if self.outcome.log:
            log.append(self.outcome.log)
        return replace(
            state,
            location_id=self.outcome.destination_id or state.location_id,
            danger=max(0, state.danger + self.outcome.danger),
            player=player,
            inventory=inventory,
            clues=clues,
            flags=flags,
            seen_encounters=seen_encounters,
            log=log,
        )


@dataclass(frozen=True, slots=True)
class Encounter:
    """An authored office event with gated choices."""

    id: str
    title: str
    body: str
    choices: tuple[Choice, ...]
    conditions: Conditions = field(default_factory=Conditions)
    repeatable: bool = False
    weight: int = 1

    def is_eligible(self, state: GameState) -> bool:
        if not self.repeatable and self.id in state.seen_encounters:
            return False
        return self.conditions.is_satisfied_by(state)

    def available_choices(self, state: GameState) -> tuple[Choice, ...]:
        return tuple(choice for choice in self.choices if choice.is_available(state))

    def resolve_choice(self, choice_id: str, state: GameState) -> GameState:
        if not self.is_eligible(state):
            raise ValueError(f"encounter {self.id} is not eligible")
        choice = self._choice_by_id(choice_id)
        reasons = choice.unavailable_reasons(state)
        if reasons:
            reason_text = ", ".join(reasons)
            raise ValueError(f"choice {choice_id} is not available: {reason_text}")
        return choice.apply(state, encounter_id=self.id).advance_turn()

    def _choice_by_id(self, choice_id: str) -> Choice:
        for choice in self.choices:
            if choice.id == choice_id:
                return choice
        raise ValueError(f"unknown choice {choice_id} for encounter {self.id}")


EncounterMap = Mapping[str, Encounter]


def _append_unique(existing: list[str], values: tuple[str, ...]) -> list[str]:
    updated = list(existing)
    for value in values:
        if value not in updated:
            updated.append(value)
    return updated


def _remove_all(existing: list[str], values: tuple[str, ...]) -> list[str]:
    remove_set = set(values)
    return [value for value in existing if value not in remove_set]


DEFAULT_ENCOUNTERS: dict[str, Encounter] = {
    "ex_employee_messenger": Encounter(
        id="ex_employee_messenger",
        title="퇴사자의 메신저",
        body="퇴사한 전임자에게서 사내 메신저가 도착했다.",
        conditions=Conditions(locations=("dev_desk",)),
        choices=(
            Choice(
                id="check_message",
                label="메시지를 확인한다",
                cost={"battery": 3, "sanity": 2},
                outcome=Outcome(
                    add_clues=("ex_employee_contacted",),
                    log="퇴사자의 메시지를 확인했다.",
                ),
            ),
            Choice(
                id="ignore_phone",
                label="무시하고 휴대폰을 엎어둔다",
                outcome=Outcome(
                    sanity=2,
                    log="휴대폰을 엎어두자 알림음이 한 박자 늦게 멈췄다.",
                ),
            ),
            Choice(
                id="search_ex_employee",
                label="전임자의 이름을 사내망에서 검색한다",
                cost={"battery": 8},
                outcome=Outcome(
                    add_items=("ex_employee_memo",),
                    add_flags=("truth_route_started",),
                    log="사내망 캐시에 남은 전임자의 흔적을 찾았다.",
                ),
            ),
        ),
    ),
    "printer_prints_alone": Encounter(
        id="printer_prints_alone",
        title="복합기가 혼자 출력한다",
        body="꺼져 있던 복합기가 아직 하지 않은 선택을 출력한다.",
        conditions=Conditions(locations=("printer_area",)),
        choices=(
            Choice(
                id="read_printout",
                label="출력물을 읽는다",
                cost={"sanity": 3},
                outcome=Outcome(
                    add_clues=("future_choice_printout",),
                    log="출력물에는 아직 고르지 않은 선택지가 적혀 있었다.",
                ),
            ),
            Choice(
                id="take_printout",
                label="출력물을 챙긴다",
                outcome=Outcome(
                    add_items=("crumpled_printout",),
                    add_flags=("printer_secret_started",),
                    log="따뜻한 출력물을 접어 주머니에 넣었다.",
                ),
            ),
            Choice(
                id="check_toner",
                label="토너 카트리지를 확인한다",
                conditions=Conditions(min_resources={"sanity": 40}),
                outcome=Outcome(
                    add_clues=("reality_link_hint_1",),
                    add_flags=("reality_link_started",),
                    log="토너 카트리지 안쪽에서 이상한 표식을 봤다.",
                ),
            ),
        ),
    ),
}
