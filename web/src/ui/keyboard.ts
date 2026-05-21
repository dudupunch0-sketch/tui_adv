import type { GameTurn } from '../game/types';

export const NEW_GAME_ACTION_ID = 'system:new-game';

export function actionIdForKey(turn: GameTurn, key: string): string | null {
  if (key.toLowerCase() === 'n') return NEW_GAME_ACTION_ID;
  if (!/^\d$/.test(key)) return null;
  const actionIndex = Number(key) - 1;
  return turn.actions[actionIndex]?.id ?? null;
}
