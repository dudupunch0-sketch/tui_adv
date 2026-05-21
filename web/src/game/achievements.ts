import { achievements } from './content';
import { conditionsSatisfied } from './conditions';
import { cloneState } from './state';
import type { AchievementData, GameState } from './types';

export interface AchievementUnlockResult {
  state: GameState;
  unlocked: AchievementData[];
}

export function unlockAchievements(state: GameState): AchievementUnlockResult {
  const unlockedIds = [...state.unlockedAchievements];
  const newlyUnlocked: AchievementData[] = [];
  for (const achievement of achievements) {
    if (unlockedIds.includes(achievement.id)) continue;
    if (!conditionsSatisfied(achievement.conditions, state)) continue;
    unlockedIds.push(achievement.id);
    newlyUnlocked.push(achievement);
  }
  if (!newlyUnlocked.length) return { state, unlocked: [] };
  return {
    state: {
      ...cloneState(state),
      unlockedAchievements: unlockedIds,
    },
    unlocked: newlyUnlocked,
  };
}

export function achievementById(id: string): AchievementData | undefined {
  return achievements.find((achievement) => achievement.id === id);
}
