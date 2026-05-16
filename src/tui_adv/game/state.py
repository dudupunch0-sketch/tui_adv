from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Mapping

from tui_adv.game.locations import DEFAULT_LOCATIONS, LocationMap


_RESOURCE_MIN = 0
_RESOURCE_MAX = 100
_ABILITY_MIN = 0
_ABILITY_MAX = 6
_DEFAULT_ABILITIES: dict[str, int] = {
    "logic": 2,
    "empathy": 2,
    "volition": 2,
    "composure": 2,
    "interface": 2,
    "physical": 2,
}


def _clamp_resource(value: int) -> int:
    return max(_RESOURCE_MIN, min(_RESOURCE_MAX, value))


def _clamp_ability(value: int) -> int:
    return max(_ABILITY_MIN, min(_ABILITY_MAX, value))


@dataclass(frozen=True, slots=True)
class PlayerState:
    """Player survival resources.

    Health, sanity, and battery are good when high.
    Hunger and thirst are pressure gauges, so they are dangerous when high.
    """

    health: int = 100
    sanity: int = 100
    battery: int = 100
    hunger: int = 0
    thirst: int = 0
    abilities: Mapping[str, int] = field(
        default_factory=lambda: dict(_DEFAULT_ABILITIES)
    )

    def __post_init__(self) -> None:
        for field_name in ("health", "sanity", "battery", "hunger", "thirst"):
            value = getattr(self, field_name)
            clamped = _clamp_resource(value)
            if value != clamped:
                object.__setattr__(self, field_name, clamped)
        normalized_abilities = dict(_DEFAULT_ABILITIES)
        normalized_abilities.update(self.abilities)
        normalized_abilities = {
            ability: _clamp_ability(score)
            for ability, score in normalized_abilities.items()
        }
        object.__setattr__(self, "abilities", normalized_abilities)

    def ability(self, ability_id: str) -> int:
        """Return a Disco Elysium-style discernment score for branching choices."""

        return self.abilities.get(ability_id, 0)

    def with_abilities(self, **abilities: int) -> PlayerState:
        updated = dict(self.abilities)
        updated.update(abilities)
        return replace(self, abilities=updated)

    @property
    def failure_reason(self) -> str | None:
        if self.health <= 0:
            return "health_depleted"
        if self.sanity <= 0:
            return "sanity_depleted"
        return None

    @property
    def should_distort_choices(self) -> bool:
        return 0 < self.sanity < 40

    @property
    def should_trigger_thirst_hallucination(self) -> bool:
        return self.thirst >= 60

    def can_spend_battery(self, amount: int) -> bool:
        return amount <= 0 or self.battery >= amount

    def apply_delta(
        self,
        *,
        health: int = 0,
        sanity: int = 0,
        battery: int = 0,
        hunger: int = 0,
        thirst: int = 0,
    ) -> PlayerState:
        """Return a new state with bounded resource deltas applied."""

        return type(self)(
            health=self.health + health,
            sanity=self.sanity + sanity,
            battery=self.battery + battery,
            hunger=self.hunger + hunger,
            thirst=self.thirst + thirst,
            abilities=self.abilities,
        )

    def apply_turn_pressure(self) -> PlayerState:
        """Apply baseline hunger/thirst decay and limit penalties."""

        next_state = self.apply_delta(hunger=1, thirst=2)
        health_penalty = 0
        sanity_penalty = 0
        if next_state.hunger >= 100:
            health_penalty -= 2
        if next_state.thirst >= 100:
            health_penalty -= 4
            sanity_penalty -= 2
        return next_state.apply_delta(health=health_penalty, sanity=sanity_penalty)


@dataclass(frozen=True, slots=True)
class GameState:
    """Serializable game state for the first vertical slice."""

    seed: int
    turn: int
    location_id: str
    disaster_type: str
    danger: int
    player: PlayerState = field(default_factory=PlayerState)
    inventory: list[str] = field(default_factory=list)
    clues: list[str] = field(default_factory=list)
    flags: list[str] = field(default_factory=list)
    seen_encounters: list[str] = field(default_factory=list)
    unlocked_achievements: list[str] = field(default_factory=list)
    log: list[str] = field(default_factory=list)

    @classmethod
    def new(
        cls,
        *,
        seed: int,
        location_id: str = "dev_desk",
        disaster_type: str = "unknown_isolation",
    ) -> GameState:
        return cls(
            seed=seed,
            turn=0,
            location_id=location_id,
            disaster_type=disaster_type,
            danger=0,
        )

    @property
    def failure_reason(self) -> str | None:
        return self.player.failure_reason

    def with_player(self, player: PlayerState) -> GameState:
        return replace(
            self,
            player=player,
            inventory=list(self.inventory),
            clues=list(self.clues),
            flags=list(self.flags),
            seen_encounters=list(self.seen_encounters),
            unlocked_achievements=list(self.unlocked_achievements),
            log=list(self.log),
        )

    def with_seen_encounter(self, encounter_id: str) -> GameState:
        seen_encounters = list(self.seen_encounters)
        if encounter_id not in seen_encounters:
            seen_encounters.append(encounter_id)
        return replace(
            self,
            inventory=list(self.inventory),
            clues=list(self.clues),
            flags=list(self.flags),
            seen_encounters=seen_encounters,
            unlocked_achievements=list(self.unlocked_achievements),
            log=list(self.log),
        )

    def available_move_ids(
        self,
        locations: LocationMap = DEFAULT_LOCATIONS,
    ) -> tuple[str, ...]:
        current_location = locations[self.location_id]
        return current_location.connections

    def move_to(
        self,
        destination_id: str,
        locations: LocationMap = DEFAULT_LOCATIONS,
    ) -> GameState:
        current_location = locations[self.location_id]
        if destination_id not in current_location.connections:
            raise ValueError(f"cannot move from {self.location_id} to {destination_id}")
        destination = locations[destination_id]
        moved = replace(
            self,
            location_id=destination_id,
            inventory=list(self.inventory),
            clues=list(self.clues),
            flags=list(self.flags),
            seen_encounters=list(self.seen_encounters),
            unlocked_achievements=list(self.unlocked_achievements),
            log=[*self.log, f"{destination.name}로 이동했다."],
        )
        return moved.advance_turn()

    def advance_turn(self) -> GameState:
        """Apply the baseline turn pressure without mutating the current state."""

        return replace(
            self,
            turn=self.turn + 1,
            player=self.player.apply_turn_pressure(),
            inventory=list(self.inventory),
            clues=list(self.clues),
            flags=list(self.flags),
            seen_encounters=list(self.seen_encounters),
            unlocked_achievements=list(self.unlocked_achievements),
            log=list(self.log),
        )
