import { endings, publicSecretsById } from './content';
import { conditionsSatisfied } from './conditions';
import type { GameState, ResolvedEnding } from './types';

export function evaluateEnding(state: GameState): ResolvedEnding | null {
  if (state.player.health <= 0) {
    return {
      id: 'game_over_health_depleted',
      name: '게임오버: 신체 반응 없음',
      kind: 'failure',
      priority: 1000,
      text: '몸이 먼저 퇴근을 포기했다.',
    };
  }
  if (state.player.sanity <= 0) {
    return {
      id: 'game_over_sanity_depleted',
      name: '게임오버: 집중도 붕괴',
      kind: 'failure',
      priority: 1000,
      text: 'LOCAL STATUS가 더 이상 현실을 붙잡지 못했다.',
    };
  }
  const matched = endings
    .filter((ending) => conditionsSatisfied(ending.conditions, state))
    .sort((left, right) => right.priority - left.priority)[0];
  if (!matched) return null;
  return {
    ...matched,
    publicSecret: matched.local_hint_id ? publicSecretsById.get(matched.local_hint_id) : undefined,
  };
}
