import { describe, expect, it } from 'vitest';

import { buildTurn, executeAction } from '../game/actions';
import { newGame } from '../game/state';
import type { GameState } from '../game/types';
import { renderStorybookPage } from '../ui/storybook/render';
import { scenePageFromLegacyTurn } from './scenePageFromTurn';

function play(initial: GameState, actions: string[]): GameState {
  return actions.reduce((state, actionId) => executeAction(state, actionId).state, initial);
}

describe('temporary legacy-turn to ScenePage adapter', () => {
  it('preserves executable core action ids and printer presentation metadata for the Storybook skeleton', () => {
    const page = scenePageFromLegacyTurn(buildTurn(newGame({ seed: 123, locationId: 'printer_area' })));

    expect(page.mode).toBe('encounter');
    expect(page.title).toBe('복합기가 혼자 출력한다');
    expect(page.location.id).toBe('printer_area');
    expect(page.visual.id).toBe('printer_anomaly');
    expect(page.dialogue_entries).toEqual([
      {
        speaker: '시스템 복합기',
        text: expect.stringContaining('꺼져 있던 복합기가') as string,
        source_id: 'printer_prints_alone',
      },
    ]);
    expect(page.actions.map((action) => action.id)).toEqual([
      'choice:read_printout',
      'choice:take_printout',
      'choice:check_toner',
    ]);
    expect(page.effect_cues[0]).toMatchObject({
      kind: 'glyph_anomaly',
      source: 'copier_output',
      stable_terms: ['비상계단', '토너', '접힌 방향'],
    });
  });

  it('carries public-safe hidden-ending hints into the Storybook page body', () => {
    const finalState = play(newGame({ seed: 123, locationId: 'printer_area' }), [
      'choice:take_printout',
      'move:pantry',
      'choice:look_behind_machine',
    ]);
    const page = scenePageFromLegacyTurn(buildTurn(finalState));
    const html = renderStorybookPage(page);

    expect(page.mode).toBe('ending');
    expect(html).toContain('현실 연결 힌트: 첫 번째 현실 연결 힌트');
    expect(html).toContain('마지막 문장은 로컬 비공개 파일이 있을 때만 완성된다.');
    expect(html).toContain('퍼즐: 복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.');
    expect(html).toContain('이 힌트는 게임 안의 장소만을 말하는 것 같지 않다.');
  });
});
