import type { ScenePage } from '../../core/types';
import type { TransitionActionContext } from '../motion/transitionPlan';
import type { AudioPreference } from '../settings/playerSettings';

const DANGER_AUDIO_THRESHOLD = 80;

export type AudioOneShotCue = 'start' | 'choice' | 'danger' | 'glyph';
export type AudioAmbienceCue = 'office' | 'danger';

export interface GeneratedAudioBackend {
  resume(): Promise<boolean>;
  playOneShot(cue: AudioOneShotCue): boolean;
  startLoop(cue: AudioAmbienceCue): boolean;
  stopLoop(cue: AudioAmbienceCue): void;
  stopAllLoops(): void;
}

export interface StorybookAudioEngine {
  setPreference(preference: AudioPreference): void;
  unlockFromUserGesture(): Promise<boolean>;
  playOneShot(cue: AudioOneShotCue): boolean;
  syncAmbience(page: ScenePage): AudioAmbienceCue | null;
  stopAmbience(): void;
}

export interface StorybookAudioEngineOptions {
  preference: AudioPreference;
  backend?: GeneratedAudioBackend;
}

export function createStorybookAudioEngine(options: StorybookAudioEngineOptions): StorybookAudioEngine {
  return new StorybookAudioEngineImpl(options.preference, options.backend ?? createWebAudioGeneratedBackend());
}

export function audioCueForSceneTransition(
  previousPage: ScenePage | null,
  nextPage: ScenePage,
  action: TransitionActionContext | null,
): AudioOneShotCue {
  if (!previousPage || action?.kind === 'start') return 'start';
  if (nextPage.effect_cues.some((cue) => cue.kind === 'glyph_anomaly')) return 'glyph';
  if (isDangerPage(nextPage)) return 'danger';
  return 'choice';
}

export function ambienceCueForPage(page: ScenePage): AudioAmbienceCue | null {
  if (page.mode === 'ending') return null;
  return isDangerPage(page) ? 'danger' : 'office';
}

export function createWebAudioGeneratedBackend(): GeneratedAudioBackend {
  return new WebAudioGeneratedBackend(globalThis);
}

class StorybookAudioEngineImpl implements StorybookAudioEngine {
  private unlocked = false;
  private currentLoop: AudioAmbienceCue | null = null;

  constructor(
    private preference: AudioPreference,
    private readonly backend: GeneratedAudioBackend,
  ) {}

  setPreference(preference: AudioPreference): void {
    this.preference = preference;
    if (preference === 'muted') {
      this.unlocked = false;
      this.currentLoop = null;
      this.backend.stopAllLoops();
    }
  }

  async unlockFromUserGesture(): Promise<boolean> {
    if (this.preference !== 'on') return false;
    this.unlocked = await this.backend.resume();
    return this.unlocked;
  }

  playOneShot(cue: AudioOneShotCue): boolean {
    if (!this.canSchedule()) return false;
    return this.backend.playOneShot(cue);
  }

  syncAmbience(page: ScenePage): AudioAmbienceCue | null {
    const nextLoop = ambienceCueForPage(page);
    if (!this.canSchedule()) return null;
    if (nextLoop === this.currentLoop) return this.currentLoop;
    if (this.currentLoop) this.backend.stopLoop(this.currentLoop);
    this.currentLoop = null;
    if (!nextLoop) return null;
    if (!this.backend.startLoop(nextLoop)) return null;
    this.currentLoop = nextLoop;
    return nextLoop;
  }

  stopAmbience(): void {
    if (!this.currentLoop) return;
    this.backend.stopLoop(this.currentLoop);
    this.currentLoop = null;
  }

  private canSchedule(): boolean {
    return this.preference === 'on' && this.unlocked;
  }
}

interface OscillatorProfile {
  frequencyHz: number;
  durationMs: number;
  gain: number;
  type: OscillatorType;
}

const ONE_SHOT_PROFILES: Record<AudioOneShotCue, OscillatorProfile> = {
  start: { frequencyHz: 196, durationMs: 180, gain: 0.035, type: 'sine' },
  choice: { frequencyHz: 262, durationMs: 90, gain: 0.03, type: 'triangle' },
  danger: { frequencyHz: 98, durationMs: 220, gain: 0.04, type: 'sawtooth' },
  glyph: { frequencyHz: 311, durationMs: 140, gain: 0.025, type: 'square' },
};

const LOOP_PROFILES: Record<AudioAmbienceCue, Omit<OscillatorProfile, 'durationMs'>> = {
  office: { frequencyHz: 62, gain: 0.012, type: 'sine' },
  danger: { frequencyHz: 46, gain: 0.018, type: 'sawtooth' },
};

type AudioContextConstructor = new () => AudioContext;
type GlobalAudioObject = typeof globalThis & { webkitAudioContext?: AudioContextConstructor };

interface LoopNode {
  oscillator: OscillatorNode;
  gain: GainNode;
}

class WebAudioGeneratedBackend implements GeneratedAudioBackend {
  private context: AudioContext | null = null;
  private readonly loops = new Map<AudioAmbienceCue, LoopNode>();

  constructor(private readonly globalObject: GlobalAudioObject) {}

  async resume(): Promise<boolean> {
    const context = this.ensureContext();
    if (!context || context.state === 'closed') return false;
    if (context.state === 'suspended') await context.resume();
    return context.state === 'running';
  }

  playOneShot(cue: AudioOneShotCue): boolean {
    const context = this.runningContext();
    if (!context) return false;
    const profile = ONE_SHOT_PROFILES[cue];
    const now = context.currentTime;
    const stopAt = now + profile.durationMs / 1000;
    const oscillator = context.createOscillator();
    const gain = context.createGain();

    oscillator.type = profile.type;
    oscillator.frequency.setValueAtTime(profile.frequencyHz, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(profile.gain, now + 0.01);
    gain.gain.exponentialRampToValueAtTime(0.0001, stopAt);
    oscillator.connect(gain).connect(context.destination);
    oscillator.start(now);
    oscillator.stop(stopAt);
    return true;
  }

  startLoop(cue: AudioAmbienceCue): boolean {
    const context = this.runningContext();
    if (!context) return false;
    if (this.loops.has(cue)) return true;
    const profile = LOOP_PROFILES[cue];
    const oscillator = context.createOscillator();
    const gain = context.createGain();

    oscillator.type = profile.type;
    oscillator.frequency.setValueAtTime(profile.frequencyHz, context.currentTime);
    gain.gain.setValueAtTime(profile.gain, context.currentTime);
    oscillator.connect(gain).connect(context.destination);
    oscillator.start();
    this.loops.set(cue, { oscillator, gain });
    return true;
  }

  stopLoop(cue: AudioAmbienceCue): void {
    const loop = this.loops.get(cue);
    if (!loop) return;
    this.loops.delete(cue);
    this.stopLoopNode(loop);
  }

  stopAllLoops(): void {
    for (const cue of this.loops.keys()) this.stopLoop(cue);
  }

  private ensureContext(): AudioContext | null {
    if (this.context) return this.context;
    const ContextConstructor = this.globalObject.AudioContext ?? this.globalObject.webkitAudioContext;
    if (!ContextConstructor) return null;
    this.context = new ContextConstructor();
    return this.context;
  }

  private runningContext(): AudioContext | null {
    if (!this.context || this.context.state !== 'running') return null;
    return this.context;
  }

  private stopLoopNode(loop: LoopNode): void {
    try {
      const context = this.context;
      const now = context?.currentTime ?? 0;
      loop.gain.gain.setTargetAtTime(0.0001, now, 0.03);
      loop.oscillator.stop(now + 0.08);
      loop.oscillator.disconnect();
      loop.gain.disconnect();
    } catch {
      // Already-stopped generated nodes are disposable.
    }
  }
}

function isDangerPage(page: ScenePage): boolean {
  return page.status_summary.danger >= DANGER_AUDIO_THRESHOLD;
}
