import { describe, expect, it } from 'vitest';

import { buildTurn, executeAction } from './actions';
import { evaluateEnding } from './endings';
import { newGame } from './state';


describe('browser game slice', () => {
  it('starts at dev_desk with the messenger encounter and fake-TUI resources', () => {
    const state = newGame({ seed: 123 });
    const turn = buildTurn(state);

    expect(state.locationId).toBe('dev_desk');
    expect(state.player.health).toBe(100);
    expect(state.player.sanity).toBe(100);
    expect(state.player.battery).toBe(100);
    expect(turn.encounter?.id).toBe('ex_employee_messenger');
    expect(turn.actions.map((action) => action.id)).toContain('choice:check_message');
  });

  it('plays the first reality-link slice from printer output to public hint ending', () => {
    let state = newGame({ seed: 123, locationId: 'printer_area' });

    state = executeAction(state, 'choice:take_printout').state;
    expect(state.inventory).toContain('crumpled_printout');
    expect(state.flags).toContain('printer_secret_started');

    state = executeAction(state, 'move:pantry').state;
    state = executeAction(state, 'choice:look_behind_machine').state;

    const ending = evaluateEnding(state);
    expect(state.flags).toContain('pantry_hint_seen');
    expect(ending?.id).toBe('hidden_reality_hint_001');
    expect(ending?.publicSecret?.id).toBe('real_note_001');
    expect(ending?.publicSecret?.public_hint_steps).toContain(
      '마지막 문장은 로컬 비공개 파일이 있을 때만 완성된다.',
    );
  });

  it('persists and restores browser save state through a storage-like adapter', async () => {
    const { loadSavedState, saveState } = await import('./save');
    const storage = new Map<string, string>();
    const adapter = {
      getItem: (key: string) => storage.get(key) ?? null,
      setItem: (key: string, value: string) => storage.set(key, value),
      removeItem: (key: string) => storage.delete(key),
    };
    const state = executeAction(newGame({ seed: 123 }), 'choice:check_message').state;

    saveState(adapter, state);
    const loaded = loadSavedState(adapter);

    expect(loaded?.turn).toBe(1);
    expect(loaded?.clues).toContain('ex_employee_contacted');
  });
});
