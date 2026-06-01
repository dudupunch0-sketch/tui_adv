# idea_box

이 폴더는 사용자가 별도 세션에서 떠오르는 아이디어를 임시 저장하는 공간이다. 다른 agent는 이곳의 아이디어를 나중에 설계/문서/구현 후보로 사용할 수 있다.

현재 표준은 Notion-first다. 원본 reference는 Notion 문서이고, 이 repo의 `idea_box/inbox/*.md`는 Notion reference를 추적하고 처리 순서를 관리하는 구조화 entry다.

## 확인 시점

- 남아 있는 plan, todo list, 명시된 사용자 지시가 있으면 그것을 먼저 따른다.
- 현재 세션에서 처리할 남은 plan/todo가 없을 때만 `idea_box`를 확인해 다음 개발/설계 후보를 찾는다.
- 사용자가 직접 `idea_box`를 확인하라고 요청한 경우에는 즉시 확인한다.
- 아이디어 검토가 현재 작업의 우선순위를 밀어내거나 범위를 임의로 확장해서는 안 된다.

## 디렉터리 구조

```text
idea_box/
  README.md
  IDEA_INTAKE_GUIDE.md      # 별도 아이디어 수집 세션 운영 지침
  BACKLOG_ORDER.md          # status: done이 아닌 idea entry의 처리 순서
  LLM_DESIGN_HANDOFF.md     # 다른 LLM/agent용 스토리팩/인카운터/6스탯 설계 표지 문서
  next_goal/                # 다른 세션에 넘길 단일 현재 목표 prompt
  inbox/                    # 아직 반영하지 않은/반영 대기 중인 구조화 아이디어
  done/                     # 반영, 폐기, 병합 처리가 끝난 아이디어
```

## Notion-first 설계 파이프라인

설계 아이디어는 다음 단계로 처리한다.

1. **Notion 정리**: 사용자가 Notion에 원본 아이디어를 적는다. 이 Notion 문서가 원본 reference다.
2. **설계 아이디어 문서화**: agent는 Notion reference를 읽고 repo 안의 설계 아이디어 문서로 변환한다. 보통 `docs/design/`, `docs/content/`, `docs/story/` 중 적절한 위치에 candidate 문서를 만든다. `idea_box/inbox/*.md`에는 Notion page id/title/url, 간단 요약, `related_docs`를 남긴다.
3. **main plan 격상**: 다음에 실제로 설계할 항목은 설계 아이디어 문서 중 하나를 `docs/dev/Development_Plan.md`의 active main plan / “현재 최우선 남은 작업”으로 격상시킨 뒤 진행한다.
4. **설계 진행**: 격상된 main plan을 기준으로 설계 문서와 필요한 계약 문서를 작성한다. runtime YAML/Rust/Web 구현은 별도 요청이나 별도 runtime slice가 있을 때만 한다.
5. **Notion reference 대조**: 설계 후 Notion 원본 reference와 실제 설계 결과를 비교해 핵심 방향, 톤, 제약, non-goals가 일치하는지 확인하고 처리 기록에 남긴다.
6. **done 처리**: Notion 대조까지 끝났거나, 명시적으로 폐기/병합 판단을 기록했을 때만 `done` 처리한다. Notion import나 설계 아이디어 문서 작성만으로는 `done`이 아니다.

## backlog 처리 순서

`idea_box`에서 `status: done`이 아닌 idea entry는 아직 반영되지 않은 backlog다. 처리 순서는 파일이 Git에 처음 추가된 commit 순서를 따른다. 원격 push 시각 자체는 일반 Git history만으로 안정적으로 복원할 수 없으므로, 이 repo에서는 “Git에 올라간 순서”를 “파일이 처음 추가된 commit 순서”로 정의한다.

현재 순서는 `BACKLOG_ORDER.md`에 명시한다. 다음 개발/설계 후보를 찾는 agent는 임의로 흥미로운 항목을 고르지 말고, `BACKLOG_ORDER.md`에서 가장 낮은 order의 open 항목부터 처리한다.

새 idea entry를 만들 때 같은 날짜 파일명이 여러 개라 순서가 애매하면 frontmatter에 `backlog_order`, `git_added_at`, `git_added_commit`을 추가한다. Git 최초 추가 시점을 알 수 없으면 처리 전에 파일명을 바꾸거나 order metadata를 추가해 순서를 명시한다.

루트의 원문/가이드 문서는 source 또는 운영 문서일 수 있다. `source_ref`가 있는 `inbox` entry가 있으면 그 entry의 `status`와 order를 따른다.

## 다른 LLM/agent 설계 핸드오프

스토리팩, 인카운터 상황 DB, 6스탯 기반 등장인물 설계를 이어갈 LLM/agent는 `BACKLOG_ORDER.md`를 먼저 읽고 현재 처리 순서를 확인한 뒤, 해당 작업이 storypack/encounter 관련이면 `LLM_DESIGN_HANDOFF.md`를 읽는다.

`LLM_DESIGN_HANDOFF.md`는 관련 설계 문서의 읽는 순서와 작업 단위를 안내하는 표지판이다. 즉시 구현 지시가 아니며, runtime YAML/Rust/Web 코드 변경은 사용자가 명시적으로 요청했을 때만 한다.

긴 copy-paste prompt를 다른 세션에 넘겨야 할 때는 `idea_box/next_goal/`를 사용한다. 이 폴더는 단일 현재 목표 prompt이며, 기본적으로 `README.md` 하나만 둔다. 새 세션에는 아래처럼 짧게 지시하면 된다.

```text
이 repo의 idea_box/next_goal/ 폴더를 읽고 README의 현재 목표만 수행해. repo canonical docs와 prompt가 충돌하면 canonical docs를 우선해.
```

## 아이디어 수집 세션

사용자가 별도 세션에서 계속 아이디어를 제공하는 경우에는 `IDEA_INTAKE_GUIDE.md`를 따른다.

핵심 원칙은 다음과 같다.

- 사용자가 던지는 내용은 확정 요구사항이 아니라 아이디어 후보로 정리한다.
- 아이디어 하나당 파일 하나를 기본으로 `idea_box/inbox/`에 저장한다.
- 애매하거나 이상하거나 이해가 가지 않는 부분은 agent가 임의로 확정하지 않고, 아이디어 문서 안의 `설계자에게 물어볼 질문` 섹션에 남긴다.
- 사용자의 아이디어 흐름을 매번 과도한 확인 질문으로 끊지 않는다.

## 아이디어 파일 형식

아이디어 하나당 Markdown 파일 하나를 만든다. 권장 위치는 `idea_box/inbox/`이다.

파일명 예시:

```text
idea_box/inbox/2026-05-21-fake-terminal-glitch.md
```

권장 템플릿:

```md
---
status: open
created: YYYY-MM-DD
source: notion
notion_page_id:
notion_title:
notion_url:
backlog_order:
git_added_at:
git_added_commit:
related_docs:
main_plan_ref:
reference_check:
used_by:
done_at:
---

# 아이디어 제목

## 핵심
짧은 요약.

## 왜 필요한가
프로젝트에 주는 가치.

## 구현 힌트
있으면 적고, 없으면 비워둔다.

## 주의점
톤, 우선순위, 구현 리스크 등.

## 처리 기록
- 아직 미사용.
```

## 상태 규칙

- `open`: 아직 반영되지 않은 backlog idea entry. `BACKLOG_ORDER.md`의 Git 최초 반영 순서대로 처리한다.
- `done`: 실제 설계/문서/구현에 반영했고 Notion reference 대조까지 마쳤거나, 명시적으로 폐기/병합 처리한 아이디어.

`done`은 단순히 읽었다는 뜻이 아니다. Notion import, 설계 아이디어 문서 작성, 또는 main plan 격상만으로는 부족하다. 최종 설계가 원본 Notion reference와 같은 방향인지 확인한 기록이 있거나, 폐기/병합 이유가 있어야 한다.

## 처리 방법

Notion-origin 아이디어를 사용할 때는 먼저 Notion reference를 다시 읽고, `related_docs`의 설계 아이디어 문서와 `docs/dev/Development_Plan.md`의 격상 상태를 확인한다.

아이디어를 사용했으면 다음 중 하나를 수행한다.

1. 파일의 frontmatter를 갱신한다.

```md
status: done
reference_check: Notion reference 대조 완료, YYYY-MM-DD
used_by: docs/some-file.md 또는 src/some-file.ts
done_at: YYYY-MM-DD
```

2. 또는 파일을 `idea_box/done/`으로 옮기고 처리 기록에 어디에 어떻게 반영했는지 적는다.

아이디어 파일은 삭제하지 않는다. 나중에 추적할 수 있어야 한다.

## 중요한 운영 원칙

아이디어는 반영 대기 backlog 입력이다. 프로젝트의 현재 plan/todo가 있으면 그것을 먼저 끝내되, 이후에는 `BACKLOG_ORDER.md`의 오래된 open 항목부터 처리한다. 반영하지 않기로 결정한 경우에도 그냥 넘기지 말고 폐기/병합 이유를 처리 기록에 남기고 `done` 처리한다.
