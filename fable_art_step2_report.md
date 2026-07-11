# 이구학지 Web Player 일러스트·밀도·HUD 개선 결과 보고서 (fable_art_step2_report)

## 1. 개요 및 최종 상태
- **브랜치**: `gemini/art-density-hud` (WSL worktree `/home/dudu/work/tui-adv-art`)
- **수행 결과**: 완료 (일러스트 일부는 API 사용량 한도로 인해 다음 사이클 백로그로 위임 및 SVG 폴백 검증 완료)
- **최종 검증**: 
  - `pytest` 97개 테스트 전체 통과 (WP-0 포함)
  - `vitest` 42개 테스트 전체 통과 (Web)
  - Vite Production client build 성공

---

## 2. 워크패키지별 상세 완료 현황

### WP-0: pytest 수리
- **현황**: 완료
- **조치**: SuperLightTUI interactive loop가 제거되면서 `README.md`에서 삭제된 `full-screen SuperLightTUI app loop`에 대한 존재-assert 검증을 삭제하여 pytest를 완전히 통과시켰습니다.

### WP-A1: 아트 매니페스트 + 로더
- **현황**: 완료
- **조치**: `artManifest.ts` 생성 및 `renderInkVisual.ts`에 이미지 로더 로직을 확장했습니다. 이미지가 등록된 ID인 경우 `<img>` 태그를 렌더링하고, 이미지 로드가 완료되면 (`onload`) 기존 SVG 요소들을 숨기고 우하단 인장(seal)만 활성화하여 이미지 위에 오버레이되도록 구현했습니다. 로드 실패 시 (`onerror`) `display: none` 처리 및 SVG가 자연스럽게 보이도록 자동 폴백을 갖추었습니다.

### WP-A2: 삽지 프레임 UI 결합
- **현황**: 완료
- **조치**: `.ink-scene__art`에 얇은 먹 테두리(1px `--line-hard`), 한지 매트(5px 패딩, `--paper-lit`), 아주 연한 세피아 필터 (`filter: saturate(0.92) sepia(0.06)`)를 입혀 한지 UI 톤과 애니메이션 일러스트가 이질감 없이 어우러지도록 CSS를 설계했습니다.

### WP-A3: 일러스트 에셋 생성 및 시작 화면 결합
- **현황**: 부분 완료 (4장 생성, 9장 백로그 위임)
- **생성된 이미지 목록**:
  1. `title_hero.webp` (세로 구도 3:5, 118.5 KB) - 시작 화면의 기존 수묵 SVG를 대체하고, 로드 실패 시 기존 SVG를 폴백으로 사용하도록 `startScreen.ts`를 확장했습니다.
  2. `wuxia_commute_rift.webp` (가로 구도 5:3, 123.9 KB)
  3. `location_jianghu_roadside.webp` (가로 구도 5:3, 126.8 KB)
  4. `location_jianghu_market_street.webp` (가로 구도 5:3, 145.0 KB)
- **미생성 및 폴백 위임 목록 (Gemini flash image API 할당량 초과로 인한 다음 사이클 위임)**:
  - `wuxia_heuksa_bang_first_fight`
  - `wuxia_cheonggi_record_first_fragment`
  - `wuxia_seo_harin_rescue`
  - `wuxia_cheongryu_apprentice_entry`
  - `wuxia_mumyeong_first_confrontation`
  - `wuxia_boss_first_appearance`
  - `wuxia_sado_final_battle`
  - `ending:wuxia_return_modern_commute_scene_resolved`
  - `ending:wuxia_settlement_stay_scene_resolved`
  *참고*: 미생성된 9개 장면에 대해서는 매니페스트에 등록하지 않아 HTTP 404 에러 시도 없이 즉시 정교한 SVG 폴백이 동작하므로 런타임에 아무런 문제가 없습니다.

### WP-A4: 라이선스 기록
- **현황**: 완료
- **조치**: `web/public/assets/art/README.md`를 생성하여, 모든 이미지가 Gemini를 통해 자체 생성된 순수 자산이며 타사 IP를 침해하지 않는다는 선언을 명시했습니다.

### Phase B: 한 화면 밀도 개선
- **현황**: 완료
- **조치**:
  - `figure[data-region="visual"]` 높이 상한을 `36dvh`로 제한하고, `preserveAspectRatio="xMidYMid slice"` 및 `object-fit: cover`를 사용하여 비율을 유지한 채 중앙 크롭되도록 만들었습니다.
  - 결과 로그(`.story-result-log`)의 줄간격과 마진을 대폭 축소하여 컴팩트하게 압축했습니다.
  - GlyphFX stable-terms 칩을 본문 아래에서 `figcaption` 옆의 인라인 flex 한 줄 배치로 변경하여 레이아웃 영역을 절약했습니다.
  - 모바일(≤560px) 해상도에서 본문 폰트 크기 `1rem/1.7`, 문단 간격 `0.7em`, 선택지 최소 높이 `48px` 패딩 축소를 적용해 스크롤을 대폭 줄였습니다.

### Phase C: 하단 status 각주 스트립 폴리시 및 겹침 버그 수정
- **현황**: 완료
- **조치**:
  - 390px 해상도에서 몸/마음 점과 겹치던 위험도 rail(`.story-progress-rail`)을 status 스트립의 상단 보더 라인 자체(2px absolute red band)로 절대 배치하여 겹침 버그를 해결했습니다.
  - 스트립 내부를 3열 정돈하여 좌측 `몸 ●●●●● 마음 ●●●○○` (한 줄 라벨 포함), 중앙 `기록 N쪽` (또는 천기록 N쪽), 우측 상세 버튼으로 정리했습니다.
  - 상세 버튼 및 드로어 메뉴 버튼을 인장풍 사각 버튼(한자 `詳` overlay 및 `--paper` 배경, `--line-hard` 테두리)으로 고급스럽게 재스킨했습니다.
  - 몸/마음이 위험 수치(`warning`/`critical` band)일 때 점 행 하단에 가는 붉은 밑줄을 추가하여 이중 부호화 가독성을 확보했습니다.
  - 드로어 내 각 섹션에 가는 구분선(괘선)을 그어 가독성을 높였습니다.

### Phase D: 문서 동기화
- **현황**: 완료
- **조치**: `docs/design/Mobile_Ink_Storybook_UI.md`에 일러스트 에셋 및 밀도 계약 섹션을 추가 작성하고 pytest 문서 동기화 검증을 완료했습니다.

---

## 3. 검증 로그 요약

### 1) Pytest 전체 검증 로그
```text
tests/test_cli.py .                                                      [  1%]
tests/test_cloud_server_helper.py ..                                     [  3%]
tests/test_docs_contract.py ............................................ [ 48%]
................                                                         [ 64%]
tests/test_web_audio_engine_contract.py .                                [ 65%]
tests/test_web_data_export.py ...........                                [ 77%]
tests/test_web_packaging_decision.py ..                                  [ 79%]
tests/test_web_player_deployment_contract.py ......                      [ 85%]
tests/test_web_transition_controller_contract.py ..                      [ 87%]
tests/test_web_visual_qa_contract.py ....                                [ 91%]
tests/test_web_wasm_build_standardization.py ........                    [100%]

============================== 97 passed in 4.37s ==============================
```

### 2) Web Test(vitest) 및 Build 검증 로그
```text
 RUN  v4.1.7 /home/dudu/work/tui-adv-art/web

 Test Files  11 passed (11)
      Tests  42 passed (42)
   Start at  22:12:49
   Duration  1.23s (transform 1.84s, setup 0ms, import 2.53s, tests 212ms, environment 2ms)

vite v8.0.14 building client environment for production...
dist/index.html                                                 0.42 kB │ gzip:  0.32 kB
dist/assets/index-CGytBHR1.css                                 23.55 kB │ gzip:  5.84 kB
dist/assets/index-BbO4OjT6.js                                 246.60 kB │ gzip: 64.69 kB │ map: 137.99 kB
✓ built in 304ms
```
