# Web Storybook 후속 정리 지시서 (Plan by Fable, implementation by Gemini)

Date: 2026-07-11
Baseline: `main` HEAD `0aa6346` ("feat(web-ui): ink wash storybook redesign + UX fixes (#126)")
Branch 규칙: 최신 `origin/main`에서 `gemini/ui-cleanup-labels` 브랜치를 만든다.
작업 위치: **WSL clone `~/work/tui-adv`** (Windows repo 아님. `git`/`npm`/`pytest` 전부 WSL에서 실행).

이 문서는 PR #126(수묵 리디자인) 리뷰에서 발견된 잔여 결함 4건의 완전한 수정
지시서다. 워크패키지를 순서대로 실행하고, WP마다 검증 → 커밋 1개를 만든다.
전부 끝나면 브랜치를 push하고 `main` 대상 PR을 연다.

---

## 0. 절대 불변 (위반 시 리뷰 반려)

1. `web/` 밖은 수정 금지 (이 문서와 보고서 파일 제외).
2. `ScenePage` JSON 소비 전용 — 게임 규칙 재계산 금지, 필드 추가/개명 금지.
3. 다음 계약 셀렉터/속성은 반드시 유지 (QA 스크립트와 테스트가 검사):
   `[data-renderer="web-storybook"]`, `.storybook-shell`,
   `.storybook-hud[data-region="status"]`, `.story-progress-rail`,
   `[data-region="visual"|"body"|"choices"|"history"]`, `.storybook-dock`,
   `button.choice-row[data-action-id]`, `.choice-bullet`,
   `[data-player-screen="start"]`, `[data-player-action="new-game"]`,
   `.storybook-runtime-warning`, `.storybook-fatal`.
4. 새 npm 의존성 금지. localStorage 키 불변. 사용자-가시 문구는 이 문서에
   명시된 것만 변경.
5. 테스트는 각 WP에 "수정 허용"으로 명시된 것만 수정. 명시 없는 테스트가
   깨지면 코드가 잘못된 것이다.
6. `web/src/data/generated/**`, fixtures, `.yaml` 수정 금지.
7. `web/package-lock.json`은 이 머신의 npm이 `libc` 필드를 벗겨내는 노이즈를
   만든다 — **커밋에 포함하지 말고 `git checkout -- web/package-lock.json`으로
   되돌린다** (의도적으로 의존성을 바꾼 경우 제외; 이번 작업엔 없음).

## 0.1 검증 명령 (모든 WP 후)

```bash
cd ~/work/tui-adv/web && npm test && npx tsc --noEmit && npm run build && cd ..
./.venv/bin/python -m pytest tests/ -q     # 97 passed 유지
```

WP-2(CSS) 후에는 추가로 시각 QA:

```bash
cd web && npm run build:wasm && npx vite preview --host 127.0.0.1 --port 4173 &
npm run qa:storybook:visual -- --base-url http://127.0.0.1:4173/ \
  --out-dir /tmp/gemini-storybook-qa --require-wasm
```

커밋 컨벤션: WP당 1커밋 —
`fix(web-ui): ... [CU-1]` / `refactor(web-css): ... [CU-2]` /
`refactor(web-ui): ... [CU-3]` / `feat(web-ui): ... [CU-4]`.

---

## WP CU-1: 각주 스트립 "N쪽 · M쪽" 이중 표기 수정

**현상**: [web/src/ui/storybook/render.ts](web/src/ui/storybook/render.ts)의
`renderHud`가 다음을 렌더한다:

```ts
<p class="hud-document" aria-label="현재 기록">${escapeHtml(documentLabel(page))} · ${page.status_summary.turn}쪽</p>
```

wuxia 팩에서 `page.chapter_label`은 이미 `천기록 1쪽` 형태이고
`documentLabel`이 이를 그대로 반환하므로 화면에 **"천기록 1쪽 · 0쪽"**처럼
쪽이 두 번 찍힌다 (chapter = turn+1이라 숫자도 어긋나 보인다).

**수정**:
1. 표시 텍스트에서 ` · ${page.status_summary.turn}쪽` 접미를 제거하고
   `documentLabel(page)`만 표시한다.
2. turn 정보는 잃지 않는다 — `aria-label`을
   `현재 기록 ${documentLabel(page)} · ${turn}턴`으로, 그리고 `title` 속성에
   `${turn}턴`을 넣는다 (표기 단위는 "쪽"이 아니라 **"턴"** — 쪽은 chapter_label의
   몫이다).
3. `documentLabel` 함수 자체는 수정하지 않는다.

**수정 허용 테스트**: `web/src/ui/storybook/render.test.ts`에서 hud-document
관련 assert가 있으면 갱신. 새 assert 1개 추가: 렌더 결과에 `쪽 · `가 포함되지
않을 것 (`expect(html).not.toContain('쪽 · ')`).

## WP CU-2: storybook.css 죽은 구세대 스타일 제거

**현상**: [web/src/styles/storybook.css](web/src/styles/storybook.css)는
"구 스타일(상단 HUD/픽셀 dock 시절) + 파일 하단 `/* Rev 2 ... */` 오버라이드"
구조라서, DOM이 더 이상 생성하지 않는 클래스 규칙과 즉시 덮어써지는 규칙이
대량으로 남아 있다 (~1,890줄).

**방법 (기계적으로, 이 순서대로)**:
1. 렌더러가 실제로 emit하는 클래스 전수 목록을 뽑는다:
   ```bash
   grep -rhoE 'class="[^"]+"' web/src/ui web/src/main.ts | tr ' ' '\n' | sort -u
   ```
   (템플릿 리터럴 내 동적 클래스 — `ink-scene--${kind}`, `story-flow--*`,
   `data-transition-*` 계열, `storybook-transition-*`(transitionController가
   부여), `storybook-confirm`(main.ts), `dock-new-mark`, `sr-only` — 도 수동으로
   목록에 추가한다. transitionController.ts와 printerFlow.ts도 grep할 것.)
2. CSS의 각 셀렉터를 이 목록과 대조해 **어떤 emit 클래스와도 매칭되지 않는
   규칙을 삭제**한다. 확실한 삭제 대상(사전 확인 완료):
   `.hud-portrait`, `.hud-portrait-noise`, `.hud-portrait-badge`,
   `.hud-identity`, `.hud-nameplate`, `.hud-subtitle`, `.hud-stat-grid`,
   `.hud-menu`, `.hud-menu-panel`, `.hud-menu-rule`, `.hud-settings`,
   `.hud-warnings`, `.dock-item`, `.dock-spacer`, `.eyebrow`,
   `.pixel-illustration`, `.printer-card`, `.message-card`, `.corridor-card`,
   `.placeholder-card`, `.wuxia-card`, `.wuxia-sky`, `.wuxia-cliff`,
   `.wuxia-gate`, `.wuxia-traveler`, `.combat-card`, `.combat-versus`,
   `.combat-side`, `.combat-arena`, `.combat-hero`, `.combat-enemy`,
   `.combat-impact`, `.combat-hint`, `.start-card`, `.start-hero-image`,
   `.storybook-glyphfx mark` 제외한 것 중 미사용분이 있으면 함께.
   삭제 전 각 클래스를 `grep -rn "<클래스명>" web/src --include="*.ts"`로
   재확인한다 — **하나라도 ts에서 발견되면 삭제하지 않는다.**
3. 같은 셀렉터가 "구 정의 → Rev 2 오버라이드"로 두 번 정의된 것
   (`.storybook-hud`, `.story-progress-rail`, `.rail-*`, `.storybook-dock`,
   `.hud-document`, `.hud-slot*`, `.choice-row` 등)은 **두 정의를 하나로
   병합**한다: 최종 계산값(오버라이드가 이긴 값)을 본문 정의로 승격하고
   하단 오버라이드 블록을 삭제한다. `@media (max-width: 560px)` 블록 안의
   죽은 클래스 규칙도 같이 정리한다.
4. `/* Rev 2 ... */` 주석 섹션 마커는 병합 후 제거한다.

**완료 판정**: 시각 결과가 정리 전과 동일해야 한다.
- `npm test`(render.test 무수정 통과) + 0.1의 시각 QA 전 뷰포트 통과.
- 수동: 시작 → 새 모험 → 선택 1회 → 이동 1회 → 드로어 개폐를 390x844에서
  육안 확인.
- 목표 규모: 파일이 대략 1,000줄 이하로 줄어야 정상이다. 애매한 규칙은
  지우지 말고 보고서에 "미정리 잔여" 목록으로 남긴다.

**수정 허용 테스트**: 없음 (CSS만. 테스트가 깨지면 잘못 지운 것).

## WP CU-3: renderInkVisual 미세 정리

[web/src/ui/storybook/ink/renderInkVisual.ts](web/src/ui/storybook/ink/renderInkVisual.ts):

1. `sceneForVisual`이 2회 호출된다 (8~11행). 다음 형태로 1회로:
   ```ts
   const resolved = sceneForVisual(visual.id, mode);
   const known = resolved !== undefined;
   const spec = resolved ?? genericScene(mode);
   ```
2. `InkSceneSpec.horizon` 필드는 어떤 렌더 경로에서도 사용되지 않는 죽은
   필드다. [inkSpec.ts](web/src/ui/storybook/ink/inkSpec.ts)의 타입에서
   제거하고, [inkScenes.ts](web/src/ui/storybook/ink/inkScenes.ts)와
   `renderInkVisual.ts`의 `genericScene`에서 모든 `horizon:` 지정을 삭제한다.
   (장면 그림 출력은 1픽셀도 변하면 안 된다 — horizon은 원래 무시되던 값이므로
   삭제해도 SVG 출력이 동일해야 한다. 확인 방법: 수정 전후로
   `npm test -- --run src/ui/storybook/render.test.ts` 및 브라우저에서 같은
   장면의 SVG innerHTML이 동일한지 육안/문자열 비교.)

**수정 허용 테스트**: 없음 (출력 불변 리팩터링).

## WP CU-4: 업적·아이템 한국어 라벨 전수 등록

**현상**: [web/src/ui/storybook/labels.ts](web/src/ui/storybook/labels.ts)에
업적 2개, 아이템 4개만 등록되어 있어 나머지는 "미번역" 각주 폴백으로 노출된다.

**절차**:
1. 번들에서 id 전수 목록을 추출한다:
   ```bash
   cd ~/work/tui-adv
   python3 - <<'EOF'
   import json
   b = json.load(open('web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json', encoding='utf-8'))
   def walk(o, key, out):
       if isinstance(o, dict):
           for k, v in o.items():
               if k == key:
                   if isinstance(v, list): out.update(x for x in v if isinstance(x, str))
                   elif isinstance(v, str): out.add(v)
               walk(v, key, out)
       elif isinstance(o, list):
           for v in o: walk(v, key, out)
   for key in ('unlock_achievements', 'achievements', 'achievement_id', 'gain_items', 'items', 'item_id'):
       s = set(); walk(b, key, s)
       if s: print(key, '=>', sorted(s))
   EOF
   ```
   위 키 후보로 안 나오면 번들 JSON 구조를 직접 열어 achievement/item이 어느
   필드에 있는지 확인하고 추출 방식을 조정한다 (스키마 참고:
   `crates/escape-core/src/content.rs`, `docs/dev/Data_Schema.md`).
2. **라벨 정본 출처** (이 순서로 찾고, 임의 창작 금지):
   1. `docs/content/encounter_db/wuxia_jianghu_pack.md` (인카운터별 보상/업적 서술)
   2. `docs/content/Item_List.md`, `docs/content/Ending_List.md`
   3. `src/tui_adv/data/` 및 번들 JSON 안의 한국어 명칭 필드
   문서에서 명칭을 찾을 수 없는 id만 인카운터 본문 맥락에 맞는 자연스러운
   한국어 라벨을 새로 짓되, 그 목록을 보고서에 "신규 명명" 절로 남긴다.
   판단이 서지 않는 id는 라벨 맵에 넣지 말고(=미번역 유지) 보고서에 기록한다.
3. `labels.ts`의 두 Record에 전수 등록한다. 기존 6개 라벨은 바꾸지 않는다.
4. **회귀 방지 테스트 추가** — 새 파일 `web/src/ui/storybook/labels.test.ts`:
   번들 JSON을 import해서 1번과 같은 방식으로 업적/아이템 id를 수집하고,
   모든 id에 대해 `hasAchievementLabel(id)` / `hasInventoryItemLabel(id)`가
   true인지 assert한다. (2번에서 의도적으로 미번역으로 남긴 id가 있으면
   테스트 내 명시적 allowlist 상수로 제외하고 주석으로 사유를 남긴다.)

**수정 허용 테스트**: `labels.test.ts` 신규 추가만. 기존 테스트 무수정.

---

## 마무리

1. `git push -u origin gemini/ui-cleanup-labels` 후 `main` 대상 PR 생성
   (gh는 WSL에 설치되어 있음; PR 본문에 WP별 요약과 검증 결과 명시).
2. 결과 보고를 `fable_ui_cleanup_report.md`(repo root)에 작성:
   완료/스킵 WP, CU-2 미정리 잔여 목록, CU-4 신규 명명/미번역 잔여 목록,
   실행한 검증 명령과 결과.

## 명시적 범위 밖

- 시각 디자인 변경 (색/레이아웃/삽화 구도 조정 금지 — CU-2는 삭제·병합만)
- ink 장면 추가/수정, 전환 연출 변경
- main.ts 구조 변경, 의존성/tsconfig 변경
- 배포 설정 (다음 사이클에서 별도 진행)
