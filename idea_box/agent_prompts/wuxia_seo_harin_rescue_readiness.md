---
type: agent_prompt
created: 2026-06-01
prompt_for: wuxia_seo_harin_rescue_readiness
mode: docs_only
---

# Prompt: `wuxia_seo_harin_rescue` readiness 검수

이 파일은 새 Hermes/agent 세션에 그대로 읽혀 실행되도록 작성한 handoff prompt다.

## 역할

이번 세션은 구현 세션이 아니다. 최신 `origin/main`의 canonical docs를 기준으로, 다음 구현 세션이 `wuxia_seo_harin_rescue`를 재판단 없이 구현할 수 있는지 확인하는 readiness / 최종 handoff 세션이다.

Hermes skill system을 사용할 수 있으면 먼저 다음 skill을 load한다.

- `workspace-continuation-safety`
- `notion-update-tui`
- `waza-think` 또는 동등한 planning/reasoning workflow

## 목표

- 최신 `origin/main` 기준으로 `wuxia_seo_harin_rescue` 구현 handoff가 충분한지 검수한다.
- 이미 충분하면 문서를 불필요하게 고치지 말고, 구현 세션이 사용할 최종 요약만 보고한다.
- 구현 blocker가 되는 모호함이 있으면 docs-only로 최소 보완한다.
- runtime YAML/Rust/Web/generated bundle은 절대 수정하지 않는다.

## 절대 금지

- `src/tui_adv/storypack-previews/**` 수정 금지.
- `crates/escape-core/fixtures/content/**` 수정 금지.
- `web/src/data/generated/**` 수정 금지.
- 기본 office bundle, 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key 수정 금지.
- Notion 원문을 runtime 구현 spec으로 직접 쓰지 않는다. repo canonical docs가 구현 gate다.
- Notion이 새로 업데이트됐다는 지시가 없으면 전체 Notion live check를 반복하지 않는다.
- repo 문서끼리 충돌하거나 `wuxia_seo_harin_rescue` handoff가 모호할 때만 관련 Notion source를 live check한다. token/credential은 출력하지 않는다.

## 먼저 repo 상태 확인

```bash
cd /home/dudupunch0/tui_adv
git fetch --prune origin
git status --short --branch -uall
git log --oneline -1 HEAD
git log --oneline -1 origin/main
git diff --stat HEAD origin/main
```

판단 기준:

- 가능하면 fresh `origin/main` 기준으로 판단한다.
- 현재 checkout이 squash-merged feature branch라면 그 사실을 보고하고, 실제 다음 작업은 `origin/main` 기반 fresh branch/worktree에서 해야 한다고 명시한다.
- docs를 수정해야 하면 fresh branch from `origin/main`을 권장한다.
- read-only handoff만 작성하면 branch 생성 없이 보고만 해도 된다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - 특히 0.15 `wuxia_seo_harin_rescue`
  - 0.19 Notion 이구학지 운영 기준 반영
- `docs/dev/Notion_Design_Coverage.md`
- `idea_box/notion_sources.yml`
- `idea_box/agent_prompts/wuxia_seo_harin_rescue_implementation.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`

Read-only로만 확인할 runtime preview source:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`

## 검수할 핵심 결정

1. 다음 구현 slice가 여전히 `wuxia_seo_harin_rescue`인지 확인한다.
2. Notion DB row `wuxia_seoharin_intervention` / `서하린의 개입`과 repo `wuxia_seo_harin_rescue` 매핑이 충분한지 확인한다.
3. 구현 시작 조건이 충분히 명확한지 확인한다.
   - 권장:
     - `runtime_mode: storypack_preview`
     - `conditions.locations: [jianghu_market_street]`
     - `required_flags: [heuksa_bang_first_fight_resolved, cheonggi_record_first_fragment_resolved]`
     - `forbidden_flags: [seo_harin_rescue_resolved]`
4. outcome destination이 충분히 명확한지 확인한다.
   - 권장: 새 preview location `cheongryu_outer_courtyard`.
5. 모든 rescue outcome이 다음 slice를 위해 최소 공통 hook을 남기는지 확인한다.
   - `seo_harin_rescue_resolved`
   - `seo_harin_intervened`
   - `taken_under_watch`
   - 가능하면 `outsider_claim_recorded`
6. choice id가 안정적인지 확인한다.
   - `tell_plain_truth`
   - `ask_for_medical_help_first`
   - `explain_company_and_commute`
   - `show_cheonggi_record_page`
   - `hide_employee_badge`
7. forbidden scope가 명확한지 확인한다.
   - 새 `RelationScore` 금지
   - `DebtLedger` 금지
   - `FactionStanding` 금지
   - healing schema 금지
   - companion schema 금지
   - combat/reward/ability schema 금지
   - 천외편린 3택 lock-in UI 금지
   - `wuxia_cheongryu_apprentice_entry`까지 한 번에 구현 금지

## 수정 기준

- 이미 문서가 충분하면 수정하지 않는다.
- 모호한 부분이 구현을 막는 blocker라면 docs-only로 최소 수정한다.
- 수정 가능한 파일은 docs/ledger/prompt 파일뿐이다.
  - `docs/dev/Development_Plan.md`
  - `docs/dev/Notion_Design_Coverage.md`
  - `docs/content/encounter_db/wuxia_jianghu_pack.md`
  - `docs/content/storypacks/wuxia_jianghu_pack.md`
  - `idea_box/notion_sources.yml`
  - `idea_box/agent_prompts/*.md` if the prompt is stale

## 검증

Docs-only 수정이 있으면 최소한 다음을 실행한다.

```bash
git diff --check
git diff --stat
```

JSON/YAML ledger를 수정했다면 관련 syntax check도 실행한다.

## 최종 보고 형식

한국어로 다음 형식을 지킨다.

```text
`wuxia_seo_harin_rescue` readiness 검수 완료.

1. 기준 repo 상태
- branch:
- HEAD:
- origin/main:
- diff vs origin/main:

2. 결론
- 구현 진행 가능 여부:
- blocker:
- docs 수정 여부:

3. 확인한 canonical docs
-

4. 구현 세션 handoff 요약
- 목표:
- 수정 예상 파일:
- required flags:
- forbidden flags:
- choice ids:
- outcome hooks:
- destination:
- 금지 범위:
- 검증 명령:

5. 구현 세션에 줄 파일
- `idea_box/agent_prompts/wuxia_seo_harin_rescue_implementation.md`
- 새 세션 지시 예시:
  `이 repo의 idea_box/agent_prompts/wuxia_seo_harin_rescue_implementation.md를 읽고 그대로 수행해.`

6. 검증 결과
- `git diff --check`:
- `git diff --stat`:
```
