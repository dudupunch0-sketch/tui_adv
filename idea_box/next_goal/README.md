---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: implement_wuxia_heavenly_archive_previous_outsiders
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

`wuxia_black_heaven_escape_price` preview runtime 구현과 `route_opener_followup_after_black_heaven` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime preview implementation**이다.

- `wuxia_heavenly_archive_previous_outsiders`를 storypack preview runtime으로 구현한다.
- 천기·귀환 opener는 `cheonggi_return_route_started` + `cheonggi_record_targeted`를 required flags로 쓴다.
- `heavenly_archive_contact`와 `heavenly_archive_triage_map_seen`는 direct/deferred branch flavor hook으로만 남긴다.
- stable choice ids는 `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory`다.
- 모든 outcome은 `heavenly_archive_previous_outsiders_resolved`, `cheonggi_return_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남긴다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.0b 2026-06-01 default storypack 전환`
  - `## 0.21 2026-06-01 무협 wuxia_baekdo_medicine_debt preview runtime slice`
  - `## 0.22 2026-06-01 docs-only route opener follow-up handoff: wuxia_black_heaven_escape_price`
  - `## 0.23 2026-06-02 무협 wuxia_black_heaven_escape_price preview runtime slice`
  - `## 0.24 2026-06-02 docs-only route opener follow-up handoff: wuxia_heavenly_archive_previous_outsiders`
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

구현할 후보:

`wuxia_heavenly_archive_previous_outsiders`

- purpose: 천기·귀환 route starter를 받아 천기각 서고의 이전 이방인 기록과 세계 균열 흔적을 첫 천기 opener로 연다.
- conditions:
  - `locations: [cheongryu_outer_courtyard]`
  - `required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]`
  - `forbidden_flags: [heavenly_archive_previous_outsiders_resolved]`
  - flavor only: `[heavenly_archive_contact, heavenly_archive_triage_map_seen]`
- presentation:
  - `visual_id: wuxia_heavenly_archive_previous_outsiders`
  - `speaker: 연소하`
  - `layout: cheonggi_return_opener`
  - stable terms: `천기각 / 이방인 / 균열`

선택지:

- `read_previous_outsider_margins` — fallback / safe reading
- `ask_yeon_soha_what_not_to_read` — boundary probe
- `mark_current_worldline_without_answer` — no-answer acceptance
- `compare_rift_terms_to_commute_memory` — return clue comparison

outcome hooks:

- common flags: `heavenly_archive_previous_outsiders_resolved`, `cheonggi_return_route_opened`
- branch flags/clues/logs:
  - `previous_outsiders_record_seen`
  - `yeon_soha_warning_heard`
  - `worldline_margin_marked`
  - `commute_rift_terms_compared`
  - `archive_has_other_outsiders`
  - `cheonggi_record_refuses_identity_answer`
  - `return_clue_is_not_return_method`
  - `worldline_gaps_have_patterns`
  - `record_gaze_without_name`

## 예상 수정 파일

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `tests/test_web_data_export.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- 필요 시 docs sync: `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md`, `docs/dev/Storypack_Runtime_Preview_Mode.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, 이 `idea_box/next_goal/README.md`
- 이 `idea_box/next_goal/README.md`

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo fmt --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_heavenly_archive_previous_outsiders_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_heavenly_archive_previous_outsiders
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
