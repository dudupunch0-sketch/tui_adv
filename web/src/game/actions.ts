import { encounters, locationsById } from './content';
import { conditionsSatisfied } from './conditions';
import { evaluateEnding } from './endings';
import { appendUnique, applyChoiceEffects } from './outcomes';
import type { ActionResult, ChoiceData, EncounterData, GameAction, GameState, GameTurn } from './types';

export function buildTurn(state: GameState): GameTurn {
  const location = locationsById.get(state.locationId);
  if (!location) throw new Error(`unknown location: ${state.locationId}`);
  const ending = evaluateEnding(state);
  const encounter = ending ? null : selectEncounter(state);
  const actions = ending ? [] : encounter ? choiceActions(encounter, state) : moveActions(location);
  return { state, location, encounter, ending, actions };
}

export function executeAction(state: GameState, actionId: string): ActionResult {
  const turn = buildTurn(state);
  const action = turn.actions.find((candidate) => candidate.id === actionId);
  if (!action) throw new Error(`unknown action: ${actionId}`);
  let nextState: GameState;
  if (action.kind === 'move') {
    if (!action.targetId) throw new Error(`move action has no target: ${action.id}`);
    nextState = {
      ...cloneState(state),
      turn: state.turn + 1,
      locationId: action.targetId,
      log: [...state.log, `이동: ${action.label}`],
    };
  } else {
    if (!turn.encounter) throw new Error('choice action requires an encounter');
    const choiceId = action.id.replace(/^choice:/, '');
    const choice = availableChoices(turn.encounter, state).find((candidate) => candidate.id === choiceId);
    if (!choice) throw new Error(`choice is not available: ${choiceId}`);
    nextState = applyChoiceEffects(state, choice.outcome, choice.cost);
    nextState = {
      ...nextState,
      turn: state.turn + 1,
      seenEncounters: appendUnique(nextState.seenEncounters, [turn.encounter.id]),
    };
  }
  const nextTurn = buildTurn(nextState);
  return { action, state: nextState, turn: nextTurn };
}

export function selectEncounter(state: GameState): EncounterData | null {
  return (
    encounters.find(
      (encounter) =>
        conditionsSatisfied(encounter.conditions, state) &&
        (encounter.repeatable || !state.seenEncounters.includes(encounter.id)),
    ) ?? null
  );
}

export function availableChoices(encounter: EncounterData, state: GameState): ChoiceData[] {
  return encounter.choices.filter((choice) => conditionsSatisfied(choice.conditions, state));
}

function choiceActions(encounter: EncounterData, state: GameState): GameAction[] {
  return availableChoices(encounter, state).map((choice) => ({
    id: `choice:${choice.id}`,
    label: choice.label,
    kind: 'choice',
  }));
}

function moveActions(location: { connections: string[] }): GameAction[] {
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
