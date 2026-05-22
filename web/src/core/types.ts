export type SceneMode = 'encounter' | 'movement' | 'ending';

export interface SceneLocation {
  id: string;
  name: string;
  description: string;
}

export interface ResourceStatus {
  id: string;
  label: string;
  band: string;
  text: string;
  value: number;
}

export interface StatusSummary {
  turn: number;
  danger: number;
  resources: ResourceStatus[];
  warnings: string[];
}

export interface BodyBlock {
  kind: string;
  text: string;
  source_id: string | null;
}

export interface DialogueEntry {
  speaker: string;
  text: string;
  source_id: string | null;
}

export interface SceneVisual {
  id: string;
  kind: string;
  alt: string;
  source_id: string | null;
}

export interface SceneAction {
  id: string;
  label: string;
  kind: string;
  cost_text: string | null;
}

export interface SceneBlockedAction extends SceneAction {
  reasons: string[];
}

export interface HistoryEntry {
  kind: string;
  text: string;
  source_id: string | null;
}

export interface InventorySummary {
  items: string[];
  overflow_count: number;
}

export interface AchievementSummary {
  unlocked: string[];
  newly_unlocked: string[];
}

export interface PressureCue {
  kind: string;
  severity: string;
  message: string;
  resource_id: string;
}

export interface SceneEffectCue {
  kind: string;
  source: string;
  intensity: number;
  stable_terms: string[];
  distortion: string;
  duration_hint_ms: number | null;
  fallback_text: string | null;
}

export interface ScenePage {
  mode: SceneMode;
  title: string;
  location: SceneLocation;
  chapter_label: string;
  status_summary: StatusSummary;
  body_blocks: BodyBlock[];
  dialogue_entries: DialogueEntry[];
  visual: SceneVisual;
  actions: SceneAction[];
  blocked_actions: SceneBlockedAction[];
  history_entries: HistoryEntry[];
  inventory_summary: InventorySummary;
  achievement_summary: AchievementSummary;
  pressure_cues: PressureCue[];
  effect_cues: SceneEffectCue[];
}
