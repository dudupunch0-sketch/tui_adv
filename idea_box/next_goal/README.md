---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_followup_after_orthodox_style_trace
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

`wuxia_mumyeong_reads_orthodox_style` / **무명의 정파 무공 간파** preview runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/terminal default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **docs-only handoff**다.

- Notion 사건 카드 DB와 repo runtime hook을 대조해 정파 무공 간파 이후의 다음 runtime 후보 하나를 고른다.
- 비교 후보는 최소 `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`를 포함한다.
- `docs/dev/Development_Plan.md`, `docs/dev/Checklist.md`, `docs/dev/Notion_Design_Coverage.md`, `docs/content/storypacks/wuxia_jianghu_pack.md`, `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypack_db/*`, `docs/design/Storypack_World_Model.md`, `docs/design/Storypack_Encounter_DB.md`, 이 파일을 handoff 결과에 맞춰 갱신한다.
- runtime YAML, Rust/Web generated preview bundle, Web default content bundle, legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`는 수정하지 않는다.
- seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation schema, epilogue/return schema, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.34`: docs-only post-copy-style handoff for `wuxia_mumyeong_reads_orthodox_style`
  - section `0.35`: `wuxia_mumyeong_reads_orthodox_style` preview runtime slice
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
- `idea_box/notion_sources.yml`

## 이미 완료된 기반

- `wuxia_mumyeong_first_sighting`가 preview runtime에 구현되어 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `mumyeong_exists`, `copied_flow_is_not_qingliu` hook을 남긴다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.
- `wuxia_mumyeong_copy_style_reveal`가 preview runtime에 구현되어 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남긴다.
- `wuxia_mumyeong_reads_orthodox_style`가 preview runtime에 구현되어 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook을 남긴다.
- Web default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, terminal `--scene content`도 bundle 인자를 생략하면 같은 이구학지 built-in fixture를 기본으로 선택한다.

## Handoff 기준

다음 runtime 후보를 고를 때는 아래를 비교한다.

- `wuxia_mumyeong_midgame_reunion`: 무명 rival thread를 관계/오해/재회로 밀고 가는 카드. 정파 무공 간파의 단서가 충분히 개인 서사로 이어지는지 검토한다.
- `wuxia_mumyeong_departure_truth_summary`: 무명 이탈 진실 정리. 후반 truth reveal 범위가 커서 너무 이른지 확인한다.
- `wuxia_boss_first_appearance`: 혈월교/보스 압박 첫 등장을 열 수 있지만 boss combat/final-wall schema를 유발하지 않는지 확인한다.
- `wuxia_qingliu_attack_after_war`: 청류문 습격 이후 과거/후폭풍 flashback. full backstory reveal을 과도하게 당기지 않는지 확인한다.

결과 문서에는 다음을 남긴다.

- 선택한 `next_runtime_candidate`
- 보류한 후보와 이유
- start condition 초안
- stable choice id 후보
- common flags/clues/log/presentation hook
- 이번 handoff에서 열지 않는 schema/non-goals
- 다음 구현 세션의 예상 수정 파일과 검증 명령

## 검증 명령

docs-only handoff에서는 runtime bundle을 재생성하지 않는다. 문서/DB만 바꿨다면 아래를 기본 검증으로 사용한다.

```bash
PYTHONPATH=src python3 -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 -m json.tool docs/content/storypack_db/storypacks.json >/tmp/storypacks.json
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/tmp/encounter_situations.json
git diff --exit-code -- src/tui_adv/storypack-previews crates/escape-core/fixtures/content/storypack-preview web/src/data/generated/storypack-preview
git diff --check
```
