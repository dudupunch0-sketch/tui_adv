---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-01
current_goal: implement_wuxia_baekdo_medicine_debt
mode: runtime-preview-implementation
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

`wuxia_jianghu_pack` storypack preview는 `wuxia_cheongryu_chore_sparring`을 포함해 `wuxia_cheongryu_raid_wounded_fallback`까지 여덟 개 runtime encounter 구현이 끝났다. route opener docs-only handoff도 완료되어 첫 route opener runtime 후보는 정파/백도맹 약상자 채무 축인 `wuxia_baekdo_medicine_debt`로 결정됐다.

이번 세션의 목표는 구현 세션이다.

- `wuxia_baekdo_medicine_debt`를 same storypack preview runtime source에 추가한다.
- runtime YAML/Rust/Web/generated artifact 변경은 storypack preview source와 preview bundle에만 한정한다.
- preview bundle만 재생성한다.
- Python/Rust/WASM/terminal/Web parity tests를 TDD 방식으로 추가/갱신한다.
- 기본 office bundle, default `src/tui_adv/data/*.yaml`, Web default generated bundle, `escape-office` save/localStorage key는 변경하지 않는다.
- route graph/faction reputation/debt ledger/relation/reward/ability schema는 열지 않는다.

## 추천 skill / reasoning

Hermes skill system을 사용할 수 있으면 먼저 다음 skill을 load한다.

- `workspace-continuation-safety`
- `test-driven-development`
- `tmp-rust-cargo-toolchain`
- `narrative-tui-game-development`
- `narrative-tui-game-development` reference `storypack-preview-runtime-slice`

## Grounding

먼저 repo 상태를 확인한다.

```bash
cd /home/dudupunch0/tui_adv
git fetch --prune origin
git status --short --branch -uall
git log --oneline -1 HEAD
git log --oneline -1 origin/main
git rev-list --left-right --count HEAD...origin/main
git diff --stat
```

주의:

- 현재 branch가 origin/main보다 뒤처져 있거나 upstream이 없을 수 있다. dirty worktree를 무리하게 rebase/reset하지 말고, 먼저 현재 diff와 canonical docs를 확인한다.
- 이 host의 `/home`은 작다. build/test/install/run 명령 전에는 `/home/dudupunch0/.config/tui_adv/tmp-installs.sh`를 source하거나 동등한 tmp cache 설정을 사용한다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.20 2026-06-01 docs-only route opener handoff: wuxia_baekdo_medicine_debt`
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
  - `## 8. wuxia_baekdo_medicine_debt`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/encounter_situations.json`
- 현재 preview source:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`

## 구현 handoff

새 encounter:

```yaml
id: wuxia_baekdo_medicine_debt
storypack_id: wuxia_jianghu_pack
runtime_mode: storypack_preview
location: cheongryu_outer_courtyard
required_flags: [righteous_route_started, cheongryu_rebuild_thread]
forbidden_flags: [baekdo_medicine_debt_resolved]
flavor_flags_only: [baekdo_alliance_debt, baekdo_medicine_debt]
```

설계 의도:

- direct route branch와 deferred wounded fallback branch를 둘 다 받는다.
- direct route의 `baekdo_alliance_debt`와 deferred route의 `baekdo_medicine_debt`는 eligibility 필수가 아니라 branch flavor hook이다.
- 정파의 도움은 사람을 살리지만, 백도맹 질서와 정치적 채무가 대가로 남는다는 첫 opener다.
- 청류문 내부 악인/배신자/정치질이 아니라 결핍과 외부 질서가 갈등 원천이다.

Stable choice ids:

```yaml
choices:
  - id: accept_medicine_with_written_debt
    role: fallback / safe acceptance
  - id: ask_terms_before_opening_gate
    role: negotiation probe
  - id: send_supplies_to_wounded_first
    role: homebase alignment
  - id: compare_banner_to_record_margin
    role: cheonggi observation without identity reveal
```

권장 common/branch outcome hooks:

```yaml
common:
  add_flags: [baekdo_medicine_debt_resolved, righteous_route_opened]
  destination_id: cheongryu_outer_courtyard
branch_flags:
  - white_path_debt_recorded
  - cheongryu_rebuild_supplies_secured
  - baekdo_terms_questioned
  - namgung_seoyun_notice
  - cheongryu_people_first
  - seo_harin_respect_thread
  - cheonggi_record_notes_baekdo_debt
branch_clues:
  - medicine_has_banner
  - white_path_help_has_price
  - order_can_save_and_bind
  - qingliu_survival_needs_outside_help
  - record_counts_debt_not_justice
```

Presentation 권장:

```yaml
presentation:
  visual_id: wuxia_baekdo_medicine_debt
  speaker: 남궁서윤
  layout: righteous_route_opener
  effect_cues:
    - kind: glyph_anomaly
      source: baekdo_medicine_debt
      stable_terms: [약상자, 백도맹, 채무]
```

천기록 관련 금지:

- 천기록 정체 reveal 금지.
- `compare_banner_to_record_margin`는 “정의와 채무가 같이 기록된다”는 감지만 남긴다.
- 검색창처럼 질문/답변 기능을 만들지 않는다.
- 천외편린 3택 reward/ability schema를 열지 않는다.

## 예상 수정 파일

Runtime/source:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`

Generated preview artifacts only:

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

Tests:

- `tests/test_web_data_export.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- `tests/test_docs_contract.py` only if docs wording changes

Docs sync after implementation:

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- 이 `idea_box/next_goal/README.md`

## 금지 범위

절대 하지 않는다.

- `src/tui_adv/data/*.yaml` 수정
- 기본 office `content.bundle.json` 수정
- Web 기본 generated `content.bundle.json` 수정
- `escape-office` save/localStorage key 수정
- sapa/cheonggi/return route opener 동시 구현
- deferred-offer card 동시 구현
- route graph/faction reputation schema
- relation/debt ledger schema
- triage system, companion death, mass combat, boss combat resolver
- 천외편린/각성편린 3택 reward/ability schema
- Notion DB 내용을 곧바로 구현 완료처럼 표시
- 천기록 정체 reveal

## TDD 순서

1. RED tests 추가:
   - Python exporter/Web generated data test가 preview bundle에 `wuxia_baekdo_medicine_debt`와 choice ids를 요구하도록 한다.
   - Rust content fixture test가 conditions/required flags/fallback choice/outcome hooks/presentation을 요구하도록 한다.
   - WASM JSON boundary test가 righteous route branch 또는 delayed righteous branch 뒤 `wuxia_baekdo_medicine_debt` scene/action ids까지 도달하도록 한다.
   - terminal smoke test가 title/action ids/stable terms를 렌더링하도록 한다.
   - Web content bundle test가 encounter id/count/order를 갱신하도록 한다.
2. RED 실패를 확인한다.
3. preview source에 encounter를 추가한다.
4. preview bundle만 재생성한다.
5. targeted tests를 GREEN으로 만든다.
6. docs/checklist/next_goal을 구현 완료와 다음 slice 기준으로 동기화한다.
7. full-ish 검증과 diff hygiene를 실행한다.

## 검증 명령

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && python3 scripts/export_web_data.py --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json --check
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable fmt --check
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-core --test content_bundle
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_baekdo_medicine_debt_through_preview_bundle
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh && cargo +stable test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_baekdo_medicine_debt
```

Web dependency가 필요하면 `/tmp` scratch copy에서 실행한다.

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
SCRATCH="/tmp/$USER-tui-adv-web-baekdo-verify"
rm -rf "$SCRATCH"
mkdir -p "$SCRATCH"
cp -a web/. "$SCRATCH"/
cd "$SCRATCH"
npm ci
npm test -- --run src/core/contentBundles.test.ts
rm -rf "$SCRATCH"
```

마지막:

```bash
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
git diff --stat
```

## 완료 보고 형식

한국어로 간결하게 보고한다.

```text
wuxia_baekdo_medicine_debt preview runtime 구현 완료.

1. 구현
- encounter:
- choices:
- conditions:
- outcome hooks:

2. 변경 파일
- source:
- generated:
- tests:
- docs/next_goal:

3. 보호한 범위
- default office bundle:
- escape-office key:
- new schema:

4. 검증
- <command>: <pass/fail/skip + 이유>

5. 다음 작업
-
```
