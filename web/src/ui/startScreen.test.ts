import { describe, expect, it } from 'vitest';

import {
  LAST_RUN_SUMMARY_KEY,
  LEGACY_SAVE_KEY,
  RUST_SAVE_KEY,
  clearPlayerSaves,
  readPlayerSaveSummary,
  renderStartScreen,
  writeRunSummary,
  type PlayerRunSummary,
} from './startScreen';

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
    expect(html).toContain('ESCAPE FROM THE OFFICE');
    expect(html).toContain('data-player-action="continue" disabled');
    expect(html).toContain('data-player-action="new-game"');
    expect(html).toContain('name="seed"');
    expect(html).toContain('value="123"');
    expect(html).toContain('저장된 run 없음');
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
    storage.setItem(RUST_SAVE_KEY, JSON.stringify({ seed: 123, turn: 1, location_id: 'dev_desk' }));
    storage.setItem(LAST_RUN_SUMMARY_KEY, JSON.stringify({ schema_version: 999 }));

    const result = readPlayerSaveSummary(storage);

    expect(result.summary).toEqual({
      schema_version: 1,
      seed: 123,
      turn: 1,
      location_id: 'dev_desk',
      saved_at: null,
    });
    expect(result.warning).toContain('지원하지 않는 저장 정보 버전');
  });

  it('clears Rust, legacy, and summary save keys together', () => {
    const storage = new MemoryStorage();
    storage.setItem(RUST_SAVE_KEY, '{}');
    storage.setItem(LEGACY_SAVE_KEY, '{}');
    storage.setItem(LAST_RUN_SUMMARY_KEY, '{}');

    clearPlayerSaves(storage);

    expect(storage.getItem(RUST_SAVE_KEY)).toBeNull();
    expect(storage.getItem(LEGACY_SAVE_KEY)).toBeNull();
    expect(storage.getItem(LAST_RUN_SUMMARY_KEY)).toBeNull();
  });
});
