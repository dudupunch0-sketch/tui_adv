# 천외편린 3택 보상 스키마 (Wuxia Cheonoe Pyeonrin Reward)

Status: design contract for the first runtime slice that opens the 천외편린 three-choice reward grammar.
Owner doc precedence: this file is the canonical contract for the reward selection surface. Runtime is still gated by `docs/dev/Development_Plan.md`, `docs/content/storypacks/wuxia_jianghu_pack.md`, and `docs/content/encounter_db/wuxia_jianghu_pack.md`. Notion upstream: `07. 천기록 / 천외편린 보상`.

## Purpose

`wuxia_cheonggi_record_first_fragment` already exists as a schema-less foreshadow: it shows three notebook phrases but only records `cheonggi_fragment_*_thread` flags as flavor. It deliberately does NOT open a reward schema.

This contract opens the first real **천외편린 3택 보상** (three-choice reward) selection grammar as runtime content. It formalizes the "present three candidate fragments, pick one, the other two vanish" loop from the storypack doc (`## 천기록 / 천외편린 규칙`) and Notion `07`.

## Hard rules (from Notion 07 + storypack doc)

1. 천기록 is not a search box. The player never types a question. The record presents candidates only when conditions are satisfied.
2. The offering presents **exactly three** candidate fragments. The player chooses **one**; the other two are **lost in that moment** (no take-two, no come-back-later for the same offering).
3. The reward is **not instant power**. A chosen fragment opens a 수련 방향 / 과제 / 해석 (training direction / task / interpretation), never a numeric stat boost in this slice.
4. Identity of 천기록 stays unrevealed. "정체 접근" is limited to sensing presence / gaze / real-time writing. No lore explanation, no recorder reveal.
5. Each candidate must belong to one documented 계열 (series): 현대 무술 / 훈련법 / 응급 처치 / 생존 전술 / 사고법 / 귀환 단서.

## Out of scope (still closed after this slice)

- Numeric ability stats, skill levels, or any combat resolver / HP numeric battle.
- A general reusable reward-table or random-draw system (this slice is a single hand-authored offering).
- Repeatable / multi-offering reward economy across the run.
- 귀환 단서 계열 candidate (it is a rare late-game series tied to return clues; this first offering uses combat/analysis/survival series only).
- Cheonggi Record identity reveal, `told_seoharin_truth`, return/settlement schema, relation/debt/faction ledger.

## Runtime contract

### Encounter

A single new encounter, id `wuxia_cheonoe_pyeonrin_first_reward` (천외편린 첫 보상).

- **Emergence timing** must match a documented 발현 moment: after a big battle / at a training-limit night. The implementation gates it after the 청류문 raid + midgame continuity (a "큰 전투 후 / 수련 한계에 부딪힌 밤" beat), reusing existing flags so first-eligible ordering does not preempt already-implemented encounters.
- **Conditions**: requires `cheonggi_record_awakened`, `first_fragment_seen`, and the chosen post-raid training-limit context flags already present in the bundle; forbids `cheonoe_pyeonrin_first_reward_resolved`. Exact flag set is chosen by implementation to keep every existing test green and to not steal eligibility from prior encounters.
- **Presentation**: `layout: fragment_choice`, `speaker: 천기록`, with a 천기록-style `glyph_anomaly` effect cue (three phrases surfacing, two fading). Body text states plainly this is not a search box and not instant power.

### The three candidates (pick-one-lose-two)

Exactly three choices, each a distinct 계열:

| choice 계열 | direction it opens | thread flag |
|---|---|---|
| 현대 무술 | 거리 조절 / 방어 자세 해석 (distance + guard reading) | `cheonoe_reward_modern_martial_thread` |
| 사고법 | 복기 / 원인 분석 (회사원 색깔의 review loop) | `cheonoe_reward_analysis_thread` |
| 생존 전술 | 다수전 회피 / 지형 활용 (escape + terrain) | `cheonoe_reward_survival_tactics_thread` |

Each choice outcome:

- adds the chosen thread flag, plus `cheonoe_pyeonrin_first_reward_resolved` and `cheonoe_pyeonrin_reward_schema_opened` (marks the reward grammar as opened for the run).
- adds clue `two_unchosen_fragments_lost` (records that the other two vanished) and a per-계열 interpretation clue.
- sets a `log` line framing the pick as a training direction, not a power gain.
- may apply small flavor resource costs (e.g. `sanity`), but no resource that represents a numeric ability.
- does NOT add any numeric stat, item that grants combat power, or route/faction/ledger flag.

There is **no fourth "decline" choice**: the grammar is strictly choose-one-of-three. (The earlier foreshadow encounter's "close without choice" option is specific to the foreshadow, not the reward grammar.)

### Mutual exclusivity

Choosing any one candidate sets `cheonoe_pyeonrin_first_reward_resolved`, which is the encounter's `forbidden_flags` guard, so the offering cannot be replayed and the unchosen two are permanently lost for this run.

## Test contract

- `content_bundle.rs` / `test_web_data_export.py` / `json_contract.rs`: encounter count, id list, and index references updated for the new encounter; its choice id list and key flags asserted.
- `cli_smoke.rs`: a playthrough reaches `wuxia_cheonoe_pyeonrin_first_reward`, picks one candidate, and asserts `cheonoe_pyeonrin_reward_schema_opened` + the chosen thread flag are set and the encounter is resolved.
- `test_docs_contract.py`: `test_wuxia_cheonoe_pyeonrin_first_reward_runtime_slice_is_docs_synced` mirrors the existing per-slice docs-sync tests.

## Docs to sync on implementation

`docs/dev/Checklist.md`, `docs/dev/Development_Plan.md`, `docs/dev/Notion_Design_Coverage.md` (mark Notion 07 reward grammar as first-runtime-opened; mark event DB row 19 fragment reward as now schema-opened), `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/content/storypacks/wuxia_jianghu_pack.md` (note the reward schema is now opened), `idea_box/notion_sources.yml`.
