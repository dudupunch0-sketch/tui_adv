# Combat — 전체 로그 열람 설계 플랜 (Web)

Baseline: `1096909` (PR #191, 조기 결착)
브랜치: `claude/combat-full-log-viewer`
워크트리: `/home/dudu/worktrees/tui-adv-combat-early-conclusion`

## 0. 왜 이 슬라이스인가

현재 Web 관전 화면의 하단 메타 줄은 이렇게 말한다:

```
전체 로그 64건 (일시정지 또는 전투 종료 후 별도 열람, 이 화면은 개수만 표시)
```

"별도 열람"이 아직 존재하지 않는다. 정본이 열람 가능해야 한다고 정한 데이터가
페이로드에는 이미 다 있는데(`view.full_log`) 도달 경로가 없다.

## 1. 정본 근거

- **정본 13 (이중 로그와 정보 계약)**: "전투 중 하단에는 핵심 로그만 출력하되,
  모든 공격·이동·판정은 전체 로그에 저장하여 **일시정지 또는 전투 종료 뒤
  열람할 수 있게 한다.**"
- **정본 07**: "전체 로그와 상세 수치는 일시정지 또는 전투 종료 뒤 별도로 열람."
- **정본 13 (중요도)**: "중요도는 **일반(전체 로그만), 중요(핵심 로그),
  결정적(항상 강조)**으로 나눈다." → 열람 화면은 중요도를 구분해 보여주고
  `decisive`를 강조한다. `CombatSpectatorLogEntry.importance`가 이미 값을 준다.
- **정본 13 (누설 금지)**: "알려지지 않은 적 능력과 숨은 판정은 누설하지
  않는다." → `view.full_log`는 core가 이미 누설 차단을 마친 배열이다
  (`AttackRoll`/`EffectSuppressed` 제외, Hidden/Conditional 효과 id 마스킹).
  **renderer는 `view.full_log`만 읽는다** — resolution·execution 레벨 로그에
  손대지 않는다.
- **정본 13 (로그 계약)**: "로그는 자유 생성 문장이 아니라 등록된 사건 태그와
  로그 템플릿을 사용한다." → 이미 있는 `combatLogTemplates.ts`를 그대로 쓴다.
  새 문장을 만들지 않는다.

### 이 화면에서 "열람 가능" 시점

이 슬라이스가 다루는 인카운터는 시스템형이며 개입 예산 0이라, `ScenePage`가
렌더될 때 전투는 **이미 종료**돼 있다(`combat.report`가 `Some`). 즉 정본의
"전투 종료 뒤"에 해당한다. **일시정지 흐름은 만들지 않는다** — 일시정지는
개입(Step 2c) 소관이다.

전투가 진행 중일 때(`report`가 `None`)는 열람 진입점을 **만들지 않는다.**
정본이 허용하는 시점이 아니다.

## 2. Scope

`view.full_log`를 열람할 수 있는 접힘 영역을 관전 표면 하단에 추가한다.

1. `renderCombatStage.ts`: 전체 로그 열람 `<details>` 섹션.
2. 중요도 3단계 표시 (`decisive` 강조, `important`, `routine`).
3. tick·sequence를 함께 보여 순서를 읽을 수 있게 한다.
4. `storybook.css`: 수묵 토큰 스타일.
5. 테스트.
6. 문서 갱신 + 메타 문구 수정("이 화면은 개수만 표시" → 열람 가능 안내).

## 3. Hard invariants

### I1. `view.full_log`만 읽는다

`resolution`·`execution` 레벨 로그에 접근하지 않는다. 누설 차단은 core가 이미
했고 renderer가 그 경계를 넘지 않는다. `page.combat.view.full_log` 외의
어떤 배열도 이 화면의 소스가 아니다.

### I2. 전투 종료 뒤에만 열람 진입점을 만든다

`combat.report`가 `undefined`(전투 진행 중)면 열람 섹션을 **만들지 않는다.**
메타 줄의 문구도 그때는 열람 가능하다고 말하지 않는다.

### I3. 문장은 기존 템플릿 테이블을 재사용한다

`combatLogTemplates.ts`의 `combatLogTemplateLine`을 그대로 쓴다. 새 문장 형식을
만들지 않는다. 알 수 없는 `template_id`의 fallback도 그대로 유효하다.

### I4. 조용한 truncation 금지

전체 로그는 **전부** 보여준다. 상한을 두지 않는다 — 상한을 두면 "전체 로그"가
아니게 된다. 길이가 문제면 스크롤로 처리하고, DOM에서 빼지 않는다.

핵심 로그 영역의 40줄 상한(`WEB_CORE_LOG_LIMIT`)은 그대로 둔다. 그건 전투 중
하단에 "핵심 로그만" 출력하라는 정본 요구를 따른 것이고, 생략 개수도 이미
명시한다.

### I5. 중요도는 데이터에서만 온다

`entry.importance`를 그대로 쓴다. 어떤 사건이 중요한지 renderer가 다시 판단하지
않는다. 세 값(`routine`/`important`/`decisive`)에 각각 시각 구분을 준다.

### I6. 핵심 로그와 중복 표시가 아님을 드러낸다

`core_log`는 `full_log`의 부분집합이다(importance >= important). 열람 화면은
전체를 보여주므로 핵심 로그 줄이 다시 나온다 — 이게 중복 버그로 읽히지 않도록
`core_log`에도 있는 줄임을 표시한다(`data-in-core-log="true"`). 판정은
renderer가 하지 않고 `importance`에서 유도한다(그 필터가 core의 정의다).

### I7. 수묵 토큰만, 신규 색상 리터럴 0개

기존 14개 토큰만. `color-mix(in oklab, ...)` 허용. 새 이미지·SVG 없음.

### I8. 3분할 레이아웃과 70:30 유지

열람 섹션은 보드:로그 70:30 그리드 **밖**, 보고서와 같은 층(표면 아래 일반
흐름)에 둔다. `.combat-stage`의 그리드 행을 건드리지 않는다.
`.storybook-shell`/`.game-viewport`/`.game-topbar`/`.storybook-hud`/
`.storybook-dock` 규칙 무수정.

### I9. 접근성

- `<details>`/`<summary>`를 쓴다 — 코드베이스의 기존 드로어 관용구이며 키보드
  조작과 접근성을 브라우저가 준다. 커스텀 토글을 만들지 않는다.
- `<summary>`는 건수를 포함해 무엇이 열리는지 말한다.
- 로그 목록은 `<ol>`로 순서를 의미로 표현한다.
- 중요도를 **색으로만** 전달하지 않는다 — 텍스트 라벨 또는 글리프를 함께 준다.
- `@media (forced-colors: active)` 대체를 준다.
- 터치 타겟: `<summary>`의 최소 높이를 24px 이상으로 둔다 (`min-block-size`).

### I10. 모션 금지

이 슬라이스는 애니메이션을 추가하지 않는다. `<details>` 열림 트랜지션도 만들지
않는다 — 필요하면 별도 슬라이스에서 `prefers-reduced-motion` 안에서 다룬다.

### I11. 이스케이프

모든 데이터 문자열은 `escapeHtml`을 통과한다.

### I12. 건드리지 않는 것

- `crates/` 전체 (core 무변경 → simulation version 그대로)
- YAML, 두 번들 JSON
- 게이트 플래그
- `web/src/core/types.ts` (`full_log`는 이미 타입에 있다)
- `web/src/main.ts`, `web/package.json`
- `combatMotion.ts`
- `crates/escape-terminal/**` (terminal은 개수만 표시하는 현재 계약 유지 —
  terminal 열람 UI는 별도 슬라이스다. 이 비대칭을 문서에 남긴다.)

## 4. 예상 변경 파일

| 파일 | 변경 |
|---|---|
| `web/src/ui/storybook/combat/renderCombatStage.ts` | 열람 섹션, 메타 문구 |
| `web/src/ui/storybook/combat/renderCombatStage.test.ts` | 신규 테스트 |
| `web/src/styles/storybook.css` | 열람 섹션 스타일 |
| `docs/design/Mobile_Ink_Storybook_UI.md` | 열람 계약 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 항목 이동 |

## 5. DOM 골격 (권장)

```
<details class="combat-full-log" data-region="combat-full-log">
  <summary>전체 로그 64건 열람</summary>
  <ol class="combat-full-log__list">
    <li class="combat-full-log__row"
        data-importance="decisive"
        data-in-core-log="true"
        data-template-id="combat.log.damage_applied">
      <span class="combat-full-log__tick">t8·2</span>
      <span class="combat-full-log__importance">결정적</span>
      … 문장 …
    </li>
  </ol>
</details>
```

중요도 라벨: `routine` 일반 / `important` 중요 / `decisive` 결정적
(정본 13의 용어를 그대로 쓴다).

## 6. 작업 패키지

### WP1 — 열람 섹션 + 테스트 (테스트 먼저 red)
테스트가 고정할 것:
- `full_log`의 **모든** 줄이 나온다 (상한·생략 없음).
- `report`가 없으면 `combat-full-log` 섹션이 출력에 없다.
- 중요도 3종이 각각 `data-importance`와 한국어 라벨로 나타난다.
- `importance >= important`인 줄에 `data-in-core-log="true"`가 붙고
  `routine` 줄에는 붙지 않는다.
- tick·sequence가 나온다.
- 문장이 `combatLogTemplateLine`과 같다 (템플릿 재사용).
- 알 수 없는 `template_id`도 버려지지 않는다.
- 데이터 문자열이 이스케이프된다.
커밋: `feat(web): let the player read the full combat log after the fight`

### WP2 — 메타 문구 수정 + 테스트
"이 화면은 개수만 표시" → 열람 가능 안내. `report`가 없을 때는 열람 가능하다고
말하지 않는다.
커밋: `feat(web): say the full log is readable once the fight has ended`

### WP3 — CSS
수묵 토큰, 중요도 구분(색 + 텍스트), forced-colors 대체, 내부 스크롤,
`<summary>` 터치 타겟. 애니메이션 0개.
커밋: `style(web): ink-wash styling for the full combat log viewer`

### WP4 — 문서
`Mobile_Ink_Storybook_UI.md`에 열람 계약(전투 종료 뒤에만, 상한 없음, 중요도
표시, `view.full_log`만 읽음). 인덱스에서 "전체 로그 열람 UI"를 Web 완료로
옮기고 **terminal은 여전히 개수만 표시**임을 명시. 일시정지 흐름은 Step 2c로
남긴다.
커밋: `docs(combat): record the full-log viewing contract`

## 7. 검증

```bash
cd web && npx tsc --noEmit
cd web && npx vitest run
cargo test --workspace --no-fail-fast   # core 무변경 확인
git diff --check
```

색상 리터럴 grep, 애니메이션 선언 부재 grep.

**오케스트레이터 실측**: 임시 하네스로 320/390/1280에서 확인 —
`<details>`를 열었을 때 가로 스크롤이 생기지 않는지, 64줄이 내부 스크롤로
처리되는지, 중요도 구분이 색 없이도 읽히는지, 3분할 레이아웃이 깨지지 않는지.

## 8. 명시적 범위 밖

- 일시정지 중 열람 (일시정지 흐름 자체가 Step 2c)
- terminal 쪽 열람 UI
- 검색·필터·정렬 (정본에 요구 없음)
- 상세 수치 확장(내부 판정 노출) — 정본 13이 숨은 판정 누설을 금지한다
- 게이트 제거, 전투원 표시 이름
- 2배속·즉시 결과, 프리셋·재도전
- core 변경 일체
