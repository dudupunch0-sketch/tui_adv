---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_sado_final_phase_1_price_tag_handoff
previous_current_goal: wuxia_sado_final_state_routing_contract
mode: docs-first-runtime-handoff
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

`wuxia_seoharin_left_meal` runtime implementation은 완료됐다. `wuxia_seoharin_left_meal_followup` docs-only handoff도 완료됐고, 그 결과 직접 final runtime을 열기 전에 `docs/design/Wuxia_Final_State_Routing.md`로 final-route state dictionary/result routing contract를 먼저 고정했다.

현재 목표는 `wuxia_sado_final_phase_1_price_tag_handoff`다. `사도 최종전 1페이즈: 가격표`를 기존 encounter schema로 구현할 수 있는지 repo canonical docs와 Notion source를 다시 대조한다.

## 비교할 후보

최소 다음 후보를 비교한다.

- `wuxia_sado_final_phase_1_price_tag`
- `wuxia_sado_final_battle`
- `wuxia_sado_final_phase_2_weakpoint_control`
- `wuxia_sado_final_phase_3_outside_calculation`
- `wuxia_boss_resolution`
- `wuxia_mumyeong_resolution`
- `wuxia_seoharin_unsaid_stay`
- `docs/design/Wuxia_Final_State_Routing.md`의 canonical final inputs와 boundary

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.55`: `wuxia_seoharin_left_meal` preview runtime slice
  - section `0.56`: final state routing contract handoff
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/design/Wuxia_Final_State_Routing.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`

## 이미 완료된 기반

- Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
- `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`까지 runtime slice는 완료됐다.
- `docs/design/Wuxia_Final_State_Routing.md`는 `canonical_final_inputs`, `final_result_priority`, `final_epilogue_master_matrix` handoff boundary, state alias/deprecation policy를 소유한다.
- `wuxia_seoharin_left_meal`은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고 `seoharin_left_meal_resolved`로 반복을 막는다.
- stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
- 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`와 `left_meal_was_kept_for_return`/`belonging_is_daily_care` clue를 남기고, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`와 `last_bowl_epilogue_seeded` clue를 남긴다.

## 금지선

- 서하린에게 무명 이탈 진실을 직접 전달하지 않는다.
- `told_seoharin_truth` flag를 추가하지 않는다.
- 무명 구원 확정, 무명 결산, 보스 결산, 사도 최종전, final battle을 바로 구현하지 않는다.
- final/epilogue/return schema, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천기록 identity reveal을 바로 열지 않는다.
- `item_unpriced_wooden_sword`를 실제 아이템으로 지급하지 않는다.
- legacy office bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key를 변경하지 않는다.
- `wuxia_sado_final_phase_1_price_tag` 후보를 검토하더라도 combat resolver, HP numeric battle, final epilogue renderer, return/settlement schema를 열지 않는다.

## 이번 handoff 산출물

다음 runtime 후보를 정한 뒤 최소 다음 파일을 갱신한다.

- `docs/design/Wuxia_Final_State_Routing.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `idea_box/next_goal/README.md`
