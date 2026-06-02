---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_seoharin_empty_place
previous_current_goal: wuxia_mumyeong_departure_truth_summary_followup
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

`wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff는 완료됐다. Notion 사건 카드 DB, `사도 최종전`, `보스 결산`, `무명 결산`, `최종장 결산 라우팅 마스터`, repo hook을 대조한 결과, 다음 runtime 후보는 **`wuxia_seoharin_empty_place`**다.

이번 목표는 `wuxia_seoharin_empty_place`를 preview runtime으로 구현하는 것이다. 이 카드의 역할은 서하린의 “비워둔 자리”를 초반 companion beat가 아니라 sealed departure truth 이후 다시 보는 late emotional bridge로 landing해, 나중 최종전의 `seoharin_axis: high`, `remembered_empty_place`, `item_unpriced_wooden_sword` 후보를 기존 flags/clues/log/presentation만으로 준비하는 것이다.

## 구현 계약

- encounter id: `wuxia_seoharin_empty_place`
- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- insert after: `wuxia_mumyeong_departure_truth_summary`
- location: `cheongryu_outer_courtyard`
- required flags:
  - `mumyeong_departure_truth_summary_resolved`
  - `sealed_departure_truth_summary_prepared`
  - `truth_delivery_still_unopened`
  - `midgame_continuity_started`
- forbidden flags: `seoharin_empty_place_resolved`
- stable choice ids:
  - `ask_who_kept_the_empty_place`
  - `leave_the_place_unclaimed`
  - `set_down_the_work_notebook_briefly`
  - `step_back_without_naming_mumyeong`
- common outcome hooks:
  - `seoharin_empty_place_resolved`
  - `seoharin_axis_opened`
  - `empty_place_remembered`
  - `truth_delivery_still_unopened`
  - `destination_id: cheongryu_outer_courtyard`
- primary clues:
  - `seoharin_remembers_without_possessing`
  - `empty_place_is_return_not_claim`
  - `mumyeong_place_still_unclaimed`
  - `unpriced_wooden_sword_condition_seeded`
- presentation:
  - `visual_id: wuxia_seoharin_empty_place`
  - `speaker: 서하린`
  - `layout: empty_place_memory`
  - stable terms: `[서하린, 무명, 청류문, 목검]`

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.51`: `wuxia_mumyeong_departure_truth_summary` preview runtime slice
  - section `0.52`: `wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff
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
- `wuxia_mumyeong_departure_truth_summary`까지 runtime slice는 완료됐다.
- `wuxia_mumyeong_departure_truth_summary`는 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened` hook을 남겼다.
- `wuxia_mumyeong_departure_truth_summary_followup` handoff는 `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_sado_final_battle`, final routing master를 바로 열지 않기로 결정했다.

## 금지선

- 서하린에게 무명 이탈 진실을 직접 전달하지 않는다.
- `told_seoharin_truth` flag를 추가하지 않는다.
- 무명 구원 확정, 무명 결산, 보스 결산, 사도 최종전, final battle을 구현하지 않는다.
- final/epilogue/return schema, combat resolver/schema, HP 숫자전, route graph, faction reputation, relation/debt ledger, reward/ability schema, 천기록 identity reveal을 열지 않는다.
- `item_unpriced_wooden_sword`를 실제 아이템으로 지급하지 않는다. 이번 slice에서는 그 조건의 seed/clue만 남긴다.
- legacy office bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key를 변경하지 않는다.

## 검증 명령

runtime implementation이라면 최소 다음을 실행한다.

```bash
PYTHONPATH=src /tmp/dudu-tui-adv-pytest-venv/bin/python -m pytest tests/test_docs_contract.py tests/test_storypack_db.py tests/test_web_data_export.py -q
python3 scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
cargo fmt --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_seoharin_empty_place
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --check
```
