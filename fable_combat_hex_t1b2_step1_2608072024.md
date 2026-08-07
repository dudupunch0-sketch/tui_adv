# T1-b2 — 좌표계 교체 (Web 측)

plan: `fable_combat_hex_t1b2_step1_2608072024.md`
report: `fable_combat_hex_t1b2_step2_report.md`
baseline: `d3d6bb7`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T1
선행 슬라이스: T1-b1(`fable_combat_hex_t1b1_step1_2608071921.md`) — 머지 완료

## 1. 목적

T1-b1이 Rust를 육각으로 옮기면서 **의도된 불일치**를 남겼다. 생성 번들과 `ScenePage.combat`의
좌표는 이미 `{q, r}`인데 `web/src/core/types.ts`의 `CombatPoint`는 아직 `{x, y}`를 말한다.

이 슬라이스가 그 정합을 맞춘다. 전투가 `combat_spectator_preview_unlocked` 게이트 뒤에 있어
현재 web 테스트 165개가 그대로 통과하지만, **타입이 거짓말을 하고 있는 상태**다.

## 2. 선행 조건

- T1-b1 머지됨 (`simulation_version: v3`, 좌표 `{q, r}`).
- 시작 전 `cd web && npm test`가 **165 passed**인지 확인한다. 다르면 멈추고 보고한다.

## 3. 소유 파일

수정 가능:

- `web/src/core/types.ts` (좌표 타입만)
- `web/src/ui/storybook/combat/renderCombatStage.ts`
- `web/src/ui/storybook/combat/combatMotion.ts`
- `web/src/ui/storybook/combat/renderCombatStage.test.ts`
- `web/src/ui/storybook/combat/combatMotion.test.ts`
- `web/src/ui/storybook/render.test.ts` (전투 픽스처가 좌표를 쓰는 부분만)

수정 금지:

- `crates/**` 전부 — Rust는 T1-b1이 끝냈다
- `web/src/data/generated/**` — export 산출물이며 이미 `{q, r}`이다
- `web/src/styles/storybook.css` — 시각 폴리시는 별도 step (아래 §8)
- `web/src/main.ts`, `web/package.json`
- `web/src/ui/storybook/combat/combatLogTemplates.ts` (좌표와 무관)
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. `CombatPoint`를 육각 좌표로 대체한다

Rust는 `CombatPosition`/`CombatFacing`을 지우고 **위치와 방향 모두 `HexCoord` 하나**로 표현한다.
TS도 그 구조를 그대로 비춘다.

```ts
/** escape-core `HexCoord` — 위치와 facing 양쪽에 쓰이며 { q, r }로 직렬화된다. */
export interface HexCoord {
  q: number;
  r: number;
}
```

`CombatPoint`라는 이름과 `{ x, y }` 필드를 남기지 않는다. 주석의 `CombatPosition / CombatFacing`
언급도 지금은 존재하지 않는 타입이므로 함께 고친다.

### 4-2. 투영 — 이 슬라이스의 핵심이자 유일한 함정

현재 `renderCombatStage.ts`는 `projectAxis(value, min, max)`로 **각 축을 독립적으로** 0~100%에
정규화한다. 직교 좌표에서는 맞았지만 **축좌표에서는 틀린다.** `q`와 `r`은 화면의 직교축이 아니라
60도로 벌어진 두 축이라, 그대로 정규화하면 판이 기울어져 보이고 **같은 육각 거리에 있는 말들이
화면에서 서로 다른 거리로 보인다.**

그러므로 정규화 **전에** 축좌표를 화면 좌표로 변환하는 단계를 넣는다.

T1-a가 좌표계를 **flat-top**으로 문서화했으므로 변환식은 이것이다.

```
px = 1.5 * q
py = sqrt(3) * (r + q / 2)
```

단위 크기는 1로 둔다 — 어차피 뒤의 정규화가 판 크기에 맞춰 다시 스케일한다.

**주의 (놓치기 쉬움).** `minX`/`maxX`/`minY`/`maxY`를 **변환된 `px`/`py`에 대해** 구해야 한다.
지금 코드는 원시 좌표로 min/max를 잡는다. 변환 후 값으로 바꾸지 않으면 판 밖으로 삐져나간다.

부동소수점을 써도 된다. **결정론 불변식은 core의 판정에만 적용되고 렌더러의 화면 배치에는
적용되지 않는다** — 지금도 퍼센트 계산에 부동소수점을 쓴다. 단, core가 이미 정한 값을 다시
계산하지 않는다는 규칙(정본 13)은 그대로다: 이 변환은 **표시 변환**이지 판정이 아니다.

### 4-3. `combatMotion.ts`의 facing

`attack` cue의 lunge 방향이 `Math.hypot(facing.x, facing.y)`로 정규화된다.
facing이 이제 육각 방향이므로 **§4-2와 같은 식으로 화면 벡터로 변환한 뒤** 정규화한다.
아무 데도 `facing.x`가 남아서는 안 된다.

영벡터 가드는 유지한다. Rust가 이제 facing을 6방향 중 하나로 제한하지만,
렌더러가 입력을 신뢰하는 코드로 바뀌면 안 된다.

`combatMotion.ts`의 I1~I5/I9 불변식(화면 시간 = 시뮬레이션 시간, 단일 선형 타임라인,
reduced-motion 래핑, 재계산 0회, `translate`/`opacity`/`filter`만 애니메이션)은 **전부 그대로다.**
이 슬라이스는 좌표의 출처만 바꾼다.

### 4-4. 접근성 대체 표

좌표 라벨이 `(x, y)`로 나간다. `(q, r)`로 바꾼다.
T1-b1이 terminal 쪽을 `q=`/`r=`로 이미 갱신했으므로 **두 렌더러의 표기를 일치시킨다.**

기존 접근성 계약은 유지한다 — `sr-only` 표, 색만으로 진영을 구분하지 않는 글리프,
`piece.active`를 "생존"이 아니라 "참전"으로 쓰는 규칙(`never calls an active piece "생존"` 테스트가 고정).

## 5. Hard invariants

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **렌더러는 판정하지 않는다.** 좌표 변환은 표시 변환이다. 피해·cue·승패·집계를 새로 만들지 않는다.
2. **`page.combat`이 없으면 마크업이 한 글자도 나오지 않는다.** `render.test.ts`의
   `I5: emits no combat markup at all when page.combat is absent`가 고정한다.
3. **신규 색상 리터럴 금지.** 이 슬라이스는 CSS를 건드리지 않는다.
4. **reduced-motion 최종 정지 상태**가 올바르게 유지된다.
5. **`crates/**` 무변경.**
6. **span이 0일 때 0으로 나누지 않는다** — 기존 `projectAxis`의 50% 중앙 fallback을 유지한다.
   변환 단계가 들어가면서 이 경계 조건이 사라지지 않게 한다.

## 6. WP 목록

순서 고정. WP당 커밋 1개.

### WP1 — 타입 교체

§4-1. `CombatPoint` → `HexCoord`. 컴파일 오류를 따라가며 사용처를 고친다.
이 시점에 투영은 아직 틀려도 된다(축좌표를 직교로 읽는 상태).

검증: `cd web && npx tsc --noEmit`

### WP2 — 투영 변환

§4-2. **min/max를 변환 후 값으로 잡는 것을 잊지 마라.**

검증: `cd web && npx vitest run src/ui/storybook/combat/renderCombatStage.test.ts`

### WP3 — facing과 모션

§4-3.

검증: `cd web && npx vitest run src/ui/storybook/combat/combatMotion.test.ts`

### WP4 — 접근성 표기

§4-4.

### WP5 — 테스트 갱신과 보강

기존 테스트의 `{x, y}` 픽스처를 `{q, r}`로 옮긴다. **기대 퍼센트 값이 바뀌는 것은 정상이다** —
투영식이 바뀌었기 때문이며, 이건 T1-b1에서 금지했던 "판정 기대값 수정"과 다른 종류다.
다만 **왜 바뀌었는지 설명할 수 없는 값 변화가 있으면 멈추고 보고한다.**

신규로 고정할 것:

| 테스트 | 고정하는 것 |
|---|---|
| `pieces_at_equal_hex_distance_are_equally_far_apart_on_screen` | §4-2가 실제로 고쳐졌다 |
| `projection_range_uses_converted_coordinates_not_raw_axial` | 놓치기 쉬운 함정 |
| `accessibility_table_labels_coordinates_as_q_and_r` | §4-4 |
| `attack_lunge_direction_follows_the_hex_facing` | §4-3 |
| `zero_facing_still_produces_no_lunge` | 가드 유지 |

검증: `cd web && npm test`

## 7. 검증 명령

```bash
cd web
npx tsc --noEmit
npm test
```

기대: **165에서 감소 없음**, 신규 테스트만큼 증가.

`npm run build`도 `tsc --noEmit`을 포함하므로 한 번 돌려 결과를 보고한다.

Rust는 이 슬라이스에서 변경이 없다. 확인 삼아 `cargo test --workspace --no-fail-fast`를 돌려
**399 passed / 0 failed**가 그대로인지 보고한다.

## 8. 실화면 QA에 대한 판단

AGENTS.md는 web 변경 시 `npm run qa:storybook:visual` 5뷰포트 통과를 공식 게이트로 정한다.
**이 슬라이스에는 그 게이트가 맞는 도구가 아니다** — 전투 화면은 여전히
`combat_spectator_preview_unlocked` 게이트 뒤에 있어 일반 플레이 경로의 어떤 뷰포트에도
나타나지 않는다. 즉 시각 QA는 아무 차이도 보여주지 못한다.

그래도 **회귀가 없음을 보이는 용도로는 유효하다.** dev 서버를 띄울 수 있으면 돌리고 결과를 적어라.
띄울 수 없으면 "실행 불가"라고 적고 사유를 남겨라 — **통과했다고 적지 마라.**

게이트를 풀고 실제 화면에서 검증하는 것은 이 슬라이스가 아니라 렌더러가 갖춰진 뒤의 별도 slice다.

## 9. 명시적 범위 밖

- **육각 타일을 실제로 그리는 것** — 격자선, 타일 배경, 점유 하이라이트는 T8/T9
- **고정 타일 메트릭 투영** — T9. 이 슬라이스는 기존 min/max 정규화를 유지하되 그 **입력만** 고친다
- 말 외형 교체 지점 분리 — T9
- 동적 카메라 — T9
- CSS·시각 폴리시 일체
- wasm 재빌드, 게이트 플래그 제거
- 점유·대형 유닛·포위 — T1-c/T1-d

## 10. 보고 형식

`fable_combat_hex_t1b2_step2_report.md`에 적고 커밋한다.

- WP별 커밋 해시와 한 줄 요약
- 검증 명령과 **실제 숫자 출력** (`npm test`, `tsc --noEmit`, `cargo test`)
- **기대 퍼센트 값이 바뀐 테스트 목록과, 각각 왜 바뀌었는지** — 이 항목을 빠뜨리지 마라
- §8의 시각 QA를 돌렸는지, 못 돌렸으면 사유
- 계획과 다르게 구현한 부분과 사유

## 11. 최종 체크리스트

- [ ] `CombatPoint`와 `{ x, y }` 좌표 필드가 `web/src`에서 사라졌다
- [ ] 투영이 축좌표 → 화면 좌표 변환을 **거친 뒤** 정규화한다
- [ ] min/max가 **변환 후** 값으로 계산된다
- [ ] `facing.x`/`facing.y`가 어디에도 남지 않았다
- [ ] 영벡터 facing 가드가 살아 있다
- [ ] 접근성 표가 `(q, r)`로 표기하고 terminal 표기와 일치한다
- [ ] span 0에서 50% 중앙 fallback이 유지된다
- [ ] `combatMotion.ts`의 I1~I5/I9가 유지된다 (애니메이션 속성은 여전히 `translate`/`opacity`/`filter`뿐)
- [ ] `page.combat` 부재 시 마크업 0바이트
- [ ] `crates/**`, CSS, 생성 번들 무변경
- [ ] `npx tsc --noEmit` 통과
- [ ] `npm test` 0 failed, 165에서 감소 없음
- [ ] `cargo test --workspace --no-fail-fast` 399 / 0 유지
