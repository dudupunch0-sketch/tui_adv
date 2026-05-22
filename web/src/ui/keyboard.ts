export const NEW_GAME_ACTION_ID = 'system:new-game';

export interface ActionListSource {
  actions: Array<{ id: string }>;
}

export function actionIdForKey(source: ActionListSource, key: string): string | null {
  if (key.toLowerCase() === 'n') return NEW_GAME_ACTION_ID;
  if (!/^\d$/.test(key)) return null;
  const actionIndex = Number(key) - 1;
  return source.actions[actionIndex]?.id ?? null;
}
