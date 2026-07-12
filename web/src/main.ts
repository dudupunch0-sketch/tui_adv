import '@fontsource/noto-serif-kr/korean-400.css';
import '@fontsource/noto-serif-kr/korean-700.css';
import '@fontsource/noto-serif-kr/korean-900.css';
import './styles/storybook.css';

import {
  DEFAULT_STORYPACK_LABEL,
  STORYPACK_PREVIEW_OPTIONS,
  defaultStorypackLoadingPage,
  storypackPreviewById,
  storypackPreviewLoadingPage,
  type StorypackPreviewOption,
} from './core/contentBundles';
import type { ScenePage } from './core/types';
import { errorMessage } from './core/errors';
import { createEscapeWasmRuntime, DEFAULT_SEED, type EscapeWasmRuntime } from './core/wasmRuntime';
import { startPrinterFlowEffect } from './effects/printerFlow';
import { audioCueForSceneTransition, createStorybookAudioEngine } from './ui/audio/audioEngine';
import { actionIdForKey, NEW_GAME_ACTION_ID, type ActionListSource } from './ui/keyboard';
import { createStorybookTransitionController } from './ui/motion/transitionController';
import type { TransitionActionContext } from './ui/motion/transitionPlan';
import {
  loadPlayerSettings,
  nextMotionPreference,
  resolveMotionMode,
  toggleAudioPreference,
  updatePlayerSettings,
  type PlayerSettings,
} from './ui/settings/playerSettings';
import {
  RUST_SAVE_KEY,
  clearPlayerSaves,
  readPlayerSaveSummary,
  renderStartScreen,
  writeRunSummary,
} from './ui/startScreen';
import { renderStorybookPage } from './ui/storybook/render';
import { readRunMetadata, writeRunMetadata, mergeRunMetadata } from './core/storage';

const STORYPACK_PREVIEW_ACTION_PREFIX = 'start-storypack-preview:';

type PlayerScreen = 'start' | 'game';

const rootElement = document.querySelector<HTMLDivElement>('#app');
if (!rootElement) throw new Error('missing #app root');
const appRoot: HTMLDivElement = rootElement;

let playerScreen: PlayerScreen = 'start';
let wasmRuntime: EscapeWasmRuntime | null = null;
let actionSource: ActionListSource = { actions: [] };
let lastError: string | null = null;
let fatalPlayerError = false;
let activeSeed = DEFAULT_SEED;
let activeStorypackPreview: StorypackPreviewOption | null = null;
let confirmReset = false;
let confirmAbandon = false;
let playerSettings: PlayerSettings = loadPlayerSettings(window.localStorage);
const transitionController = createStorybookTransitionController(appRoot);
const audioEngine = createStorybookAudioEngine({ preference: playerSettings.audio });

render();

async function bootstrapWasmRuntime(initialStateJson?: string): Promise<void> {
  try {
    wasmRuntime = await createEscapeWasmRuntime({
      contentBundleJson: activeStorypackPreview?.contentBundleJson,
      initialStateJson,
      seed: activeSeed,
    });
    if (playerScreen !== 'game') return;
    fatalPlayerError = false;
    lastError = null;
    saveWasmState();
    transitionController.cancel();
    render();
  } catch (error) {
    if (playerScreen !== 'game') return;
    const detail = activeStorypackPreview
      ? 'storypack preview는 Rust/WASM GameCore를 필요로 합니다'
      : 'Rust/WASM GameCore를 불러오지 못했습니다. 새로고침 후에도 계속되면 배포된 WASM 파일 경로를 확인해주세요.';
    const msg = `${detail}: ${errorMessage(error)}`;
    renderFatalPlayerError(msg, error);
  }
}

function render(): void {
  if (playerScreen === 'start') {
    renderStart();
    return;
  }
  if (fatalPlayerError) return;
  const page = currentScenePage();
  renderGamePage(page);
}

function renderGamePage(page: ScenePage): void {
  if (page.mode === 'ending') {
    const endingId = page.visual.source_id;
    if (endingId) {
      const meta = readRunMetadata(window.localStorage);
      const updated = mergeRunMetadata(meta, [endingId], page.achievement_summary.unlocked);
      writeRunMetadata(window.localStorage, updated);
    }
  }
  actionSource = page;
  appRoot.innerHTML = renderStorybookPage(page, {
    audioLabel: playerSettings.audio === 'on' ? '소리 켜짐' : '소리 꺼짐',
    motionLabel: `연출 ${playerSettings.motion}`,
  });
  if (confirmAbandon) {
    const gamePage = appRoot.querySelector<HTMLElement>('.storybook-page');
    gamePage?.prepend(renderAbandonConfirmation());
  }
  if (lastError) {
    const errorElement = document.createElement('p');
    errorElement.className = 'storybook-runtime-warning';
    errorElement.textContent = lastError;
    appRoot.prepend(errorElement);
  }
  appRoot.querySelectorAll<HTMLButtonElement>('[data-action-id]').forEach((button) => {
    button.addEventListener('click', () => runAction(button.dataset.actionId ?? ''));
  });
  wirePlayerActionButtons(appRoot);
  const canvas = appRoot.querySelector<HTMLCanvasElement>('[data-anomaly-canvas="printer-flow"]');
  if (canvas) {
    void startPrinterFlowEffect(canvas);
  }
  audioEngine.syncAmbience(page);
}

function renderStart(): void {
  transitionController.cancel();
  audioEngine.stopAmbience();
  actionSource = { actions: [] };
  const saveSummary = readPlayerSaveSummary(window.localStorage);
  const defaultSeed = saveSummary.summary?.seed ?? activeSeed;
  appRoot.innerHTML = renderStartScreen({
    defaultSeed,
    summary: saveSummary.summary,
    warning: saveSummary.warning,
    confirmReset,
    settings: playerSettings,
    storypackPreviews: STORYPACK_PREVIEW_OPTIONS,
  });
  wirePlayerActionButtons(appRoot);
}

function wirePlayerActionButtons(root: HTMLElement): void {
  root.querySelectorAll<HTMLButtonElement>('[data-player-action]').forEach((button) => {
    button.addEventListener('click', () => {
      void runPlayerAction(button.dataset.playerAction ?? '');
    });
  });

  const drawer = root.querySelector<HTMLDetailsElement>('#storybook-info-drawer');
  const drawerToggle = root.querySelector<HTMLButtonElement>('[data-player-action="toggle-storybook-drawer"]');
  if (drawer && drawerToggle) {
    drawer.addEventListener('toggle', () => drawerToggle.setAttribute('aria-expanded', String(drawer.open)));
  }
}

const playerActionHandlers: Record<string, () => void | Promise<void>> = {
  'open-start-menu': () => {
    const startDrawer = appRoot.querySelector<HTMLElement>('.start-menu-drawer');
    startDrawer?.setAttribute('data-start-menu-open', 'true');
  },
  continue: async () => {
    await unlockAudioFromGesture();
    startGameFromSave();
  },
  'new-game': async () => {
    await unlockAudioFromGesture();
    requestNewGame();
  },
  'confirm-new-game': async () => {
    await unlockAudioFromGesture();
    startNewGame({ clearExistingSave: true });
  },
  'cancel-new-game': () => {
    confirmReset = false;
    render();
  },
  'reset-save': () => {
    clearPlayerSaves(window.localStorage);
    confirmReset = false;
    confirmAbandon = false;
    render();
  },
  'show-start': () => {
    playerScreen = 'start';
    confirmReset = false;
    confirmAbandon = false;
    lastError = null;
    render();
  },
  'abandon-run': () => {
    if (activeStorypackPreview) {
      abandonRun({ clearSave: false });
      return;
    }
    confirmAbandon = true;
    render();
  },
  'confirm-abandon-run': () => {
    abandonRun({ clearSave: true });
  },
  'cancel-abandon-run': () => {
    confirmAbandon = false;
    render();
  },
  'toggle-audio': async () => {
    playerSettings = updatePlayerSettings(window.localStorage, { audio: toggleAudioPreference(playerSettings) });
    await unlockAudioFromGesture();
    render();
  },
  'cycle-motion': () => {
    playerSettings = updatePlayerSettings(window.localStorage, { motion: nextMotionPreference(playerSettings) });
    render();
  },
  'toggle-storybook-drawer': () => {
    const drawer = appRoot.querySelector<HTMLDetailsElement>('#storybook-info-drawer');
    if (drawer) drawer.open = !drawer.open;
  },
};

function abandonRun(options: { clearSave: boolean }): void {
  if (options.clearSave) clearPlayerSaves(window.localStorage);
  playerScreen = 'start';
  confirmReset = false;
  confirmAbandon = false;
  fatalPlayerError = false;
  lastError = null;
  wasmRuntime = null;
  activeStorypackPreview = null;
  render();
}

function renderAbandonConfirmation(): HTMLElement {
  const panel = document.createElement('section');
  panel.className = 'storybook-confirm';
  panel.setAttribute('role', 'alertdialog');
  panel.setAttribute('aria-label', '모험 포기 확인');
  panel.innerHTML = `<p>이 모험의 기록을 지우고 처음으로 돌아갈까요?</p>
    <div>
      <button type="button" data-player-action="confirm-abandon-run">기록을 지우고 돌아간다</button>
      <button type="button" data-player-action="cancel-abandon-run">계속 모험한다</button>
    </div>`;
  return panel;
}

async function runPlayerAction(action: string): Promise<void> {
  if (action.startsWith(STORYPACK_PREVIEW_ACTION_PREFIX)) {
    await unlockAudioFromGesture();
    startStorypackPreview(action.slice(STORYPACK_PREVIEW_ACTION_PREFIX.length));
    return;
  }

  await playerActionHandlers[action]?.();
}

async function unlockAudioFromGesture(): Promise<void> {
  audioEngine.setPreference(playerSettings.audio);
  if (playerSettings.audio !== 'on') return;
  try {
    await audioEngine.unlockFromUserGesture();
  } catch (error) {
    console.warn('Unable to unlock generated Web Audio cues', error);
  }
}

function startGameFromSave(): void {
  const saveSummary = readPlayerSaveSummary(window.localStorage);
  const initialStateJson = window.localStorage.getItem(RUST_SAVE_KEY) ?? undefined;
  activeSeed = saveSummary.summary?.seed ?? seedFromStartInput();
  startGame({
    seed: activeSeed,
    initialStateJson,
    clearExistingSave: false,
    continueExistingSave: true,
    storypackPreview: null,
  });
}

function requestNewGame(): void {
  if (readPlayerSaveSummary(window.localStorage).summary) {
    confirmReset = true;
    render();
    return;
  }
  startNewGame({ clearExistingSave: false });
}

function startNewGame(options: { clearExistingSave: boolean }): void {
  const seed = seedFromStartInput();
  startGame({
    seed,
    initialStateJson: undefined,
    clearExistingSave: options.clearExistingSave,
    continueExistingSave: false,
    storypackPreview: null,
  });
}

function startStorypackPreview(previewId: string): void {
  const preview = storypackPreviewById(previewId);
  if (!preview) {
    lastError = `알 수 없는 storypack preview입니다: ${previewId}`;
    render();
    return;
  }
  startGame({
    seed: seedFromStartInput(),
    initialStateJson: undefined,
    clearExistingSave: false,
    continueExistingSave: false,
    storypackPreview: preview,
  });
}

function startGame(options: {
  seed: number;
  initialStateJson?: string;
  clearExistingSave: boolean;
  continueExistingSave: boolean;
  storypackPreview: StorypackPreviewOption | null;
}): void {
  if (options.clearExistingSave) clearPlayerSaves(window.localStorage);
  if (!options.continueExistingSave) {
    const meta = readRunMetadata(window.localStorage);
    meta.run_count += 1;
    writeRunMetadata(window.localStorage, meta);
  }
  playerScreen = 'game';
  confirmReset = false;
  confirmAbandon = false;
  fatalPlayerError = false;
  wasmRuntime = null;
  activeSeed = options.seed;
  activeStorypackPreview = options.storypackPreview;
  void bootstrapWasmRuntime(options.initialStateJson);
  const initialPage = currentScenePage();
  const startAction = { id: 'player:start', kind: 'start' };
  audioEngine.playOneShot(audioCueForSceneTransition(null, initialPage, startAction));
  renderGameTransition(null, initialPage, startAction);
}

function seedFromStartInput(): number {
  const input = appRoot.querySelector<HTMLInputElement>('input[name="seed"]');
  const parsed = Number(input?.value ?? DEFAULT_SEED);
  if (!Number.isFinite(parsed) || parsed < 0) return DEFAULT_SEED;
  return Math.trunc(parsed);
}

function renderFatalPlayerError(message: string, error: unknown): void {
  fatalPlayerError = true;
  transitionController.cancel();
  audioEngine.stopAmbience();
  actionSource = { actions: [] };
  console.error('Failed to bootstrap required Rust GameCore WASM runtime', error);

  appRoot.innerHTML = '';
  const shell = document.createElement('main');
  shell.className = 'storybook-shell storybook-fatal';
  shell.dataset.app = 'tui-adv';
  shell.dataset.renderer = 'web-storybook';
  shell.dataset.mode = 'fatal-error';

  const title = document.createElement('h1');
  title.textContent = DEFAULT_STORYPACK_LABEL;
  const summary = document.createElement('p');
  summary.className = 'storybook-runtime-error';
  summary.textContent = message;
  const detail = document.createElement('p');
  detail.className = 'storybook-runtime-error-detail';
  detail.textContent = '이 player build는 Rust/WASM GameCore를 필수로 요구합니다.';

  shell.append(title, summary, detail);
  appRoot.append(shell);
}

function currentScenePage(): ScenePage {
  if (wasmRuntime) {
    return wasmRuntime.scenePage();
  }
  if (activeStorypackPreview) {
    return storypackPreviewLoadingPage(activeStorypackPreview);
  }
  return defaultStorypackLoadingPage();
}

function runAction(actionId: string): void {
  if (fatalPlayerError) return;
  if (!actionId) return;
  if (actionId === NEW_GAME_ACTION_ID) {
    playerScreen = 'start';
    confirmReset = readPlayerSaveSummary(window.localStorage).summary !== null;
    confirmAbandon = false;
    lastError = null;
    render();
    return;
  }
  const previousPage = currentScenePage();
  const action = transitionActionContext(previousPage, actionId);
  try {
    if (!wasmRuntime) {
      lastError = '게임 코어가 아직 로드되지 않았습니다.';
      render();
      return;
    }
    wasmRuntime.applyAction(actionId);
    saveWasmState();
    lastError = null;
    const nextPage = currentScenePage();
    audioEngine.playOneShot(audioCueForSceneTransition(previousPage, nextPage, action));
    renderGameTransition(previousPage, nextPage, action);
  } catch (error) {
    lastError = `입력 오류: ${errorMessage(error)}`;
    render();
  }
}

function renderGameTransition(
  previousPage: ScenePage | null,
  nextPage: ScenePage,
  action: TransitionActionContext | null,
): void {
  transitionController.transitionTo({
    previousPage,
    nextPage,
    action,
    motionMode: currentMotionMode(),
    renderNextPage: () => renderGamePage(nextPage),
  });
}

function transitionActionContext(page: ScenePage, actionId: string): TransitionActionContext | null {
  const action = page.actions.find((candidate) => candidate.id === actionId);
  if (!action) return null;
  return { id: action.id, kind: action.kind };
}

function currentMotionMode(): ReturnType<typeof resolveMotionMode> {
  const prefersReducedMotion =
    typeof window.matchMedia === 'function' && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  return resolveMotionMode(playerSettings, { prefersReducedMotion });
}

function saveWasmState(): void {
  if (!wasmRuntime) return;
  if (activeStorypackPreview) return;
  window.localStorage.setItem(RUST_SAVE_KEY, wasmRuntime.stateJson);
  writeRunSummary(window.localStorage, wasmRuntime.stateJson);
}


document.addEventListener('keydown', (event) => {
  if (playerScreen === 'start') return;
  const actionId = actionIdForKey(actionSource, event.key);
  if (!actionId) return;
  event.preventDefault();
  runAction(actionId);
});
