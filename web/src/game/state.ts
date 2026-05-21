import type { GameState, PlayerState, ResourceDelta, ResourceName } from './types';

export const RESOURCE_NAMES: ResourceName[] = ['health', 'sanity', 'battery', 'hunger', 'thirst'];

const DEFAULT_ABILITIES: Record<string, number> = {
  logic: 2,
  empathy: 2,
  volition: 2,
  composure: 2,
  interface: 2,
  physical: 2,
};

export function newGame(options: { seed: number; locationId?: string } = { seed: 0 }): GameState {
  return {
    seed: options.seed,
    turn: 0,
    locationId: options.locationId ?? 'dev_desk',
    disasterType: 'unknown_isolation',
    danger: 0,
    player: defaultPlayer(),
    inventory: [],
    clues: [],
    flags: [],
    seenEncounters: [],
    unlockedAchievements: [],
    log: [],
  };
}

export function defaultPlayer(): PlayerState {
  return {
    health: 100,
    sanity: 100,
    battery: 100,
    hunger: 0,
    thirst: 0,
    abilities: { ...DEFAULT_ABILITIES },
  };
}

export function cloneState(state: GameState): GameState {
  return {
    ...state,
    player: { ...state.player, abilities: { ...DEFAULT_ABILITIES, ...state.player.abilities } },
    inventory: [...state.inventory],
    clues: [...state.clues],
    flags: [...state.flags],
    seenEncounters: [...state.seenEncounters],
    unlockedAchievements: [...state.unlockedAchievements],
    log: [...state.log],
  };
}

export function applyResourceDelta(state: GameState, delta: ResourceDelta): GameState {
  const next = cloneState(state);
  return {
    ...next,
    player: {
      ...next.player,
      health: clampResource(next.player.health + (delta.health ?? 0)),
      sanity: clampResource(next.player.sanity + (delta.sanity ?? 0)),
      battery: clampResource(next.player.battery + (delta.battery ?? 0)),
      hunger: clampResource(next.player.hunger + (delta.hunger ?? 0)),
      thirst: clampResource(next.player.thirst + (delta.thirst ?? 0)),
      abilities: normalizeAbilities(next.player.abilities),
    },
  };
}

export function advanceTurn(state: GameState): GameState {
  let next = applyResourceDelta(state, { hunger: 1, thirst: 2 });
  const limitPenalty: ResourceDelta = {};
  if (next.player.hunger >= 100) limitPenalty.health = (limitPenalty.health ?? 0) - 2;
  if (next.player.thirst >= 100) {
    limitPenalty.health = (limitPenalty.health ?? 0) - 4;
    limitPenalty.sanity = (limitPenalty.sanity ?? 0) - 2;
  }
  if (Object.keys(limitPenalty).length) next = applyResourceDelta(next, limitPenalty);

  const flags = [...next.flags];
  const log = [...next.log];
  if (next.player.thirst >= 60 && !flags.includes('pressure_thirst_warning_seen')) {
    flags.push('pressure_thirst_warning_seen');
    log.push('목이 마르자 가장 가까운 정수기 물소리가 한 박자 늦게 따라온다.');
  }
  if (shouldDistortChoices(next.player) && !flags.includes('pressure_low_sanity_warning_seen')) {
    flags.push('pressure_low_sanity_warning_seen');
    log.push('선택지 문장이 화면 가장자리에서 흐려지기 시작했다.');
  }

  return {
    ...next,
    turn: next.turn + 1,
    flags,
    log,
  };
}

export function shouldDistortChoices(player: PlayerState): boolean {
  return player.sanity > 0 && player.sanity < 40;
}

function normalizeAbilities(abilities: Record<string, number>): Record<string, number> {
  return Object.fromEntries(
    Object.entries({ ...DEFAULT_ABILITIES, ...abilities }).map(([ability, value]) => [ability, clampAbility(value)]),
  );
}

function clampResource(value: number): number {
  return Math.max(0, Math.min(100, Math.trunc(value)));
}

function clampAbility(value: number): number {
  return Math.max(0, Math.min(6, Math.trunc(value)));
}
