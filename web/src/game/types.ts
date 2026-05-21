export type ResourceName = 'health' | 'sanity' | 'battery' | 'hunger' | 'thirst';

export type ResourceDelta = Partial<Record<ResourceName, number>>;

export interface PlayerState {
  health: number;
  sanity: number;
  battery: number;
  hunger: number;
  thirst: number;
  abilities: Record<string, number>;
}

export interface GameState {
  seed: number;
  turn: number;
  locationId: string;
  disasterType: string;
  danger: number;
  player: PlayerState;
  inventory: string[];
  clues: string[];
  flags: string[];
  seenEncounters: string[];
  unlockedAchievements: string[];
  log: string[];
}

export interface LocationData {
  id: string;
  name: string;
  description: string;
  connections: string[];
  tags?: string[];
  danger?: number;
}

export interface Conditions {
  locations?: string[];
  disaster_types?: string[];
  required_items?: string[];
  required_clues?: string[];
  required_flags?: string[];
  forbidden_flags?: string[];
  min_resources?: Partial<Record<ResourceName, number>>;
  max_resources?: Partial<Record<ResourceName, number>>;
  min_abilities?: Record<string, number>;
}

export interface Outcome {
  resources?: ResourceDelta;
  health?: number;
  sanity?: number;
  battery?: number;
  hunger?: number;
  thirst?: number;
  add_items?: string[];
  remove_items?: string[];
  add_clues?: string[];
  add_flags?: string[];
  remove_flags?: string[];
  destination_id?: string;
  danger?: number;
  log?: string;
}

export interface AbilityCheckData {
  ability: string;
  difficulty: number;
  success: Outcome;
  failure: Outcome;
}

export interface ChoiceData {
  id: string;
  label: string;
  conditions?: Conditions;
  cost?: ResourceDelta;
  outcome?: Outcome;
  check?: AbilityCheckData;
}

export interface EncounterData {
  id: string;
  title: string;
  body: string;
  conditions?: Conditions;
  choices: ChoiceData[];
  repeatable?: boolean;
  weight?: number;
}

export interface EndingData {
  id: string;
  name: string;
  kind: string;
  priority: number;
  conditions?: Conditions;
  local_hint_id?: string;
  text: string;
}

export interface PublicSecret {
  id: string;
  title: string;
  unlock_flags?: string[];
  public_hint_steps: string[];
  puzzle_prompt?: string;
  placeholder_ip_address?: string;
  final_hint_policy?: string;
  reward_text?: string;
}

export interface ItemData {
  id: string;
  name: string;
  description: string;
  type: string;
  tags?: string[];
  usable?: boolean;
  use_effects?: ResourceDelta;
  use_log?: string;
}

export interface AchievementData {
  id: string;
  name: string;
  description: string;
  conditions?: Conditions;
  hidden?: boolean;
}

export interface GameAction {
  id: string;
  label: string;
  kind: 'choice' | 'move' | 'item';
  targetId?: string;
}

export interface GameTurn {
  state: GameState;
  location: LocationData;
  encounter: EncounterData | null;
  ending: ResolvedEnding | null;
  actions: GameAction[];
}

export interface ActionResult {
  action: GameAction;
  state: GameState;
  turn: GameTurn;
  unlockedAchievements: AchievementData[];
}

export interface ResolvedEnding extends EndingData {
  publicSecret?: PublicSecret;
}
