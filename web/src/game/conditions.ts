import type { Conditions, GameState, ResourceName } from './types';

const RESOURCE_NAMES: ResourceName[] = ['health', 'sanity', 'battery', 'hunger', 'thirst'];

export function conditionsSatisfied(conditions: Conditions | undefined, state: GameState): boolean {
  return unavailableReasons(conditions, state).length === 0;
}

export function unavailableReasons(conditions: Conditions | undefined, state: GameState): string[] {
  if (!conditions) return [];
  const reasons: string[] = [];
  if (conditions.locations?.length && !conditions.locations.includes(state.locationId)) {
    reasons.push('location');
  }
  if (conditions.disaster_types?.length && !conditions.disaster_types.includes(state.disasterType)) {
    reasons.push('disaster_type');
  }
  for (const itemId of conditions.required_items ?? []) {
    if (!state.inventory.includes(itemId)) reasons.push(`missing_item:${itemId}`);
  }
  for (const clueId of conditions.required_clues ?? []) {
    if (!state.clues.includes(clueId)) reasons.push(`missing_clue:${clueId}`);
  }
  for (const flagId of conditions.required_flags ?? []) {
    if (!state.flags.includes(flagId)) reasons.push(`missing_flag:${flagId}`);
  }
  for (const flagId of conditions.forbidden_flags ?? []) {
    if (state.flags.includes(flagId)) reasons.push(`forbidden_flag:${flagId}`);
  }
  for (const resourceName of RESOURCE_NAMES) {
    const minimum = conditions.min_resources?.[resourceName];
    if (minimum !== undefined && state.player[resourceName] < minimum) {
      reasons.push(`${resourceName}<${minimum}`);
    }
    const maximum = conditions.max_resources?.[resourceName];
    if (maximum !== undefined && state.player[resourceName] > maximum) {
      reasons.push(`${resourceName}>${maximum}`);
    }
  }
  for (const [abilityId, minimum] of Object.entries(conditions.min_abilities ?? {})) {
    if ((state.player.abilities[abilityId] ?? 0) < minimum) {
      reasons.push(`${abilityId}<${minimum}`);
    }
  }
  return reasons;
}
