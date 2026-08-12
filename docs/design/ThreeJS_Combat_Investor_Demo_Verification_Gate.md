# Three.js 전투 investor-demo 검증 게이트

Status: canonical verification contract; implementation not active

Scope: `ScenePage.combat`의 production WASM 경로, Three.js 전투 renderer, DOM 대체 표면의 최종 investor-demo 증거

## 1. 목적과 권한 경계

이 문서는 전투 renderer의 새 기능이나 새 gameplay 규칙을 설계하지 않는다. 구현 WP들이 만든 결과를 investor-demo 후보로 승인할 때 필요한 **통합 증거 경로, 시나리오 행렬, artifact 형식, 합격·중단 조건**만 소유한다.

읽기 순서는 다음과 같다.

1. [Development Plan](../dev/Development_Plan.md)의 현재 ACTIVE UI track과 merge gate
2. [Three.js 전투 비주얼 아키텍처](ThreeJS_Combat_Visual_Architecture.md)의 truth ownership, 결정론, DOM/fallback, lifecycle, 성능 방향
3. [Combat System Implementation Plan Index](Combat_System_Implementation_Plan_Index.md)의 실제 producer, gate, 남은 Web QA 상태
4. [Combat contract handoff](../content/design_source/handoffs/combat_contract_handoff.md)의 identity/log/runtime handoff
5. 이 문서의 최종 증거 protocol

권한은 다음처럼 제한한다.

- Rust GameCore와 `ScenePage.combat`은 전투 판정, 좌표, 상태, tick, cue/log 순서, 승패, seed 입력을 소유한다.
- Three.js architecture는 renderer 구조와 각 WP의 구현 계약을 소유한다.
- 이 문서는 그 계약을 재정의하지 않고 최종 검증 증거의 충족 여부만 판정한다. 단, §7의 streaming log announcement는 기존의 넓은 접근성 문구와 실제 구현 계약 사이의 모순을 닫는 좁은 정본 clarification이다.
- [기존 Storybook QA script](../../web/scripts/storybook-reference-qa.mjs)는 일반 Storybook shell 회귀 검증을 계속 소유한다. 이 문서는 그 script를 수정하거나 전투 전용 QA가 이미 존재한다고 선언하지 않는다.
- gameplay encounter gate 제거, 표시 이름 schema, 전투 balance/AI, save schema 변경은 이 문서의 권한 밖이다.

## 2. 현재 증거의 한계

현재 generic Storybook QA는 다섯 viewport에서 fresh storage로 새 게임을 시작하고 첫 일반 선택지, shell/HUD, drawer, click/keyboard 전환을 확인한다. 모든 browser context가 `reducedMotion: "reduce"`이며, 일반 플레이가 세우지 않는 `combat_spectator_preview_unlocked` flag 뒤의 전투 producer에는 도달하지 않는다. 따라서 이 결과는 다음을 증명하지 않는다.

- production WASM이 실제 `ScenePage.combat`을 만들고 production Three mount까지 전달하는지
- normal-motion replay가 core frame/cue/log와 동기화되는지
- WebGL unavailable/context loss 뒤 DOM fallback과 user action이 유지되는지
- forced colors, semantic log manual reading, 반복 mount/dispose, stale async completion이 안전한지
- 12-unit worst-cue의 PC 성능과 자원 추세가 어떤지

`renderCombatStage()`를 직접 호출한 HTML, 임시 harness에 붙인 handwritten combat JSON, unit-test mock renderer, synthetic canvas screenshot은 국소 검증에는 유효하다. 그러나 production start/continue, Rust/WASM `scene_page_json`, production page render와 production Three mount를 우회하므로 **최종 investor-demo E2E 증거가 아니다**. 기존 synthetic/direct-render 증거를 폐기한다는 뜻은 아니며, 최종 gate를 대신할 수 없다는 뜻이다.

## 3. production WASM route-only 최종 증거

최종 E2E run은 실제 Web player 문서와 `main.ts`의 production start/continue 흐름을 사용해야 한다. 검사 code가 production 내부 render/mount 함수를 직접 호출하거나 `ScenePage.combat`을 주입해서는 안 된다.

전투 진입에는 향후 Rust public generator가 만든 **serialized `GameState` save fixture**를 사용한다. fixture는 canonical run seed와 전용 gate flag를 포함하고, 현재 player가 사용하는 production Rust-save storage boundary에 bytes 그대로 설치한다. 이후 보이는 start screen의 continue action을 사용해 다음 경로를 통과해야 한다.

```text
start screen
  -> production continue action
  -> serialized GameState load
  -> production WASM scene_page_json
  -> actual encounter producer
  -> ScenePage.combat
  -> renderStorybookPage
  -> production Three mount + persistent semantic DOM
```

QA-only runtime backdoor, exported debug setter, renderer-only query parameter로 GameCore state나 `ScenePage.combat`을 주입하지 않는다. fixture를 이용한 start/continue는 gameplay gate를 제거하거나 일반 플레이에서 flag를 획득 가능하게 만들지 않는다.

### 3.1 아직 존재하지 않는 필수 선행물

이 문서를 작성한 기준 main에는 위 serialized `GameState` save fixture와 그 public Rust generator/golden test가 없다. PR #217이 제안한 2-unit combat payload fixture도 아직 main의 canonical artifact가 아니며, 7×6 edge fixture와 12-unit worst-cue fixture도 없다. save fixture, edge fixture, worst-cue fixture는 모두 **future producer-owned fixture**이고, 2-unit payload는 #217 review/merge 뒤에만 canonical input으로 취급한다.

각 fixture에는 다음이 먼저 있어야 한다.

- repository에서 재실행할 수 있는 public Rust generator 또는 public producer/builder path
- checked-in bytes와 byte-for-byte golden test
- simulation version, producer/build input, stable fingerprint 또는 byte digest
- Web test가 별도 handwritten copy 없이 같은 canonical artifact를 소비하는 경로

public Rust generator와 golden이 없는 handwritten `GameState`, handwritten final `ScenePage.combat`, renderer-local JSON copy로는 이 gate의 acceptance를 주장할 수 없다. fixture 생성에 renderer가 좌표, log, cue, seed, 승패를 발명해야 하면 즉시 중단한다.

## 4. 필수 시나리오 행렬

| ID | 입력과 현재 상태 | 검증 목적 | 최종 증거 경로 |
| --- | --- | --- | --- |
| `no-combat` | production 새 게임의 `combat` absent page | combat host/canvas/diagnostic 0, 기존 story action parity | production start/new-game/WASM |
| `producer-2` | canonical 2-unit producer smoke; PR #217이 payload golden을 제안했으나 아직 main에 없고 save fixture도 없음 | actual producer→WASM→page→mount, core order/fingerprint 보존 | #217 merge 뒤 payload recheck + future Rust-generated save fixture + continue |
| `edge-7x6` | **future producer-owned**, 네 corner와 중앙을 포함하는 7×6 edge fixture | projection, fixed camera, RTL q 의미, DOM 좌표 parity | public Rust generator/golden + continue |
| `worst-cue-12` | **future producer-owned**, 12-unit 대표 worst-cue fixture | 혼전 가독성, 결정론, PC 성능/자원 추세 | public Rust generator/golden + continue |
| `hard-malformed` | adapter contract가 정한 consumed-field hard error | throw 없는 localized fallback, raw payload/stack 비노출, story action 유지 | production integration seam; raw payload를 DOM formatter에 전달 금지 |
| `webgl-unavailable` | browser가 WebGL context 생성을 거부 | canvas ready 오표시 없음, semantic DOM primary, action 계속 가능 | production page + browser capability interception |
| `context-loss` | mounted production canvas의 synthetic `webglcontextlost` | 즉시 DOM fallback, render 중단, GameCore/action 지속 | production mount에 browser event 전달 |

`hard-malformed`만은 Rust producer의 정상 output이 아니므로 producer fixture가 아니다. 이 행은 production integration이 adapter fallback result를 안전하게 연결하는지를 검사하며, malformed 값을 final combat truth처럼 표시하거나 GameCore가 만들었다고 주장하지 않는다.

## 5. lifecycle과 복구 순서

각 run은 isolated browser context와 기록된 build에서 시작한다. 다음 순서를 임의로 생략하거나 direct-render harness로 대체하지 않는다.

1. **No-combat baseline:** empty storage의 start screen에서 production new-game을 선택하고 no-combat page에 combat host/canvas가 없으며 keyboard action이 동작함을 확인한다.
2. **Combat continue:** 새 isolated context에 Rust-generated save fixture bytes만 production save boundary로 설치한다. start screen에서 보이는 continue를 선택하고 actual producer fingerprint, combat DOM, Three canvas가 함께 나타나는지 확인한다.
3. **Combat action:** keyboard-only로 core가 제공한 action을 실행해 combat page에서 no-combat page로 이동한다. replacement 직전 old stage가 dispose되고 replacement 뒤 canvas/listener/observer가 남지 않는지 기록한다.
4. **Start replacement:** combat fixture로 다시 진입한 뒤 production menu/new-game action으로 start screen을 연다. combat canvas와 active handle이 남지 않아야 한다.
5. **Fatal replacement:** production continue로 combat을 mount한 같은 document/player lifetime에서 실제 production fatal replacement를 일으키고 old stage dispose와 fatal surface를 확인한다. 내부 fatal 함수 직접 호출, debug fatal action, production 내부 함수 export는 금지한다. 현재 production UI로 이 상태를 재현할 수 없으면 page reload나 browser-context 종료로 통과를 꾸미지 말고, lifecycle testability decision이 필요하다고 중단한다.
6. **Context loss:** combat fixture로 production mount한 뒤 synthetic context-loss event를 전달한다. canvas는 숨거나 제거되고 semantic board, portrait가 존재하면 portrait, ordered log/report, user action이 유지되어야 한다. 자동 context restore를 acceptance로 요구하지 않는다.
7. **Stale async:** asset WP가 async load를 추가한 뒤에는 한 resource completion을 의도적으로 지연하고 combat→no-combat 또는 combat→start replacement를 끝낸 다음 old completion을 해제한다. 이전 generation의 object, listener, canvas가 새 scene에 attach되면 실패다. async load가 아직 없는 WP는 이 항목을 통과로 세지 않고 `not-applicable-before-async-assets`로 기록한다.
8. **Ten-cycle stability:** `combat continue -> keyboard action -> no-combat`, `combat continue -> start`, `combat continue -> production fatal replacement`를 각각 10회 반복한다. 각 cycle은 같은 document/player lifetime에서 되돌아와 다음 combat mount를 시작해야 한다. 각 cycle 뒤 canvas, active listener/observer, Three geometry/material/texture, JS heap sample을 기록한다. 단조 증가 추세나 이전 generation attachment가 있으면 실패다.

브라우저 context 종료, full page reload, fresh document로 누수를 숨기지 않는다. 세 replacement leg의 10회 반복은 각각 같은 context와 document/player lifetime 안에서 수행한다. Production fatal replacement 뒤 정상 surface로 되돌아오는 경로가 없다면 10-cycle gate는 실행 불가이며 acceptance가 아니라 stop condition이다.

## 6. 결정론 replay 증거

`producer-2`, `edge-7x6`, `worst-cue-12`는 같은 build, fixture bytes, viewport, DPR, quality setting에서 각각 두 번 replay한다. structured trace는 다음 항목을 exact 비교한다.

- fixture digest/fingerprint와 `simulation_version`
- frame tick, piece id/order, axial coordinate, cue type/order와 visual seed
- scene graph object kind/order, world transform과 camera pose/frustum
- primitive/particle count와 `renderer.info`의 구조적 count
- semantic table과 core log/report 순서

wall clock, `Math.random()`, viewport, DPR, GPU 결과를 visual seed에 섞지 않는다. screenshot은 visual regression 검토 자료이며 다른 GPU의 raster byte equality를 결정론 계약으로 삼지 않는다. structured trace가 다르거나 동일 환경의 캡처 차이를 설명할 수 없으면 acceptance를 멈춘다.

## 7. 접근성·설정 증거와 `aria-live` clarification

같은 production route에 대해 다음을 별도 run으로 남긴다. 기존 generic QA의 reduced-motion run 하나를 normal과 reduced 양쪽 증거로 재사용하지 않는다.

- `normal`: 1920×1080, DPR 1, normal motion에서 replay/cue/log timing과 keyboard action
- `reduced-motion`: 같은 fixture의 resting information, final position, action parity; 의미 있는 정보 손실 없음
- `forced-colors`: canvas가 유일한 정보 표면이 아니며 semantic board/table/log와 visible focus가 유지됨
- `keyboard-only`: start/continue, combat action, no-combat/start 전환, context-loss 뒤 action을 pointer 없이 완료
- `semantic-manual-read`: screen reader 또는 accessibility tree로 board description, display name/declared fallback label, side, axial coordinate, state, last cue, ordered core/full log를 사용자가 직접 탐색해 읽을 수 있음

Streaming combat log container에는 `aria-live`를 두지 않는다. 여러 tick의 log를 연속 announce해 사용자의 읽기를 빼앗지 않고, ordered semantic list를 처음부터 manual-read surface로 유지한다. 이는 현재 streaming log에 대한 canonical clarification이며, 단순히 `aria-live="polite"` 존재 여부를 합격 조건으로 쓰지 않는다.

향후 live announcement가 필요하면 GameCore가 제공한 terminal 또는 decisive summary만 별도 bounded status region에서 announce할 수 있다. renderer가 raw events를 묶어 새 요약, 중요도, 승패, 생존 상태를 추론해서는 안 된다. core-authored summary field와 announce 빈도 계약이 없으면 새 live region을 만들지 않는다.

사용자 문장과 accessibility label에는 internal combatant id를 노출하지 않는다. canonical display name이 아직 runtime에 없으면 identity handoff가 정한 declared fallback을 사용하며 renderer가 이름을 발명하지 않는다. 그 경로도 없으면 final demo acceptance를 중단한다.

## 8. PC 성능 증거 schema

성능 증거는 `worst-cue-12` production route를 대표 시나리오로 사용한다. `producer-2` 결과만으로 investor-demo 성능을 승인하지 않는다. 결과 JSON은 최소 다음 필드를 가진다.

```text
schema_version
run_id, generated_at_utc, git_commit, build_kind
scenario_id, fixture_digest, simulation_version
environment:
  os, cpu, gpu, gpu_driver, browser, browser_version
  power_mode, thermal_state, display_refresh_hz
  viewport_width, viewport_height, device_pixel_ratio
  quality_flags, browser_flags
measurement:
  cold_or_warm, warmup_frames, warmup_millis
  sample_count, sample_duration_millis
  frame_time_ms: { p50, p95, p99, max }
  long_frames: { over_16_67_ms, over_33_33_ms, refs }
renderer_info:
  calls, triangles, points, lines, geometries, textures, programs
memory:
  js_heap_before, js_heap_after, js_heap_peak
resource_trend:
  cycle_count, per_cycle_refs, monotonic_growth_detected
raw_refs, capture_pair_refs, notes
```

Cold load와 warm replay를 분리한다. 각 값과 함께 warmup frame/time, sample count/duration, raw frame-time sample reference를 기록한다. percentile을 평균 FPS 하나로 대체하지 않는다. browser/GPU timing API가 제공하지 않는 값은 `unavailable`과 이유를 기록하며 추정값을 측정값처럼 쓰지 않는다.

`capture_pair_refs`는 같은 run의 steady-state combat와 context-loss 뒤 DOM fallback 캡처를 가리킨다. 비교 실험 WP에서는 같은 fixture/environment의 baseline과 enabled variant를 추가 pair로 남긴다.

정상 상태 60fps는 목표이며, 16.67ms 초과 frame count와 p95/p99 악화는 reviewer가 원자료를 확인해야 하는 **review trigger**다. 이 문서는 draw call, triangle, memory, percentile의 hard pass number를 발명하지 않는다. 대표 측정과 별도 performance decision record 없이 수치를 완화하거나 hard budget으로 승격하지 않는다.

## 9. artifact 이름과 보존

artifact root는 다음 형식을 사용한다.

```text
<out-dir>/threejs-combat-demo-gate/<UTC-basic>-<git-short-commit>/
```

최소 파일은 `report.json`, `raw/frame-times.json`, `raw/lifecycle-cycles.json`, `traces/<scenario>-run-1.json`, `traces/<scenario>-run-2.json`, `screenshots/<scenario>-steady.png`, `screenshots/<scenario>-fallback.png`다. 파일 reference는 `report.json` 기준 상대 경로이며 report가 fixture digest, build commit, environment와 각 acceptance 결과를 연결한다.

이 artifact는 scratch 또는 CI artifact이며 repository에 commit하지 않는다. reviewer sign-off가 끝날 때까지 보존한다. 같은 commit/scenario의 새 run으로 교체할 때도 실패 artifact를 조용히 덮어쓰지 않고 새 `run_id` directory를 만든다. CI 보존 기간이 별도로 정해져 있으면 report에 그 policy와 만료 시점을 기록한다. one-off HTML/harness와 로컬 절대 경로를 canonical evidence로 링크하지 않는다.

## 10. acceptance gate

다음을 모두 만족하기 전에는 “investor-demo combat gate 통과”라고 쓰지 않는다.

1. `producer-2`, `edge-7x6`, `worst-cue-12` 각각에 public Rust generator/producer path와 checked-in golden이 있고, final browser evidence가 production start/continue→WASM route를 통과한다.
2. no-combat, hard-malformed, WebGL unavailable, context-loss 시나리오가 story content와 user action을 보존하며 raw payload, internal stack, internal combatant id를 노출하지 않는다.
3. 두 replay의 structured determinism trace가 exact match하고 입력 fixture와 adapter output을 mutate하지 않는다.
4. normal, reduced motion, forced colors, keyboard-only, semantic manual-read log 증거가 별도 결과로 남는다. Streaming log에는 `aria-live`가 없고 renderer가 terminal/decisive 의미를 추론하지 않는다.
5. lifecycle 순서와 각 10-cycle gate에서 stale attachment, 남은 canvas/listener/observer, 증가하는 Three resource/heap 추세가 없다.
6. 12-unit worst-cue의 cold/warm performance artifact가 §8 schema를 만족하고 raw sample과 capture pair를 포함한다. 60fps target miss는 숨기거나 threshold를 바꾸지 않고 review 결과와 함께 기록한다.
7. artifact naming/retention이 §9를 따르고 별도 reviewer가 production route, fixture provenance, fallback/action continuity, raw performance refs를 직접 대조한다.

## 11. forbidden changes와 stop conditions

이 검증 계약과 그 후속 QA에서 금지한다.

- Rust/GameCore 판정, 좌표, cue/log, 승패, seed를 renderer나 test harness에서 재계산·수정·재정렬
- handwritten final `ScenePage.combat`, renderer-local fixture copy, direct-render harness를 final E2E 증거로 사용
- 일반 gameplay gate flag 제거 또는 획득 경로 추가
- 표시 이름, save schema, encounter YAML, balance/AI를 검증 편의를 위해 변경
- debug state setter, debug combat route, production 내부 render/mount 함수 export를 추가
- 대표 측정 전 hard performance budget을 만들거나 실패 뒤 expectation을 완화
- reference pack의 고유 tile pattern, 캐릭터, HUD 배치, 색 조합, logo, shader/code를 복제
- screenshot, raw performance dump, one-off report/harness를 repository에 commit

다음 중 하나면 범위를 넓히지 말고 중단해 reviewer에게 보고한다.

- public Rust save generator/golden 없이 fixture를 손으로 만들어야 함
- 7×6 edge 또는 12-unit worst-cue fixture의 producer/builder owner가 없음
- reviewed/merged adapter 또는 stage 구현의 exact public API, diagnostic order, fixture fingerprint가 이 문서를 적용할 때의 가정과 다름
- production start/continue 경로가 아닌 runtime backdoor가 있어야 combat에 도달함
- same-document production fatal replacement와 복귀를 debug hook 없이 재현할 수 없어 10-cycle fatal leg를 실행할 수 없음
- identity fallback 부재로 internal id가 사용자 또는 accessibility label에 노출됨
- streaming log announcement를 위해 renderer가 terminal/decisive 의미를 추론해야 함
- context loss나 replacement 뒤 DOM/action continuity 또는 stale async prevention을 보장할 수 없음
- 대표 환경, raw samples, lifecycle resource trend를 기록할 수 없음
- evidence를 만들기 위해 gameplay gate 제거, Rust truth 변경, renderer 범위 밖 schema 변경이 필요함

## 12. PR/WP dependency와 merge 해석

- PR #217의 adapter/producer fixture는 이 gate의 입력 선행물이다. 문서 계약 자체는 미병합 API나 SHA를 최종값으로 이름 붙이지 않는다. #217 review/merge 뒤 exact public API, diagnostic ordering, fixture bytes/fingerprint를 다시 읽고 불일치하면 증거 실행 전에 중단한다.
- PR #220은 WP2 board 계약 문서다. 그 문서의 merge만으로 production Three stage가 존재하거나 이 gate가 통과한 것이 아니다. #220이 승인한 후속 WP2 **구현**이 review/merge되고 production mount/lifecycle이 존재해야 board, WebGL, context-loss 증거를 실행할 수 있다. exact stage API는 그 구현 merge 뒤 다시 확인한다.
- PR #219와 #221의 intervention authoring/provenance track은 이 검증 문서의 작성·merge와 독립이다. 이 문서는 그 PR들의 pause/transaction/terminal semantics를 흡수하거나 완료로 주장하지 않는다.
- 미래 intervention UI가 investor-demo 범위에 들어오면 별도 approved scenario를 추가한다. 그 전에는 현재 static/systemic combat gate를 I2 lifecycle E2E로 과장하지 않는다.

이 문서는 어떤 PR의 merge, 어떤 renderer WP의 완료, 어떤 fixture의 존재도 대신 선언하지 않는다. 특히 gameplay gate는 유지하며, 이 문서가 있다는 이유로 gate removal 또는 investor-demo readiness를 주장할 수 없다.
