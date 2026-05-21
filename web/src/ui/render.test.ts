import { describe, expect, it } from 'vitest';

import { buildTurn } from '../game/actions';
import { newGame } from '../game/state';
import { renderGameShell } from './render';


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
});
