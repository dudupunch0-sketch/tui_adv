import { cloneState, applyResourceDelta, RESOURCE_NAMES } from './state';
import type { GameState, Outcome, ResourceDelta, ResourceName } from './types';

const GOOD_RESOURCES: ResourceName[] = ['health', 'sanity', 'battery'];
const PRESSURE_RESOURCES: ResourceName[] = ['hunger', 'thirst'];

export function applyChoiceEffects(
  state: GameState,
  outcomes: Outcome | Outcome[] | undefined,
  cost: ResourceDelta = {},
): GameState {
  const normalizedOutcomes = Array.isArray(outcomes) ? outcomes : [outcomes ?? {}];
  const resourceDelta: ResourceDelta = {};
  for (const resourceName of GOOD_RESOURCES) {
    resourceDelta[resourceName] = -(cost[resourceName] ?? 0);
  }
  for (const resourceName of PRESSURE_RESOURCES) {
    resourceDelta[resourceName] = cost[resourceName] ?? 0;
  }
  for (const outcome of normalizedOutcomes) {
    const outcomeDelta = outcomeResources(outcome);
    for (const resourceName of RESOURCE_NAMES) {
      resourceDelta[resourceName] = (resourceDelta[resourceName] ?? 0) + (outcomeDelta[resourceName] ?? 0);
    }
  }

  let next = applyResourceDelta(state, resourceDelta);
  next = {
    ...next,
    danger: Math.max(0, next.danger + normalizedOutcomes.reduce((total, outcome) => total + (outcome.danger ?? 0), 0)),
  };
  for (const outcome of normalizedOutcomes) {
    next.inventory = removeAll(next.inventory, outcome.remove_items ?? []);
    next.inventory = appendUnique(next.inventory, outcome.add_items ?? []);
    next.clues = appendUnique(next.clues, outcome.add_clues ?? []);
    next.flags = removeAll(next.flags, outcome.remove_flags ?? []);
    next.flags = appendUnique(next.flags, outcome.add_flags ?? []);
    if (outcome.destination_id) {
      next.locationId = outcome.destination_id;
    }
    if (outcome.log) {
      next.log = [...next.log, outcome.log];
    }
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

export function removeFirst(values: string[], removal: string): string[] {
  const next = [...values];
  const index = next.indexOf(removal);
  if (index >= 0) next.splice(index, 1);
  return next;
}

function outcomeResources(outcome: Outcome): ResourceDelta {
  const resources: ResourceDelta = { ...(outcome.resources ?? {}) };
  for (const resourceName of RESOURCE_NAMES) {
    const value = outcome[resourceName];
    if (value !== undefined) resources[resourceName] = value;
  }
  return resources;
}

function removeAll(values: string[], removals: readonly string[]): string[] {
  return values.filter((value) => !removals.includes(value));
}

export function clonedState(state: GameState): GameState {
  return cloneState(state);
}
