# Fable Gameloop 3 — Step 2 구현 보고서

작성일: 2026-07-16
기준 문서: `fable_gameloop3_step1_2607161330.md`

## 결과

Gameloop 3의 코어 규칙, Web Storybook 표시 계층, 무협 storypack preview 콘텐츠를 구현했다. 기존 선택지 ID와 이벤트 경로는 유지했으며, 새 필드는 optional/default 규칙으로 추가했다.

## 워크패키지별 반영

- D1: 진행도·수련 포인트·기연·아이템/칭호 상세 계약을 `docs/design/Progression_and_Title_Model.md`에 기록하고 개발 계획/인덱스를 갱신했다.
- S1: Rust `ScenePage`에 `inventory_details`, `CharacterSummary.title_description`을 추가했다.
- S2: leveling thresholds, 사용하지 않은 stat point, `train:{ability}` 액션, 능력치 상한 5, 수련 로그와 `stat_points` 요약을 코어가 계산하도록 했다. 수련은 턴을 진행하지 않는다.
- S3: `InsightDef`와 `add_insights`를 추가하고, 코어 판정 합계에 기연 보너스를 반영했다. 주사위와 판정 해시는 변경하지 않으며 기연은 중복 획득되지 않는다.
- W1: 결과 beat 자동 진행 타이머를 제거하고 탭/키 입력만으로 진행하게 했다. HUD를 체력/정신력 숫자 잉크 게이지로 바꾸고 변화 시 pulse를 추가했다.
- W2–W4: 수련 포인트/버튼, 칭호 설명 disclosure, 기연 drawer, 아이템 상세 disclosure·결정적 픽셀 아이콘·사용/비활성 버튼을 Web Storybook에 추가했다. ordered story stream에도 `data-region="body"`를 유지한다.
- C1: 무협 preview에 `[30, 75, 120]` leveling thresholds와 3개 기연을 추가하고 기존 결과 3곳에 기연 획득을 연결했다. office 기본 데이터는 빈 `insights` 배열로 호환성을 유지한다. 생성 JSON/Rust fixture를 갱신했다.
- D2: 본 보고서.

## 변경 원칙

- 규칙·판정·보상·상태 변경은 Rust core가 소유한다.
- 기존 action prefix와 route graph는 변경하지 않았다. 신규 prefix는 `train:` 하나다.
- 새 JSON 필드는 누락 시 기존 저장/콘텐츠가 계속 읽히도록 optional/default 처리했다.
- `prefers-reduced-motion`에서 신규 pulse/전환을 억제한다.

## 검증

- `cargo test --workspace` — 전체 통과
- `.venv/bin/pytest -q tests/test_web_data_export.py` — 12 passed
- `python3 scripts/export_web_data.py --check` 및 무협 preview `--check` — 최신
- `cd web && npx vitest run` — 69 passed
- `cd web && npx tsc --noEmit` — 통과
- `cd web && npm run build` — 통과
- `wasm-pack build` — 통과
- `npm run qa:storybook:visual -- --base-url http://127.0.0.1:5173/ --out-dir /tmp/tui-adv-gameloop3-qa --require-wasm` — 390/414/768/1024/1440 뷰포트 통과
- 자동 브라우저 흐름에서 첫 선택지 클릭과 숫자키 선택 후 페이지 변경을 확인했다.

QA 결과 원본: `/tmp/tui-adv-gameloop3-qa/visual-qa-report.json`

## 커밋

- `82ea5ba` S1 코어 상세 계약
- `93d0329` D1 설계 문서
- `10d185c` S2 leveling/training
- `32de750` S3 insight 판정 보너스
- `2709246` W1 action beat/HUD 게이지
- `1605c78` W2–W4 Storybook surfaces
- `1d91db2` C1 preview 콘텐츠/exporter/fixture
- `855431d` ordered body region 및 runtime 테스트 보정
