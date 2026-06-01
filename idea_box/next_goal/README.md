---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_first_confrontation
mode: implementation
---

# next_goal

이 폴더는 다른 Hermes/agent 세션에 넘길 단일 prompt entry point다. 앞으로 새 세션에는 긴 프롬프트를 복사하지 말고 아래처럼 짧게 지시한다.

```text
이 repo의 idea_box/next_goal/ 폴더를 읽고 README의 현재 목표만 수행해. repo canonical docs와 충돌하면 canonical docs를 우선하고 충돌 사실을 보고해.
```

운영 원칙:

- 이 폴더에는 기본적으로 이 README 하나만 둔다.
- 여러 prompt 파일이나 future-design 분기 prompt를 만들지 않는다.
- 이 README는 “지금 다음으로 할 일” 하나만 가리킨다.
- 목표가 바뀌면 새 파일을 추가하지 말고 이 README를 교체/갱신한다.
- 최종 source of truth는 이 README가 아니라 repo canonical docs다.

## 현재 목표

`wuxia_mumyeong_first_confrontation_after_sighting` docs-only handoff는 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **runtime implementation**이다.

- `wuxia_mumyeong_first_sighting` 뒤에 `wuxia_mumyeong_first_confrontation`를 구현한다.
- 첫 대치는 “무명을 이기는 전투”가 아니라 “버티기, 카피 무공 관찰, 서하린과 무명 사이의 침묵 확인” encounter로 구현한다.
- 기존 encounter schema의 flags/clues/resources/danger/log/presentation만 사용한다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, combat resolver/schema, boss combat, 천기록 정체 reveal은 열지 않는다.

이미 완료된 기반:

- `wuxia_wounded_shelter_dawn_offers`까지 route pressure/deferred branch가 정리됐다.
- 세 route opener는 `route_opener_resolved`로 fan-in되고, `wuxia_mumyeong_first_sighting`가 그 뒤 common midgame bridge로 구현됐다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.29 2026-06-02 무협 wuxia_mumyeong_first_sighting preview runtime slice`
  - `## 0.30 2026-06-02 docs-only rival confrontation handoff: wuxia_mumyeong_first_confrontation`
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/encounter_situations.json`
- 현재 preview source:
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`

## 구현 범위

Preview source와 generated preview artifacts에만 `wuxia_mumyeong_first_confrontation`를 추가한다.

Expected runtime behavior:

- 시작 위치: `cheongryu_outer_courtyard`.
- phase: `midgame_rival` / `rival_confrontation`.
- `runtime_mode: storypack_preview` boundary 유지.
- 기존 preview route `wuxia_commute_rift_arrival` → `wuxia_heuksa_bang_first_fight` → `wuxia_cheonggi_record_first_fragment` → `wuxia_seo_harin_rescue` → `wuxia_cheongryu_apprentice_entry` → raid/route opener → `wuxia_mumyeong_first_sighting` 뒤에 붙는다.
- default office bundle에는 무협 id가 계속 없어야 한다.

## 예상 수정 파일

Preview source:

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
  - `wuxia_mumyeong_first_sighting` 뒤에 `wuxia_mumyeong_first_confrontation` 추가.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`
  - 가능하면 `cheongryu_outer_courtyard`를 재사용한다.
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`
  - 수정하지 않는 것을 기본으로 한다.

Generated preview artifacts only:

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

Likely tests/docs:

- `tests/test_web_data_export.py`
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-wasm/tests/json_contract.rs`
- `crates/escape-terminal/tests/cli_smoke.rs`
- `web/src/core/contentBundles.test.ts`
- docs sync only if implementation changes docs-visible status/counts.

## Required / forbidden conditions

Recommended encounter conditions:

```yaml
runtime_mode: storypack_preview
conditions:
  locations:
    - cheongryu_outer_courtyard
required_flags:
  - mumyeong_first_sighting_resolved
  - midgame_continuity_started
  - cheongryu_raid_survived
  - first_fragment_seen
forbidden_flags:
  - mumyeong_first_confrontation_resolved
```

Flavor-only flags:

- `mumyeong_shadow_seen`
- `copied_qingliu_flow_noted`
- `seo_harin_recognized_mumyeong`
- `mumyeong_wound_thread_opened`
- `black_serpent_trail_marked`
- `mumyeong_clue_deferred`
- `righteous_route_opened`
- `sapa_route_opened`
- `cheonggi_return_route_opened`

## Stable choice ids

Use these stable choice/action ids unless existing runtime conventions require a mechanical variant:

- `meet_mumyeong_head_on` — 위험한 정면 대치. 이기는 선택지가 아니라 물러서지 않았다는 thread를 남긴다.
- `endure_until_copy_flow_breaks` — fallback/safe endurance. 버티며 카피한 흐름의 끊김을 읽는다.
- `watch_seo_harin_hold_back` — companion observation. 서하린이 배신자라는 말을 하지 않는 침묵을 본다.
- `read_mumyeongs_copied_form` — information probe. 훔친 초식과 이해한 흐름의 차이를 읽는다.
- `do_not_provoke_mumyeong` — de-escalation. 싸움을 키우지 않지만 unresolved rival debt를 남긴다.

## Outcome hooks

기존 runtime schema만 사용한다.

Every outcome should leave the common bridge hooks:

- `mumyeong_first_confrontation_resolved`
- `mumyeong_rival_thread_opened`
- `destination_id: cheongryu_outer_courtyard`

Branch-specific flags/clues/logs may include:

- `rival_endured_not_defeated`
- `copied_flow_weakness_noted`
- `seo_harin_mumyeong_silence_confirmed`
- `cheonggi_copy_contrast_noted`
- `rivalry_deferred_not_avoided`
- `mumyeong_is_not_boss_wall`
- `winning_is_not_required`
- `copy_style_has_gap`
- `copied_flow_is_not_qingliu`
- `mumyeong_was_not_only_enemy`
- `understanding_is_not_copying`
- `not_provoking_still_leaves_debt`

Tone: 주인공은 아직 무명을 압도하지 못한다. 핵심은 이김이 아니라 버티고 읽는 것이다.

## 금지 범위

이 slice에서 열지 않는다.

- 새 `CombatState`
- HP 숫자전 / skill cooldown / combat resolver
- boss first appearance / boss combat
- `RouteGraph`
- `FactionStanding`
- `DebtLedger`
- `RelationScore`
- reward/ability/fragment choice schema
- epilogue/return system
- 천기록 정체 reveal
- 기본 office bundle / legacy `escape-office` key 변경

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_web_data_export.py tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
cargo test -p escape-core --test content_bundle
cargo test -p escape-wasm --test json_contract json_boundary_reaches_wuxia_mumyeong_first_confrontation_through_preview_bundle
cargo test -p escape-terminal --test cli_smoke content_tui_smoke_reaches_wuxia_mumyeong_first_confrontation
cd web && npm test -- --run src/core/contentBundles.test.ts
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
