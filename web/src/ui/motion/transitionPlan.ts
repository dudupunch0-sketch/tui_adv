import type { ScenePage } from '../../core/types';
import type { EffectiveMotionMode } from '../settings/playerSettings';

export type TransitionName = 'start-fade' | 'paper-slide' | 'ink-pulse' | 'ending-fade' | 'danger-glitch' | 'paper-fade';

export interface TransitionActionContext {
  id: string;
  kind: string;
}

export interface TransitionPlan {
  name: TransitionName;
  durationMs: number;
  dangerOverlay: boolean;
}

const DURATION_BY_NAME: Record<TransitionName, number> = {
  'start-fade': 220,
  'paper-slide': 240,
  'ink-pulse': 220,
  'ending-fade': 320,
  'danger-glitch': 320,
  'paper-fade': 180,
};

export function planTransition(
  previousPage: ScenePage | null,
  nextPage: ScenePage,
  action: TransitionActionContext | null,
  motionMode: EffectiveMotionMode,
): TransitionPlan {
  const name = chooseTransitionName(previousPage, nextPage, action);
  const durationMs = motionMode === 'normal' ? DURATION_BY_NAME[name] : 0;

  return {
    name,
    durationMs,
    dangerOverlay: isDangerTransition(nextPage),
  };
}

function chooseTransitionName(
  previousPage: ScenePage | null,
  nextPage: ScenePage,
  action: TransitionActionContext | null,
): TransitionName {
  if (isDangerTransition(nextPage)) return 'danger-glitch';
  if (!previousPage || action?.kind === 'start') return 'start-fade';
  if (nextPage.mode === 'ending') return 'ending-fade';
  if (!action) return 'paper-fade';
  if (action.kind === 'move' || previousPage.location.id !== nextPage.location.id) return 'paper-slide';
  if (nextPage.mode === 'encounter') return 'ink-pulse';
  return 'paper-fade';
}

function isDangerTransition(page: ScenePage): boolean {
  return page.status_summary.danger >= 70 || page.effect_cues.length > 0;
}
