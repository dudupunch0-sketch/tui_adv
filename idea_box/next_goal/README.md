---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-01
current_goal: implement_wuxia_black_heaven_escape_price
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

`wuxia_baekdo_medicine_debt` preview runtime 구현은 완료됐고, `route_opener_followup_after_baekdo` docs-only handoff도 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime preview implementation**이다.

- `wuxia_black_heaven_escape_price`를 같은 `wuxia_jianghu_pack` storypack preview source에 구현한다.
- 사파/흑천련 거래 opener를 `sapa_route_started` + `dowol_debt` 조건으로 열고, `black_heaven_deal_marked`와 `black_heaven_escape_marker`는 direct/deferred flavor hook으로만 읽는다.
- stable choice ids는 `accept_dowol_marker_for_safehouse`, `ask_who_collects_the_price`, `keep_cheongryu_names_off_ledger`, `map_exit_before_following_dowol`이다.
- 모든 outcome은 `black_heaven_escape_price_resolved`, `sapa_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남겨야 한다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema는 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.0b 2026-06-01 default storypack 전환`
  - `## 0.21 2026-06-01 무협 wuxia_baekdo_medicine_debt preview runtime slice`
  - `## 0.22 2026-06-01 docs-only route opener follow-up handoff: wuxia_black_heaven_escape_price`
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

## handoff 방향

구현할 후보:

- id: `wuxia_black_heaven_escape_price`
- role: 사파/흑천련 opener, 흑천련 거래 opener
- required start conditions: `[sapa_route_started, dowol_debt]`
- forbidden flags: `[black_heaven_escape_price_resolved]`
- flavor-only flags: `[black_heaven_deal_marked, black_heaven_escape_marker]`
- presentation: `visual_id: wuxia_black_heaven_escape_price`, `speaker: 도월`, `layout: sapa_route_opener`, stable terms `탈출로 / 흑천련 / 값`
- common bridge: `black_heaven_escape_price_resolved`, `sapa_route_opened`, `destination_id: cheongryu_outer_courtyard`
- outcome flavor hooks: `black_heaven_safehouse_marked`, `dowol_terms_questioned`, `cheongryu_names_kept_off_ledger`, `market_route_debt_recorded`, `sapa_survival_principle_seen`
- clues: `black_heaven_help_marks_debt`, `black_heaven_bargain_has_teeth`, `survival_bargain_is_not_loyalty`, `sapa_can_save_without_mercy`, `ledger_can_be_bent_not_broken`

구현 기준:

- 기존 flags/clues/log/presentation만으로 구현한다.
- 청류문 내부 배신/고구마가 아니라 외부 압박과 결핍, 흑천련 거래를 갈등 원천으로 유지한다.
- 천기록 정체 reveal, 천외편린 3택 reward/ability schema, route graph/faction reputation/debt ledger를 열지 않는다.

## 예상 수정 파일

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `docs/dev/Notion_Design_Coverage.md` if Notion comparison needs a ledger update
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `tests/test_web_data_export.py`
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- 이 `idea_box/next_goal/README.md`

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo fmt --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_black_heaven_escape_price_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_black_heaven_escape_price
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
