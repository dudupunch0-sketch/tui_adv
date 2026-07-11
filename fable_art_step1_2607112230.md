# 이구학지 Web Player 일러스트·밀도·HUD 개선 플랜 (Plan by Fable, implementation by Gemini)

Date: 2026-07-11
Baseline: `main` HEAD `e2ead24` ("docs: add problem_gemini.md and style reference image.png (#129)")
입력 문서: `problem_gemini.md` (3개 문제) + Fable 실화면 검증(390x844 스크린샷)
작업 위치: **WSL clone `~/work/tui-adv`**. 브랜치: `gemini/art-density-hud`.

이 플랜은 `problem_gemini.md`의 세 문제를 해결한다:

1. **일러스트 저품질** → Gemini가 생성한 **애니메이션풍 일러스트 에셋** 도입
   (코드 SVG는 폴백으로 유지). ← 이번 사이클의 핵심.
2. **한 화면 초과 스크롤** → 삽화 높이 상한 + 밀도 축소.
3. **하단 상태 스트립 저품질** → 겹침 버그 수정 + 인장/각주 스타일 폴리시.

워크패키지 순서: **WP-0 → A1 → A2 → A3(이미지 생성) → A4 → B → C → D**.
WP마다 검증 → 커밋 1개. 전부 끝나면 push + `main` 대상 PR + 결과 보고서
`fable_art_step2_report.md` 작성.

---

## 0. 절대 불변

1. `web/` + 이 문서에 명시된 파일 밖은 수정 금지.
2. `ScenePage` JSON 소비 전용. 게임 규칙 재계산 금지. 필드 추가 금지.
3. 계약 셀렉터 유지: `[data-renderer="web-storybook"]`, `.storybook-shell`,
   `.storybook-hud[data-region="status"]`, `.story-progress-rail`,
   `[data-region="visual"|"body"|"choices"|"history"]`, `.storybook-dock`,
   `button.choice-row[data-action-id]`, `.choice-bullet`,
   `[data-player-screen="start"]`, `[data-player-action="new-game"]`.
4. **저작권 — 가장 중요**:
   - `docs/design/image.png`(타사 게임 공식 아트)는 **화풍 무드 참조 전용**이다.
     크롭/변형/트레이싱 포함 어떤 형태로도 에셋·생성 입력 이미지로 쓰지 않는다.
   - 저장소에 커밋하는 이미지는 **Gemini가 이 플랜의 프롬프트로 직접 생성한
     것만** 허용. 타사 IP의 캐릭터·복장·로고·고유 디자인이 생성물에 나타나면
     폐기하고 재생성한다.
   - 생성물에 워터마크/서명/뭉개진 글자가 있으면 폐기·재생성.
5. 새 npm 의존성 금지. localStorage 키 불변.
6. GlyphFX `stable_terms`/`fallback_text` 가독성 유지, reduced-motion 존중,
   색+형태 이중 부호화, 터치 타깃 44px.
7. `web/package-lock.json` 노이즈는 커밋 제외 (`git checkout --`로 원복).
8. 테스트는 각 WP에 "수정 허용"으로 명시된 것만 수정.

## 0.1 검증 명령 (모든 WP 후)

```bash
cd ~/work/tui-adv/web && npm test && npx tsc --noEmit && npm run build && cd ..
./.venv/bin/python -m pytest tests/ -q      # WP-0 이후 전부 통과해야 함
```

시각 WP(A2, B, C) 후 추가:

```bash
cd web && npm run build:wasm && npx vite preview --host 127.0.0.1 --port 4173 &
npm run qa:storybook:visual -- --base-url http://127.0.0.1:4173/ \
  --out-dir /tmp/gemini-art-qa --require-wasm
```

수동 확인 뷰포트: **390x844(1차 기준)**, 414x896, 800x1440.

커밋 컨벤션: `fix(tests): ... [AR-0]` / `feat(web-art): ... [AR-A1]` /
`assets(web-art): ... [AR-A3]` / `feat(web-ui): ... [AR-B]` 등.

---

## WP-0: PR #128이 깨뜨린 pytest 수리

`tests/test_docs_contract.py::test_superlighttui_terminal_polish_is_checklisted_and_documented`가
SuperLightTUI 제거(#128) 이후 실패 중이다. 테스트를 읽고, #128의 제거 의도에
맞게 갱신한다: 제거된 renderer에 대한 존재-assert는 삭제하거나 "제거됨이
문서화되어 있는지" assert로 교체한다. #128이 남긴 문서 서술(제거 사실 기록)과
일치시키는 방향으로 — **문서를 되살리는 방향이 아니라 테스트를 현재 상태에
맞추는 방향**이다. 다른 테스트는 건드리지 않는다.

**수정 허용 테스트**: 해당 테스트 1개만.

---

## Phase A — 애니메이션풍 일러스트 에셋 파이프라인 (문제 3)

### WP-A1: 아트 매니페스트 + 로더

1. 새 파일 `web/src/ui/storybook/art/artManifest.ts`:
   ```ts
   export const ART_BASE_PATH = 'assets/art/';
   /** visual_id → 파일명(확장자 포함). 등록된 장면만 이미지, 나머지는 SVG 폴백. */
   export const artManifest: Record<string, string> = {};
   export function artAssetFor(visualId: string): string | undefined {
     const normalized = visualId.startsWith('ending:') ? visualId.slice('ending:'.length) : visualId;
     const file = artManifest[visualId] ?? artManifest[normalized];
     return file ? `${import.meta.env.BASE_URL}${ART_BASE_PATH}${file}` : undefined;
   }
   ```
   (`BASE_URL` 사용 필수 — 추후 GitHub Pages 하위 경로 배포 대비.)
2. `renderInkVisual.ts` 확장: `artAssetFor(visual.id)`가 있으면 SVG 대신
   ```html
   <img class="ink-scene__art" src="..." alt="" loading="lazy" decoding="async" width="1120" height="672" />
   ```
   를 `<figure>` 안에 렌더 (figure의 `figcaption`=visual.alt, `data-region`,
   `data-visual-id` 계약 그대로; `data-visual-kind`는 `"art"`).
   이미지 로드 실패 시에도 깨진 아이콘이 안 보이게 `onerror`로 img를 숨기고
   SVG 폴백을 함께 렌더해 두거나(뒤에 겹침), CSS `background`로 한지색 유지 —
   구현 방식은 자유, "실패해도 빈칸/깨짐 없이 SVG나 한지가 보인다"가 요구사항.
3. `render.test.ts`에 테스트 추가: 매니페스트에 있는 id는 `<img`와
   `data-visual-kind="art"`, 없는 id는 기존 SVG 경로 유지.

**수정 허용 테스트**: `render.test.ts` 추가만.

### WP-A2: 삽지(貼紙) 프레임 — 애니 일러스트와 한지 UI의 톤 결합

애니메이션풍 일러스트는 채도가 높아 한지 UI에 그대로 붙이면 튄다. CSS로
"기록책에 붙여 둔 그림" 프레임을 만든다:

- `.ink-scene__art`(및 `<figure>`): 얇은 먹 테두리(1px `--line-hard`) + 안쪽
  한지 매트(4~6px padding, `--paper-lit`) + 아주 약한 세피아 톤 보정
  (`filter: saturate(0.92) sepia(0.06)` 수준 — 과하지 않게).
- figure 우하단의 기존 인장(seal)은 이미지 위에도 유지한다 (inkScenes의
  seal 글자 재사용 — 렌더러에서 이미지 모드일 때도 seal만 오버레이).
- figcaption은 그림 아래 각주 스타일 유지.

### WP-A3: 일러스트 생성 (Gemini 이미지 생성 작업)

**공통 스타일 스펙 (모든 생성에 프롬프트 공통부로 사용):**

```text
Korean webtoon/anime style illustration, wuxia (martial arts) world,
cinematic wide composition, painterly cel shading, muted warm palette
that harmonizes with aged hanji paper UI (avoid neon, avoid pure white),
soft rim light, no text, no watermark, no signature, no UI elements.
Aspect ratio 5:3.
```

**주인공 고정 설정 (모든 등장 장면에 포함):**

```text
Main character: Korean office worker in his/her rumpled modern business
attire (white shirt, loosened tie, slacks) with a company ID badge on a
lanyard, transported into an ancient wuxia world. Ordinary build, tired
but determined expression.
```

**규격**: 1120x672(5:3, 기존 viewBox 280x168과 동일 비율), webp 변환,
파일당 **150KB 이하**(초과 시 품질 낮춰 재인코딩), 파일명 `<visual_id>.webp`,
저장 위치 `web/public/assets/art/`.

**절차**:
1. 먼저 **캐릭터 앵커 4장**을 생성해 스타일을 고정한다(주인공, 무명(떠돌이
   검객, 주인공과 거울상), 서하린(청류문 여성 문도), 흑사방 보스). 이 4장은
   커밋하지 않고 이후 장면 생성 시 스타일/외형 일관성 참조로만 쓴다.
2. **1차 슬라이스 13장**을 생성·커밋한다 (자주 노출되는 장면 우선):

   | visual_id | 장면 프롬프트 핵심 (본문을 읽고 구체화할 것) |
   |---|---|
   | `title_hero` (시작 화면 전용) | 협곡 관도 위 출근복 주인공 뒷모습, 안개 낀 무협 산세, 세로 구도 허용(이 장면만 3:5 세로 1120x1867) |
   | `wuxia_commute_rift` | 균열에서 떨어진 직후, 낯선 관도 한복판의 주인공, 아침 안개 |
   | `location:jianghu_roadside` | 대나무 숲 사이 흙길, 멀리 청류문 표식 |
   | `location:jianghu_market_street` | 무협 장터 거리, 노점과 깃발 |
   | `wuxia_heuksa_bang_first_fight` | 장터 입구, 몽둥이 든 흑사방 말단들과 대치하는 주인공 |
   | `wuxia_cheonggi_record_first_fragment` | 등불 아래 서안에서 천기록 조각을 읽는 주인공 |
   | `wuxia_seo_harin_rescue` | 부상자를 부축하는 구조 장면 |
   | `wuxia_cheongryu_apprentice_entry` | 산문 돌계단 앞에서 예를 갖추는 주인공 |
   | `wuxia_mumyeong_first_confrontation` | 안개 낀 길, 검을 든 무명과의 첫 대면 |
   | `wuxia_boss_first_appearance` | 높은 단 위 흑사방 보스의 위압적 실루엣, 검은 깃발 |
   | `wuxia_sado_final_battle` | 최종 결전, 검격이 교차하는 순간 |
   | `ending:wuxia_return_modern_commute_scene_resolved` | 현대 출근길로 돌아온 주인공, 여운 |
   | `ending:wuxia_settlement_stay_scene_resolved` | 강호에 정착한 초옥의 평온한 저녁 |

   각 장면의 구도·소품·인물 배치는 (a) 해당 encounter의 본문 텍스트
   (`web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`)와
   (b) `web/src/ui/storybook/ink/inkScenes.ts`의 구도 스펙(far/mid/figures/seal —
   기존 수묵 구도 테이블이 곧 콘티다)을 읽고 구체화한다.
3. 생성물 검수 체크리스트 (한 장마다): 5:3 비율 / 타사 IP 요소 없음 /
   글자·워터마크 없음 / 주인공 외형 일관성 / 한지 UI 위에서 톤 조화
   (실제 화면에 띄워 확인) / 150KB 이하.
4. `artManifest.ts`에 13개 등록. `startScreen.ts`의 hero SVG를
   `title_hero.webp` + 기존 로고 락업으로 교체 (SVG는 이미지 로드 실패 폴백으로
   유지 가능).
5. 나머지 ~27개 장면은 **이번 사이클 범위 밖** (SVG 폴백이 동작하므로).
   2차 생성은 다음 사이클에서 별도 진행 — 보고서에 미생성 목록을 남긴다.

**수정 허용 테스트**: `startScreen.test.ts` (hero 마크업 변경분).

### WP-A4: 라이선스 기록

`web/public/assets/art/README.md` 신규 작성 (5줄 이내): 이 디렉터리의 모든
이미지는 프로젝트가 AI 생성으로 직접 제작한 자산이며, 외부 게임/작품의 아트를
포함하지 않는다는 선언 + 생성 규격 요약.

---

## Phase B — 한 화면 밀도 (문제 2)

목표: **390x844에서 "본문 제목 + 서사 4~6줄 + 삽화 + 선택지 2개 + 하단
스트립"이 스크롤 없이 또는 한 번의 짧은 스크롤로** 들어온다.

1. 삽화 높이 상한: `figure[data-region="visual"]`에 `max-height: 36dvh` +
   이미지 `object-fit: cover`(5:3 유지, 중앙 크롭). SVG 폴백도 동일 상한.
2. 결과 로그 압축: `.story-result-log`는 최근 1행 + 소지품/업적 행만 표시하고
   그 외는 드로어의 "기록"으로 유도 (현재도 유사 — 행간·여백만 축소).
3. GlyphFX stable-terms 칩(`.storybook-glyphfx`)을 삽화 아래 별도 블록에서
   **figcaption 옆 한 줄 인라인**으로 축소 (mark 태그와 가독성은 유지 —
   계약 위반 금지).
4. 모바일(≤560px) 타이포·간격 재조정: 본문 `1rem/1.7`, 문단 간격 `0.7em`,
   `.storybook-page` 상하 패딩 축소, 선택지 row `min-height 48px` 패딩 축소.
5. 판정: 390x844에서 위 목표 장면(`wuxia_heuksa_bang_first_fight`) 기준
   스크린샷으로 확인하고 보고서에 첨부 경로 기록.

**수정 허용 테스트**: 없음 (CSS/스페이싱. 기존 테스트 무수정 통과).

---

## Phase C — 하단 각주 스트립 폴리시 (문제 1)

실화면에서 확인된 결함 기준:

1. **겹침 버그 수정**: 390px에서 몸/마음 먹점 행과 위험 실(`story-progress-rail`)이
   겹쳐 렌더된다. 위험 실을 점 행과 같은 셀에 두지 말고 **스트립의 상단 보더
   라인 자체**로 옮긴다: 스트립 top border 위에 절대배치된 2px 붉은 실
   (`--progress` 폭, `data-danger-band` 색). rail의 클래스/aria 계약은 유지.
2. 스트립 내부를 3열 정리: 좌 `몸 ●●●●● 마음 ●●●○○`(한 줄, 라벨 포함),
   중앙 `천기록 N쪽`, 우 상세 버튼.
3. **상세 버튼 재스킨**: 흰 폼 버튼 → 인장풍 사각 버튼(`--paper` 바탕,
   `--line-hard` 테두리, 한자 `詳` aria-hidden + "상세" 텍스트 유지, 44px).
4. 드로어 시트: 섹션 사이 가는 괘선, 메뉴 버튼도 3번과 같은 인장풍으로 통일.
5. 몸/마음이 위험(band warning/critical)일 때 해당 점 행 아래 가는 붉은 밑줄
   (색+형태 이중 부호화 유지).

**수정 허용 테스트**: `render.test.ts`에서 스트립 구조를 검사하는 assert가
있으면 새 구조로 갱신 (계약 셀렉터 assert는 유지).

---

## Phase D — 문서 갱신

`docs/design/Mobile_Ink_Storybook_UI.md`에 절 추가:

- "일러스트 에셋": artManifest 규칙, 5:3 규격, 삽지 프레임, SVG 폴백, 미등록
  id는 SVG generic — 그리고 **에셋 라이선스 규칙**(자체 생성물만).
- "밀도 계약": 390x844 한 화면 목표 문장.
- 갱신 후 `./.venv/bin/python -m pytest tests/test_docs_contract.py tests/test_web_visual_qa_contract.py -q` 통과 확인
  (이 문서를 읽는 테스트가 있으므로, 기존 assert 문구는 지우지 말고 절만 추가).

---

## 범위 밖

- 나머지 ~27개 장면 일러스트 (2차 사이클)
- GitHub Pages 배포 정비 (이 사이클 다음에 별도 플랜으로 진행 예정 —
  그래서 A1에서 `BASE_URL`을 미리 쓰게 했다)
- 전투/전환 연출 변경, 콘텐츠 추가, main.ts 구조 변경
- `docs/design/image.png` 삭제 여부 (사용자 결정으로 유지 — 참조 전용)

## 최종 체크리스트

- [ ] WP당 커밋 1개, `gemini/art-density-hud` 브랜치, main 대상 PR.
- [ ] `npm test && tsc && build` green, pytest 전부 통과(WP-0 포함).
- [ ] 시각 QA 전 뷰포트 통과 + 390x844 밀도 목표 스크린샷.
- [ ] 커밋된 이미지 전수: 자체 생성물, ≤150KB, `<visual_id>.webp`, README 존재.
- [ ] `fable_ui_cleanup_report.md`처럼 `fable_art_step2_report.md` 작성
      (완료/스킵, 미생성 장면 목록, 검수 체크 결과, 검증 로그).
