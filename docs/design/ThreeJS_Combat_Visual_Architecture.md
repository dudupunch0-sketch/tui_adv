# Three.js 전투 비주얼 아키텍처

Status: approved canonical direction, 2026-08-12
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
| 보드 | 첫 검증 fixture는 7×6 flat-top axial hex, 42칸. `q`는 좌우 축, `r`은 대각 축. 영구 gameplay cap이 아니다. |
| 전투원 | 첫 검증 fixture는 동시 12명. 영구 roster/encounter cap이 아니며 칸 점유와 상태는 GameCore가 결정. |
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
renderer는 각 cue value를 `cue_type`, 배열 index를 파생 `cue_ordinal`로 정규화하고
`(simulation_version, resolution_fingerprint, fingerprint, tick, combatant_id, cue_type, cue_ordinal)`
tuple로 visual seed를 만든다. `cue_ordinal`은 wire field가 아니다. renderer가 별도의
`run_seed`, `encounter_id`, `combat_id`를 추측하거나 합성하지 않는다.

동일 tuple은 동일한 primitive, 방향, 위상, 색상, particle count를 만든다. GameCore 결정론 해시를 사용하고 `Math.random()`·wall-clock을 섞지 않는다. viewport, GPU, DPR, quality는 seed 밖이다.

## 4. 보드 투영과 캐릭터

첫 fixture는 `q ∈ [0, 6]`, `r ∈ [0, 5]`다. 확장 시 선언된 board bounds를 읽으며 7×6을 영구 상수로 두지 않는다. q 방향은 RTL에서도 고정한다. flat-top 투영은 다음과 같다.

```text
x = size * (3/2 * q)
z = size * sqrt(3) * (r + q/2)
```

칸 중심이 character/contact-shadow anchor다. Orthographic frustum은 viewport에 맞춰 크기만 갱신한다. 보드 밖 좌표, 중복 점유, 알 수 없는 진영은 판정을 고치지 않고 diagnostic과 DOM으로 처리한다.

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
| pointy-top 또는 pixel hex | flat-top axial, 첫 fixture 7×6 |
| 전투원별 rig/animation | shared skeleton/common animations/modular parts |
| 얼굴 표정이 감정의 진실 | authored 2D portrait와 core cue |
| 모바일 30fps 및 고정 저예산을 초기 기준 | PC-first 60fps 목표, 측정 후 budget 확정 |
| shadow/postprocessing을 실측 없이 금지하거나 필수화 | 3-tone/blob baseline, shadow/postprocess는 비교 실험 후 결정 |
| canvas가 유일한 정보 surface | DOM semantic mirror와 failure fallback 필수 |

GameCore 판정 소유권, renderer의 seed·AI·log 재구현 금지, `ScenePage.combat` additive-optional은 superseded되지 않는다.

## 10. acceptance tests

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
