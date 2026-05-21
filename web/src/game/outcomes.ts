import type { GameState, Outcome, ResourceDelta, ResourceName } from './types';

const GOOD_RESOURCES: ResourceName[] = ['health', 'sanity', 'battery'];
const PRESSURE_RESOURCES: ResourceName[] = ['hunger', 'thirst'];
const RESOURCE_NAMES: ResourceName[] = ['health', 'sanity', 'battery', 'hunger', 'thirst'];

export function applyChoiceEffects(state: GameState, outcome: Outcome | undefined, cost: ResourceDelta = {}): GameState {
  let next = cloneState(state);
  const resourceDelta = outcomeResources(outcome);
  for (const resourceName of GOOD_RESOURCES) {
    resourceDelta[resourceName] = (resourceDelta[resourceName] ?? 0) - (cost[resourceName] ?? 0);
  }
  for (const resourceName of PRESSURE_RESOURCES) {
    resourceDelta[resourceName] = (resourceDelta[resourceName] ?? 0) + (cost[resourceName] ?? 0);
  }
  next = {
    ...next,
    player: {
      ...next.player,
      ...Object.fromEntries(
        RESOURCE_NAMES.map((resourceName) => [
          resourceName,
          clampResource(next.player[resourceName] + (resourceDelta[resourceName] ?? 0)),
        ]),
      ) as Pick<GameState['player'], ResourceName>,
    },
    danger: Math.max(0, next.danger + (outcome?.danger ?? 0)),
  };
  next.inventory = removeAll(next.inventory, outcome?.remove_items ?? []);
  next.inventory = appendUnique(next.inventory, outcome?.add_items ?? []);
  next.clues = appendUnique(next.clues, outcome?.add_clues ?? []);
  next.flags = removeAll(next.flags, outcome?.remove_flags ?? []);
  next.flags = appendUnique(next.flags, outcome?.add_flags ?? []);
  if (outcome?.destination_id) {
    next.locationId = outcome.destination_id;
  }
  if (outcome?.log) {
    next.log = [...next.log, outcome.log];
  }
  return next;
}

export function appendUnique(values: string[], additions: readonly string[]): string[] {
  const next = [...values];
  for (const value of additions) {
    if (!next.includes(value)) next.push(value);
  }
  return next;
}

function cloneState(state: GameState): GameState {
  return {
    ...state,
    player: { ...state.player, abilities: { ...state.player.abilities } },
    inventory: [...state.inventory],
    clues: [...state.clues],
    flags: [...state.flags],
    seenEncounters: [...state.seenEncounters],
    unlockedAchievements: [...state.unlockedAchievements],
    log: [...state.log],
  };
}

function outcomeResources(outcome: Outcome | undefined): ResourceDelta {
  const resources: ResourceDelta = { ...(outcome?.resources ?? {}) };
  if (!outcome) return resources;
  for (const resourceName of RESOURCE_NAMES) {
    const value = outcome[resourceName];
    if (value !== undefined) resources[resourceName] = value;
  }
  return resources;
}

function removeAll(values: string[], removals: readonly string[]): string[] {
  return values.filter((value) => !removals.includes(value));
}

function clampResource(value: number): number {
  return Math.max(0, Math.min(100, value));
}
