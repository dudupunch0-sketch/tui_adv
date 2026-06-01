---
type: agent_prompt
created: 2026-06-01
prompt_for: wuxia_seo_harin_rescue_implementation
mode: implementation
---

# Prompt: `wuxia_seo_harin_rescue` preview runtime 구현

이 파일은 새 Hermes/agent 구현 세션에 그대로 읽혀 실행되도록 작성한 handoff prompt다.

## 역할

`wuxia_jianghu_pack` storypack preview에 네 번째 runtime encounter `wuxia_seo_harin_rescue`를 구현한다. 이 slice는 first fight / cheonggi fragment 이후 주인공을 서하린 구조, 청류문 보호/감시, 다음 수습생 bridge로 연결한다.

Hermes skill system을 사용할 수 있으면 먼저 다음 skill을 load한다.

- `workspace-continuation-safety`
- `test-driven-development`
- `tmp-rust-cargo-toolchain`
- `narrative-tui-game-development` 또는 story/content 관련 동등 workflow

## Grounding

먼저 fresh `origin/main` 기준인지 확인한다.

```bash
cd /home/dudupunch0/tui_adv
git fetch --prune origin
git status --short --branch -uall
git log --oneline -1 HEAD
git log --oneline -1 origin/main
git diff --stat HEAD origin/main
```

- 현재 checkout이 squash-merged feature branch이거나 `origin/main`과 다르면, 구현은 fresh branch/worktree from `origin/main`에서 시작한다.
- 이미 사용자가 구현 branch를 지정했다면 그 branch를 쓰되, 위 상태를 보고한다.
- 이 host의 `/home`은 작다. build/test/install/run 명령 전에는 `/home/dudupunch0/.config/tui_adv/tmp-installs.sh`를 source하거나 동등한 tmp cache 설정을 사용한다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - 0.15 `wuxia_seo_harin_rescue`
  - 0.19 Notion 이구학지 운영 기준 반영
- `docs/dev/Notion_Design_Coverage.md`
- `idea_box/notion_sources.yml`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- 현재 preview source:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`

Repo canonical docs가 이 prompt와 충돌하면 repo docs를 우선하고, 충돌 사실을 보고한다. Notion 원문을 runtime spec으로 직접 쓰지 않는다.

## 구현 목표

Preview source와 generated preview artifacts에만 `wuxia_seo_harin_rescue`를 추가한다.

Expected runtime behavior:

- 시작 위치: `jianghu_market_street`.
- 새 destination 후보: `cheongryu_outer_courtyard`.
- phase: `rescue_and_investigation`.
- `runtime_mode: storypack_preview` boundary 유지.
- 기존 preview route `wuxia_commute_rift_arrival` → `wuxia_heuksa_bang_first_fight` → `wuxia_cheonggi_record_first_fragment` 뒤에 붙는다.
- default office bundle에는 무협 id가 계속 없어야 한다.

## 예상 수정 파일

Preview source:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - `cheongryu_outer_courtyard` 또는 동등한 청류문 외곽 거점 위치 추가.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `wuxia_seo_harin_rescue` 추가.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`
  - 실제 outcome에서 preview-only item을 추가/소모할 때만 수정.

Generated preview artifacts only:

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

Likely tests/docs if needed:

- `tests/test_web_data_export.py`
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- docs sync only if implementation changes docs-visible contracts.

## Required / forbidden conditions

Recommended encounter conditions:

```yaml
runtime_mode: storypack_preview
conditions:
  locations:
    - jianghu_market_street
required_flags:
  - heuksa_bang_first_fight_resolved
  - cheonggi_record_first_fragment_resolved
forbidden_flags:
  - seo_harin_rescue_resolved
```

If the existing schema expresses these fields differently, match the existing schema and tests rather than inventing a new schema.

## Stable choice ids

Use these stable choice/action ids unless existing runtime conventions require a mechanical variant:

- `tell_plain_truth` — fallback/safe honesty. 진실을 말하지만 감시 대상으로 이동한다.
- `ask_for_medical_help_first` — survival priority. 치료/안전은 얻지만 채무 hook을 남긴다.
- `explain_company_and_commute` — workplace memory probe. 현대어가 통하지 않아 오해와 sanity/resource cost를 남긴다.
- `show_cheonggi_record_page` — risky record disclosure. 천기록을 보여 도움/위험을 동시에 부른다.
- `hide_employee_badge` — high-risk concealment. 사원증/수첩은 숨기지만 의심 flag를 키운다.

## Outcome hooks

기존 runtime schema만 사용한다.

Allowed hook families:

- `resources`
- `danger`
- `add_flags`
- `add_clues`
- `add_items` / `remove_items`
- `destination_id`
- `log`
- optional `presentation` / `effect_cues` if already supported

Every rescue outcome should leave the common bridge hooks needed by the next slice:

- `seo_harin_rescue_resolved`
- `seo_harin_intervened`
- `taken_under_watch`
- preferably `outsider_claim_recorded`
- `destination_id: cheongryu_outer_courtyard`

Branch-specific flags/clues/logs may include:

- `rescue_debt_recorded`
- `seo_harin_noticed_cheonggi_record`
- `cheonggi_record_must_be_hidden`
- `cheongryu_name_heard`
- `sect_identity_matters`
- `company_words_fail_clue`
- `sect_protection_has_price`
- `notebook_draws_sect_attention`

Tone: 구조는 구원이 아니라 보호와 감시의 시작이다.

## 금지 범위

이 slice에서 열지 않는다.

- 새 `RelationScore`
- `DebtLedger`
- `FactionStanding`
- healing schema
- companion schema
- combat/reward/ability schema
- 천외편린 3택 lock-in UI
- `wuxia_cheongryu_apprentice_entry` 구현
- yageunmong runtime
- 기본 office bundle 변경
- `escape-office` save/localStorage key 변경
- default office generated bundle에 무협 id 추가

## 구현 순서

1. Repo 상태와 canonical docs를 읽고, 현재 preview source의 기존 세 encounter/location/item conventions를 확인한다.
2. 실패하는 targeted test를 먼저 추가/수정한다.
   - preview bundle에 `wuxia_seo_harin_rescue`와 `cheongryu_outer_courtyard`가 포함되어야 한다.
   - default office bundle에는 무협 id가 없어야 한다.
   - Rust/Web/terminal/WASM parity에서 encounter/action id가 보여야 한다.
3. preview source YAML에 location/encounter를 최소 추가한다.
4. preview bundle export 명령으로 Rust/Web generated preview artifact만 재생성한다.
5. targeted tests를 통과시킨다.
6. `git diff --check`로 whitespace를 확인한다.
7. 최종 보고에서 default office bundle 보호 결과를 별도 항목으로 보고한다.

## 검증 명령

Python/docs/exporter:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
```

Preview export check:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
```

Rust/WASM/terminal targeted tests, adapting exact test names to the codebase:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo test -p escape-core --test content_bundle
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_seo_harin_rescue_through_preview_bundle
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_seo_harin_rescue
```

Web targeted test, if Web bundle parity changed:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cd web && npm test -- --run src/core/contentBundles.test.ts
```

Final diff check:

```bash
git diff --check
git diff --stat
```

If a command name/test selector differs, inspect existing tests and use the nearest targeted equivalent. Do not skip verification silently; report any unavailable tool or environment blocker.

## 완료 보고 형식

한국어로 간결하게 보고한다.

```text
`wuxia_seo_harin_rescue` 구현 완료.

1. 기준 repo 상태
- branch:
- HEAD:
- base:

2. 변경 파일
- preview source:
- generated preview artifacts:
- tests:
- docs:

3. 구현 내용
- encounter id:
- destination:
- required/forbidden flags:
- choice ids:
- common outcome hooks:
- default office bundle 보호 결과:

4. 검증
- <command>: <pass/fail/skip + 이유>

5. 남은 일 / caveat
-
```
