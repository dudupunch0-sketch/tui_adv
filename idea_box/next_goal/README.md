---
type: next_goal_prompt
created: 2026-06-01
updated: 2026-06-02
current_goal: route_opener_followup_after_heavenly_archive
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

`wuxia_heavenly_archive_previous_outsiders` preview runtime 구현은 완료됐다. `wuxia_jianghu_pack` / **이구학지 — 천기록**은 Web/default storypack이며, office content는 legacy/parity fixture로 유지한다.

이번 세션의 목표는 **docs-only handoff**다.

- `route_opener_followup_after_heavenly_archive`를 설계한다.
- 정파/사파/천기·귀환 opener 이후 다음 runtime 승격 후보 하나를 고른다.
- 비교 후보는 `stabilize_wounded_until_dawn` branch를 받는 deferred-offer card와 세 route opener 이후의 첫 midgame continuity card다.
- Notion reference와 repo storypack/encounter DB를 대조해 start conditions, stable choice ids, outcome hooks, schema non-goals를 문서화한다.
- runtime YAML/Rust/Web/generated artifact는 수정하지 않는다.
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

## handoff 방향

비교할 후보:

1. deferred-offer card
   - route flags: `route_commitment_deferred`, `deferred_route_reopened`, `wounded_shelter_stabilized`
   - purpose: `stabilize_wounded_until_dawn` fallback branch가 세 route opener를 직접 타지 않은 경우, 회복/제안/후속 압박으로 다시 메인 흐름에 붙인다.
   - 위험: triage/companion death/mass combat/relation/debt 시스템으로 커지는 유혹
2. post-opener midgame continuity card
   - route flags: `righteous_route_opened`, `sapa_route_opened`, `cheonggi_return_route_opened`
   - purpose: 세 route opener 이후 공통 midgame pressure를 연다.
   - 위험: any-of route condition schema, route graph/faction reputation/ending schema를 너무 빨리 여는 유혹

선택 기준:

- 기존 flags/clues/log/presentation만으로 구현 가능한가.
- 세 route opener 이후 route pressure를 가장 선명하게 다음으로 밀어 주는가.
- 청류문 내부 배신/고구마가 아니라 외부 압박과 결핍을 갈등 원천으로 유지하는가.
- 천기록 정체 reveal, 천외편린 3택 reward/ability schema, return system, route graph/faction reputation/debt ledger를 열지 않아도 되는가.

## 예상 수정 파일

- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `docs/dev/Notion_Design_Coverage.md` if Notion comparison needs a ledger update
- `tests/test_docs_contract.py`
- `tests/test_storypack_db.py` only if machine-readable DB shape changes
- 이 `idea_box/next_goal/README.md`

## 검증 명령

```bash
PYTHONPATH=src python3 -m pytest tests/test_docs_contract.py tests/test_storypack_db.py -q
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
git diff --exit-code -- src/tui_adv/data crates/escape-core/fixtures/content/content.bundle.json web/src/data/generated/content.bundle.json
git diff --check
```

pytest가 없는 WSL 환경이면 `/tmp` venv를 만들거나 repo 문서의 tmp install helper를 사용한다.
