import { describe, expect, it } from 'vitest';

import type { ScenePage } from '../../core/types';
import {
  ambienceCueForPage,
  audioCueForSceneTransition,
  createStorybookAudioEngine,
  type AudioAmbienceCue,
  type AudioOneShotCue,
  type GeneratedAudioBackend,
} from './audioEngine';

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

class FakeBackend implements GeneratedAudioBackend {
  resumeCalls = 0;
  stopAllCalls = 0;
  readonly oneShots: AudioOneShotCue[] = [];
  readonly loopsStarted: AudioAmbienceCue[] = [];
  readonly loopsStopped: AudioAmbienceCue[] = [];

  async resume(): Promise<boolean> {
    this.resumeCalls += 1;
    return true;
  }

  playOneShot(cue: AudioOneShotCue): boolean {
    this.oneShots.push(cue);
    return true;
  }

  startLoop(cue: AudioAmbienceCue): boolean {
    this.loopsStarted.push(cue);
    return true;
  }

  stopLoop(cue: AudioAmbienceCue): void {
    this.loopsStopped.push(cue);
  }

  stopAllLoops(): void {
    this.stopAllCalls += 1;
  }
}

describe('Web Storybook audio engine', () => {
  it('keeps muted audio as a true no-op without resuming Web Audio or scheduling cues', async () => {
    const backend = new FakeBackend();
    const engine = createStorybookAudioEngine({ preference: 'muted', backend });

    expect(await engine.unlockFromUserGesture()).toBe(false);
    expect(engine.playOneShot('choice')).toBe(false);
    expect(
      engine.syncAmbience(
        page({ status_summary: { turn: 4, danger: 88, resources: [], warnings: ['위험합니다.'] } }),
      ),
    ).toBeNull();

    expect(backend.resumeCalls).toBe(0);
    expect(backend.oneShots).toEqual([]);
    expect(backend.loopsStarted).toEqual([]);
    expect(backend.stopAllCalls).toBe(0);
  });

  it('requires explicit opt-in gesture before one-shot cues and looping ambience are scheduled', async () => {
    const backend = new FakeBackend();
    const engine = createStorybookAudioEngine({ preference: 'muted', backend });

    engine.setPreference('on');

    expect(backend.resumeCalls).toBe(0);
    expect(engine.playOneShot('choice')).toBe(false);
    expect(await engine.unlockFromUserGesture()).toBe(true);
    expect(backend.resumeCalls).toBe(1);

    expect(engine.playOneShot('choice')).toBe(true);
    expect(backend.oneShots).toEqual(['choice']);

    expect(
      engine.syncAmbience(
        page({ status_summary: { turn: 7, danger: 84, resources: [], warnings: ['경계선 접근'] } }),
      ),
    ).toBe('danger');
    expect(engine.syncAmbience(page())).toBe('office');
    engine.stopAmbience();
    expect(backend.loopsStarted).toEqual(['danger', 'office']);
    expect(backend.loopsStopped).toEqual(['danger', 'office']);

    engine.setPreference('muted');

    expect(engine.playOneShot('danger')).toBe(false);
    expect(backend.stopAllCalls).toBe(1);
  });

  it('maps renderer-local page/action context to cue names without adding audio fields to ScenePage', () => {
    expect(audioCueForSceneTransition(null, page(), { id: 'player:start', kind: 'start' })).toBe('start');
    expect(audioCueForSceneTransition(page(), page(), { id: 'choice:inspect', kind: 'choice' })).toBe('choice');
    expect(
      audioCueForSceneTransition(
        page(),
        page({
          effect_cues: [
            {
              kind: 'glyph_anomaly',
              source: 'printer',
              intensity: 0.8,
              stable_terms: ['복합기'],
              distortion: 'glyphs',
              duration_hint_ms: 300,
              fallback_text: '복합기 잡음',
            },
          ],
        }),
        { id: 'choice:listen', kind: 'choice' },
      ),
    ).toBe('glyph');
    expect(
      audioCueForSceneTransition(
        page(),
        page({ status_summary: { turn: 9, danger: 82, resources: [], warnings: ['위험합니다.'] } }),
        { id: 'choice:run', kind: 'choice' },
      ),
    ).toBe('danger');

    expect(ambienceCueForPage(page())).toBe('office');
    expect(ambienceCueForPage(page({ mode: 'ending' }))).toBeNull();
    expect(ambienceCueForPage(page({ status_summary: { turn: 9, danger: 82, resources: [], warnings: [] } }))).toBe(
      'danger',
    );
  });
});
