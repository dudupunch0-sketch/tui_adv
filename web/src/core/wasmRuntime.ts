import contentBundle from '../data/generated/content.bundle.json';
import type { ScenePage } from './types';

export interface EscapeWasmBindings {
  new_game_json(seed: bigint, contentBundleJson: string): string;
  scene_page_json(stateJson: string, contentBundleJson: string): string;
  apply_action_json(stateJson: string, contentBundleJson: string, actionId: string): string;
}

export interface EscapeWasmActionResult {
  encounter_id: string;
  action_id: string;
  state: unknown;
  logs: string[];
  effect_cues: unknown[];
  newly_unlocked_achievements: string[];
}

interface EscapeWasmModule extends Partial<EscapeWasmBindings> {
  default?: () => Promise<unknown>;
}

const DEFAULT_SEED = 123;
const DEFAULT_WASM_MODULE_PATH = new URL(/* @vite-ignore */ './wasm-pkg/escape_wasm.js', import.meta.url).toString();
const DEFAULT_CONTENT_BUNDLE_JSON = JSON.stringify(contentBundle);

export class EscapeWasmRuntime {
  private currentStateJson: string;

  constructor(
    private readonly bindings: EscapeWasmBindings,
    private readonly contentBundleJson: string = DEFAULT_CONTENT_BUNDLE_JSON,
    initialStateJson?: string,
    seed: number = DEFAULT_SEED,
  ) {
    this.currentStateJson = initialStateJson ?? bindings.new_game_json(BigInt(seed), contentBundleJson);
    this.parseScenePage(this.currentStateJson);
  }

  get stateJson(): string {
    return this.currentStateJson;
  }

  newGame(seed: number = DEFAULT_SEED): ScenePage {
    const nextStateJson = this.bindings.new_game_json(BigInt(seed), this.contentBundleJson);
    const nextPage = this.parseScenePage(nextStateJson);
    this.currentStateJson = nextStateJson;
    return nextPage;
  }

  scenePage(): ScenePage {
    return this.parseScenePage(this.currentStateJson);
  }

  applyAction(actionId: string): EscapeWasmActionResult {
    const result = JSON.parse(
      this.bindings.apply_action_json(this.currentStateJson, this.contentBundleJson, actionId),
    ) as EscapeWasmActionResult;
    const nextStateJson = JSON.stringify(result.state);
    this.parseScenePage(nextStateJson);
    this.currentStateJson = nextStateJson;
    return result;
  }

  private parseScenePage(stateJson: string): ScenePage {
    return JSON.parse(this.bindings.scene_page_json(stateJson, this.contentBundleJson)) as ScenePage;
  }
}

export async function loadEscapeWasmBindings(
  modulePath: string = DEFAULT_WASM_MODULE_PATH,
): Promise<EscapeWasmBindings> {
  const wasmModule = (await import(/* @vite-ignore */ modulePath)) as EscapeWasmModule;
  if (typeof wasmModule.default === 'function') {
    await wasmModule.default();
  }
  if (
    typeof wasmModule.new_game_json !== 'function' ||
    typeof wasmModule.scene_page_json !== 'function' ||
    typeof wasmModule.apply_action_json !== 'function'
  ) {
    throw new Error('escape-wasm module is missing JSON boundary functions');
  }
  return {
    new_game_json: wasmModule.new_game_json.bind(wasmModule),
    scene_page_json: wasmModule.scene_page_json.bind(wasmModule),
    apply_action_json: wasmModule.apply_action_json.bind(wasmModule),
  };
}

export async function createEscapeWasmRuntime(options: {
  bindings?: EscapeWasmBindings;
  contentBundleJson?: string;
  initialStateJson?: string;
  seed?: number;
} = {}): Promise<EscapeWasmRuntime> {
  const bindings = options.bindings ?? (await loadEscapeWasmBindings());
  return new EscapeWasmRuntime(
    bindings,
    options.contentBundleJson ?? DEFAULT_CONTENT_BUNDLE_JSON,
    options.initialStateJson,
    options.seed ?? DEFAULT_SEED,
  );
}
