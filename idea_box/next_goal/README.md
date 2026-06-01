---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: implement_wuxia_mumyeong_first_sighting
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

`wuxia_wounded_shelter_dawn_offers` runtime implementation과 `route_midgame_continuity_after_wounded_shelter` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime preview implementation**이다.

- `wuxia_mumyeong_first_sighting`를 preview runtime에 구현한다.
- 세 route opener(`wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`)의 모든 choice outcome에 공통 `route_opener_resolved` flag를 추가한다.
- `wuxia_wounded_shelter_dawn_offers` 뒤에 `wuxia_mumyeong_first_sighting` encounter를 추가하고, required flags는 `route_opener_resolved`, `cheongryu_raid_survived`, `cheongryu_trial_started`, `first_fragment_seen`로 둔다.
- Rust/Web storypack preview bundle만 재생성한다. 기본 office bundle, legacy `escape-office` save/localStorage key, 기본 generated content는 수정하지 않는다.
- 새 any-of condition, route graph, faction reputation/debt ledger/relation/reward/ability/epilogue schema, combat schema, boss first appearance, `wuxia_mumyeong_first_confrontation`, return system, 천기록 정체 reveal은 열지 않는다.

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
  - `## 0.27 2026-06-02 무협 wuxia_wounded_shelter_dawn_offers preview runtime slice`
  - `## 0.28 2026-06-02 docs-only midgame continuity handoff: wuxia_mumyeong_first_sighting`
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

`route_midgame_continuity_after_wounded_shelter` handoff의 결정:

- route별 midgame card 3개는 route tone이 선명하지만, fan-out과 route graph/faction reputation 유혹이 커서 보류했다.
- deferred-offer 후속 bridge는 direct opener branch를 제외하므로 보류했다.
- `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_midgame_reunion`, boss first appearance는 Notion 선행 조건상 후속이다.
- common midgame bridge `wuxia_mumyeong_first_sighting`를 다음 runtime 후보로 골랐다.

구현 계약:

- route opener 세 card의 모든 outcome에 `route_opener_resolved`를 추가한다.
- `wuxia_mumyeong_first_sighting` start condition은 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_sighting_resolved]`다.
- route-specific opened flags(`righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`)는 eligibility가 아니라 flavor hook으로만 둔다.
- stable choice id는 `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form`다.

## 예상 수정 파일

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- `tests/test_web_data_export.py`
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`
- 이 README

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_first_sighting_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_first_sighting
cd web && npm test -- contentBundles.test.ts
git diff --exit-code -- src/tui_adv/storypack-previews/wuxia_jianghu_pack crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
