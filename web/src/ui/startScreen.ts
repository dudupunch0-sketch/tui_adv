import { errorMessage } from '../core/errors';
import { escapeHtml } from './storybook/html';
import { DEFAULT_PLAYER_SETTINGS, type PlayerSettings } from './settings/playerSettings';
import {
  LAST_RUN_SUMMARY_KEY,
  LEGACY_SAVE_KEY,
  OFFICE_RUST_SAVE_KEY,
  PLAYER_SETTINGS_KEY,
  RUST_SAVE_KEY,
  readRunMetadata,
  type StorageLike,
} from '../core/storage';

export {
  LAST_RUN_SUMMARY_KEY,
  LEGACY_SAVE_KEY,
  OFFICE_RUST_SAVE_KEY,
  RUST_SAVE_KEY,
  type StorageLike,
} from '../core/storage';

const SUMMARY_SCHEMA_VERSION = 1;

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

export interface StorypackPreviewStartOption {
  id: string;
  label: string;
  description: string;
}

export interface StartScreenModel extends PlayerSaveSummaryResult {
  defaultSeed: number;
  confirmReset: boolean;
  settings?: PlayerSettings;
  storypackPreviews?: StorypackPreviewStartOption[];
  menuOpen?: boolean;
  notice?: string | null;
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
  const warning = model.warning
    ? `<p class="start-save-warning" role="status">${escapeHtml(model.warning)}</p>`
    : '';
  const notice = model.notice
    ? `<p class="start-menu-notice" role="status"><span aria-hidden="true">焚</span>${escapeHtml(model.notice)}</p>`
    : '';
  const confirmation = model.confirmReset ? renderResetConfirmation() : '';
  const previewPanel = renderStorypackPreviewPanel(model.storypackPreviews ?? []);
  const menuOpen = model.menuOpen ? 'true' : 'false';

  const storage = typeof window !== 'undefined' ? window.localStorage : { getItem: () => null, setItem: () => {}, removeItem: () => {} };
  const meta = readRunMetadata(storage);
  const runCountText = meta.run_count > 0 ? `${meta.run_count + 1}번째 기록` : '첫 번째 기록';
  const endingsCount = meta.endings_seen.length;
  const endingsText = endingsCount > 0 ? ` · 지금까지 본 결말 ${endingsCount}편` : '';

  return `
<main class="storybook-shell storybook-start" data-app="tui-adv" data-renderer="web-storybook" data-player-screen="start">
  <section class="start-hero" aria-label="게임 시작">
    <img class="start-hero-art" src="${import.meta.env.BASE_URL}assets/art/title_hero.webp" alt="" onload="this.classList.add('loaded'); this.closest('.start-hero').setAttribute('data-art-status', 'loaded')" onerror="this.style.display='none'; this.closest('.start-hero').setAttribute('data-art-status', 'failed')" />
    <svg class="start-ink-scene" viewBox="0 0 390 700" aria-hidden="true">
      <defs><filter id="start-mist"><feGaussianBlur stdDeviation="10"/></filter></defs>
      <rect width="390" height="700" fill="#e9dfc6"/>
      <path d="M0 310 Q70 190 142 300 Q215 150 390 290 V700 H0Z" fill="#20242b" opacity=".34"/>
      <path d="M0 410 Q100 320 196 400 Q285 305 390 385 V700 H0Z" fill="#3a352b" opacity=".56"/>
      <g fill="#3a352b" opacity=".16" filter="url(#start-mist)"><ellipse cx="120" cy="355" rx="130" ry="34"/><ellipse cx="278" cy="430" rx="140" ry="38"/></g>
      <path d="M145 555 L195 458 L245 555 M164 555 V505 H226 V555 M130 458 H260" fill="none" stroke="#20242b" stroke-width="9" stroke-linejoin="round"/>
      <path d="M195 555 V635 M195 580 L171 604 M195 580 L220 602 M195 635 L178 665 M195 635 L212 665" fill="none" stroke="#20242b" stroke-width="7" stroke-linecap="round"/>
      <circle cx="195" cy="548" r="12" fill="#20242b"/>
      <g transform="rotate(3 318 592)"><rect x="290" y="564" width="56" height="56" fill="#9e3c3f"/><text x="318" y="605" fill="#f3ecd6" text-anchor="middle" font-size="31">記</text></g>
    </svg>
    <div class="start-logo-lockup">
      <h1>이구학지</h1>
      <p>天記錄 — 천기록</p>
      <div class="start-meta-indicator">${escapeHtml(runCountText)}${escapeHtml(endingsText)}</div>
    </div>
    <section class="start-menu-drawer" data-start-menu-open="${menuOpen}">
      <button type="button" class="start-tap-button" data-player-action="open-start-menu">천기록을 연다</button>
      <div class="start-menu-panel" data-save-key="${RUST_SAVE_KEY}" data-summary-key="${LAST_RUN_SUMMARY_KEY}">
        <header class="start-menu-head">
          <span class="start-menu-head__title"><i aria-hidden="true">記</i>천기록</span>
          <span class="start-menu-head__meta">${escapeHtml(runCountText)}${escapeHtml(endingsText)}</span>
        </header>
        ${warning}
        ${notice}
        ${confirmation}
        <nav class="start-menu-actions" aria-label="시작 메뉴">
          <button type="button" class="start-menu-item start-menu-item--primary" data-player-action="continue"${continueDisabled}>
            <span class="start-menu-item__glyph" aria-hidden="true">續</span>
            <span class="start-menu-item__label">이어하기</span>
            <span class="start-menu-item__meta">${model.summary ? escapeHtml(summaryLine(model.summary)) : '저장된 기록 없음'}</span>
          </button>
          <button type="button" class="start-menu-item" data-player-action="new-game">
            <span class="start-menu-item__glyph" aria-hidden="true">新</span>
            <span class="start-menu-item__label">새 모험</span>
            <span class="start-menu-item__meta">낯선 관도에서 다시 눈을 뜬다</span>
          </button>
          <button type="button" class="start-menu-item start-menu-item--danger" data-player-action="reset-save"${continueDisabled}>
            <span class="start-menu-item__glyph" aria-hidden="true">焚</span>
            <span class="start-menu-item__label">기록 태우기</span>
            <span class="start-menu-item__meta">저장된 기록을 지운다</span>
          </button>
        </nav>
        ${previewPanel}
        <footer class="start-menu-footer">
          ${renderSettingsPanel(settings)}
          <details class="start-seed-detail">
            <summary><span aria-hidden="true">因</span>인연의 씨앗</summary>
            <label class="start-seed-label">
              <span>같은 씨앗은 같은 운명을 부른다</span>
              <input name="seed" type="number" inputmode="numeric" value="${escapeHtml(String(model.defaultSeed))}" />
            </label>
          </details>
        </footer>
      </div>
    </section>
  </section>
</main>`.trim();
}

function renderStorypackPreviewPanel(options: StorypackPreviewStartOption[]): string {
  if (!options.length) return '';
  const rows = options
    .map(
      (option) => `<li>
        <div>
          <strong>${escapeHtml(option.label)}</strong>
          <p>${escapeHtml(option.description)}</p>
        </div>
        <button type="button" data-player-action="start-storypack-preview:${escapeHtml(option.id)}">Preview 시작</button>
      </li>`,
    )
    .join('');

  return `<section class="start-preview-panel" data-storypack-preview-list="true" aria-label="Storypack preview">
    <h2>Storypack preview</h2>
    <p class="start-preview-copy">현재 기본 storypack과 분리해 별도 세계관을 시험합니다.</p>
    <ul>${rows}</ul>
  </section>`;
}

function renderSettingsPanel(settings: PlayerSettings): string {
  const audioPressed = settings.audio === 'on' ? 'true' : 'false';
  const audioLabel = settings.audio === 'on' ? '소리 켜짐' : '소리 꺼짐';
  const motionPressed = settings.motion === 'auto' ? 'false' : 'true';
  const motionLabel = `연출 ${settings.motion}`;

  return `<div class="start-settings-actions" role="group" aria-label="플레이어 설정" data-settings-key="${PLAYER_SETTINGS_KEY}">
        <button type="button" data-player-action="toggle-audio" aria-pressed="${audioPressed}">${audioLabel}</button>
        <button type="button" data-player-action="cycle-motion" aria-pressed="${motionPressed}">${motionLabel}</button>
      </div>`;
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
  storage.removeItem(OFFICE_RUST_SAVE_KEY);
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

function summaryLine(summary: PlayerRunSummary): string {
  const savedAt = summary.saved_at ? formatSavedAt(summary.saved_at) : '저장 시간 미기록';
  return `${summary.turn}턴까지 기록됨 · 씨앗 ${summary.seed} · ${savedAt}`;
}

function renderResetConfirmation(): string {
  return `<section class="start-reset-confirmation" role="alert" aria-label="새 모험 확인">
    <p>이전 기록이 남아 있다. 태우고 새 장을 펼칠까?</p>
    <div>
      <button type="button" class="start-reset-confirmation__burn" data-player-action="confirm-new-game">태우고 시작한다</button>
      <button type="button" data-player-action="cancel-new-game">돌아간다</button>
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
