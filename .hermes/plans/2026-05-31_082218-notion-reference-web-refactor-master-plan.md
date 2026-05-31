# Notion Reference Web-First Refactor Master Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task after the user approves the next slice. Planning-only artifact; do not treat this file as the canonical repo plan until its relevant parts are promoted into `docs/dev/Development_Plan.md`.

**Goal:** Refactor the project around the updated Notion reference as the primary system/design authority, preserve only the strongest existing repo concepts, and pivot the product to a Web-only PokéRogue-like browser player instead of a TUI/terminal product.

**Architecture:** Keep Rust GameCore + WASM as the gameplay truth boundary and Web/Vite as the production player. Replace the old office-apocalypse/generic-wuxia/TUI-oriented design with Notion-aligned storypack systems: `야근몽`, `이구학지 — 천기록`, 10-round goal cycles, weighted encounters, side-quest threads, companion interventions, ending-quality, and epilogue cards. Decommission TUI/SuperLightTUI/Python Textual as product surfaces after docs and tests make the Web path authoritative.

**Tech Stack:** Rust workspace (`crates/escape-core`, `crates/escape-wasm`), Web Vite/TypeScript (`web/`), generated content bundles, Notion API markdown references, existing Python content-export/test utilities only where still useful, Playwright/Vitest/Cargo/Pytest validation.

---

## 0. User Decisions Already Made

These are now project decisions and should be promoted into canonical repo docs early.

1. Notion reference is the default authority for system design.
   - Existing repo ideas may be reused only when they strengthen or cleanly support Notion reference.
   - If Notion and repo conflict, follow Notion unless the user explicitly overrides it.

2. Overall project direction is a full refactor, not a small patch.
   - The target is not “add Notion ideas to the current design.”
   - The target is “reshape the project around Notion reference.”

3. TUI basis can be removed.
   - SuperLightTUI / terminal-native horror edition is no longer a product target.
   - Old terminal/Python/Textual paths may remain temporarily as migration or parity scaffolding only, but docs must stop describing them as active direction.

4. Production play/deploy target is Web.
   - PokéRogue-like means: open URL, play immediately, browser client, static deploy, localStorage save, quick restart/replay loop.
   - This project remains a narrative choice/storybook game, not an action/Phaser clone.

5. Implementation should proceed one item at a time.
   - The agent should ask the user for intervention only at decision gates that materially change product scope, naming, or irreversible cleanup.

---

## 1. Reference Inputs Checked Before This Plan

### Notion reference dump

Local read-only dump used during audit:

- `/tmp/tui_adv_notion_audit/pages_summary.json`
- `/tmp/tui_adv_notion_audit/pages/root_escape.md`
- `/tmp/tui_adv_notion_audit/pages/storypack_system_old.md`
- `/tmp/tui_adv_notion_audit/pages/office_yageunmong_old.md`
- `/tmp/tui_adv_notion_audit/pages/wuxia_igu_hakji_old.md`
- `/tmp/tui_adv_notion_audit/pages/storypack_management_overview.md`
- `/tmp/tui_adv_notion_audit/pages/storypack_apply_rules.md`
- `/tmp/tui_adv_notion_audit/pages/exploration_overview.md`
- `/tmp/tui_adv_notion_audit/pages/goal_choice_system.md`
- `/tmp/tui_adv_notion_audit/pages/random_encounter_system.md`
- `/tmp/tui_adv_notion_audit/pages/side_quest_system.md`
- `/tmp/tui_adv_notion_audit/pages/joint_choices_companion.md`
- `/tmp/tui_adv_notion_audit/pages/stat_exploration_choice_rules.md`
- `/tmp/tui_adv_notion_audit/pages/stat_combat_choice_rules.md`
- `/tmp/tui_adv_notion_audit/pages/ending_system.md`
- `/tmp/tui_adv_notion_audit/pages/ending_system_overview.md`
- `/tmp/tui_adv_notion_audit/pages/main_story_branching_final_choice.md`
- `/tmp/tui_adv_notion_audit/pages/aftermath_card_system.md`
- `/tmp/tui_adv_notion_audit/pages/first_tengirok_fragment.md`

Important: future implementation should re-fetch Notion pages before finalizing docs, because `/tmp` dump is a planning snapshot.

### Repo references

Key current repo files identified:

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `pyproject.toml`
- `web/package.json`
- `docs/00_Index.md`
- `docs/01_Game_Overview.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Data_Schema.md`
- `docs/dev/Architecture.md`
- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- `docs/dev/Web_Distribution_Decision.md`
- `docs/design/Game_Loop.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/design/Combat_System_Auto_Brawl.md`
- `docs/design/Character_Stats_and_Generator.md`
- `docs/design/Player_State.md`
- `docs/design/Mobile_Pixel_Storybook_UI.md`
- `docs/design/TUI_Storybook_GlyphFX_Concept.md`
- `docs/design/UI_Rules.md`
- `docs/content/storypacks/README.md`
- `docs/content/storypacks/isolation_pack.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/isolation_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/Encounter_List.md`
- `docs/content/Ending_List.md`
- `src/tui_adv/data/*.yaml`
- `crates/escape-core/src/*`
- `crates/escape-wasm/src/lib.rs`
- `crates/escape-terminal/src/main.rs`
- `web/src/core/wasmRuntime.ts`
- `web/src/ui/storybook/render.ts`
- `web/src/ui/startScreen.ts`
- `web/src/data/generated/content.bundle.json`
- `tests/test_docs_contract.py`
- `tests/test_web_data_export.py`
- `tests/test_content_data.py`
- `tests/test_web_player_deployment_contract.py`
- `tests/test_web_visual_qa_contract.py`

---

## 2. High-Level Target Shape

### 2.1 Product identity

Target statement:

> A Web-first Korean narrative roguelite/storybook game where each run is structured by 10-round goal cycles, storypack-specific systems, weighted encounters, companion interventions, side-quest threads, and card-based epilogues. The first canonical storypacks are `야근몽` and `이구학지 — 천기록`.

Do not keep saying:

- “TUI 기반 게임” as the product identity.
- “office apocalypse” as the core engine identity.
- “wuxia_jianghu_pack” as canonical if it competes with `이구학지 — 천기록`.
- “SuperLightTUI terminal horror edition” as active target.

### 2.2 Runtime architecture

Target runtime after refactor:

```text
Notion reference / canonical docs
    ↓
Machine-readable content DB
    ↓
content.bundle.json
    ↓
Rust escape-core
    - run state
    - storypack state
    - goal-cycle scheduler
    - encounter selection
    - side-quest thread state
    - companion intervention resolver
    - ending/final-choice/epilogue-card evaluator
    ↓
escape-wasm JSON boundary
    ↓
Web player
    - PokéRogue-like URL play
    - storypack/run start
    - mobile portrait board
    - narrative body
    - choice buttons
    - HUD/status
    - companion/side-quest/fragment/epilogue surfaces
    - localStorage saves/archive
```

Transitional-only:

```text
Python package / YAML exporter / legacy TS mirror / terminal crate
```

These can temporarily support export, validation, migration, or parity checks, but should not remain user-facing product surfaces.

---

## 3. Existing Repo Ideas Worth Preserving

Use these unless they directly block Notion alignment.

1. Rust GameCore owns truth.
   - Keep gameplay truth in `crates/escape-core`.
   - Web renderer must not recalculate eligibility/outcomes/endings.

2. WASM JSON boundary.
   - Keep `crates/escape-wasm` and `web/src/core/wasmRuntime.ts` as the browser bridge.
   - Strengthen it for new state/schema rather than moving gameplay into TypeScript.

3. ScenePage/ActionResult separation.
   - Preserve `ScenePage` as renderer-neutral semantic page.
   - Expand it only with system-neutral fields needed by Notion systems.

4. Web Storybook/mobile board direction.
   - Preserve mobile portrait/pixel storybook board work.
   - Reframe it as Web player product UI, not fake terminal/TUI mood.

5. GlyphFX and transition/audio skeletons.
   - Keep them as Web mood layer where useful.
   - Re-theme from “terminal glitch” to storypack-specific anomalies: dream distortion, Cheonggi-record text reaction, fragment selection, corrupted final choices.

6. Static Web deployment contract.
   - Keep `web/package.json` scripts such as `build:player`, `preview:player`, WASM build/copy, visual QA.

7. Public-safe/private-safe boundary.
   - Preserve the guardrails for real-world/local hints.
   - Reinterpret old reality-link ideas as `야근몽` reality anchors / hidden records only if still desired.

8. Seed/localStorage/replay direction.
   - Useful for PokéRogue-like Web play.

9. Achievement/archive concepts.
   - Reuse if they support epilogue archive, ending archive, discovered card archive, or run history.

10. Auto-brawl + situation intervention.
   - This already aligns with Notion combat rules.
   - Expand with Notion stat-based intervention and companion joint choices.

---

## 4. Existing Repo Concepts To Demote Or Remove

1. TUI product direction.
   - Remove from product docs.
   - Defer deletion of code until Web path covers required validation.

2. `escape-terminal` as active product crate.
   - Remove from active docs first.
   - Later remove from Cargo workspace after tests/docs no longer depend on it.

3. Python/Textual runtime as product path.
   - Keep only if needed for content export or migration.
   - Later remove `textual` dependency and `src/tui_adv/tui/*`.

4. TypeScript mirror game core as gameplay implementation.
   - Keep only as fallback/parity while WASM migration is ongoing.
   - New gameplay rules should go into Rust, not TS mirror.

5. `office_apocalypse` / `isolation_pack` as canonical premise.
   - Mark as legacy/current runtime substrate or superseded placeholder.
   - Extract reusable concepts into `야근몽`: isolation feeling, office surfaces, reality hints, system pressure.

6. `wuxia_jianghu_pack` generic premise.
   - Mark as bootstrap placeholder superseded by `이구학지 — 천기록`.
   - Extract useful low-resolution wuxia-surface ideas only if they support Cheonggi/Qingliu/Heuksabang story.

7. Location-per-turn primary loop.
   - Demote to internal context/location tags.
   - Replace player-facing loop with 10-round goal cycle.

---

## 5. Decision Gates Requiring User Intervention

The agent should proceed independently until these points. At each gate, pause and ask the user.

### Gate A — Naming and repo/package identity

Question to ask before large renames:

- Keep repo/package names (`tui_adv`, `escape-office-web`, `escape-core`) as historical/internal names for now, or rename product-facing names soon?

Default if user does not care:

- Do not rename repo/package in early phases.
- Rename only product docs/UI labels first.
- Defer technical package rename until Web-first refactor is green.

### Gate B — First playable storypack after refactor

Question:

- Should the first Web refactor playable slice be `야근몽` or `이구학지 — 천기록`?

Default recommendation:

- Start with `야근몽` for Web MVP because current runtime content is office-based.
- Design `이구학지 — 천기록` in parallel as the first non-office storypack DB.

Alternative:

- If user wants to prove the new Notion systems faster, start with a tiny `이구학지` vertical slice: first Heuksabang fight → Qingliu rescue → first Cheonggi fragment.

### Gate C — Reality-link / ARG scope

Question:

- Keep the old reality-link hidden route as a product feature, or reduce it to dream/reality-anchor fiction only?

Default recommendation:

- Keep the design concept, but downgrade it from core MVP.
- Reframe as `야근몽` hidden/reality-anchor records.
- Keep all real locations private/local-only.

### Gate D — TUI deletion timing

Question:

- After docs pivot, should terminal/Python code be deleted aggressively, or only after Web/Rust tests replace its coverage?

Default recommendation:

- Two-step deprecation.
  1. Docs and plan stop presenting TUI as active.
  2. Remove code only when Web/Rust smoke coverage is sufficient.

### Gate E — Epilogue card schema timing

Question:

- Introduce epilogue card schema early, or keep first implementation text-backed and schema it later?

Default recommendation:

- Design schema early.
- Runtime implement a minimal card schema before large content migration, because Notion says epilogue cards are core reward, not a minor add-on.

### Gate F — Storypack selection UX

Question:

- Should the Web player expose storypack selection at start, or hardcode the first storypack until the second is playable?

Default recommendation:

- Show a storypack selector only after two storypacks have at least a playable slice.
- Before that, show locked/coming-soon cards if needed.

---

## 6. Phase Roadmap

### Phase 0 — Authority Reset And Planning Hygiene

**Objective:** Make repo docs acknowledge the new Notion/Web-first direction before touching gameplay code.

**Files likely to change:**

- `AGENTS.md`
- `README.md`
- `docs/00_Index.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Architecture.md`
- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- `docs/dev/Data_Schema.md`
- `idea_box/BACKLOG_ORDER.md`
- `idea_box/inbox/*.md`
- possibly new: `docs/reference/notion/README.md`
- possibly new: `docs/reference/notion/*.md` or `docs/reference/notion/manifest.md`

**Tasks:**

#### Task 0.1 — Re-fetch Notion reference

- Use Notion API `/search` and `/pages/{id}/markdown`.
- Refresh all relevant pages into a scratch or explicit repo reference location.
- If saving Notion markdown in repo, decide whether it is reference cache or design docs.
- Do not mark idea_box entries done just because pages were imported.

Verification:

- List page titles and `last_edited_time`.
- Confirm the following groups are present:
  - storypack system
  - 야근몽
  - 이구학지 — 천기록
  - 탐색 시스템
  - 목표 선택
  - 랜덤 인카운터
  - 사이드 퀘스트
  - 동료/합동 선택지
  - 스탯별 탐색/전투 선택지
  - 엔딩 시스템
  - 후일담 카드
  - 천기록 첫 천외편린

#### Task 0.2 — Update canonical plan top-level decision

Modify `docs/dev/Development_Plan.md`:

- Add a top “2026-05-31 pivot decision” section.
- State that Notion reference is system authority.
- State Web-only/PokéRogue-like player is product target.
- State TUI/SuperLightTUI is deprecated as product direction.
- Keep Rust GameCore/WASM/Web as target runtime.
- Move old current-priority items into “legacy / before pivot / historical status” if they conflict.

Verification:

- Search for active claims that terminal/SuperLightTUI is primary.
- They should either be removed or marked historical.

#### Task 0.3 — Update AGENTS.md project instructions

Modify `AGENTS.md`:

- Replace current “TUI/fake-terminal + Web Storybook + SuperLightTUI” active direction.
- New active direction: Web-only static player, Rust GameCore/WASM truth, Notion-first systems.
- Add instruction: if Notion reference conflicts with repo docs, follow Notion unless user overrides.
- Add instruction: terminal/TUI code is migration-only and not new feature target.

Verification:

- `search_files("SuperLightTUI|TUI/fake-terminal|terminal-native", path="AGENTS.md")`
- Remaining references must be deprecation/history-only.

#### Task 0.4 — Update docs index and README entry points

Modify:

- `README.md`
- `docs/00_Index.md`

Required changes:

- Product one-liner becomes Web-first storypack narrative roguelite/storybook.
- Remove “TUI 기반” as product identity.
- Link canonical Notion-refactor docs once created.
- Mark Python/Textual and terminal docs as legacy/migration if still listed.

Verification:

- README first 30 lines should no longer say terminal/TUI is the main product.
- `docs/00_Index.md` should list Web player docs and Notion-aligned design docs as current.

#### Task 0.5 — Normalize idea_box state

Modify:

- `idea_box/BACKLOG_ORDER.md`
- possibly `idea_box/inbox/2026-05-29-*.md`

Required changes:

- Keep existing 3 Notion-origin entries open until actual design sync/reference comparison is done.
- Add notes that 2026-05-30/31 Notion system pages expand those entries or create grouped follow-up entries.
- Do not explode every Notion page into dozens of standalone backlog items unless user wants that management style.

Recommended grouped tracking:

1. `storypack-identity-sync`
2. `exploration-loop-sync`
3. `ending-companion-reward-sync`
4. `web-only-product-pivot`

Verification:

- No `done` status without actual reflection/reference-check evidence.

**Exit criteria for Phase 0:**

- Canonical plan and agent instructions no longer contradict the user’s pivot.
- No code implementation yet.
- Next phase can safely edit design docs without agents being pulled back to old TUI goals.

---

### Phase 1 — Canonical Design Rebase Around Notion Systems

**Objective:** Rewrite the design-contract layer so Notion systems are represented in repo docs before schema/code changes.

**Files likely to change:**

- `docs/design/Game_Loop.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/design/Player_State.md`
- `docs/design/Combat_System_Auto_Brawl.md`
- `docs/design/Character_Stats_and_Generator.md`
- `docs/content/storypacks/README.md`
- new: `docs/design/Goal_Cycle_System.md`
- new: `docs/design/Random_Encounter_System.md`
- new: `docs/design/Side_Quest_System.md`
- new: `docs/design/Companion_Intervention_System.md`
- new: `docs/design/Ending_Epilogue_System.md`
- new: `docs/design/Fragment_Reward_System.md`

**Tasks:**

#### Task 1.1 — Replace Game Loop with 10-round goal cycle

Update or split from `docs/design/Game_Loop.md`.

Canonical model:

```text
Goal selection
↓
10-round goal cycle
  1: main goal entry
  2: main goal deepening / companion exchange
  3-9: weighted random encounter slots
  10: main goal settlement
↓
side quest / relation / resource / epilogue-debt updates
↓
next goal selection
```

Design details to capture:

- Goals are purpose-centered, not location names.
- Default UI offers 3 base goals + 1 conditional goal.
- Conditional goal is pressure/opportunity, not forced.
- Locations remain context/tags, not primary player-facing loop.
- Random slots guarantee at least 1 current-goal-related encounter.

Verification:

- Search docs for “매 턴 어디로 이동” or equivalent active loop claims.
- They should be historical/legacy or removed.

#### Task 1.2 — Define storypack identity model

Update `Storypack_World_Model.md`.

Canonical storypacks:

- `office_yageunmong`
  - title: `야근몽`
  - goal: wake from lucid office nightmare
  - special systems: lucidity, dream stability, reality anchors, awakening fragments, leaving-work gate

- `wuxia_igu_hakji_cheonggi`
  - title: `이구학지 — 천기록`
  - goal: survive, settle/return/choose faction, understand Cheonggi record
  - special systems: Cheonggi record, heavenly fragments, Qingliu sect, Seo Harin, Heuksabang, heart-demon/corruption

Legacy storypacks:

- `isolation_pack`: superseded/absorbed into `야근몽` unless preserved as a sub-route.
- `wuxia_jianghu_pack`: superseded bootstrap for `이구학지 — 천기록`.

Verification:

- `wuxia_jianghu_pack` docs must not remain an equal active canonical storypack.
- `office_apocalypse` must not remain product identity.

#### Task 1.3 — Define random encounter scheduler

Create/update `docs/design/Random_Encounter_System.md`.

Required sections:

- categories:
  - progress
  - danger
  - relation
  - resource
  - state
  - rare fragment / 기연
  - side seed
  - side follow-up
- slot budget for rounds 3-9:
  - main-goal related: minimum 1
  - side quest seeds: 0-2
  - active side follow-ups: 0-2
  - companion events: 0-2
  - danger encounters: 0-2
  - rare fragment encounters: 0-1
  - state/resource/environment as needed
- priority order:
  - forced main story
  - active side follow-up
  - current-goal minimum guarantee
  - companion state/relation
  - danger/environment
  - resource/recovery
  - new side seed
  - rare fragment
- weight formula:
  - base_weight
  - goal_multiplier
  - player_state_multiplier
  - companion_multiplier
  - danger_multiplier
  - story_multiplier
  - history_multiplier
  - flag_multiplier
  - side_quest_multiplier

Verification:

- Existing `Storypack_Encounter_DB.md` references should point here instead of duplicating incomplete rules.

#### Task 1.4 — Define side quest thread model

Create/update `docs/design/Side_Quest_System.md`.

Required state model:

```text
undiscovered
seeded
contacted
accepted
active
followup_in_progress
paused
promoted_to_conditional_goal
completed
failed
mutated
reflected_in_epilogue
```

Required constraints:

- small clues: many allowed
- active side quests: max 3
- major side quest: max 1
- player-facing UI should not show internal state names directly
- side quests can create companions, rare fragments, items, route changes, enemies, ending conditions, epilogue debts

Verification:

- Current encounter card schema should gain planned hooks for side quest thread, follow-up group, epilogue debt.

#### Task 1.5 — Define companion intervention system

Create `docs/design/Companion_Intervention_System.md`.

Required intervention types:

- automatic intervention
- choice intervention
- crisis intervention
- advice/opinion
- opposition
- joint action
- unilateral action

Required state:

- trust
- conflict
- fear
- bond
- fatigue
- injury
- intervention cooldown
- companion role/stat

Important design rule:

- Companion does not play for the player.
- Opposition/unilateral action must feel like relationship consequence, not random punishment.

Verification:

- `Character_Stats_and_Generator.md` should point to this for runtime companion usage.

#### Task 1.6 — Define player/storypack state layer

Update `Player_State.md` or create a new `Storypack_State.md`.

Keep generic resources where useful:

- health
- sanity/focus
- battery/tool charge if still relevant
- hunger/thirst if the storypack needs survival pressure

Add storypack-specific layer:

`야근몽`:

- lucidity / 자각도
- dream_stability / 꿈 안정도
- reality_anchor facts
- awakening_fragment facts
- leaving_gate readiness
- nightmare_pressure / responsibility_debt

`이구학지 — 천기록`:

- qi_blood / 기혈
- heart_demon / 심마
- cheonggi_corruption
- heavenly_fragment facts
- qingliu_reputation
- heuksabang_pressure
- modern_self_integrity

Verification:

- Do not pretend all Notion states are just display aliases for existing `sanity`.
- State explicitly which are new future schema fields vs display aliases.

#### Task 1.7 — Define ending/final-choice/epilogue card design

Create `docs/design/Ending_Epilogue_System.md`.

Required model:

- main ending type:
  - failure
  - escape
  - return
  - settlement
  - truth
  - conquest
  - sacrifice
  - corruption
  - hidden
- ending quality internal only:
  - true
  - good
  - normal
  - bitter
  - bad
  - collapse
- final choice states:
  - available
  - unavailable_hidden
  - unavailable_probe
  - distorted
  - forced
- epilogue cards:
  - category
  - subjects
  - illustration_hint
  - unlock_condition
  - order_bucket
  - priority
  - exclusive_group
  - linked_group
  - archive_visibility

Required UX rule:

- Do not show `GOOD END`, `BAD END`, `TRUE END` labels directly.
- The player interprets ending meaning through title and epilogue cards.

Verification:

- `docs/content/Ending_List.md` should be clearly marked as current runtime/legacy list until migrated.

#### Task 1.8 — Define fragment reward system

Create `docs/design/Fragment_Reward_System.md`.

Canonical notion:

- `각성편린` for `야근몽`
- `천외편린` for `이구학지 — 천기록`

Rules:

- Triggered by event/observation/crisis conditions.
- Present 3 candidates.
- Player selects exactly 1.
- Not a search box or generic reward list.
- Wrong/unsupported fragment can create side effects.

Cheonggi first fragment example:

```text
condition fulfilled
↓
Cheonggi record reacts
↓
3 modern-knowledge candidates
↓
player chooses one
↓
chosen fragment affects martial application or situation resolution
```

Verification:

- Link to combat/stat/side-quest docs where fragments may be triggered.

**Exit criteria for Phase 1:**

- Design docs fully reflect Notion systems.
- Old design docs are either updated or marked legacy.
- No runtime schema/code changes yet unless separately approved.

---

### Phase 2 — Content Taxonomy And Machine-Readable Schema Proposal

**Objective:** Convert the new design into a concrete content schema before modifying Rust runtime deeply.

**Files likely to change:**

- `docs/dev/Data_Schema.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/content/storypacks/README.md`
- `docs/content/storypacks/*.md`
- `docs/content/encounter_db/*.md`
- possible new data files:
  - `content/storypacks.yaml` or `src/tui_adv/data/storypacks.yaml`
  - `content/goals.yaml` or `src/tui_adv/data/goals.yaml`
  - `content/side_quests.yaml`
  - `content/companions.yaml`
  - `content/epilogue_cards.yaml`
  - `content/fragments.yaml`

Important naming decision:

- Avoid committing to `src/tui_adv/data/*` if Python package is being retired.
- Prefer a future-neutral path such as `content/*.yaml` or `data/*.yaml` if doing a larger cleanup.
- Ask Gate A if this implies package/repo renaming.

**Tasks:**

#### Task 2.1 — Define content DB root layout

Proposed target:

```text
content/
  storypacks.yaml
  goals.yaml
  encounters.yaml
  side_quests.yaml
  companions.yaml
  endings.yaml
  epilogue_cards.yaml
  fragments.yaml
  items.yaml
  achievements.yaml
```

Alternative transitional target:

```text
src/tui_adv/data/
  storypacks.yaml
  goals.yaml
  encounters.yaml
  ...
```

Default recommendation:

- Use `content/` as a new product-neutral root.
- Keep old `src/tui_adv/data/*.yaml` until exporter/runtime is migrated.

User intervention:

- Ask only if the user wants a full technical package rename immediately.

#### Task 2.2 — Storypack schema

Proposed fields:

```yaml
storypacks:
  - id: office_yageunmong
    title: 야근몽
    status: canonical
    source_refs:
      - notion:<page_id>
    start_profile: office_dream_start
    core_loop: ten_round_goal_cycle
    state_model: office_dream
    base_goals: []
    conditional_goal_sources: []
    special_systems:
      - lucidity
      - dream_stability
      - reality_anchor
      - awakening_fragment
      - leaving_work_gate
    main_ending_types: []
    epilogue_card_sets: []
```

Verification:

- Validate unique storypack ids.
- Validate source refs exist in Notion manifest.

#### Task 2.3 — Goal schema

Proposed fields:

```yaml
goals:
  - id: office_find_reality_anchor
    storypack_id: office_yageunmong
    label: 현실 앵커를 찾는다
    type: base
    weight_tags: [truth, anchor]
    entry_encounter_tags: []
    deepening_encounter_tags: []
    settlement_rules: []
```

Rules:

- Every storypack must have at least 3 base goals.
- Conditional goals must have unlock conditions.
- Goals are not location IDs.

Verification:

- Test that no goal label is a bare location name.

#### Task 2.4 — Encounter schema extension

Proposed fields:

```yaml
encounters:
  - id: heuksabang_first_fight
    storypack_id: wuxia_igu_hakji_cheonggi
    phase: opening
    categories: [danger, progress]
    round_roles: [main_entry, random_slot]
    goal_tags: [survive_first_contact]
    location_tags: [market, street]
    companion_hooks: []
    side_quest_hooks: []
    fragment_hooks: []
    epilogue_debt_hooks: []
    repeat_policy: once
    base_weight: 100
    conditions: {}
    presentation: {}
    choices: []
```

Verification:

- Existing encounter validation expands to storypack/goal/category/repeat policy.

#### Task 2.5 — Side quest schema

Proposed fields:

```yaml
side_quests:
  - id: wuxia_treasure_map_thread
    storypack_id: wuxia_igu_hakji_cheonggi
    tier: active
    max_importance: active
    seed_encounters: []
    followup_encounters: []
    conditional_goal: null
    outcomes:
      completed: []
      failed: []
      mutated: []
    epilogue_cards: []
```

Verification:

- Max 3 active side quests and max 1 major side quest enforced in runtime later.
- Design-time validator checks references.

#### Task 2.6 — Companion schema

Proposed fields:

```yaml
companions:
  - id: seo_harin
    storypack_id: wuxia_igu_hakji_cheonggi
    name: 서하린
    role_tags: [protector, qingliu_contact]
    stats:
      logic: 2
      empathy: 3
      volition: 4
      composure: 4
      interface: 0
      physical: 5
    relationship_defaults:
      trust: 0
      conflict: 0
      fear: 0
      bond: 0
    intervention_rules: []
    epilogue_exclusive_group: companion.seoharin.fate
```

Open design issue:

- Existing repo NPC stat model has sense/social/reason/self/impulse/body 0-20.
- Notion player stat model uses logic/empathy/volition/composure/interface/physical.
- Need decide whether companion stats use player stat names or old NPC stat names.

Default recommendation:

- Use the same six stat names as player-facing choice generation for runtime companions.
- Keep old 0-20 generator only as design aid if useful.

#### Task 2.7 — Ending and epilogue schema

Proposed fields:

```yaml
endings:
  - id: yageunmong_wake_by_rejecting_approval
    storypack_id: office_yageunmong
    main_ending_type: escape
    quality_rules: []
    final_choice_state_rules: []
    title: 승인 없이 퇴근한다
    text: ...
```

```yaml
epilogue_cards:
  - id: qingliu_and_seoharin_fate
    storypack_id: wuxia_igu_hakji_cheonggi
    title: 청류문과 서하린
    category: companion
    subjects: [qingliu_sect, seo_harin]
    illustration_hint: ...
    text: ...
    unlock_condition: {}
    order_bucket: companion
    priority: 50
    exclusive_group: companion.seoharin.fate
    linked_group: qingliu_fate
    archive_visibility:
      undiscovered_display: ????
```

Verification:

- Every exclusive group resolves to one highest-priority card if multiple unlock.
- Cards can be skipped in UI but remain in archive.

#### Task 2.8 — Fragment schema

Proposed fields:

```yaml
fragment_events:
  - id: cheonggi_first_fragment
    storypack_id: wuxia_igu_hakji_cheonggi
    trigger_conditions: {}
    candidates:
      - id: boxing_footwork
        title: 복싱 풋워크
        effects: []
        risks: []
      - id: commuter_crowd_movement
        title: 출근길 인파 피하기
        effects: []
        risks: []
      - id: industrial_safety_flow
        title: 산업안전 작업동선
        effects: []
        risks: []
    choose_exactly: 1
```

Verification:

- Exactly 3 candidates unless explicitly overridden.
- Exactly 1 selected.

**Exit criteria for Phase 2:**

- Data schema doc contains enough detail for implementation.
- Validator test plan is written.
- User has answered any naming/stat-scale gates if needed.

---

### Phase 3 — Web-First Architecture Cleanup Plan

**Objective:** Make Web/Rust/WASM the only active runtime path and plan TUI/Python retirement safely.

**Files likely to change:**

- `Cargo.toml`
- `crates/escape-terminal/*`
- `pyproject.toml`
- `src/tui_adv/*`
- `scripts/export_web_data.py`
- `web/package.json`
- `web/src/core/*`
- `web/src/ui/*`
- `tests/*`
- `docs/dev/Architecture.md`
- `docs/dev/Data_Schema.md`
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- `docs/dev/Web_Distribution_Decision.md`

**Tasks:**

#### Task 3.1 — Classify runtime components

Create a table in architecture docs:

| Component | Current role | Future role |
|---|---|---|
| `crates/escape-core` | gameplay core | keep/expand |
| `crates/escape-wasm` | web bridge | keep/expand |
| `web/` | web player | production player |
| `crates/escape-terminal` | terminal renderer | deprecate/remove |
| `src/tui_adv/tui/*` | Python Textual renderer | deprecate/remove |
| `src/tui_adv/game/*` | old Python game mirror/export source | migrate/retire |
| `scripts/export_web_data.py` | bundle export | keep until replaced by Rust/neutral exporter |
| `web/src/game/*` | TS mirror fallback | freeze/remove after WASM required |

Verification:

- Architecture docs distinguish product vs transition clearly.

#### Task 3.2 — Web scripts become primary commands

Keep/confirm:

```bash
cd web
npm run build:player
npm run preview:player
npm test
npm run qa:storybook:visual
```

Possible future scripts:

```json
"dev:player": "npm run wasm:build && vite --host 127.0.0.1",
"test:player": "vitest run && npm run qa:storybook:visual -- --require-wasm",
"check:player": "npm test && npm run build:player"
```

Verification:

- README quickstart uses Web commands first.
- Terminal commands are not presented as player launch path.

#### Task 3.3 — Decommission terminal crate in two stages

Stage 1 docs-only:

- Mark `crates/escape-terminal` as deprecated/migration-only.
- Stop adding new tests against terminal UI.

Stage 2 code removal after Web coverage:

- Remove `crates/escape-terminal` from workspace members in `Cargo.toml`.
- Delete or archive `crates/escape-terminal` if user approves aggressive cleanup.
- Remove tests referencing terminal smoke.
- Remove docs references to terminal verification.

Verification:

- `cargo test -p escape-core`
- `cargo test -p escape-wasm`
- no workspace member errors.

User intervention:

- Ask before deleting the crate, not before marking docs deprecated.

#### Task 3.4 — Decommission Python/Textual in two stages

Stage 1:

- Remove Textual from product docs.
- Keep Python only for data export/tests if still required.

Stage 2:

- Move content source from `src/tui_adv/data` to product-neutral `content/` if chosen.
- Replace Python content loader dependency with Rust or neutral exporter.
- Remove `textual` dependency from `pyproject.toml`.
- Remove `src/tui_adv/tui/*` and TUI tests.
- Rename package later only if user approves.

Verification:

- `python -m pytest tests/test_web_data_export.py tests/test_content_data.py -q` still passes during transition.
- Later, remove obsolete Python tests or rewrite as content validator tests.

User intervention:

- Ask before package/data-root rename.

#### Task 3.5 — Freeze or remove TypeScript mirror gameplay

Current files:

- `web/src/game/actions.ts`
- `web/src/game/conditions.ts`
- `web/src/game/endings.ts`
- `web/src/game/outcomes.ts`
- `web/src/game/state.ts`
- etc.

Plan:

- Confirm WASM path is required for production build.
- Keep TS mirror only for tests that convert legacy TurnView to ScenePage if still needed.
- Stop implementing new Notion systems in TS mirror.
- Eventually delete TS mirror or reduce it to type adapters/error fallback.

Verification:

- Web tests assert new gameplay comes from `wasmRuntime`/Rust contract.
- `VITE_REQUIRE_WASM` behavior remains fatal for production if configured.

**Exit criteria for Phase 3:**

- Product docs and commands are Web-first.
- TUI/Python/terminal are marked transitional or removed according to user decisions.
- New gameplay implementation path is unambiguous.

---

### Phase 4 — Rust Core System Refactor

**Objective:** Implement Notion systems in Rust core behind tests before Web UI polish.

**Files likely to change:**

- `crates/escape-core/src/state.rs`
- `crates/escape-core/src/content.rs`
- `crates/escape-core/src/turn.rs`
- `crates/escape-core/src/scene_page.rs`
- `crates/escape-core/src/effects.rs`
- `crates/escape-core/src/save.rs`
- `crates/escape-core/tests/content_bundle.rs`
- `crates/escape-core/tests/core_contract.rs`
- `crates/escape-core/tests/route_parity.rs`
- `crates/escape-wasm/src/lib.rs`
- `crates/escape-wasm/tests/json_contract.rs`

**Implementation sequence:**

#### Task 4.1 — Add storypack identity to content and state

Add to content model:

- storypacks map
- active storypack id
- storypack state model metadata

Add to state:

- `storypack_id`
- `run_phase` or equivalent
- storypack-specific state container placeholder

Tests:

- new game defaults to selected storypack.
- invalid storypack id is rejected user-facing.
- ScenePage includes storypack title/theme info semantically.

#### Task 4.2 — Add goal-cycle state

State fields:

- current_goal_id
- goal_cycle_round: 1-10
- completed_goal_cycles
- conditional_goal_candidates
- cycle_history

Core functions:

- `available_goals(state, content)`
- `select_goal(state, content, goal_id)`
- `advance_goal_round(...)`
- `settle_goal_cycle(...)`

Tests:

- new run starts at goal selection page.
- selecting a goal enters round 1.
- rounds advance to 10 and return to goal selection.
- conditional goals appear only when unlocked.

#### Task 4.3 — Add encounter scheduler for 10-round cycle

Core functions:

- `eligible_encounters_for_round(state, content)`
- `select_weighted_encounter(state, content, rng)`
- `enforce_goal_related_minimum(state, content)`

Tests:

- round 1 returns main goal entry encounter when present.
- round 2 prefers deepening/companion exchange.
- rounds 3-9 use random slot scheduler.
- at least one random slot per cycle is goal-related.
- repeated encounters obey repeat policy.

#### Task 4.4 — Add side quest thread state

State:

- `side_quests: map<id, SideQuestState>`
- active count
- major active id optional
- unresolved debts

Core behavior:

- seed side quest from encounter outcome.
- schedule follow-up in random slots.
- promote to conditional goal.
- resolve complete/fail/mutate.
- record epilogue debt.

Tests:

- cannot exceed max 3 active side quests.
- cannot exceed max 1 major side quest.
- ignored side quest can create unresolved debt without direct UI warning.

#### Task 4.5 — Add companion state/intervention hooks

State:

- companion records per run
- trust/conflict/fear/bond/fatigue/injury/cooldowns

Core behavior:

- advice text as ScenePage cue.
- companion choice actions.
- crisis intervention result.
- joint choice availability.
- opposition/unilateral behavior as authored event, not arbitrary random mutation.

Tests:

- high trust enables joint choice.
- conflict can add warning/opposition cue.
- crisis intervention has cost/cooldown.
- renderer receives action ids, not logic.

#### Task 4.6 — Add storypack-specific states

Implement first as tagged generic maps if needed, but with typed accessors where possible.

Office/yageunmong:

- lucidity
- dream_stability
- reality anchors
- awakening fragments
- leaving gate readiness

Wuxia:

- qi_blood
- heart_demon
- cheonggi_corruption
- heavenly fragments
- Qingliu reputation
- Heuksabang pressure

Tests:

- fragment event changes appropriate state.
- dream stability can affect final choice distortion.
- Cheonggi corruption can affect ending/corruption path.

#### Task 4.7 — Add fragment selection scene

Core behavior:

- trigger fragment event.
- create ScenePage mode for fragment selection.
- present exactly 3 candidates.
- candidate action id selects one.
- selected fragment applies effects and records history.

Tests:

- exactly one fragment can be chosen.
- stale fragment action rejected.
- chosen fragment appears in state/history.

#### Task 4.8 — Add final choice state and ending quality

Core behavior:

- determine available final choices.
- represent unavailable_hidden/unavailable_probe/distorted/forced.
- choosing unavailable_probe returns monologue then same final choice scene without resource/turn loss.
- choosing distorted/forced can route to corruption/collapse.
- compute main_ending_type and internal quality.

Tests:

- hidden choices not shown.
- probe choice shown but non-final.
- low state/corruption distorts choice label/effect.
- quality is internal and not displayed as GOOD/BAD.

#### Task 4.9 — Add epilogue card evaluator

Core behavior:

- after ending, evaluate card unlock conditions.
- enforce exclusive_group priority.
- preserve linked_group semantics.
- order cards by order_bucket/priority.
- include archive visibility metadata.

Tests:

- all qualifying non-exclusive cards output.
- exclusive group returns one card.
- unresolved debt can unlock rumor/debt card.
- cards serialize through WASM JSON.

#### Task 4.10 — Save schema migration

Update save envelope.

Requirements:

- schema_version bump.
- old saves rejected or migrated deliberately.
- no browser bricking on stale save.
- localStorage summary can show storypack, goal cycle, timestamp.

Tests:

- old save behavior explicit.
- new state round-trip.
- invalid storypack/goal/sidequest ids rejected safely.

**Exit criteria for Phase 4:**

- Rust core can run a minimal Notion-style goal cycle and ending/epilogue flow headlessly.
- WASM JSON contract exposes it.
- Web UI can consume it without implementing gameplay rules.

---

### Phase 5 — Web Player Refactor

**Objective:** Make the Web player express the new Notion systems and PokéRogue-like run UX.

**Files likely to change:**

- `web/src/main.ts`
- `web/src/core/wasmRuntime.ts`
- `web/src/core/types.ts`
- `web/src/ui/startScreen.ts`
- `web/src/ui/storybook/render.ts`
- `web/src/ui/storybook/html.ts`
- `web/src/ui/storybook/history.ts`
- `web/src/ui/storybook/visualCatalog.ts`
- `web/src/ui/render.ts`
- `web/src/ui/settings/playerSettings.ts`
- `web/src/ui/keyboard.ts`
- `web/src/effects/glyphfx.ts`
- `web/src/ui/audio/audioEngine.ts`
- `web/src/ui/motion/transitionController.ts`
- tests under `web/src/**/*.test.ts`
- `web/scripts/storybook-reference-qa.mjs`

**Tasks:**

#### Task 5.1 — Update Web type contract

Update TypeScript types to match new ScenePage modes:

- goal_selection
- goal_cycle_scene
- encounter
- fragment_selection
- final_choice
- ending
- epilogue_cards
- archive

Tests:

- parse fixture JSON from Rust/WASM.
- unknown modes render safe fallback.

#### Task 5.2 — Start screen: Web run launcher

Target UX:

- title/logo
- new run
- continue
- storypack display/selection if available
- seed display/input or random seed
- settings: motion/audio
- archive access

PokéRogue-like principle:

- no install
- no terminal
- URL starts playable client
- localStorage guest save

Tests:

- continue disabled when no save.
- corrupt save shows recover/reset option.
- new run invokes WASM storypack start.

#### Task 5.3 — Goal selection UI

Render:

- 3 base goals
- 1 conditional goal slot if available
- goal descriptions
- current storypack state summary
- not location list

Tests:

- clicking goal sends action id from ScenePage.
- renderer does not compute eligibility.

#### Task 5.4 — Goal-cycle HUD

Show:

- current storypack
- current goal
- round 1-10 progress
- relevant state bars/badges:
  - office: 자각도, 꿈 안정도, 현실 앵커
  - wuxia: 기혈, 심마, 청류문 평판, 흑사방 압박
- companion compact state if present
- side quest hints/debts only as diegetic clues, not raw internal state.

Tests:

- HUD adapts to storypack semantic status fields.
- missing optional fields do not crash.

#### Task 5.5 — Encounter/story scene renderer

Render body and choices in storybook style.

Preserve:

- large readable Korean text.
- mobile portrait board.
- sentence-like choices.
- semantic visual/effect cues.

Remove/demote:

- terminal prompt metaphors where they no longer fit.
- fake shell command feel as primary UX.

Tests:

- choice button calls exact action id.
- long Korean text remains readable in viewport QA.

#### Task 5.6 — Fragment selection UI

Render:

- special card/screen for awakening/heavenly fragments.
- exactly 3 candidate cards.
- storypack-specific copy:
  - `각성편린` for `야근몽`
  - `천외편린` / `천기록 반응` for wuxia
- clear “choose one” affordance.

Tests:

- exactly one click submits action.
- candidate metadata displayed without leaking hidden outcome if not desired.

#### Task 5.7 — Companion intervention UI

Render:

- advice before choices.
- companion choice rows.
- joint action labels.
- opposition warnings.
- crisis intervention result callout.

Tests:

- companion advice does not become selectable unless core provides action.
- joint action uses core action id.

#### Task 5.8 — Final choice UI

Render final choices with states:

- available: normal selectable
- unavailable_hidden: absent
- unavailable_probe: visible with uncertain style; selecting returns monologue
- distorted: visibly corrupted/altered label
- forced: intrusive/overriding style

Tests:

- no GOOD/BAD/TRUE labels.
- distorted/forced style from semantic state, not local rule.

#### Task 5.9 — Ending and epilogue card UI

Render:

- main ending title/text.
- card sequence.
- skip current card.
- skip all.
- archive save/replay.
- illustration_hint placeholder or visual catalog mapping.

Tests:

- all cards can be browsed.
- exclusive-group result count matches Rust output.
- archive persists discovered ending/card ids.

#### Task 5.10 — Visual QA update

Update `web/scripts/storybook-reference-qa.mjs` to check:

- start screen
- goal selection
- encounter page
- fragment selection
- final choice page
- epilogue card page
- mobile portrait viewport
- reduced motion
- WASM required mode

Verification:

```bash
cd web
npm test
npm run build:player
npm run qa:storybook:visual -- --require-wasm
```

Use tmp policy for installs/builds on this host.

**Exit criteria for Phase 5:**

- Browser player can start, select a goal, resolve encounters, select fragment, reach final choice, view epilogue cards.
- No terminal path needed for user play.

---

### Phase 6 — Content Migration: `야근몽`

**Objective:** Convert existing office runtime/content into Notion-aligned `야근몽` without losing good existing ideas.

**Files likely to change:**

- storypack data/content files chosen in Phase 2
- `docs/content/storypacks/office_yageunmong.md` or equivalent
- `docs/content/encounter_db/office_yageunmong.md`
- `docs/story/Story.md`
- `docs/story/Reality_Link.md`
- `docs/content/Encounter_List.md`
- `docs/content/Ending_List.md`
- old `src/tui_adv/data/*.yaml` during transition

**Tasks:**

#### Task 6.1 — Create canonical `야근몽` storypack record

Include:

- start: late-night desk nap / lucid office nightmare
- core goal: wake up, not finish work
- tone: black comedy → office ghost story → psychological horror → recovery
- key systems: lucidity, dream stability, reality anchors, awakening fragments, leaving-work gate
- enemies: Kim manager, endless-report Park lead, non-approving team lead, unread-mail ghost, forgotten-off-work self

Verification:

- No active claim that objective is merely office escape or system conquest.

#### Task 6.2 — Map existing office content to `야근몽`

Existing reusable ideas:

- messenger from ex-employee → dream messenger / dead project message
- printer anomaly → reality anchor / dream contradiction
- meeting room all-hands → repeated nightmare meeting
- security/CCTV → delayed dream evidence
- emergency stairs escape → leaving-work gate candidate
- server/conquest routes → nightmare structure/truth/corruption routes
- reality hidden hints → reality anchor/hidden record

Classify each existing encounter:

- keep as-is with copy change
- rewrite as dream logic
- demote to legacy
- remove

Verification:

- Create a migration table in docs before code changes.

#### Task 6.3 — Define first `야근몽` playable vertical slice

Recommended slice:

1. Start at office desk after nap.
2. Goal selection:
   - 현실 앵커를 찾는다
   - 갇힌 동료를 찾는다
   - 퇴근 게이트 단서를 찾는다
   - conditional: 김과장의 회의 요청
3. One 10-round cycle with simplified rounds.
4. One awakening fragment event.
5. Final mini-choice or cycle settlement.
6. One ending/epilogue card stub.

Tests:

- goal selection available.
- one chosen goal advances through a minimal cycle.
- fragment event chooses one of three.
- epilogue card appears after ending.

#### Task 6.4 — Migrate endings

Map old ending types:

- `escape_commute` → wake/leave-work gate escape variant
- `truth_isolation_protocol` → nightmare structure/truth variant
- `conquest_*` → nightmare control/corruption/conquest variants, if still desired
- `hidden_reality_hint_*` → hidden/reality-anchor records
- `game_over_*` → failure/collapse variants

Verification:

- No player-facing GOOD/BAD labels.
- Epilogue cards carry consequences.

**Exit criteria for Phase 6:**

- `야근몽` is first coherent Web-playable storypack slice.
- Existing best office ideas are preserved but reinterpreted through Notion.

---

### Phase 7 — Content Migration: `이구학지 — 천기록`

**Objective:** Replace generic wuxia placeholder with Notion canonical wuxia storypack.

**Files likely to change:**

- `docs/content/storypacks/wuxia_jianghu_pack.md` or replacement
- `docs/content/encounter_db/wuxia_jianghu_pack.md` or replacement
- new canonical content files for `wuxia_igu_hakji_cheonggi`
- storypack/goal/encounter/fragment/companion data

**Tasks:**

#### Task 7.1 — Create canonical storypack record

Include:

- title: 이구학지 — 천기록
- premise: modern office worker, own body, commute/work clothes, transferred to wuxia world
- starting items: suit, ID badge, wallet fragment, pen, work notebook; smartphone gone
- first pressure: Heuksabang thugs
- rescue/homebase: Qingliu sect
- key companion: Seo Harin
- key artifact/system: Cheonggi record
- reward system: heavenly fragments / modern knowledge candidates
- risks: qi-blood backlash, heart demon, Cheonggi corruption, modern knowledge misuse

Verification:

- Generic inn/shaolin/wudang/emei material is not canonical unless explicitly subordinated.

#### Task 7.2 — Mark `wuxia_jianghu_pack` superseded

Options:

- Rename file to canonical id.
- Keep old file with banner “superseded by...”
- Move old content into archive.

Default recommendation:

- Create new canonical file and add supersession banner to old file first.
- Delete/rename later after references are updated.

User intervention:

- Ask before deleting old file.

#### Task 7.3 — Define first wuxia vertical slice

Recommended slice:

1. Prologue: commute fracture.
2. Market arrival in work clothes.
3. Heuksabang thug fight as auto-brawl tutorial.
4. Most outcomes are loss/injury/escape; high physical/composure can barely hold out.
5. Seo Harin/Qingliu rescue or contact.
6. First Cheonggi reaction.
7. Heavenly fragment choice: 3 candidates.
8. Cycle settlement: Qingliu trainee entry / unresolved Heuksabang pressure.

Tests:

- fight scene uses auto-brawl/intervention structure.
- Cheonggi fragment is not a search box.
- exactly one fragment chosen.
- Qingliu/Seo Harin state updates.

#### Task 7.4 — Define wuxia base goals

Base goals from Notion examples:

- 무공을 쌓는다
- 흑사방의 움직임을 추적한다
- 청류문 내 신임을 얻는다

Conditional examples:

- [보물지도] 폐정자 너머의 장소를 확인한다
- [서하린] 서하린의 판단을 따른다
- [천기록] 빈 장의 반응을 확인한다

Verification:

- Goals are purpose-based, not place-based.

#### Task 7.5 — Define core companion and factions

Initial data:

- Seo Harin
- Qingliu sect
- Heuksabang
- optional later: Dowel/other Notion characters if present in future pages

Rules:

- Main cast under 10.
- No frustrating internal villain arc inside Qingliu homebase unless Notion adds it.
- External pressure can arrive via rumor, notice, courier, public letter.

Verification:

- Companion intervention rules have at least one Seo Harin advice/joint/opposition path.

#### Task 7.6 — Define first epilogue cards

Candidates:

- Qingliu sect fate
- Seo Harin fate
- Heuksabang rumor
- Cheonggi record trace
- chosen heavenly fragment consequence

Verification:

- At least one card has exclusive_group for Seo Harin fate.

**Exit criteria for Phase 7:**

- Wuxia placeholder is no longer competing with Notion.
- First wuxia Web/Rust slice exists or is ready to implement.

---

### Phase 8 — Archive, Meta-Progression, And Replay Loop

**Objective:** Build the PokéRogue-like retention loop around runs, endings, cards, and discovered storypack knowledge.

**Files likely to change:**

- `crates/escape-core/src/save.rs`
- `crates/escape-core/src/state.rs`
- `web/src/game/save.ts` or replacement save adapter
- `web/src/ui/startScreen.ts`
- new Web archive UI files
- data/schema for archive entries

**Features:**

1. Run save.
   - current storypack
   - current goal/cycle round
   - seed
   - timestamp

2. Archive save.
   - discovered endings
   - epilogue cards
   - fragment records
   - maybe achievements

3. Quick restart.
   - new run with random seed
   - replay same seed
   - choose storypack if unlocked/available

4. Failure recovery.
   - failed run still can unlock archive cards if Notion design allows.

5. LocalStorage only for MVP.
   - no account system.
   - export/import save optional later.

Tests:

- archive persists after ending.
- corrupt archive does not brick start screen.
- run save schema mismatch gives reset/import option.

User intervention:

- Ask before adding meta-progression beyond archive/discovery.

---

### Phase 9 — Deployment And Production Hardening

**Objective:** Make the Web player deployable and stable as a public/static browser game.

**Files likely to change:**

- `.github/workflows/*`
- `web/package.json`
- `web/vite.config.*`
- `docs/dev/Web_Distribution_Decision.md`
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- `tests/test_web_player_deployment_contract.py`

**Tasks:**

1. Verify static build.
   - `cd web && npm run build:player`

2. Verify WASM asset paths.
   - subpath deploy works.
   - GitHub Pages or Cloudflare Pages base path works.

3. Verify production requires WASM.
   - no silent TS gameplay fallback in production.

4. Verify localStorage save reset UI.

5. Verify visual QA at reference mobile viewport.

6. Add deployment checklist.

7. Add release smoke:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
npm run build:player
npm run preview:player
npm run qa:storybook:visual -- --require-wasm
```

Use the local storage guardrail on this host: do not create `node_modules` or build caches under repo/home if avoidable; source tmp setup before builds/installs.

---

## 7. Validation Matrix By Phase

### Docs-only phases

Commands:

```bash
git diff --check
python - <<'PY'
from pathlib import Path
for p in Path('docs').rglob('*.md'):
    s = p.read_text(encoding='utf-8')
    if s.count('```') % 2:
        print('unbalanced fence', p)
PY
```

Also search stale terms:

```bash
/home/dudupunch0/rg/rg -n "SuperLightTUI|Textual|TUI 기반|terminal-native|wuxia_jianghu_pack|office_apocalypse|isolation_pack" docs README.md AGENTS.md
```

Expected:

- Active stale terms either gone or marked legacy/superseded.

### Content schema phases

Commands:

```bash
python -m pytest tests/test_content_data.py tests/test_web_data_export.py -q
python scripts/export_web_data.py --check \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json
```

Later replace with new validator when content root changes.

### Rust core phases

Commands:

```bash
cargo test -p escape-core
cargo test -p escape-wasm
```

If terminal crate still in workspace during transition:

```bash
cargo test --workspace
```

After removing terminal crate:

```bash
cargo test -p escape-core -p escape-wasm
```

### Web phases

Commands:

```bash
source /home/dudupunch0/.config/tui_adv/tmp-installs.sh
cd web
npm test
npm run build:player
npm run qa:storybook:visual -- --require-wasm
```

Do not install dependencies into repo/home on this host without tmp setup.

---

## 8. Suggested Execution Order For The Next Several Work Sessions

### Session 1 — Phase 0 only

Goal:

- Promote user pivot into canonical docs and agent instructions.

Do not:

- Change runtime code.
- Delete TUI code.
- Rename packages.

Expected PR/diff:

- `AGENTS.md`
- `README.md`
- `docs/00_Index.md`
- `docs/dev/Development_Plan.md`
- maybe `docs/dev/Checklist.md`
- maybe `idea_box/BACKLOG_ORDER.md`

Ask user only if:

- They want immediate repo/package rename.

### Session 2 — Phase 1 design docs

Goal:

- Write Notion-aligned design docs.

Expected PR/diff:

- new/updated design docs for goal cycle, random encounter, side quest, companion, ending/epilogue, fragments.

Ask user:

- First playable storypack choice if needed.
- Reality-link scope if docs must commit to keeping/removing it.

### Session 3 — Phase 2 schema proposal

Goal:

- Make schema concrete enough for Rust implementation.

Ask user:

- Content root naming/package rename if needed.
- Companion stat scale/name decision if not obvious.

### Session 4 — Phase 4 minimal Rust vertical slice

Goal:

- Implement storypack id + goal selection + minimal 10-round cycle skeleton.

Do not yet:

- Implement all side quests/companions/epilogue features.

### Session 5 — Phase 5 Web UI for skeleton

Goal:

- Web can render storypack start + goal selection + one cycle scene.

### Session 6 — Phase 6 `야근몽` content slice

Goal:

- First playable Notion-aligned office storypack run.

### Session 7 — Phase 7 `이구학지` content slice

Goal:

- First playable wuxia storypack vertical slice.

---

## 9. Important Risks

1. Scope explosion.
   - Notion systems are broad.
   - Mitigation: docs first, schema proposal second, minimal vertical slices only.

2. Premature deletion of old code.
   - Removing Python/terminal too early may break exporters/tests.
   - Mitigation: docs deprecate first, code delete after Web/Rust coverage replaces it.

3. Schema over-engineering.
   - Trying to model all Notion pages in one pass can stall implementation.
   - Mitigation: implement a minimal subset per vertical slice but keep schema names aligned with future model.

4. TypeScript gameplay drift.
   - Web renderer may accidentally grow its own gameplay rules.
   - Mitigation: new rules only in Rust core; Web consumes ScenePage/ActionResult.

5. Old docs misleading future agents.
   - Existing docs heavily mention TUI/SuperLightTUI/office apocalypse.
   - Mitigation: Phase 0 is mandatory before deeper implementation.

6. Notion reference drift.
   - Notion may change after local dump.
   - Mitigation: re-fetch before each design sync and reference-check before marking idea done.

7. Naming churn.
   - Renaming repo/package/crates too early can create large mechanical diffs.
   - Mitigation: product-facing rename first, technical rename later with explicit approval.

---

## 10. Open Questions To Ask Only At The Right Time

Do not ask all of these now. Ask them only when implementation reaches the relevant gate.

1. Product/package naming:
   - Should `tui_adv` remain the repo/package name for now?

2. First playable refactor storypack:
   - `야근몽` first, or `이구학지 — 천기록` first?

3. Reality link:
   - Keep actual local/ARG hidden clues, or make them purely fictional reality anchors?

4. Technical deletion:
   - Delete terminal/Python code as soon as docs pivot, or wait until Web replacement tests exist?

5. Content root:
   - New `content/` root, or continue `src/tui_adv/data/` until package rename?

6. Companion stat names:
   - Reuse player six stats for companions, or preserve old NPC 0-20 stat generator model as canonical?

7. Epilogue illustrations:
   - For card `illustration_hint`, should first Web version use placeholders/GlyphFX, pixel stills, or generated art later?

8. Meta progression:
   - Archive-only, or unlockable storypack/fragment/ending progression?

---

## 11. Immediate Next Action Recommendation

Start with Phase 0.

Concrete first implementation slice:

1. Re-fetch Notion page list and titles.
2. Update `docs/dev/Development_Plan.md` with the pivot decision.
3. Update `AGENTS.md` with Web-only/Notion-first instructions.
4. Update `README.md` and `docs/00_Index.md` so future agents/users do not follow old TUI direction.
5. Update `idea_box/BACKLOG_ORDER.md` to mention the new grouped Notion reference sync state.
6. Verify with markdown/stale-term searches and `git diff --check`.

This slice is low-risk, docs-only, and prevents future implementation from being pulled back toward the obsolete TUI/office-apocalypse plan.

---

## 12. Final Success Criteria

The refactor is complete when:

1. Product docs say Web-only PokéRogue-like browser player.
2. Notion reference systems are represented in canonical design/schema docs.
3. `야근몽` and `이구학지 — 천기록` are canonical storypacks.
4. Old `isolation_pack` / `wuxia_jianghu_pack` are absorbed, superseded, or archived.
5. Rust core supports goal cycles, encounters, side quests, companions, fragments, final choices, endings, and epilogue cards at least for MVP slices.
6. Web player can start a run, select a goal, play cycle scenes, choose fragments, reach endings, and view epilogue cards.
7. TUI/terminal/Python product paths are removed or clearly transitional with no active-user documentation.
8. Static Web build/preview/visual QA passes.
9. Notion-origin idea entries are marked done only after reference comparison confirms the repo design follows Notion or records explicit deviations.
