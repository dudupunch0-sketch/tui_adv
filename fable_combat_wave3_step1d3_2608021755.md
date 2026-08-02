# Combat Wave 3 — Step 1d-3 설계 플랜: Web 관전 재생 연출

Baseline: PR #183 머지 커밋 (`claude/combat-wave3-step1d2`)
선행 플랜: `fable_combat_wave3_step1d2_2608021618.md`
브랜치(예정): `claude/combat-wave3-step1d3`

## 0. 이 슬라이스와 게이트

1d-2가 정지 프레임을 만들었다. 이 슬라이스가 **재생 연출**을 얹어 정본 13의
"상단은 실제 실시간 전투 시뮬레이션"을 충족시킨다.

**게이트 플래그 `combat_spectator_preview_unlocked`는 이 슬라이스에서도 풀지
않는다.** 1d-2에서 발견한 갭 때문이다 — 관전 화면과 보고서가 전투원의 내부
id(`wuxia_spectator_bout_ally`)를 그대로 플레이어에게 보여준다. 표시 이름
필드가 `CombatSpectatorPiece`·`CombatCombatantReport`에도, 인카운터 combat
authoring에도 없어서 renderer가 유도할 수 없다. 이름을 발명하지 않는다.

따라서 순서는 이렇게 된다:

| 슬라이스 | 내용 |
|---|---|
| 1d-1 | terminal 관전 렌더러 (완료, PR #182) |
| 1d-2 | Web 정지 프레임 + 로그 + 보고서 + TS 타입 (완료, PR #183) |
| **1d-3 (이 플랜)** | **Web 재생 연출 + 로그 동시 노출** (게이트 유지) |
| 1d-4 | 전투원 표시 이름 (core schema + authoring) → 게이트 제거 → wasm 재빌드 → 5뷰포트 실화면 QA |

게이트가 남아 있는 동안 `npm run qa:storybook:visual`은 이 화면에 도달할 수
없다. 1d-2와 같이 오케스트레이터가 임시 하네스 페이지로 실측한다(§7).

## 1. 정본 근거 (이번에 확정한 것)

### 정본 13 — 중요 순간 표현 (원문 기준 하드 제약)

- **"시간 조작은 금지한다. 순간 정지와 슬로모션을 사용하지 않으며, 진동·
  플리커·감광·음향·이펙트만 사용한다."**
- **"중요 로그의 잠시 고정은 허용한다."**
- **"시뮬레이션 시간과 화면 시간은 항상 일치하고, 연출은 판정 뒤 표현만
  담당하여 seed·판정·AI 순서에 영향하지 않는다."**

→ 재생 총 길이는 **정확히** `(frames.length - 1) × view.tick_millis` ms다.
연출을 위해 늘이거나 줄이지 않는다. 테스트로 고정한다.

### 정본 13 — 공용 연출 문법

- 공격 = 짧은 전진/복귀, 피격 = 밀림/진동, 회피 = 측면 이동,
  균형 붕괴 = 흔들림/기울어짐, 전투불능 = 흐려짐/표식
- "공용 좌표 이동·진동·전후 움직임·플리커·감광과 기술/상태 이펙트로 자산
  비용을 절감한다" — 새 이미지·SVG 파일을 만들지 않는다.
- "상단 연출과 하단 로그는 색·아이콘을 동기화한다" (1d-2의 I11 규칙 유지:
  증명 가능한 짝만 연결).

### 정본 09 — 화면 축 계약 (**새로 확인. 지금 코드가 보장하지 않는다**)

- **"화면 왼쪽: 아군 영역 / 화면 오른쪽: 적 영역"**
- 아군 전방 = 아군 영역의 오른쪽, 적 전방 = 적 영역의 왼쪽
- **"측면: 화면 위·아래"**
- "플레이어 UI는 좌표 대신 전술 구역으로 상황을 읽게 한다."
- "관측된 적의 위치는 정확히 보여 주고, **은폐 적과 증원은 숨긴다**."
- "평상시 화면은 전투 연출을 우선한다. 역할, 목표, 압박, 혼란, 고립, 지원·
  공격 범위, 후방 각도는 배치·선택·위기·개입 시에만 오버레이로 강조한다."
  → 이 슬라이스는 **상시 오버레이를 추가하지 않는다.**

현재 보드는 좌표를 그대로 비례 투영한다. **실측 결과 이 계약이 재생 중에
깨진다**: 저작 시점에는 아군 x=0 / 도전자 x=5로 맞지만, 프레임을 덤프해 보면
아군 x가 1,2,3,2,3,… 도전자 x가 4,3,2,3,2,… 로 진동하며 **tick 3부터 아군이
도전자보다 오른쪽에 놓인다**. 역할 가중치 `preferred_distance: 0`이 두 말을
서로 통과시킨다.

→ 고칠 곳은 **AI·충돌 규칙**이며 이 슬라이스(web) 범위가 아니다. 렌더러가
좌표를 왜곡해 맞추면 거리 읽기가 망가진다. 저작 시점 좌표만 검사하는
테스트로도 잡히지 않는다(저작 값은 계약을 지킨다). 인덱스의 core 결함
블록에 기록했고, 이 슬라이스는 **재생하면 좌우가 뒤바뀌는 것이 보인다는 사실
자체를 문서에 남긴다** — 게이트를 유지하는 또 하나의 근거다.

### 정본 09/13 — 일시정지의 두 의미 (충돌 아님)

- 정본 13: 연출로서의 **순간 정지·슬로모션 금지**.
- 정본 09: **"감독 전투에서는 개입 순간 시뮬레이션을 완전히 일시정지하고
  선택 후 재개한다."**

둘은 층이 다르다 — 연출용 시간 조작은 금지, 개입 선택을 위한 게임 흐름
정지는 필수다. 개입 정지는 Step 2c 소관이며 이 슬라이스는 만들지 않는다.
이 슬라이스가 만드는 재생은 **개입 예산 0인 시스템형** 전투의 통짜 재생이다.

### 정본 13 — 배속·즉시 결과 (이 슬라이스 범위 밖, 계약만 기록)

- 일반 관전 1배속 / 고속 관전 2배속 / 즉시 결과 / 자동 전투를 구분한다.
- "배속과 즉시 결과도 동일 seed, AI, 기술 순서, 피해·상태 판정과 전체 로그를
  사용한다. **처리·표현 속도만 달라진다.**"
- "승률로 버튼을 잠금/해제하지 않는다."

→ 이 슬라이스는 **1배속만** 구현한다. core에 이미 표현 속도 enum이 있으므로
(`combat_execution.rs`), 2배속·즉시 결과 UI는 별도 슬라이스에서 그 enum에
연결한다. 승률 기반 잠금은 만들지 않는다.

## 2. Scope

1d-2의 DOM·CSS 변수 위에 데이터 구동 모션을 얹는다.

1. 프레임 배열 → 말별 `@keyframes` 생성 (렌더 시 `<style>` 블록으로 방출).
2. `core_log` 줄을 tick 시각에 맞춰 노출 (`animation-delay`).
3. cue별 연출(진동·측면 이동·기울어짐·감광)을 공용 문법대로 적용.
4. `prefers-reduced-motion: reduce`에서는 **1d-2의 정지 프레임이 그대로 최종
   상태**다 (모션 선언 전체가 `no-preference` 안에만 들어간다).
5. 축 계약 authoring 테스트.

## 3. Hard invariants

### I1. 화면 시간 = 시뮬레이션 시간

재생 총 길이는 정확히 `(frames.length - 1) × tick_millis` ms다. 프레임 k의
키프레임 오프셋은 `k / (frames.length - 1) × 100%`다. 임의의 여유·이징으로
총 길이를 바꾸지 않는다.

- `frames.length <= 1`이면 재생할 것이 없다 → 모션 선언을 만들지 않고 정지
  프레임만 남긴다 (0으로 나누지 않는다).
- `tick_millis`는 core가 준다. 없거나 0이면 core가 이미 `MissingProvenance`로
  거부하므로 renderer가 기본값을 지어내지 않는다.

### I2. 시간 조작 금지

- `animation-play-state: paused`로 연출용 정지를 만들지 않는다.
- 구간별로 재생 속도를 바꾸지 않는다 (슬로모션 금지). 이징은 말의 이동
  보간에만 쓰고, 전체 타임라인은 선형이다.
- 허용: 진동, 플리커, 감광(opacity/filter), 이펙트, **중요 로그의 잠시 고정**.

### I3. reduced-motion 경로가 1d-2 출력과 동일

모든 `animation`/`transition` 선언은 `@media (prefers-reduced-motion:
no-preference)` 안에만 둔다. `reduce`에서는 1d-2가 만든 정지 프레임(마지막
프레임 위치, 전체 로그 즉시 노출)이 그대로 최종 상태다.

- 생성된 `<style>` 블록 자체도 `@media (prefers-reduced-motion:
  no-preference)`로 감싼다.
- 테스트: `reduce`용 경로에서 `--piece-x`/`--piece-y` 인라인 값이 **마지막
  프레임 좌표**임을 고정한다 (1d-2 테스트가 이미 고정하고 있다 — 깨뜨리지 말
  것).

### I4. 판정 재계산 0회 (1d-2에서 이어짐)

TS는 프레임·로그 배열을 좌표·시각으로 옮기기만 한다. cue를 파생하지 않고,
어떤 tick에 무엇이 일어났는지 다시 계산하지 않는다.

### I5. 결정론적 출력

같은 `CombatSpectatorPage`에서 같은 HTML·같은 CSS 문자열이 나온다.
`Math.random()`, `Date.now()`, 실행 순서 의존을 쓰지 않는다. 생성되는
keyframe 이름은 말 id에서 결정론적으로 만들고 **CSS 식별자로 안전하게
새니타이즈**한다 (id에 `<`, 공백, 콜론 등이 들어올 수 있다 — 1d-2의
이스케이프 테스트와 같은 이유다).

### I6. 로그 노출은 tick 시각에 맞춘다

`core_log` 각 줄의 노출 시각 = `entry.tick × tick_millis`.
- 같은 tick의 여러 줄은 `sequence` 순서를 유지한다 (core가 정한 순서를
  바꾸지 않는다).
- `reduce`에서는 전부 즉시 보인다.
- 노출 전에도 **DOM에서 제거하지 않는다** — `opacity`만 다룬다. 스크린리더가
  전체 로그를 읽을 수 있어야 하고, `full_log` 개수 표시와 어긋나지 않아야
  한다.
- `aria-live`를 쓰지 않는다. 초당 여러 줄이 붙으면 스크린리더 도배가 된다
  (정본 13의 "로그 도배를 막는다"와 같은 취지). 로그 영역은 `aria-label`만
  유지한다.

### I7. 수묵 토큰만, 신규 색상 리터럴 0개

1d-2의 I2를 그대로 유지한다. 새 이미지·SVG 파일도 만들지 않는다
(정본 13: 공용 좌표 이동·진동으로 자산 비용 절감).

### I8. 3분할 레이아웃과 70:30 유지

1d-2의 I3·I4를 그대로 유지한다. `.storybook-shell` / `.game-viewport` /
`.game-topbar` / `.storybook-hud` / `.storybook-dock` 무수정.

### I9. 성능

- 애니메이션은 `translate`·`opacity`·`filter`만 쓴다 (`left`/`top` 애니메이션
  금지 — 1d-2가 위치를 `left`/`top`으로 잡았으므로, 모션은 그 위에
  `translate`로 얹는다).
- 말 수는 정본 09 상한이 12명(아군 4 + 적 8)이다. 현재 인카운터는 2명이지만
  생성되는 keyframe 수가 말 수에 비례함을 주석에 남긴다.

### I10. 상시 오버레이 금지

역할·목표·압박·혼란·고립·범위·후방 각도 오버레이를 만들지 않는다
(정본 09: 배치·선택·위기·개입 시에만). 이 슬라이스는 배치·개입 UI가 없다.

### I11. 건드리지 않는 것

- `crates/` — **예외: WP5의 authoring 테스트 1개만** (§WP5의 근거 참조)
- 게이트 플래그 (제거 금지)
- 두 번들 JSON, `src/tui_adv/**` YAML
- `crates/escape-terminal/**` (terminal 렌더러는 정적 스냅샷이며 모션 개념이
  없다), `crates/escape-terminal/tests/cli_smoke.rs`
- `.claude/worktrees/`
- `web/package.json` — 의존성 추가 금지 (애니메이션 라이브러리 금지)

## 4. 구현 방향

### 4-1. 왜 CSS 생성 keyframe인가 (WAAPI 대신)

`renderStorybookPage`는 HTML **문자열**을 만든다. 마운트 훅이 없다.
`element.animate()`(WAAPI)를 쓰려면 `web/src/main.ts` 배선이 필요하고,
`prefers-reduced-motion`을 JS로 직접 확인해야 하며, jsdom 단위 테스트로
검증하기 어렵다.

데이터에서 만든 `@keyframes`를 렌더 결과에 함께 방출하면:
- 문자열 렌더러 구조를 그대로 쓴다 (main.ts 무변경).
- `prefers-reduced-motion`을 미디어 쿼리로 네이티브 처리한다.
- 생성된 CSS 문자열이 결정론적이므로 **단위 테스트로 총 길이·오프셋·
  waypoint 좌표를 그대로 고정**할 수 있다.

대가: `<style>` 요소가 `<body>` 안에 들어간다. HTML 스펙상 `style`은 metadata
content이므로 body 안에서는 엄격히는 비적합이다. 모든 브라우저가 적용하며
기능 문제는 없다. **이 트레이드오프를 코드 주석과 문서에 남긴다.**

### 4-2. 위치 보간

1d-2는 위치를 `left: var(--piece-x); top: var(--piece-y)` + `translate: -50%
-50%`로 잡는다. 모션은 그 위에 얹는다 — `left`/`top`을 애니메이션하지 않고
(레이아웃 속성이라 매 프레임 리플로가 난다) 말의 **오프셋**을 `translate`로
움직인다:

```
translate: calc(-50% + var(--dx)) calc(-50% + var(--dy));
```

**`--dx`/`--dy`의 단위는 `%`가 아니다.** `translate`의 백분율은 컨테이너가
아니라 **요소 자기 크기** 기준으로 해석되므로, 보드 좌표 오프셋을 `%`로 쓰면
말 크기에 비례하는 엉뚱한 이동이 된다. 대신 **컨테이너 쿼리 단위**를 쓴다:

- `.combat-stage__board`에 `container-type: size`를 준다 (그리드 행이라 높이가
  내용과 무관하게 정해지므로 size containment가 안전하다).
- x 오프셋은 `cqw`, y 오프셋은 `cqh`로 표현한다. 즉 보드 폭/높이의 백분율이다.

주의: 1d-2는 말 크기에 `12cqi`/`15cqi`를 쓰고 이 값이 지금은
`.combat-stage`(inline-size 컨테이너) 기준으로 해석된다. 보드에
`container-type: size`를 주면 가장 가까운 컨테이너가 보드로 바뀌어 말이 조금
작아진다. 의도된 변화이며, 실측으로 크기를 확인하고 필요하면 계수를 조정한다.

정지 상태(`reduce`)에서 `--dx`/`--dy`는 0이며 말은 1d-2가 계산한 마지막 프레임
위치에 그대로 있다. 애니메이션은 `-offset → 0`으로 끝나므로 재생 종료 상태와
reduced-motion 정지 상태가 정확히 같다 (`animation-fill-mode`에 의존하지
않는다).

즉 **기준점은 마지막 프레임**이고 keyframe은 각 tick의 좌표를 마지막 프레임
대비 상대 오프셋(%)으로 표현한다. 이렇게 하면 reduced-motion 정지 상태와
애니메이션 종료 상태가 정확히 같은 위치가 된다(`animation-fill-mode` 의존
없이).

투영은 1d-2의 `projectAxis`(여백 14~86% 띠)를 **전체 프레임의 min/max**로
확장해야 한다 — 마지막 프레임만으로 범위를 잡으면 이동 중인 말이 보드를
벗어난다. 정지 프레임 렌더 결과는 범위가 바뀌므로 1d-2 테스트의 기대값
갱신이 필요하다(로직 변경이 아니라 투영 범위 확장이며, 그 이유를 테스트
주석에 남긴다).

### 4-3. cue 연출

| cue | 정본 문법 | 구현 |
|---|---|---|
| attack | 짧은 전진/복귀 | 해당 tick 구간에서 목표 방향으로 짧게 `translate` 후 복귀 |
| hit | 밀림/진동 | 짧은 밀림 + 감쇠 진동 |
| evade | 측면 이동 | 측면(y축) 짧은 이동 후 복귀 |
| balance_broken | 흔들림/기울어짐 | `rotate` 흔들림 유지 |
| incapacitated | 흐려짐/표식 | `opacity`/`filter` 감광 + 기존 `倒` 표식 유지 |

`attack`의 "목표 방향"은 core가 주는 `piece.facing`에서 얻는다 — 목표를
renderer가 추론하지 않는다. `facing`이 (0,0)이면 방향 연출을 생략한다
(방향을 지어내지 않는다).

## 5. 예상 변경 파일

| 파일 | 변경 |
|---|---|
| `web/src/ui/storybook/combat/combatMotion.ts` | 신규. 프레임 → keyframe CSS 생성 |
| `web/src/ui/storybook/combat/combatMotion.test.ts` | 신규 |
| `web/src/ui/storybook/combat/renderCombatStage.ts` | 투영 범위 확장, `<style>` 방출, 로그 `animation-delay` |
| `web/src/ui/storybook/combat/renderCombatStage.test.ts` | 기대값 갱신 + 신규 |
| `web/src/styles/storybook.css` | `@media (prefers-reduced-motion: no-preference)` 모션 블록 |
| `crates/escape-core/tests/encounter_combat_wave3.rs` | 축 계약 테스트 1개 (WP5) |
| `docs/design/Mobile_Ink_Storybook_UI.md` | 재생 계약 절 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 1d-3 상태, 1d-4 신설 |

## 6. 작업 패키지

### WP1 — keyframe 생성 모듈 + 테스트 (테스트 먼저 red)
`combatMotion.ts`. 테스트가 고정할 것:
- 총 길이가 정확히 `(frames-1) × tick_millis` ms다.
- 프레임 k의 오프셋이 `k/(frames-1)×100%`다.
- `frames.length <= 1`이면 빈 문자열을 반환한다 (0으로 나누지 않음, `NaN`
  부재).
- 같은 입력 → 같은 문자열 (결정론).
- 말 id에 CSS 식별자로 위험한 문자가 있어도 안전한 keyframe 이름이 나오고
  서로 충돌하지 않는다.
커밋: `feat(web): generate combat playback keyframes from the frame list`

### WP2 — 보드 재생 연결
투영 범위를 전체 프레임으로 확장, `<style>` 방출, `--dx`/`--dy` 도입.
1d-2 정지 프레임 테스트 기대값 갱신(이유 주석 필수).
커밋: `feat(web): play the combat board back at simulation speed`

### WP3 — cue 연출 CSS
공용 문법 5종. `no-preference` 안에만. 신규 색상 리터럴 0개.
커밋: `style(web): cue presentation grammar for the combat board`

### WP4 — 로그 tick 동기 노출
`animation-delay: {tick × tick_millis}ms`, `sequence` 순서 유지, DOM 유지.
커밋: `feat(web): reveal core log lines at their simulation tick`

### WP5 — 축 계약 위반을 문서에 남긴다 (`crates/` 무변경)
정본 09의 "화면 왼쪽=아군, 오른쪽=적"이 재생 중에 깨진다는 사실(§1의 실측)을
재생 계약 문서에 명시한다. **테스트를 추가하지 않는다** — 저작 값은 계약을
지키므로 authoring 테스트로는 잡히지 않고, 렌더러가 좌표를 왜곡해 맞추면
거리 읽기가 망가진다. 고칠 곳은 AI·충돌 규칙이며 별도 슬라이스다.
커밋: WP6에 합친다.

### WP6 — 문서
재생 계약(총 길이 = 시뮬레이션 시간, 시간 조작 금지, reduced-motion 경로),
`<style>` in body 트레이드오프, 축 계약, 1d-4(이름 + 게이트 + QA) 신설.
커밋: `docs(combat): record the playback contract and the remaining 1d-4 scope`

## 7. 검증

```bash
cd web && npx tsc --noEmit
cd web && npm test
cargo test --workspace --no-fail-fast
git diff --check
```

색상 리터럴 grep(1d-2와 동일), `transition`/`animation` 선언이 **전부**
`@media (prefers-reduced-motion: no-preference)` 안에 있는지 확인.

**오케스트레이터 실측**(게이트가 남아 있어 정식 QA 불가):
임시 하네스 페이지를 320 / 390 / 1280에서 본다.
- 재생 총 길이를 계측해 `(frames-1) × tick_millis`와 일치하는지 확인
  (`getAnimations()`의 `effect.getTiming().duration` 합산)
- 재생 중 말이 보드를 벗어나지 않는지
- `emulateMedia({reducedMotion: 'reduce'})`에서 모션 0개, 위치가 마지막
  프레임과 일치하는지
- 로그가 tick 시각에 맞춰 나타나는지, 재생 종료 후 전부 보이는지

## 8. 범위 밖

- 게이트 제거, 전투원 표시 이름, wasm 재빌드, 5뷰포트 정식 QA → 1d-4
- 2배속·즉시 결과·자동 전투 UI (core enum은 이미 있음)
- 개입 일시정지 흐름, 기회/대응 제시 → Step 2c
- 전체 로그 열람 UI
- 전술 구역 표현(정본 09) — 구역 경계 정의가 정본에 수치로 없다
- 상시 오버레이(역할·압박·혼란·고립·범위·후방 각도)
- 은폐 적·증원 숨김 — 파이프라인에 은폐 개념이 아직 없다
- 음향 (정본 13의 연출 수단 중 하나이나 오디오 엔진 연결은 별도)
- 치유·명줄·패배 결과, 밸런스 수치
