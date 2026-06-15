import type { ScenePage } from './types';
import wuxiaJianghuBundle from '../data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json';
import yageunmongBundle from '../data/generated/storypack-preview/yageunmong_pack.content.bundle.json';

export const DEFAULT_STORYPACK_ID = 'wuxia_jianghu_pack';
export const DEFAULT_STORYPACK_LABEL = '이구학지 — 천기록';
export const DEFAULT_CONTENT_BUNDLE_JSON = JSON.stringify({
  ...wuxiaJianghuBundle,
  runtime: {
    ...wuxiaJianghuBundle.runtime,
    runtime_mode: 'storypack_main',
  },
});

export type StorypackPreviewId = 'wuxia_jianghu_pack' | 'yageunmong_pack';

export interface StorypackPreviewOption {
  id: StorypackPreviewId;
  label: string;
  description: string;
  contentBundleJson: string;
}

export const STORYPACK_PREVIEW_OPTIONS: StorypackPreviewOption[] = [
  {
    id: 'yageunmong_pack',
    label: '야근몽',
    description: '야근 중 책상에 엎드렸다가 회사 악몽에 갇힌다. 업무 완료가 아니라 꿈에서 깨어나는 것이 목표다.',
    contentBundleJson: JSON.stringify(yageunmongBundle),
  },
];

export function storypackPreviewById(id: string): StorypackPreviewOption | undefined {
  return STORYPACK_PREVIEW_OPTIONS.find((option) => option.id === id);
}

export function defaultStorypackLoadingPage(): ScenePage {
  return storypackLoadingPage({
    id: DEFAULT_STORYPACK_ID,
    label: DEFAULT_STORYPACK_LABEL,
    description: '출근복 그대로 강호에 떨어지는 이구학지 본편을 불러오는 중',
    chapterLabel: 'default storypack',
    warning: 'Rust/WASM GameCore default storypack을 불러오는 중',
    visualKind: 'storypack_main',
  });
}

export function storypackPreviewLoadingPage(preview: StorypackPreviewOption): ScenePage {
  return storypackLoadingPage({
    id: preview.id,
    label: preview.label,
    description: preview.description,
    chapterLabel: 'storypack preview',
    warning: 'Rust/WASM GameCore preview bundle을 불러오는 중',
    visualKind: 'storypack_preview',
  });
}

function storypackLoadingPage(options: {
  id: string;
  label: string;
  description: string;
  chapterLabel: string;
  warning: string;
  visualKind: string;
}): ScenePage {
  return {
    mode: 'movement',
    title: `${options.label} loading`,
    location: {
      id: 'storypack_loading',
      name: options.label,
      description: options.description,
    },
    chapter_label: options.chapterLabel,
    status_summary: {
      turn: 0,
      danger: 0,
      resources: [],
      warnings: [options.warning],
    },
    body_blocks: [{ kind: 'system', text: options.description, source_id: options.id }],
    dialogue_entries: [],
    visual: {
      id: `storypack:${options.id}`,
      kind: options.visualKind,
      alt: options.label,
      source_id: options.id,
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
