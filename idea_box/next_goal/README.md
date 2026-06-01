---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_copy_style_reveal
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

`wuxia_mumyeong_followup_after_first_confrontation` docs-only handoff까지 완료됐다. 다음 runtime 후보는 `wuxia_mumyeong_copy_style_reveal`이다.

`wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime implementation**이다.

- `wuxia_mumyeong_copy_style_reveal`를 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 추가한다.
- `wuxia_mumyeong_first_confrontation` 바로 뒤에 배치한다.
- Rust/Web storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return schema, boss first appearance, 무명 중반 재회, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.31`: 무협 `wuxia_mumyeong_first_confrontation` preview runtime slice
  - section `0.32`: docs-only post-confrontation handoff for `wuxia_mumyeong_copy_style_reveal`
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`

## 구현 방향

이미 완료된 기반:

- `wuxia_mumyeong_first_sighting`가 preview runtime에 구현되어 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `mumyeong_exists`, `copied_flow_is_not_qingliu` hook을 남긴다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.
- `wuxia_mumyeong_followup_after_first_confrontation` handoff는 Notion 사건 카드 DB `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`를 비교했고, 첫 구현 후보를 `wuxia_mumyeong_copy_style_reveal`로 골랐다.

`wuxia_mumyeong_copy_style_reveal` runtime boundary:

- start conditions:
  - `conditions.locations: [cheongryu_outer_courtyard]`
  - `required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]`
  - `forbidden_flags: [mumyeong_copy_style_reveal_resolved]`
- flavor-only flags:
  - `copied_flow_weakness_noted`
  - `cheonggi_copy_contrast_noted`
  - `seo_harin_mumyeong_silence_confirmed`
  - `rival_endured_not_defeated`
  - `rivalry_deferred_not_avoided`
  - `righteous_route_opened`
  - `sapa_route_opened`
  - `cheonggi_return_route_opened`
- stable choice ids:
  - `read_the_stolen_blade_path`
  - `watch_mumyeongs_footwork`
  - `listen_for_breath_mismatch`
  - `wait_for_body_to_shudder`
- common outcome hooks:
  - `mumyeong_copy_style_reveal_resolved`
  - `copy_style_hint_recorded`
  - `destination_id: cheongryu_outer_courtyard`
- primary flags/clues:
  - `copied_blade_path_noted`
  - `copied_footwork_noted`
  - `copied_breath_mismatch_noted`
  - `copy_side_effect_seen`
  - `copied_form_family_seen`
  - `copy_is_surface_not_root`
  - `breath_mismatch_marks_copy`
  - `fragment_candidate_variation_foreshadowed`
  - `understanding_is_not_copying`
- presentation:
  - `visual_id: wuxia_mumyeong_copy_style_reveal`
  - `speaker: 서하린`
  - `layout: copy_style_analysis`
  - stable terms `[무명, 청류안, 천기록]`

## 예상 수정 파일

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
- 이 README

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
python3 -m json.tool docs/content/storypack_db/storypacks.json >/tmp/storypacks.json
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/tmp/encounter_situations.json
cargo fmt --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_copy_style_reveal_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_copy_style_reveal
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
