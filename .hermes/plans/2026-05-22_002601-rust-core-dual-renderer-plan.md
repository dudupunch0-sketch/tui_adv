# Rust GameCore + Dual Renderer 전환 계획

작성 시각: 2026-05-22 00:26:01
대상 repo: `/home/dudupunch0/tui_adv`
현재 기준: `main` = `origin/main` = `13b96fc`

## 1. 목표

현재 `escape from the office`를 다음 방향으로 재정렬한다.

```text
하나의 공통 Rust GameCore
  + 1번: SuperLightTUI 기반 terminal renderer
  + 3번: Web Storybook + GlyphFX renderer
```

핵심 결정:

- 1번 terminal 방향은 살린다.
  - 단, 기존 Python/Textual 화면을 최종 목표로 키우기보다, SuperLightTUI 기반 terminal-native GlyphFX 가능성을 검토하고 점진 전환한다.
- 3번 Web Storybook + GlyphFX 방향을 플레이어용 메인 UX 후보로 승격한다.
- 2번 browser fake-TUI dashboard + Canvas 방향은 장기 제품 방향에서 제외한다.
  - 다만 이미 만든 Canvas/Pretext-like 효과는 Web Storybook 또는 terminal GlyphFX 실험의 참고/재료로 보존한다.
- 게임 규칙은 renderer마다 재구현하지 않는다.
  - `Core owns truth. Renderer owns mood.`

## 2. 현재 맥락

### 2.1 현재 구현 상태

`README.md` 기준 현재 repo는 다음 상태다.

- Python/Textual TUI 유지
- TypeScript mirror core 기반 browser fake-TUI parity 확장 완료
- YAML 공개 콘텐츠 로더/검증
- YAML -> browser JSON export
- Vite browser shell
- localStorage save
- 복합기 현실 연결 Pretext/Canvas 장면
- 대표 루트 parity 테스트
- 아이템/업적/능력치/압박 상태 UI
- 현실 연결 local-only secret 경계 문서화

### 2.2 현재 문서의 오래된 전제

`docs/dev/Architecture.md`, `docs/dev/Development_Plan.md`는 아직 다음 전제를 중심으로 한다.

- 언어: Python 3.x + TypeScript
- TUI: Textual 기본
- Browser: Vite + TypeScript mirror core
- 콘텐츠: YAML, browser는 생성 JSON

새 방향에서는 이 전제를 바로 삭제하기보다 다음처럼 구분해야 한다.

- 현재 구현: Python/Textual + TypeScript mirror
- 목표 구조: Rust GameCore + SuperLightTUI terminal + Web/WASM Storybook
- 전환 기간: Python/TS 구현은 parity oracle 및 fallback으로 유지

### 2.3 Storybook/GlyphFX 문서 상태

`docs/design/TUI_Storybook_GlyphFX_Concept.md`는 이미 다음 핵심을 정의한다.

- 실제 터미널 조작 게임이 아니다.
- 텍스트 중심의 스토리북 화면이 기본이다.
- 특정 사건에서만 글자/문단/선택지/그림 일부가 살아 움직인다.
- `cd`, `ls`, `grep` 같은 shell command는 기본 조작이 아니다.
- 기본 행동은 읽기, 선택하기, 조사하기, 대화하기, 기억하기, 숨기기, 사용하기, 기다리기다.

이번 전환에서는 이 문서를 `후보 컨셉`에서 `Web primary UX 후보`로 승격하는 문서 정리가 필요하다.

### 2.4 SuperLightTUI 검토 요약

확인한 SuperLightTUI 특성:

- Rust immediate-mode TUI framework
- `slt::run` / `run_with` 기반 closure UI
- flexbox layout
- double-buffer diff rendering
- `RunConfig::tick_rate(Duration::from_millis(16))`, `max_fps(60)` 등 60fps 지향 설정
- animation primitive: Tween, Spring, Keyframes, Sequence, Stagger
- raw draw: `container().draw(...)`로 terminal buffer 직접 접근
- `demo_pretext`: terminal에서 text reflow around cursor 예제
- `demo_fire`: DOOM fire effect 예제
- backend/test 구조: `Backend`, `AppState`, `TestBackend`
- MIT license

따라서 1번 terminal 방향에서 GlyphFX와 유사한 terminal-native 효과를 실험할 수 있다.

## 3. 설계 원칙

### 3.1 GameCore는 renderer를 모른다

`escape-core`는 다음을 import하지 않아야 한다.

- SuperLightTUI / crossterm
- wasm-bindgen / web-sys
- DOM / Canvas
- CSS/style object
- terminal color/style object

Core는 다음만 책임진다.

- 상태 모델
- 콘텐츠 모델
- 조건 검사
- action 생성
- action 적용
- 자원/압박/엔딩/업적 판정
- 저장/로드 schema
- public/private secret boundary 관련 정책 데이터
- effect cue / presentation cue의 의미론적 메타데이터

### 3.2 Renderer는 mood와 interaction만 책임진다

Terminal renderer:

- SuperLightTUI layout
- terminal-native GlyphFX
- keyboard/mouse input
- terminal save/load UX
- terminal smoke/snapshot

Web renderer:

- 모바일 세로형 Storybook layout
- DOM/Canvas GlyphFX
- 터치/버튼 input
- localStorage save/load
- browser-only animation polish

### 3.3 Core API는 작고 안정적이어야 한다

권장 public contract:

```text
new_game(seed, options, content_bundle) -> GameState
turn_view(state, content_bundle) -> TurnView
apply_action(state, action_id, content_bundle) -> ActionResult
save_state(state) -> SaveEnvelope
load_state(save_envelope) -> GameState
validate_content(content_bundle) -> ValidationReport
```

Renderer가 직접 인카운터 조건을 계산하지 않게 한다.
Renderer는 `TurnView.actions`에 나온 action만 표시/실행한다.

### 3.4 Effect cue는 renderer 공통 언어다

Core가 “어떤 연출이 의미상 발생해야 하는지”를 알려주고, 실제 표현은 renderer가 정한다.

예시:

```json
{
  "kind": "glyph_anomaly",
  "source": "copier_output",
  "intensity": 0.72,
  "stable_terms": ["비상계단", "토너", "접힌 방향"],
  "distortion": "reflow_then_stabilize"
}
```

Terminal renderer 해석:

- stable_terms는 고정된 초록/호박색 글자로 유지
- 나머지는 `▒`, `░`, 색상 떨림, 위치 흔들림
- terminal cell grid 기반 reflow/fragmentation

Web renderer 해석:

- stable_terms는 DOM/Canvas overlay에서 안정 위치로 고정
- 나머지는 GlyphFX particle/reflow/overlay 처리
- 모바일 스토리북 본문은 최종적으로 읽을 수 있게 복구

### 3.5 Content authoring은 YAML을 유지한다

사람이 쓰는 원본 콘텐츠는 당분간 YAML을 유지한다.
다만 runtime core 입력은 renderer 공통성을 위해 generated JSON bundle로 통일하는 것을 우선 검토한다.

권장 흐름:

```text
src/tui_adv/data/*.yaml
        |
        v
scripts/export_content_bundle.py 또는 기존 export_web_data.py 확장
        |
        v
generated/content.bundle.json
        |
        +--> escape-core native load
        +--> escape-wasm browser load
        +--> validation/golden tests
```

이렇게 하면 browser/WASM에서도 파일 시스템 의존 없이 동일 content bundle을 사용할 수 있다.

## 4. 제안 repo 구조

장기 목표 구조:

```text
Cargo.toml                         # Rust workspace
crates/
  escape-core/
    Cargo.toml
    src/
      lib.rs
      state.rs
      player.rs
      content.rs
      conditions.rs
      outcomes.rs
      actions.rs
      turn.rs
      endings.rs
      achievements.rs
      save.rs
      rng.rs
      effects.rs                  # EffectCue / GlyphCue 의미론
      validation.rs
    tests/
      route_parity.rs
      content_validation.rs
      save_roundtrip.rs
      secret_safety.rs

  escape-terminal/
    Cargo.toml
    src/
      main.rs
      app.rs
      layout.rs
      glyphfx.rs                  # terminal-native GlyphFX
      smoke.rs
      theme.rs
    tests/
      snapshot.rs

  escape-wasm/
    Cargo.toml
    src/
      lib.rs                      # wasm-bindgen wrapper over escape-core

web/
  src/
    main.ts
    core/
      wasm.ts                     # escape-wasm loader
      types.ts                    # generated/handwritten TS types
    ui/
      storybook/
        render.ts
        controls.ts
        layout.ts
    effects/
      glyphfx.ts
      printerFlow.ts              # 필요 시 storybook effect로 흡수
    data/
      generated/
        content.bundle.json

src/tui_adv/                      # legacy Python implementation during migration
web/src/game/                     # legacy TS mirror core during migration
```

주의:

- `escape-core`는 `escape-terminal`과 `escape-wasm`에 의존하지 않는다.
- `escape-terminal`과 `escape-wasm`은 모두 `escape-core`에 의존한다.
- 기존 Python/TS core는 migration 기간 동안 삭제하지 않고 parity oracle로 둔다.

## 5. 단계별 계획

## Phase 0 — 문서 refactor 먼저

목표:

- 새 방향을 tracked docs에 반영한다.
- 구현 전에 “무엇을 버리고 무엇을 남길지” 명확히 한다.

작업 후보:

1. 새 아키텍처 문서 작성
   - `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`
   - Rust GameCore + dual renderer 구조 설명
   - renderer 책임과 core 책임 분리
   - effect cue contract 정의

2. 기존 아키텍처 문서 갱신
   - `docs/dev/Architecture.md`
   - 현재 구조와 목표 구조를 분리해서 표시
   - Python/Textual + TS mirror를 “현재/legacy”로 명명
   - Rust core migration 경로 추가

3. 개발 계획 갱신
   - `docs/dev/Development_Plan.md`
   - 기존 Phase 0-10은 “초기 Python/Textual 계획 및 완료 이력”으로 정리
   - 새 Phase 11 또는 “Rust Core Migration” 섹션 추가

4. 데이터 스키마 문서 갱신
   - `docs/dev/Data_Schema.md`
   - YAML authoring schema와 generated content bundle schema 분리
   - `TurnView`, `ActionResult`, `EffectCue`, `SaveEnvelope` schema 초안 추가

5. Storybook/GlyphFX 문서 상태 갱신
   - `docs/design/TUI_Storybook_GlyphFX_Concept.md`
   - “후보”에서 “Web primary UX candidate / adopted direction pending spike”로 상태 명확화
   - terminal-native GlyphFX와 Web GlyphFX의 차이 기록

6. README 갱신은 Phase 0 후반 또는 Phase 1 이후
   - 너무 일찍 README를 목표 구조로 바꾸면 실제 실행 명령과 어긋날 수 있음
   - 우선 “계획 중인 target architecture”로만 짧게 연결

7. idea_box 처리
   - `idea_box/inbox/2026-05-21-tui-storybook-glyphfx-concept-v2.md`는 단순히 읽었다고 done 처리하지 않는다.
   - 위 문서들이 실제로 갱신되어 이 컨셉이 설계에 병합되면 그때 `used_by`와 `done_at`을 기록한다.

검증:

```bash
git diff --check
python scripts/export_web_data.py --check
PYTHONPATH=src python -m pytest tests -q
cd web && npm test && npm run build
```

Phase 0에서는 코드 구현 없이 문서 정렬만 하는 것이 좋다.

## Phase 1 — Rust workspace skeleton + core contract + executable smoke

목표:

- Rust workspace를 만들되, 처음부터 전체 게임을 포팅하지 않는다.
- `복합기가 혼자 출력한다` 장면 하나로 core contract를 검증한다.
- 실제로 실행 가능한 최소 headless terminal 실행 파일을 만든다.
  - 최종 산출물 후보: `target/release/escape-terminal`
  - smoke 명령: `cargo run -p escape-terminal -- --scene printer --seed 123 --smoke`

작업 후보 파일:

```text
Cargo.toml
crates/escape-core/Cargo.toml
crates/escape-core/src/lib.rs
crates/escape-core/src/state.rs
crates/escape-core/src/content.rs
crates/escape-core/src/actions.rs
crates/escape-core/src/turn.rs
crates/escape-core/src/effects.rs
crates/escape-core/src/save.rs
crates/escape-core/tests/core_contract.rs
crates/escape-terminal/Cargo.toml
crates/escape-terminal/src/main.rs
crates/escape-terminal/tests/cli_smoke.rs
fixtures/content/printer_slice.bundle.json
```

구현 범위:

- `GameState`
- `PlayerState`
- `TurnView`
- `ActionView`
- `ActionResult`
- `EffectCue`
- `SaveEnvelope`
- deterministic seed field 보존
- `printer_prints_alone` 하나의 encounter/action/result만 fixture로 구현
- `escape-terminal` 최소 실행 파일
  - `--scene printer`
  - `--seed <n>`
  - `--smoke`
  - core의 `TurnView`와 `EffectCue`를 headless 텍스트로 출력

비범위:

- 전체 YAML parser
- 전체 route 포팅
- full interactive terminal UI
- SuperLightTUI animation wiring
- wasm
- local secret loader
- install/package 배포 자동화

검증:

```bash
cargo test -p escape-core
cargo test -p escape-terminal
cargo test --workspace
cargo run -p escape-terminal -- --scene printer --seed 123 --smoke
cargo build -p escape-terminal --release
./target/release/escape-terminal --scene printer --seed 123 --smoke
```

성공 기준:

- Rust core가 최소 turn/action/save roundtrip을 제공한다.
- renderer가 필요한 정보를 `TurnView`만으로 받을 수 있다.
- `EffectCue::GlyphAnomaly`가 terminal/web 양쪽에서 해석 가능한 정보만 가진다.
- `escape-terminal` 실행 파일이 core truth를 사용해 같은 printer scene smoke를 출력한다.
- release build가 `target/release/escape-terminal` 실행 파일을 생성한다.

## Phase 2 — Content bundle exporter refactor

목표:

- 현재 `scripts/export_web_data.py`를 renderer-specific export가 아니라 core runtime bundle export로 확장하거나 새 스크립트를 만든다.

선택지:

A. 기존 스크립트 확장

```text
scripts/export_web_data.py
  --write
  --check
  --bundle crates/escape-core/fixtures/content/default.bundle.json
```

B. 새 스크립트 작성

```text
scripts/export_content_bundle.py
  --write
  --check
```

추천:

- 초반에는 새 스크립트보다 기존 export script를 확장하는 편이 drift가 적다.
- 다만 이름이 browser 전용이라 장기적으로는 `export_content_bundle.py`로 rename할 수 있다.

출력 후보:

```text
web/src/data/generated/content.bundle.json
crates/escape-core/fixtures/content/content.bundle.json
```

또는 하나의 canonical generated path:

```text
generated/content.bundle.json
```

주의:

- public secret guard는 반드시 유지한다.
- 실제 `final_hint`, 실제 사무실 위치, private-only 필드는 bundle에 들어가면 안 된다.
- bundle schema version을 둔다.

검증:

```bash
python scripts/export_web_data.py --check
python -m pytest tests/test_web_data_export.py -q
cargo test -p escape-core content_validation
```

## Phase 3 — Rust core parity 확장

목표:

- Python core와 Rust core의 결과가 대표 action sequence에서 일치하게 한다.
- 한번에 전체 기능을 포팅하지 말고 route family별로 확장한다.

권장 순서:

1. 상태/resource/turn pressure
2. location movement
3. encounter eligibility
4. choice cost/outcome
5. items/use actions
6. endings
7. achievements
8. ability checks / 2d6
9. reality-link public secret reward metadata
10. save/load full roundtrip

대표 parity matrix:

```text
- starting messenger route
- printer -> pantry reality hint chain
- emergency stairs escape/failure
- server room conquest
- truth route
- rooftop signal escape
- parking lot escape
- lobby exit route
- low sanity distortion cue
- high thirst hallucination cue
- consumable item use
```

검증 방식:

- Python CLI smoke 결과를 golden fixture로 저장하거나 test에서 subprocess로 비교
- Rust core action sequence 결과와 비교
- 비교 대상은 renderer string이 아니라 state/action/ending/effect cue JSON

예상 테스트:

```text
crates/escape-core/tests/python_parity.rs
crates/escape-core/tests/route_matrix.rs
crates/escape-core/tests/save_roundtrip.rs
crates/escape-core/tests/secret_safety.rs
```

명령:

```bash
PYTHONPATH=src python -m tui_adv --new --seed 123 --action ... --save /tmp/python_state.json
cargo test -p escape-core route_matrix
cargo test --workspace
```

## Phase 4 — SuperLightTUI terminal spike

목표:

- 1번 terminal 방향이 단순 debug 화면이 아니라 terminal-native horror edition으로 성립하는지 확인한다.
- 전체 TUI를 만들기 전에 `복합기가 혼자 출력한다` 한 장면만 만든다.

작업 후보 파일:

```text
crates/escape-terminal/Cargo.toml
crates/escape-terminal/src/main.rs
crates/escape-terminal/src/app.rs
crates/escape-terminal/src/layout.rs
crates/escape-terminal/src/glyphfx.rs
crates/escape-terminal/src/theme.rs
crates/escape-terminal/tests/snapshot.rs
```

화면 범위:

- location/status 요약
- 복합기 ASCII/half-block scene
- 문장 reflow/corruption animation
- stable terms 강조
- 선택지 1/2/3
- q 종료
- headless snapshot/test backend 출력

SuperLightTUI 사용 포인트:

- `slt::run_with(RunConfig::default().tick_rate(...).max_fps(60))`
- `container().draw(...)` raw buffer access
- `ui.tick()` 기반 animation
- CJK width 고려를 위해 `unicode-width` 테스트
- 필요 시 mouse는 후순위

검증:

```bash
cargo run -p escape-terminal -- --scene printer --seed 123
cargo test -p escape-terminal
cargo test --workspace
```

성공 기준:

- terminal에서도 “글자가 살아 움직인다”는 느낌이 난다.
- Web Storybook과 다른 terminal-native 매력이 있다.
- 읽어야 하는 단서가 효과 이후 안정적으로 남는다.
- headless snapshot으로 회귀 테스트가 가능하다.

## Phase 5 — Web Storybook + WASM core spike

목표:

- 3번 Web Storybook이 TypeScript mirror core 대신 Rust core/WASM으로 최소 장면을 구동할 수 있음을 확인한다.

작업 후보 파일:

```text
crates/escape-wasm/Cargo.toml
crates/escape-wasm/src/lib.rs
web/src/core/wasm.ts
web/src/core/types.ts
web/src/ui/storybook/render.ts
web/src/ui/storybook/controls.ts
web/src/effects/glyphfx.ts
web/src/main.ts
web/vite.config.ts
web/package.json
```

API 형태 후보:

```ts
const core = await loadCore()
const state = core.newGame({ seed: 123, contentBundle })
const turn = core.turnView(state)
const result = core.applyAction(state, "choice:read_printout")
```

초기에는 JSON string boundary를 추천한다.

```text
wasm new_game_json(options_json, content_json) -> state_json
wasm turn_view_json(state_json, content_json) -> turn_view_json
wasm apply_action_json(state_json, action_id, content_json) -> action_result_json
```

이유:

- wasm-bindgen 타입 설계에 시간을 덜 씀
- Rust/TS schema drift를 JSON fixture로 잡기 쉬움
- 초반 spike에 충분함

검증:

```bash
cargo test -p escape-wasm
cd web && npm test
cd web && npm run build
```

성공 기준:

- Web Storybook이 Rust core의 `TurnView`를 렌더링한다.
- 선택 실행 후 Rust core `ActionResult`를 반영한다.
- `EffectCue::GlyphAnomaly`를 Web GlyphFX가 해석한다.
- 기존 public secret guard가 유지된다.

## Phase 6 — Legacy TypeScript mirror core 축소

목표:

- Web이 Rust/WASM core를 쓰기 시작하면 TypeScript mirror core는 점진적으로 제거하거나 UI helper로 축소한다.

작업 후보:

- `web/src/game/*` 중 순수 game rule 모듈 제거/축소
- `web/src/game/parity.test.ts`를 WASM core parity test로 전환
- `web/src/ui/render.ts`를 storybook renderer로 대체 또는 legacy fake-TUI renderer로 격하
- `web/src/effects/printerFlow.ts`를 `glyphfx.ts` 또는 scene effect registry로 통합

주의:

- 2번 fake-TUI dashboard를 즉시 삭제하지 않는다.
- 먼저 Storybook/WASM route smoke가 기존 parity test 범위를 대체해야 한다.
- 삭제는 별도 cleanup PR로 한다.

## Phase 7 — Python/Textual legacy 처리

목표:

- Python/Textual 구현을 언제까지 유지할지 결정한다.

가능한 선택지:

A. Python/Textual을 dev/debug oracle로 장기 유지

장점:
- 기존 테스트와 CLI smoke를 그대로 보존
- Rust migration 중 안정망

단점:
- 두 core 유지 비용 발생
- 문서 혼란 가능

B. Rust core parity 완료 후 Python core를 freeze

장점:
- 더 이상 Python feature 추가하지 않음
- migration 비용 통제

단점:
- 기존 Python-based workflow 일부 재작성 필요

C. Rust core parity 완료 후 Python/Textual 제거

장점:
- 단일 core 원칙이 가장 명확

단점:
- 한 번에 바꾸면 회귀 위험 큼

추천:

- Phase 1-5 동안은 A
- Phase 6 이후 B
- Rust terminal + Web Storybook이 대표 루트를 안정적으로 통과하면 C 검토

## Phase 8 — 제품 방향 문서 최종 정리

목표:

- repo 문서가 실제 구조와 일치하게 만든다.

문서 후보:

```text
README.md
AGENTS.md
docs/00_Index.md
docs/dev/Architecture.md
docs/dev/Development_Plan.md
docs/dev/Data_Schema.md
docs/dev/TUI_Layout.md
docs/design/UI_Rules.md
docs/design/TUI_Storybook_GlyphFX_Concept.md
docs/implementation-map/README.md
```

반영 내용:

- primary player UX: Web Storybook + GlyphFX
- terminal edition: SuperLightTUI terminal-native edition
- core: Rust `escape-core`
- legacy: Python/Textual, TS mirror status
- 현실 연결 local-only safety boundary
- effect cue contract
- content bundle flow

## 6. 파일 변경 예상 목록

### 문서

```text
docs/dev/Rust_Core_Dual_Renderer_Architecture.md      # 신규 권장
docs/dev/Architecture.md                              # 목표/현재 구조 분리
docs/dev/Development_Plan.md                          # Rust migration phase 추가
docs/dev/Data_Schema.md                               # bundle/TurnView/EffectCue schema 추가
docs/design/TUI_Storybook_GlyphFX_Concept.md          # 채택 후보 상태 업데이트
docs/design/UI_Rules.md                               # terminal/web GlyphFX 차이 반영
docs/dev/TUI_Layout.md                                # Textual -> SLT terminal target 분리
README.md                                             # 실제 구현 후 target architecture 요약
idea_box/inbox/2026-05-21-tui-storybook-glyphfx-concept-v2.md  # 설계 반영 후 done 처리
```

### Rust workspace

```text
Cargo.toml
crates/escape-core/**
crates/escape-terminal/**
crates/escape-wasm/**
```

### Content/export

```text
scripts/export_web_data.py                            # 확장 또는 rename 후보
scripts/export_content_bundle.py                      # 신규 후보
generated/content.bundle.json                         # 위치 결정 필요
web/src/data/generated/content.bundle.json            # browser import 후보
crates/escape-core/fixtures/content/*.json            # test fixture 후보
```

### Web

```text
web/package.json
web/vite.config.ts
web/src/main.ts
web/src/core/wasm.ts
web/src/core/types.ts
web/src/ui/storybook/**
web/src/effects/glyphfx.ts
web/src/effects/printerFlow.ts                        # 통합/축소 후보
web/src/game/**                                       # legacy mirror 축소 후보
```

### Legacy Python

초기에는 수정 최소화.
나중에 다음만 필요할 수 있다.

```text
src/tui_adv/main.py                                   # legacy smoke fixture export 보조
src/tui_adv/game/save.py                              # Rust parity fixture 비교용 schema 확인
tests/**                                             # Rust parity oracle helper 추가 가능
```

## 7. 검증 계획

### 7.1 문서-only 단계

```bash
git diff --check
python scripts/export_web_data.py --check
PYTHONPATH=src python -m pytest tests -q
cd web && npm test
cd web && npm run build
```

### 7.2 Rust core 단계

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### 7.3 Terminal renderer 단계

```bash
cargo run -p escape-terminal -- --scene printer --seed 123
cargo run -p escape-terminal -- --smoke --seed 123
cargo test -p escape-terminal
```

추가 snapshot 검증:

- normal state snapshot
- low sanity distortion snapshot
- printer glyph anomaly snapshot
- ending panel snapshot
- CJK width alignment snapshot

### 7.4 Web/WASM 단계

```bash
cargo test -p escape-wasm
cd web && npm test
cd web && npm run build
```

브라우저 smoke 후보:

```bash
cd web && npm run dev -- --port 8765
```

수동 확인:

- Storybook default page readability
- GlyphFX가 단서를 가리지 않는지
- mobile portrait width에서 선택지가 안정적인지
- localStorage save/load schema migration
- public bundle에 private secret field가 없는지

### 7.5 Cross-renderer parity

같은 action sequence에 대해 다음을 비교한다.

- final `GameState`
- unlocked achievements
- ending id/kind
- public secret reward metadata
- emitted `EffectCue`
- available next actions

비교 대상은 terminal/web rendered string이 아니라 core JSON이어야 한다.

## 8. 리스크와 대응

### 리스크 1: Rust/WASM build complexity

문제:

- Web build에 Rust/WASM toolchain이 들어가면 setup이 무거워진다.

대응:

- Phase 5 전까지 Web은 기존 TS mirror 유지
- WASM wrapper는 JSON string API로 시작
- build command와 setup 문서 명확화
- CI에서 cargo + npm build를 분리

### 리스크 2: SuperLightTUI maturity

문제:

- SuperLightTUI는 새 라이브러리이며 API 변화 가능성이 있다.

대응:

- 버전 pin
- terminal spike 먼저
- core는 SuperLightTUI에 의존하지 않게 설계
- 필요하면 terminal renderer만 교체 가능하게 유지

### 리스크 3: 세 구현 drift

문제:

- Python core, TS mirror, Rust core가 전환 기간 동안 어긋날 수 있다.

대응:

- 전환 기간 동안 새 gameplay feature는 가능하면 Python/TS가 아니라 Rust core contract 쪽에 먼저 설계
- parity tests를 route family별로 작성
- migration 완료 전까지 feature freeze 범위 결정

### 리스크 4: CJK/한글 terminal layout

문제:

- terminal cell width, 줄바꿈, box drawing, 한글 width가 깨질 수 있다.

대응:

- `unicode-width` 기반 테스트
- SuperLightTUI TestBackend snapshot에서 한글 포함
- terminal GlyphFX는 너무 미세한 좌표 연출에 의존하지 않는다.

### 리스크 5: GlyphFX가 가독성을 해침

문제:

- 효과가 많아지면 단서/선택지를 읽기 어렵다.

대응:

- effect cue에 `stable_terms`, `intensity`, `duration_hint` 포함
- 저정신력/현실 연결 장면에서도 최종 단서는 안정적으로 남김
- terminal/web 모두 “복구 후 readable” 테스트 작성

### 리스크 6: 현실 연결 비밀 누출

문제:

- 실제 사무실 위치가 generated bundle, WASM fixture, web build에 포함될 수 있다.

대응:

- bundle export 시 private-only field denylist 유지
- Rust core secret safety test 추가
- `private/`, `*.local.*`, `secrets.local.*` gitignore 유지
- PR summary에 실제 위치 작성 금지

### 리스크 7: 너무 큰 rewrite

문제:

- Rust core + terminal + web을 한 번에 하려 하면 진행이 멈출 수 있다.

대응:

- `복합기가 혼자 출력한다` 한 장면으로 spike
- 기존 Python/TS 구현 유지
- route family별 migration
- 문서 PR -> core spike PR -> terminal spike PR -> web wasm spike PR로 분리

## 9. 열린 질문

1. Rust core의 content input은 `content.bundle.json` 하나로 통일할 것인가?
   - 기본 제안: 예.

2. Rust core가 YAML을 직접 읽어야 하는가?
   - 기본 제안: 아니오. YAML은 authoring source, runtime은 generated JSON bundle.

3. Web Storybook은 언제 primary로 선언할 것인가?
   - 기본 제안: WASM core spike + Storybook scene spike가 기존 fake-TUI보다 낫다는 판단 후.

4. Python/Textual은 언제 freeze할 것인가?
   - 기본 제안: Rust core가 대표 route parity를 통과한 뒤.

5. SuperLightTUI terminal은 full product로 키울 것인가, terminal edition으로 둘 것인가?
   - 기본 제안: terminal edition. Web Storybook과 같은 기능을 모두 같은 polish로 따라가려 하지 않는다.

6. 2번 browser fake-TUI dashboard는 언제 제거할 것인가?
   - 기본 제안: Web Storybook/WASM이 route parity와 save/load를 대체한 뒤 cleanup PR에서 제거.

7. updated `docs/demo/dual-renderer-directions.html`는 commit할 것인가?
   - 기본 제안: 남아있는 1번 terminal / 3번 Web Storybook 방향만 보여주는 decision aid로 `docs/demo/`에 보존한다. 이전 3방향 비교 데모는 제거했다.

## 10. 권장 다음 액션

다음 작업은 구현이 아니라 문서 정렬 PR이 좋다.

1. `docs/dev/Rust_Core_Dual_Renderer_Architecture.md` 신규 작성
2. `docs/dev/Architecture.md`를 “현재 구조 / 목표 구조 / 전환 구조”로 refactor
3. `docs/dev/Development_Plan.md`에 Rust Core Migration phase 추가
4. `docs/dev/Data_Schema.md`에 `TurnView`, `ActionResult`, `EffectCue`, `content.bundle.json` 초안 추가
5. `docs/design/TUI_Storybook_GlyphFX_Concept.md`에 Web primary candidate 및 terminal-native GlyphFX 관계 추가
6. 문서-only 검증 실행
7. PR로 올린 뒤, 그 다음에 Phase 1 Rust core spike 착수

## 11. 요약 판단

이 방향은 장기적으로 타당하다.

좋은 점:

- 하나의 GameCore로 1번 terminal과 3번 web을 동시에 살릴 수 있다.
- TypeScript mirror core drift를 줄일 수 있다.
- SuperLightTUI는 terminal-native GlyphFX 실험에 적합하다.
- Web Storybook + GlyphFX는 플레이어용 primary UX로 자연스럽다.

주의할 점:

- 바로 C 방향 전체 rewrite를 시작하면 위험하다.
- 먼저 문서 refactor, 그 다음 `printer_prints_alone` 한 장면 spike로 검증해야 한다.
- core와 renderer 경계를 절대 흐리면 안 된다.

핵심 원칙은 계속 이것이다.

```text
Core owns truth.
Renderer owns mood.
```
