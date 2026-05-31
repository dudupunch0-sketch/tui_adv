import type { ScenePage } from './types';
import contentBundle from '../data/generated/content.bundle.json';
import wuxiaJianghuPreviewBundle from '../data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json';

export const DEFAULT_CONTENT_BUNDLE_JSON = JSON.stringify(contentBundle);

export type StorypackPreviewId = 'wuxia_jianghu_pack';

export interface StorypackPreviewOption {
  id: StorypackPreviewId;
  label: string;
  description: string;
  contentBundleJson: string;
}

export const STORYPACK_PREVIEW_OPTIONS: StorypackPreviewOption[] = [
  {
    id: 'wuxia_jianghu_pack',
    label: '이구학지 — 천기록',
    description: '출근복 그대로 강호에 떨어지는 무협 storypack preview',
    contentBundleJson: JSON.stringify(wuxiaJianghuPreviewBundle),
  },
];

export function storypackPreviewById(id: string): StorypackPreviewOption | undefined {
  return STORYPACK_PREVIEW_OPTIONS.find((option) => option.id === id);
}

export function storypackPreviewLoadingPage(preview: StorypackPreviewOption): ScenePage {
  return {
    mode: 'movement',
    title: `${preview.label} preview loading`,
    location: {
      id: 'storypack_preview_loading',
      name: preview.label,
      description: preview.description,
    },
    chapter_label: 'storypack preview',
    status_summary: {
      turn: 0,
      danger: 0,
      resources: [],
      warnings: ['Rust/WASM GameCore preview bundle을 불러오는 중'],
    },
    body_blocks: [{ kind: 'system', text: preview.description, source_id: preview.id }],
    dialogue_entries: [],
    visual: {
      id: `storypack-preview:${preview.id}`,
      kind: 'storypack_preview',
      alt: preview.label,
      source_id: preview.id,
    },
    actions: [],
    blocked_actions: [],
    history_entries: [],
    inventory_summary: { items: [], overflow_count: 0 },
    achievement_summary: { unlocked: [], newly_unlocked: [] },
    pressure_cues: [],
    effect_cues: [],
  };
}
