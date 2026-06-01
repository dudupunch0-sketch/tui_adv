---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_boss_first_appearance
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

`wuxia_mumyeong_midgame_reunion` runtime implementation과 `wuxia_mumyeong_followup_after_midgame_reunion` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **`wuxia_boss_first_appearance` runtime implementation**이다.

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_midgame_reunion` 뒤에 `wuxia_boss_first_appearance` encounter를 추가한다.
- Rust/Web storypack preview generated bundle만 재생성한다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 이 장면은 보스 전투나 최종 결산이 아니다. 압도감, 조직력, 약점 읽기, 무명이 따르는 이유를 flags/clues/log/presentation으로만 각인한다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.38`: docs-only midgame-reunion follow-up handoff
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

## 이미 완료된 기반

- `wuxia_mumyeong_first_sighting`가 preview runtime에 구현되어 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `mumyeong_exists`, `copied_flow_is_not_qingliu` hook을 남긴다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.
- `wuxia_mumyeong_copy_style_reveal`가 preview runtime에 구현되어 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남긴다.
- `wuxia_mumyeong_reads_orthodox_style`가 preview runtime에 구현되어 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook을 남긴다.
- `wuxia_mumyeong_midgame_reunion`가 preview runtime에 구현되어 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook을 남긴다.
- Web default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.

## 구현 계약

- encounter id: `wuxia_boss_first_appearance`
- insert after: `wuxia_mumyeong_midgame_reunion`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `mumyeong_midgame_reunion_resolved`
  - `mumyeong_mirror_thread_deepened`
  - `cheongryu_raid_survived`
  - `midgame_continuity_started`
- forbidden flags:
  - `boss_first_appearance_resolved`
- flavor-only flags/clues:
  - `boss_used_mumyeongs_wound`
  - `hyeonakmun_trace_shared_without_accusation`
  - `seoharin_does_not_call_mumyeong_traitor`
  - `rival_mirror_relationship_deepened`
  - `reunion_truth_deferred`
- stable choice ids:
  - `read_the_boss_flow_and_fail_to_move`
  - `pull_seo_harin_behind_broken_gate`
  - `watch_mumyeong_answer_the_boss`
  - `retreat_before_the_second_step`
- common outcome hook:
  - `boss_first_appearance_resolved`
  - `boss_wall_thread_opened`
  - `black_serpent_core_pressure_opened`
  - `destination_id: cheongryu_outer_courtyard`
- primary clues:
  - `boss_reads_people_not_forms`
  - `boss_is_final_logic_wall`
  - `mumyeong_follows_power_that_saw_his_wound`
  - `qingliu_cannot_outmuscle_boss_yet`
- presentation:
  - `visual_id: wuxia_boss_first_appearance`
  - `speaker: 흑사방주`
  - `layout: boss_wall_pressure`
  - stable terms: `[흑사방주, 무명, 청류문]`

## 명시적 non-goals

- boss combat/final boss resolution
- `wuxia_mumyeong_departure_truth_summary`
- `wuxia_qingliu_attack_after_war` full flashback
- `wuxia_mumyeong_request_for_aid` backstory bridge
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal
- legacy office bundle/default bundle 변경

## 검증 명령

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
cargo test -p escape-core content_bundle
cargo test -p escape-wasm json_boundary_reaches_wuxia_boss_first_appearance_through_preview_bundle
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_boss_first_appearance
cd web && npm test -- --run src/core/contentBundles.test.ts
PYTHONPATH=src python3 scripts/export_web_data.py --check --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
cargo fmt --check
git diff --check
```
