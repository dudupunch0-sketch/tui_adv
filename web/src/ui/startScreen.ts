import { escapeHtml } from './storybook/html';
import { DEFAULT_PLAYER_SETTINGS, PLAYER_SETTINGS_KEY, type PlayerSettings } from './settings/playerSettings';

export const LEGACY_SAVE_KEY = 'escape-office.save.v1';
export const RUST_SAVE_KEY = 'escape-office.rust.save.v1';
export const SETTINGS_KEY = 'escape-office.settings.v1';
export const LAST_RUN_SUMMARY_KEY = 'escape-office.last-run-summary.v1';

const SUMMARY_SCHEMA_VERSION = 1;

export interface StorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): unknown;
  removeItem(key: string): unknown;
}

export interface PlayerRunSummary {
  schema_version: 1;
  seed: number;
  turn: number;
  location_id: string;
  saved_at: string | null;
}

export interface PlayerSaveSummaryResult {
  summary: PlayerRunSummary | null;
  warning: string | null;
}

export interface StartScreenModel extends PlayerSaveSummaryResult {
  defaultSeed: number;
  confirmReset: boolean;
  settings?: PlayerSettings;
}

interface RawStatePreview {
  seed?: unknown;
  turn?: unknown;
  location_id?: unknown;
  locationId?: unknown;
}

export function renderStartScreen(model: StartScreenModel): string {
  const settings = model.settings ?? DEFAULT_PLAYER_SETTINGS;
  const continueDisabled = model.summary ? '' : ' disabled';
  const saveText = model.summary ? renderSummary(model.summary) : '<p class="start-save-empty">저장된 run 없음</p>';
  const warning = model.warning
    ? `<p class="start-save-warning" role="status">${escapeHtml(model.warning)}</p>`
    : '';
  const confirmation = model.confirmReset ? renderResetConfirmation() : '';

  return `
<main class="storybook-shell storybook-start" data-app="escape-office" data-renderer="web-storybook" data-player-screen="start">
  <section class="start-card" aria-label="게임 시작">
    <p class="start-kicker">WEB PLAYER · RUST/WASM GAMECORE</p>
    <h1>ESCAPE FROM THE OFFICE</h1>
    <p class="start-copy">주소만 열면 바로 시작되는 회사-아포칼립스 생존 기록입니다. 저장은 이 브라우저의 localStorage에만 남습니다.</p>
    ${warning}
    <div class="start-save-panel" data-save-key="${RUST_SAVE_KEY}" data-summary-key="${LAST_RUN_SUMMARY_KEY}">
      <h2>격리 run</h2>
      ${saveText}
    </div>
    ${renderSettingsPanel(settings)}
    <label class="start-seed-label">
      <span>Seed</span>
      <input name="seed" type="number" inputmode="numeric" value="${escapeHtml(String(model.defaultSeed))}" />
    </label>
    <div class="start-actions">
      <button type="button" class="start-primary" data-player-action="continue"${continueDisabled}>이어하기</button>
      <button type="button" data-player-action="new-game">새 게임</button>
      <button type="button" data-player-action="reset-save"${continueDisabled}>저장 초기화</button>
    </div>
    ${confirmation}
  </section>
</main>`.trim();
}

function renderSettingsPanel(settings: PlayerSettings): string {
  const audioPressed = settings.audio === 'on' ? 'true' : 'false';
  const audioLabel = settings.audio === 'on' ? 'Audio on' : 'Audio muted';
  const motionPressed = settings.motion === 'auto' ? 'false' : 'true';
  const motionLabel = `Motion ${settings.motion}`;

  return `<section class="start-settings-panel" aria-label="플레이어 설정" data-settings-key="${PLAYER_SETTINGS_KEY}">
      <h2>연출 설정</h2>
      <p class="start-settings-copy">소리와 움직임은 이 브라우저에만 저장됩니다. Audio는 사용자가 켠 뒤에만 시작합니다.</p>
      <div class="start-settings-actions">
        <button type="button" data-player-action="toggle-audio" aria-pressed="${audioPressed}">${audioLabel}</button>
        <button type="button" data-player-action="cycle-motion" aria-pressed="${motionPressed}">${motionLabel}</button>
      </div>
    </section>`;
}

export function readPlayerSaveSummary(storage: StorageLike): PlayerSaveSummaryResult {
  const rawRustSave = storage.getItem(RUST_SAVE_KEY);
  const rawSummary = storage.getItem(LAST_RUN_SUMMARY_KEY);
  let warning: string | null = null;

  if (rawSummary) {
    try {
      const parsed = JSON.parse(rawSummary) as Partial<PlayerRunSummary>;
      if (parsed.schema_version !== SUMMARY_SCHEMA_VERSION) {
        warning = `지원하지 않는 저장 정보 버전입니다: ${String(parsed.schema_version)}`;
      } else if (isRunSummary(parsed)) {
        return { summary: parsed, warning: null };
      } else {
        warning = '저장 정보가 손상되었습니다. 저장 본문에서 가능한 정보를 복구합니다.';
      }
    } catch (error) {
      warning = `저장 정보를 읽을 수 없습니다. 저장 본문에서 가능한 정보를 복구합니다: ${errorMessage(error)}`;
    }
  }

  if (!rawRustSave) {
    return { summary: null, warning };
  }

  try {
    return { summary: summaryFromStateJson(rawRustSave, null), warning };
  } catch (error) {
    return {
      summary: null,
      warning: `저장 데이터를 읽을 수 없습니다. 새 게임을 시작하거나 저장을 초기화해주세요: ${errorMessage(error)}`,
    };
  }
}

export function writeRunSummary(storage: StorageLike, stateJson: string, savedAt: Date = new Date()): void {
  const summary = summaryFromStateJson(stateJson, savedAt.toISOString());
  storage.setItem(LAST_RUN_SUMMARY_KEY, JSON.stringify(summary));
}

export function clearPlayerSaves(storage: StorageLike): void {
  storage.removeItem(RUST_SAVE_KEY);
  storage.removeItem(LEGACY_SAVE_KEY);
  storage.removeItem(LAST_RUN_SUMMARY_KEY);
}

export function summaryFromStateJson(stateJson: string, savedAt: string | null): PlayerRunSummary {
  const parsed = JSON.parse(stateJson) as RawStatePreview;
  const seed = numberField(parsed.seed, 'seed');
  const turn = numberField(parsed.turn, 'turn');
  const location = stringField(parsed.location_id ?? parsed.locationId, 'location_id');

  return {
    schema_version: SUMMARY_SCHEMA_VERSION,
    seed,
    turn,
    location_id: location,
    saved_at: savedAt,
  };
}

function renderSummary(summary: PlayerRunSummary): string {
  const savedAt = summary.saved_at ? formatSavedAt(summary.saved_at) : '저장 시간 미기록';
  return `<dl class="start-save-summary">
    <div><dt>Seed</dt><dd>Seed ${summary.seed}</dd></div>
    <div><dt>Turn</dt><dd>Turn ${summary.turn}</dd></div>
    <div><dt>Location</dt><dd>${escapeHtml(summary.location_id)}</dd></div>
    <div><dt>Saved</dt><dd>${escapeHtml(savedAt)}</dd></div>
  </dl>`;
}

function renderResetConfirmation(): string {
  return `<section class="start-reset-confirmation" role="alert" aria-label="새 게임 확인">
    <p>기존 저장을 지우고 새 격리 run을 시작할까요?</p>
    <div>
      <button type="button" data-player-action="confirm-new-game">기존 저장 삭제 후 시작</button>
      <button type="button" data-player-action="cancel-new-game">돌아가기</button>
    </div>
  </section>`;
}

function isRunSummary(value: Partial<PlayerRunSummary>): value is PlayerRunSummary {
  return (
    value.schema_version === SUMMARY_SCHEMA_VERSION &&
    typeof value.seed === 'number' &&
    typeof value.turn === 'number' &&
    typeof value.location_id === 'string' &&
    (typeof value.saved_at === 'string' || value.saved_at === null)
  );
}

function numberField(value: unknown, name: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new Error(`missing numeric ${name}`);
  }
  return value;
}

function stringField(value: unknown, name: string): string {
  if (typeof value !== 'string' || !value) {
    throw new Error(`missing string ${name}`);
  }
  return value;
}

function formatSavedAt(value: string): string {
  return value.slice(0, 16).replace('T', ' ');
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
