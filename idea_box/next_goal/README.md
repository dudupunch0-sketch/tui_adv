---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_midgame_reunion
mode: implementation
---

# next_goal

이 폴더는 다른 Hermes/agent 세션에 넘길 단일 prompt entry point다. 새 세션에는 긴 프롬프트를 복사하지 말고 아래처럼 짧게 지시한다.

```text
이 repo의 idea_box/next_goal/ 폴더를 읽고 README의 현재 목표만 수행해. repo canonical docs와 충돌하면 canonical docs를 우선하고 충돌 사실을 보고해.
```

운영 원칙:

- 이 README는 “지금 다음으로 할 일” 하나만 가리킨다.
- 목표가 바뀌면 새 파일을 추가하지 말고 이 README를 교체/갱신한다.
- 최종 source of truth는 이 README가 아니라 repo canonical docs다.

## 현재 목표

`wuxia_mumyeong_followup_after_orthodox_style_trace` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **`wuxia_mumyeong_midgame_reunion` runtime implementation**이다.

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_reads_orthodox_style` 뒤에 `wuxia_mumyeong_midgame_reunion`을 추가한다.
- Rust/Web storypack preview generated bundle만 갱신한다.
- docs/storypack DB와 runtime preview 문서의 "selected for next runtime" 상태를 "implemented" 상태로 갱신한다.
- legacy office `content.bundle.json`, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 보류 후보는 `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`다.
- seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation schema, epilogue/return schema, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.36`: docs-only orthodox-style follow-up handoff for `wuxia_mumyeong_midgame_reunion`
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

## 구현 기준

Start condition:

- `runtime_mode: storypack_preview`
- `conditions.locations: [cheongryu_outer_courtyard]`
- `required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]`
- `forbidden_flags: [mumyeong_midgame_reunion_resolved]`
- flavor-only flags: `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete`, `seo_harin_mumyeong_silence_confirmed`, `copied_flow_weakness_noted`, `copy_style_hint_recorded`

Stable choice ids:

- `ask_why_seoharin_never_called_him_traitor`
- `show_the_hyeonakmun_trace_without_accusing`
- `point_out_the_copied_form_gap`
- `keep_blades_low_and_watch_his_answer`

Common hook:

- 모든 선택지는 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard`를 남긴다.
- primary clues: `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation`

Presentation:

- `visual_id: wuxia_mumyeong_midgame_reunion`
- `speaker: 무명`
- `layout: rival_reunion_trace`
- stable terms: `[무명, 서하린, 현악문]`

## 이미 완료된 기반

- `wuxia_mumyeong_first_sighting`가 preview runtime에 구현되어 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `mumyeong_exists`, `copied_flow_is_not_qingliu` hook을 남긴다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.
- `wuxia_mumyeong_copy_style_reveal`가 preview runtime에 구현되어 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남긴다.
- `wuxia_mumyeong_reads_orthodox_style`가 preview runtime에 구현되어 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook을 남긴다.
- Web default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.

## 검증 명령

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
cargo test -p escape-core content_bundle
cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_mumyeong_midgame_reunion
python3 scripts/export_web_data.py --check --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
git diff --check
```
