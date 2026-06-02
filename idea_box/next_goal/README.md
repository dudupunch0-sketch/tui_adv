---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_departure_truth_summary
previous_current_goal: wuxia_boss_recruits_mumyeong_followup
mode: runtime-implementation
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

`wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

`wuxia_boss_recruits_mumyeong` preview runtime implementation은 완료됐고, **`wuxia_boss_recruits_mumyeong_followup` docs-only handoff도 완료됐다.**

이번 세션의 다음 목표는 **`wuxia_mumyeong_departure_truth_summary` runtime implementation**이다. 구현 범위는 무명 이탈의 진실을 `sealed_departure_truth_summary`로 정리해 보관하는 trace다. 서하린에게 진실을 전달하거나 무명 구원을 확정하지 않는다.

## handoff에서 비교 완료한 후보

다음 후보는 이미 Notion 사건 카드 DB와 repo hook 기준으로 비교했다.

- `wuxia_mumyeong_departure_truth_summary`
- `wuxia_mumyeong_resolution`
- `wuxia_boss_resolution`
- `wuxia_seoharin_empty_place`
- `wuxia_mumyeong_destroys_orthodox_sect_followup`에서 보류했던 후반 truth/final 후보들

결론:

- 선택: `wuxia_mumyeong_departure_truth_summary`
- 이유: `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_saw_mumyeongs_wound`, `departure_truth_thread_deepened`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened` hook을 가장 직접 받는다.
- 제한: 다음 runtime은 진실을 정리하지만 아직 전하지 않는 sealed summary다.
- 보류: `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`은 최종장/후일담 routing이다.
- 보류: `wuxia_seoharin_empty_place`는 초반 companion beat라 현재 stage와 맞지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.49`: `wuxia_boss_recruits_mumyeong` preview runtime slice
  - section `0.50`: `wuxia_boss_recruits_mumyeong_followup` docs-only handoff
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
- `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong` runtime slice는 완료됐다.
- `wuxia_mumyeong_destroys_orthodox_sect_followup` docs-only handoff도 완료됐고, 그 결과 `wuxia_boss_recruits_mumyeong`을 다음 runtime 후보로 선택했다.
- `wuxia_boss_recruits_mumyeong_followup` docs-only handoff도 완료됐고, 그 결과 `wuxia_mumyeong_departure_truth_summary`를 다음 runtime 후보로 선택했다.
- `wuxia_mumyeong_awakening` runtime implementation은 완료됐다.
- `wuxia_mumyeong_followup_after_awakening` docs-only handoff도 완료됐다.
- `wuxia_qingliu_attack_after_war` runtime implementation은 완료됐다.
- `wuxia_qingliu_attack_after_war_followup` docs-only handoff도 완료됐다.
- `wuxia_mumyeong_destroys_orthodox_sect` preview runtime implementation은 완료됐다.
- `wuxia_boss_recruits_mumyeong` required flags는 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`다.
- `wuxia_boss_recruits_mumyeong`는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent` stable choice id를 사용한다.
- `wuxia_boss_recruits_mumyeong`는 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_saw_mumyeongs_wound`, `boss_recruited_mumyeong_after_hyeonakmun`, `recruitment_was_not_salvation`, `mumyeong_had_nowhere_to_stand_after_destruction`, `boss_turned_wound_into_power`, `departure_truth_not_ready_for_seoharin`, `boss_resolution_still_unopened` hook을 남긴다.
- `wuxia_mumyeong_departure_truth_summary` 구현 계약은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started` required flags와 `mumyeong_departure_truth_summary_resolved` forbidden flag를 사용한다.
- Stable choice ids는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`다.
- Common outcome hooks는 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.

Historical hook anchors: `mumyeong_first_confrontation_resolved`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `boss_first_appearance_resolved`, `mumyeong_request_for_aid_resolved`, `mumyeong_awakening_resolved`, `qingliu_attack_after_war_resolved`, `mumyeong_destroys_orthodox_sect_resolved`.

Historical handoff anchors: `wuxia_mumyeong_followup_after_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_request_for_aid`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_mumyeong_destroys_orthodox_sect_followup`.

Historical choice anchors: `compare_anger_to_copied_flow`, `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`, `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`.

Historical guardrails: `random copy-style system`, `seed 기반 random copy-style system/table`, Web/terminal default storypack, 현악문 멸문 전투를 playable combat으로 만들기.

## 구현 범위

해야 할 것:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_recruits_mumyeong` 뒤에 `wuxia_mumyeong_departure_truth_summary`를 추가한다.
- 기존 encounter schema와 flags/clues/log/presentation만 사용한다.
- Rust/Web storypack preview generated bundle을 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트를 갱신한다.
- canonical docs와 docs contract tests를 구현 완료 기준으로 갱신한다.

하지 말 것:

- legacy `escape-office` save/localStorage key 변경
- 서하린에게 진실 전달
- `told_seoharin_truth` flag 추가
- 무명 구원 확정
- 무명 결산/보스 결산 구현
- final battle, epilogue/return 구현
- combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, epilogue/return system, 천기록 identity reveal 열기

## 검증 명령

runtime implementation이라면 최소 다음을 실행한다.

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
