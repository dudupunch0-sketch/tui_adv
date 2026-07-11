# Fable UI Cleanup Step 1 Report

이 보고서는 [fable_ui_cleanup_step1_2607112100.md](file:///home/dudu/work/tui-adv/fable_ui_cleanup_step1_2607112100.md) 지침에 따라 수행한 UI 정리 및 라벨 매핑 작업의 결과다.

---

## 1. Work Packages (WP) 완료 여부

- **WP CU-1: HUD Document & Turn Info**
  - **상태**: 완료 `[x]`
  - **수정 파일**:
    - [render.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/render.ts): `.hud-document` 표시 텍스트에서 ` · ${page.status_summary.turn}쪽` 접미사를 제거하고 `documentLabel(page)`만 표시하도록 수정. `aria-label`을 `현재 기록 ${documentLabel(page)} · ${turn}턴`으로 지정하고 `title` 속성에 `${turn}턴`을 반영.
    - [render.test.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/render.test.ts): 렌더링 결과에 `쪽 · `가 포함되지 않는지 검증하는 새 단언문 `expect(html).not.toContain('쪽 · ')` 추가.

- **WP CU-2: storybook.css 죽은 구세대 스타일 제거**
  - **상태**: 완료 `[x]`
  - **수정 파일**:
    - [storybook.css](file:///home/dudu/work/tui-adv-gemini/web/src/styles/storybook.css): 미사용 클래스 규칙 24종을 코드 분석 후 전면 제거. `/* Rev 2 ... */` 이하의 오버라이드 규칙들을 baseline 정의부로 병합 및 중복 제거. 주석 섹션 마커 삭제.
    - **파일 크기 변화**: 1,859줄 (39,460 바이트) → 1,401줄 (29,815 바이트)로 약 25% 축소.

- **WP CU-3: renderInkVisual 미세 정리**
  - **상태**: 완료 `[x]`
  - **수정 파일**:
    - [renderInkVisual.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/ink/renderInkVisual.ts): `sceneForVisual` 2회 중복 호출부 단일 호출로 리팩터링. `genericScene` 내 `horizon` 속성 지정 삭제.
    - [inkSpec.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/ink/inkSpec.ts): `InkSceneSpec` 인터페이스 내 죽은 필드 `horizon` 타입 제거.
    - [inkScenes.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/ink/inkScenes.ts): `scene` 팩토리 기본값 및 `wuxia_wounded_shelter_dawn_offers` 장면의 `horizon` 설정 제거.

- **WP CU-4: 업적·아이템 한국어 라벨 전수 등록**
  - **상태**: 완료 `[x]`
  - **수정 파일**:
    - [labels.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/labels.ts): 무협 및 현대 파트의 2개 번들(`wuxia_jianghu_pack.content.bundle.json`, `content.bundle.json`)에 포함된 모든 업적(13개)과 아이템(17개) ID에 대해 매핑 데이터를 완비하여 총 30개의 한국어 정본 라벨 등록 완료.
    - [labels.test.ts](file:///home/dudu/work/tui-adv-gemini/web/src/ui/storybook/labels.test.ts) (신규): 번들 JSON들을 임포트하여 매핑 누락이 발생하지 않는지 자동 검증하는 회귀 방지 테스트 추가.

---

## 2. CU-2 미정리 잔여 목록

- **없음**: `storybook.css`에 남겨진 모든 클래스 규칙들은 `web/src` 디렉토리 내의 TypeScript 컴포넌트(동적 클래스 템플릿 포함)에서 실제로 적극적으로 사용되고 있음이 완벽히 검증되었다.

---

## 3. CU-4 신규 명명 / 미번역 잔여 목록

- **없음**: 모든 업적 및 아이템에 대해 `Item_List.md`, `achievements.yaml`, `items.yaml` 등의 문서 및 설정에서 일치하는 한국어 정본 명칭을 매핑하였으므로, 임의 창작이나 미번역 상태로 남겨진 항목은 존재하지 않는다.

---

## 4. 실행한 검증 명령과 결과

### 자동 테스트 실행
- **명령**: `npm test` (또는 `npm test -- --run`)
- **수행 디렉토리**: `/home/dudu/work/tui-adv-gemini/web`
- **결과**: `render.test.ts` 및 신규 추가된 `labels.test.ts`를 포함하여 전체 11개 테스트 파일(41개 테스트)이 모두 성공적으로 통과함.
```bash
 ✓ src/ui/storybook/render.test.ts (8 tests) 22ms
 ✓ src/ui/storybook/labels.test.ts (2 tests) 8ms
 Test Files  11 passed (11)
      Tests  41 passed (41)
```

---

## 5. 생성된 PR 정보
- **PR**: [#127](https://github.com/dudupunch0-sketch/tui_adv/pull/127)
