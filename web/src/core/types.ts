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
  /** 전투 인카운터가 아닌 페이지에는 이 필드 자체가 없다 (Wave 3 Step 1d-2). */
  combat?: CombatSpectatorPage;
}

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-2: combat spectator types (Rust serde 표현과 1:1).
//
// Hard invariant: this module only mirrors escape-core's serialized shape.
// TS renderers built on these types must never recompute damage, hit/miss,
// win/loss, cue derivation, or log-importance filtering — escape-core has
// already decided all of that (정본 13).
// ---------------------------------------------------------------------------

export type CombatSide = 'ally' | 'enemy';
export type CombatSpectatorCue = 'attack' | 'hit' | 'evade' | 'balance_broken' | 'incapacitated';
export type CombatLogImportance = 'routine' | 'important' | 'decisive';
export type CombatConclusionOutcome =
  | 'in_progress'
  | 'ally_victory'
  | 'enemy_victory'
  | 'mutual_defeat'
  | 'stalemate';
export type CombatConclusionReason =
  | 'no_terminal_condition'
  | 'all_enemies_defeated'
  | 'all_allies_defeated'
  | 'both_sides_defeated'
  | 'max_ticks_reached';

/** CombatPosition / CombatFacing — both serialize as { x, y }. */
export interface CombatPoint {
  x: number;
  y: number;
}

export interface CombatSpectatorPiece {
  id: string;
  side: CombatSide;
  position: CombatPoint;
  facing: CombatPoint;
  active: boolean;
  /** serde(default) on the Rust side → may be an empty array. */
  cues: CombatSpectatorCue[];
}

export interface CombatSpectatorFrame {
  tick: number;
  pieces: CombatSpectatorPiece[];
}

export interface CombatSpectatorLogEntry {
  tick: number;
  sequence: number;
  /** Sentence table lives in the renderer (combatLogTemplates.ts), not here. */
  template_id: string;
  importance: CombatLogImportance;
  actor_id: string;
  target_id?: string | null;
  value_hundredths?: number | null;
  effect_id?: string | null;
}

export interface CombatSpectatorView {
  /** Newtype struct on the Rust side → serializes as a bare string. */
  simulation_version: string;
  resolution_fingerprint: string;
  tick_millis: number;
  frames: CombatSpectatorFrame[];
  core_log: CombatSpectatorLogEntry[];
  full_log: CombatSpectatorLogEntry[];
  fingerprint: string;
}

export interface CombatCombatantReport {
  id: string;
  damage_dealt_hundredths: number;
  damage_taken_hundredths: number;
  kills: number;
  incapacitated: boolean;
}

export interface CombatConclusionReport {
  resolution_fingerprint: string;
  outcome: CombatConclusionOutcome;
  reason: CombatConclusionReason;
  decisive_tick: number | null;
  active_allies: number;
  active_enemies: number;
  survivor_ids: string[];
  defeated_ids: string[];
  removed_combat_effect_ids: string[];
  retained_effect_ids: string[];
  duration_millis: number;
  combatants: CombatCombatantReport[];
  /** null이면 피해가 발생하지 않은 것 — "없음" 문구 없이 그 줄 자체를 생략한다. */
  top_damage_dealt_id?: string | null;
  top_damage_taken_id?: string | null;
  fingerprint: string;
}

/** ScenePage.combat — 전투 인카운터가 아니면 필드 자체가 없다. */
export interface CombatSpectatorPage {
  view: CombatSpectatorView;
  /** 전투가 진행 중이면 없다. */
  report?: CombatConclusionReport;
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
