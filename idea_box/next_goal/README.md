---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_request_for_aid
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

`wuxia_boss_first_appearance` runtime implementation과 `wuxia_boss_followup_after_first_appearance` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **`wuxia_mumyeong_request_for_aid` runtime implementation**이다.

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_first_appearance` 뒤에 새 encounter를 추가한다.
- Rust/Web storypack preview generated bundle만 재생성한다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 바로 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 보스 최종 결산, reward schema, combat resolver를 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.40`: `wuxia_mumyeong_request_for_aid` docs-only handoff
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
- `wuxia_boss_first_appearance`가 preview runtime에 구현되어 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook을 남긴다.
- 보스 첫 등장 stable choice id는 `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step`다.
- Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.

## 구현 계약

- encounter id: `wuxia_mumyeong_request_for_aid`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_boss_first_appearance`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `boss_first_appearance_resolved`
  - `boss_wall_thread_opened`
  - `black_serpent_core_pressure_opened`
  - `mumyeong_mirror_thread_deepened`
  - `orthodox_style_trace_recorded`
  - `midgame_continuity_started`
- forbidden flags:
  - `mumyeong_request_for_aid_resolved`
- flavor-only flags/clues:
  - `boss_used_mumyeongs_wound`
  - `hyeonakmun_trace_shared_without_accusation`
  - `mumyeong_follows_power_that_saw_his_wound`
  - `boss_reads_people_not_forms`
  - `boss_is_final_logic_wall`
  - `qingliu_cannot_outmuscle_boss_yet`
  - `seoharin_does_not_call_mumyeong_traitor`
- stable choice ids:
  - `search_the_rejected_aid_letters`
  - `follow_old_inn_rumors_about_mumyeong`
  - `ask_seo_harin_what_help_never_came`
  - `keep_the_failed_aid_record_unshown`
- common outcome hook:
  - `mumyeong_request_for_aid_resolved`
  - `mumyeong_failed_aid_thread_opened`
  - `orthodox_hypocrisy_thread_opened`
  - `destination_id: cheongryu_outer_courtyard`
- primary clues:
  - `mumyeong_tried_to_save_qingliu`
  - `orthodox_refusal_broke_mumyeong`
  - `boss_logic_found_mumyeongs_wound`
  - `aid_refusal_precedes_departure_truth`
  - `seoharin_does_not_know_failed_aid`
- presentation:
  - `visual_id: wuxia_mumyeong_request_for_aid`
  - `speaker: 천기록`
  - `layout: failed_aid_records`
  - stable terms: `[무명, 청류문, 정파]`

## 이전 handoff 결정

`wuxia_boss_followup_after_first_appearance`에서 `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`을 비교했다.

- `wuxia_mumyeong_request_for_aid`: 보스 첫 등장으로 열린 “힘의 논리가 무명에게 왜 설득력 있었나”를 중반 backstory bridge로 준비할 수 있어 다음 runtime 후보로 선택했다.
- `wuxia_mumyeong_departure_truth_summary`: 서하린에게 진실 전달, 무명 구원 조건, 후반 route 조건을 너무 직접 건드려 보류한다.
- `wuxia_qingliu_attack_after_war`: full flashback/backstory reveal이 되어 현재 boss pressure를 덮을 수 있어 보류한다.
- `wuxia_boss_resolution`: final boss resolution/boss combat/epilogue 결산을 너무 일찍 열 수 있어 보류한다.

## 명시적 non-goals

- 기본 office bundle/default bundle 변경
- legacy `escape-office` save/localStorage key 변경
- boss combat/final boss resolution
- `wuxia_mumyeong_departure_truth_summary` runtime 구현
- `wuxia_qingliu_attack_after_war` full flashback runtime 구현
- `wuxia_boss_resolution` runtime 구현
- 무명 구원 확정
- 서하린에게 진실 전달
- seed 기반 random copy-style system/table
- combat resolver/schema 또는 HP 숫자전
- route graph, faction reputation, relation/debt ledger
- reward/ability schema, 천외편린 3택 성장
- epilogue/return system
- 천기록 identity reveal

## 검증 명령

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
