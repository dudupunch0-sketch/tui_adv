---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_followup_after_awakening
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

`wuxia_mumyeong_awakening` runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **`wuxia_mumyeong_followup_after_awakening` docs-only handoff**다.

- Notion 사건 카드 DB와 repo runtime hooks를 다시 대조해 무명 각성 이후 다음 runtime 후보를 하나 고른다.
- 이번 handoff에서는 runtime YAML, Rust/Web generated preview bundle, Web default bundle, legacy office bundle을 수정하지 않는다.
- 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 바로 정파 문파 멸문, 보스 스카웃, 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 보스 최종 결산, reward schema, combat resolver를 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.42`: `wuxia_mumyeong_awakening` docs-only handoff
  - section `0.43`: `wuxia_mumyeong_awakening` preview runtime slice
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

## 직전 구현 계약

- encounter id: `wuxia_mumyeong_awakening`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_request_for_aid`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `mumyeong_request_for_aid_resolved`
  - `mumyeong_failed_aid_thread_opened`
  - `orthodox_hypocrisy_thread_opened`
  - `mumyeong_reads_orthodox_style_resolved`
  - `orthodox_style_trace_recorded`
  - `mumyeong_copy_style_reveal_resolved`
  - `copy_style_hint_recorded`
  - `midgame_continuity_started`
- forbidden flags:
  - `mumyeong_awakening_resolved`
- stable choice ids:
  - `compare_anger_to_copied_flow`
  - `trace_awakening_from_failed_aid`
  - `ask_what_the_copy_cost_him`
  - `stop_before_calling_it_salvation`
- common outcome hook:
  - `mumyeong_awakening_resolved`
  - `mumyeong_awakening_thread_opened`
  - `copy_corruption_thread_opened`
  - `destination_id: cheongryu_outer_courtyard`
- presentation:
  - `visual_id: wuxia_mumyeong_awakening`
  - `speaker: 천기록`
  - `layout: anger_copy_bloom`
  - stable terms: `[무명, 카피, 분노]`

## 다음 handoff 후보 set

다음 handoff에서는 최소 아래 후보를 비교한다.

- `wuxia_mumyeong_destroys_orthodox_sect`: 무명 각성 이후 결과로 자연스럽지만, 정파 문파 멸문과 무명 이탈 이유를 크게 확정할 위험이 있다.
- `wuxia_boss_recruits_mumyeong`: 보스가 무명의 상처를 이용하는 후반 스카웃 사건이지만, `wuxia_mumyeong_destroys_orthodox_sect` 이후가 선행 조건이다.
- `wuxia_mumyeong_departure_truth_summary`: 서하린에게 진실 전달, 무명 구원 조건, 후반 route 조건을 너무 직접 건드릴 위험이 있다.
- `wuxia_qingliu_attack_after_war`: full flashback/backstory reveal이 되어 현재 중반 pressure를 덮을 위험이 있다.
- `wuxia_mumyeong_resolution`: 최종 route/결산 범위라 아직 이르다.

필요하면 Notion 사건 카드 DB에서 `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_request_for_aid` 관련 후속 reference도 future source로 확인하되, 이번 handoff의 후보로 격상할지는 canonical docs와 대조해 결정한다.

## 이전 handoff 결정

`wuxia_mumyeong_followup_after_failed_aid`에서 `wuxia_mumyeong_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`를 비교했다.

- `wuxia_mumyeong_awakening`: 도움 요청 실패 기록과 정파 무공 흔적을 무명의 분노/카피 무공 변질로 이을 수 있어 runtime 후보로 선택했고, 이제 구현 완료됐다.
- `wuxia_mumyeong_departure_truth_summary`: 서하린에게 진실 전달, 무명 구원 조건, 후반 route 조건을 너무 직접 건드려 보류했다.
- `wuxia_qingliu_attack_after_war`: full flashback/backstory reveal이 되어 현재 중반 pressure를 덮을 수 있어 보류했다.
- `wuxia_boss_resolution`: final boss resolution/boss combat/epilogue 결산을 너무 일찍 열 수 있어 보류했다.
- `wuxia_boss_recruits_mumyeong`: 후반 스카웃/동화 사건이라 보류했다.
- `wuxia_mumyeong_destroys_orthodox_sect`: `wuxia_mumyeong_awakening` 이후의 결정적 결과라 보류했다.

## 명시적 non-goals

- 기본 office bundle/default bundle 변경
- legacy `escape-office` save/localStorage key 변경
- runtime YAML/Rust/Web generated bundle 변경
- `wuxia_mumyeong_destroys_orthodox_sect` runtime 구현
- `wuxia_boss_recruits_mumyeong` runtime 구현
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
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core preview_fixture_indexes_wuxia_first_fight
cargo test -p escape-wasm json_boundary_reaches_wuxia_mumyeong_awakening_through_preview_bundle
cargo test -p escape-terminal content_tui_smoke_reaches_wuxia_mumyeong_awakening
cd web && npm test -- --run src/core/contentBundles.test.ts
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
