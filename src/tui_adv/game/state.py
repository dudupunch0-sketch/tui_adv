from __future__ import annotations

from dataclasses import dataclass, field, replace


_RESOURCE_MIN = 0
_RESOURCE_MAX = 100


def _clamp_resource(value: int) -> int:
    return max(_RESOURCE_MIN, min(_RESOURCE_MAX, value))


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

    def __post_init__(self) -> None:
        for field_name in ("health", "sanity", "battery", "hunger", "thirst"):
            value = getattr(self, field_name)
            clamped = _clamp_resource(value)
            if value != clamped:
                object.__setattr__(self, field_name, clamped)

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
        )


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
    log: list[str] = field(default_factory=list)

    @classmethod
    def new(
        cls,
        *,
        seed: int,
        location_id: str = "desk",
        disaster_type: str = "unknown_isolation",
    ) -> GameState:
        return cls(
            seed=seed,
            turn=0,
            location_id=location_id,
            disaster_type=disaster_type,
            danger=0,
        )

    def advance_turn(self) -> GameState:
        """Apply the baseline turn pressure without mutating the current state."""

        return replace(
            self,
            turn=self.turn + 1,
            player=self.player.apply_delta(hunger=1, thirst=2),
            inventory=list(self.inventory),
            clues=list(self.clues),
            flags=list(self.flags),
            seen_encounters=list(self.seen_encounters),
            log=list(self.log),
        )
