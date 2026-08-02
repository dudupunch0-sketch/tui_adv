# Combat Wave 3 — Step 1d-3 보고서: Web 관전 재생 연출

플랜: `fable_combat_wave3_step1d3_2608021755.md`
Baseline: `2347617` (PR #185)
브랜치: `claude/combat-wave3-step1d3`

## 결과 요약

1d-2가 만든 정지 프레임 위에 데이터 구동 재생 연출을 얹었다. 정본 13의
"상단은 실제 실시간 전투 시뮬레이션"과 "시뮬레이션 시간과 화면 시간은 항상
일치"를 충족한다. **게이트 플래그는 유지**했다.

| 항목 | 값 |
|---|---|
| `cd web && npx tsc --noEmit` | 통과 |
| `cd web && npm test` | 16 파일 **148 테스트, 0 failed** (이전 108 + 신규 40) |
| `npx vite build` | 통과 |
| `git diff --check` | 통과 |
| 변경 파일 | `web/` 5개 + `docs/design/` 2개 + 플랜/보고서. `crates/`·YAML·번들·게이트·`main.ts`·`package.json`·`types.ts` 무변경 |

## 구현 방식

`renderCombatStage.ts`가 프레임별 투영 좌표를 `combatMotion.ts`에 넘기고,
`combatMotion.ts`가 말별 `@keyframes`를 문자열로 생성해 렌더 결과에 `<style>`
블록으로 함께 방출한다.

- **재생 길이 = 정확히 `(frames-1) × tick_millis`.** 실측 900ms.
- 위치는 `left`/`top`(1d-2가 고정)을 그대로 두고 `translate`로만 움직인다.
  오프셋 단위는 `%`가 아니라 `cqw`/`cqh`다 — `translate`의 백분율은 컨테이너가
  아니라 요소 자기 크기 기준이라 보드 좌표를 표현할 수 없다. 보드에
  `container-type: size`를 준다.
- keyframe은 **마지막 프레임 기준 상대 오프셋**이라 `100%` stop이
  `calc(-50% + 0cqw)`가 된다 → 재생 종료 상태와 reduced-motion 정지 상태가
  `animation-fill-mode`에 의존하지 않고 같은 위치다.
- 투영 범위를 마지막 프레임에서 **전체 프레임**으로 확장했다. 마지막 프레임만
  쓰면 이동 중인 말이 범위 밖 좌표를 지나며 보드를 벗어난다.
- 로그는 각 줄에 `animation-delay`를 인라인으로 얹어 tick 시각에 노출한다.
  DOM에서 제거하지 않고 opacity만 다룬다. `aria-live`는 쓰지 않는다.

## 검증

### mutation test — 12건 시도, 12건 전부 테스트가 잡았다

| # | 깨뜨린 규칙 | 잡은 테스트 |
|---|---|---|
| M1 | 재생 길이에 50ms 패딩 | `produces a duration of exactly (frames.length - 1) * tickMillis` (+2건) |
| M2 | keyframe 오프셋을 제곱으로 이징 | `places tick k at exactly k / (frames.length - 1) * 100%` (+2건) |
| M3 | reduced-motion 미디어 쿼리 래퍼 제거 | `wraps the entire generated block in the media query` (+2건) |
| M4 | `frames <= 1` 가드 제거 | `returns an empty string and 0 duration for a single frame` (+1건) |
| M5 | `<style>` breakout 가드 제거 | `skips animating a piece id that could break out of a <style> raw-text element` (+1건) |
| M6 | facing이 0일 때 방향을 지어냄 | `omits the lunge stop entirely when facing is (0, 0)` (+1건) |
| M7 | balance_broken의 rotate 생략 | `tilts by alternating degrees while the cue is present` |
| M8 | 감광 속성을 항상 방출 | `omits opacity/filter entirely when no frame carries incapacitated` |
| M9 | 개행 이스케이프 제거 | `escapes a newline in a piece id so the attribute selector cannot break the stylesheet` |
| M10 | 로그 원점 미차감 | `anchors log reveal to the first frame tick so the board and the log share one origin` (+1건) |
| M11 | 투영 범위를 마지막 프레임으로 되돌림 | `expands the projection range across every frame` |
| M12 | hit 진동이 facing을 사용 | `does not use piece.facing for the hit direction` |

### 실화면 계측 (Playwright, `reducedMotion` 컨텍스트, 320/390/1280)

실제 덤프 데이터로 하네스를 만들어 계측했다 (하네스는 커밋하지 않았다).

| 항목 | no-preference | reduce |
|---|---|---|
| 애니메이션 수 | 42 (말 2 × 900ms linear both + 로그 40행) | **0** |
| 로그 40행 즉시 표시 | 0행 (시각에 맞춰 노출) | **40행** |
| 말 위치 | 첫 프레임(0%) → 마지막 프레임 | 마지막 프레임(38%/62%) |
| 보드:로그 | 0.700 | 0.700 |
| 가로 스크롤 | 없음 | 없음 |

- 로그 지연 실측: **0, 100, …, 900ms** — 보드 재생 구간(0~900ms)과 정확히 일치.
- 재생 전 구간(50ms 간격 19지점) 샘플링에서 말이 보드 경계에 32px보다 가까워지지
  않는다.
- 재생 종료 상태와 `reduce` 정지 상태의 말 중심 좌표가 일치한다.

## 오케스트레이터 리뷰에서 고친 결함 5건

테스트 148건이 전부 통과한 상태에서 화면은 다섯 군데 틀려 있었다.

1. **보드와 로그의 시간 원점이 한 tick 어긋났다.** 보드는 프레임 인덱스 k를
   `k × tick_millis`에 놓고 실측 데이터의 첫 프레임 tick은 1이므로 보드의 0ms는
   tick 1이다. 로그는 `entry.tick × tick_millis`를 써서 같은 사건이 보드보다
   100ms 늦게 나타났다. 정본 13의 동기화 요구 위반이다.
2. **전투불능 감광이 애니메이션 안에만 있었다.** `reduce`에서 마지막 프레임이
   전투불능인데도 말이 멀쩡하게 보여 두 경로의 그림이 달랐다. 정적 층으로
   올렸다.
3. **감광 속성을 매 stop에 방출해 정적 감광을 덮어썼다.** `opacity: 1`이 재생
   내내 `[data-active="false"]` 감광을 지웠다.
4. **`.combat-log__row { opacity: 0 }` 기본값.** `fill: both`가 이미 delay
   구간에 `from`을 적용하므로 불필요하고, 애니메이션이 돌지 않으면 로그가
   영구히 안 보이는 실패 모드가 된다.
5. **속성 선택자 값의 개행 이스케이프 누락.** CSS 문자열에 raw 개행이 들어가면
   그 규칙과 뒤따르는 규칙까지 파싱이 깨진다.

## 플랜 정정 — I9가 틀렸다

I9의 허용 애니메이션 속성 목록에 `rotate`를 빼놓은 것은 내 실수다. 정본 13은
균형 붕괴를 **"흔들림/기울어짐"**으로 정의하고, 흔들림만 쓰면 `hit`의 진동과
구별되지 않아 공용 문법이 무너진다. `rotate`는 `translate`와 같은 개별 transform
속성이라 컴포지터에서 처리되고 레이아웃을 만들지 않는다. I9를 정본에 맞춰
`rotate` 허용으로 바로잡고 기울어짐을 구현했다.

subagent는 I9를 문자대로 따라 흔들림만 구현하고 그 이탈을 명시적으로 보고했다 —
절차상 옳은 판단이었다. 플랜의 Hard invariant와 구현 지침(§4-3)이 서로 충돌했던
것이 원인이다.

## 재생이 드러낸 core 결함 (렌더러 범위 밖)

재생하면 **두 말이 중간 시점에 거의 겹쳐 지나가며 아군이 도전자 오른쪽으로
넘어간다.** 정본 09의 "화면 왼쪽: 아군 영역 / 화면 오른쪽: 적 영역"을 재생 중에
위반한다. 렌더러가 좌표를 왜곡해 맞추면 거리 읽기가 망가지므로 고칠 곳은
AI·충돌 규칙이다. 인덱스의 core 결함 블록에 기록했다.

## 범위 밖 (Step 1d-4로 남김)

- **전투원 표시 이름** (core schema + authoring) — 화면과 보고서가 내부 id를
  그대로 보여준다. 게이트를 풀기 전에 결정해야 한다.
- 게이트 플래그 제거, wasm 재빌드, 5뷰포트 정식 QA
- 2배속·즉시 결과·자동 전투 UI (core enum은 이미 있음)
- 개입 일시정지 흐름, 기회/대응 제시 → Step 2c
- 전체 로그 열람 UI
- 전술 구역 표현 — 구역 경계 정의가 정본에 수치로 없다
- 상시 오버레이(역할·압박·혼란·고립·범위·후방 각도)
- 은폐 적·증원 숨김 — 파이프라인에 은폐 개념이 없다
- 음향
- 치유·명줄·패배 결과, 밸런스 확정
