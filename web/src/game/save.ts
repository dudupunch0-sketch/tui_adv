import type { GameState } from './types';

const SAVE_KEY = 'escape-office.save.v1';
const SAVE_SCHEMA_VERSION = 1;

export interface StorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): unknown;
  removeItem(key: string): unknown;
}

interface SaveEnvelope {
  schema_version: number;
  state: GameState;
}

export function saveState(storage: StorageLike, state: GameState): void {
  const envelope: SaveEnvelope = {
    schema_version: SAVE_SCHEMA_VERSION,
    state,
  };
  storage.setItem(SAVE_KEY, JSON.stringify(envelope));
}

export function loadSavedState(storage: StorageLike): GameState | null {
  const raw = storage.getItem(SAVE_KEY);
  if (raw === null) return null;
  const envelope = JSON.parse(raw) as SaveEnvelope;
  if (envelope.schema_version !== SAVE_SCHEMA_VERSION) {
    throw new Error(`unsupported save schema: ${envelope.schema_version}`);
  }
  return envelope.state;
}

export function clearSavedState(storage: StorageLike): void {
  storage.removeItem(SAVE_KEY);
}
