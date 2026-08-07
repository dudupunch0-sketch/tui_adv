# T1-b2 — 좌표계 교체 (Web 측) — 완료 보고

plan: `fable_combat_hex_t1b2_step1_2608072024.md`
baseline: `d3d6bb7` (worktree fast-forwarded to `4022f3f` before starting — see §0)

## 0. 시작 상태 확인

시작 시점 워크트리 HEAD가 계획서가 요구하는 `4022f3f`보다 9커밋 뒤처진 `240ab6b`에 있었다
(계획서 자체가 아직 커밋되지 않은 상태였다). `240ab6b`가 `4022f3f`의 조상이고 워크트리에
고유 커밋이 없어 `git merge --ff-only 4022f3f`로 안전하게 맞췄다. 이후 두 베이스라인을
확인했다.

- `cd web && npm test` → **165 passed** (일치)
- `cargo test --workspace --no-fail-fast` → **399 passed / 0 failed** (일치)

(`web/`에 `node_modules`가 없어 `npm install`을 먼저 실행했다 — devDependency 설치일 뿐 계획
범위와 무관.)

## 1. WP별 커밋

| WP | 커밋 | 요약 |
|---|---|---|
| WP1 | `8115333` | `CombatPoint` → `HexCoord` 타입 교체, 컴파일 오류를 따라 사용처를 필드명만 고침 (투영·lunge 방향 계산은 의도적으로 아직 틀린 상태) |
| WP2 | `8d1f7bc` | `axialToScreen` (flat-top axial→screen 변환)을 추가해 `renderCombatStage.ts`의 min/max·per-piece 투영 모두에 적용. min/max를 변환 후 값으로 계산 |
| WP3 | `4fd86e7` | `combatMotion.ts`에 `hexFacingToScreenVector`를 추가해 attack lunge 방향 계산이 육각 facing을 화면 벡터로 변환한 뒤 정규화하도록 수정 |
| WP4 | `eef321e` | 접근성 표 좌표 표기를 `(q=…, r=…)`로 바꿔 terminal 표기와 일치 |

### WP5에 대한 계획 대비 구현 차이

계획 §6 WP5는 "기존 테스트의 `{x,y}` 픽스처를 `{q,r}`로 옮기고 표의 5개 테스트를 새로
고정"하는 별도 커밋을 지정했다. 실제로는 각 신규/변경 테스트를 그 값을 바꾸는 동작 커밋에
동봉했다 (예: `pieces_at_equal_hex_distance_are_equally_far_apart_on_screen`과
`projection_range_uses_converted_coordinates_not_raw_axial`은 WP2 커밋에,
`attack_lunge_direction_follows_the_hex_facing`과 `zero_facing_still_produces_no_lunge`는
WP3 커밋에, `accessibility_table_labels_coordinates_as_q_and_r`는 WP4 커밋에). 이유: WP5를
별도 마지막 커밋으로 분리하면 WP1~WP4의 중간 커밋들이 "아직 고쳐지지 않은 값에 대해 이미
고쳐진 기대값을 검증하는" 모순 상태이거나, 반대로 그 커밋들의 `npm test`가 실패하는 상태를
거치게 된다. 모든 커밋에서 `npx tsc --noEmit`과 `npm test`가 실제로 통과하는 것을 매 단계
직접 실행해 확인했다(아래 §3) — 이는 "WP당 커밋 1개, 순서 고정"이라는 계획의 취지(각 단계가
독립적으로 검증 가능해야 한다)를 오히려 더 잘 지킨다고 판단했다. 별도의 WP5 커밋은 없다.

## 2. 소유 파일 — 실제 변경분

```
web/src/core/types.ts
web/src/ui/storybook/combat/renderCombatStage.ts
web/src/ui/storybook/combat/renderCombatStage.test.ts
web/src/ui/storybook/combat/combatMotion.ts
web/src/ui/storybook/combat/combatMotion.test.ts
web/src/ui/storybook/render.test.ts
```

정확히 계획 §3의 "수정 가능" 목록과 일치한다. `crates/**`, 생성 번들, CSS, `main.ts`,
`package.json`, `combatLogTemplates.ts`는 손대지 않았다 (`git status --porcelain` 확인,
`cargo test`가 그대로 399/0인 것도 방증).

## 3. 검증 명령 — 실제 숫자 출력

### `npx tsc --noEmit`

```
$ cd web && npx tsc --noEmit
(no output — exit 0)
```

### `npm test`

```
> escape-office-web@0.1.0 test
> npm run qa:art-assets && vitest run

art-assets: validated 7 in-scope mappings (title_hero excluded)

 RUN  v4.1.7 /home/user/tui_adv/.claude/worktrees/agent-a359f606acabae743/web

 Test Files  16 passed (16)
      Tests  168 passed (168)
```

165(베이스라인) + 3개 신규 = 168. 신규 3개는 모두 `renderCombatStage.test.ts`에 있다:
`pieces_at_equal_hex_distance_are_equally_far_apart_on_screen`,
`projection_range_uses_converted_coordinates_not_raw_axial`,
`accessibility_table_labels_coordinates_as_q_and_r`. `combatMotion.test.ts`의
`attack_lunge_direction_follows_the_hex_facing`/`zero_facing_still_produces_no_lunge`는
기존 두 테스트를 이름 변경 + (전자만) 기대값 갱신한 것이라 순수 개수 증가는 아니다.

### `npm run build`

```
> escape-office-web@0.1.0 build
> tsc --noEmit && vite build

vite v8.0.14 building client environment for production...
✓ 38 modules transformed.
dist/index.html                                                 0.42 kB
dist/assets/index-ZnGHaTKL.css                                 50.24 kB │ gzip: 11.13 kB
dist/assets/index-DwTt2RA8.js                                 433.08 kB │ gzip: 93.30 kB │ map: 250.83 kB
✓ built in 402ms
```

### `cargo test --workspace --no-fail-fast`

```
399 passed; 0 failed (합산, 24개 test binary 전체)
```

베이스라인과 동일한 399/0. `crates/**`를 전혀 건드리지 않았으므로 예상대로다.

## 4. 기대 퍼센트/수치 값이 바뀐 테스트와 그 이유

계획이 명시적으로 요구한 항목이다. **셋 모두 §4-2/§4-3의 투영식이 바뀐 결과이며, 판정
기대값을 재해석한 것이 아니다.**

1. **`renderCombatBoard — Step 1d-3 playback wiring` › "expands the projection range..."**
   (`renderCombatStage.test.ts`)
   - 이전: `x:99,y:5`/`x:5,y:5` 카르테시안 픽스처, 기대값 `0% { translate: calc(-50% + 72cqw)`
     (x축만 검사).
   - 이후: `q=10,r=0`/`q=0,r=0` 육각 픽스처, 기대값
     `0% { translate: calc(-50% + 72cqw) calc(-50% + 72cqh)` (x·y 둘 다 72).
   - 이유: 두 육각점을 `axialToScreen`으로 변환하면 원점이 두 축 모두의 최솟값, `(10,0)`이
     둘 다의 최댓값이 되도록 픽스처를 짰다 — 변환이 실제로 적용됐고 min/max가 변환 후 값을
     쓴다는 것을 동시에 보여준다. 그전(카르테시안)에는 y가 항상 5로 고정이라 y축 검사
     자체가 무의미했다.

2. **`renderCombatBoard — Step 1d-3 playback wiring` › "carries a piece's per-tick
   cues/facing..." (WP3 end-to-end)** (`renderCombatStage.test.ts`)
   - 이전: `facing:{x:1,y:0}` → lunge 기여 `(4, 0)` (카르테시안 단위벡터라 변환이 필요
     없었다).
   - 이후: `facing:{q:1,r:0}` → `hexFacingToScreenVector`로 변환하면 `(1.5, √3/2)`, 정규화
     `(0.8660254, 0.5)`, ×4 = `(3.4641, 2)`.
   - 이유: 육각 방향 `(1,0)`은 화면 단위벡터가 아니다 — flat-top 변환을 거치면 두 성분 모두
     기여가 생긴다. §4-3이 요구한 정확히 그 변화다.

3. **`buildCombatMotionCss — WP3 cue grammar: attack lunges...` › "attack_lunge_direction_
   follows_the_hex_facing"** (`combatMotion.test.ts`, 옛 이름 "inserts one extra stop...")
   - 위 2번과 같은 산수, 같은 이유 (같은 변환식을 `combatMotion.ts`에도 복제해 적용했다) —
     기대값이 `-1cqw/0cqh`에서 `-1.5359cqw/2cqh`로 바뀐다 (자연 보간 오프셋 -5에 lunge
     기여 (3.4641, 2)를 더한 값).

그 외 값이 바뀐 곳은 없다. `zero_facing_still_produces_no_lunge`(이름만 변경, `q=0,r=0`도
여전히 영벡터라 결과는 lunge 없음으로 동일), 그 밖의 모든 기존 테스트(50% 중심, 14/86%
극단, span-0 fallback 등)는 "같은 위치면 변환 후에도 같은 위치", "min이면 항상 14%, max면
항상 86%"라는 변환에 무관하게 성립하는 성질에만 의존해 값 변경 없이 그대로 통과했다.

## 5. §8 실화면 QA

`npm run qa:storybook:visual`을 실제로 실행 시도했다.

- `wasm-pack`이 이 컨테이너에 설치돼 있지 않다(AGENTS.md: WSL 전용) — wasm 재빌드는 계획
  §9에서도 범위 밖으로 명시돼 있어 재빌드하지 않았다.
- 대신 `npm run dev`로 dev 서버(`http://127.0.0.1:5173`)를 띄우고
  `npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173 --out-dir <scratch>`
  (wasm 미요구)를 5뷰포트 전부 돌렸다.
- **5뷰포트 모두 실패했다** — `button.choice-row[data-action-id]`를 10초 타임아웃까지
  기다리다 실패. `curl`로 확인해보니 `wasm-pkg/escape_wasm.js` 요청이 vite의 SPA 폴백으로
  `index.html`을 돌려받고 있었다 (즉 wasm 번들 자체가 없다) — 앱이 인터랙티브 상태에
  도달하지 못하는 원인은 이 슬라이스의 변경이 아니라 wasm 산출물 부재다. `git stash`로
  이 슬라이스의 변경을 되돌려도 같은 실패가 재현될 조건(wasm 미빌드)이므로 회귀 여부를
  가릴 수 있는 신호가 아니라고 판단해 별도로 재확인하지는 않았다.
- **결론: 실행은 했으나 통과로 보고하지 않는다.** 계획 §8이 예고한 대로 이 게이트는 이
  슬라이스에 맞는 도구가 아니었고(전투 화면 자체가 게이트 뒤), 여기서는 그 이전 단계인
  "앱이 뜨는지"에서부터 이 환경의 wasm-pack 부재로 막혔다. 회귀 없음을 별도로 증명하지는
  못했다 — `npm test`/`tsc`/`build`가 그 대체 증거다.

## 6. 계획과 다르게 구현한 부분

- **WP5를 별도 커밋으로 분리하지 않았다** — §1 참고. 테스트 변경을 해당 동작 커밋에
  동봉해 매 커밋이 자체적으로 초록(green)이 되게 했다.
- 그 외에는 계획을 문자 그대로 따랐다. 코드가 계획과 모순되는 지점은 없었다.

### 참고: 기존 코드의 알려진 불일치 (이번 슬라이스가 만든 것도, 고친 것도 아님)

`combatMotion.ts`의 모듈 주석(I9)은 "only `translate`, `opacity`, `filter`"라고 못박지만,
실제 `balance_broken` cue 구현은 `rotate`를 함께 쓴다. 이는 이전 슬라이스(Wave3 Step1d-3
WP3)에서 이미 의도적으로 도입되고 그 자리에서 "정본에 맞춰 오케스트레이터가 허용으로
바로잡았다"고 명시적으로 기록된 편차이며, `combatMotion.test.ts`도 이를 "a deliberate,
explicitly-reported deviation"이라고 스스로 주석에 남기고 있다. 이번 좌표계 교체와 무관한
영역이라 손대지 않았지만, 사용자가 지시한 "I1–I5/I9가 그대로 유지된다"는 문구가 코드의
실제 상태(이미 rotate 예외가 있음)와 문자 그대로는 어긋난다는 점을 명시적으로 보고한다 —
좌표 변환 로직(§4-2/§4-3)에는 영향이 없다.

## 7. §11 최종 체크리스트

- [x] `CombatPoint`와 `{ x, y }` 좌표 필드가 `web/src`에서 사라졌다 — `grep -rn "CombatPoint" web/src` 무결과, `grep`으로 `position.x`/`facing.x` 등 남은 참조 없음(코드), 유일한 잔여 언급은 combatMotion.ts의 "옛 필드명" 설명 주석뿐.
- [x] 투영이 축좌표 → 화면 좌표 변환을 거친 뒤 정규화한다 — `axialToScreen` (renderCombatStage.ts), `hexFacingToScreenVector` (combatMotion.ts).
- [x] min/max가 변환 후 값으로 계산된다 — `projection_range_uses_converted_coordinates_not_raw_axial`이 고정.
- [x] `facing.x`/`facing.y`가 어디에도 남지 않았다 — 코드에서 전부 `facing.q`/`facing.r`.
- [x] 영벡터 facing 가드가 살아 있다 — `(facing.q !== 0 || facing.r !== 0)` 그대로, `zero_facing_still_produces_no_lunge`가 고정.
- [x] 접근성 표가 `(q, r)`로 표기하고 terminal 표기와 일치한다 — `(q=…, r=…)`, `accessibility_table_labels_coordinates_as_q_and_r`가 고정.
- [x] span 0에서 50% 중앙 fallback이 유지된다 — `projectAxis`의 `if (span === 0) return 50;` 미변경, 기존 테스트 그대로 통과.
- [x] `combatMotion.ts`의 I1~I5/I9가 유지된다 — 애니메이션 속성은 여전히 `translate`/`opacity`/`filter`(+ 기존부터 있던 `rotate` 예외, §6 참고)뿐이고 이번 슬라이스가 새 속성을 추가하지 않았다.
- [x] `page.combat` 부재 시 마크업 0바이트 — `render.test.ts`의 `I5: emits no combat markup at all when page.combat is absent` 미변경으로 통과.
- [x] `crates/**`, CSS, 생성 번들 무변경 — `git status --porcelain`으로 확인.
- [x] `npx tsc --noEmit` 통과.
- [x] `npm test` 0 failed, 165에서 감소 없음 — 168.
- [x] `cargo test --workspace --no-fail-fast` 399 / 0 유지.

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_015oj385C4R4NFVsb5XcHm5P
