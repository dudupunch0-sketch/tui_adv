---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_qingliu_attack_after_war_followup
mode: docs-only-handoff
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

`wuxia_mumyeong_awakening` runtime implementation은 완료됐다. `wuxia_qingliu_attack_after_war` runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다. `wuxia_mumyeong_followup_after_awakening` docs-only handoff도 완료됐고, 선택된 `wuxia_qingliu_attack_after_war`는 full flashback 없이 현악문/복호금쇄수 흔적 조사로 preview runtime에 landing했다.

이번 세션의 다음 목표는 **`wuxia_qingliu_attack_after_war_followup` docs-only handoff**다.

- Notion 사건 카드 DB와 repo hooks를 다시 대조해 `wuxia_qingliu_attack_after_war` 다음 후보를 고른다.
- 최소 후보군: `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, 필요 시 `wuxia_seoharin_empty_place`.
- 다음 runtime 후보를 고르더라도 이번 docs-only handoff에서는 YAML/Rust/Web generated artifact를 변경하지 않는다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.43`: `wuxia_mumyeong_awakening` preview runtime slice
  - section `0.44`: `wuxia_qingliu_attack_after_war` docs-only handoff
  - section `0.45`: `wuxia_qingliu_attack_after_war` preview runtime slice
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`

## 이미 완료된 기반

- Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.
- `wuxia_mumyeong_copy_style_reveal`가 preview runtime에 구현되어 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남긴다.
- `wuxia_mumyeong_reads_orthodox_style`가 preview runtime에 구현되어 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook을 남긴다.
- `wuxia_mumyeong_midgame_reunion`가 preview runtime에 구현되어 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook을 남긴다.
- `wuxia_boss_first_appearance`가 preview runtime에 구현되어 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook을 남긴다.
- `wuxia_mumyeong_request_for_aid`가 preview runtime에 구현되어 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `rejected_aid_letters_read`, `inn_rumor_thread_followed`, `seoharin_failed_aid_question_asked`, `failed_aid_record_kept_unshown` hook을 남긴다.
- `wuxia_mumyeong_awakening`가 preview runtime에 구현되어 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened` hook을 남긴다.
- `wuxia_mumyeong_awakening` primary clues는 `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready`다.
- `wuxia_mumyeong_awakening` stable choice id는 `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation`다.
- `wuxia_qingliu_attack_after_war`가 preview runtime에 구현되어 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened` hook을 남긴다.
- `wuxia_qingliu_attack_after_war` primary clues는 `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`다.
- `wuxia_qingliu_attack_after_war` stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`다.
- Historical handoffs include `wuxia_mumyeong_first_confrontation_after_sighting`, `wuxia_mumyeong_followup_after_first_confrontation`, `wuxia_mumyeong_followup_after_copy_style_reveal`, `wuxia_mumyeong_followup_after_orthodox_style_trace`, `wuxia_mumyeong_followup_after_midgame_reunion`, `wuxia_boss_followup_after_first_appearance`, `wuxia_mumyeong_followup_after_failed_aid`, and `wuxia_mumyeong_followup_after_awakening`.

## 완료된 구현 계약

- encounter id: `wuxia_qingliu_attack_after_war`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_awakening`
- location: `cheongryu_outer_courtyard`
- presentation: `visual_id: wuxia_qingliu_attack_after_war`, `speaker: 천기록`, `layout: attack_trace_investigation`, stable terms `[청류문, 현악문, 복호금쇄수]`

required flags:

- `mumyeong_awakening_resolved`
- `mumyeong_awakening_thread_opened`
- `copy_corruption_thread_opened`
- `mumyeong_request_for_aid_resolved`
- `mumyeong_failed_aid_thread_opened`
- `orthodox_hypocrisy_thread_opened`
- `mumyeong_reads_orthodox_style_resolved`
- `orthodox_style_trace_recorded`
- `midgame_continuity_started`

forbidden flags:

- `qingliu_attack_after_war_resolved`

flavor-only flags/clues:

- `hyeonakmun_trace_suspected`
- `bokho_geumsaesu_name_recorded`
- `mumyeong_tried_to_save_qingliu`
- `orthodox_refusal_broke_mumyeong`
- `seoharin_does_not_know_failed_aid`
- `salvation_truth_still_unready`
- `awakening_points_to_hyeonakmun_without_full_truth`

stable choice ids:

- `inspect_bokho_lock_scars`
- `compare_hyeonakmun_trace_to_qingliu_wounds`
- `ask_seo_harin_what_she_saw_afterward`
- `stop_before_replaying_the_attack`

common outcome hook:

- flags: `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`
- clues: `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`
- `destination_id: cheongryu_outer_courtyard`

## 구현 범위

완료한 것:

- 무명 각성 이후 청류문 외원에 남은 상처를 조사하는 encounter를 만든다.
- 현악문/복호금쇄수 단서를 강화한다.
- 서하린은 습격의 중심을 본 사람이 아니라 aftermath를 본 사람으로만 다룬다.
- 천기록은 장면을 재생하지 않고 흔적만 적는다.

하지 말 것:

- 청류문 습격 full flashback/backstory reveal
- `wuxia_mumyeong_destroys_orthodox_sect` runtime 구현
- `wuxia_boss_recruits_mumyeong` runtime 구현
- `wuxia_mumyeong_departure_truth_summary` runtime 구현
- `wuxia_mumyeong_resolution` runtime 구현
- `wuxia_boss_resolution` runtime 구현
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle 변경

## 다음 handoff 범위

해야 할 것:

- 청류문 습격 흔적 조사 이후 가장 작은 다음 runtime 후보를 고른다.
- 후보를 고를 때 full flashback, 무명 구원 확정, 서하린 진실 전달, 보스 최종 결산을 너무 일찍 열지 않는지 확인한다.
- 다음 후보의 required/forbidden flags, stable choice ids, common hooks, non-goals를 문서화한다.
- `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md`, `docs/dev/Notion_Design_Coverage.md`, `docs/content/storypack_db/*`, `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, `idea_box/notion_sources.yml`를 docs-only로 동기화한다.

하지 말 것:

- runtime YAML/Rust/Web generated bundle 변경
- full Qingliu attack flashback/backstory reveal
- 정파 문파 멸문, 보스 스카웃, 무명 이탈 진실 전체, final boss resolution을 바로 구현
- 새 combat/reward/route graph/faction/relation/debt/epilogue/return schema 열기

## 검증 명령

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core preview_fixture_indexes_wuxia_first_fight
cargo test -p escape-wasm json_boundary_reaches_wuxia_qingliu_attack_after_war_through_preview_bundle
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_qingliu_attack_after_war
cd web && npm test -- --run src/core/contentBundles.test.ts
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
