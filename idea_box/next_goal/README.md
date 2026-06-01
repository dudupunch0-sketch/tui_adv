---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: wuxia_mumyeong_followup_after_first_confrontation
mode: docs-only-handoff
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

`wuxia_mumyeong_first_confrontation` runtime implementation까지 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이자 메인 개발 기준이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **docs-only handoff**다.

- `wuxia_mumyeong_followup_after_first_confrontation`를 설계한다.
- Notion 사건 카드 DB `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`를 비교하고 다음 runtime 후보 하나만 고른다.
- `wuxia_mumyeong_copy_style_reveal`는 첫 대치 이후가 선행 조건이고, 이번 회차 무명이 어떤 무공을 카피했는지 공개하는 후보라 강한 후속 후보다.
- 이번 goal에서는 runtime YAML, Rust/Web generated preview bundle, 기본 office bundle을 수정하지 않는다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, random copy-style system, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, combat resolver/schema, boss combat, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.30 2026-06-02 docs-only rival confrontation handoff: wuxia_mumyeong_first_confrontation`
  - `## 0.31 2026-06-02 무협 wuxia_mumyeong_first_confrontation preview runtime slice`
  - 현재 최우선 남은 작업
  - `## 10. 다음 액션`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/dev/Notion_Design_Coverage.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/encounter_situations.json`

## 설계 방향

이미 완료된 기반:

- `wuxia_mumyeong_first_sighting`가 preview runtime에 구현되어 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `mumyeong_exists`, `copied_flow_is_not_qingliu` hook을 남긴다.
- `wuxia_mumyeong_first_confrontation`가 preview runtime에 구현되어 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `copied_flow_weakness_noted`, `seo_harin_mumyeong_silence_confirmed`, `cheonggi_copy_contrast_noted` hook을 남긴다.

비교할 후보:

- `wuxia_mumyeong_copy_style_reveal`: Notion 선행 조건이 첫 대치 이후다. 랜덤 카피 무공을 공개하고 청류안 분석/천외편린 후보 변형을 암시한다. 단, seed 기반 random copy-style system이나 reward schema 없이 기존 flags/clues/log/presentation으로 먼저 표현 가능한지 결정해야 한다.
- `wuxia_mumyeong_midgame_reunion`: Notion 선행 조건이 첫 대치와 과거 단서 일부다. 관계를 라이벌/거울로 강화하지만, 아직 과거 단서가 충분한지 확인해야 한다.
- `wuxia_boss_first_appearance`: 보스가 압도적 벽으로 등장한다. boss-wall logic을 열 수 있지만 boss combat/final-route pressure가 커져 현 slice에는 클 수 있다.
- route-specific clue bridge: 정파/사파/천기 opener flavor에 따라 무명 후속 단서를 하나 더 둘지 비교한다.

결정 기준:

- Notion `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 사건 카드 DB row를 대조한다.
- 새 random copy-style system, combat resolver/schema, RouteGraph, FactionStanding, DebtLedger, RelationScore, reward/ability/epilogue/return schema를 열지 않고 가능한 후보를 우선한다.
- 다음 runtime 후보는 하나만 고르고, 구현은 다음 세션으로 넘긴다.

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
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py`
- 이 README

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
git diff --exit-code -- src/tui_adv/storypack-previews/wuxia_jianghu_pack crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
