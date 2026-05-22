import './styles/storybook.css';

import { buildTurn, executeAction } from './game/actions';
import { loadSavedState, saveState } from './game/save';
import { newGame } from './game/state';
import type { GameState, GameTurn } from './game/types';
import { scenePageFromLegacyTurn } from './core/scenePageFromTurn';
import { startPrinterFlowEffect } from './effects/printerFlow';
import { actionIdForKey, NEW_GAME_ACTION_ID } from './ui/keyboard';
import { renderStorybookPage } from './ui/storybook/render';

const rootElement = document.querySelector<HTMLDivElement>('#app');
if (!rootElement) throw new Error('missing #app root');
const appRoot: HTMLDivElement = rootElement;

let state: GameState = loadSavedState(window.localStorage) ?? newGame({ seed: 123 });
let turn: GameTurn = buildTurn(state);

function render(): void {
  turn = buildTurn(state);
  appRoot.innerHTML = renderStorybookPage(scenePageFromLegacyTurn(turn));
  appRoot.querySelectorAll<HTMLButtonElement>('[data-action-id]').forEach((button) => {
    button.addEventListener('click', () => runAction(button.dataset.actionId ?? ''));
  });
  const canvas = appRoot.querySelector<HTMLCanvasElement>('[data-anomaly-canvas="printer-flow"]');
  if (canvas) {
    void startPrinterFlowEffect(canvas);
  }
}

function runAction(actionId: string): void {
  if (!actionId) return;
  if (actionId === NEW_GAME_ACTION_ID) {
    window.localStorage.removeItem('escape-office.save.v1');
    state = newGame({ seed: 123 });
    render();
    return;
  }
  try {
    const result = executeAction(state, actionId);
    state = result.state;
    saveState(window.localStorage, state);
    render();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    state = { ...state, log: [...state.log, `입력 오류: ${message}`] };
    render();
  }
}

document.addEventListener('keydown', (event) => {
  const actionId = actionIdForKey(turn, event.key);
  if (!actionId) return;
  event.preventDefault();
  runAction(actionId);
});

render();
