---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_followup_after_copy_style_reveal
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

`wuxia_mumyeong_copy_style_reveal` runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **docs-only handoff**다.

- Notion 사건 카드 DB와 repo hooks를 다시 대조해 copy-style reveal 이후 다음 runtime 후보를 하나만 고른다.
- 비교 후보는 최소 `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_reads_orthodox_style`를 포함한다.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml`, Rust/Web generated preview bundle, legacy office bundle은 수정하지 않는다.
- seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, boss combat/final-wall pressure, 무명 과거 진실 reveal, route graph/faction reputation/debt/relation schema, epilogue/return schema, 천기록 정체 reveal은 바로 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.32`: docs-only post-confrontation handoff for `wuxia_mumyeong_copy_style_reveal`
  - section `0.33`: 무협 `wuxia_mumyeong_copy_style_reveal` preview runtime slice
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

## handoff 방향

다음 docs-only handoff는 아래 질문에 답해야 한다.

1. `wuxia_mumyeong_midgame_reunion`을 열 만큼 무명 과거 단서가 충분한가?
2. `wuxia_boss_first_appearance`를 지금 열면 boss-wall/final logic pressure가 너무 빨라지는가?
3. `wuxia_mumyeong_departure_truth_summary` 또는 `wuxia_mumyeong_reads_orthodox_style`가 copy-style reveal 다음의 더 안전한 bridge인가?
4. 다음 runtime 후보가 기존 encounter schema의 `conditions`, `choices`, `outcome.flags`, `outcome.clues`, `log`, `presentation`만으로 표현 가능한가?

## 예상 수정 파일

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `idea_box/next_goal/README.md`
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 -m json.tool docs/content/storypack_db/storypacks.json >/tmp/storypacks.json
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/tmp/encounter_situations.json
git diff --exit-code -- src/tui_adv/storypack-previews/wuxia_jianghu_pack crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
