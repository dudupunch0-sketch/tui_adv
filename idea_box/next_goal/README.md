---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_boss_followup_after_first_appearance
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

`wuxia_boss_first_appearance` runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **`wuxia_boss_followup_after_first_appearance` docs-only handoff**다.

- Notion 사건 카드 DB와 repo canonical docs를 다시 대조해 보스 첫 등장 이후 다음 runtime 후보를 결정한다.
- 이번 handoff에서는 runtime YAML, Rust/Web generated preview bundle, Web default bundle, legacy office bundle을 수정하지 않는다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 바로 boss combat/final resolution, 무명 truth reveal, full flashback, reward schema를 열지 않는다. 먼저 어떤 사건이 기존 flags/clues/log/presentation만으로 안전하게 이어지는지 문서에서 결정한다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.39`: `wuxia_boss_first_appearance` runtime implementation
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
- Web default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.

## 보스 첫 등장 구현 계약

- encounter id: `wuxia_boss_first_appearance`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_midgame_reunion`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `mumyeong_midgame_reunion_resolved`
  - `mumyeong_mirror_thread_deepened`
  - `cheongryu_raid_survived`
  - `midgame_continuity_started`
- forbidden flags:
  - `boss_first_appearance_resolved`
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
- presentation:
  - `visual_id: wuxia_boss_first_appearance`
  - `speaker: 흑사방주`
  - `layout: boss_wall_pressure`
  - stable terms: `[흑사방주, 무명, 청류문]`

## 비교할 후보

- `wuxia_mumyeong_request_for_aid`
  - 장점: 무명이 청류문을 살리려 했다는 backstory bridge로, 보스가 무명의 상처를 읽고 이용했다는 단서와 잘 붙는다.
  - 위험: `wuxia_mumyeong_departure_truth_summary`와 가까워져 후반 truth reveal을 앞당길 수 있다.
- `wuxia_mumyeong_departure_truth_summary`
  - 장점: 무명 이탈 진실과 서하린/무명 관계를 정리한다.
  - 위험: 서하린에게 진실 전달, 무명 구원 확정, 후반 route 조건을 너무 직접 건드린다.
- `wuxia_qingliu_attack_after_war`
  - 장점: 현악문/복호금쇄수와 청류문 붕괴 원인을 full source로 보여 줄 수 있다.
  - 위험: full flashback/backstory reveal이 되어 현재 boss pressure를 덮을 수 있다.
- `wuxia_boss_resolution`
  - 장점: 보스가 대표하는 최종 논리와 결산 방향을 확인할 수 있다.
  - 위험: final boss resolution/boss combat을 너무 일찍 열 가능성이 높다.

## handoff 산출물

- `docs/dev/Development_Plan.md`에 새 docs-only handoff section을 추가한다.
- `docs/dev/Checklist.md`에 해당 handoff 완료 항목을 추가한다.
- `docs/dev/Notion_Design_Coverage.md`, `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, storypack DB JSON mirror, `docs/design/Storypack_World_Model.md`, `docs/design/Storypack_Encounter_DB.md`를 다음 후보 기준으로 동기화한다.
- 이 README를 다음 목표로 교체한다.
- 필요한 경우 `tests/test_docs_contract.py`와 `tests/test_storypack_db.py`를 docs-only handoff 기준으로 갱신한다.

## 명시적 non-goals

- runtime YAML/Rust/Web generated preview bundle 수정
- boss combat/final boss resolution
- `wuxia_mumyeong_departure_truth_summary` runtime 구현
- `wuxia_qingliu_attack_after_war` full flashback runtime 구현
- `wuxia_mumyeong_request_for_aid` runtime 구현
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
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
