---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: route_midgame_continuity_after_wounded_shelter
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

`wuxia_wounded_shelter_dawn_offers` runtime implementation은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **docs-only handoff**다.

- `route_midgame_continuity_after_wounded_shelter`를 설계한다.
- 세 route opener(`wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`)와 deferred-offer card(`wuxia_wounded_shelter_dawn_offers`)가 모두 구현된 뒤 첫 midgame continuity를 어떻게 열지 결정한다.
- route별 3개 card, 공통 midgame card, 또는 opened flags 기반 schema-less bridge 중 하나를 비교하고 다음 runtime 후보 하나만 고른다.
- 이번 goal에서는 runtime YAML, Rust/Web generated preview bundle, 기본 office bundle을 수정하지 않는다.
- 기본 office bundle, legacy `escape-office` save/localStorage key, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않는다.

## 반드시 읽을 문서

- `AGENTS.md`
- `docs/dev/Development_Plan.md`
  - `## 0.0b 2026-06-01 default storypack 전환`
  - `## 0.21 2026-06-01 무협 wuxia_baekdo_medicine_debt preview runtime slice`
  - `## 0.22 2026-06-01 docs-only route opener follow-up handoff: wuxia_black_heaven_escape_price`
  - `## 0.23 2026-06-02 무협 wuxia_black_heaven_escape_price preview runtime slice`
  - `## 0.24 2026-06-02 docs-only route opener follow-up handoff: wuxia_heavenly_archive_previous_outsiders`
  - `## 0.25 2026-06-02 무협 wuxia_heavenly_archive_previous_outsiders preview runtime slice`
  - `## 0.26 2026-06-02 docs-only route opener follow-up handoff: wuxia_wounded_shelter_dawn_offers`
  - `## 0.27 2026-06-02 무협 wuxia_wounded_shelter_dawn_offers preview runtime slice`
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

비교할 후보:

- route별 midgame card 3개: `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`를 각각 받는다. 장점은 route tone이 선명하지만, runtime 후보가 3개로 커지고 route graph 유혹이 생긴다.
- 공통 midgame bridge 1개: route opened flags 중 하나를 받아 공통 압박으로 묶는다. 장점은 다음 slice가 작지만, 기존 schema에 any-of condition이 없으므로 start condition 표현을 신중히 해야 한다.
- deferred-offer 후속 bridge: `route_commitment_reopened`와 route starter/opened flags의 조합을 살핀다. 장점은 방금 구현한 card와 자연스럽지만, 이미 opener를 탄 direct branch와의 균형을 확인해야 한다.

결정 기준:

- Notion `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 repo docs를 대조한다.
- 새 any-of condition, RouteGraph, FactionStanding, DebtLedger, RelationScore, reward/ability/epilogue/return schema를 열지 않고도 가능한 후보를 우선한다.
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
- 필요 시 `tests/test_storypack_db.py`
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
