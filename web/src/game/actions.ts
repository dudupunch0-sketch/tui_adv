import { encounters, itemsById, locationsById } from './content';
import { conditionsSatisfied } from './conditions';
import { evaluateEnding } from './endings';
import { useItem } from './items';
import { appendUnique, applyChoiceEffects } from './outcomes';
import { advanceTurn, cloneState } from './state';
import { unlockAchievements } from './achievements';
import type { AbilityCheckData, ActionResult, ChoiceData, EncounterData, GameAction, GameState, GameTurn, LocationData, Outcome, ResourceName } from './types';

const GOOD_RESOURCES: ResourceName[] = ['health', 'sanity', 'battery'];

export function buildTurn(state: GameState): GameTurn {
  const location = locationsById.get(state.locationId);
  if (!location) throw new Error(`unknown location: ${state.locationId}`);
  const ending = evaluateEnding(state);
  const encounter = ending ? null : selectEncounter(state);
  const itemActions = ending ? [] : usableItemActions(state);
  const actions = ending ? [] : encounter ? [...choiceActions(encounter, state), ...itemActions] : [...moveActions(location), ...itemActions];
  return { state, location, encounter, ending, actions };
}

export function executeAction(state: GameState, actionId: string): ActionResult {
  const turn = buildTurn(state);
  const action = turn.actions.find((candidate) => candidate.id === actionId);
  if (!action) throw new Error(`unknown action: ${actionId}`);
  let nextState: GameState;
  if (action.kind === 'move') {
    if (!action.targetId) throw new Error(`move action has no target: ${action.id}`);
    const target = locationsById.get(action.targetId);
    if (!target) throw new Error(`unknown location: ${action.targetId}`);
    nextState = advanceTurn({
      ...cloneState(state),
      locationId: action.targetId,
      danger: Math.max(0, state.danger + (target.danger ?? 0)),
      log: [...state.log, `${target.name}로 이동했다.`],
    });
  } else if (action.kind === 'choice') {
    if (!turn.encounter) throw new Error('choice action requires an encounter');
    if (!action.targetId) throw new Error(`choice action has no target: ${action.id}`);
    const choice = availableChoices(turn.encounter, state).find((candidate) => candidate.id === action.targetId);
    if (!choice) throw new Error(`choice is not available: ${action.targetId}`);
    const outcomes = resolveChoiceOutcomes(choice, state);
    const resolved = applyChoiceEffects(state, outcomes, choice.cost);
    nextState = advanceTurn({
      ...resolved,
      seenEncounters: appendUnique(resolved.seenEncounters, [turn.encounter.id]),
    });
  } else {
    if (!action.targetId) throw new Error(`item action has no target: ${action.id}`);
    nextState = useItem(state, action.targetId).state;
  }
  const achievementResult = unlockAchievements(nextState);
  const nextTurn = buildTurn(achievementResult.state);
  return { action, state: achievementResult.state, turn: nextTurn, unlockedAchievements: achievementResult.unlocked };
}

export function selectEncounter(state: GameState): EncounterData | null {
  const candidates = encounters.filter(
    (encounter) =>
      (encounter.repeatable || !state.seenEncounters.includes(encounter.id)) &&
      (encounter.weight ?? 1) > 0 &&
      conditionsSatisfied(encounter.conditions, state) &&
      availableChoices(encounter, state).length > 0,
  );
  if (!candidates.length) return null;
  return pickWeightedEncounter(candidates, state);
}

export function availableChoices(encounter: EncounterData, state: GameState): ChoiceData[] {
  return encounter.choices.filter((choice) => conditionsSatisfied(choice.conditions, state) && canPayCost(choice, state));
}

function choiceActions(encounter: EncounterData, state: GameState): GameAction[] {
  return availableChoices(encounter, state).map((choice) => ({
    id: `choice:${choice.id}`,
    label: choice.label,
    kind: 'choice',
    targetId: choice.id,
  }));
}

function moveActions(location: LocationData): GameAction[] {
  return location.connections.map((targetId) => {
    const target = locationsById.get(targetId);
    return {
      id: `move:${targetId}`,
      label: target?.name ?? targetId,
      kind: 'move',
      targetId,
    };
  });
}

function usableItemActions(state: GameState): GameAction[] {
  const actions: GameAction[] = [];
  const seenItemIds = new Set<string>();
  for (const itemId of state.inventory) {
    if (seenItemIds.has(itemId)) continue;
    seenItemIds.add(itemId);
    const item = itemsById.get(itemId);
    if (!item?.usable || !item.use_effects) continue;
    actions.push({ id: `use:${item.id}`, label: item.name, kind: 'item', targetId: item.id });
  }
  return actions;
}

function canPayCost(choice: ChoiceData, state: GameState): boolean {
  for (const resourceName of GOOD_RESOURCES) {
    const amount = choice.cost?.[resourceName] ?? 0;
    if (amount > 0 && state.player[resourceName] < amount) return false;
  }
  return true;
}

function resolveChoiceOutcomes(choice: ChoiceData, state: GameState): Outcome[] {
  const outcomes: Outcome[] = [choice.outcome ?? {}];
  if (choice.check) outcomes.push(resolveAbilityCheck(choice.check, state));
  return outcomes;
}

function resolveAbilityCheck(check: AbilityCheckData, state: GameState): Outcome {
  const [first, second] = roll2d6(`${state.seed}:${state.turn}:${check.ability}:${check.difficulty}`);
  const total = first + second + (state.player.abilities[check.ability] ?? 0);
  return total >= check.difficulty ? check.success : check.failure;
}

function roll2d6(seed: string): [number, number] {
  const hash = hashString(seed);
  return [(hash % 6) + 1, (Math.floor(hash / 6) % 6) + 1];
}

function hashString(value: string): number {
  let hash = 2166136261;
  for (let index = 0; index < value.length; index += 1) {
    hash ^= value.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return hash >>> 0;
}

function pickWeightedEncounter(candidates: EncounterData[], state: GameState): EncounterData {
  const totalWeight = candidates.reduce((total, encounter) => total + (encounter.weight ?? 1), 0);
  const seed = `${state.seed}:${state.turn}:encounter:${state.locationId}:${candidates.map((encounter) => encounter.id).join(',')}`;
  let picked = hashString(seed) % totalWeight;
  for (const encounter of candidates) {
    picked -= encounter.weight ?? 1;
    if (picked < 0) return encounter;
  }
  return candidates[candidates.length - 1];
}
