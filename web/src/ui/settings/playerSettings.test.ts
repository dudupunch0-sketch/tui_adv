import { describe, expect, it } from 'vitest';

import {
  DEFAULT_PLAYER_SETTINGS,
  PLAYER_SETTINGS_KEY,
  loadPlayerSettings,
  resolveMotionMode,
  savePlayerSettings,
  updatePlayerSettings,
  type PlayerSettings,
} from './playerSettings';

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

describe('player settings persistence', () => {
  it('defaults to muted audio and automatic motion without touching the game save', () => {
    const storage = new MemoryStorage();

    const settings = loadPlayerSettings(storage);

    expect(settings).toEqual(DEFAULT_PLAYER_SETTINGS);
    expect(settings.audio).toBe('muted');
    expect(settings.motion).toBe('auto');
    expect(resolveMotionMode(settings, { prefersReducedMotion: false })).toBe('normal');
    expect(resolveMotionMode(settings, { prefersReducedMotion: true })).toBe('reduced');
    expect(storage.getItem(PLAYER_SETTINGS_KEY)).toBeNull();
  });

  it('persists explicit audio and motion choices under the player settings key', () => {
    const storage = new MemoryStorage();
    const settings: PlayerSettings = { schema_version: 1, audio: 'on', motion: 'reduced' };

    savePlayerSettings(storage, settings);

    expect(loadPlayerSettings(storage)).toEqual(settings);
    expect(storage.getItem(PLAYER_SETTINGS_KEY)).toContain('"audio":"on"');
    expect(storage.getItem(PLAYER_SETTINGS_KEY)).toContain('"motion":"reduced"');
    expect(storage.getItem('igu-hakji.rust.save.v1')).toBeNull();
  });

  it('falls back to safe defaults for corrupt or unsupported settings', () => {
    const storage = new MemoryStorage();
    storage.setItem(PLAYER_SETTINGS_KEY, '{"schema_version":999,"audio":"on","motion":"off"}');

    expect(loadPlayerSettings(storage)).toEqual(DEFAULT_PLAYER_SETTINGS);

    storage.setItem(PLAYER_SETTINGS_KEY, 'not json');
    expect(loadPlayerSettings(storage)).toEqual(DEFAULT_PLAYER_SETTINGS);
  });

  it('updates one preference without dropping the other and resolves motion overrides', () => {
    const storage = new MemoryStorage();
    savePlayerSettings(storage, { schema_version: 1, audio: 'on', motion: 'auto' });

    const updated = updatePlayerSettings(storage, { motion: 'off' });

    expect(updated).toEqual({ schema_version: 1, audio: 'on', motion: 'off' });
    expect(loadPlayerSettings(storage)).toEqual(updated);
    expect(resolveMotionMode(updated, { prefersReducedMotion: false })).toBe('off');
  });
});
