import { itemsById } from './content';
import { removeFirst } from './outcomes';
import { advanceTurn, applyResourceDelta } from './state';
import type { GameState, ItemData } from './types';

export interface ItemUseResult {
  item: ItemData;
  state: GameState;
}

export function useItem(state: GameState, itemId: string): ItemUseResult {
  const item = itemsById.get(itemId);
  if (!item) throw new Error(`unknown item: ${itemId}`);
  if (!state.inventory.includes(itemId)) throw new Error(`inventory does not contain item: ${itemId}`);
  if (!item.usable || !item.use_effects) throw new Error(`item is not usable: ${itemId}`);

  const affected = applyResourceDelta(state, item.use_effects);
  const used = {
    ...affected,
    inventory: removeFirst(affected.inventory, itemId),
    log: [...affected.log, item.use_log || `${item.name}을 사용했다.`],
  };
  return { item, state: advanceTurn(used) };
}
