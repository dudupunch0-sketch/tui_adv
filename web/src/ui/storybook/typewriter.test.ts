import { describe, expect, it, vi } from 'vitest';

import { startTypewriter } from './typewriter';

type ChoiceNav = {
  reveal: string | null;
  setAttribute(name: string, value: string): void;
};

function choiceNav(): ChoiceNav {
  return {
    reveal: null,
    setAttribute(name, value) {
      if (name === 'data-reveal') this.reveal = value;
    },
  };
}

function orderedShell(choices: ChoiceNav[], textNode: { textContent: string; parentElement: { appendChild(node: unknown): void } }) {
  const viewport = {
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
  };
  const ordered = {
    querySelectorAll: () => choices,
  };
  return {
    querySelectorAll: (selector: string) => (selector === '.storybook-choices' ? choices : []),
    querySelector: (selector: string) => {
      if (selector === '.story-flow--ordered') return ordered;
      if (selector === '.game-viewport') return viewport;
      return null;
    },
    ordered,
    viewport,
    textNode,
  } as unknown as HTMLElement;
}

describe('event-stage typewriter', () => {
  it('settles every choice nav when motion is disabled', () => {
    const choices = [choiceNav(), choiceNav()];
    const shell = orderedShell(choices, { textContent: '서술', parentElement: { appendChild() {} } });

    startTypewriter(shell, { enabled: false });

    expect(choices.map((choice) => choice.reveal)).toEqual(['settled', 'settled']);
  });

  it('types ordered narration while keeping all choice navs pending until finish', () => {
    vi.stubGlobal('NodeFilter', { SHOW_TEXT: 4, FILTER_REJECT: 2, FILTER_ACCEPT: 1 });
    vi.stubGlobal('window', { setInterval: vi.fn(() => 1), clearInterval: vi.fn() });
    vi.stubGlobal('document', {
      createTreeWalker: () => {
        let visited = false;
        return { nextNode: () => (visited ? null : ((visited = true), textNode)) };
      },
      createElement: () => ({
        className: '',
        setAttribute() {},
        remove() {},
      }),
    });

    const textNode = {
      textContent: '서하린이 기록을 펼친다.',
      parentElement: { appendChild() {}, closest: () => null },
    };
    const choices = [choiceNav(), choiceNav()];
    const handle = startTypewriter(orderedShell(choices, textNode), { enabled: true });

    expect(choices.map((choice) => choice.reveal)).toEqual(['pending', 'pending']);
    expect(textNode.textContent).toBe('');

    handle.finish();

    expect(textNode.textContent).toBe('서하린이 기록을 펼친다.');
    expect(choices.map((choice) => choice.reveal)).toEqual(['done', 'done']);
  });
});
