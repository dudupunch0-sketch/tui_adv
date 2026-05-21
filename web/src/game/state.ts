import type { GameState, PlayerState } from './types';

const DEFAULT_ABILITIES: Record<string, number> = {
  logic: 3,
  empathy: 2,
  will: 3,
  composure: 3,
  interface: 4,
  body: 2,
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
