export const PLAYER_SETTINGS_KEY = 'escape-office.player-settings.v1';

const SETTINGS_SCHEMA_VERSION = 1;

export type AudioPreference = 'muted' | 'on';
export type MotionPreference = 'auto' | 'reduced' | 'off';
export type EffectiveMotionMode = 'normal' | 'reduced' | 'off';

export interface PlayerSettings {
  schema_version: 1;
  audio: AudioPreference;
  motion: MotionPreference;
}

export interface StorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): unknown;
}

export const DEFAULT_PLAYER_SETTINGS: PlayerSettings = {
  schema_version: SETTINGS_SCHEMA_VERSION,
  audio: 'muted',
  motion: 'auto',
};

export function loadPlayerSettings(storage: StorageLike): PlayerSettings {
  const raw = storage.getItem(PLAYER_SETTINGS_KEY);
  if (!raw) return defaultSettings();

  try {
    const parsed = JSON.parse(raw) as Partial<PlayerSettings>;
    if (isPlayerSettings(parsed)) return parsed;
  } catch {
    // Corrupt settings must not block the start screen.
  }

  return defaultSettings();
}

export function savePlayerSettings(storage: StorageLike, settings: PlayerSettings): void {
  storage.setItem(PLAYER_SETTINGS_KEY, JSON.stringify(settings));
}

export function updatePlayerSettings(storage: StorageLike, patch: Partial<Omit<PlayerSettings, 'schema_version'>>): PlayerSettings {
  const next: PlayerSettings = {
    ...loadPlayerSettings(storage),
    ...patch,
    schema_version: SETTINGS_SCHEMA_VERSION,
  };
  savePlayerSettings(storage, next);
  return next;
}

export function toggleAudioPreference(settings: PlayerSettings): AudioPreference {
  return settings.audio === 'muted' ? 'on' : 'muted';
}

export function nextMotionPreference(settings: PlayerSettings): MotionPreference {
  if (settings.motion === 'auto') return 'reduced';
  if (settings.motion === 'reduced') return 'off';
  return 'auto';
}

export function resolveMotionMode(
  settings: PlayerSettings,
  environment: { prefersReducedMotion: boolean },
): EffectiveMotionMode {
  if (settings.motion === 'off') return 'off';
  if (settings.motion === 'reduced') return 'reduced';
  return environment.prefersReducedMotion ? 'reduced' : 'normal';
}

function isPlayerSettings(value: Partial<PlayerSettings>): value is PlayerSettings {
  return (
    value.schema_version === SETTINGS_SCHEMA_VERSION &&
    (value.audio === 'muted' || value.audio === 'on') &&
    (value.motion === 'auto' || value.motion === 'reduced' || value.motion === 'off')
  );
}

function defaultSettings(): PlayerSettings {
  return { ...DEFAULT_PLAYER_SETTINGS };
}
