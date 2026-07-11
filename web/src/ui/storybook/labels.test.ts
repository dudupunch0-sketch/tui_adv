import { describe, expect, it } from 'vitest';
import { hasAchievementLabel, hasInventoryItemLabel } from './labels';

import wuxiaBundle from '../../data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json';
import defaultBundle from '../../data/generated/content.bundle.json';

describe('Achievement and Item Labels Translation Coverage', () => {
  it('covers all achievement and item IDs in wuxia_jianghu_pack bundle', () => {
    const achievementIds = (wuxiaBundle.content?.achievements ?? []).map((a: any) => a.id);
    const itemIds = (wuxiaBundle.content?.items ?? []).map((i: any) => i.id);

    for (const id of achievementIds) {
      expect(hasAchievementLabel(id)).toBe(true);
    }
    for (const id of itemIds) {
      expect(hasInventoryItemLabel(id)).toBe(true);
    }
  });

  it('covers all achievement and item IDs in default content bundle', () => {
    const achievementIds = (defaultBundle.content?.achievements ?? []).map((a: any) => a.id);
    const itemIds = (defaultBundle.content?.items ?? []).map((i: any) => i.id);

    for (const id of achievementIds) {
      expect(hasAchievementLabel(id)).toBe(true);
    }
    for (const id of itemIds) {
      expect(hasInventoryItemLabel(id)).toBe(true);
    }
  });
});
