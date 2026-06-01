---
type: agent_prompt
created: 2026-06-01
prompt_for: seo_harin_future_design
mode: read_only_or_docs_only
---

# Prompt: 서하린 companion/emotional-axis + 후일담 future design 정리

이 파일은 `wuxia_seo_harin_rescue` 구현과 병렬로 돌릴 수 있는 future design 세션용 handoff prompt다.

## 역할

이번 세션은 구현 세션이 아니다. Notion source와 repo coverage를 기준으로, 서하린 companion/emotional-axis와 후일담 future design을 정리한다. 이것을 현재 `wuxia_seo_harin_rescue` 구현 scope에 끼워 넣지 않는다.

Hermes skill system을 사용할 수 있으면 먼저 다음 skill을 load한다.

- `workspace-continuation-safety`
- `notion-update-tui`
- `waza-think` 또는 동등한 planning/reasoning workflow

## 절대 금지

- runtime code, YAML, Rust, Web, tests, generated bundle 수정 금지.
- `wuxia_seo_harin_rescue` 현재 구현 scope 확장 금지.
- 후일담 DB row를 runtime 완료로 표시하지 않는다.
- companion/relation/debt/faction schema를 현재 runtime requirement로 만들지 않는다.
- 기본 office bundle, 기본 generated bundle, `escape-office` save/localStorage key 수정 금지.

## 목표

- 서하린 future design이 현재 rescue bridge와 어디서 분리되는지 명확히 정리한다.
- companion/relation schema를 지금 열지 않는 이유를 정리한다.
- 나중에 relation/debt/companion/epilogue schema를 열 때 필요한 최소 조건을 정리한다.
- `wuxia_seo_harin_rescue` 구현 세션이 건드리면 안 되는 companion/epilogue 범위를 명확히 한다.
- 다음 docs-only 후보 또는 implementation 후보를 추천한다.

## 먼저 repo 상태 확인

```bash
cd /home/dudupunch0/tui_adv
git fetch --prune origin
git status --short --branch -uall
git log --oneline -1 HEAD
git log --oneline -1 origin/main
git diff --stat HEAD origin/main
```

가능하면 fresh `origin/main` 기준으로 read-only 검토한다. docs-only 수정을 해야 하면 fresh branch from `origin/main`에서 한다.

## 반드시 읽을 repo docs

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Notion_Design_Coverage.md`
- `idea_box/notion_sources.yml`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`

## Notion/live source 기준

Notion live access가 가능하고 사용자가 broad audit을 허용했거나 repo docs가 부족하면 관련 source만 확인한다. token/credential은 출력하지 않는다.

우선 확인 후보:

- `서하린 상세 설계`
- Notion 사건 DB에서 서하린 companion/emotional-axis 관련 rows
- Notion 후일담 DB rows:
  - `epilogue_seoharin_closed_gate`
  - `epilogue_seoharin_last_bowl`
  - `epilogue_seoharin_open_gate`
  - `epilogue_seoharin_empty_place`
  - `epilogue_seoharin_future`

Live access가 불가능하면 `docs/dev/Notion_Design_Coverage.md`와 `idea_box/notion_sources.yml` 기준으로 “live Notion recheck needed”를 caveat로 남긴다.

## 산출물

Read-only 보고서 또는 docs-only 최소 보완을 만든다. 보고서는 한국어로 작성한다.

반드시 포함:

1. 기준 repo 상태.
2. 확인한 canonical docs / Notion sources.
3. 현재 `wuxia_seo_harin_rescue` scope와 분리해야 하는 future scope.
4. companion/emotional-axis를 지금 구현하지 않는 이유.
5. epilogue/future design을 열기 위한 전제 조건.
6. 구현 세션이 건드리면 안 되는 범위.
7. 다음 docs-only 후보 / implementation 후보 추천.
8. docs-only 수정이 있었다면 `git diff --check` 결과.

## 수정 기준

기본은 read-only다. docs-only 수정이 필요한 경우에만 다음 파일 중 최소 범위를 수정한다.

- `docs/dev/Development_Plan.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `idea_box/notion_sources.yml`

Runtime source/test/generated artifact는 수정하지 않는다.

## 최종 보고 형식

```text
서하린 future design 검토 완료.

1. 기준 repo 상태
- branch:
- HEAD:
- origin/main:
- diff vs origin/main:

2. 결론
- rescue 구현 scope에 포함할 것:
- rescue 구현 scope에서 제외할 것:
- companion/epilogue future design 상태:

3. 확인한 source
-

4. future design 요약
-

5. 구현 세션 금지 범위
-

6. 다음 후보
-

7. 검증 / caveat
-
```
