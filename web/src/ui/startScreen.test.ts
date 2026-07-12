import { describe, expect, it } from 'vitest';

import {
  LAST_RUN_SUMMARY_KEY,
  LEGACY_SAVE_KEY,
  OFFICE_RUST_SAVE_KEY,
  RUST_SAVE_KEY,
  clearPlayerSaves,
  readPlayerSaveSummary,
  renderStartScreen,
  writeRunSummary,
  type PlayerRunSummary,
} from './startScreen';
import { DEFAULT_PLAYER_SETTINGS } from './settings/playerSettings';

class MemoryStorage {
  private readonly data = new Map<string, string>();

  getItem(key: string): string | null {
    return this.data.get(key) ?? null;
  }

  setItem(key: string, value: string): void {
    this.data.set(key, value);
  }

  removeItem(key: string): void {
    this.data.delete(key);
  }
}

const summary: PlayerRunSummary = {
  schema_version: 1,
  seed: 777,
  turn: 4,
  location_id: 'printer_area',
  saved_at: '2026-05-26T05:00:00.000Z',
};

describe('player start/save UX', () => {
  it('renders a start screen with disabled continue when no save exists', () => {
    const html = renderStartScreen({ defaultSeed: 123, summary: null, warning: null, confirmReset: false });

    expect(html).toContain('data-player-screen="start"');
    expect(html).toContain('이구학지');
    expect(html).not.toContain('intro-main.png');
    expect(html).not.toContain('모험가 이야기');
    expect(html).not.toContain('LIFE IN ADVENTURE');
    expect(html).toContain('천기록을 연다');
    expect(html).toContain('data-player-action="continue" disabled');
    expect(html).toContain('data-player-action="new-game"');
    expect(html).toContain('name="seed"');
    expect(html).toContain('value="123"');
    expect(html).toContain('저장된 모험 없음');
  });

  it('renders renderer-local audio and motion controls without enabling autoplay', () => {
    const html = renderStartScreen({
      defaultSeed: 123,
      summary: null,
      warning: null,
      confirmReset: false,
      settings: DEFAULT_PLAYER_SETTINGS,
    });

    expect(html).toContain('data-settings-key="tui-adv.player-settings.v1"');
    expect(html).toContain('data-player-action="toggle-audio"');
    expect(html).toContain('data-player-action="cycle-motion"');
    expect(html).toContain('aria-pressed="false"');
    expect(html).toContain('소리 꺼짐');
    expect(html).toContain('연출 auto');
    expect(html).not.toContain('autoplay');
  });

  it('omits storypack preview launchers when the main storypack is already selected by default', () => {
    const html = renderStartScreen({ defaultSeed: 123, summary: null, warning: null, confirmReset: false });

    expect(html).not.toContain('data-storypack-preview-list="true"');
    expect(html).not.toContain('start-storypack-preview:wuxia_jianghu_pack');
    expect(html).toContain('data-save-key="igu-hakji.rust.save.v1"');
  });

  it('can still render explicit future storypack preview launchers without changing main save keys', () => {
    const html = renderStartScreen({
      defaultSeed: 123,
      summary: null,
      warning: null,
      confirmReset: false,
      storypackPreviews: [
        {
          id: 'yageunmong_pack',
          label: '야근몽',
          description: '회사 악몽을 별도 storypack preview로 시험합니다',
        },
      ],
    });

    expect(html).toContain('data-storypack-preview-list="true"');
    expect(html).toContain('data-player-action="start-storypack-preview:yageunmong_pack"');
    expect(html).toContain('야근몽');
    expect(html).toContain('storypack preview');
    expect(html).toContain('data-save-key="igu-hakji.rust.save.v1"');
    expect(html).not.toContain('escape-office.storypack-preview');
  });

  it('renders continue metadata and reset confirmation for an existing save', () => {
    const html = renderStartScreen({ defaultSeed: 123, summary, warning: null, confirmReset: true });

    expect(html).toContain('data-player-action="continue"');
    expect(html).not.toContain('data-player-action="continue" disabled');
    expect(html).toContain('Seed 777');
    expect(html).toContain('Turn 4');
    expect(html).toContain('printer_area');
    expect(html).toContain('2026-05-26 05:00');
    expect(html).toContain('data-player-action="confirm-new-game"');
    expect(html).toContain('data-player-action="cancel-new-game"');
    expect(html).toContain('기존 저장을 지우고 새 모험을 시작할까요?');
    expect(html).not.toContain('새 격리 run');
  });

  it('writes and reads public run summary metadata without changing Rust save JSON', () => {
    const storage = new MemoryStorage();
    const stateJson = JSON.stringify({ seed: 777, turn: 4, location_id: 'printer_area', private_secret: 'not copied' });

    writeRunSummary(storage, stateJson, new Date('2026-05-26T05:00:00.000Z'));

    expect(storage.getItem(RUST_SAVE_KEY)).toBeNull();
    const stored = storage.getItem(LAST_RUN_SUMMARY_KEY);
    expect(stored).not.toBeNull();
    expect(stored).not.toContain('private_secret');
    expect(readPlayerSaveSummary(storage)).toEqual({ summary, warning: null });
  });

  it('reports schema mismatch as a user-facing start screen warning', () => {
    const storage = new MemoryStorage();
    storage.setItem(RUST_SAVE_KEY, JSON.stringify({ seed: 123, turn: 1, location_id: 'wuxia_commute_rift' }));
    storage.setItem(LAST_RUN_SUMMARY_KEY, JSON.stringify({ schema_version: 999 }));

    const result = readPlayerSaveSummary(storage);

    expect(result.summary).toEqual({
      schema_version: 1,
      seed: 123,
      turn: 1,
      location_id: 'wuxia_commute_rift',
      saved_at: null,
    });
    expect(result.warning).toContain('지원하지 않는 저장 정보 버전');
  });

  it('renders metadata run count and seen endings count on the start screen', () => {
    const mockStorage = new MemoryStorage();
    (globalThis as any).window = { localStorage: mockStorage };

    const html1 = renderStartScreen({ defaultSeed: 123, summary: null, warning: null, confirmReset: false });
    expect(html1).toContain('첫 번째 기록');
    expect(html1).not.toContain('결말');

    const mockMeta = {
      schema_version: 1,
      run_count: 4,
      endings_seen: ['ending_1', 'ending_2'],
      achievements_seen: [],
    };
    mockStorage.setItem('igu-hakji.meta.v1', JSON.stringify(mockMeta));

    const html2 = renderStartScreen({ defaultSeed: 123, summary: null, warning: null, confirmReset: false });
    expect(html2).toContain('5번째 기록');
    expect(html2).toContain('지금까지 본 결말 2편');

    delete (globalThis as any).window;
  });

  it('clears Rust, legacy, and summary save keys together', () => {
    const storage = new MemoryStorage();
    storage.setItem(RUST_SAVE_KEY, '{}');
    storage.setItem(OFFICE_RUST_SAVE_KEY, '{}');
    storage.setItem(LEGACY_SAVE_KEY, '{}');
    storage.setItem(LAST_RUN_SUMMARY_KEY, '{}');

    clearPlayerSaves(storage);

    expect(storage.getItem(RUST_SAVE_KEY)).toBeNull();
    expect(storage.getItem(OFFICE_RUST_SAVE_KEY)).toBeNull();
    expect(storage.getItem(LEGACY_SAVE_KEY)).toBeNull();
    expect(storage.getItem(LAST_RUN_SUMMARY_KEY)).toBeNull();
  });
});
