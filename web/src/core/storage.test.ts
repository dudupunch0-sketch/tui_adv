import { describe, expect, it } from 'vitest';
import {
  readRunMetadata,
  writeRunMetadata,
  mergeRunMetadata,
  defaultRunMetadata,
  METADATA_KEY,
  type StorageLike,
} from './storage';

class MockStorage implements StorageLike {
  private data: Record<string, string> = {};

  getItem(key: string): string | null {
    return this.data[key] ?? null;
  }
  setItem(key: string, value: string): void {
    this.data[key] = value;
  }
  removeItem(key: string): void {
    delete this.data[key];
  }
}

describe('RunMetadata storage helpers', () => {
  it('reads default metadata when storage is empty', () => {
    const storage = new MockStorage();
    const meta = readRunMetadata(storage);
    expect(meta).toEqual(defaultRunMetadata());
  });

  it('performs write and read round-trip correctly', () => {
    const storage = new MockStorage();
    const meta = {
      schema_version: 1,
      run_count: 5,
      endings_seen: ['ending_a'],
      achievements_seen: ['achievement_b'],
    };
    writeRunMetadata(storage, meta);
    const read = readRunMetadata(storage);
    expect(read).toEqual(meta);
  });

  it('recovers with default metadata if JSON is corrupt or outdated', () => {
    const storage = new MockStorage();
    storage.setItem(METADATA_KEY, '{invalid json');
    expect(readRunMetadata(storage)).toEqual(defaultRunMetadata());

    storage.setItem(METADATA_KEY, JSON.stringify({ schema_version: 2, run_count: 3 }));
    expect(readRunMetadata(storage)).toEqual(defaultRunMetadata());
  });

  it('merges endings and achievements uniquely without duplicates', () => {
    const initial = {
      schema_version: 1,
      run_count: 2,
      endings_seen: ['ending_1'],
      achievements_seen: ['ach_1', 'ach_2'],
    };
    const merged = mergeRunMetadata(initial, ['ending_1', 'ending_2'], ['ach_2', 'ach_3']);
    expect(merged.endings_seen).toEqual(['ending_1', 'ending_2']);
    expect(merged.achievements_seen).toEqual(['ach_1', 'ach_2', 'ach_3']);
  });
});
