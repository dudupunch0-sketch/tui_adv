import '@fontsource/noto-serif-kr/korean-400.css';
import '@fontsource/noto-serif-kr/korean-700.css';
import './styles/storybook.css';

import { scenePageFromLegacyTurn } from './core/scenePageFromTurn';
import type { ScenePage } from './core/types';
import { createEscapeWasmRuntime, type EscapeWasmRuntime } from './core/wasmRuntime';
import { startPrinterFlowEffect } from './effects/printerFlow';
import { buildTurn, executeAction } from './game/actions';
import { loadSavedState, saveState } from './game/save';
import { newGame } from './game/state';
import type { GameState, GameTurn } from './game/types';
import { actionIdForKey, NEW_GAME_ACTION_ID, type ActionListSource } from './ui/keyboard';
import { renderStorybookPage } from './ui/storybook/render';

const LEGACY_SAVE_KEY = 'escape-office.save.v1';
const RUST_SAVE_KEY = 'escape-office.rust.save.v1';
const REQUIRE_WASM = import.meta.env.VITE_REQUIRE_WASM === 'true';

const rootElement = document.querySelector<HTMLDivElement>('#app');
if (!rootElement) throw new Error('missing #app root');
const appRoot: HTMLDivElement = rootElement;

let wasmRuntime: EscapeWasmRuntime | null = null;
let state: GameState = loadSavedState(window.localStorage) ?? newGame({ seed: 123 });
let turn: GameTurn = buildTurn(state);
let actionSource: ActionListSource = turn;
let lastError: string | null = null;
let fatalPlayerError = false;

void bootstrapWasmRuntime();
render();

async function bootstrapWasmRuntime(): Promise<void> {
  try {
    wasmRuntime = await createEscapeWasmRuntime({
      initialStateJson: window.localStorage.getItem(RUST_SAVE_KEY) ?? undefined,
      seed: 123,
    });
    fatalPlayerError = false;
    lastError = null;
    render();
  } catch (error) {
    if (REQUIRE_WASM) {
      lastError = `게임 코어를 불러오지 못했습니다. 새로고침 후에도 계속되면 배포된 WASM 파일 경로를 확인해주세요: ${errorMessage(error)}`;
      renderFatalPlayerError(lastError, error);
      return;
    }
    lastError = `Rust GameCore WASM을 불러오지 못해 legacy mirror로 임시 실행 중입니다: ${errorMessage(error)}`;
    render();
  }
}

function render(): void {
  if (fatalPlayerError) return;
  const page = currentScenePage();
  actionSource = page;
  appRoot.innerHTML = renderStorybookPage(page);
  if (lastError) {
    const errorElement = document.createElement('p');
    errorElement.className = 'storybook-runtime-warning';
    errorElement.textContent = lastError;
    appRoot.prepend(errorElement);
  }
  appRoot.querySelectorAll<HTMLButtonElement>('[data-action-id]').forEach((button) => {
    button.addEventListener('click', () => runAction(button.dataset.actionId ?? ''));
  });
  const canvas = appRoot.querySelector<HTMLCanvasElement>('[data-anomaly-canvas="printer-flow"]');
  if (canvas) {
    void startPrinterFlowEffect(canvas);
  }
}

function renderFatalPlayerError(message: string, error: unknown): void {
  fatalPlayerError = true;
  actionSource = { actions: [] };
  console.error('Failed to bootstrap required Rust GameCore WASM runtime', error);

  appRoot.innerHTML = '';
  const shell = document.createElement('main');
  shell.className = 'storybook-shell storybook-fatal';
  shell.dataset.app = 'escape-office';
  shell.dataset.renderer = 'web-storybook';
  shell.dataset.mode = 'fatal-error';

  const title = document.createElement('h1');
  title.textContent = 'ESCAPE FROM THE OFFICE';
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
  turn = buildTurn(state);
  return scenePageFromLegacyTurn(turn);
}

function runAction(actionId: string): void {
  if (fatalPlayerError) return;
  if (!actionId) return;
  if (actionId === NEW_GAME_ACTION_ID) {
    window.localStorage.removeItem(LEGACY_SAVE_KEY);
    window.localStorage.removeItem(RUST_SAVE_KEY);
    state = newGame({ seed: 123 });
    if (wasmRuntime) {
      wasmRuntime.newGame(123);
      saveWasmState();
    }
    lastError = null;
    render();
    return;
  }
  try {
    if (wasmRuntime) {
      wasmRuntime.applyAction(actionId);
      saveWasmState();
    } else {
      const result = executeAction(state, actionId);
      state = result.state;
      saveState(window.localStorage, state);
    }
    lastError = null;
    render();
  } catch (error) {
    lastError = `입력 오류: ${errorMessage(error)}`;
    if (!wasmRuntime) {
      state = { ...state, log: [...state.log, lastError] };
    }
    render();
  }
}

function saveWasmState(): void {
  if (!wasmRuntime) return;
  window.localStorage.setItem(RUST_SAVE_KEY, wasmRuntime.stateJson);
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

document.addEventListener('keydown', (event) => {
  const actionId = actionIdForKey(actionSource, event.key);
  if (!actionId) return;
  event.preventDefault();
  runAction(actionId);
});
