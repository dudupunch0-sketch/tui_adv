# /goal 프롬프트: idea_box backlog 처리

이 문서는 `tui_adv` 저장소에서 `idea_box`의 open backlog를 반복 처리할 때 `/goal`에 붙여넣기 위한 목표 프롬프트다.

핵심 의도:

- 흥미로운 아이디어를 임의로 고르지 않는다.
- `idea_box/BACKLOG_ORDER.md`의 가장 낮은 order open entry부터 1개만 처리한다.
- 먼저 “이미 반영됐는지”를 감사한다.
- 실제 반영, 폐기, 병합, 또는 canonical plan 승격이 끝났을 때만 `done` 처리한다.
- 단순히 읽었다는 이유로 `done` 처리하지 않는다.

## /goal에 붙여넣을 프롬프트

아래 블록을 그대로 `/goal`에 붙여넣어 사용한다.

```text
목표: 현재 repo의 `idea_box` open backlog를 Git 최초 추가 순서대로 1개 처리하라.

작업 언어: 한국어.

중요한 운영 규칙:
1. 이 repo의 canonical main plan은 `docs/dev/Development_Plan.md`다.
2. `docs/dev/Checklist.md`는 완료 추적용이지 독립적인 다음 계획 source가 아니다.
3. `idea_box/`는 active plan/todo가 없거나 사용자가 명시적으로 요청했을 때 처리하는 backlog다.
4. `idea_box/BACKLOG_ORDER.md`에서 가장 낮은 order의 `open` entry 1개만 선택한다. 임의로 흥미로운 항목을 고르지 않는다.
5. 읽기만 했다고 `done`으로 바꾸지 않는다. 실제 docs/code/data에 반영했거나, 명시적으로 폐기/병합 처리하고 이유를 적었을 때만 `done`이다.
6. Web Storybook + GlyphFX는 primary UX, Rust SuperLightTUI는 terminal-native horror/fallback edition이다. terminal 경로를 단순 debug dump로 격하하지 않는다.
7. private/local 현실 단서, 실제 사무실 정보, secrets는 공개 산출물에 넣지 않는다.
8. 변경은 최소화한다. 이번 backlog 처리에 직접 필요한 문서/코드만 수정한다.

시작 전 확인:
1. `git status --short --branch`로 현재 작업트리를 확인한다.
2. `docs/dev/Development_Plan.md` 상단의 계획 우선순위와 현재 활성 방향을 확인한다.
3. `idea_box/README.md`와 `idea_box/BACKLOG_ORDER.md`를 확인한다.
4. 선택한 idea entry의 frontmatter, `source_ref`, `related_docs`, 처리 기록을 읽는다.
5. 필요한 경우 `/home/dudupunch0/rg/rg`를 우선 사용해 README/docs/src/web/tests에서 이미 반영된 흔적을 찾는다. 없으면 일반 search 도구를 사용한다.

처리 절차:
1. 선택한 open idea를 요약한다.
   - 어떤 문제/가치를 다루는가?
   - 현재 main plan과 충돌하는가, 보완하는가?
   - Web Storybook/GlyphFX primary 방향 또는 SuperLightTUI 방향과 어떤 관계인가?

2. 현재 repo 반영 상태를 감사한다.
   - 이미 반영된 항목
   - 부분 반영된 항목
   - 아직 빠진 항목
   - 현재 방향과 맞지 않아 폐기/병합할 항목
   로 나눈다.

3. 판정한다.
   - 이미 충분히 반영됨: 새 구현 없이 idea entry를 `done` 처리한다.
   - 일부만 반영됨: 빠진 핵심을 `docs/dev/Development_Plan.md` 또는 적절한 설계 문서에 최소 승격한 뒤 `done` 처리할 수 있다.
   - 아직 구현/설계가 필요함: 사용자가 실행까지 요청한 경우에만 최소 slice를 구현한다. 실행 요청이 아니라면 canonical plan에 다음 slice로 승격하고 idea entry에는 그 사실을 기록한다.
   - 현재 방향과 맞지 않음: 폐기 또는 다른 문서로 병합 처리하고, 이유를 처리 기록에 남긴 뒤 `done` 처리한다.

4. 필요한 파일만 수정한다.
   - idea entry frontmatter:
     - `status: done`
     - `used_by: <반영/병합/폐기 근거 파일 또는 문서>`
     - `done_at: <date -I 결과>`
   - idea entry `## 처리 기록`:
     - 어떤 판단을 했는지
     - 어디에 반영/병합/폐기했는지
     - 구현이 남아 있다면 어느 문서의 어떤 후속 slice로 승격했는지
   - `idea_box/BACKLOG_ORDER.md`:
     - 현재 open backlog 표에서 처리한 entry를 제거하거나 상태를 갱신한다.
     - 이미 done인 구조화 entry 표에 처리한 entry를 추가한다.
   - 필요한 경우에만 `docs/dev/Development_Plan.md`, 설계 문서, README, 테스트/코드를 수정한다.

5. 검증한다.
   - 문서만 바뀐 경우: `git diff --check`를 실행한다.
   - 코드/스키마/테스트가 바뀐 경우: 관련 targeted test를 먼저 실행하고, 가능한 범위에서 전체 검증을 실행한다.
   - markdown 표, frontmatter, code fence가 깨지지 않았는지 확인한다.
   - `git diff --stat`과 `git diff -- <changed files>`로 변경 범위가 목표와 일치하는지 검토한다.

완료 기준:
1. 처리한 idea entry 1개가 명확히 선택되어 있다.
2. 해당 entry와 source/related docs를 읽었다.
3. 이미 반영/부분 반영/미반영/폐기 또는 병합 판단이 기록되어 있다.
4. 실제 반영, 폐기, 병합, 또는 canonical plan 승격 없이 `done` 처리하지 않았다.
5. `idea_box/BACKLOG_ORDER.md`가 최신 상태다.
6. 검증 명령 결과를 확인했다.
7. 최종 응답에는 다음을 한국어로 요약한다.
   - 선택한 backlog entry
   - 판정: 이미 반영 / plan 승격 / 구현 완료 / 폐기 / 병합
   - 변경한 파일
   - 실행한 검증
   - 다음 open backlog가 무엇인지
```

## agent가 지켜야 할 판단 기준

### 이미 반영된 것으로 볼 수 있는 경우

다음 중 하나 이상이 명확하면 “이미 반영됨” 또는 “대부분 반영됨”으로 볼 수 있다.

- idea의 핵심 방향이 `docs/dev/Development_Plan.md` 상단의 active direction에 이미 들어 있다.
- 관련 설계 계약이 `docs/dev/Data_Schema.md`, `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/design/UI_Rules.md`, `docs/dev/TUI_Layout.md` 등에 구체화되어 있다.
- 관련 구현 또는 테스트가 `src/`, `crates/`, `web/`, `tests/`에 존재한다.
- README나 개발 문서가 사용자/agent에게 같은 방향을 이미 안내한다.

단, “비슷한 단어가 있다”만으로는 충분하지 않다. idea의 핵심 결정 또는 후속 작업 위치가 실제로 repo에 들어 있어야 한다.

### canonical plan에 승격하고 done 처리할 수 있는 경우

idea가 아직 구현되지는 않았지만, 현재 방향에 맞고 후속 작업으로 보존해야 한다면 다음 조건을 모두 만족할 때 `done` 처리할 수 있다.

- `docs/dev/Development_Plan.md` 또는 적절한 설계 문서에 구체적인 후속 slice로 반영했다.
- idea entry 처리 기록에 “구현 완료”가 아니라 “canonical plan에 승격”이라고 명확히 썼다.
- `used_by`가 실제 승격된 문서 경로를 가리킨다.

### done 처리하면 안 되는 경우

- 단순히 읽고 요약만 했다.
- “나중에 하면 좋겠다”라고 말했지만 어느 문서에도 반영하지 않았다.
- 관련 구현이 필요하다는 사실만 확인하고 계획/문서/코드에 아무 변경도 하지 않았다.
- 여러 open idea를 한꺼번에 읽고 선택 기준이 흐려졌다.

## 권장 최종 응답 형식

```text
처리 완료.

- 선택한 backlog: <order> <path>
- 판정: <이미 반영 / plan 승격 / 구현 완료 / 폐기 / 병합>
- 변경 파일:
  - <path>: <변경 이유>
- 검증:
  - <command>: <결과>
- 다음 open backlog:
  - <order> <path> 또는 없음
```
