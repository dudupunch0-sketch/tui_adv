---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_reads_orthodox_style
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

`wuxia_mumyeong_followup_after_copy_style_reveal` docs-only handoff는 완료됐다. 다음 runtime 후보는 `wuxia_mumyeong_reads_orthodox_style` / **무명의 정파 무공 간파**다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이다.

이번 세션의 목표는 **runtime implementation**이다.

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_copy_style_reveal` 뒤에 `wuxia_mumyeong_reads_orthodox_style`를 추가한다.
- Rust/Web storypack preview bundle만 재생성한다.
- Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트를 갱신한다.
- `src/tui_adv/data/*.yaml`, 기본 office `content.bundle.json`, Web 기본 generated office bundle, legacy `escape-office` save/localStorage key는 수정하지 않는다.
- 무명 중반 재회(`wuxia_mumyeong_midgame_reunion`), 보스 첫 등장(`wuxia_boss_first_appearance`), 무명 이탈 진실 정리(`wuxia_mumyeong_departure_truth_summary`), `wuxia_qingliu_attack_after_war` full flashback, seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, combat resolver/schema, route graph/faction reputation/debt/relation schema, epilogue/return schema, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - section `0.34`: docs-only post-copy-style handoff for `wuxia_mumyeong_reads_orthodox_style`
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
- `wuxia_mumyeong_reads_orthodox_style` handoff는 현악문/복호금쇄수 단서를 다음 runtime 대상으로 고정했다.

## 구현 경계

Start conditions:

```yaml
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags:
    - mumyeong_copy_style_reveal_resolved
    - copy_style_hint_recorded
    - midgame_continuity_started
    - first_fragment_seen
  forbidden_flags: [mumyeong_reads_orthodox_style_resolved]
```

Stable choice ids:

- `compare_copied_form_to_old_wound`
- `trace_qingliu_eye_variation`
- `reconstruct_mumyeongs_sightline`
- `stop_before_truth_becomes_accusation`

Common outcome hooks:

- `mumyeong_reads_orthodox_style_resolved`
- `orthodox_style_trace_recorded`
- `destination_id: cheongryu_outer_courtyard`

Primary flags/clues:

- `hyeonakmun_trace_suspected`
- `bokho_geumsaesu_name_recorded`
- `mumyeong_eye_variation_noted`
- `orthodox_control_is_violence`
- `departure_truth_still_incomplete`

Presentation:

```yaml
visual_id: wuxia_mumyeong_reads_orthodox_style
speaker: 천기록
layout: orthodox_style_trace
effect_cues:
  - stable_terms: [현악문, 복호금쇄수, 무명]
```

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
PYTHONPATH=src python3 scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
python3 -m json.tool docs/content/storypack_db/storypacks.json >/tmp/storypacks.json
python3 -m json.tool docs/content/storypack_db/encounter_situations.json >/tmp/encounter_situations.json
cargo fmt --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_reads_orthodox_style_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_reads_orthodox_style
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
