# Three.js 전투 비주얼 아키텍처

Status: approved canonical direction, WP2 ACTIVE (blocked on WP1 merge/review), 2026-08-13
Scope: `ScenePage.combat` Web renderer contract
Primary storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**
Compatibility: `escape from the office` legacy surface

## 1. 결정 요약

이 문서는 Rust GameCore의 전투 결과를 Three.js로 표시하는 정본이다. **PC-first**는 전투 성능 기준이지 Storybook을 desktop dashboard로 바꾸는 지시가 아니다. 기존 **모바일 텍스트RPG 게임 프레임**과 3분할 shell/HUD는 유지한다. 대표 중급 Windows PC의 1080p/60fps를 목표로 시각 품질을 먼저 만들고 계측 후 hard budget을 확정한다. 모바일 최적화는 후속 WP다.

화면은 고정 평면 육각 보드, 반 SD 저폴리 캐릭터, simple toon, authored portrait, 결정론적 Three.js 전투 VFX를 결합한다. 얼굴은 개성을 보조하지만 감정과 상태의 의미를 소유하지 않는다.

## 2. 확정 아키텍처 계약

| 영역 | 결정 |
| --- | --- |
| 카메라 | Three.js 고정 `OrthographicCamera`. pose·각도·보드 중심 고정, 사용자 pan/rotate/zoom과 camera shake 없음. viewport 변경 때 frustum만 refit. |
| 보드 | 첫 3D stage/test board preset은 7×6 flat-top axial hex, 42칸. `q`는 좌우 축, `r`은 대각 축. 영구 gameplay cap이 아니다. |
| 전투원 | 후속 stage 혼전 검증 목표는 동시 12명. seed-2 WP1 fixture는 2명이며, 12명은 영구 roster/encounter cap이 아니다. 칸 점유와 상태는 GameCore가 결정. |
| 캐릭터 | 4~4.5-head semi-SD low-poly GLB. 명확한 실루엣과 큰 손·발. |
| 리깅 | shared humanoid skeleton, common animations, modular body/clothing/weapon parts. 전투원별 bespoke rig 금지. |
| 셰이딩 | base, shadow, highlight의 simple 3-tone toon. 재질과 palette token 공유. |
| 얼굴·감정 | 얼굴 geometry와 비의미적 표정은 허용. authored 2D portrait와 GameCore cue가 authoritative source. |
| 그림자 | blob/contact shadow baseline. real-time directional shadow는 시각 품질 후보로 구현·측정할 수 있다. |
| 후처리 | 제한적 bloom, outline, color grade를 품질 후보로 측정한다. 실측 없이 영구 금지하거나 필수화하지 않는다. |
| VFX | `arc`, `burst`, `ring`, `glyph`, `dust`, `trail` 재사용 primitive. cue당 최대 2 major effect 방향. |
| DOM | semantic/accessibility mirror가 항상 존재. WebGL은 시각 표현만 담당. |
| 실패 | WebGL 미지원, context loss, GLB fetch 실패 시 DOM 보드·portrait·log로 즉시 fallback. |

## 3. 진실 원천과 결정론

GameCore가 전투원 id·진영·axial 좌표·상태, tick 순서, 이동·충돌·명중·피해·결착 판정, cue/log 순서, `simulation_version`, `resolution_fingerprint`, view `fingerprint`, `tick_millis`를 소유한다. `ScenePage.combat`은 additive-optional 결과 경계다. 값이 없으면 Web은 combat DOM과 WebGL을 만들지 않고 기존 story output parity를 유지한다.

Web/Three.js는 AI, 판정, 승패, 충돌, 로그 재정렬, 누락 cue 발명, seed 재계산을 하지 않는다. `active`를 생존으로 표시하지 않으며 생존·전투불능은 core report/cue만 따른다.

소유권은 다음처럼 나눈다.

| owner | 소유 | 금지 |
| --- | --- | --- |
| Rust GameCore | simulation, 좌표·점유, 상태, cue/log 순서, 승패, seed 입력 | renderer 상태를 gameplay truth로 읽기 |
| `ScenePage.combat` | versioned renderer-neutral snapshot/replay 경계 | Three.js object나 DOM node 직렬화 |
| Web adapter | schema 검증, 좌표 투영 입력, 오류 진단 | 판정 보정, cue 합성 |
| Three.js stage | camera, mesh, animation, light, shadow, combat VFX | 의미·조작의 유일한 표면 되기 |
| Storybook DOM | HUD, portrait, log, 접근성 mirror, fallback | core 결과 재계산 |
| GlyphFX | story/UI 전환·글리치·텍스트 효과 | 전투 보드 공간 VFX 소유 |

wire 입력은 `view.simulation_version`, `view.resolution_fingerprint`,
`view.fingerprint`, `frame.tick`, `piece.id`, ordered `piece.cues[]`다.
adapter는 각 cue value를 `cue_type`, `piece.cues`의 0-based ordered index를
`cue_ordinal`로 정규화한다. `cue_ordinal`은 파생 값이며 wire field가 아니다.
visual seed tuple의 정확한 순서는 다음 7개다.

```text
[simulation_version, resolution_fingerprint, view.fingerprint, frame.tick, piece.id, cue_type, cue_ordinal]
```

문자열 항목은 이미 검증된 non-empty wire string을 그대로 쓰며 Unicode 정규화나
case 변환을 하지 않는다. 숫자 항목인 `frame.tick`과 `cue_ordinal`은 JavaScript safe
nonnegative integer여야 한다. tuple은 ECMAScript `JSON.stringify`를 replacer와
whitespace 없이 동기 호출해 직렬화한다. 그 문자열을 `TextEncoder`로 UTF-8 bytes로
바꾼 뒤 FNV-1a 64-bit를 적용한다. 초기값은 `0xcbf29ce484222325n`, prime은
`0x100000001b3n`이다. 각 byte마다 `hash = hash ^ BigInt(byte)`를 먼저 대입하고,
그 갱신된 hash로 `hash = BigInt.asUintN(64, hash * prime)`을 계산한다. 출력은 leading zero를 포함한 정확히
16자리 lowercase hex다. 이 seed는 visual-only, non-security 값이다.
`Math.random()`, 시간, viewport, GPU, DPR, quality, `run_seed`, `encounter_id`,
`combat_id`를 입력에 넣지 않는다.

현재 producer fixture의 seed golden 예제는 다음 값으로 고정한다. 테스트는 직렬화
문자열, UTF-8 bytes 또는 동등한 byte hex, 최종 hash를 모두 pin한다.

```text
tuple       = ["v3","1978f44e79dd23d1","bb9240dfbc6e72b0",1,"wuxia_spectator_bout_ally","attack",0]
JSON        = ["v3","1978f44e79dd23d1","bb9240dfbc6e72b0",1,"wuxia_spectator_bout_ally","attack",0]
UTF-8 hex   = 5b227633222c2231393738663434653739646432336431222c2262623932343064666263366537326230222c312c2277757869615f737065637461746f725f626f75745f616c6c79222c2261747461636b222c305d
FNV-1a64    = fba89b551ab6959d
```

동일 tuple은 동일한 primitive, 방향, 위상, 색상, particle count를 만든다.

### 3.1 WP1 adapter public contract

WP1의 정본 module은
`web/src/ui/storybook/combat/combatThreeAdapter.ts`다. 이 module은 DOM, Three.js,
브라우저 전역에 의존하지 않는 pure adapter다. public surface는 아래 이름과 판별자를
유지한다. 세부 readonly 표기는 구현에서 더 엄격하게 할 수 있지만 필드 의미와 result
kind는 바꾸지 않는다.

```ts
export interface CombatBoardBounds {
  minQ: number;
  maxQ: number;
  minR: number;
  maxR: number;
}
export interface CombatAdapterOptions { boardBounds: CombatBoardBounds }

export type CombatAdapterDiagnosticCode =
  | "INVALID_COMBAT_OBJECT" | "INVALID_VIEW_OBJECT"
  | "INVALID_SIMULATION_VERSION" | "INVALID_RESOLUTION_FINGERPRINT"
  | "INVALID_VIEW_FINGERPRINT" | "INVALID_TICK_MILLIS"
  | "INVALID_FRAMES" | "INVALID_FRAME" | "INVALID_FRAME_TICK"
  | "NON_MONOTONIC_FRAME_TICK" | "INVALID_PIECES"
  | "INVALID_PIECE" | "INVALID_PIECE_ID" | "DUPLICATE_PIECE_ID"
  | "INVALID_POSITION" | "INVALID_FACING" | "INVALID_SIDE"
  | "INVALID_ACTIVE" | "INVALID_CUES" | "INVALID_CUE"
  | "INVALID_BOARD_BOUNDS" | "OUT_OF_BOUNDS" | "DUPLICATE_OCCUPANCY";
export interface CombatAdapterDiagnostic {
  code: CombatAdapterDiagnosticCode;
  severity: "error" | "warning";
  path: string;
}

export type NormalizedCombatCueType =
  | "attack" | "hit" | "evade" | "balance_broken" | "incapacitated";
export interface NormalizedCombatCoord { q: number; r: number }
export interface NormalizedCombatCue {
  type: NormalizedCombatCueType;
  ordinal: number;
  seedHex: string;
}
export interface NormalizedCombatPiece {
  id: string;
  side: "ally" | "enemy";
  position: NormalizedCombatCoord;
  facing: NormalizedCombatCoord;
  active: boolean;
  cues: readonly NormalizedCombatCue[];
}
export interface NormalizedCombatFrame {
  tick: number;
  pieces: readonly NormalizedCombatPiece[];
}
export interface NormalizedCombatReplay {
  simulationVersion: string;
  resolutionFingerprint: string;
  viewFingerprint: string;
  tickMillis: number;
  frames: readonly NormalizedCombatFrame[];
}

export type CombatAdapterResult =
  | { kind: "absent"; diagnostics: readonly [] }
  | { kind: "fallback"; diagnostics: readonly CombatAdapterDiagnostic[] }
  | { kind: "ready"; replay: NormalizedCombatReplay; diagnostics: readonly CombatAdapterDiagnostic[] };

export function adaptCombatForThree(
  combat: unknown,
  options: CombatAdapterOptions,
): CombatAdapterResult;
export function axialToWorld(
  coord: Readonly<NormalizedCombatCoord>,
  size: number,
): { x: number; z: number };
export function combatVisualSeedHex(
  tuple: readonly [string, string, string, number, string, NormalizedCombatCueType, number],
): string;
```

`adaptCombatForThree`는 input array 순서를 보존하고 input object나 nested array를
변경하지 않는다. `frames`, `pieces`, `cues`의 output도 같은 순서다. adapter는
`core_log`, `full_log`, `report`를 소비, 복사, 정규화, 검증하지 않는다. WP1 동안
기존 DOM renderer가 visible surface를 계속 제공하며 adapter는 아직 그 경로에 연결되지
않는다. adapter는 `active`를 alive로 바꾸거나 그 의미를 추론하지 않는다.

`boardBounds`는 wire에 없는 필수 host config다. `minQ <= maxQ`, `minR <= maxR`인
safe integer bounds여야 하며 invalid config도 throw하지 않고 fallback diagnostic을
반환한다. WP1 test는 test-local constant
`{ minQ: 0, maxQ: 6, minR: 0, maxR: 5 }`를 명시적으로 넘긴다. 이는 첫 3D stage가
검증할 **의도된 board preset**이며 producer JSON에서 나온 값이나 production default가
아니다. adapter는 piece 좌표에서 bounds를 추론하지 않는다. 첫 production bounds는 §11.2의
Web host preset이 소유하고, content-defined bounds는 후속 additive schema 결정으로 남긴다.
`axialToWorld`는 flat-top 공식을
그대로 쓴다: `x = size * 1.5 * q`, `z = size * sqrt(3) * (r + q / 2)`.

### 3.2 malformed input policy

`combat === undefined` 또는 `combat === null`이면 `absent`다. 다른 입력은 어떤
경우에도 throw하지 않는다. hard malformed consumed field가 하나라도 있으면
`fallback`과 deterministic diagnostics를 반환하고 Three.js 경로를 건너뛴다. caller는
Storybook 본문과 user actions를 유지하고 combat 영역에는 안전한 localized unavailable
표면만 표시한다. hard malformed raw payload를 기존 DOM combat renderer에 그대로 넘겨
semantic board/log/report를 억지로 만들지 않는다. UI에는 raw payload나 internal stack을
넣지 않는다. integration caller가 추가되는 WP에서는 반환된 첫 error의 `code`와 `path`만
한 번 sanitized log로 남긴다. WP1 pure adapter 자체는 console이나 telemetry side effect를
만들지 않는다. 이 fail-soft wiring은 WP2 범위이며 WP1에는 사용자 표면 변화가 없다.

hard malformed 범위는 non-object combat/view, 비어 있는
`simulation_version`/`resolution_fingerprint`/view `fingerprint`, invalid
`tick_millis`, invalid `frames` type 또는 frame tick 비증가 순서, invalid frame,
piece, position, facing, side, cue, 한 frame 안의 duplicate piece id다. consumed number는
모두 JavaScript safe integer여야 하고 fractional/unsafe 값은 hard error다.
`tick_millis`는 positive safe integer, `frame.tick`은 nonnegative safe integer여야 한다.
position의 `q`/`r`은 signed safe integer를 허용한다. facing은 flat-top axial 인접 방향
`(1,0)`, `(1,-1)`, `(0,-1)`, `(-1,0)`, `(-1,1)`, `(0,1)` 중 하나여야 한다.
string id는 non-empty여야 하고 `active`는 boolean이어야 한다.

필드가 아예 없는 Rust-defaultable `frames`, `pieces`, `cues`는 빈 배열로 정규화한다.
그 필드가 존재하지만 array가 아니면 hard error다. empty frames와 empty pieces는
valid다. unknown extra field는 무시한다. 13명 이상이라는 이유만으로 거부하지 않는다.
out-of-bounds position과 같은 frame 안의 duplicate occupancy는 clamp/drop하지 않고
원래 piece와 순서를 보존한 `ready`를 반환하며 각각 `OUT_OF_BOUNDS`,
`DUPLICATE_OCCUPANCY` warning을 붙인다. unknown side는 hard error다. adapter가 소비하지
않는 log/report는 malformed 판정 대상이 아니다. unknown log template은 기존
`combatLogTemplates.ts` fallback을 그대로 사용한다.

diagnostic 순서는 options, root/view field, frame input order, frame 안 piece input order,
cue input order, 마지막으로 bounds/occupancy warning 순서다. 같은 code/path는 한 번만 낸다.
diagnostic에는 code, severity, canonical path만 넣고 offending value나 exception text를
넣지 않는다.

정확한 code/path mapping은 다음과 같다. path는 아래 JSONPath-like 표기만 사용한다.

| 검사 실패 | code | path |
| --- | --- | --- |
| combat가 object가 아님 | `INVALID_COMBAT_OBJECT` | `$` |
| view 누락/object 아님 | `INVALID_VIEW_OBJECT` | `$.view` |
| version/fingerprint 문자열 오류 | 대응하는 `INVALID_*FINGERPRINT` 또는 `INVALID_SIMULATION_VERSION` | `$.view.<field>` |
| tick_millis 오류 | `INVALID_TICK_MILLIS` | `$.view.tick_millis` |
| frames가 존재하지만 array 아님 | `INVALID_FRAMES` | `$.view.frames` |
| frame가 object가 아님 | `INVALID_FRAME` | `$.view.frames[i]` |
| frame.tick 자료형·범위 오류 | `INVALID_FRAME_TICK` | `$.view.frames[i].tick` |
| 이전 tick 이하 | `NON_MONOTONIC_FRAME_TICK` | `$.view.frames[i].tick` |
| pieces가 존재하지만 array 아님 | `INVALID_PIECES` | `$.view.frames[i].pieces` |
| piece가 object가 아님 | `INVALID_PIECE` | `$.view.frames[i].pieces[j]` |
| id 오류 / 같은 frame의 두 번째 동일 id | `INVALID_PIECE_ID` / `DUPLICATE_PIECE_ID` | `$.view.frames[i].pieces[j].id` |
| position/facing/side/active 오류 | 각각 `INVALID_POSITION`, `INVALID_FACING`, `INVALID_SIDE`, `INVALID_ACTIVE` | 해당 piece field path |
| cues가 존재하지만 array 아님 | `INVALID_CUES` | `$.view.frames[i].pieces[j].cues` |
| cue enum 오류 | `INVALID_CUE` | `$.view.frames[i].pieces[j].cues[k]` |
| adapter bounds 오류 | `INVALID_BOARD_BOUNDS` | `$options.boardBounds` |
| bounds 밖 position | `OUT_OF_BOUNDS` | 해당 `.position` |
| 같은 frame의 두 번째 동일 점유 | `DUPLICATE_OCCUPANCY` | 두 번째 piece의 `.position` |

`undefined`/`null` absent 판정은 options보다 먼저 하므로 absent에는 bounds diagnostic을
붙이지 않는다. 그 밖에는 options → root/view scalar → frame/piece/cue input order →
warning 순서로 가능한 sibling 오류를 모두 누적한다. container가 invalid면 그 subtree만
더 내려가지 않는다. invalid tick은 monotonic 비교에서 제외하고, invalid bounds면
bounds/occupancy warning을 계산하지 않는다. error가 하나라도 있으면 `fallback`, error 없이
warning만 있으면 `ready`다.

### 3.3 producer-owned golden fixture

WP1의 canonical fixture는
`crates/escape-core/fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json`다.
다음 명령의 stdout bytes로만 생성한다.

```bash
cargo run -q -p escape-core --example dump_combat_spectator -- 2 > crates/escape-core/fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json
```

현재 expected SHA-256은
`3ecfb08390379fa3cea7f2bc802ab47dc164695ffdb0af5cc54222c9af3fd53a`다. fixture는
`simulation_version: "v3"`, frame 8개, `tick_millis: 100`, combatant 2명을 포함한다.
`crates/escape-core/tests/combat_spectator_fixture_golden.rs`는 같은 public producer path를
호출해 pretty JSON과 trailing newline을 만들고 checked-in fixture bytes와 byte-for-byte
비교한다. Web Vitest는 `resolveJsonModule`을 사용해
`../../../../../crates/escape-core/fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json`
을 static JSON import한다. Node `fs`와 `@types/node`는 사용하지 않고 `web/` 아래에
fixture를 복제하지 않는다. 기존 prototype copy는 noncanonical이다. 후속 7x6 edge, 12-unit,
worst-cue fixture도 handwritten final JSON이 아니라 Rust builder/producer로 생성한다.

## 4. 보드 투영과 캐릭터

첫 3D stage/test board preset은 `q ∈ [0, 6]`, `r ∈ [0, 5]`다. seed-2 producer
fixture의 실제 piece span은 `q=1..4`, `r=0`이며 board bounds를 제공하지 않는다.
WP1 test가 preset을 options로 명시한다. 확장 시 선언된 board bounds를 읽으며 7×6을
영구 상수로 두지 않는다. q 방향은 RTL에서도 고정한다. flat-top 투영은 다음과 같다.

```text
x = size * (3/2 * q)
z = size * sqrt(3) * (r + q/2)
```

칸 중심이 character/contact-shadow anchor다. Orthographic frustum은 viewport에 맞춰 크기만 갱신한다. 보드 밖 좌표와 중복 점유는 판정을 고치지 않고 warning과 DOM으로 보존한다. 알 수 없는 진영은 WP1 hard malformed로 처리해 Three.js 경로만 fallback한다.

가림 순서는 world `z`와 카메라 depth로 결정하고 임의의 per-frame sort를 만들지 않는다. 전투원 발 anchor는 칸 중심을 벗어나지 않는다. 무기·소매는 이웃 칸으로 시각적으로 뻗을 수 있지만 selection/status ring과 다른 전투원의 얼굴·몸통을 지속적으로 가리면 안 된다. 동일 칸 중복은 모델을 겹쳐 그리지 않고 진단 marker 하나와 DOM 목록으로 보존한다. 이름표·HP·상태 icon은 screen-space DOM HUD가 소유하며 겹침 시 우선순위는 선택 대상, 현재 행동자, 위험 상태, 나머지다. 낮은 우선순위 표지는 축약하거나 leader line으로 밀되 core 상태를 숨기지 않는다.

공용 skeleton은 bone 이름·축·rest pose·unit scale을 먼저 고정한다. body, hair/headwear, clothing, weapon, accent part는 교체 가능하고 missing part/material placeholder를 제공한다. baseline animation semantic set은 `idle`, `move`, `attack`, `hit`, `evade`, `balance_broken`, `incapacitated`, `victory`다. 보드 이동은 axial frame이 소유하며 clip은 제자리 재생이 기본이다.

얼굴이 없어도 전투 의미는 유지되어야 한다. portrait alt text는 authoring display name과 authored emotion label을 사용하고 얼굴 geometry를 해석해 만들지 않는다. 진영과 역할은 색보다 실루엣을 먼저 쓴다. 머리·어깨·무기 외곽, stance, base/ring shape가 grayscale과 색각 이상 조건에서도 구분되어야 하며 색은 보조 신호다.

HUD는 authored 2D portrait, 이름, 진영, 체력/호흡 등 core 제공 수치, 상태, 현재 cue, log를 표시한다. 감정은 portrait와 텍스트가 소유한다. 3D 얼굴 표정, procedural gaze, animation clip만으로 공포·분노·배신 같은 서사 의미를 전달하지 않는다.

## 5. VFX와 실험 기능

VFX는 `primitive + parameter + seed + lifetime` 데이터이며 pool에서 재사용한다. 방향·수명·수량은 seed/preset으로 정하고 reduced motion에서는 static marker로 축약한다. blob/contact shadow가 baseline이다. directional shadow와 restrained postprocessing은 toggle 뒤에서 같은 fixture의 품질·CPU/GPU 비용을 비교한 뒤 유지 여부를 정한다.

## 6. DOM·접근성 계약

- 보드에는 `role="img"` 설명과 전투원별 semantic list/table을 제공한다. id, 진영, axial 좌표, 상태, 마지막 cue를 포함한다.
- log는 `aria-live="polite"` 영역이며 core log 순서를 그대로 표시한다. full log는 별도 펼침 영역이다.
- keyboard focus, Enter/Space, visible focus, forced colors, reduced motion을 지원한다.
- canvas가 없어도 id, 진영, 좌표, 상태, cue, portrait, core log를 읽을 수 있어야 한다.
- WebGL 생성 실패나 `contextlost`에서는 DOM board + portrait + log로 전환하며 GameCore 진행은 중단하지 않는다.

## 7. PC-first 성능 단계

### 7.1 Prototype visual target

대표 중급 Windows PC, 1920×1080, DPR 1.0 또는 합리적 cap, 7×6 보드, 12명, 대표 worst-case cue fixture를 첫 계측 기준으로 고정한다. 목표는 정상 상태 60fps이며 p50/p95/p99 frame time을 기록한다. 대표 기기와 브라우저/GPU/드라이버, cold/warm run, 측정 길이도 결과에 함께 남긴다.

초기에는 draw calls, triangles, texture memory, particle 수를 hard budget으로 선언하지 않는다. `renderer.info`, GPU/CPU frame time, JS heap/GPU memory 추정, load/decode, shader hitch, context loss를 수집하고 baseline·shadow·postprocessing을 같은 조건에서 비교한다.

gate는 같은 fixture의 반복 실행, frame-time percentile, 긴 프레임 원인, `renderer.info`, heap 추세, 시각 비교 캡처가 모두 남는 것이다. 평균 fps 하나만으로 통과시키지 않는다. 먼저 visual-quality target을 만든 뒤 baseline, shadow, restrained postprocessing을 같은 조건에서 비교한다. 측정 결과를 근거로 후속 decision record에서 resource budget과 quality tier를 확정한다. 다만 구조적 효율은 즉시 필수다: shared rigs/materials, instancing, object/particle pooling, 명확한 disposal과 중복 asset 방지. scene 재진입 반복 뒤 resource count와 heap이 안정화되지 않으면 실패다.

모바일 DPR·payload·저사양 preset은 **Mobile Optimization WP**에서 다룬다. 초기 acceptance에 모바일 30fps나 모바일 hard budget을 넣지 않는다. DOM 접근성과 WebGL failure fallback은 PC-first와 무관하게 초기 필수다.

## 8. 자산·레퍼런스 정책

private study에서는 레퍼런스의 직접 연구·adaptation을 허용한다. asset마다 `source`, `observed`, `adapted`, `license/status`, `publication_gate`만 기록한다. 공개 전 고유 캐릭터·로고·서명 pose·문구의 복제와 라이선스/유사성을 재감사하고, `cleared`가 아니면 대체·제외한다. provenance는 `ScenePage`에 넣지 않는다.

구현자가 사용할 이미지, 저장소 URL, 기준 commit, 파일별 참고 목적은
[Three.js 전투 구현 레퍼런스 팩](references/threejs_combat/README.md)에 고정한다.
WP1~2의 기술 contract는 이 문서만으로 시작할 수 있지만, 캐릭터·VFX·화면 구성 WP는
레퍼런스 팩까지 읽는다.

## 9. superseded older rules

다음 규칙은 이 문서와 충돌할 때 superseded다. 역사 기록은 보존하지만 전투 Web visual의 구현 기준으로 사용하지 않는다.

| 이전 규칙 | 새 canonical 규칙 |
| --- | --- |
| browser fake-TUI가 primary surface | Web Storybook + Three.js canvas primary, TUI는 parity/fallback |
| flat 2D 또는 체스말만 사용 | 3D low-poly GLB primary, DOM mirror 병행 |
| perspective/free camera, camera shake | 고정 OrthographicCamera |
| pointy-top 또는 pixel hex | flat-top axial, 첫 3D stage/test board preset 7×6 |
| 전투원별 rig/animation | shared skeleton/common animations/modular parts |
| 얼굴 표정이 감정의 진실 | authored 2D portrait와 core cue |
| 모바일 30fps 및 고정 저예산을 초기 기준 | PC-first 60fps 목표, 측정 후 budget 확정 |
| shadow/postprocessing을 실측 없이 금지하거나 필수화 | 3-tone/blob baseline, shadow/postprocess는 비교 실험 후 결정 |
| canvas가 유일한 정보 surface | DOM semantic mirror와 failure fallback 필수 |

GameCore 판정 소유권, renderer의 seed·AI·log 재구현 금지, `ScenePage.combat` additive-optional은 superseded되지 않는다.

## 10. acceptance tests

### 10.1 WP1 acceptance

WP1은 다음 adapter/fixture 증거만 만든다.

1. `undefined`와 `null`은 `absent`이며 throw하지 않는다.
2. 각 hard malformed case는 `fallback`과 deterministic sanitized diagnostic을 반환하며 throw하지 않는다.
3. flat-top axial projection이 `x = 1.5q`, `z = sqrt(3)(r + q/2)`에 정확히 일치한다(`size = 1` 기준).
4. frame, piece, cue input 순서와 input immutability를 보존한다.
5. seed tuple 직렬화, UTF-8 bytes, FNV-1a64 hash가 §3 golden과 정확히 일치한다.
6. 같은 piece의 같은 cue라도 `cue_ordinal` 하나만 바꾸면 해당 seed만 달라진다.
7. empty frames와 empty pieces를 `ready`로 처리하고 13번째 전투원을 거부하지 않는다.
8. out-of-bounds와 duplicate occupancy는 warning을 내되 piece를 clamp/drop/reorder하지 않는다.
9. Rust golden test가 producer output과 checked-in fixture를 byte-for-byte 비교하고, Vitest가 같은 fixture를 static JSON import해 exact projection과 normalization을 검증한다.

### 10.2 후속 전체 renderer acceptance

1. 동일 `ScenePage.combat` 두 번 렌더하면 transform, primitive, particle count, DOM log 순서가 동일하다.
2. seed tuple의 tick 또는 cue index만 바꾸면 해당 effect만 달라진다.
3. 7×6 모서리·중앙 좌표가 올바르게 투영되고 RTL에서 q 의미가 뒤집히지 않는다.
4. 12명까지 shared skeleton/material/animation과 instance pool을 사용한다.
5. 13번째 전투원을 hard reject하지 않는다. 선언된 board 밖 좌표와 중복 점유는 판정을 고치지 않고 diagnostic/DOM으로 보존한다.
6. portrait/GLB 실패, WebGL 미지원, context loss 모두에서 의미 있는 DOM 화면이 남는다.
7. keyboard-only, screen reader, forced colors, reduced motion에서 의미와 조작성이 유지된다.
8. representative PC fixture에서 1920×1080 60fps 목표, p50/p95/p99, `renderer.info`, GPU/CPU 결과 artifact를 남긴다.
9. baseline과 shadow/postprocess 실험군을 같은 조건에서 비교하고 settings로 끌 수 있다.
10. shared resource, instancing, pooling, disposal에 중복 생성·누수 테스트가 있다.
11. no-combat page는 combat markup/canvas를 생성하지 않는다.
12. `active`를 생존으로 부르지 않고 core report/cue의 전투불능을 정확히 표시한다.

모바일 최적화 수치, 30fps gate, 최종 resource hard budget은 이 목록에 없다. 이는 후속 WP의 acceptance다.

## 11. 단계별 독립 merge WP

각 WP는 자기 타입·fixture·테스트를 포함하고, 다음 WP 없이도 main에서 build/test가 통과해야 한다. feature flag 또는 additive-optional 경계 뒤로 병합하며 선행되지 않은 자산은 placeholder로 대체한다. WP 완료는 코드 존재가 아니라 표의 결과와 해당 acceptance 증거가 함께 남은 상태다.

### 11.1 WP1 review/merge prerequisite

읽기 순서는 다음과 같다.

1. `docs/dev/Development_Plan.md`의 ACTIVE Web renderer track과 병렬 작업 경계
2. 이 문서 §3~4의 adapter, seed, malformed, projection 계약과 §10.1 acceptance
3. `docs/design/Combat_System_Implementation_Plan_Index.md`의 ACTIVE Three.js WP2 항목

WP1이 소유하는 구현 파일은 아래 네 종류뿐이다.

- `web/src/ui/storybook/combat/combatThreeAdapter.ts`
- `web/src/ui/storybook/combat/combatThreeAdapter.test.ts`
- `crates/escape-core/fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json`
- Rust byte golden test `crates/escape-core/tests/combat_spectator_fixture_golden.rs`

WP1은 pure adapter contract slice다. `web/src/main.ts`, `web/src/ui/storybook/render.ts`,
`web/src/ui/storybook/combat/renderCombatStage.ts`, `web/src/core/types.ts`, CSS,
`package.json`, lockfile, Rust gameplay/production logic을 수정하지 않는다. Three.js dependency,
canvas host, renderer mount도 추가하지 않는다. 기존 `renderCombatStage` DOM이 현재 visible
surface로 남는다. WP1 acceptance를 통과하면 멈추고 review를 기다린다. 이 승격은 WP2를
자동 승인하지 않았지만, 이후 §11.2의 별도 정본 결정으로 WP2 board 계약이 승인됐다.

WP2부터 lifecycle acceptance는 다음처럼 고정한다. game/start/fatal 경로에서
`appRoot.innerHTML`을 실제 교체하기 직전 기존 Three stage를 즉시 dispose하고, 교체가 끝난
뒤 새 host에 mount한다. transition이 있으면 outgoing stage를 transition 종료까지 유지한 뒤
그 replacement 직전에 dispose한다. 진행 중인 GLB async load에는 generation/abort guard를
두어 이전 mount의 stale completion이 새 scene에 attach되지 않게 한다. 이 항목들은 WP1
구현 범위가 아니다.

### 11.2 ACTIVE WP2 board 계약

WP2 정본은 승인됐지만 **구현 착수는 WP1 PR #217이 review 후 main에 merge된 뒤**다. WP1의
공개 API나 진단 순서가 review에서 바뀌면 이 절을 먼저 다시 대조한다. WP2는 첫 실제 Three.js
surface를 붙이는 독립 board slice다. investor demo에 실재하는 3D 공간·고정 카메라·결정적
core replay 연결을 제공하되 캐릭터/애니메이션/VFX/최종 품질을 선점하지 않는다.

#### 결정과 근거

1. production board bounds는 새 Rust/`ScenePage` schema가 아니라 Web host preset
   `COMBAT_THREE_BOARD_BOUNDS = { minQ: 0, maxQ: 6, minR: 0, maxR: 5 }`가 소유한다.
   현재 producer fixture에는 bounds가 없고 WP1 adapter가 bounds를 필수 option으로 받으므로,
   첫 test board의 표현 설정을 gameplay truth에 올리는 것보다 이 방식이 작고 되돌릴 수 있다.
   후속 content-defined board가 승인되면 별도 additive schema WP에서 교체한다. piece 좌표로
   bounds를 추론하지 않는다.
2. 공식 `three` package `0.185.1`을 정확한 버전으로 추가하고 lockfile을 함께 갱신한다.
   `@types/three`, renderer wrapper, React, physics, postprocessing dependency는 추가하지 않는다.
   Three.js 공식 API의 `WebGLRenderer`, `OrthographicCamera`, `ResizeObserver`, 명시적 resource
   `dispose()`만 사용한다. dependency 설치 결과가 이 exact version과 다르면 멈춘다.
3. WP2 scene은 42개 hex tile과 final normalized frame의 **임시 anchor marker**만 그린다.
   marker는 ally/enemy를 원형/각진 base shape로 구별하고 foot anchor를 칸 중심에 둔다.
   이것은 캐릭터 art가 아니며 WP3의 modular GLB가 교체한다. GLB, rig, clip, portrait, cue VFX,
   shadow map, postprocessing, bloom, outline, texture asset은 만들지 않는다. baseline light는 ambient
   + 한 개 directional light, shadow는 꺼진 상태다.
4. 카메라는 `OrthographicCamera`, yaw 30°, elevation 48°, board bounds의 기하 중심을 고정
   target으로 삼는다. 42칸의 여섯 꼭짓점을 camera basis에 투영한 bounds에 8% padding을 더하고
   host aspect ratio에 맞춰 frustum만 refit한다. near/far는 `0.1/100`, DPR은
   `min(window.devicePixelRatio, 2)`다. resize는 buffer와 frustum만 바꾸며 pose, target, board
   center, q축 의미를 바꾸지 않는다. RTL도 같은 world transform을 쓴다. 사용자
   pan/rotate/zoom, adaptive yaw, camera shake는 없다.
5. tile 중심은 WP1 `axialToWorld(coord, 1)`의 `(x,z)`를 그대로 쓴다. 별도 projection 수식,
   좌표 clamp, occupancy 수정, piece sort, AI/판정 재실행을 만들지 않는다. out-of-bounds marker는
   원 좌표에 두고 DOM warning이 의미를 보존한다. duplicate occupancy는 첫 marker 하나와
   non-semantic collision marker 하나만 그리며 두 전투원은 semantic DOM 표에 모두 남긴다.
6. WP2는 static final-frame board다. cue의 `seedHex`와 input order를 보존해 받지만 cue visual을
   만들지 않으므로 seed를 PRNG, 위치 jitter, 색, marker 변형에 소비하지 않는다.
   `Math.random()`, wall clock, viewport, DPR, GPU를 visual input으로 쓰지 않는다. 같은 ready
   replay와 viewport는 scene graph transform, geometry/material 수, `renderer.info`가 같다.
   tick playback과 seeded cue primitive는 WP6 소유다.

가까운 대안은 bounds를 `ScenePage.combat`에 추가하는 것이었다. 여러 board size를 authoring하는
시점에는 맞지만 현재 단일 7×6 preset 때문에 Rust producer, serde, WASM, TS schema를 함께 여는
비용이 더 크므로 채택하지 않는다. 기존 Canvas/CSS board를 3D처럼 꾸미는 최소안도 dependency와
lifecycle을 미룰 뿐 primary renderer 방향을 검증하지 못하므로 채택하지 않는다.

#### 공개 API와 소유 파일

신규 `web/src/ui/storybook/combat/combatThreeStage.ts`의 public surface는 다음 이름과 result
판별자를 고정한다. 세부 readonly 표기는 더 엄격해질 수 있다.

```ts
export const COMBAT_THREE_BOARD_BOUNDS: CombatBoardBounds;
export interface CombatThreeMetrics {
  calls: number; triangles: number; geometries: number; textures: number;
}
export interface CombatThreeStageHandle {
  resize(): void;
  metrics(): CombatThreeMetrics;
  dispose(): void;
}
export type CombatThreeStageDiagnosticCode =
  | CombatAdapterDiagnosticCode
  | "MISSING_HOST" | "ZERO_SIZE_HOST" | "WEBGL_UNAVAILABLE" | "CONTEXT_LOST";
export interface CombatThreeStageDiagnostic {
  code: CombatThreeStageDiagnosticCode;
  severity: "error" | "warning";
  path: string;
}
export type CombatThreeMountResult =
  | { kind: "absent"; diagnostics: readonly [] }
  | { kind: "fallback"; diagnostics: readonly CombatThreeStageDiagnostic[] }
  | { kind: "mounted"; handle: CombatThreeStageHandle; diagnostics: readonly CombatThreeStageDiagnostic[] };
export function mountCombatThreeStage(host: HTMLElement | null, combat: unknown): CombatThreeMountResult;
```

stage-owned diagnostic path는 각각 `$host`, `$host.size`, `$webgl`, `$webgl.context`로 고정하고
offending value나 exception text를 담지 않는다. adapter diagnostic은 순서와 path를 그대로
앞에 보존한다. ready payload에서 host/WebGL error가 생기면 그 뒤에 stage diagnostic 하나를
붙여 fallback한다. context loss는 mount 뒤 event이므로 host state와 sanitized one-time log에
`CONTEXT_LOST@$webgl.context`를 사용한다.

`dispose()`는 idempotent다. animation loop가 생기지 않는 WP2에서도 `setAnimationLoop(null)`,
observer/listener 해제, scene traversal을 통한 owned geometry/material/texture dispose,
`renderer.dispose()` 순서를 지킨다. test용 `forceContextLoss()`를 production dispose의 필수
동작으로 사용하지 않는다. `metrics()`는 계측용이며 gameplay/HUD에 노출하지 않는다.

WP2 implementation subagent의 **수정 가능 파일은 정확히 아래 8개**다.

- `web/package.json`
- `web/package-lock.json`
- `web/src/main.ts`
- `web/src/ui/storybook/combat/combatThreeStage.ts` (new)
- `web/src/ui/storybook/combat/combatThreeStage.test.ts` (new)
- `web/src/ui/storybook/combat/renderCombatStage.ts`
- `web/src/ui/storybook/combat/renderCombatStage.test.ts`
- `web/src/styles/storybook.css`

`web/src/ui/storybook/render.ts`는 이미 `renderCombatStage(page.combat)`을 호출하므로 **수정하지
않는다**. `web/src/core/types.ts`, `combatThreeAdapter.ts`와 그 test, `combatMotion.ts`, Rust
crate/source/test/fixture, YAML/generated bundle, WASM, terminal, story shell/HUD CSS, save key,
route graph는 전부 금지 파일이다. 금지 파일에서 부족함을 발견하면 범위를 넓히지 말고 멈춘다.

`renderCombatStage.ts`는 ready일 때 기존 semantic board 안에
`[data-combat-three-host]`를 만들고 DOM board/table/log/report를 유지한다. mounted canvas만
visual DOM board 위에 놓이며 semantic table은 계속 접근 가능하다. absent는 기존처럼 combat
markup 0개다. adapter fallback이면 raw payload를 기존 formatter로 넘기지 않고 localized
"전투 화면을 표시할 수 없습니다" surface와 첫 error의 sanitized `code`/`path` data만 만든다.
ready warning은 input order 그대로 `.combat-stage__diagnostics` semantic list에 "보드 범위 밖 위치" 또는
"한 칸 중복 점유"와 sanitized path를 표시한다. 두 경우 모두 piece를 clamp/drop/reorder하지 않는다.
`main.ts`는 replacement 뒤 그 값을 읽어 같은 `code|path`를 mount generation당 한 번만
`console.warn`하고 raw value, exception, stack을 출력하지 않는다.

WebGL palette는 mount 시 host의 computed CSS custom property만 읽는다: tile base/alternate는
`--paper-deep`/`--paper-lit`, edge는 `--ink-faded`, ally/enemy marker는 `--jade`/`--seal-red`,
collision marker는 `--gold-leaf`, clear alpha는 0이다. CSS 값을 읽을 수 없는 test environment의
fallback literal은 `combatThreeStage.ts`의 단일 palette table에만 둔다. 다른 module이나 shader에
색상 literal을 흩뜨리지 않는다.

#### scene lifecycle과 실패 정책

`main.ts`는 module-level active handle 하나만 가진다. `renderGamePage`, `renderStart`,
`renderFatalPlayerError`에서 `appRoot.innerHTML`을 실제 교체하기 **직전** active handle을 dispose하고
null로 만든다. `renderGamePage` replacement 뒤 새 host에 mount한다. 현재 transition controller가
outgoing DOM을 유지하는 동안은 stage도 유지하고, 최종 replacement 직전에만 dispose한다.
동일 handle의 double dispose, no-combat page, empty frames/pieces는 throw하지 않는다.

WebGL2 미지원/renderer constructor 실패/zero-size host는 fallback result를 내고 canvas를
ready로 표시하지 않는다. `webglcontextlost`에서는 `preventDefault()`, render 중단, canvas 숨김,
`data-three-state="fallback"` 전환으로 기존 DOM board/log/report를 즉시 드러낸다. 자동 context
restore나 GameCore 진행 중단은 하지 않는다. WP2에는 async GLB load가 없으므로 abort/generation
guard 대상도 없다. WP3이 asset load를 추가할 때 이 handle에 guard를 추가하며 stale completion은
폐기한다. forced-colors에서는 canvas를 숨기고 DOM board를 primary로 한다. reduced-motion은
정적 final frame이라 normal과 같은 resting transform이며 loop를 만들지 않는다.

#### acceptance, evidence, performance boundary

unit/integration acceptance:

1. canonical 7×6 bounds가 42개 unique tile을 만들고 네 corner `(0,0)`, `(6,0)`, `(0,5)`,
   `(6,5)`와 중앙 표본의 center가 WP1 `axialToWorld` exact 값과 일치한다.
2. PR #217 producer fixture가 `mounted`가 되고 final frame marker id/order/position이 normalized
   replay와 같으며 core fixture, input, adapter output은 mutate하지 않는다. 같은 replay/host size의
   두 mount는 mocked Three scene matrix 배열과 geometry/material/marker 수가 exact match한다.
3. absent는 host/canvas/diagnostic 0개, malformed는 throw 없이 localized fallback이며 첫
   diagnostic만 sanitized log된다. warning-only ready는 mount되고 원 좌표와 DOM warning을 보존한다.
4. empty frames/pieces는 빈 42-tile board, 13명은 hard reject 없음, duplicate occupancy는 marker
   overlap 없이 DOM에 두 id가 남는다.
5. resize 전후 camera pose/target/q축은 같고 frustum만 refit하며 RTL transform이 동일하다.
6. mount → dispose → dispose와 10회 replacement에서 observer/listener/canvas가 남지 않고
   geometry/texture count가 증가 추세를 보이지 않는다. start/fatal/no-combat 전환도 같은 gate다.
7. synthetic `webglcontextlost` 뒤 canvas가 숨고 semantic board/log가 남으며 user action button은
   동작한다. forced-colors/reduced-motion에서도 같은 정보와 final position이 남는다.
8. `render.ts`, Rust/GameCore/schema/fixture와 기존 gameplay 기대값은 byte diff 0이다.

browser evidence는 실제 WASM player 또는 같은 production mount 함수를 쓰는 fixture harness에서
남긴다. 1920×1080 DPR 1 normal screenshot, existing Storybook 5 viewport QA, reduced-motion,
forced-colors, context-loss 후 screenshot을 scratch artifact로 보존하고 commit하지 않는다. 캡처에는
42칸, four corners, final two markers, 기존 DOM log/report, context-loss 뒤 DOM fallback이 보여야
한다. production mount를 호출하지 않는 별도 mock canvas 캡처는 증거가 아니다.

WP2 성능은 budget 결정 단계가 아니다. 1920×1080 DPR 1 warm 상태에서 정적 board 120 render의
CPU frame p50/p95/p99와 `renderer.info.render.calls/triangles`,
`renderer.info.memory.geometries/textures`, 10회 mount/dispose 전후 값을 기록한다. 평균 FPS 하나,
GPU timer, shader hitch, 12명 worst-cue, shadow/postprocess 비교, hard draw/triangle/memory budget은
WP4/5/8로 남긴다. 측정값이 나쁘더라도 테스트 기대값을 완화하지 말고 원자료와 환경을 보고한다.

#### originality와 review gate

레퍼런스 팩의 autobattler 이미지는 "고정 시점에서 보드 전체가 읽힘"만, Wave-Racer는 resource
ownership/disposal과 deterministic capture 방식만 참고한다. 고유 tile pattern, 캐릭터 IP,
상점/벤치/HUD 배치, 색 조합, 로고, shader/code를 복제하지 않는다. WP2의 육각 tile과 anchor
marker는 프로젝트 palette token(`--paper*`, `--ink*`, `--jade`, `--seal-red`)과 기본 geometry로
독자 구성한다. 외부 코드/asset을 실제로 가져오게 되면 stop하고 source/commit/adapted 범위를
provenance에 먼저 추가한다.

구현자는 다음 중 하나면 즉시 멈춘다: WP1 merge SHA/API/fixture SHA가 정본과 다름, baseline
test 수치나 existing expectation이 바뀜, `render.ts`/Rust/schema/금지 파일 수정이 필요함,
WebGL fallback이 DOM 의미를 보존하지 못함, 두 가지로 읽히는 lifecycle, exact `three` version
설치 실패, fixture fingerprint/bytes 변경. 구현 완료 뒤 별도 reviewer가 owned-file diff,
dependency/lock, lifecycle disposal, raw payload 비노출, browser screenshots와 직접 재실행한 test
출력을 검수해야 한다. 그 review가 통과하기 전 WP3를 ACTIVE로 올리거나 gate flag를 해제하지 않는다.

| WP | 결과 | non-goal |
| --- | --- | --- |
| 1. contract | TS 타입, adapter, seed golden test | Three.js 구현 |
| 2. board | OrthographicCamera, 7×6 projection/edge fixture | GLB/VFX |
| 3. asset kit | shared skeleton, clips, modular GLB, missing placeholder | bespoke rig, balance |
| 4. stage | 12명 fixture, pool, toon, blob shadow, overlap QA | 영구 12명 cap |
| 5. experiments | shadow/postprocess toggle과 동일조건 캡처·계측 | 실측 전 baseline 승격 |
| 6. cue VFX | seeded primitive pool, cue mapping golden test | 새 core cue |
| 7. DOM/fallback | semantic mirror, portrait/HUD, context-loss test | canvas-only 정보 |
| 8. PC benchmark | 대표 PC p50/p95/p99와 resource artifact | 선행 측정 없는 hard budget |
| 9. mobile later | mobile DPR/payload/quality tier decision | 초기 acceptance 변경 |
| 10. provenance/content | private-study note와 publication gate | 공개 clearance 간주 |
| 11. integration QA | replay, settings, viewport, disposal 반복 QA | GameCore rewrite |

## 12. premise-collapse conditions

다음 증거가 재현되면 새 decision record를 열고 방향을 재검토한다.

- PC 1920×1080에서도 고정 OrthographicCamera, 7×6 board, 12명 상태가 읽히지 않는다.
- shared assets, instancing, pooling, disposal과 합리적 quality toggle 뒤에도 대표 PC에서 60fps 목표를 지속적으로 달성할 수 없다.
- shared skeleton/animation으로 실루엣과 무기 식별성이 유지되지 않아 bespoke rig가 baseline이 된다.
- 3-tone toon/blob shadow가 진영·상태·cue를 구분하지 못하고 실측상 shadow/postprocess가 선택 사항이 아닌 필수가 된다.
- GameCore의 deterministic frame/cue/log 계약이 없어 renderer가 판정·seed·log를 재구현해야 한다.
- fixture를 확장할 때 renderer 상수 때문에 GameCore/ScenePage 계약을 깨야 한다.
- provenance review 후 핵심 visual identity를 public-safe asset으로 전환할 수 없다.

“예쁘지 않다”만으로 collapse를 선언하지 않는다. 재현 fixture, 측정 결과, 계약 위반 증거와 폐기할 premise를 함께 기록한다.
