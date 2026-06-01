import '@fontsource/noto-serif-kr/korean-400.css';
import '@fontsource/noto-serif-kr/korean-700.css';
import './styles/storybook.css';

import {
  DEFAULT_STORYPACK_LABEL,
  STORYPACK_PREVIEW_OPTIONS,
  defaultStorypackLoadingPage,
  storypackPreviewById,
  storypackPreviewLoadingPage,
  type StorypackPreviewOption,
} from './core/contentBundles';
import { scenePageFromLegacyTurn } from './core/scenePageFromTurn';
import type { ScenePage } from './core/types';
import { createEscapeWasmRuntime, type EscapeWasmRuntime } from './core/wasmRuntime';
import { startPrinterFlowEffect } from './effects/printerFlow';
import { buildTurn, executeAction } from './game/actions';
import { audioCueForSceneTransition, createStorybookAudioEngine } from './ui/audio/audioEngine';
import { loadSavedState, saveState } from './game/save';
import { newGame } from './game/state';
import type { GameState, GameTurn } from './game/types';
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

const DEFAULT_SEED = 123;
const REQUIRE_WASM = import.meta.env.VITE_REQUIRE_WASM !== 'false';
const STORYPACK_PREVIEW_ACTION_PREFIX = 'start-storypack-preview:';

type PlayerScreen = 'start' | 'game';

const rootElement = document.querySelector<HTMLDivElement>('#app');
if (!rootElement) throw new Error('missing #app root');
const appRoot: HTMLDivElement = rootElement;

let playerScreen: PlayerScreen = 'start';
let wasmRuntime: EscapeWasmRuntime | null = null;
let state: GameState = newGame({ seed: DEFAULT_SEED });
let turn: GameTurn = buildTurn(state);
let actionSource: ActionListSource = { actions: [] };
let lastError: string | null = null;
let fatalPlayerError = false;
let activeSeed = DEFAULT_SEED;
let activeStorypackPreview: StorypackPreviewOption | null = null;
let confirmReset = false;
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
    if (REQUIRE_WASM) {
      lastError = `게임 코어를 불러오지 못했습니다. 새로고침 후에도 계속되면 배포된 WASM 파일 경로를 확인해주세요: ${errorMessage(error)}`;
      renderFatalPlayerError(lastError, error);
      return;
    }
    if (activeStorypackPreview) {
      lastError = `게임 코어를 불러오지 못했습니다. storypack preview는 Rust/WASM GameCore를 필요로 합니다: ${errorMessage(error)}`;
      renderFatalPlayerError(lastError, error);
      return;
    }
    lastError = `Rust GameCore WASM을 불러오지 못해 legacy mirror로 임시 실행 중입니다: ${errorMessage(error)}`;
    transitionController.cancel();
    render();
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
  actionSource = page;
  appRoot.innerHTML = renderStorybookPage(page, {
    audioLabel: playerSettings.audio === 'on' ? '소리 켜짐' : '소리 꺼짐',
    motionLabel: `연출 ${playerSettings.motion}`,
  });
  if (lastError) {
    const errorElement = document.createElement('p');
    errorElement.className = 'storybook-runtime-warning';
    errorElement.textContent = lastError;
    appRoot.prepend(errorElement);
  }
  appRoot.querySelectorAll<HTMLButtonElement>('[data-action-id]').forEach((button) => {
    button.addEventListener('click', () => runAction(button.dataset.actionId ?? ''));
  });
  appRoot.querySelectorAll<HTMLButtonElement>('[data-player-action]').forEach((button) => {
    button.addEventListener('click', () => {
      void runPlayerAction(button.dataset.playerAction ?? '');
    });
  });
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
  appRoot.querySelectorAll<HTMLButtonElement>('[data-player-action]').forEach((button) => {
    button.addEventListener('click', () => {
      void runPlayerAction(button.dataset.playerAction ?? '');
    });
  });
}

async function runPlayerAction(action: string): Promise<void> {
  if (action === 'open-start-menu') {
    const startDrawer = appRoot.querySelector<HTMLElement>('.start-menu-drawer');
    startDrawer?.setAttribute('data-start-menu-open', 'true');
    return;
  }
  if (action === 'continue') {
    await unlockAudioFromGesture();
    startGameFromSave();
    return;
  }
  if (action === 'new-game') {
    await unlockAudioFromGesture();
    requestNewGame();
    return;
  }
  if (action === 'confirm-new-game') {
    await unlockAudioFromGesture();
    startNewGame({ clearExistingSave: true });
    return;
  }
  if (action === 'cancel-new-game') {
    confirmReset = false;
    render();
    return;
  }
  if (action === 'reset-save') {
    clearPlayerSaves(window.localStorage);
    confirmReset = false;
    render();
    return;
  }
  if (action === 'show-start') {
    playerScreen = 'start';
    confirmReset = false;
    lastError = null;
    render();
    return;
  }
  if (action === 'abandon-run') {
    if (!activeStorypackPreview) clearPlayerSaves(window.localStorage);
    playerScreen = 'start';
    confirmReset = false;
    fatalPlayerError = false;
    lastError = null;
    wasmRuntime = null;
    activeStorypackPreview = null;
    state = newGame({ seed: activeSeed });
    turn = buildTurn(state);
    render();
    return;
  }
  if (action === 'toggle-audio') {
    playerSettings = updatePlayerSettings(window.localStorage, { audio: toggleAudioPreference(playerSettings) });
    await unlockAudioFromGesture();
    render();
    return;
  }
  if (action === 'cycle-motion') {
    playerSettings = updatePlayerSettings(window.localStorage, { motion: nextMotionPreference(playerSettings) });
    render();
    return;
  }
  if (action.startsWith(STORYPACK_PREVIEW_ACTION_PREFIX)) {
    await unlockAudioFromGesture();
    startStorypackPreview(action.slice(STORYPACK_PREVIEW_ACTION_PREFIX.length));
  }
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
  playerScreen = 'game';
  confirmReset = false;
  fatalPlayerError = false;
  wasmRuntime = null;
  activeSeed = options.seed;
  activeStorypackPreview = options.storypackPreview;
  state = legacyInitialState(options.seed, options.continueExistingSave);
  turn = buildTurn(state);
  if (!activeStorypackPreview && (!options.continueExistingSave || options.initialStateJson === undefined)) {
    saveLegacyState();
  }
  void bootstrapWasmRuntime(options.initialStateJson);
  const initialPage = currentScenePage();
  const startAction = { id: 'player:start', kind: 'start' };
  audioEngine.playOneShot(audioCueForSceneTransition(null, initialPage, startAction));
  renderGameTransition(null, initialPage, startAction);
}

function legacyInitialState(seed: number, shouldContinue: boolean): GameState {
  if (!shouldContinue) return newGame({ seed });
  try {
    return loadSavedState(window.localStorage) ?? newGame({ seed });
  } catch (error) {
    lastError = `저장 데이터를 읽을 수 없어 새 run으로 복구했습니다: ${errorMessage(error)}`;
    return newGame({ seed });
  }
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
  if (REQUIRE_WASM) {
    return defaultStorypackLoadingPage();
  }
  turn = buildTurn(state);
  return scenePageFromLegacyTurn(turn);
}

function runAction(actionId: string): void {
  if (fatalPlayerError) return;
  if (!actionId) return;
  if (actionId === NEW_GAME_ACTION_ID) {
    playerScreen = 'start';
    confirmReset = readPlayerSaveSummary(window.localStorage).summary !== null;
    lastError = null;
    render();
    return;
  }
  const previousPage = currentScenePage();
  const action = transitionActionContext(previousPage, actionId);
  try {
    if (wasmRuntime) {
      wasmRuntime.applyAction(actionId);
      saveWasmState();
    } else {
      const result = executeAction(state, actionId);
      state = result.state;
      saveLegacyState();
    }
    lastError = null;
    const nextPage = currentScenePage();
    audioEngine.playOneShot(audioCueForSceneTransition(previousPage, nextPage, action));
    renderGameTransition(previousPage, nextPage, action);
  } catch (error) {
    lastError = `입력 오류: ${errorMessage(error)}`;
    if (!wasmRuntime) {
      state = { ...state, log: [...state.log, lastError] };
    }
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

function saveLegacyState(): void {
  if (activeStorypackPreview) return;
  saveState(window.localStorage, state);
  writeRunSummary(
    window.localStorage,
    JSON.stringify({ seed: state.seed, turn: state.turn, locationId: state.locationId }),
  );
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

document.addEventListener('keydown', (event) => {
  if (playerScreen === 'start') return;
  const actionId = actionIdForKey(actionSource, event.key);
  if (!actionId) return;
  event.preventDefault();
  runAction(actionId);
});
