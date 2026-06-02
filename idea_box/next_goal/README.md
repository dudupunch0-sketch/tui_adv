---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_seoharin_left_meal_followup
previous_current_goal: wuxia_seoharin_left_meal
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

`wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

`wuxia_seoharin_left_meal` runtime implementation은 완료됐다. 이 slice는 `wuxia_seoharin_empty_place` 뒤에서 서하린이 남겨 둔 식은 밥 한 그릇을 통해 말보다 생활로 드러나는 소속감 bridge를 landing했고, `seoharin_left_meal_resolved`, `seoharin_axis_deepened`, `qingliu_belonging_warmed`, `left_meal_left_untouched`, `truth_delivery_still_unopened` hook을 남겼다. `wuxia_seoharin_unsaid_stay`의 귀환/정착/침식 최종 관계 분기, 무명/보스 결산, 사도 최종전, `item_unpriced_wooden_sword` 지급은 열지 않았다.

이번 목표는 `wuxia_seoharin_left_meal_followup` docs-only handoff다. Notion 사건 카드 DB, 후일담/최종 routing 문서, repo canonical docs를 다시 확인해 다음 runtime 후보를 결정한다.

## 비교할 후보

최소 다음 후보를 비교한다.

- `wuxia_seoharin_unsaid_stay`
- `wuxia_mumyeong_resolution`
- `wuxia_boss_resolution`
- `wuxia_sado_final_battle`
- `wuxia_sado_final_phase_2_weakpoint_control`
- `wuxia_sado_final_phase_3_outside_calculation`
- final-route state dictionary / 무명 구원 / 보스 결산 / 서하린 late companion 후보
- 남은 late truth/final/companion 후보

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.54`: `wuxia_seoharin_left_meal` 선택 handoff
  - section `0.55`: `wuxia_seoharin_left_meal` preview runtime slice
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

- Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
- `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`까지 runtime slice는 완료됐다.
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

## 이번 handoff 산출물

다음 runtime 후보를 정한 뒤 최소 다음 파일을 갱신한다.

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `idea_box/next_goal/README.md`
