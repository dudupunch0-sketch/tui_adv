# Rust GameCore + Dual Renderer Architecture

## 상태

이 문서는 2026-05-22 기준 활성 렌더러 방향을 고정한다.

현재 체크인된 코드는 Python/Textual, TypeScript mirror core, Rust `escape-core`, Web Storybook renderer, `escape-wasm` JSON boundary, SuperLightTUI 기반 `escape-terminal` snapshot/play renderer를 함께 포함한다. Python/Textual과 TypeScript mirror는 구현 이력과 전환기 parity/fallback scaffold다. 목표 아키텍처는 다음이다.

```text
Rust GameCore
  ├─ Web Storybook + GlyphFX renderer
  │   └─ primary player UX
  └─ SuperLightTUI terminal renderer
      └─ terminal-native fallback / horror edition, not a debug dump
```

짧은 원칙:

```text
Core owns truth. Renderer owns mood.
```

Wire schema의 canonical source는 `docs/dev/Data_Schema.md`다. 이 문서는 crate 책임, renderer 분리, migration 흐름을 설명하고, 실제 JSON/YAML field shape는 `Data_Schema.md`의 `content.bundle.json`, `ScenePage`, `ActionResult`, `EffectCue`, WASM boundary contract를 따른다.

## 절대 잊지 말아야 할 방향

- Web Storybook + GlyphFX를 플레이어가 실제로 보게 될 primary UX로 먼저 만든다.
- Rust terminal 경로는 계속 유지하되, 반드시 SuperLightTUI 기반 renderer로 전환한다.
- terminal renderer는 우선순위와 배포 표면에서는 fallback이지만 품질 면에서 fallback이 아니다. layout, input, snapshot, GlyphFX-style cell effects를 갖춘 terminal-native horror edition이어야 한다.
- 현재 `cargo run -p escape-terminal -- ... --play` 출력은 SuperLightTUI snapshot 기반 content renderer다. 전체 화면 app loop, richer visual card, terminal-native GlyphFX polish는 다음 확장 대상이다.
- 오래된 browser fake-TUI dashboard를 장기 제품 UI로 키우지 않는다. 거기서 얻은 Canvas/pretext 교훈은 Web Storybook/GlyphFX 안으로 흡수한다.
- TypeScript 또는 terminal renderer code에 새 게임 규칙을 추가하지 않는다. 게임 규칙은 Rust core에 둔다.

## Runtime 책임

### `crates/escape-core`

게임 truth만 소유한다.

- content bundle loading and validation
- `GameState` / `PlayerState`
- turn/action availability
- action application
- resource, item, clue, flag, ending, achievement rules
- save/load schema
- `TurnView` / `ScenePage` 같은 renderer-safe semantic view
- `EffectCue`, `VisualCue`, `PressureCue` 같은 semantic presentation cue

절대 import하지 않는다.

- SuperLightTUI or crossterm
- wasm-bindgen or web-sys
- DOM, Canvas, CSS, browser APIs
- terminal color/style objects

### `web/` + `crates/escape-wasm`

Primary player UX다.

- mobile/narrow Storybook layout
- readable dialogue/body/history presentation
- important scene visual cards
- core effect cue를 Canvas/GlyphFX로 해석
- touch/click/keyboard selection
- core save JSON에 기반한 localStorage save surface

Web renderer는 Rust/WASM에서 semantic state를 받고 mood를 렌더링한다. 어떤 action이 가능한지, outcome을 어떻게 적용하는지 직접 다시 계산하지 않는다.

### `crates/escape-terminal`

SuperLightTUI terminal renderer다. 현재 slice는 content `ScenePage`를 SuperLightTUI `TestBackend` snapshot으로 렌더링하고 play loop에서 사용한다.

- Web과 같은 `ScenePage`/semantic view를 소비한다.
- SuperLightTUI layout primitive로 status, visual card, body/dialogue, choices, recent history를 배치한다.
- 다음 확장에서 SuperLightTUI tick/raw-draw capability로 terminal-native GlyphFX를 구현한다.
- `visual_id`를 ASCII/Unicode/ANSI visual card로 매핑한다.
- keyboard input과 headless/snapshot smoke test를 지원한다.
- inline image 지원 없이도 plain WSL/SSH terminal에서 동작한다.

Terminal image는 baseline이 아니다. Kitty/Sixel/iTerm2 inline image support는 나중에 optional로 검토할 수 있지만, 필수 fallback은 SuperLightTUI cell/ASCII/GlyphFX rendering이다.

## 목표 crate 구조

```text
crates/
  escape-core/
    src/
      lib.rs
      content.rs
      state.rs
      turn.rs
      scene_page.rs       # renderer-safe story page contract
      effects.rs          # EffectCue / VisualCue / PressureCue semantics
      save.rs

  escape-terminal/
    src/
      main.rs             # CLI entry, mode selection
      app.rs              # SuperLightTUI run loop
      layout.rs           # SuperLightTUI page layout
      ascii_visuals.rs    # visual_id -> terminal visual card
      glyphfx.rs          # EffectCue -> terminal cell animation
      smoke.rs            # headless/snapshot rendering
      theme.rs            # terminal renderer style constants only
    tests/
      storybook_fallback.rs
      glyphfx_snapshot.rs

  escape-wasm/
    src/
      lib.rs              # JSON-string wasm boundary over escape-core

web/
  src/
    core/
      wasm.ts
      types.ts
    ui/storybook/
      render.ts
      visualCatalog.ts
      history.ts
    effects/
      glyphfx.ts
```

## `ScenePage` contract

`TurnView`는 low-level runner compatibility용으로 남길 수 있다. 하지만 renderer는 더 풍부한 `ScenePage` 형태로 수렴한다.

```text
ScenePage
- mode: encounter | movement | ending
- title
- location
- chapter_label
- status_summary
- body_blocks
- dialogue_entries
- visual
- actions
- blocked_actions
- history_entries
- inventory_summary
- achievement_summary
- pressure_cues
- effect_cues
```

규칙:

- `ScenePage.actions[*].id`는 `apply_action_from_content`가 받을 수 있는 action id여야 한다.
- renderer는 action을 표시할 뿐, eligibility를 다시 계산하지 않는다.
- `ScenePage.visual.id`는 semantic id이며 file path가 아니다.
- renderer가 `visual.id`를 실제 visual output으로 매핑한다.
- effect는 의미를 설명하고, 정확한 pixel/cell은 renderer가 결정한다.

세부 field type, example JSON, fallback visual id policy, action id contract는 `docs/dev/Data_Schema.md`의 `ScenePage JSON contract`와 `Action ID contract`가 기준이다. Architecture 문서 안에서 임의로 field를 늘리지 않는다.

## Content bundle and WASM contract

공통 runtime 입력은 renderer-neutral content bundle로 수렴한다.

```text
src/tui_adv/data/*.yaml
        |
        v
scripts/export_web_data.py --write --bundle ...
        |
        +--> crates/escape-core/fixtures/content/content.bundle.json
        +--> web/src/data/generated/content.bundle.json
```

`escape-wasm`의 초기 boundary는 JSON string API다.

```text
new_game_json(seed, content_bundle_json) -> GameState JSON
scene_page_json(state_json, content_bundle_json) -> ScenePage JSON
apply_action_json(state_json, content_bundle_json, action_id) -> ActionResult JSON
save_state_json(state_json) -> SaveEnvelope JSON
load_state_json(save_json) -> GameState JSON
```

Web은 이 boundary를 통해 Rust GameCore를 호출한다. TypeScript mirror core는 전환기 parity oracle일 뿐, 새 gameplay rule을 추가하는 장소가 아니다.

Fallback/error policy:

- Bundle validation 실패, WASM init 실패, schema version mismatch는 Web error panel로 보여준다.
- Unknown `visual_id`는 blank page가 아니라 safe placeholder로 표시한다.
- Unknown/stale action id는 state를 변경하지 않고 사용자-facing error로 처리한다.
- Save/load 오류는 renderer-specific traceback이 아니라 core error message를 보여준다.

## SuperLightTUI 구현 메모

terminal renderer는 SuperLightTUI를 사용한다. 현재 slice는 headless/snapshot rendering으로 content `ScenePage`를 표시하며, ad-hoc `println!` layout으로 만족하지 않는다.

목표 사용 지점:

- `slt::run_with(...)` 또는 현재 SuperLightTUI의 equivalent app loop
- `RunConfig::tick_rate(...)` / `max_fps(60)` 또는 equivalent frame/tick control
- page region을 위한 flexbox-style layout
- normal widget tree로 부족한 GlyphFX를 위한 raw draw / cell-grid access
- `ui.tick()` 또는 equivalent animation tick signal
- 가능한 경우 snapshot smoke용 test/headless backend

첫 terminal slice:

1. `printer_anomaly`의 `ScenePage` 하나를 렌더링한다.
2. status, ASCII/Unicode visual card, dialogue/body, choices, recent history를 표시한다.
3. `EffectCue::GlyphAnomaly`를 해석하되 stable terms를 보존한다.
4. terminal page가 debug dump가 아님을 증명하는 headless snapshot test를 추가한다.

## Web Storybook 구현 메모

Web은 primary UX지만 반드시 core output을 소비해야 한다. 현재 `web/src/core/wasmRuntime.ts`가 Web용 generated content bundle을 `escape-wasm` JSON-string function에 전달하고, 반환된 Rust `ScenePage`를 Web Storybook renderer에 넘긴다. generated wasm package가 없는 개발 환경에서는 legacy TypeScript mirror fallback을 사용한다.

1. 완료: `escape-wasm` JSON-string function을 추가한다.
2. 완료: Web/Rust 양쪽 generated content bundle을 export/check한다.
3. 완료: Web에서 generated content bundle을 로드하고 wasm boundary를 호출하는 runtime wrapper를 추가한다.
4. 현재: `web/src/ui/storybook/render.ts`가 `ScenePage` shape을 렌더링한다.
5. 현재: `visualCatalog.ts`가 `visual.id`를 매핑한다.
6. 현재: `web/src/effects/glyphfx.ts`가 effect cue를 해석한다.
7. 기존 fake-TUI/TypeScript mirror code는 generated wasm package가 없는 환경의 fallback과 legacy/parity reference로만 유지한다.

## Design gate before code slices

각 구현 slice는 다음 설계 gate를 만족해야 시작한다.

1. Core slice: `Data_Schema.md`에 `ScenePage`, action id, `EffectCue`, save/error contract가 기록되어 있어야 한다.
2. Content slice: YAML `presentation` metadata가 semantic field만 담고 private-only field를 포함하지 않는다는 검증 기준이 있어야 한다.
3. Web slice: Storybook region(`visual`, `body`, `choices`, `history`, `status`)과 WASM action flow가 문서화되어 있어야 한다.
4. Terminal slice: SuperLightTUI가 `escape-terminal`에만 들어가고, `escape-core`는 renderer dependency를 갖지 않는다는 확인이 있어야 한다.
5. Parity slice: Web/terminal rendered string이 아니라 `ScenePage`와 `GameState` semantic fields를 비교해야 한다.

## 검증 체크리스트

Core:

```bash
cargo fmt --check
cargo test -p escape-core
```

Terminal:

```bash
cargo test -p escape-terminal
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --play
```

Terminal acceptance:

- `escape-terminal`이 renderer slice에서 SuperLightTUI를 사용한다.
- 화면이 debug dump가 아니라 storybook-like region을 가진다.
- 알려진 `visual_id`가 terminal visual card로 렌더링된다.
- glyph anomaly가 stable clue terms를 읽을 수 있게 남긴다.
- plain WSL/SSH terminal에서도 동작한다.

Web:

```bash
cd web
npm test
npm run build
npm run dev -- --host 127.0.0.1 --port 8765
```

Web acceptance:

- 첫 화면은 fake-TUI dashboard가 아니라 Storybook이다.
- visual region, dialogue/body, choices, history가 보인다.
- 해당 slice가 구현된 뒤에는 action이 Rust/WASM을 통해 실행된다.
- reduced-motion/fallback에서도 clue text가 읽힌다.

Content/export:

```bash
python scripts/export_web_data.py --check --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json
```

Secret safety:

- public JSON/bundle에 `final_hint`, `actual_ip_address`, `office_location`, `treasure_location`이 없다.
- Web asset에 실제 사무실 이미지, 사람 이름, 좌표, private hint text가 없다.
