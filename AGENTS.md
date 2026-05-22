# AGENTS.md

이 파일은 이 저장소에서 작업하는 AI agent를 위한 프로젝트 운영 지침이다.

## 프로젝트 성격

- 한국어 회사-아포칼립스 선택지 게임이다.
- 시각 정체성은 TUI/fake-terminal 분위기를 유지하되, 현재 활성 방향은 Web Storybook + GlyphFX primary UX와 SuperLightTUI terminal-native horror edition이다.
- 특수 효과는 Web Storybook의 Canvas/GlyphFX와 SuperLightTUI terminal cell/GlyphFX 쪽으로 흡수한다. 기존 browser fake-TUI는 legacy/parity fallback으로만 취급한다.
- 실제 사용자의 메모/사적 노트는 공개 산출물로 옮기지 않는다.

## 계획 문서 우선순위

- `docs/dev/Development_Plan.md`가 이 저장소의 canonical main plan이다. 현재 방향, 다음 작업, 우선순위, phase 순서는 이 파일을 기준으로 판단한다.
- 사용자가 “다음 작업”, “계속해”, “계획대로 해”처럼 말하면 먼저 `docs/dev/Development_Plan.md`의 상단 우선순위와 “현재 최우선 남은 작업” / “다음 액션”을 확인한다.
- `docs/dev/Checklist.md`는 완료 여부 추적용이며, 독립적인 다음 계획 source가 아니다.
- 아키텍처/스키마 문서는 구현 계약 참조이고, README는 실행법과 문서 입구다.
- `.hermes/plans/`는 일회성 세션 artifact이며 canonical 계획으로 쓰지 않는다.

## idea_box 운영 규칙

이 프로젝트에는 `idea_box/`가 있다. 사용자가 별도 세션에서 떠오르는 아이디어를 저장하는 공간이다.

### 확인 우선순위

- 남아 있는 plan, todo list, 또는 명시된 사용자 지시가 있으면 그것을 먼저 따른다.
- 모든 작업 시작 전에 `idea_box`를 확인하지 않는다. 이는 현재 작업의 우선순위를 흐릴 수 있다.
- 현재 세션에서 처리할 남은 plan/todo가 없을 때만 `idea_box/README.md`, `idea_box/BACKLOG_ORDER.md`, `idea_box/inbox/`의 열린 아이디어를 확인해 다음 설계/개발 항목을 찾는다.
- 사용자가 직접 `idea_box` 확인을 요청한 경우에는 즉시 확인한다.

### 아이디어 처리

- 아이디어는 즉시 현재 작업에 끼워 넣는 요구사항은 아니지만, `status: done`이 아닌 entry는 반영되지 않은 backlog다.
- 남은 plan/todo가 없거나 사용자가 `idea_box` 처리를 요청하면 `idea_box/BACKLOG_ORDER.md`의 Git 최초 추가 순서대로 처리한다.
- 프로젝트의 톤, 우선순위, 현재 구현 단계에 맞지 않으면 구현하지 않고 폐기/병합 판단을 할 수 있지만, 그 이유를 처리 기록에 남겨야 한다.
- 아이디어를 실제 설계/문서/구현에 사용했거나 명시적으로 폐기/병합 처리했다면 `done` 처리한다.
- `done`은 단순히 읽었다는 뜻이 아니다. 어디에 반영했는지, 또는 왜 폐기/병합했는지 기록한다.
- 아이디어 파일은 삭제하지 않는다.

자세한 파일 형식과 처리 방식은 `idea_box/README.md`를 따른다.
