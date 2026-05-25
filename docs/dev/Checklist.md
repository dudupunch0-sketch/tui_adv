# escape from the office 개발 체크리스트

이 문서는 전체 개발 진행 상황을 추적하기 위한 체크리스트다.
체크박스는 실제 작업 완료 후 갱신한다.

계획 문서 우선순위: 다음 작업 순서와 우선순위는 `docs/dev/Development_Plan.md`가 canonical main plan이다. 이 파일은 완료 여부 추적용이며, 독립적인 다음 계획을 두지 않는다.

## 상태 범례

- `[ ]` 아직 시작하지 않음
- `[x]` 완료
- `[~]` 진행 중으로 표시하고 싶으면 줄 끝에 `진행 중` 메모를 남긴다
- `[!]` 막힘이 있으면 줄 끝에 `BLOCKED:` 사유를 적는다

## Phase 0: 문서 기반 정렬

### 0.1 문서 계층

- [x] `docs/00_Index.md` 생성
- [x] `docs/dev/Development_Plan.md` 생성
- [x] `docs/dev/Checklist.md` 생성
- [x] `docs/01_Game_Overview.md` 생성
- [x] `docs/story/Story.md` 생성
- [x] `docs/story/Reality_Link.md` 생성
- [x] `docs/story/Dream_Ending_Branching.md` 생성
- [x] `docs/story/Real_Escape_Ending_Branching.md` 생성
- [x] `docs/design/Player_State.md` 생성
- [x] `docs/design/Game_Loop.md` 생성
- [x] `docs/design/Map.md` 생성
- [x] `docs/design/UI_Rules.md` 생성
- [x] `docs/dev/Architecture.md` 생성
- [x] `docs/dev/Data_Schema.md` 생성
- [x] `docs/dev/TUI_Layout.md` 생성

### 0.2 핵심 결정

- [x] 게임 제목 또는 임시 코드명 결정: `escape from the office`
- [x] 핵심 자원 5개 확정: 체력, 정신력, 배터리, 허기, 갈증
- [x] 허기/갈증 수치 방향 확정: 낮을수록 좋음, 시간이 지날수록 증가
- [x] 1차 수직 슬라이스 범위 확정: 제안 범위 그대로
- [x] 1차 재난 타입 확정: 불명 재난, 사람 실종, 공간/차원 격리, 사내망 간헐 연락
- [x] 1차 엔딩 범위 확정: 실패, 비상계단 탈출, 첫 히든 힌트
- [x] 현실 연결 안전 원칙 문서화
- [x] 실제 위치 정보 비공개 관리 방식 확정: 공개 문서에는 중간 힌트까지만, 최종 위치는 private/local

### 0.2b 2026-05-22 렌더러/런타임 방향 갱신

- [x] 활성 방향 확정: Rust GameCore 공통 + Web Storybook/GlyphFX primary UX + SuperLightTUI terminal renderer/fallback
- [x] `docs/dev/Rust_Core_Dual_Renderer_Architecture.md` 생성
- [x] 최신 계획 문서에 SuperLightTUI terminal renderer가 fallback/debug dump와 다르다는 기준 반영
- [x] `docs/dev/Data_Schema.md`에 renderer-neutral bundle, `ScenePage`, action id, `EffectCue`, WASM JSON boundary 설계 기록
- [x] `docs/design/UI_Rules.md`와 `docs/dev/TUI_Layout.md`에 Web Storybook/SuperLightTUI renderer UX contract 기록
- [x] `escape-core`에 renderer-safe `ScenePage` contract 추가
- [x] Web Storybook renderer skeleton 추가
- [x] `escape-wasm` JSON-string boundary 추가
- [x] `escape-terminal`을 SuperLightTUI renderer로 전환
- [x] Web/terminal 모두 같은 Rust core action id를 표시하는 parity smoke 추가

### 0.2c 2026-05-22 Rust GameCore route parity 확장

- [x] Movement pages를 Rust core + `ScenePage` 기준으로 확장하고 terminal/Web action id contract 유지
- [x] Item use를 Rust core truth로 이전: usable inventory action, resource effect, consume, turn advance
- [x] Ability checks를 Rust core에서 seeded 2d6 + ability로 처리하고 success/failure outcome 적용
- [x] Escape/failure/truth/conquest/hidden reality-link ending `ScenePage` smoke 추가
- [x] Achievement unlock과 `newly_unlocked_achievements`/`achievement_summary` JSON contract 추가
- [x] Low sanity/low battery/high hunger/high thirst pressure cues를 Rust `ScenePage` semantic cue로 노출
- [x] Reality-link public reward metadata만 ending body block으로 노출하고 private-only fields 차단 유지
- [x] Web Storybook runtime을 `escape-wasm` JSON boundary + generated content bundle에 연결하고 Rust state localStorage 저장 추가

### 0.2d 2026-05-22 Web WASM build/preview 표준화

- [x] Web WASM build/preview 절차 표준화
- [x] `web/package.json`에 `wasm:build`, `build:wasm`, `dev:wasm`, `preview:wasm` script 추가
- [x] `build:wasm`이 generated wasm package를 `web/dist/assets/wasm-pkg/`로 복사
- [x] `web/src/core/wasm-pkg/` generated package를 local-only artifact로 ignore
- [x] legacy Python/Textual/TypeScript mirror freeze 범위 결정

### 0.2e 2026-05-22 SuperLightTUI terminal polish

- [x] terminal visual card가 visual_id/layout/alt를 ASCII/Unicode card로 표시
- [x] GlyphFX fallback이 intensity meter, stable terms, fallback text를 보존
- [x] 직접 플레이 입력 안내가 현재 턴 번호 범위와 action id 사용법을 표시

### 0.2f 2026-05-22 Web 배포 표면 결정

- [x] Web/Tauri/Electron 패키징 검토와 Web-only 배포 표면 결정
- [x] `web/package.json`에 `build:player` / `preview:player` alias 추가
- [x] Tauri/Electron은 desktop wrapper 고유 가치가 생길 때까지 deferred로 문서화

### 0.2g 2026-05-22 SuperLightTUI app loop / GlyphFX baseline

- [x] `escape-terminal --app` full-screen SuperLightTUI app loop 추가
- [x] `--app-smoke --tick` headless app-frame smoke 추가
- [x] raw-draw GlyphFX layer가 tick 변화와 stable terms/fallback text를 함께 검증
- [x] inline image는 baseline 밖 optional future로 deferred 결정

### 0.2h 2026-05-23 Web Storybook 모바일 픽셀 board redesign

- [x] Web Storybook 모바일 픽셀 board contract 문서화
- [x] HUD/rail/dock/sentence-choice renderer contract 구현
- [x] reference-size browser visual QA

### 0.2i 2026-05-23 Web Storybook visual regression 자동화

- [x] `web/scripts/storybook-reference-qa.mjs` Playwright viewport runner 추가
- [x] package script로 visual QA command 노출
- [x] reference viewport DOM/layout/interaction contract 자동 검증
- [x] optional `--require-wasm` Rust/WASM-primary resource load smoke 추가
- [x] screenshots/JSON report를 scratch output에만 남기도록 문서화
- [x] visual QA contract/docs tests 추가

### 0.2j 2026-05-24 Web player 공개 배포 계획 문서화

- [x] `idea_box/web_play_like_pokerogue.md`를 읽고 `docs/dev/Web_Player_PokeRogue_Style_Plan.md`로 승격
- [x] Web player URL 즉시 플레이, WASM-required production policy, static deploy QA, start/save UX PR 순서를 문서화

### 0.3 완료 기준

- [x] README 또는 인덱스만 보고 프로젝트 방향을 이해할 수 있다.
- [x] 개발자가 어떤 문서에 무엇을 써야 하는지 알 수 있다.
- [x] 공개 문서와 비공개 현실 위치 정보의 경계가 명확하다.

### 0.4 1차 콘텐츠 초안 문서

- [x] `docs/content/Location_List.md` 생성
- [x] `docs/content/Item_List.md` 생성
- [x] `docs/content/Encounter_List.md` 생성
- [x] `docs/content/Ending_List.md` 생성
- [x] `docs/content/Secret_List.md` 생성
- [x] `docs/content/Horror_Ideas.md` 생성
- [x] `docs/archive/idea_0515.md` 원본 아이디어 노트 보존

## Phase 1: 프로젝트 스캐폴딩과 기술 확정

### 1.1 기술 선택

- [x] Python 버전 결정: Python 3.x
- [x] TUI 라이브러리 결정: Textual
- [x] 데이터 포맷 결정: YAML, 저장 파일은 JSON
- [x] 테스트 프레임워크 결정: pytest
- [x] 패키지/의존성 관리 방식 결정: pyproject.toml 기준

### 1.2 파일 구조

- [x] `pyproject.toml` 생성
- [x] `src/tui_adv/__init__.py` 생성
- [x] `src/tui_adv/main.py` 생성
- [x] `src/tui_adv/game/` 패키지 생성
- [x] `src/tui_adv/tui/` 패키지 생성
- [x] `src/tui_adv/data/` 디렉터리 생성
- [x] `tests/` 디렉터리 생성
- [x] `.gitignore` 생성 또는 갱신
- [x] `private/` 또는 `.local` 비밀 파일 ignore 규칙 추가

### 1.3 실행 검증

- [x] 기본 실행 명령 정의: `PYTHONPATH=src python -m tui_adv --new --seed 123`
- [x] 앱이 초기 상태 smoke 출력을 하며 실행된다.
- [x] `pytest`가 실행된다.
- [x] README에 초기 실행 방법을 기록한다.

## Phase 2: 도메인 모델과 상태 시스템

### 2.1 GameState

- [x] `GameState` 모델 생성
- [x] 현재 위치 필드 추가
- [x] 턴 수 또는 시간 필드 추가
- [x] 위험도 필드 추가
- [x] 인벤토리 필드 추가
- [x] 단서 목록 필드 추가
- [x] 플래그 집합 필드 추가
- [x] 최근 로그 필드 추가

### 2.2 PlayerState 또는 Resources

- [x] 체력 필드 추가
- [x] 정신력 필드 추가
- [x] 배터리 필드 추가
- [x] 허기 필드 추가
- [x] 갈증 필드 추가
- [x] 능력치 필드 추가
- [x] 0-100 범위 clamp 구현
- [x] 0-6 능력치 clamp 구현
- [x] 상태 변경 함수 구현
- [x] 턴 경과 함수 구현

### 2.3 임계치 효과

- [x] 체력 0 실패 상태 정의
- [x] 정신력 0 실패 상태 정의
- [x] 허기 한계 실패 또는 지속 피해 정의
- [x] 갈증 한계 실패 또는 지속 피해 정의
- [x] 배터리 0일 때 제한되는 행동 정의
- [x] 정신력 낮음 선택지 왜곡 규칙 초안 구현 또는 문서화
- [x] 갈증 높음 환각 이벤트 규칙 초안 구현 또는 문서화

### 2.4 테스트

- [x] 상태 기본값 테스트
- [x] 상태 clamp 테스트
- [x] 턴 경과 시 허기/갈증 변화 테스트
- [x] 임계치 계산 테스트
- [x] 실패 상태 판정 테스트

## Phase 3: 위치, 이동, 기본 게임 루프

### 3.1 위치 모델

- [x] Location 모델 정의
- [x] 위치 id/name/description 필드 정의
- [x] 인접 위치 목록 정의
- [x] 위험도 보정 또는 태그 필드 정의
- [x] 위치별 가능한 인카운터 태그 정의

### 3.2 1차 위치

- [x] 내 자리
- [x] 개발팀 사무실
- [x] 복도
- [x] 탕비실
- [x] 회의실
- [x] 복합기 구역
- [x] 서버실 앞
- [x] 비상계단

### 3.3 이동과 턴

- [x] 현재 위치에서 가능한 이동 선택지 생성
- [x] 이동 선택 적용
- [x] 이동 시 턴 증가
- [x] 이동 시 허기/갈증 변화
- [x] 이동 로그 기록
- [x] 위험도 변화 규칙 적용

### 3.4 테스트

- [x] 시작 위치 테스트
- [x] 인접 위치 이동 테스트
- [x] 연결되지 않은 위치 이동 불가 테스트
- [x] 이동 시 턴 증가 테스트
- [x] 이동 로그 테스트

## Phase 4: 인카운터와 선택지 엔진

### 4.1 모델

- [x] Encounter 모델 정의
- [x] Choice 모델 정의
- [x] Condition 모델 또는 조건 검사 함수 정의
- [x] Effect 모델 또는 결과 적용 함수 정의
- [x] Outcome 모델 정의
- [x] AbilityCheck 모델 정의
- [x] 현재 상태 기반 인카운터 선택 함수 정의

### 4.2 조건과 비용

- [x] 자원 최소/최대 조건 검사
- [x] 아이템 보유 조건 검사
- [x] 플래그 조건 검사
- [x] 위치 조건 검사
- [x] 재난 타입 조건 검사
- [x] 선택 비용 적용
- [x] 비용 부족 시 선택 불가 처리
- [x] 능력치 기반 선택지 조건 검사

### 4.3 결과 적용

- [x] 체력 변화
- [x] 정신력 변화
- [x] 배터리 변화
- [x] 허기 변화
- [x] 갈증 변화
- [x] 아이템 추가/제거
- [x] 단서 추가
- [x] 플래그 추가/제거
- [x] 위치 이동
- [x] 위험도 변화
- [x] 로그 추가

### 4.4 샘플 인카운터

- [x] 퇴사자의 메신저
- [x] 복합기가 혼자 출력한다
- [x] 탕비실 커피머신

### 4.5 테스트

- [x] 조건 만족 선택지만 표시되는지 테스트
- [x] 비용 적용 테스트
- [x] 아이템/플래그 결과 테스트
- [x] 성공/실패 분기 테스트
- [x] 현재 상태 기반 인카운터 선택 테스트
- [x] 능력치 기반 선택지 노출 테스트
- [x] 인카운터 후 로그 테스트
- [x] CLI 한 턴 인카운터 표시 테스트
- [x] CLI 선택지 실행 테스트
- [x] 선택지 판정 결과 포맷터 테스트

## Phase 5: 데이터 파일 분리

### 5.1 스키마 문서

- [x] `docs/dev/Data_Schema.md` 작성
- [x] Location 스키마 정의
- [x] Item 스키마 정의
- [x] Encounter 스키마 정의
- [x] Choice 스키마 정의
- [x] Ending 스키마 정의
- [x] Secret 힌트 스키마 정의
- [x] Achievement 스키마 정의

### 5.2 데이터 파일

- [x] `src/tui_adv/data/locations.yaml` 생성
- [x] `src/tui_adv/data/items.yaml` 생성
- [x] `src/tui_adv/data/encounters.yaml` 생성
- [x] `src/tui_adv/data/endings.yaml` 생성
- [x] `src/tui_adv/data/achievements.yaml` 생성
- [x] `src/tui_adv/data/secrets.example.yaml` 생성
- [x] 실제 위치용 로컬 비공개 secret 파일 경로 결정: `private/secrets.local.yaml`

### 5.3 데이터 로더

- [x] 데이터 로더 구현: `src/tui_adv/game/content.py`
- [x] 필수 필드 로드 경로 구현
- [x] id 중복 방지: dict 변환과 테스트로 보장
- [x] 참조 무결성 검증: `validate_public_content()`
- [x] 오류 메시지 개선: 위치 연결, 저장 파일, secret 안전 검사

### 5.4 테스트

- [x] 정상 데이터 로드 테스트
- [x] 필수 필드/스키마 오류 테스트
- [x] 잘못된 id 참조 테스트
- [x] 중복 id에 준하는 런타임 dict wire 테스트
- [x] 샘플 데이터 기반 한 턴 실행 테스트

## Phase 6: TUI 수직 슬라이스

### 6.1 레이아웃

- [x] Textual Header/Footer와 단일 스크롤 게임 패널 구현
- [x] 위치 섹션 구현
- [x] LOCAL STATUS 섹션 구현
- [x] 현재 인카운터/선택지 섹션 구현
- [x] 현재 이동/아이템 행동 섹션 구현
- [x] 소지품/단서 요약 섹션 구현
- [x] 하단 최근 로그 섹션 구현
- [x] 저장 파일 목록과 시작 화면 섹션 구현
- [x] 압박 경고 패널 구현
- [x] 분리된 Textual 위젯/그리드 패널 스타일링

### 6.2 입력

- [x] 숫자 키 선택 지원
- [x] 이동 단축키 지원
- [x] 도움말 키 구현
- [x] 소지품/단서 상세 키 구현
- [x] 전체 로그 상세 키 구현
- [x] 저장 키 구현
- [x] 종료 키 구현
- [x] 시작 화면 새 게임/저장 슬롯 선택/삭제 입력 구현

### 6.3 표시 규칙

- [x] 체력/정신력/배터리는 높을수록 좋은 상태 라벨로 표시
- [x] 허기/갈증은 높을수록 위험한 상태 라벨로 표시
- [x] 선택 불가 선택지 이유 표시
- [x] 정신력 낮음 상태의 텍스트 왜곡 표시
- [x] 압박 경고 패널 표시
- [x] 긴 로그 스크롤 또는 최근 N개만 표시 결정: 기본 최근 5개, `l` 상세 로그 패널
- [x] Textual CSS 색상 테마 위젯 연결

### 6.4 스모크 테스트

- [x] TUI snapshot 시작 가능: `--tui-smoke`
- [x] 시작 화면 표시
- [x] 선택지 입력 가능
- [x] 한 턴 진행 가능
- [x] 다중 턴 scripted smoke 가능
- [x] 저장/불러오기/삭제 smoke 가능
- [x] 실제 Textual 화면 수동 QA

## Phase 7: 공개 콘텐츠 작성

### 7.1 위치 16개

- [x] 런타임 위치 YAML 16개 작성
- [x] `docs/content/Location_List.md`를 YAML 기준으로 갱신
- [x] `docs/implementation-map/data/content.js` 위치 목록 갱신

### 7.2 아이템 13개

- [x] 생수
- [x] 커피
- [x] 과자
- [x] 컵라면
- [x] 구급상자
- [x] 보조배터리
- [x] 손전등
- [x] 사원증
- [x] 보안실 우회권한
- [x] 구겨진 출력물
- [x] 퇴사자의 메모
- [x] 지하주차장 키태그
- [x] 임시 방문증

### 7.3 인카운터 20개

- [x] 퇴사자의 메신저
- [x] 복합기가 혼자 출력한다
- [x] 탕비실 커피머신
- [x] 정수기의 이상한 물
- [x] 물품창고 비상 보급함
- [x] 회의실 화이트보드 모서리
- [x] 존재하지 않는 부서의 전체회의
- [x] 지연된 CCTV 화면
- [x] 어긋난 층수의 보안 콘솔
- [x] 비상계단 문틈 표식
- [x] 비상계단 공간 왜곡
- [x] 서버실 문 앞 무전기
- [x] 관리자 콘솔
- [x] 존재하지 않는 층의 엘리베이터
- [x] 옥상의 제한된 외부 신호
- [x] 지하주차장의 시동음
- [x] 지하주차장 차단기
- [x] 무인 로비 안내 키오스크
- [x] 로비 출구 게이트
- [x] 대표실 결재 콘솔

### 7.4 엔딩 13개 + 즉시 실패 2개

- [x] 코드 내 실패 엔딩: 체력 0
- [x] 코드 내 실패 엔딩: 정신력 0
- [x] YAML 실패 엔딩: 비상계단 공간 붕괴
- [x] 탈출 엔딩: 비상계단 퇴근
- [x] 탈출 엔딩: 옥상 외부 신호
- [x] 탈출 엔딩: 지하주차장
- [x] 탈출 엔딩: 로비 회전문
- [x] 히든 현실 연결 엔딩 3개
- [x] 진실 엔딩: 격리 프로토콜
- [x] 정복 엔딩: 사내망 관리자 권한
- [x] 정복 엔딩: 보안 격리 권한 장악
- [x] 정복 엔딩: 사내 방송 장악
- [x] 정복 엔딩: 대표 승인권 장악

### 7.5 검증

- [x] 새 게임에서 모든 핵심 자원을 볼 수 있다.
- [x] 최소 3개 위치를 오갈 수 있다.
- [x] 음식/물 아이템으로 허기/갈증을 조절할 수 있다.
- [x] 배터리를 쓰는 선택지가 있다.
- [x] 정신력에 영향을 주는 선택지가 있다.
- [x] 하나 이상의 엔딩에 도달할 수 있다.
- [x] 탈출/정복/진실/현실 연결 대표 루트 smoke가 있다.

## Phase 8: 저장/불러오기와 랜덤 시드

### 8.1 저장 데이터

- [x] 저장 파일 버전 필드 추가
- [x] 현재 위치 저장
- [x] 턴/시간 저장
- [x] 위험도 저장
- [x] 플레이어 상태 저장
- [x] 아이템 저장
- [x] 단서 저장
- [x] 플래그 저장
- [x] 랜덤 시드 저장
- [x] 본 인카운터와 해금 업적 저장
- [x] 로그 저장

### 8.2 기능

- [x] 저장하기 구현
- [x] 불러오기 구현
- [x] 저장 파일 목록 보기 구현
- [x] 저장 파일 삭제 구현
- [x] 저장 파일 손상 시 오류 처리
- [x] 버전 불일치 처리 방식 결정

### 8.3 테스트

- [x] 저장 파일 생성 테스트
- [x] 로드 후 상태 동일성 테스트
- [x] 랜덤 시드 재현 테스트
- [x] 잘못된 저장 파일 오류 테스트
- [x] 저장 슬롯 목록/선택/삭제 테스트

## Phase 9: 엔딩 루트 확장

### 9.1 탈출 루트

- [x] 비상계단 탈출 루트 강화
- [x] 옥상 구조 루트 추가
- [x] 지하주차장 차량 루트 추가
- [x] 로비 회전문/출구 게이트 루트 추가

### 9.2 정복 루트

- [x] 보안실 접근 조건 설계
- [x] 서버실 권한 획득 설계
- [x] 사내 방송 장악 설계
- [x] 대표실 결재 콘솔 정복 분기 추가
- [x] 생존자 또는 시스템 제압/설득 설계
- [x] 정복 엔딩 작성

### 9.3 진실 루트

- [x] 퇴사자의 로그 체인 설계
- [x] 회의실 반복 이벤트 설계
- [x] 서버 로그 조각 설계
- [x] 재난 원인 문서 설계
- [x] 진실 엔딩 작성

### 9.4 현실 연결 루트

- [x] `Reality_Link.md` 원칙 반영
- [x] 공개 힌트와 비공개 최종 위치 분리
- [x] 첫 번째 현실 메모 루트 구현
- [x] 두 번째 현실 메모 후보 구현
- [x] 세 번째 현실 메모 후보 구현
- [x] 비밀 정보 릴리즈 검사 추가
- [x] 로컬 비공개 secret 템플릿과 안전 점검 문서 추가

### 9.5 현실 탈출 후일담

- [x] 첫 runtime slice 범위 확정: `escape_commute` 단일 엔딩, text-backed `[POST-ESCAPE REPORT]`, 새 schema/kind 없음
- [x] 설계 계약 문서화: `Development_Plan.md`, `Real_Escape_Ending_Branching.md`, `Data_Schema.md`, `Ending_List.md`
- [x] RED 테스트 추가: Python content, Rust `ScenePage`, SuperLightTUI snapshot, Web generated parity
- [x] `escape_commute.text`에 공개-safe 후일담 블록 추가
- [x] Rust/Web generated content bundle 갱신
- [x] targeted GREEN 및 전체 validation matrix 통과

## Phase 10: 밸런싱, QA, 패키징

### 10.1 밸런싱

- [x] 턴당 허기 증가량 조정
- [x] 턴당 갈증 증가량 조정
- [x] 배터리 사용량 조정
- [x] 체력 피해량 조정
- [x] 정신력 피해량 조정
- [x] 음식/물 회복량 조정
- [x] 인카운터 발생률 조정
- [x] 엔딩 도달 난이도 조정

### 10.2 QA

- [x] 새 게임 10회 수동 플레이 기록
- [x] 탈출 엔딩 도달 테스트
- [x] 실패 엔딩 도달 테스트
- [x] 히든 힌트 도달 테스트
- [x] 저장/로드 반복 테스트
- [x] 터미널 크기별 화면 확인
- [x] 키 입력 오류 처리 확인

### 10.3 패키징과 문서

- [x] README 업데이트
- [x] 설치 방법 작성
- [x] 실행 방법 작성
- [x] 조작법 작성
- [x] 게임 컨셉 소개 작성
- [x] 현실 연결 안전 고지 작성
- [x] 릴리즈 전 비밀 정보 검사

## 릴리즈 전 최종 체크

- [x] 공개 저장소에 실제 사무실 최종 위치가 없다.
- [x] 공개 저장소에 개인 이름, 회사 기밀, 고객 정보가 없다.
- [x] `private/` 또는 `.local` 파일이 커밋되지 않았다.
- [x] 모든 테스트가 통과한다.
- [x] README만 보고 실행할 수 있다.
- [x] 최소 하나의 정상 엔딩에 도달할 수 있다.
- [x] 최소 하나의 실패 엔딩에 도달할 수 있다.
- [x] 히든 힌트 루트가 안전한 위치만 안내한다.
