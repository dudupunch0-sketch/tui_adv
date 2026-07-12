export interface StorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export const LEGACY_SAVE_KEY = 'escape-office.save.v1';
export const OFFICE_RUST_SAVE_KEY = 'escape-office.rust.save.v1';
export const RUST_SAVE_KEY = 'igu-hakji.rust.save.v1';
export const LAST_RUN_SUMMARY_KEY = 'igu-hakji.last-run-summary.v1';
export const PLAYER_SETTINGS_KEY = 'tui-adv.player-settings.v1';
export const METADATA_KEY = 'igu-hakji.meta.v1';

export interface RunMetadata {
  schema_version: number;
  run_count: number;
  endings_seen: string[];
  achievements_seen: string[];
}

export function defaultRunMetadata(): RunMetadata {
  return {
    schema_version: 1,
    run_count: 0,
    endings_seen: [],
    achievements_seen: [],
  };
}

export function readRunMetadata(storage: StorageLike): RunMetadata {
  const raw = storage.getItem(METADATA_KEY);
  if (!raw) {
    return defaultRunMetadata();
  }
  try {
    const parsed = JSON.parse(raw);
    if (
      parsed &&
      typeof parsed === 'object' &&
      parsed.schema_version === 1 &&
      typeof parsed.run_count === 'number' &&
      Array.isArray(parsed.endings_seen) &&
      Array.isArray(parsed.achievements_seen)
    ) {
      return {
        schema_version: 1,
        run_count: parsed.run_count,
        endings_seen: parsed.endings_seen.filter((x: unknown) => typeof x === 'string'),
        achievements_seen: parsed.achievements_seen.filter((x: unknown) => typeof x === 'string'),
      };
    }
  } catch {
    // fall through
  }
  return defaultRunMetadata();
}

export function writeRunMetadata(storage: StorageLike, meta: RunMetadata): void {
  storage.setItem(METADATA_KEY, JSON.stringify(meta));
}

export function mergeRunMetadata(
  meta: RunMetadata,
  endings: string[],
  achievements: string[],
): RunMetadata {
  const endingsSet = new Set([...meta.endings_seen, ...endings]);
  const achievementsSet = new Set([...meta.achievements_seen, ...achievements]);
  return {
    schema_version: meta.schema_version,
    run_count: meta.run_count,
    endings_seen: Array.from(endingsSet),
    achievements_seen: Array.from(achievementsSet),
  };
}
