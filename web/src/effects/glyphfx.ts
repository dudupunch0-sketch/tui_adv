import type { SceneEffectCue } from '../core/types';

export function glyphFxFallbackText(cue: SceneEffectCue): string {
  if (cue.fallback_text) return cue.fallback_text;
  if (cue.stable_terms.length) return `안정화된 단서: ${cue.stable_terms.join(' · ')}`;
  return `${cue.kind} effect cue`;
}

export function glyphFxStableTerms(cue: SceneEffectCue): string[] {
  return cue.stable_terms;
}
