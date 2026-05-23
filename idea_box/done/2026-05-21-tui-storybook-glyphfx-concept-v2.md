---
status: done
created: 2026-05-21
source: user
backlog_order: 001
git_added_at: 2026-05-21T14:50:28+00:00
git_added_commit: 57a381bd
related_docs: docs/design/TUI_Storybook_GlyphFX_Concept.md
used_by:
  - docs/design/TUI_Storybook_GlyphFX_Concept.md
  - docs/dev/Development_Plan.md
  - docs/dev/Rust_Core_Dual_Renderer_Architecture.md
  - docs/dev/Data_Schema.md
  - web/src/ui/storybook/render.ts
  - crates/escape-terminal/src/main.rs
done_at: 2026-05-23
---

# TUI Storybook + GlyphFX Concept v2

## 원문/요지

사용자는 `Escape from the Office`를 실제 터미널 조작 게임이 아니라, 텍스트 중심의 스토리북 화면이 특정 순간 살아 움직이는 TUI풍 인터랙티브 게임으로 재정의했다.

핵심 표현은 다음과 같다.

> 이 게임은 터미널을 사용하는 게임이 아니라, 텍스트로 이루어진 세계가 때때로 살아 움직이는 게임이다.

기본 화면은 모바일 세로형 텍스트 어드벤처처럼 읽기 좋아야 한다. 특정 사건, 이상 현상, 기억 오류, 감시, 금지된 문서, 탈출 단서가 등장하는 순간에만 글자, 문단, 선택지, 그림 속 텍스트가 GlyphFX로 반응한다.

## 핵심

이 아이디어는 프로젝트의 시각/상호작용 방향을 “fake terminal 시뮬레이터”에서 “TUI풍 스토리북 화면”으로 옮긴다.

중요한 차이는 다음이다.

- 실제 shell command를 배우거나 입력하는 구조가 아니다.
- `cd`, `ls`, `grep` 같은 명령어는 기본 조작이 아니다.
- 기본 조작은 읽기, 선택하기, 조사하기, 대화하기, 기억하기, 숨기기, 사용하기, 기다리기다.
- 화면은 평소에는 정적이고 차분해야 한다.
- 특수효과는 중요한 순간에만 발생해야 한다.
- 글리치와 움직임은 장식이 아니라 단서, 위협, 거짓말, 기억 오류, 세계 개입의 표현이어야 한다.

## 기대 효과

- 일반 게이머가 실제 터미널 문법을 몰라도 접근할 수 있다.
- 기존 회사 아포칼립스/기록물/결재문서/복사기/사원증 톤과 잘 맞는다.
- TUI 정체성을 검은 화면+초록 글자에 가두지 않고, 글자·선·여백·문단·그림 컷 전체로 확장할 수 있다.
- Pretext-like layout과 Canvas GlyphFX를 결합해 텍스트 자체를 연출 대상으로 만들 수 있다.
- “정적인 업무 문서가 어느 순간 살아 움직인다”는 프로젝트 고유의 감각을 만들 수 있다.

## 구현 후보

### 화면 모델

1. 텍스트 중심 장면
   - 상단 이름/챕터/간단 상태
   - 중앙 본문
   - 하단 선택지
   - 충분한 여백과 읽기 좋은 텍스트

2. 그림이 있는 상황 장면
   - 중요한 장소, 인물, 물체, 선택 직전에만 픽셀/TUI풍 장면 컷 삽입
   - 매 장면 그림을 넣지 않음

3. GlyphFX 발생 장면
   - 평소 HTML/CSS 본문을 정적으로 보여줌
   - 이상 현상 시작 시 텍스트를 글자/단어/줄 단위로 prepare
   - Canvas overlay에서 글자 흔들림, 치환, 밀림, 파편화, 리플로우 처리
   - 효과 종료 후 HTML 본문으로 복구하고 단서 단어만 강조

### 기술 방향

- Primary: Web / mobile portrait first
- Later: Tauri 또는 Electron desktop build
- Not primary: native Linux terminal app

역할 분담 후보:

```text
GameCore: story state and choices
Renderer: page UI and visual output
Pretext-like layout: text prepare/layout/cache
GlyphFX: animation/effect system
```

### MVP 후보

세 화면짜리 프로토타입:

1. 야근 중인 사무실에서 낡은 사원증을 발견하는 텍스트 중심 장면
2. 기록보관팀 앞 복도에서 황동판과 초록빛 모니터를 발견하는 그림 삽입 장면
3. 기록보관팀 내부에서 모니터 문장이 본문으로 흘러나와 선택지를 방해하고, 효과 후 `비상계단` 단서가 남는 GlyphFX 장면

## 주의점

- 이 아이디어는 아직 후보이며, 기존 Python/Textual TUI 구현을 즉시 폐기하라는 지시가 아니다.
- 현재 plan/todo가 있다면 그것을 먼저 처리해야 한다. 이 아이디어는 다음 설계/개발 후보로 검토한다.
- GlyphFX를 남발하면 싸구려 해킹 연출처럼 보일 수 있다.
- 단서는 글리치 중에도 최종적으로 읽을 수 있어야 한다.
- UI를 과하게 RPG/전투/상점/업적 중심으로 만들지 않는다.
- 그림은 사건의 밀도를 높일 때만 사용한다.
- 실제 사무실 위치, 개인 메모, 회사 기밀, 위험한 현실 행동은 공개 문서에 넣지 않는다.

## 설계자에게 물어볼 질문

- 이 컨셉을 기존 Python/Textual TUI의 장기 방향으로 대체할 것인가, 아니면 브라우저 fake-TUI 수직 슬라이스의 별도 고급 방향으로 둘 것인가?
- `Web / mobile portrait first`를 공식 primary runtime으로 전환할지, 아니면 현재 Python/Textual과 병행할지 결정해야 하는가?
- 기존 자원 시스템(체력, 정신력, 배터리, 허기, 갈증)은 상단에 얼마나 노출할 것인가?
- GlyphFX 첫 프로토타입은 기존 `web/` 구조 안에서 만들 것인가, 별도 실험 디렉터리에서 만들 것인가?
- Pretext-like layout에 실제 `@chenglou/pretext`를 사용할 것인가, 아니면 프로젝트용 lightweight text layout/cache를 먼저 만들 것인가?
- “그림이 있는 상황 화면”의 그림은 이미지 에셋, Canvas, CSS/HTML ASCII block 중 무엇을 우선할 것인가?
- 이 컨셉 문서를 공식 설계 방향으로 승격한다면 `docs/design/UI_Rules.md`, `docs/dev/TUI_Layout.md`, `docs/dev/Development_Plan.md` 중 어디까지 동기화해야 하는가?

## 처리 기록

- 2026-05-21: 사용자 아이디어를 `docs/design/TUI_Storybook_GlyphFX_Concept.md`로 문서화하고, 이 파일에 idea_box 후보로 별도 저장했다. 아직 채택/구현 여부는 미정이므로 `open` 상태를 유지한다.
- 2026-05-23: 감사 결과, 핵심 방향은 이미 `docs/design/TUI_Storybook_GlyphFX_Concept.md`, `docs/dev/Development_Plan.md`, `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/dev/Data_Schema.md`와 Web Storybook/SuperLightTUI 구현에 반영되어 있었다. 원 MVP 장면 자체는 메신저/복합기/복도 slice로 대체 흡수되었고, 새 구현 없이 adopted/merged로 `done` 처리한다.
