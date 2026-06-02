---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_destroys_orthodox_sect
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

`wuxia_qingliu_attack_after_war_followup` docs-only handoff는 완료됐다. `wuxia_qingliu_attack_after_war` runtime implementation도 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 다음 목표는 **`wuxia_mumyeong_destroys_orthodox_sect` preview runtime implementation**이다.

이 구현은 Notion 사건 카드 DB의 `정파 문파 멸문`을 그대로 전투/멸문 장면으로 재생하지 않는다. `wuxia_qingliu_attack_after_war`가 남긴 현악문/복호금쇄수 흔적 이후, **빈 현악문 산문과 남은 기록/풍문을 확인하는 제한된 consequence trace**로만 구현한다.

## Notion 대조 결과

직접 확인한 source:

- parent page `무협 스토리팩: 이구학지 — 천기록`
- 사건 카드 DB `wuxia_mumyeong_destroys_orthodox_sect` / `정파 문파 멸문`
- 사건 카드 DB `wuxia_boss_recruits_mumyeong` / `흑사방 보스의 스카웃`
- 사건 카드 DB `wuxia_mumyeong_departure_truth_summary` / `무명 이탈의 진실 정리`
- 사건 카드 DB `wuxia_mumyeong_resolution` / `무명 결산`
- 사건 카드 DB `wuxia_boss_resolution` / `보스 결산`
- 사건 카드 DB `wuxia_seoharin_empty_place` / `비워둔 자리`
- 운영 문서 `04. 메인 루트 구조`
- 운영 문서 `99. 통합 체크포인트`

선택 이유:

- `wuxia_mumyeong_destroys_orthodox_sect`는 `wuxia_qingliu_attack_after_war`가 남긴 `hyeonakmun_attack_thread_opened`를 가장 직접적으로 회수한다.
- `wuxia_boss_recruits_mumyeong`은 정파 문파 멸문 이후의 후반 스카웃 사건이라 아직 선행이 부족하다.
- `wuxia_mumyeong_departure_truth_summary`는 서하린에게 진실 전달과 무명 구원 조건을 너무 직접 연다.
- `wuxia_mumyeong_resolution`과 `wuxia_boss_resolution`은 최종장/후일담 routing 범위다.
- `wuxia_seoharin_empty_place`는 초반 서하린 감정선이라 현재 post-Qingliu trace 위치로 되돌리기보다 future companion beat로 남긴다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.45`: `wuxia_qingliu_attack_after_war` preview runtime slice
  - section `0.46`: `wuxia_qingliu_attack_after_war_followup` docs-only handoff
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
- `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.
- `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war` runtime slice는 완료됐다.
- `wuxia_mumyeong_awakening` runtime implementation은 완료됐다.
- `wuxia_mumyeong_followup_after_awakening` docs-only handoff도 완료됐다.
- `wuxia_qingliu_attack_after_war` runtime implementation은 완료됐고, `wuxia_qingliu_attack_after_war_followup` docs-only handoff도 완료됐다.
- 이전 handoff anchor: `wuxia_mumyeong_followup_after_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`.
- 이전 hook anchor: `mumyeong_first_confrontation_resolved`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `boss_first_appearance_resolved`, `mumyeong_request_for_aid_resolved`.
- 이전 choice anchor: `compare_anger_to_copied_flow`, `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`.
- `wuxia_mumyeong_request_for_aid`는 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid` hook을 남긴다.
- `wuxia_mumyeong_awakening`는 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready` hook을 남긴다.
- `wuxia_qingliu_attack_after_war`는 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened` hook을 남긴다.
- `wuxia_qingliu_attack_after_war` primary clues는 `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`다.

## 구현 계약

- encounter id: `wuxia_mumyeong_destroys_orthodox_sect`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_qingliu_attack_after_war`
- location: `cheongryu_outer_courtyard`
- presentation: `visual_id: wuxia_mumyeong_destroys_orthodox_sect`, `speaker: 천기록`, `layout: hyeonakmun_empty_gate_record`, stable terms `[현악문, 복호금쇄수, 무명]`

required flags:

- `qingliu_attack_after_war_resolved`
- `qingliu_attack_trace_confirmed`
- `hyeonakmun_attack_thread_opened`
- `mumyeong_awakening_resolved`
- `midgame_continuity_started`

forbidden flags:

- `mumyeong_destroys_orthodox_sect_resolved`

flavor-only flags/clues:

- `qingliu_attack_trace_points_to_hyeonakmun`
- `bokho_geumsaesu_used_on_qingliu`
- `main_sect_not_directly_accused`
- `full_flashback_still_unopened`
- `mumyeong_tried_to_save_qingliu`
- `orthodox_refusal_broke_mumyeong`
- `salvation_truth_still_unready`

stable choice ids:

- `read_hyeonakmun_empty_gate_record`
- `trace_bokho_lock_to_mumyeong`
- `ask_why_seoharin_never_heard_full_story`
- `stop_before_counting_the_dead`

common outcome hook:

- flags: `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`
- clues: `hyeonakmun_was_destroyed_after_qingliu_attack`, `mumyeong_destroyed_hyeonakmun_alone`, `destruction_is_consequence_not_salvation`, `seoharin_truth_delivery_still_unopened`, `boss_recruitment_still_unopened`
- `destination_id: cheongryu_outer_courtyard`

## 구현 범위

해야 할 것:

- 빈 현악문 산문, 부러진 현판, 금쇄수 자국이 사라진 기록 조각처럼 보이는 trace encounter를 만든다.
- 현악문/복호금쇄수 확정명을 유지한다.
- 무명이 현악문을 멸문시켰다는 사실을 “결과/풍문/기록”으로 좁혀 연다.
- 멸문은 구원이나 정의가 아니라 consequence라는 clue를 남긴다.
- 보스 스카웃과 departure truth summary로 이어질 bridge hook만 남긴다.

하지 말 것:

- 현악문 멸문 전투를 playable combat으로 만들기
- 청류문 습격 full flashback/backstory reveal
- 서하린에게 진실 전달
- 무명 구원 확정
- 보스 스카웃 runtime 구현
- 보스 전투/final boss resolution
- 정파 문파 전체를 공동파 본산 악역으로 확정
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
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core preview_fixture_indexes_wuxia_first_fight
cd web && npm test -- --run src/core/contentBundles.test.ts
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
git diff --check
```
