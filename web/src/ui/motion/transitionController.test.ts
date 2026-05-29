import { describe, expect, it } from 'vitest';

import type { ScenePage } from '../../core/types';
import { createStorybookTransitionController } from './transitionController';

function page(overrides: Partial<ScenePage> = {}): ScenePage {
  return {
    mode: 'movement',
    title: '복도',
    location: { id: 'hallway', name: '복도', description: '형광등이 깜빡인다.' },
    chapter_label: '격리 1턴',
    status_summary: { turn: 1, danger: 12, resources: [], warnings: [] },
    body_blocks: [],
    dialogue_entries: [],
    visual: { id: 'hallway', kind: 'location', alt: '복도', source_id: null },
    actions: [],
    blocked_actions: [],
    history_entries: [],
    inventory_summary: { items: [], overflow_count: 0 },
    achievement_summary: { unlocked: [], newly_unlocked: [] },
    pressure_cues: [],
    effect_cues: [],
    ...overrides,
  };
}

class FakeClassList {
  readonly tokens = new Set<string>();

  add(...tokens: string[]): void {
    for (const token of tokens) this.tokens.add(token);
  }

  remove(...tokens: string[]): void {
    for (const token of tokens) this.tokens.delete(token);
  }

  contains(token: string): boolean {
    return this.tokens.has(token);
  }
}

class FakeShell {
  readonly classList = new FakeClassList();
  readonly dataset: Record<string, string | undefined> = {};
  private readonly listeners = new Map<string, Set<(event: Event) => void>>();

  addEventListener(type: string, listener: (event: Event) => void): void {
    const listeners = this.listeners.get(type) ?? new Set<(event: Event) => void>();
    listeners.add(listener);
    this.listeners.set(type, listeners);
  }

  removeEventListener(type: string, listener: (event: Event) => void): void {
    this.listeners.get(type)?.delete(listener);
  }

  dispatchTransitionEnd(): void {
    const event = { target: this } as unknown as Event;
    for (const listener of this.listeners.get('transitionend') ?? []) listener(event);
  }
}

class FakeRoot {
  shell: FakeShell | null = new FakeShell();

  querySelector<T extends Element = Element>(selector: string): T | null {
    if (selector !== '.storybook-shell') return null;
    return this.shell as unknown as T;
  }
}

class FakeScheduler {
  private nextHandle = 1;
  private readonly callbacks = new Map<number, () => void>();

  setTimeout(callback: () => void): number {
    const handle = this.nextHandle;
    this.nextHandle += 1;
    this.callbacks.set(handle, callback);
    return handle;
  }

  clearTimeout(handle: number): void {
    this.callbacks.delete(handle);
  }

  flushOne(): void {
    const [handle, callback] = this.callbacks.entries().next().value as [number, () => void];
    this.callbacks.delete(handle);
    callback();
  }

  get pendingCount(): number {
    return this.callbacks.size;
  }
}

describe('Web Storybook transition controller', () => {
  it('captures the outgoing shell and waits for transitionend before rendering the next page', () => {
    const root = new FakeRoot();
    const outgoingShell = root.shell;
    const incomingShell = new FakeShell();
    const scheduler = new FakeScheduler();
    let renderCount = 0;

    const controller = createStorybookTransitionController(root as unknown as ParentNode, scheduler);
    const plan = controller.transitionTo({
      previousPage: page(),
      nextPage: page({ location: { id: 'printer_area', name: '복합기 구역', description: '' } }),
      action: { id: 'move:printer_area', kind: 'move' },
      motionMode: 'normal',
      renderNextPage: () => {
        renderCount += 1;
        root.shell = incomingShell;
      },
    });

    expect(plan).toMatchObject({ name: 'paper-slide', durationMs: 240, dangerOverlay: false });
    expect(renderCount).toBe(0);
    expect(outgoingShell?.classList.contains('storybook-transition-shell')).toBe(true);
    expect(outgoingShell?.dataset.transitionPhase).toBe('exit');
    expect(outgoingShell?.dataset.transitionName).toBe('paper-slide');

    outgoingShell?.dispatchTransitionEnd();

    expect(renderCount).toBe(1);
    expect(incomingShell.classList.contains('storybook-transition-shell')).toBe(true);
    expect(incomingShell.dataset.transitionPhase).toBe('enter');
    expect(incomingShell.dataset.transitionName).toBe('paper-slide');
  });

  it('uses a timeout fallback when transitionend never arrives', () => {
    const root = new FakeRoot();
    const scheduler = new FakeScheduler();
    let renderCount = 0;

    const controller = createStorybookTransitionController(root as unknown as ParentNode, scheduler);
    controller.transitionTo({
      previousPage: page(),
      nextPage: page({ mode: 'encounter' }),
      action: { id: 'choice:inspect', kind: 'choice' },
      motionMode: 'normal',
      renderNextPage: () => {
        renderCount += 1;
      },
    });

    expect(renderCount).toBe(0);
    expect(scheduler.pendingCount).toBe(1);

    scheduler.flushOne();

    expect(renderCount).toBe(1);
  });

  it('renders immediately without transition attributes when motion is reduced or off', () => {
    const root = new FakeRoot();
    const scheduler = new FakeScheduler();
    let renderCount = 0;

    const controller = createStorybookTransitionController(root as unknown as ParentNode, scheduler);
    const plan = controller.transitionTo({
      previousPage: page(),
      nextPage: page({ mode: 'encounter' }),
      action: { id: 'choice:inspect', kind: 'choice' },
      motionMode: 'reduced',
      renderNextPage: () => {
        renderCount += 1;
      },
    });

    expect(plan).toMatchObject({ name: 'ink-pulse', durationMs: 0 });
    expect(renderCount).toBe(1);
    expect(scheduler.pendingCount).toBe(0);
    expect(root.shell?.classList.contains('storybook-transition-shell')).toBe(false);
    expect(root.shell?.dataset.transitionPhase).toBeUndefined();
  });
});
