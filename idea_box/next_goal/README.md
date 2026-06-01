---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-01
current_goal: wuxia_cheongryu_raid_route_split
mode: implementation
---

# next_goal

이 폴더는 다른 Hermes/agent 세션에 넘길 단일 prompt entry point다. 앞으로 새 세션에는 긴 프롬프트를 복사하지 말고 아래처럼 짧게 지시한다.

```text
이 repo의 idea_box/next_goal/ 폴더를 읽고 README의 현재 목표만 수행해. repo canonical docs와 충돌하면 canonical docs를 우선하고 충돌 사실을 보고해.
```

운영 원칙:

- 이 폴더에는 기본적으로 이 README 하나만 둔다.
- 여러 prompt 파일이나 future-design 분기 prompt를 만들지 않는다.
- 이 README는 “지금 다음으로 할 일” 하나만 가리킨다.
- 목표가 바뀌면 새 파일을 추가하지 말고 이 README를 교체/갱신한다.
- 최종 source of truth는 이 README가 아니라 repo canonical docs다.

## 현재 목표

`wuxia_jianghu_pack` storypack preview에 여섯 번째 runtime encounter `wuxia_cheongryu_raid_route_split`를 구현한다.

목적:

- 이미 구현된 `wuxia_seo_harin_rescue`와 `wuxia_cheongryu_apprentice_entry` 이후의 청류문 공통 루트를 이어받는다.
- `cheongryu_apprentice_entry_resolved` / `cheongryu_trial_started`와 `cheonggi_record_awakened` / `first_fragment_seen` hook을 요구한다.
- 청류문 습격을 통해 정파/사파/천기·귀환 route pressure를 처음 노출한다.
- route graph, faction reputation, companion death, boss combat, reward/ability schema는 열지 않고 기존 encounter schema의 flags/clues/log/presentation만 사용한다.
- 기본 office bundle을 건드리지 않고 storypack preview bundle에만 추가한다.

이 목표를 별도 readiness 세션, future-design 세션, 후일담 세션으로 쪼개지 않는다. 구현 시작 전에 짧게 readiness를 확인하고, blocker가 없으면 그대로 구현한다.

## 추천 skill / reasoning

Hermes skill system을 사용할 수 있으면 먼저 다음 skill을 load한다.

- `workspace-continuation-safety`
- `test-driven-development`
- `tmp-rust-cargo-toolchain`
- `narrative-tui-game-development` 또는 story/content 관련 동등 workflow

Planning/reasoning은 충분히 높게 잡되, 별도 설계 세션을 새로 만들지 않는다.

## Grounding

먼저 repo 상태를 확인한다.

```bash
cd /home/dudupunch0/tui_adv
git fetch --prune origin
git status --short --branch -uall
git log --oneline -1 HEAD
git log --oneline -1 origin/main
git diff --stat HEAD origin/main
```

기준:

- 가능하면 fresh `origin/main` 기반 branch/worktree에서 구현한다.
- 현재 checkout이 squash-merged feature branch이거나 `origin/main`과 다르면, 구현은 fresh branch/worktree from `origin/main`에서 시작한다.
- 이미 사용자가 구현 branch를 지정했다면 그 branch를 쓰되, 상태를 보고한다.
- 이 host의 `/home`은 작다. build/test/install/run 명령 전에는 `/home/dudupunch0/.config/tui_adv/tmp-installs.sh`를 source하거나 동등한 tmp cache 설정을 사용한다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - 0.17 `wuxia_cheongryu_raid_route_split`
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

Repo canonical docs가 이 README와 충돌하면 repo docs를 우선하고, 충돌 사실을 보고한다. Notion 원문을 runtime spec으로 직접 쓰지 않는다.

## 짧은 readiness check

구현 전에 다음만 확인한다. blocker가 없으면 별도 보고서만 쓰고 멈추지 말고 구현을 진행한다.

1. `wuxia_cheongryu_apprentice_entry`가 같은 preview source/generated bundle에 구현되어 있는지 확인한다.
2. 모든 apprentice outcome이 최소 `cheongryu_apprentice_entry_resolved`, `cheongryu_trial_started`, `destination_id: cheongryu_outer_courtyard`를 남기는지 확인한다.
3. `wuxia_cheonggi_record_first_fragment`가 공통 `cheonggi_record_awakened`와 `first_fragment_seen` hook을 남기는지 확인한다.
4. 다음 구현 slice가 `wuxia_cheongryu_raid_route_split`인지 canonical docs와 이 README가 일치하는지 확인한다.
5. blocker가 있으면 raid runtime 구현을 시작하지 말고, 필요한 최소 apprentice/first-fragment 보정 또는 blocker 보고를 먼저 한다.

## 구현 범위

Preview source와 generated preview artifacts에만 `wuxia_cheongryu_raid_route_split`를 추가한다.

Expected runtime behavior:

- 시작 위치: `cheongryu_outer_courtyard`.
- phase: `cheongryu_raid` / `route_commitment`.
- `runtime_mode: storypack_preview` boundary 유지.
- 기존 preview route `wuxia_commute_rift_arrival` → `wuxia_heuksa_bang_first_fight` → `wuxia_cheonggi_record_first_fragment` → `wuxia_seo_harin_rescue` → `wuxia_cheongryu_apprentice_entry` 뒤에 붙는다.
- default office bundle에는 무협 id가 계속 없어야 한다.

## 예상 수정 파일

Preview source:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `wuxia_cheongryu_apprentice_entry` 뒤에 `wuxia_cheongryu_raid_route_split` 추가.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - 가능하면 `cheongryu_outer_courtyard`를 재사용한다.
  - 새 장소가 꼭 필요하면 `cheongryu_raid_courtyard` 또는 `raid_aftermath_shelter`를 preview-only로 추가한다.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`
  - 첫 raid slice에서는 보통 수정하지 않는다. route token/item이 필요하면 preview-only item으로 제한한다.

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
- docs sync only if implementation changes docs-visible status/counts.

## Required / forbidden conditions

Recommended encounter conditions:

```yaml
runtime_mode: storypack_preview
conditions:
  locations:
    - cheongryu_outer_courtyard
required_flags:
  - cheongryu_apprentice_entry_resolved
  - cheongryu_trial_started
  - cheonggi_record_awakened
  - first_fragment_seen
forbidden_flags:
  - cheongryu_raid_route_split_resolved
```

Branch-specific first-fragment flags such as `cheonggi_fragment_guard_basics_thread` are flavor only, not required eligibility.

## Stable choice ids

Use these stable choice/action ids unless existing runtime conventions require a mechanical variant:

- `evacuate_the_wounded_first` — fallback/safe human priority. route commitment를 지연하지만 부상자/청류문 생존 hook을 남긴다.
- `defend_cheongryu_with_white_path` — righteous route pressure. 백도맹 지원을 받아 청류문을 지키지만 정치적 빚을 남긴다.
- `trade_with_black_heaven` — sapa survival bargain. 흑천련과 거래해 생존 자원을 얻지만 신뢰/채무 hook을 남긴다.
- `follow_heavenly_archive` — cheonggi/return truth pressure. 천기각 기록관을 따라 천기록/귀환 단서를 쫓지만 청류문 관계 위험을 남긴다.

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

Every raid outcome should leave the common bridge hooks needed by later slices:

- `cheongryu_raid_route_split_resolved`
- `cheongryu_raid_survived`
- `route_commitment_pressure`
- `destination_id: cheongryu_outer_courtyard` or a preview-only aftermath location

Branch-specific flags/clues/logs may include:

- `wounded_saved_flag`
- `route_commitment_deferred`
- `seo_harin_survived_raid`
- `righteous_route_started`
- `baekdo_alliance_debt`
- `sapa_route_started`
- `black_heaven_deal_marked`
- `dowol_debt`
- `cheonggi_return_route_started`
- `heavenly_archive_contact`
- `cheonggi_record_targeted`
- `martial_knowledge_conflict`
- `blood_moon_targets_cheonggi_record`
- `white_path_help_has_price`
- `black_heaven_bargain_has_teeth`
- `heavenly_archive_knows_previous_outsiders`
- `saving_people_delays_route_choice`

Tone: 어느 편도 완전히 선하거나 안전하지 않고, 선택하지 않는 것도 대가가 있다.

## 금지 범위

이 slice에서 열지 않는다.

- 새 `FactionStanding`
- `RouteGraph`
- `BranchLock`
- `CompanionDeath`
- `MassCombat`
- boss combat resolver
- combat/reward/ability schema
- 천외편린 3택 lock-in UI
- `wuxia_cheongryu_raid_wounded_fallback` 구현
- route opener / multi-ending 구현
- 서하린 companion/emotional-axis 사건 구현
- 서하린 후일담 구현
- yageunmong runtime
- 기본 office bundle 변경
- `escape-office` save/localStorage key 변경
- default office generated bundle에 무협 id 추가

## 구현 순서

1. Repo 상태와 canonical docs를 읽고, 현재 apprentice/first-fragment implementation이 readiness 조건을 만족하는지 확인한다.
2. 실패하는 targeted test를 먼저 추가/수정한다.
   - preview bundle에 `wuxia_cheongryu_raid_route_split`가 포함되어야 한다.
   - default office bundle에는 무협 id가 없어야 한다.
   - Rust/Web/terminal/WASM parity에서 raid encounter/action id가 보여야 한다.
3. preview source YAML에 raid route split encounter를 최소 추가한다.
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
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_cheongryu_raid_route_split_through_preview_bundle
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_cheongryu_raid_route_split
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
`wuxia_cheongryu_raid_route_split` 구현 완료.

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
- location:
- required/forbidden flags:
- choice ids:
- common outcome hooks:
- default office bundle 보호 결과:

4. 검증
- <command>: <pass/fail/skip + 이유>

5. 남은 일 / caveat
-
```
