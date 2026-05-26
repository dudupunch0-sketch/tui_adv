import { describe, expect, it } from 'vitest';

import type { ScenePage } from '../../core/types';
import { planTransition } from './transitionPlan';

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

describe('Web Storybook transition plan', () => {
  it('maps start and movement transitions without requiring ScenePage schema changes', () => {
    expect(planTransition(null, page(), { id: 'player:start', kind: 'start' }, 'normal')).toMatchObject({
      name: 'start-fade',
      durationMs: 220,
      dangerOverlay: false,
    });

    expect(
      planTransition(page(), page({ location: { id: 'printer_area', name: '복합기 구역', description: '' } }), { id: 'move:printer_area', kind: 'move' }, 'normal'),
    ).toMatchObject({ name: 'paper-slide', durationMs: 240, dangerOverlay: false });
  });

  it('uses encounter, ending, and danger overlay variants deterministically', () => {
    expect(planTransition(page(), page({ mode: 'encounter' }), { id: 'choice:inspect', kind: 'choice' }, 'normal')).toMatchObject({
      name: 'ink-pulse',
      durationMs: 220,
      dangerOverlay: false,
    });
    expect(planTransition(page(), page({ mode: 'ending' }), { id: 'choice:escape', kind: 'choice' }, 'normal')).toMatchObject({
      name: 'ending-fade',
      durationMs: 320,
      dangerOverlay: false,
    });
    expect(
      planTransition(
        page(),
        page({
          status_summary: { turn: 9, danger: 81, resources: [], warnings: ['위험합니다.'] },
          effect_cues: [{ kind: 'glyph_anomaly', source: 'printer', intensity: 0.9, stable_terms: [], distortion: 'glitch', duration_hint_ms: 300, fallback_text: null }],
        }),
        { id: 'choice:listen', kind: 'choice' },
        'normal',
      ),
    ).toMatchObject({ name: 'danger-glitch', durationMs: 320, dangerOverlay: true });
  });

  it('collapses every transition to a no-op when motion is reduced or off', () => {
    expect(planTransition(page(), page({ mode: 'encounter' }), { id: 'choice:inspect', kind: 'choice' }, 'reduced')).toMatchObject({
      name: 'ink-pulse',
      durationMs: 0,
    });
    expect(planTransition(page(), page({ mode: 'encounter' }), null, 'off')).toMatchObject({
      name: 'paper-fade',
      durationMs: 0,
    });
  });
});
