import { describe, expect, it } from 'vitest';

import { buildTurn, executeAction } from '../game/actions';
import { newGame } from '../game/state';
import type { GameState } from '../game/types';
import { renderGameShell } from './render';

function play(initial: GameState, actions: string[]): GameState {
  return actions.reduce((state, actionId) => executeAction(state, actionId).state, initial);
}

describe('fake TUI renderer', () => {
  it('renders a terminal-like shell with status, encounter, choices, and log regions', () => {
    const html = renderGameShell(buildTurn(newGame({ seed: 123 })));

    expect(html).toContain('escape from the office');
    expect(html).toContain('LOCAL STATUS');
    expect(html).toContain('CURRENT ENCOUNTER');
    expect(html).toContain('퇴사자의 메신저');
    expect(html).toContain('data-panel="status"');
    expect(html).toContain('data-panel="log"');
    expect(html).toContain('data-action-id="choice:check_message"');
  });

  it('marks printer output scenes as pretext-capable anomaly panels', () => {
    const html = renderGameShell(buildTurn(newGame({ seed: 123, locationId: 'printer_area' })));

    expect(html).toContain('data-effect="printer-flow"');
    expect(html).toContain('ANOMALY CANVAS READY');
  });

  it('renders inventory and usable item controls in a dedicated fake-TUI panel', () => {
    const state = play(newGame({ seed: 123 }), [
      'choice:check_message',
      'move:dev_office',
      'move:supply_closet',
      'choice:take_power_bank',
    ]);
    const html = renderGameShell(buildTurn(state));

    expect(html).toContain('data-panel="inventory"');
    expect(html).toContain('INVENTORY');
    expect(html).toContain('보조배터리');
    expect(html).toContain('data-action-id="use:power_bank"');
  });

  it('renders unlocked achievements in a dedicated fake-TUI panel', () => {
    const state = executeAction(newGame({ seed: 123 }), 'choice:check_message').state;
    const html = renderGameShell(buildTurn(state));

    expect(html).toContain('data-panel="achievements"');
    expect(html).toContain('ACHIEVEMENTS');
    expect(html).toContain('첫 신호 확인');
    expect(html).toContain('퇴사자의 첫 메시지를 확인했다.');
  });

  it('renders help/save and pressure panels without leaving the fake-terminal identity', () => {
    const lowSanity = {
      ...newGame({ seed: 123 }),
      player: { ...newGame({ seed: 123 }).player, sanity: 30 },
    };
    const html = renderGameShell(buildTurn(lowSanity));

    expect(html).toContain('data-panel="controls"');
    expect(html).toContain('CONTROLS / SAVE');
    expect(html).toContain('1-9 선택');
    expect(html).toContain('N 새 게임');
    expect(html).toContain('localStorage 자동 저장');
    expect(html).toContain('data-panel="pressure"');
    expect(html).toContain('LOW SANITY');
    expect(html).toContain('선택지 왜곡');
  });
});
