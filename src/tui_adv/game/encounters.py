from __future__ import annotations

from dataclasses import dataclass, field, replace
import random
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
    min_abilities: Mapping[str, int] = field(default_factory=dict)

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
        for ability_id, minimum in self.min_abilities.items():
            if state.player.ability(ability_id) < minimum:
                reasons.append(f"{ability_id}<{minimum}")
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
class CheckResult:
    """A resolved ability check for UI/result formatting."""

    ability: str
    difficulty: int
    rolls: tuple[int, int]
    ability_score: int
    total: int
    succeeded: bool
    outcome: Outcome


@dataclass(frozen=True, slots=True)
class AbilityCheck:
    """A two-dice ability check that branches into success or failure."""

    ability: str
    difficulty: int
    success: Outcome
    failure: Outcome

    def roll(self, state: GameState, rng: random.Random) -> CheckResult:
        first = rng.randint(1, 6)
        second = rng.randint(1, 6)
        ability_score = state.player.ability(self.ability)
        total = first + second + ability_score
        succeeded = total >= self.difficulty
        return CheckResult(
            ability=self.ability,
            difficulty=self.difficulty,
            rolls=(first, second),
            ability_score=ability_score,
            total=total,
            succeeded=succeeded,
            outcome=self.success if succeeded else self.failure,
        )

    def resolve(self, state: GameState, rng: random.Random) -> Outcome:
        return self.roll(state, rng).outcome


@dataclass(frozen=True, slots=True)
class ChoiceResolution:
    """A choice result paired with check metadata for rendering."""

    encounter_id: str
    choice_id: str
    before_state: GameState
    state: GameState
    check_result: CheckResult | None = None

    @property
    def new_logs(self) -> tuple[str, ...]:
        return tuple(self.state.log[len(self.before_state.log) :])


@dataclass(frozen=True, slots=True)
class Choice:
    """A selectable action within an encounter."""

    id: str
    label: str
    outcome: Outcome
    conditions: Conditions = field(default_factory=Conditions)
    cost: Mapping[str, int] = field(default_factory=dict)
    check: AbilityCheck | None = None

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

    def apply(
        self,
        state: GameState,
        *,
        encounter_id: str,
        rng: random.Random | None = None,
    ) -> GameState:
        return self.resolve(state, encounter_id=encounter_id, rng=rng).state

    def resolve(
        self,
        state: GameState,
        *,
        encounter_id: str,
        rng: random.Random | None = None,
    ) -> ChoiceResolution:
        rng = rng or _rng_for_state(state, encounter_id, self.id, "check")
        outcomes, check_result = self._resolved_outcomes(state, rng)
        player = state.player.apply_delta(
            health=sum(outcome.health for outcome in outcomes)
            - self.cost.get("health", 0),
            sanity=sum(outcome.sanity for outcome in outcomes)
            - self.cost.get("sanity", 0),
            battery=sum(outcome.battery for outcome in outcomes)
            - self.cost.get("battery", 0),
            hunger=sum(outcome.hunger for outcome in outcomes)
            + self.cost.get("hunger", 0),
            thirst=sum(outcome.thirst for outcome in outcomes)
            + self.cost.get("thirst", 0),
        )
        inventory = list(state.inventory)
        flags = list(state.flags)
        clues = list(state.clues)
        log = [*state.log]
        destination_id = state.location_id
        danger_delta = 0
        for outcome in outcomes:
            inventory = _remove_all(inventory, outcome.remove_items)
            inventory = _append_unique(inventory, outcome.add_items)
            flags = _remove_all(flags, outcome.remove_flags)
            flags = _append_unique(flags, outcome.add_flags)
            clues = _append_unique(clues, outcome.add_clues)
            if outcome.destination_id:
                destination_id = outcome.destination_id
            danger_delta += outcome.danger
            if outcome.log:
                log.append(outcome.log)
        seen_encounters = _append_unique(state.seen_encounters, (encounter_id,))
        next_state = replace(
            state,
            location_id=destination_id,
            danger=max(0, state.danger + danger_delta),
            player=player,
            inventory=inventory,
            clues=clues,
            flags=flags,
            seen_encounters=seen_encounters,
            log=log,
        )
        return ChoiceResolution(
            encounter_id=encounter_id,
            choice_id=self.id,
            before_state=state,
            state=next_state,
            check_result=check_result,
        )

    def _resolved_outcomes(
        self,
        state: GameState,
        rng: random.Random,
    ) -> tuple[tuple[Outcome, ...], CheckResult | None]:
        if self.check is None:
            return (self.outcome,), None
        check_result = self.check.roll(state, rng)
        return (self.outcome, check_result.outcome), check_result


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

    def resolve_choice(
        self,
        choice_id: str,
        state: GameState,
        *,
        rng: random.Random | None = None,
    ) -> GameState:
        return self.resolve_choice_result(choice_id, state, rng=rng).state

    def resolve_choice_result(
        self,
        choice_id: str,
        state: GameState,
        *,
        rng: random.Random | None = None,
    ) -> ChoiceResolution:
        if not self.is_eligible(state):
            raise ValueError(f"encounter {self.id} is not eligible")
        choice = self._choice_by_id(choice_id)
        reasons = choice.unavailable_reasons(state)
        if reasons:
            reason_text = ", ".join(reasons)
            raise ValueError(f"choice {choice_id} is not available: {reason_text}")
        resolution = choice.resolve(state, encounter_id=self.id, rng=rng)
        return replace(resolution, state=resolution.state.advance_turn())

    def _choice_by_id(self, choice_id: str) -> Choice:
        for choice in self.choices:
            if choice.id == choice_id:
                return choice
        raise ValueError(f"unknown choice {choice_id} for encounter {self.id}")


EncounterMap = Mapping[str, Encounter]


def _rng_for_state(state: GameState, *parts: str) -> random.Random:
    seed_parts = ":".join((str(state.seed), str(state.turn), *parts))
    return random.Random(seed_parts)


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
            Choice(
                id="trace_packet_delay",
                label="[인터페이스] 알림 지연 시간을 역추적한다",
                conditions=Conditions(min_abilities={"interface": 4}),
                cost={"battery": 2},
                outcome=Outcome(log="알림 패킷을 조심스럽게 붙잡았다."),
                check=AbilityCheck(
                    ability="interface",
                    difficulty=10,
                    success=Outcome(
                        add_clues=("delayed_packet_route",),
                        add_flags=("network_truth_hint",),
                        log="지연 시간 사이에서 숨은 라우팅을 찾았다.",
                    ),
                    failure=Outcome(
                        sanity=-4,
                        danger=1,
                        log="패킷이 역으로 당신의 단말을 훑고 지나갔다.",
                    ),
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
    "pantry_coffee_machine": Encounter(
        id="pantry_coffee_machine",
        title="탕비실 커피머신",
        body="커피머신 화면에 '물을 보충하십시오'가 반복된다. 물통은 가득 차 있다.",
        conditions=Conditions(locations=("pantry",)),
        choices=(
            Choice(
                id="brew_coffee",
                label="커피를 뽑는다",
                outcome=Outcome(
                    sanity=4,
                    hunger=-3,
                    thirst=5,
                    log="커피는 아직 따뜻했고, 컵 바닥에는 작은 검은 점이 남았다.",
                ),
            ),
            Choice(
                id="inspect_water_tank",
                label="물통을 확인한다",
                outcome=Outcome(
                    sanity=-2,
                    add_clues=("full_water_tank_warning",),
                    log="물통은 가득 차 있는데 센서는 계속 빈 상태를 보고했다.",
                ),
            ),
            Choice(
                id="look_behind_machine",
                label="커피머신 뒤를 본다",
                conditions=Conditions(required_flags=("printer_secret_started",)),
                outcome=Outcome(
                    add_clues=("reality_link_hint_2",),
                    add_flags=("coffee_machine_back_panel",),
                    log="커피머신 뒤쪽 패널에 복합기 출력물과 같은 표식이 있었다.",
                ),
            ),
        ),
    ),
    "spatial_exit_puzzle": Encounter(
        id="spatial_exit_puzzle",
        title="비상계단 공간 왜곡",
        body="층수 표시는 4, 4, 4, 4를 반복한다. 한 칸만 진짜 계단처럼 숨을 쉰다.",
        conditions=Conditions(
            locations=("emergency_stairs",),
            required_flags=("escape_puzzle_ready",),
        ),
        choices=(
            Choice(
                id="solve_distorted_floor",
                label="반복되는 층수의 비밀을 풀고 문을 통과한다",
                outcome=Outcome(
                    add_flags=("escape_route_completed",),
                    log="층수 표시가 한 번 거꾸로 흐른 뒤 비상문이 열렸다.",
                ),
            ),
            Choice(
                id="walk_down_wrong_stairs",
                label="아래라고 믿고 계속 내려간다",
                outcome=Outcome(
                    sanity=-25,
                    add_flags=("spatial_exit_failed",),
                    log="내려갈수록 계단은 회사 안쪽으로 더 깊게 접혔다.",
                ),
            ),
        ),
    ),
}


def eligible_encounters(
    state: GameState,
    encounters: EncounterMap | None = None,
) -> tuple[Encounter, ...]:
    encounter_map = DEFAULT_ENCOUNTERS if encounters is None else encounters
    return tuple(
        encounter for encounter in encounter_map.values() if encounter.is_eligible(state)
    )


def select_encounter(
    state: GameState,
    encounters: EncounterMap | None = None,
    *,
    rng: random.Random | None = None,
) -> Encounter | None:
    candidates = tuple(
        encounter
        for encounter in eligible_encounters(state, encounters)
        if encounter.weight > 0 and encounter.available_choices(state)
    )
    total_weight = sum(encounter.weight for encounter in candidates)
    if total_weight <= 0:
        return None
    rng = rng or _rng_for_state(state, "encounter")
    picked = rng.randrange(total_weight)
    running_total = 0
    for encounter in candidates:
        running_total += encounter.weight
        if picked < running_total:
            return encounter
    return candidates[-1]
