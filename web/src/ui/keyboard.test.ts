import { describe, expect, it } from 'vitest';

import { buildTurn } from '../game/actions';
import { newGame } from '../game/state';
import { actionIdForKey } from './keyboard';


describe('fake TUI keyboard mapping', () => {
  it('maps number keys to visible actions by index', () => {
    const turn = buildTurn(newGame({ seed: 123 }));

    expect(actionIdForKey(turn, '1')).toBe('choice:check_message');
    expect(actionIdForKey(turn, '9')).toBeNull();
  });

  it('maps n to reset and ignores reserved unknown keys for gameplay actions', () => {
    const turn = buildTurn(newGame({ seed: 123 }));

    expect(actionIdForKey(turn, 'n')).toBe('system:new-game');
    expect(actionIdForKey(turn, 'Escape')).toBeNull();
  });
});
