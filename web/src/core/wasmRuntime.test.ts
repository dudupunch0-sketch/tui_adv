import { describe, expect, it } from 'vitest';

import type { ScenePage } from './types';
import { EscapeWasmRuntime, type EscapeWasmBindings } from './wasmRuntime';

function samplePage(overrides: Partial<ScenePage> = {}): ScenePage {
  return {
    mode: 'encounter',
    title: 'Rust ScenePage',
    location: { id: 'dev_desk', name: '내 자리', description: 'desk' },
    chapter_label: '격리 0턴',
    status_summary: {
      turn: 0,
      danger: 0,
      resources: [
        { id: 'health', label: '신체 반응', band: 'normal', text: '정상 범위', value: 100 },
        { id: 'sanity', label: '집중도', band: 'normal', text: '안정', value: 100 },
        { id: 'battery', label: '단말기 전원', band: 'normal', text: '100%', value: 100 },
        { id: 'hunger', label: '허기', band: 'normal', text: '버틸 만함', value: 0 },
        { id: 'thirst', label: '갈증', band: 'normal', text: '버틸 만함', value: 0 },
      ],
      warnings: [],
    },
    body_blocks: [{ kind: 'narration', text: 'body', source_id: 'dev_desk' }],
    dialogue_entries: [],
    visual: { id: 'encounter:rust', kind: 'encounter', alt: 'Rust ScenePage', source_id: 'rust' },
    actions: [{ id: 'choice:check_message', label: '메시지를 확인한다', kind: 'choice', cost_text: null }],
    blocked_actions: [],
    history_entries: [],
    inventory_summary: { items: [], overflow_count: 0 },
    achievement_summary: { unlocked: [], newly_unlocked: [] },
    pressure_cues: [],
    effect_cues: [],
    ...overrides,
  };
}

describe('EscapeWasmRuntime', () => {
  it('drives Web Storybook state through escape-wasm JSON boundary functions', () => {
    const calls: string[] = [];
    const bindings: EscapeWasmBindings = {
      new_game_json(seed, contentBundleJson) {
        calls.push(`new:${String(seed)}:${contentBundleJson.includes('content')}`);
        return JSON.stringify({ seed: Number(seed), turn: 0, location_id: 'dev_desk' });
      },
      scene_page_json(stateJson, contentBundleJson) {
        calls.push(`page:${JSON.parse(stateJson).turn}:${contentBundleJson.includes('content')}`);
        return JSON.stringify(samplePage({ title: `Rust turn ${JSON.parse(stateJson).turn}` }));
      },
      apply_action_json(stateJson, contentBundleJson, actionId) {
        calls.push(`action:${actionId}:${contentBundleJson.includes('content')}`);
        const state = { ...JSON.parse(stateJson), turn: JSON.parse(stateJson).turn + 1 };
        return JSON.stringify({
          encounter_id: 'ex_employee_messenger',
          action_id: actionId,
          state,
          logs: ['퇴사자의 메시지를 확인했다.'],
          effect_cues: [],
          newly_unlocked_achievements: ['first_signal_received'],
        });
      },
    };

    const runtime = new EscapeWasmRuntime(bindings, JSON.stringify({ content: {} }));

    expect(runtime.scenePage().title).toBe('Rust turn 0');
    const result = runtime.applyAction('choice:check_message');

    expect(result.newly_unlocked_achievements).toEqual(['first_signal_received']);
    expect(runtime.scenePage().title).toBe('Rust turn 1');
    expect(calls).toEqual([
      'new:123:true',
      'page:0:true',
      'page:0:true',
      'action:choice:check_message:true',
      'page:1:true',
      'page:1:true',
    ]);
  });

  it('rejects corrupt saved Rust state before callers install the runtime', () => {
    const bindings: EscapeWasmBindings = {
      new_game_json() {
        return JSON.stringify({ turn: 0, location_id: 'dev_desk' });
      },
      scene_page_json(stateJson: string) {
        const state = JSON.parse(stateJson) as { location_id?: string };
        if (state.location_id !== 'dev_desk') throw new Error('unknown location');
        return JSON.stringify(samplePage());
      },
      apply_action_json() {
        throw new Error('not used');
      },
    };

    expect(
      () => new EscapeWasmRuntime(bindings, JSON.stringify({ content: {} }), '{"location_id":"stale"}'),
    ).toThrow('unknown location');
  });

  it('does not commit an action result whose next state cannot render a ScenePage', () => {
    const bindings: EscapeWasmBindings = {
      new_game_json() {
        return JSON.stringify({ turn: 0, location_id: 'dev_desk' });
      },
      scene_page_json(stateJson: string) {
        const state = JSON.parse(stateJson) as { turn: number; location_id: string };
        if (state.location_id !== 'dev_desk') throw new Error('unknown location');
        return JSON.stringify(samplePage({ title: `Rust turn ${state.turn}` }));
      },
      apply_action_json() {
        return JSON.stringify({
          encounter_id: 'ex_employee_messenger',
          action_id: 'choice:bad_transition',
          state: { turn: 1, location_id: 'missing_location' },
          logs: ['bad transition'],
          effect_cues: [],
          newly_unlocked_achievements: [],
        });
      },
    };
    const runtime = new EscapeWasmRuntime(bindings, JSON.stringify({ content: {} }));

    expect(() => runtime.applyAction('choice:bad_transition')).toThrow('unknown location');
    expect(runtime.scenePage().title).toBe('Rust turn 0');
  });
});
