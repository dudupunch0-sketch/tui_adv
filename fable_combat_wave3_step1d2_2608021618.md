# Combat Wave 3 — Step 1d-2 설계 플랜: Web Storybook 전투 관전 표면 (정지 프레임)

Baseline: `ddf9d29` (PR #182, Wave 3 Step 1d-1 terminal 관전 렌더러)
브랜치: `claude/combat-wave3-step1d2`
선행 플랜: `fable_combat_wave3_step1d1_2608021329.md`

## 0. 이 슬라이스가 Step 1d의 어디인가

Step 1d(관전 렌더러)는 세 조각으로 나눈다.

| 슬라이스 | 내용 | 상태 |
|---|---|---|
| 1d-1 | terminal(SuperLightTUI) 관전 렌더러 | 완료 (PR #182) |
| **1d-2 (이 플랜)** | **Web 관전 표면 — 정지 프레임(= reduced-motion 안정 상태) + 로그 + 보고서 + TS 타입** | 이 슬라이스 |
| 1d-3 | 재생 연출(틱 기반 모션) + 로그 동시 노출 + 게이트 플래그 제거 + wasm 재빌드 + 5뷰포트 실화면 QA | 후속 |

### 왜 정지 프레임을 먼저 만드는가 (재작업이 없는 이유)

정본 13은 상단에 **실시간 시뮬레이션**을 요구하고, 동시에 `prefers-reduced-motion`
사용자에게는 모션 없는 안정 상태를 보여줘야 한다. 이 두 요구의 교집합이
**"마지막 프레임을 정적으로 그린 화면"**이다.

즉 1d-2가 만드는 화면은 1d-3에서 버려지는 임시물이 아니라 **1d-3의
reduced-motion 경로 그 자체**다. 1d-3은 같은 DOM·같은 CSS 변수 위에
`@media (prefers-reduced-motion: no-preference)` 안에서만 모션을 얹는다.
그래서 이 슬라이스는 말 위치를 `--piece-x` / `--piece-y` 커스텀 속성으로
표현해야 하며(WP3 불변식), 1d-3은 그 변수를 시간축으로 움직이기만 한다.

### 왜 게이트를 이 슬라이스에서 풀지 않는가

`combat_spectator_preview_unlocked` 게이트를 풀면 이구학지 기본 플레이 경로에
전투 화면이 노출된다. 정본 13이 요구하는 실시간 연출이 아직 없는 화면을
플레이어 경로에 올리지 않는다. 1d-1에서 게이트를 유지한 것과 같은 판단이다.
게이트 제거 = 1d-3 소관.

부수 효과: 이 슬라이스는 인카운터에 도달할 수 없으므로 `npm run
qa:storybook:visual`(시작 화면에서 실제 플레이로 진입하는 스크립트)로는 이
표면을 찍을 수 없다. 5뷰포트 실화면 QA도 1d-3 소관이다. 대신 이 슬라이스는
**렌더 결과 HTML을 단위 테스트로 고정**한다.

## 1. 정본 근거

- **정본 13 (감독형 관전·전략 피드백)**
  - 상단 실시간 시뮬레이션 **65~75%**, 하단 로그 **25~35%**.
  - 말은 **체스말 형태**.
  - 공용 연출 문법: 공격=짧은 전진/복귀, 피격=밀림/진동, 회피=측면 이동,
    균형 붕괴=흔들림/기울어짐, 전투불능=흐려짐/표식.
  - **시간 조작 금지** — 순간 정지·슬로모션 없음. 진동·플리커·감광·이펙트만.
  - **상단 연출과 하단 로그의 색·아이콘 동기화.**
  - 로그는 자유 생성 문장이 아니라 **등록된 사건 태그 + 로그 템플릿**.
  - 보고서 **금지 항목**: 전략 수행 평가, 핵심 전환점, 자동 원인 분석,
    전략 조언, 종합 MVP, 이전 전투 결과 자동 비교.
- **정본 07 (UI·템포·리스크)**: 전체 로그와 상세 수치는 일시정지 또는 전투
  종료 뒤 **별도로 열람**. → 이 화면은 `core_log`만 문장화하고 `full_log`는
  개수만 표시한다.
- **정본 11 §8**: 수치는 hundredths 고정소수점. 표시할 때 정수 반올림.
- **정본 03**: 결정성은 같은 `simulation_version` 안에서만 보장된다.
  → fingerprint를 표시하면 **반드시 `simulation_version`과 함께** 표시한다.
- **프로젝트 규칙**: Rust GameCore가 gameplay truth를 소유한다. Web renderer는
  판정을 재계산하지 않는다.
- **UI 방향**: `docs/design/Mobile_Ink_Storybook_UI.md` (수묵 서책 + 정보 드로어),
  3분할 레이아웃(상단바 / 스크롤 뷰포트 / 하단 HUD·드로어).

## 2. Scope

`ScenePage.combat`(= `CombatSpectatorPage`)을 Web Storybook 렌더러가 화면으로
옮긴다. 새로 만드는 것:

1. `web/src/core/types.ts`에 관전 타입 추가 (Rust serde 표현과 1:1).
2. `web/src/ui/storybook/combat/renderCombatStage.ts` — 관전 표면 HTML 생성.
3. `web/src/ui/storybook/combat/combatLogTemplates.ts` — 6개 template id →
   한국어 문장 테이블 (renderer 소유).
4. `web/src/styles/storybook.css`에 관전 표면 스타일 추가.
5. 단위 테스트 (`renderCombatStage.test.ts`, `combatLogTemplates.test.ts`,
   `render.test.ts`에 통합 지점 1건).
6. 문서 갱신.

## 3. Hard invariants (하나라도 어기면 이 슬라이스는 실패다)

### I1. 판정 재계산 0회

TS 코드는 피해 계산, 명중 판정, 승패 판정, cue 파생, 로그 중요도 필터링,
피해 합산, 최대 피해자 선정을 **하지 않는다**. `page.combat`의 필드를 읽어
문자열·DOM으로 옮기는 것만 한다.

허용되는 계산은 표시 변환뿐이다: hundredths → 정수 반올림, tick → 경과 ms
(`tick * view.tick_millis`), 좌표 → 보드 백분율 투영, 배열 길이 표시.

### I2. 수묵 토큰만 사용 — 신규 색상 리터럴 0개

`--paper` `--paper-deep` `--paper-lit` `--ink` `--ink-body` `--ink-faded`
`--ink-wash` `--seal-red` `--seal-red-lit` `--jade` `--gold-leaf`
`--line-soft` `--line-hard` `--void-black` 만 쓴다.

이 diff에 `#`으로 시작하는 색상, `rgb(`, `rgba(`, `hsl(`, `oklch(`,
명명 색상(`red` 등)을 **한 개도 추가하지 않는다**. `color-mix()`로 위 토큰을
섞는 것은 허용한다(`in oklab` 명시). `currentColor`도 허용한다.

검증: `git diff origin/main..HEAD -- web/src/styles/storybook.css` 에서 신규
추가 줄에 색상 리터럴이 없어야 한다.

### I3. 3분할 레이아웃 침범 금지

- `.storybook-shell`의 `grid-template-rows: auto minmax(0, 1fr) auto`를
  **수정하지 않는다.**
- `.game-viewport` / `.game-topbar` / `.storybook-hud` / `.storybook-dock`의
  기존 규칙을 **수정하지 않는다.** (새 자손 선택자 추가는 허용)
- 관전 표면은 `.game-viewport` 안, `.storybook-page` 안의 한 블록으로 들어간다.
  `position: fixed`, 뷰포트 단위 기반 전면 오버레이, `z-index`로 상단바/하단바
  덮기 **금지**.
- 가로 스크롤을 만들지 않는다. 넘칠 수 있는 내용(로그, 보고서 표)은 자기
  컨테이너 안에서 `overflow-y: auto`로 처리한다.

### I4. 65~75 / 25~35 비율

관전 표면 내부는 상단 보드 : 하단 로그 = **70 : 30**으로 고정한다
(양쪽 범위의 중앙값). `grid-template-rows: minmax(0, 70fr) minmax(0, 30fr)`.
전투 보고서는 이 비율 밖, 표면 **아래**에 일반 흐름으로 둔다 (비율은 보드:로그의
비율이다).

### I5. `page.combat`이 없으면 출력이 이전과 바이트 단위로 같다

기존 51개 인카운터에는 `combat`이 없다. 관전 표면은 `page.combat`이
`undefined`일 때 **빈 문자열**을 반환하고, 래퍼 요소·클래스·`data-*` 속성도
추가하지 않는다.

테스트: `render.test.ts`에 `combat`이 없는 페이지의 `renderStorybookPage`
출력이 관전 관련 문자열(`combat-stage`, `data-region="combat`)을 전혀 포함하지
않음을 고정한다.

### I6. 로그 템플릿 계약 — 조용히 버리지 않는다

6개 template id를 문장화한다:

| template_id | 문장 형식 (target 있음 / 없음) |
|---|---|
| `combat.log.move_intent` | `{actor} 이동 의도 (목표 {target})` / `{actor} 이동 의도` |
| `combat.log.target_selection` | `{actor} → 목표 지정: {target}` / `{actor} 목표 지정 (대상 없음)` |
| `combat.log.collision` | `{actor} × {target} 충돌` / `{actor} 충돌 (대상 없음)` |
| `combat.log.damage_applied` | `{actor} → {target} 피해 {value}` / `{actor} 피해 {value} (대상 없음)` |
| `combat.log.effect_applied` | `{actor} → {target} 효과 적용 [{effect}]` / `{actor} 효과 적용 [{effect}] (대상 없음)` |
| `combat.log.effect_applied_hidden` | `{actor} → {target} 효과 적용 [정체불명]` / `{actor} 효과 적용 [정체불명] (대상 없음)` |

이 문장은 `crates/escape-terminal/src/snapshot.rs::combat_log_template_line`의
표와 **글자 단위로 같게** 맞춘다 (두 렌더러가 같은 사건을 다르게 부르지 않도록).
`value`는 `value_hundredths`를 정수 반올림한 값이며, `value_hundredths`가
`null`이면 `0`이 아니라 `(수치 없음)`을 쓴다 — **없는 값을 0으로 때우지 않는다.**

알 수 없는 `template_id`는 **버리지 않는다.** `{actor} → {target} 알 수 없는 사건
[template_id={id}]` 줄을 만들고 `data-log-unknown="true"`를 붙여 시각적으로도
구분한다.

### I7. `full_log`는 개수만, 생략은 개수 명시

- `core_log`만 문장화한다.
- `full_log`는 건수와 "일시정지 또는 전투 종료 후 별도 열람" 안내만 표시한다.
  전체 로그 열람 UI를 만들지 않는다 (범위 밖).
- 표시 상한(`WEB_CORE_LOG_LIMIT = 40`)을 넘으면 **`…(생략 N줄)` 줄을 반드시
  출력한다.** 조용한 truncation 금지.

### I8. 보고서 — 없는 것은 숨긴다, 금지 항목은 문구조차 만들지 않는다

- `combat.report`가 `undefined`면 보고서 섹션 **자체를 만들지 않는다**
  (전투 진행 중이라는 뜻이다).
- `top_damage_dealt_id` / `top_damage_taken_id`가 `null`이면 **그 줄을 아예
  출력하지 않는다.** "없음" 같은 대체 문구도 만들지 않는다.
- `decisive_tick`이 `null`이면 같은 규칙을 적용한다.
- 금지 항목(전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·
  이전 전투 비교)은 **문구도 계산도 만들지 않는다.** 보고서는
  `CombatConclusionReport` 필드를 그대로 옮기기만 한다.
- fingerprint를 표시하는 요소에는 `simulation_version`을 같은 요소 안에 둔다
  (정본 03 비교 계약). 테스트로 고정한다.

### I9. 접근성

- 보드는 그래픽 투영이다. **동등한 의미의 semantic table을 함께 제공한다**
  (`<table>`: 말 id / 진영 / 좌표 / 상태 / cue). 시각적으로는 `sr-only`로
  숨기고, 보드에는 `role="img"` + 요약 `aria-label`을 준다.
  (근거: modern-web-guidance `css` 가이드 — 그래픽으로 데이터를 표현하면
  semantic data table 대체를 반드시 제공한다.)
- **의미를 색으로만 전달하지 않는다.** 진영과 cue는 글리프/텍스트로도 읽힌다.
- `@media (forced-colors: active)`에서 진영 구분과 cue 표식이 사라지지 않도록
  시스템 색 키워드(`CanvasText`, `Highlight`, `GrayText`) 기반 대체를 준다.
  `background-image`/`box-shadow`만으로 경계를 표현하지 않는다.
- 로그 영역은 `aria-label`을 갖는다. 이 슬라이스는 정적 렌더이므로
  `aria-live`는 쓰지 않는다 (1d-3에서 재생 노출과 함께 판단).
- 인터랙티브 요소를 새로 추가하지 않는다 → 포커스 링·터치 타겟 이슈 없음.
  (`<details>` 등 조작 요소가 필요하다고 판단되면 추가하지 말고 보고하라.)

### I10. 보드 투영 규칙

- `view.frames`의 **마지막 프레임만** 그린다 (1d-3이 시간축을 얹는다).
- 좌표 → 보드 백분율: 마지막 프레임 말들의 `min`/`max`로 범위를 잡고 0~100%로
  비례 투영한다. **`span === 0`이면 0으로 나누지 말고 50%(중앙)에 놓는다.**
  x·y 각각 독립으로 판단한다.
- 좌표 단위 의미는 정본에 확정되지 않았다. 투영은 **비례 배치**일 뿐이며
  거리·속도를 수치로 주장하는 문구를 만들지 않는다.
- `frames`가 비어 있으면 "표시할 프레임이 없다", 말이 0개면 "표시할 말이 없다"를
  출력하고 **예외를 던지지 않는다.**
- 좌표는 전장 기하학이다. RTL에서 좌우가 뒤집히면 안 된다 → 보드 배치에는
  **논리 속성을 쓰지 않고** 물리 좌표(`translate` X/Y)를 쓴다.
  (텍스트 블록의 여백·정렬에는 논리 속성을 쓴다.)

### I11. 색·아이콘 동기화는 **증명 가능한 대응만** 한다

정본 13의 "상단 연출과 하단 로그의 색·아이콘 동기화"를 구현하되, core에서
대응이 유도되는 짝만 연결한다.

core의 `cues_for`는 `CombatSpectatorCue::Hit`을
`outcome.hit && outcome.damage_hundredths > 0`에서 만들고, 로그의
`combat.log.damage_applied`는 같은 `DamageApplied` 사건에서 나온다.
→ **`combat.log.damage_applied` 로그 줄은 Hit cue와 같은 색·같은 글리프를 쓴다.**

나머지 5개 template id는 대응하는 cue가 core에 없다 (`Attack` cue는 로그
태그가 없고, `collision`/`move_intent`/`target_selection`/`effect_applied`는
cue를 만들지 않는다). → **중립 잉크색으로 두고, 대응을 발명하지 않는다.**
이 판단 근거를 코드 주석에 남긴다.

cue 5종 표식 (terminal 표식과 짝을 이룬다 — 한쪽만 고치지 말 것):

| cue | 정본 연출 의미 | terminal 표식 | web 글리프 | web 색 토큰 |
|---|---|---|---|---|
| `attack` | 짧은 전진/복귀 | `>` | 攻 | `--seal-red` |
| `hit` | 밀림/진동 | `<` | 打 | `--seal-red-lit` |
| `evade` | 측면 이동 | `~` | 避 | `--jade` |
| `balance_broken` | 흔들림/기울어짐 | `!` | 傾 | `--gold-leaf` |
| `incapacitated` | 흐려짐/표식 | `x` | 倒 | `--ink-faded` |

### I12. 건드리지 않는 것

- `crates/` 전체 (Rust 무변경)
- `src/tui_adv/**` YAML, 생성된 두 번들 JSON
- 게이트 플래그 `combat_spectator_preview_unlocked` (제거 금지)
- `crates/escape-terminal/tests/cli_smoke.rs` (다른 작업자 소유. 읽기만)
- `.claude/worktrees/`
- `web/src/main.ts` (관전 표면은 `renderStorybookPage` 안에서 조립된다.
  main.ts 배선이 필요하다고 판단되면 수정하지 말고 보고하라.)
- `package.json` — **의존성 추가 금지.**

### I13. 이스케이프

`page.combat`에서 온 모든 문자열(말 id, actor/target id, effect id,
template_id, fingerprint, simulation_version, 진영·결과 라벨 원본)은
`escapeHtml`을 통과해야 한다. 숫자는 `String()` 후 삽입한다.

## 4. CSS 작성 규칙 (modern-web-guidance `css` 가이드 적용)

- **`@layer`를 새로 도입하지 않는다.** `storybook.css`는 현재 레이어가 없고,
  레이어 스타일은 비레이어 스타일보다 항상 약하다. 여기서 레이어를 도입하면
  기존 규칙과의 우선순위가 뒤집힌다. 대신 `:where()`로 특이도를 낮게 유지한다.
- 전역 리셋(`*`) 추가 금지.
- `:not()` 대신 상태별 override를 쌓지 않는다. 의도를 선택자에 쓴다.
- 폰트 크기에 `px` 금지 — `rem`/`em`.
- `line-height`는 단위 없는 수.
- 보드 크기는 `aspect-ratio` + `min()`/`clamp()`로 잡는다. 고정 `px` 높이 금지.
- 컨테이너 기준 반응이 필요하면 `@container`를 쓴다 (`.combat-stage`에
  `container-type: inline-size`). 뷰포트 미디어쿼리로 컴포넌트를 분기하지 않는다.
- `dvh`가 필요하면 `vh` 대신 `dvh`.
- 말 실루엣은 타원 `border-radius`(예: `50% 50% 22% 22% / 60% 60% 40% 40%`)로
  체스 폰 형태를 만든다. 이미지·SVG 파일을 새로 추가하지 않는다.
- 진영 표현: 아군 = 먹으로 채운 말, 적 = 종이 바탕 + 먹 윤곽. 색 하나에만
  의존하지 않도록 글리프도 다르게 준다.
- 애니메이션·트랜지션을 **이 슬라이스에서는 추가하지 않는다** (1d-3 소관).
  `transition`/`animation` 선언이 diff에 없어야 한다.

## 5. 예상 변경 파일

| 파일 | 변경 |
|---|---|
| `web/src/core/types.ts` | 관전 타입 추가, `ScenePage.combat?` 추가 |
| `web/src/ui/storybook/combat/combatLogTemplates.ts` | 신규. 6+1 문장 테이블 |
| `web/src/ui/storybook/combat/combatLogTemplates.test.ts` | 신규 |
| `web/src/ui/storybook/combat/renderCombatStage.ts` | 신규. 보드·로그·보고서 |
| `web/src/ui/storybook/combat/renderCombatStage.test.ts` | 신규 |
| `web/src/ui/storybook/render.ts` | 관전 표면 삽입 (`page.combat` 있을 때만) |
| `web/src/ui/storybook/render.test.ts` | 통합 1건 + I5 무변경 1건 |
| `web/src/styles/storybook.css` | 관전 표면 스타일 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 1d-2 상태·경계 갱신 |
| `docs/design/Mobile_Ink_Storybook_UI.md` | 관전 표면 절 추가 |

## 6. TS 타입 (Rust serde 표현과 1:1)

Rust 쪽 실제 직렬화 모양이다. 추측하지 말고 이대로 쓴다.

```ts
export type CombatSide = 'ally' | 'enemy';
export type CombatSpectatorCue = 'attack' | 'hit' | 'evade' | 'balance_broken' | 'incapacitated';
export type CombatLogImportance = 'routine' | 'important' | 'decisive';
export type CombatConclusionOutcome =
  | 'in_progress' | 'ally_victory' | 'enemy_victory' | 'mutual_defeat' | 'stalemate';
export type CombatConclusionReason =
  | 'no_terminal_condition' | 'all_enemies_defeated' | 'all_allies_defeated'
  | 'both_sides_defeated' | 'max_ticks_reached';

export interface CombatPoint { x: number; y: number }           // CombatPosition / CombatFacing

export interface CombatSpectatorPiece {
  id: string;
  side: CombatSide;
  position: CombatPoint;
  facing: CombatPoint;
  active: boolean;
  cues: CombatSpectatorCue[];                                    // serde(default) → 빈 배열 가능
}

export interface CombatSpectatorFrame { tick: number; pieces: CombatSpectatorPiece[] }

export interface CombatSpectatorLogEntry {
  tick: number;
  sequence: number;
  template_id: string;                                           // 문장은 renderer 소유
  importance: CombatLogImportance;
  actor_id: string;
  target_id?: string | null;
  value_hundredths?: number | null;
  effect_id?: string | null;
}

export interface CombatSpectatorView {
  simulation_version: string;                                    // newtype struct → bare string
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
  top_damage_dealt_id?: string | null;                           // 피해 0이면 없음
  top_damage_taken_id?: string | null;
  fingerprint: string;
}

/** ScenePage.combat — 전투 인카운터가 아니면 필드 자체가 없다. */
export interface CombatSpectatorPage {
  view: CombatSpectatorView;
  report?: CombatConclusionReport;                               // 진행 중이면 없음
}
```

`ScenePage`에 `combat?: CombatSpectatorPage;`를 추가한다. 주석으로
"전투 인카운터가 아닌 페이지에는 이 필드가 없다"를 남긴다.

한국어 라벨 매핑. **아래 표를 믿지 말고 `crates/escape-terminal/src/snapshot.rs`의
`combat_outcome_label` / `combat_reason_label`을 열어 그 문자열을 그대로 쓴다.**
이 표는 그 소스에서 옮긴 것이며, 어긋나면 소스가 기준이다.

- outcome: `in_progress` 진행 중 / `ally_victory` 아군 승리 /
  `enemy_victory` 적 승리 / `mutual_defeat` 양측 전멸 / `stalemate` 무승부
- reason: `no_terminal_condition` 종료 조건 없음 /
  `all_enemies_defeated` 적 전멸 / `all_allies_defeated` 아군 전멸 /
  `both_sides_defeated` 양측 전멸 / `max_ticks_reached` 최대 tick 도달
- side: `ally` 아군 / `enemy` 적군 (terminal의 접근 대체 표에는 없는 라벨이므로
  web 쪽에서 정한다)

## 7. DOM 구조 (권장 골격)

```
<section class="combat-stage" data-region="combat" aria-label="전투 관전">
  <div class="combat-stage__board" data-region="combat-board" role="img" aria-label="...요약...">
    <div class="combat-board__piece" data-piece-id="..." data-side="ally" data-active="true"
         data-cue-hit="true" style="--piece-x: 50%; --piece-y: 12%">
      <span class="combat-board__glyph" aria-hidden="true">…</span>
      <span class="combat-board__cue" data-cue="hit" aria-hidden="true">打</span>
    </div>
    …
  </div>
  <table class="combat-board__table sr-only"> … 말별 진영·좌표·상태·cue … </table>
  <div class="combat-stage__log" data-region="combat-log" aria-label="전투 핵심 로그">
    <p class="combat-log__meta">전체 로그 N건 …</p>
    <ol class="combat-log__list">
      <li class="combat-log__row" data-template-id="combat.log.damage_applied" data-cue="hit">…</li>
    </ol>
  </div>
</section>
<section class="combat-report" data-region="combat-report" aria-label="전투 종료 보고서"> … </section>
```

`--piece-x` / `--piece-y`를 인라인 스타일로 주는 것은 I2의 "비자명한 값 인라인
금지"의 예외다 — 데이터에서 온 좌표이므로 CSS에 쓸 수 없다. 색·크기 같은
디자인 값은 인라인에 넣지 않는다.

## 8. 작업 패키지 (WP당 커밋 1개)

### WP1 — TS 타입
§6 그대로 `types.ts`에 추가. `npx tsc --noEmit` 통과.
커밋: `feat(web): add combat spectator types to the scene page contract`

### WP2 — 로그 템플릿 테이블 + 테스트
`combatLogTemplates.ts` + 테스트. **테스트를 먼저 red로 만든 뒤 구현한다.**
테스트가 고정할 것:
- 6개 id × (target 있음/없음) 문장이 terminal 표와 글자 단위로 같다.
- `value_hundredths: null` → `(수치 없음)`, `0`이 아니다.
- `value_hundredths: 1333` → `13` (정수 반올림).
- 알 수 없는 id → template_id를 노출하는 fallback 문장이 나오고 버려지지 않는다.
- `effect_id: null` + `effect_applied` → `(효과 id 없음)`.
커밋: `feat(web): add the combat log template sentence table`

### WP3 — 보드 + 접근 대체 표 + 테스트
`renderCombatStage.ts`의 보드 부분. 테스트가 고정할 것:
- 마지막 프레임만 그린다 (프레임 2개 이상일 때 첫 프레임 말 좌표가 안 나온다).
- `span === 0`(모든 말이 같은 x)일 때 `--piece-x: 50%`이고 `NaN`/`Infinity`가
  출력에 없다.
- cue 5종이 각각 `data-cue-*`와 글리프로 나타난다.
- `frames: []` → "표시할 프레임이 없다", 예외 없음.
- `pieces: []` → "표시할 말이 없다", 예외 없음.
- semantic table에 모든 말의 id·진영·좌표가 들어간다.
- 말 id에 `<script>`가 들어와도 이스케이프된다.
커밋: `feat(web): render the combat board resting frame with a table alternative`

### WP4 — 로그 영역 + 보고서 + 테스트
테스트가 고정할 것:
- `core_log`만 문장화되고 `full_log` 전용 항목은 문장으로 나오지 않는다.
- `full_log` 건수가 표시된다.
- `core_log` 41건 → 40줄 + `…(생략 1줄)`.
- `report`가 없으면 `combat-report` 섹션이 출력에 없다.
- `top_damage_dealt_id: null` → 그 줄이 없다 ("없음" 문구도 없다).
- fingerprint가 있는 요소 텍스트에 `simulation_version`이 함께 있다.
- 금지 문구가 출력에 없다 (`MVP`, `전략`, `전환점`, `조언`, `원인` 부분문자열
  부재를 테스트로 고정).
- `damage_applied` 로그 줄에 `data-cue="hit"`이 붙고, 나머지 5개 id에는
  `data-cue` 속성이 없다.
커밋: `feat(web): render the combat core log and conclusion report`

### WP5 — `render.ts` 통합 + I5 무변경 테스트
`renderStorybookPage`에 삽입. `page.combat`이 없으면 빈 문자열.
`render.test.ts`에 통합 1건 + I5 무변경 1건.
커밋: `feat(web): mount the combat spectator surface in the storybook page`

### WP6 — CSS
§4 규칙대로 스타일 추가. 70:30 그리드, 말 실루엣, cue 토큰, forced-colors 대체,
`sr-only` 표. `transition`/`animation` 선언 금지.
커밋: `style(web): ink-wash styling for the combat spectator surface`

### WP7 — 문서
- `docs/design/Combat_System_Implementation_Plan_Index.md`: 1d-2 완료,
  1d-3 남음(모션·게이트 제거·5뷰포트 QA)을 경계 섹션에 **자기 블록으로** 적는다
  (기존 문단에 이어붙이지 말 것). 수치를 쓰면 그 수치를 고정하는 테스트
  함수명을 같이 적는다.
- `docs/design/Mobile_Ink_Storybook_UI.md`: 관전 표면 절. 70:30 비율, cue 표,
  접근 대체 표, forced-colors 대체를 기록한다. cue 표는 terminal 표
  (`docs/dev/TUI_Layout.md`)와 짝임을 명시한다.
커밋: `docs(combat): record the web spectator surface contract`

## 8-b. 오케스트레이터 검증 (subagent 범위 밖)

단위 테스트는 HTML 문자열만 본다. 레이아웃·대비·겹침은 잡지 못한다. 게이트가
남아 있어 `npm run qa:storybook:visual`로는 이 화면에 도달할 수 없으므로,
오케스트레이터가 임시 하네스 페이지(`web/dist`에 실제 빌드된 CSS를 링크한
단일 HTML)를 만들어 320 / 390 / 1280 폭에서 실측한다. 하네스 파일은 커밋
전에 삭제한다.

실측으로 확인할 항목:

- 보드 : 로그 높이 비율이 0.65~0.75 안인가
- `document.documentElement.scrollWidth === window.innerWidth` (가로 스크롤 0)
- 말이 보드 경계에서 잘리지 않는가
- 한 말이 cue를 3개 가질 때 표식이 서로 겹치지 않고 전부 보드 안에 있는가
- 적 말의 글리프가 배경과 구분되어 읽히는가
- 관전 표면 너비가 본문 칸 너비와 같은가 (grid item stretch + `aspect-ratio` +
  `max-block-size`가 함께 걸리면 브라우저가 높이 상한에 맞춰 **너비**를 줄인다)

## 9. 검증 명령 (WSL)

```bash
cd web && npx tsc --noEmit
cd web && npm test
cd ~/work/tui-adv && cargo test --workspace --no-fail-fast    # 무변경 확인용
git diff --check
git diff --name-only origin/main..HEAD                        # crates/·YAML·번들 부재 확인
```

색상 리터럴 확인:
```bash
git diff origin/main..HEAD -- web/src/styles/storybook.css | grep '^+' | grep -nE '#[0-9a-fA-F]{3}|rgba?\(|hsla?\(|oklch\('
```
(출력이 없어야 한다)

## 10. 명시적 범위 밖

- 재생 연출(틱 기반 모션), 로그 동시 노출 타이밍 → 1d-3
- 게이트 플래그 제거, wasm 재빌드, 5뷰포트 실화면 QA → 1d-3
- 전체 로그 열람 UI, 일시정지 흐름
- 배속·즉시 결과·자동 전투 UI
- 개입 기회/대응 제시 → Step 2c
- 혼합형·각본형 인카운터 → Step 2c
- 프리셋·재도전, 치유·명줄·패배 결과, 밸런스 수치 확정
- Rust 쪽 변경 일체

## 11. 최종 체크리스트

- [ ] I1 판정 재계산 0회 — 피해·승패·cue·필터링 계산 없음
- [ ] I2 신규 색상 리터럴 0개 (grep 확인)
- [ ] I3 3분할 레이아웃 기존 규칙 무수정, 가로 스크롤 없음
- [ ] I4 보드:로그 = 70:30
- [ ] I5 `combat` 없는 페이지 출력 무변경 (테스트)
- [ ] I6 6개 템플릿 문장이 terminal과 글자 단위 일치, 알 수 없는 id 노출
- [ ] I7 `full_log` 개수만, 생략 개수 명시
- [ ] I8 보고서 없는 항목 숨김, 금지 항목 부재, fingerprint+version 동반
- [ ] I9 semantic table 대체 + forced-colors 대체 + 색 단독 의존 없음
- [ ] I10 마지막 프레임만, span 0 가드, 빈 프레임/말 0명 무예외
- [ ] I11 `damage_applied`만 hit cue와 동기화, 나머지 중립
- [ ] I12 crates/·YAML·번들·게이트·cli_smoke.rs·main.ts·package.json 무변경
- [ ] I13 모든 데이터 문자열 `escapeHtml`
- [ ] `transition`/`animation` 선언 없음
- [ ] `npx tsc --noEmit` / `npm test` / `cargo test --workspace` 통과
- [ ] `git diff --check` 통과
