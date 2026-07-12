# Game Loop Expansion, Slice 2 — Execution Report

- **Date**: 2026-07-12
- **Author**: Codex / Antigravity AI
- **Repository Branch**: `feat/gameloop-slice2`

---

## 1. Work Package Status

모든 Work Package (WP)가 누락이나 스킵 없이 순서대로 완수되었습니다:

- [x] **WP-D1: design doc first** — 설계 문서 반영 완료, `test_docs_contract.py` 검증 통과.
- [x] **WP-S1: content_labels (§2.1 engine)** — Rust Core 내 dynamic label lookup 및 `content_labels` 속성 추가 완료.
- [x] **WP-S2: check resolution (§2.3 engine)** — `last_check` 구조체 정의, FNV-1a 32-bit 기반 2d6 주사위 굴림 함수 연동 및 SaveEnvelope 하위 호환 로딩 처리 완료.
- [x] **WP-S3: collapse gate (§2.4 engine + validation)** — 플레이어 체력 0 이하 도달 시 번들 메타 `runtime.collapse`를 참고한 붕괴 게이트 우회 메커니즘 엔진 반영 및 validation 로직 구현 완료.
- [x] **WP-W1: TS mirrors + label preference** — 웹 플레이어 `types.ts` 인터페이스 연동, labels lookup 우선순위 개선 완료.
- [x] **WP-W2: check resolution banner (§2.3 renderer)** — Storybook render 단에 판정 결과 배너 마크업(주사위 글리프 매핑, 계산식 텍스트) 통합 완료.
- [x] **WP-W3: collapse phase hook (§2.4 renderer)** — 붕괴 게이트 visual kind 노출 시 combat phase(적색 UI 테마) 강제 전환 훅 연동 완료.
- [x] **WP-C1: wuxia check-density pass (§2.2)** — 무협 팩 첫 전투(`wuxia_heuksa_bang_first_fight`)의 큰길 도망 선택지에 dexterity 판정(난이도 7) 적용.
- [x] **WP-C2: collapse content (§2.4 content)** — 무협 팩 내 신규 인카운터 `wuxia_collapse_gate` 추가 및 `wuxia_death_rest` 데스 엔딩 연동 완료.
- [x] **WP-D2: closeout** — 전체 검증 완료 및 이 실행 레포트(`fable_gameloop2_step2_report.md`) 작성.

---

## 2. Deviations and Resolutions (주요 해결과제 및 편차)

### A. 붕괴 게이트의 조건 누락으로 인한 무한 루프 발생 문제 (WP-C2)
- **현상**: 새로 추가한 `wuxia_collapse_gate` 인카운터의 `conditions` 를 비워둘 경우, 일반 인카운터 조건 검색(`encounter_is_available`) 단계에서 최우선으로 매칭되어 첫 턴(Turn 0) 시작 시 정상적인 균열 인카운터 대신 붕괴 게이트가 바로 노출되는 버그 발생.
- **해결**: `wuxia_collapse_gate` 의 `conditions` 에 임의로 절대 성립하지 않는 더미 플래그 (`never_set_flag_this_is_only_accessible_via_collapse_override`) 를 요구 사항으로 지정하여 평시 룩업을 차단함. 오버라이드 트리거 발동 시에는 `content.encounter(&collapse.encounter_id)`를 통해 직접 인카운터를 조회하므로 정상 작동함.

### B. 체력 clamping 에 따른 복구 수치 차이 (WP-S3 테스트)
- **현상**: 플레이어 체력이 `-20` 이하로 내려가 기사회생 시 체력 +40 복구를 하였을 때, 예측한 복구 값인 `20`이 아닌 `40`으로 테스트가 실패함.
- **해결**: Rust Core 엔진의 스탯 증가/감소 구현부인 `clamp_resource`가 자원 값을 항상 `0..=100`으로 강제 보정(clamp)하므로, 체력이 `-20`이 되는 즉시 `0`으로 보정된 뒤 복구 (+40) 가 진행되었기 때문에 `40`이 된 것임을 밝히고, 테스트 케이스의 어설션을 수정함.

### C. 로컬 웹 패키지 node_modules native binding 빌드 에러
- **현상**: WSL(리눅스) 환경에서 Windows 스타일 path의 `web` 패키지 테스트를 실행할 때, `@rolldown/binding-linux-x64-gnu` 등 리눅스 native binding 모듈 부재로 vitest가 크래시되는 현상 발생.
- **해결**: WSL 터미널에서 `cd web && npm install` 을 실행하여 리눅스 바인딩을 의존성에 주입하여 로컬 테스트를 정상적으로 돌려 검증함.

---

## 3. Verification Results

모든 빌드 및 테스트 스위트가 완전하게 통과되었습니다.

1. **Cargo Workspace Tests**:
   - `cargo test --workspace` 완료 (61개 content_tui_smoke 테스트, 34개 json_contract 테스트 포함 총 123개 테스트 패스)
2. **Web Tests & Types**:
   - `npm test -- --run` 완료 (12개 파일, 55개 테스트 전체 통과)
   - `npx tsc --noEmit` 완료 (unused import 컴파일러 경고 해결)
   - `npm run build` 완료 (배포 파일 번들링 성공)
