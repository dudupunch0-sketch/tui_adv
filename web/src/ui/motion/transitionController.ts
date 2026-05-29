import type { ScenePage } from '../../core/types';
import type { EffectiveMotionMode } from '../settings/playerSettings';
import { planTransition, type TransitionActionContext, type TransitionPlan } from './transitionPlan';

export interface TransitionScheduler {
  setTimeout(callback: () => void, delayMs: number): number;
  clearTimeout(handle: number): void;
}

export interface StorybookTransitionRequest {
  previousPage: ScenePage | null;
  nextPage: ScenePage;
  action: TransitionActionContext | null;
  motionMode: EffectiveMotionMode;
  renderNextPage: () => void;
}

export interface StorybookTransitionController {
  transitionTo(request: StorybookTransitionRequest): TransitionPlan;
  cancel(): void;
}

const STORYBOOK_SHELL_SELECTOR = '.storybook-shell';
const TRANSITION_TIMEOUT_BUFFER_MS = 80;
const TRANSITION_CLASSES = ['storybook-transition-shell', 'storybook-transition-exit', 'storybook-transition-enter'];

type TransitionPhase = 'exit' | 'enter';

export function createStorybookTransitionController(
  root: ParentNode,
  scheduler: TransitionScheduler = defaultTransitionScheduler(),
): StorybookTransitionController {
  let pendingTimeout: number | null = null;
  let pendingShell: HTMLElement | null = null;
  let pendingListener: ((event: Event) => void) | null = null;

  function clearPending(): void {
    if (pendingTimeout !== null) {
      scheduler.clearTimeout(pendingTimeout);
      pendingTimeout = null;
    }
    if (pendingShell && pendingListener) {
      pendingShell.removeEventListener('transitionend', pendingListener);
    }
    if (pendingShell) {
      clearTransitionAttributes(pendingShell);
      pendingShell = null;
    }
    pendingListener = null;
  }

  function transitionTo(request: StorybookTransitionRequest): TransitionPlan {
    clearPending();

    const plan = planTransition(request.previousPage, request.nextPage, request.action, request.motionMode);
    if (plan.durationMs === 0) {
      request.renderNextPage();
      const shell = findStorybookShell(root);
      if (shell) clearTransitionAttributes(shell);
      return plan;
    }

    const outgoingShell = findStorybookShell(root);
    if (!outgoingShell) {
      renderEnteringPage(request.renderNextPage, plan);
      return plan;
    }

    applyTransitionAttributes(outgoingShell, plan, 'exit');

    let finished = false;
    const finish = (): void => {
      if (finished) return;
      finished = true;
      if (pendingTimeout !== null) {
        scheduler.clearTimeout(pendingTimeout);
        pendingTimeout = null;
      }
      outgoingShell.removeEventListener('transitionend', listener);
      pendingShell = null;
      pendingListener = null;
      renderEnteringPage(request.renderNextPage, plan);
    };
    const listener = (event: Event): void => {
      if (event.target !== outgoingShell) return;
      finish();
    };

    pendingShell = outgoingShell;
    pendingListener = listener;
    outgoingShell.addEventListener('transitionend', listener);
    pendingTimeout = scheduler.setTimeout(finish, plan.durationMs + TRANSITION_TIMEOUT_BUFFER_MS);
    return plan;
  }

  function renderEnteringPage(renderNextPage: () => void, plan: TransitionPlan): void {
    renderNextPage();
    const incomingShell = findStorybookShell(root);
    if (!incomingShell) return;

    applyTransitionAttributes(incomingShell, plan, 'enter');
    pendingShell = incomingShell;
    pendingListener = null;
    pendingTimeout = scheduler.setTimeout(() => {
      clearTransitionAttributes(incomingShell);
      if (pendingShell === incomingShell) pendingShell = null;
      pendingTimeout = null;
    }, plan.durationMs);
  }

  return {
    transitionTo,
    cancel: clearPending,
  };
}

function defaultTransitionScheduler(): TransitionScheduler {
  return {
    setTimeout: (callback, delayMs) => globalThis.setTimeout(callback, delayMs),
    clearTimeout: (handle) => globalThis.clearTimeout(handle),
  };
}

function findStorybookShell(root: ParentNode): HTMLElement | null {
  return root.querySelector<HTMLElement>(STORYBOOK_SHELL_SELECTOR);
}

function applyTransitionAttributes(shell: HTMLElement, plan: TransitionPlan, phase: TransitionPhase): void {
  shell.classList.remove('storybook-transition-exit', 'storybook-transition-enter');
  shell.classList.add('storybook-transition-shell', `storybook-transition-${phase}`);
  shell.dataset.transitionName = plan.name;
  shell.dataset.transitionPhase = phase;
  shell.dataset.transitionDanger = plan.dangerOverlay ? 'true' : 'false';
}

function clearTransitionAttributes(shell: HTMLElement): void {
  shell.classList.remove(...TRANSITION_CLASSES);
  delete shell.dataset.transitionName;
  delete shell.dataset.transitionPhase;
  delete shell.dataset.transitionDanger;
}
