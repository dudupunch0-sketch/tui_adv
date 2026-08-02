# Combat Wave 3 — Step 1d-2 보고서: Web Storybook 전투 관전 표면 (정지 프레임)

플랜: `fable_combat_wave3_step1d2_2608021618.md`
Baseline: `ddf9d29` (PR #182)
브랜치: `claude/combat-wave3-step1d2`

## 결과 요약

`ScenePage.combat`을 Web Storybook이 화면으로 옮긴다. 정지 프레임(= 1d-3의
reduced-motion 안정 상태) + 핵심 로그 + 전투 종료 보고서 + TS 타입 계약.
**게이트 플래그는 유지**했다 — 실시간 연출이 없는 화면을 플레이어 경로에
올리지 않는다(1d-1과 같은 판단).

| 항목 | 값 |
|---|---|
| `cd web && npx tsc --noEmit` | 통과 |
| `cd web && npm test` | 15 파일 **107 테스트, 0 failed** (이전 73 + 신규 34) |
| `cargo test --workspace --no-fail-fast` | **346 passed, 0 failed** (Rust 무변경) |
| `git diff --check` | 통과 |
| `npx vite build` | 통과 (CSS 파싱 확인) |
| 변경 파일 | `web/` 8개 + `docs/design/` 2개. `crates/`·YAML·번들·게이트·`main.ts`·`package.json` 무변경 |

## 커밋

| hash | 내용 |
|---|---|
| `a05edf8` | TS 타입 계약 (`ScenePage.combat?`) |
| `1d8e843` | 로그 템플릿 문장 테이블 |
| `7612b5e` | 체스말 보드 + 접근 대체 표 |
| `9e1a32a` | 핵심 로그 + 전투 종료 보고서 |
| `72c4e8b` | `renderStorybookPage` 통합 |
| `b119476` | 수묵 토큰 CSS |
| `98142c9` | 문서 |
| `accd659` | (리뷰) 보드 경계 클리핑 수정 + 거짓 테스트 이름 정정 |
| `7976923` | (리뷰) 실화면 QA에서 나온 결함 3건 수정 |

## 검증 방식

### mutation test — 규칙이 실제로 고정됐는지 9건 확인

규칙을 일부러 깨뜨려 어떤 테스트가 잡는지 확인한 뒤 복원했다. **9건 전부
테스트가 잡았다.**

| # | 깨뜨린 규칙 | 잡은 테스트 |
|---|---|---|
| M1 | `span === 0` 가드 제거 | `centers pieces at 50% when the coordinate span is zero, with no NaN/Infinity` |
| M2 | 투영 여백 14 → 0 | `keeps the extreme pieces off the board edge so translate(-50%) cannot clip them` |
| M3 | 모든 로그 줄에 hit cue 부여 | `marks the damage_applied row with data-cue="hit"; the other 5 template ids carry no data-cue` |
| M4 | `top_damage_dealt_id` null에 "없음" 대체 문구 | `hides top_damage_dealt_id / top_damage_taken_id lines entirely when null` |
| M5 | 말 id 이스케이프 제거 | `escapes a piece id containing markup` |
| M6 | 생략 개수 줄 삭제 | `truncates core_log at 40 rows and states the omitted count explicitly` |
| M7 | `(수치 없음)` → `0` | `damage_applied: value_hundredths null shows an explicit "no value" marker, never 0` |
| M8 | 알 수 없는 template_id를 빈 문자열로 버림 | `unknown template_id falls back to a line that exposes the id, never dropped` (+1건) |
| M9 | hundredths 반올림 → 절삭 | `rounds half up for both signs, matching Rust round_hundredths_to_int` (+2건) |

### 실화면 실측 — 단위 테스트가 잡지 못한 결함 3건

게이트가 남아 있어 `npm run qa:storybook:visual`은 이 화면에 도달할 수 없다.
대신 `web/dist`의 실제 빌드 CSS를 링크한 임시 하네스 HTML을 만들어
**320 / 390 / 1280** 폭에서 실측했다(하네스는 커밋하지 않았다).

테스트 107건이 전부 통과한 상태에서 화면은 세 군데 틀려 있었다:

1. **cue 표식이 겹쳐 하나만 보였다.** 도전자는 결착 시점에 `hit` +
   `balance_broken` + `incapacitated` 세 cue를 동시에 갖는데, 표식을 각각
   같은 top-right 오프셋으로 절대 배치해 마지막 하나만 보였다. flex
   컨테이너에 담아 나란히 놓고, 컨테이너 존재를 테스트로 고정했다.
2. **적 말이 거의 보이지 않았다.** 종이색 배경에 종이색 채움 + 1px 파선이라
   `敵` 글리프가 읽히지 않았다. 짙은 종이색으로 채우고 윤곽을 2px로 올렸다.
   진영 신호는 여전히 색 단독이 아니다 — 채움 대비 + 파선/실선 + `我`/`敵`.
3. **데스크톱에서 보드 너비가 764px 칸 안에서 420px로 축소됐다.** 두 축 모두
   stretch인 grid item에 `aspect-ratio`와 `max-block-size`가 함께 걸리면
   브라우저가 높이 상한을 만족시키려고 **너비**를 줄인다. `inline-size: 100%`로
   너비를 확정해 비율이 높이를 정하게 했다.

또 리뷰에서 별도로 2건 고쳤다:

4. **말이 보드 경계에서 절반 잘렸다.** 말은 `translate: -50% -50%`로 중심을
   좌표에 맞추므로 투영 범위 0~100%는 최소·최대 좌표의 말을 자른다. 이
   인카운터는 전투원이 2명이라 두 말이 **항상** 극단에 놓여 둘 다 잘렸다 —
   예외가 아니라 기본 경우다. 여백을 둔 띠(14~86%)로 투영한다. 대칭 여백이라
   span 0은 여전히 정확히 50%다.
5. **테스트 이름이 검증하지 않는 것을 주장했다.** `I5: renders byte-identical
   output …`은 실제로 관전 마크업 부재만 확인한다. 삽입 지점이 템플릿 리터럴
   한 줄이라 공백 줄은 남으므로 바이트 동일이 아니다. 이름을 실제 검증 내용에
   맞췄다.

수정 후 실측: 보드:로그 = **0.700** (320/390/1280 전부), 가로 스크롤 0,
cue 겹침 0, 모든 표식이 보드 안, 말 경계 여백 25/32/87px.

## 하드 불변식 이행

| # | 규칙 | 이행 |
|---|---|---|
| I1 | 판정 재계산 0회 | `resolve`/`conclude`/`spectate` 호출 없음. 계산은 hundredths 반올림·tick→ms·좌표 투영·배열 길이뿐 |
| I2 | 신규 색상 리터럴 0개 | diff grep 0건 (`var(--seal-red)` 등 토큰 참조만) |
| I3 | 3분할 침범 금지 | `.storybook-shell`/`.game-viewport`/`.game-topbar`/`.storybook-hud`/`.storybook-dock` 무수정 (CSS diff에 삭제 줄 0) |
| I4 | 70:30 | `grid-template-rows: minmax(0, 70fr) minmax(0, 30fr)`, 실측 0.700 |
| I5 | `combat` 없으면 마크업 0 | `I5: emits no combat markup at all when page.combat is absent` |
| I6 | 6개 문장 terminal 일치, 알 수 없는 id 노출 | `combatLogTemplates.test.ts` 11건 (M7·M8·M9로 확인) |
| I7 | `full_log` 개수만, 생략 개수 명시 | `shows the full_log count`, `truncates core_log at 40 rows…` (M6) |
| I8 | 없는 항목 숨김, 금지 문구 부재, fingerprint+version | `hides top_damage…`, `hides the decisive_tick line when null`, `never contains forbidden strategic-analysis phrases`, `puts the fingerprint and simulation_version in the same element` (M4) |
| I9 | semantic table + forced-colors + 색 단독 의존 없음 | `includes every piece id, side, and coordinate in the semantic alternative table` + `@media (forced-colors: active)` 블록 + `我`/`敵` 글리프 |
| I10 | 마지막 프레임, span 0 가드, 무예외 | `renders only the last frame`, `centers pieces at 50%…`(M1), `handles an empty frames array`, `handles a frame with zero pieces` |
| I11 | `damage_applied`만 hit cue 동기화 | `marks the damage_applied row with data-cue="hit"…` (M3) |
| I12 | 지정 영역 무변경 | `git diff --name-only origin/main..HEAD` 10개 파일 전부 `web/` 또는 `docs/design/` |
| I13 | 전부 `escapeHtml` | `escapes a piece id containing markup` (M5) |

## 플랜 문서 정정

플랜 §6의 outcome/reason 한국어 라벨 표가 실제 terminal 소스와 어긋난 초안
문구였다(`적군 승리`/`양측 패배`/`교착`/`종료 조건 미충족`). 실제
`crates/escape-terminal/src/snapshot.rs`는 `적 승리`/`양측 전멸`/`무승부`/
`종료 조건 없음`을 쓴다. subagent가 플랜 표 대신 Rust 소스를 기준으로 삼은
것이 맞는 판단이었고, 플랜 §6을 소스 문자열로 고치고 "표를 믿지 말고 소스를
열어라"는 지시를 명시했다.

## 알려진 갭 (발명하지 않고 남긴 것)

**전투원 표시 이름이 없다.** 관전 화면과 보고서가 내부
id(`wuxia_spectator_bout_ally`)를 그대로 플레이어에게 보여준다.
`CombatSpectatorPiece`·`CombatCombatantReport`에도, 인카운터 combat authoring
에도 표시 이름 필드가 없어 renderer가 유도할 수 없다. 이름을 만들어내지 않고
문서에 갭으로 기록했다 — **게이트를 푸는 1d-3 전에 authoring에 이름 필드를
추가할지 결정해야 한다.**

## 범위 밖 (Step 1d-3으로 남김)

- 재생 연출(틱 기반 모션) — 같은 DOM·같은 `--piece-x`/`--piece-y` 위에
  `@media (prefers-reduced-motion: no-preference)` 안에서만 얹는다.
  `reduce`에서는 이 슬라이스의 정지 프레임이 그대로 최종 상태다(재작업 아님).
- 로그 실시간 동시 노출 타이밍
- 게이트 플래그 제거, wasm 재빌드, 5뷰포트 실화면 QA
- 전체 로그 열람 UI, 일시정지 흐름
- 개입 기회/대응 제시 → Step 2c
- 치유·명줄·패배 결과, 밸런스 확정
