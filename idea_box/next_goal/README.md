---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_seoharin_empty_place_followup
previous_current_goal: wuxia_seoharin_empty_place
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

`wuxia_seoharin_empty_place` runtime implementation은 완료됐다. 이 slice는 `wuxia_mumyeong_departure_truth_summary` 뒤에서 서하린의 “비워둔 자리”를 late emotional bridge로 landing했고, `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened` hook을 남겼다. `item_unpriced_wooden_sword`는 지급하지 않고 `unpriced_wooden_sword_condition_seeded` clue만 남겼다.

이번 목표는 `wuxia_seoharin_empty_place_followup` docs-only handoff다. Notion 사건 카드 DB와 최신 final routing 문서를 다시 확인해, 다음 runtime 후보를 결정한다.

## 비교할 후보

최소 다음 후보를 비교한다.

- `wuxia_mumyeong_resolution`
- `wuxia_boss_resolution`
- `wuxia_sado_final_battle`
- `wuxia_sado_final_phase_2_weakpoint_control`
- `wuxia_sado_final_phase_3_outside_calculation`
- `wuxia_seoharin_unsaid_stay`
- `wuxia_seoharin_left_meal`
- 남은 late truth/final/companion 후보

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.52`: `wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff
  - section `0.53`: `wuxia_seoharin_empty_place` preview runtime slice
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
- `wuxia_mumyeong_departure_truth_summary`까지 runtime slice는 완료됐다.
- `wuxia_seoharin_empty_place`도 preview runtime으로 구현됐다.
- `wuxia_seoharin_empty_place`는 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고 `seoharin_empty_place_resolved`로 반복을 막는다.
- stable choice id는 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`다.

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
- 관련 docs/storypack DB contract tests

## 검증 명령

docs-only handoff라면 최소 다음을 실행한다.

```bash
python3 -m json.tool docs/content/storypack_db/storypacks.json >/dev/null
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/dev/null
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
git diff --check
```
