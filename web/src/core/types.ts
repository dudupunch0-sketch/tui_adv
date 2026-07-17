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

export interface ActionCheckInfo {
  ability_id: string;
  ability_label: string;
  success_percent: number;
}

export interface SceneAction {
  id: string;
  label: string;
  kind: string;
  cost_text: string | null;
  check?: ActionCheckInfo;
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

export interface AbilityStatus {
  id: string;
  label: string;
  value: number;
}

export interface CharacterSummary {
  name: string;
  title_label?: string;
  title_description?: string;
  stat_points?: number;
  abilities: AbilityStatus[];
}

export interface InventoryDetail {
  id: string;
  name: string;
  description: string;
  item_type: string;
  usable: boolean;
  reveal_immediate?: boolean;
}

export interface RewardEntry {
  id: string;
  name: string;
  concept?: string;
  description?: string;
  effect_text?: string;
  rarity?: string;
  category?: string;
  reveal_immediate?: boolean;
}

export interface InsightStatus {
  id: string;
  name: string;
  description: string;
  effect_text: string;
  reveal_immediate?: boolean;
}

export interface ProgressionStatus {
  experience: number;
  target: number;
  label: string;
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
  inventory_details?: InventoryDetail[];
  achievement_summary: AchievementSummary;
  pressure_cues: PressureCue[];
  effect_cues: SceneEffectCue[];
  character_summary?: CharacterSummary;
  progression?: ProgressionStatus;
  content_labels?: ContentLabels;
  check_result?: CheckResolution;
  insights?: InsightStatus[];
  skills?: RewardEntry[];
  titles?: RewardEntry[];
  /** Ordered event presentation. Older bundles may omit this and use the legacy fields above. */
  content_stream?: SceneContentItem[];
}

export type SceneContentKind =
  | 'narration'
  | 'dialogue'
  | 'illustration'
  | 'document'
  | 'system'
  | 'cheongirok'
  | 'result_summary'
  | 'choice'
  | 'continue';

export interface SceneContentItem {
  kind: SceneContentKind | string;
  stage_id?: string | null;
  text?: string | null;
  speaker?: string | null;
  visual_id?: string | null;
  alt?: string | null;
  branch?: 'success' | 'failure' | null;
  placeholder: boolean;
  actions?: SceneAction[];
}

export interface LabeledId {
  id: string;
  label: string;
}

export interface ContentLabels {
  items: LabeledId[];
  achievements: LabeledId[];
}

export interface CheckResolution {
  ability_id: string;
  ability_label: string;
  dice: [number, number];
  ability_value: number;
  insight_bonus?: number;
  difficulty: number;
  total: number;
  success: boolean;
}

/** Per-action presentation data returned by the Rust/WASM boundary. */
export interface ActionResultDelta {
  logs: string[];
  newly_unlocked_achievements: string[];
}
