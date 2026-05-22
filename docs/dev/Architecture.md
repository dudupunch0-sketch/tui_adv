# 아키텍처 설계

## 목적

`escape from the office`의 코드는 게임 규칙, 콘텐츠 데이터, 표현 계층을 분리한다.
현재 활성 원칙은 “Core owns truth. Renderer owns mood.”다.

## 현재 활성 방향 (2026-05-22)

이 문서의 과거 Python/Textual, TypeScript mirror core 설명은 현재 구현 이력과 전환기 scaffold를 이해하기 위한 것이다. 앞으로 새 시각/상호작용 작업은 다음 구조를 따른다.

```text
Rust GameCore
  ├─ Web Storybook + GlyphFX renderer
  │   └─ primary player UX
  └─ SuperLightTUI terminal renderer
      └─ terminal-native fallback / horror edition
```

확정 기준:

- `crates/escape-core`가 상태, 콘텐츠, 선택 가능 action, 결과, 엔딩, 업적, save schema, `EffectCue`/`ScenePage`의 truth를 소유한다.
- Web Storybook + GlyphFX가 플레이어용 primary UX 후보다. 이미지/장면 컷, 대화 내역, 읽기 중심 선택지, Canvas/GlyphFX는 이 경로에서 먼저 구현한다.
- Rust terminal 경로는 SuperLightTUI 기반 renderer로 유지한다. terminal은 fallback이지만 단순 `println!` debug dump가 아니라 terminal-native horror edition이어야 한다.
- 기존 Python/Textual과 TypeScript mirror core는 당분간 legacy/parity oracle로 유지한다. 새 게임 규칙은 TypeScript나 renderer에서 늘리지 않는다.
- 세부 기준은 `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`를 따른다.

## 현재 구현/레거시 접근

- 언어: Python 3.x + TypeScript + Rust workspace 전환기
- 현재 Python 엔진: 표준 라이브러리 중심의 순수 Python 모듈
- 현재 브라우저: Vite 기반 fake-TUI shell, TypeScript mirror core, 특수 장면은 Canvas/pretext 계열 실험
- 현재 Rust: `escape-core`와 `escape-terminal` content runner 일부 parity
- 목표 TUI: `escape-terminal`의 SuperLightTUI renderer
- 콘텐츠 데이터: YAML 파일, browser/Rust는 생성 JSON/content bundle 사용
- 저장 데이터: Python은 JSON 파일, 브라우저는 localStorage, Rust core는 save schema를 수렴 목표로 둔다.
- 테스트: pytest + Vitest + Cargo tests

Textual을 유지하는 이유(legacy/parity):

- 이미 저장/시작 화면/도움말/인벤토리/로그 패널 smoke coverage가 있다.
- Rust/Web 전환 중 대표 route와 save/load 동작을 비교하는 oracle로 쓸 수 있다.
- 단, Textual을 장기 primary UX로 더 키우지 않는다.

주의:

- `escape-core`는 SuperLightTUI, Textual, DOM, Canvas, CSS를 import하지 않는다.
- renderer 없이도 테스트와 headless 실행이 가능해야 한다.
- YAML 로더와 데이터 검증은 앱 시작 전에 독립적으로 실행 가능해야 한다.
- 브라우저/러스트 앱은 공개 YAML을 직접 수정하지 않고 `scripts/export_web_data.py`로 생성한 JSON/bundle을 읽는다.
- 브라우저 번들에는 실제 사무실 최종 위치나 local secret이 들어가지 않는다.

## 상위 구조

```text
src/tui_adv/
  __init__.py
  main.py                         # CLI/TUI 진입점

  game/
    __init__.py
    state.py                      # GameState, PlayerState, inventory, flags
    resources.py                  # 자원 변화, 임계치, 실패 판정
    engine.py                     # 턴 진행, 선택 적용, 엔딩 체크
    locations.py                  # Location 모델과 이동 가능성
    encounters.py                 # Encounter/Choice 모델과 선택 처리
    effects.py                    # 선택 결과 적용 로직
    conditions.py                 # 조건 검사 로직
    endings.py                    # 엔딩 조건과 결과
    rng.py                        # 시드 기반 랜덤 래퍼
    log.py                        # 게임 로그 모델

  data/
    __init__.py
    loader.py                     # YAML/JSON 로더
    validate.py                   # 데이터 검증
    locations.yaml
    items.yaml
    encounters.yaml
    endings.yaml
    secrets.example.yaml          # 공개 예시. 실제 위치 없음

  tui/
    __init__.py
    app.py                        # Textual App
    layout.py                     # 레이아웃 구성
    widgets.py                    # 상태/지도/이벤트/로그 위젯
    theme.py                      # 색상, 스타일, 리소스 바 규칙
    input.py                      # 키 입력을 게임 명령으로 변환

  save/
    __init__.py
    manager.py                    # 저장/불러오기
    schema.py                     # 저장 파일 버전과 변환

  cli/
    __init__.py
    headless.py                   # TUI 없이 한 턴씩 실행하는 디버그/테스트 모드

tests/
  game/
  data/
  save/
  tui/

scripts/
  export_web_data.py              # 공개 YAML을 브라우저 JSON으로 export/check

web/
  package.json
  index.html
  src/
    main.ts                       # Vite 브라우저 진입점
    game/                         # TypeScript mirror core: actions/items/achievements/endings/pressure
    ui/                           # fake-TUI HTML renderer와 키 입력
    effects/                      # pretext/Canvas anomaly panel
    security/                     # public secret guard
    data/generated/               # export된 공개 JSON
```

## 컴포넌트 경계

```text
YAML content files
        |
        v
scripts/export_web_data.py
        |
        +--> web/src/data/generated/*.json              # legacy TS mirror/parity
        +--> crates/escape-core/fixtures/content/*.json # Rust content bundle
        |
        v
crates/escape-core
        |
        +--> ScenePage / TurnView / ActionResult / EffectCue
        |
        +--> web Storybook renderer via escape-wasm
        |
        +--> escape-terminal SuperLightTUI renderer

legacy Python/Textual + TypeScript mirror remain temporary parity/oracle surfaces.
```

금지 방향:

- `escape-core`가 SuperLightTUI/crossterm, wasm-bindgen/web-sys, DOM/Canvas/CSS를 import하지 않는다.
- renderer가 action eligibility, outcome, ending, achievement를 재계산하지 않는다.
- `game/`이 `tui/`를 import하지 않는다.
- `game/`이 Textual/Rich 스타일 객체를 만들지 않는다.
- 데이터 파일이 Python/Rust/TypeScript 코드를 실행하지 않는다.
- 현실 최종 위치가 공개 데이터 파일이나 Web asset에 들어가지 않는다.

## 핵심 모델

### PlayerState

책임:

- 체력, 정신력, 배터리, 허기, 갈증 보관
- 0-100 clamp
- 임계치 상태 계산

### GameState

책임:

- 현재 위치
- 턴 수와 시간대
- 위험도
- 플레이어 상태
- 인벤토리
- 단서 목록
- 플래그 집합
- 최근 로그
- 현재 재난 타입
- 랜덤 시드 상태

### Location

책임:

- 위치 id/name/description
- 인접 위치
- 태그
- 위험도 보정
- 위치별 인카운터 후보

### Encounter

책임:

- 발생 조건
- 이벤트 설명
- 선택지 목록
- 재난 타입/위치 태그
- 발생 가중치

### Choice

책임:

- 표시 텍스트
- 필요 조건
- 비용
- 성공/실패/확정 결과
- 선택 후 로그

### Effect

책임:

- 자원 변경
- 위치 변경
- 아이템 추가/제거
- 단서 추가
- 플래그 추가/제거
- 위험도 변경
- 엔딩 트리거

## 엔진 흐름

게임 엔진의 공개 함수는 작고 테스트하기 쉬워야 한다.

권장 함수 경계:

```text
new_game(seed, scenario_id) -> GameState
get_available_actions(state, content) -> list[Action]
apply_action(state, action_id, content, rng) -> GameState
advance_turn(state, content, rng) -> GameState
check_endings(state, content) -> EndingResult | None
```

`apply_action`은 기존 state를 직접 수정하지 않고 새 state를 반환하는 방향을 우선 검토한다.
완전 불변 구조가 과하면 내부 복사는 최소화하되 테스트에서 side effect가 명확해야 한다.

## 데이터 로딩 흐름

앱 시작 시:

1. 기본 데이터 파일 로드
2. 데이터 스키마 검증
3. id 중복 검사
4. 참조 무결성 검사
5. private/local secret 파일이 있으면 선택적으로 로드
6. 공개 데이터와 private 데이터의 충돌 검사
7. 새 게임 생성

브라우저/Rust 앱은 앱 시작 전 또는 CI에서 다음 흐름을 사용한다.

1. `python scripts/export_web_data.py --write --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json`
2. `web/src/data/generated/*.json`, Web content bundle, Rust content bundle 갱신
3. `python scripts/export_web_data.py --check --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json`로 stale 여부와 public secret private-only 필드 누출 확인
4. Legacy Vite fake-TUI는 생성 JSON을 import해 TypeScript mirror core에서 사용한다.
5. Web Storybook은 현재 generated content bundle과 `escape-wasm` boundary를 연결할 준비 상태이며, 실제 wasm 호출 연결은 다음 slice다.
6. `cd web && npm test`가 현재 TypeScript parity, 소모품, 업적, 능력치 판정, 압박 UI, secret guard를 검증한다.
7. `cargo test --workspace`가 Rust content/core/terminal contract를 검증한다.

private/local secret 파일이 없어도 게임은 실행되어야 한다.
그 경우 현실 연결 루트는 중간 힌트까지만 표시한다.

## 현실 연결 보안 경계

공개 코드/데이터가 가질 수 있는 정보:

- secret id
- 게임 내 힌트 단계
- 가짜 예시 위치
- unlock flags
- 안전 원칙

공개 코드/데이터가 가지면 안 되는 정보:

- 실제 최종 위치
- 실제 회사명/층/좌석/보안구역
- 실제 사람 이름
- 실제 보상 배치 사진

`.gitignore`는 `private/`, `*.local.*`, `src/tui_adv/data/secrets.local.*`를 차단한다.
릴리즈 전에는 별도 secret scan을 수행한다.

## 테스트 전략 연결

우선 테스트 대상:

1. 상태 변화와 clamp
2. 턴 경과
3. 조건 검사
4. 효과 적용
5. 인카운터 선택
6. 엔딩 판정
7. 데이터 로더와 참조 무결성
8. 저장/불러오기 동일성

브라우저 쪽 Vitest는 현재 `web/src/game/parity.test.ts`에서 대표 terminal 루트(탈출·정복·진실·히든), 소모품 사용, 업적 해금, 고갈증/저정신력 압박, 능력치 판정 분기를 mirror core로 검증하고, `web/src/ui/render.test.ts`에서 legacy fake-TUI 패널을 검증한다. 새 Web Storybook 작업은 `web/src/ui/storybook/*` 테스트로 분리하고, 새 게임 규칙은 TypeScript mirror가 아니라 Rust GameCore/WASM contract에서 검증한다.

## 1차 구현 비범위

1차 수직 슬라이스에서는 다음을 만들지 않는다.

- 실제 회사 지도를 정밀하게 반영한 맵
- 여러 재난 타입의 완전한 룰셋
- 복잡한 전투 시스템
- 네트워크 기능
- 실제 인터넷 접속 기능
- 공개 데이터에 실제 현실 위치 포함

## 현재 이후 구현 순서

완료된 기반 작업:

1. `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`와 README/계획 문서가 Web primary + SuperLightTUI terminal 방향을 명확히 고정했다.
2. `escape-core`에 renderer-safe `ScenePage` contract를 추가했다.
3. YAML/content bundle에 optional presentation metadata(`visual_id`, speaker/effect hints)를 추가했다.
4. Web Storybook renderer skeleton과 visual/history/choice region을 추가했다.
5. `escape-wasm` JSON-string boundary와 Web용 generated content bundle을 추가했다.
6. `escape-terminal`의 content TUI snapshot/play loop를 SuperLightTUI renderer로 전환했다.
7. Web/terminal action id parity smoke를 추가했다.

남은 구현 순서:

1. route parity를 Rust core 기준으로 확장하고, legacy Python/Textual/TS mirror는 검증 oracle로 점진 축소한다.
