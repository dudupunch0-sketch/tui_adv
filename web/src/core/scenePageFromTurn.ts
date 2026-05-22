import type { SceneAction, SceneEffectCue, SceneMode, ScenePage, PressureCue, ResourceStatus } from './types';
import type { EncounterPresentation, GameAction, GameTurn, PlayerState } from '../game/types';
import { publicSecretSummary } from '../security/publicSecretGuard';

export function scenePageFromLegacyTurn(turn: GameTurn): ScenePage {
  const sourceId = turn.encounter?.id ?? turn.ending?.id ?? turn.location.id;
  const presentation = turn.encounter?.presentation;
  const mode = sceneMode(turn);
  const title = turn.encounter?.title ?? turn.ending?.name ?? turn.location.name;
  const bodyText = turn.encounter?.body ?? turn.ending?.text ?? turn.location.description;
  const bodyBlocks = [
    {
      kind: turn.ending ? 'system' : 'narration',
      text: bodyText,
      source_id: sourceId,
    },
  ];
  if (turn.ending?.publicSecret) {
    bodyBlocks.push({
      kind: 'clue',
      text: publicSecretSummary(turn.ending.publicSecret),
      source_id: turn.ending.publicSecret.id,
    });
  }

  return {
    mode,
    title,
    location: {
      id: turn.location.id,
      name: turn.location.name,
      description: turn.location.description,
    },
    chapter_label: `격리 ${turn.state.turn}턴`,
    status_summary: statusSummary(turn.state.player, turn.state.turn, turn.state.danger),
    body_blocks: bodyBlocks,
    dialogue_entries: presentation?.speaker
      ? [
          {
            speaker: presentation.speaker,
            text: bodyText,
            source_id: sourceId,
          },
        ]
      : [],
    visual: {
      id: visualId(mode, sourceId, presentation),
      kind: presentation?.layout ?? mode,
      alt: title,
      source_id: sourceId,
    },
    actions: turn.actions.map(sceneAction),
    blocked_actions: [],
    history_entries: turn.state.log.map((entry) => ({ kind: 'action', text: entry, source_id: null })),
    inventory_summary: { items: turn.state.inventory, overflow_count: 0 },
    achievement_summary: { unlocked: turn.state.unlockedAchievements, newly_unlocked: [] },
    pressure_cues: pressureCues(turn.state.player),
    effect_cues: effectCues(presentation),
  };
}

function sceneMode(turn: GameTurn): SceneMode {
  if (turn.ending) return 'ending';
  if (turn.encounter) return 'encounter';
  return 'movement';
}

function visualId(mode: SceneMode, sourceId: string, presentation: EncounterPresentation | undefined): string {
  if (presentation?.visual_id) return presentation.visual_id;
  if (mode === 'ending') return `ending:${sourceId}`;
  if (mode === 'encounter') return `encounter:${sourceId}`;
  return `location:${sourceId}`;
}

function statusSummary(player: PlayerState, turn: number, danger: number): ScenePage['status_summary'] {
  const pressure = pressureCues(player);
  return {
    turn,
    danger,
    resources: [
      healthStatus(player.health),
      sanityStatus(player.sanity),
      batteryStatus(player.battery),
    ],
    warnings: pressure.map((cue) => cue.message),
  };
}

function healthStatus(value: number): ResourceStatus {
  if (value <= 20) return resourceStatus('health', '신체 반응', 'critical', '붕괴 직전', value);
  if (value <= 50) return resourceStatus('health', '신체 반응', 'warning', '불안정', value);
  return resourceStatus('health', '신체 반응', 'normal', '정상 범위', value);
}

function sanityStatus(value: number): ResourceStatus {
  if (value <= 20) return resourceStatus('sanity', '집중도', 'critical', '붕괴 직전', value);
  if (value <= 30) return resourceStatus('sanity', '집중도', 'warning', '불안정', value);
  return resourceStatus('sanity', '집중도', 'normal', '안정', value);
}

function batteryStatus(value: number): ResourceStatus {
  if (value <= 10) return resourceStatus('battery', '단말기 전원', 'critical', `${value}%`, value);
  if (value <= 20) return resourceStatus('battery', '단말기 전원', 'warning', `${value}%`, value);
  return resourceStatus('battery', '단말기 전원', 'normal', `${value}%`, value);
}

function resourceStatus(id: string, label: string, band: string, text: string, value: number): ResourceStatus {
  return { id, label, band, text, value };
}

function pressureCues(player: PlayerState): PressureCue[] {
  const cues: PressureCue[] = [];
  if (player.sanity <= 30) {
    cues.push({
      kind: 'low_sanity',
      severity: player.sanity <= 20 ? 'critical' : 'warning',
      message: '집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.',
      resource_id: 'sanity',
    });
  }
  if (player.battery <= 20) {
    cues.push({
      kind: 'low_battery',
      severity: player.battery <= 10 ? 'critical' : 'warning',
      message: '단말기 전원이 낮습니다. 전력 행동이 제한될 수 있습니다.',
      resource_id: 'battery',
    });
  }
  return cues;
}

function sceneAction(action: GameAction): SceneAction {
  return {
    id: action.id,
    label: action.label,
    kind: action.kind === 'item' ? 'use' : action.kind,
    cost_text: null,
  };
}

function effectCues(presentation: EncounterPresentation | undefined): SceneEffectCue[] {
  return (presentation?.effect_cues ?? []).map((cue) => ({
    kind: cue.kind,
    source: cue.source,
    intensity: cue.intensity,
    stable_terms: cue.stable_terms,
    distortion: cue.distortion,
    duration_hint_ms: cue.duration_hint_ms ?? null,
    fallback_text: cue.fallback_text ?? null,
  }));
}
