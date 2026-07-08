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
