---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: implement_wuxia_wounded_shelter_dawn_offers
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

`route_opener_followup_after_heavenly_archive` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime preview implementation**이다.

- `wuxia_wounded_shelter_dawn_offers`를 구현한다.
- 이 목표는 docs-only handoff에서 선택한 deferred-offer card를 실제 preview runtime으로 승격하는 작업이다.
- 이 카드는 `stabilize_wounded_until_dawn` branch가 남긴 `route_commitment_deferred`, `deferred_route_reopened`, `wounded_shelter_stabilized`를 받아 새벽 피난처 제안으로 다시 메인 흐름에 붙인다.
- docs-only handoff의 start conditions, stable choice ids, outcome hooks, schema non-goals를 따른다.
- runtime source는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`만 수정하고, Rust/Web preview bundle artifact만 재생성한다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.0b 2026-06-01 default storypack 전환`
  - `## 0.21 2026-06-01 무협 wuxia_baekdo_medicine_debt preview runtime slice`
  - `## 0.22 2026-06-01 docs-only route opener follow-up handoff: wuxia_black_heaven_escape_price`
  - `## 0.23 2026-06-02 무협 wuxia_black_heaven_escape_price preview runtime slice`
  - `## 0.24 2026-06-02 docs-only route opener follow-up handoff: wuxia_heavenly_archive_previous_outsiders`
  - `## 0.25 2026-06-02 무협 wuxia_heavenly_archive_previous_outsiders preview runtime slice`
  - `## 0.26 2026-06-02 docs-only route opener follow-up handoff: wuxia_wounded_shelter_dawn_offers`
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/encounter_situations.json`

## 구현 방향

선택된 후보:

- id: `wuxia_wounded_shelter_dawn_offers`
- 목적: `stabilize_wounded_until_dawn` fallback branch가 세 route opener를 직접 타지 않은 경우, 부상자 피난처의 새벽 제안으로 route pressure를 다시 연다.
- start conditions: `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]`, `forbidden_flags: [wounded_shelter_dawn_offers_resolved]`
- flavor flags: `survivor_roll_call_complete`, `route_delay_cost_recorded`
- stable choice ids: `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha`
- common hook: 모든 선택지는 `wounded_shelter_dawn_offers_resolved`, `route_commitment_reopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- route choice hooks: 정파/사파/천기 선택지는 각각 기존 opener가 읽는 starter flags를 남겨 다음 턴에 `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`가 열릴 수 있게 한다.
- fallback hook: `wounded_shelter_until_noon`, `deferred_offer_debt_recorded`를 남기되 같은 카드가 반복되지 않도록 resolved flag를 금지 조건으로 둔다.
- 금지선: triage system, companion death, mass combat, route graph, faction reputation, relation score, debt ledger, reward/ability schema, epilogue schema, return system, 천기록 정체 reveal은 열지 않는다.

## 예상 수정 파일

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `tests/test_web_data_export.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- docs/checklist/next_goal status files after implementation

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_wounded_shelter_dawn_offers_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_wounded_shelter_dawn_offers
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
