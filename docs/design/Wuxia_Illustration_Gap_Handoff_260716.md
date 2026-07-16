# Wuxia Illustration Gap Handoff

Date: 2026-07-16
Audit baseline: `origin/main` `43534ba`
Audience: illustration production and web-art integration

## 1. Purpose

This document lists every current Wuxia scene that does not resolve to a dedicated raster illustration. It distinguishes true `NO IMAGE` placeholders from procedural ink SVG fallbacks. Both categories require new illustration work under the current art direction.

The audit covers:

- 44 encounter-level `presentation.visual_id` values
- event-stage `kind: illustration` blocks
- 5 runtime locations
- 3 endings
- `web/src/ui/storybook/art/artManifest.ts`
- files under `web/public/assets/art/`
- actual browser rendering on `origin/main`

## 2. Production Contract

- Format: WebP
- Aspect ratio: 5:3
- Maximum file size: 150 KB
- Visual direction: original ink-wash Wuxia illustration; do not imitate a specific copyrighted composition
- Protagonist: modern office clothing is allowed when narratively necessary, but the face must remain hidden, cropped, turned away, silhouetted, or visually minimized
- Palette: paper, ink, seal red, muted jade, and restrained gold accents
- Avoid readable corporate logos, real company marks, modern brand names, and recognizable reference-image characters
- Avoid turning Seo Harin or other characters into romance-reward framing; preserve the story's restrained tone

Integration requires both:

1. Add the file to `web/public/assets/art/<visual_id>.webp`.
2. Add the exact `visual_id` mapping to `web/src/ui/storybook/art/artManifest.ts`.

Adding only the file does not activate it. After integration, rebuild and run the five-viewport Storybook visual QA.

## 3. Priority Summary

| Priority | Category | Count | Meaning |
|---|---:|---:|---|
| P0 | Visible generic `NO IMAGE` / generic placeholder | 4 | Player directly sees an empty illustration frame in an important flow |
| P1 | Finale and major turning points using procedural SVG | 12 | Story-critical scenes without authored illustration |
| P2 | Character and relationship beats using procedural SVG | 13 | Recurring-character scenes that need visual continuity |
| P3 | Supporting progression and travel scenes using procedural SVG | 14 | Lower urgency, still required for full illustration coverage |
| P1/P2 | Locations and endings without dedicated raster art | 6 | Reused surfaces or terminal moments need dedicated coverage |

Counts overlap when an encounter also owns a stage-level illustration slot. Use the exact visual ID in each row, not the encounter ID by assumption.

## 4. P0 — Generic Placeholder / `NO IMAGE`

### 4.1 Opening arrival stage

- Encounter: `wuxia_commute_rift_arrival`
- Stage: `arrival_story`
- Required visual ID: `wuxia_commute_rift_arrival`
- Current block ID: `placeholder:wuxia_commute_rift_arrival`
- Player-visible symptom: the first game screen displays `NO IMAGE`
- Scene: an office worker in commute clothes wakes on an unfamiliar Jianghu road, employee badge still hanging from the neck, while Qingliu trainees approach through distant dust
- Composition: rear or three-quarter back view; badge and office silhouette readable; face hidden; road and sect silhouettes dominate
- Mood: dislocation, morning haze, restrained danger rather than combat
- Integration note: replace the placeholder-prefixed stage visual ID with the final registered ID after the asset exists

### 4.2 Cheonoe analysis bridge

- Encounter/visual ID: `wuxia_cheonoe_analysis_thread_phase1_bridge`
- Title: 복기 루프 — 장부고의 틈
- Scene: immediately after the boss's first “price tag,” modern work-notebook analysis marks overlay the ledger-vault structure and reveal one narrow broken logical link
- Must communicate: a difficult structural insight, not an easy victory or magical omniscience
- Key motifs: 천기록, 복기, 장부고, notebook grid, ink ledger threads

### 4.3 Seo Harin's unsaid request

- Encounter/visual ID: `wuxia_seoharin_unsaid_stay`
- Title: 가지 말라는 말
- Scene: the Qingliu gate remains open in evening light; Seo Harin knows the protagonist may return home but does not say “do not go”
- Must communicate: return/stay ambiguity and an acknowledged empty place
- Avoid: romantic reward pose, embrace, direct confession, glamour portrait
- Key motifs: 서하린, 귀환, 청류문, 빈자리, open gate

### 4.4 Collapse gate

- Encounter/visual ID: `wuxia_collapse_gate`
- Title: 천기록 붕괴
- Scene: Cheon-gi Record strokes scatter in every direction and vision turns red as the protagonist chooses between burning a fragment to revive or dissolving into final rest
- Must communicate: terminal system collapse and a costly binary choice
- Key motifs: torn record strokes, seal-red wash, fragment burning, fading office-clothed silhouette

## 5. P1 — Finale and Major Turning Points

| Visual ID | Scene/title | Illustration brief |
|---|---|---|
| `wuxia_cheongryu_raid_route_split` | 습격과 갈라지는 길 | Qingliu under attack; White Alliance pressure and Cheon-gi Record route split shown through divided movement, not a menu-like diagram |
| `wuxia_heavenly_archive_previous_outsiders` | 이전 이방인 기록 | Dusty archive records reveal traces of earlier outsiders and dimensional rifts; protagonist remains small against shelves and hanging records |
| `wuxia_boss_first_appearance` | 보스 첫 등장 | Black Serpent boss first dominates the space; Mumyeong and Qingliu tension visible; avoid exposed protagonist face |
| `wuxia_mumyeong_awakening` | 무명 각성 | Mumyeong's copied styles break into a furious composite motion; emphasize loss of control rather than power fantasy |
| `wuxia_qingliu_attack_after_war` | 청류문 습격 흔적 | Damaged Qingliu grounds after the fighting; Hyeonak techniques left as physical traces in architecture and dust |
| `wuxia_mumyeong_destroys_orthodox_sect` | 빈 현악문 산문 | Empty Hyeonak gate after destruction, copied orthodox technique marks, absence more important than spectacle |
| `wuxia_sado_final_battle` | 최종전 | Black Serpent boss and ledger-vault price system converge before unavoidable combat; original composition, no heroic poster pose |
| `wuxia_sado_battle_loss_route_bridge` | 패배의 길 | Defeat route bridge: broken stance, ledger lines closing in, final page approaching; not a triumphant comeback |
| `wuxia_sado_final_phase_1_price_tag` | 가격표 | Boss turns people and debts into a visible price system; ledger tags and Qingliu obligations surround the confrontation |
| `wuxia_phase_2_weakpoint_control` | 약점 장악 | Seo Harin and Mumyeong exploit/control weak points while the protagonist reads the Cheon-gi pattern; tactical pressure, not clean victory |
| `wuxia_phase_3_outside_calculation` | 계산식 밖 | Seo Harin, Mumyeong, and the wooden sword break outside the boss's calculation; asymmetrical motion and ruptured ledger geometry |
| `wuxia_black_serpent_aftermath` | 사도 결산 이후 | Silent ledger vault after resolution; consequences and remaining debt should dominate over victory celebration |

### Stage-level finale slots

These illustration blocks exist in the final-battle event and require exact asset IDs even when the encounter also receives a presentation image:

| Stage visual ID | Required scene |
|---|---|
| `wuxia_sado_ledger_vault_threshold` | Ledger-vault entrance with black-scale ledgers and debt ink-lines suspended across the threshold |
| `wuxia_sado_final_battle` | Boss looking at the protagonist immediately before drawing blades |
| `wuxia_sado_blade_and_ledger` | Price tags and black-scale ledger spread beneath a blade; symbolic close detail |

## 6. P2 — Character and Relationship Beats

| Visual ID | Scene/title | Illustration brief |
|---|---|---|
| `wuxia_mumyeong_first_sighting` | 첫 목격 | Mumyeong observed at the edge of Qingliu/Black Serpent tension; keep identity partially obscured |
| `wuxia_mumyeong_first_confrontation` | 첫 대치 | Mumyeong, Seo Harin, and Qingliu face one another with restrained threat; replace the intentionally disabled unsafe candidate |
| `wuxia_mumyeong_copy_style_reveal` | 카피 무공 | First unmistakable copied Qingliu-eye technique; show mirrored movement rather than UI-like duplication |
| `wuxia_mumyeong_reads_orthodox_style` | 정파 무공 간파 | Mumyeong reads Hyeonak's orthodox hand technique through posture and spacing |
| `wuxia_mumyeong_midgame_reunion` | 중반 재회 | Uneasy reunion among Mumyeong, Seo Harin, and Hyeonak pressure; distance between figures is meaningful |
| `wuxia_mumyeong_request_for_aid` | 도움 요청 | Mumyeong asks Qingliu for aid without becoming submissive; tense, practical alliance framing |
| `wuxia_boss_recruits_mumyeong` | 스카웃 흔적 | Evidence that the boss tried to recruit Mumyeong; use traces, documents, or remembered silhouette rather than exposition text |
| `wuxia_mumyeong_departure_truth_summary` | 봉한 이탈 진실 | The sealed truth of Mumyeong's departure links Seo Harin, Hyeonak, and the boss; layered memory composition |
| `wuxia_seoharin_empty_place` | 비워둔 자리 | Empty wooden-sword training position preserved for someone absent; Seo Harin present but not centered as a portrait |
| `wuxia_seoharin_left_meal` | 남겨둔 밥 | A bowl deliberately left warm at Qingliu; quiet evidence of anticipated return |
| `wuxia_boss_resolution` | 보스 결산 | Boss resolution aftermath with the ledger system broken or inherited according to story tone |
| `wuxia_mumyeong_resolution` | 무명 결산 | Mumyeong's final personal state; avoid generic victory portrait |
| `wuxia_seoharin_qingliu_resolution` | 서하린·청류문 결산 | Seo Harin and Qingliu's future expressed through place, work, and surviving people |

## 7. P3 — Progression, Travel, and Supporting Scenes

| Visual ID | Scene/title | Illustration brief |
|---|---|---|
| `wuxia_cheongryu_chore_sparring` | 장작 마당 첫 겨루기 | Balance and breathing learned beside split firewood; humble practice, not elite duel |
| `wuxia_cheongryu_raid_wounded_fallback` | 부상자 피난처 | Wounded people sheltered near the archive; practical triage and constrained space |
| `wuxia_baekdo_medicine_debt` | 약상자와 채무 | Medicine chest offered with White Alliance debt attached; object-centered moral pressure |
| `wuxia_black_heaven_escape_price` | 탈출로의 값 | Black Heaven escape route and its cost; narrow passage, watchers, transactional tension |
| `wuxia_wounded_shelter_dawn_offers` | 새벽 제안 | Dawn proposals among wounded survivors; cold light and exhausted negotiation |
| `wuxia_cheonoe_pyeonrin_first_reward` | 첫 보상 | First fragment reward opens possible training directions; mystical record, restrained effect |
| `wuxia_cheonggi_record_writing_sense` | 적히는 감각 | Office notebook strokes write themselves as the Cheon-gi Record takes hold |
| `wuxia_cheonoe_pyeonrin_second_reward` | 두 번째 보상 | Stronger fragment reward with accumulated cost visible; distinct from the first reward |
| `wuxia_cheongirok_resolution` | 천기록 결산 | Cheon-gi Record's final relationship to the protagonist; record as system and witness |
| `wuxia_return_modern_commute_scene` | 현대 귀환 출근길 | Return to a modern commute after Jianghu; preserve ambiguity and hide the protagonist's face |
| `wuxia_settlement_stay_scene` | 강호 정착 | Staying in Jianghu as daily life and responsibility, not a throne or power fantasy |
| `location:wuxia_commute_rift` | 출근길 균열 location | Reusable location view distinct from the opening stage: empty rift-road establishing shot |
| `location:cheongryu_outer_courtyard` | 청류문 외원 location | Reusable Qingliu courtyard establishing shot with training and work traces |
| `location:black_serpent_ledger_vault` | 흑사방 장부고 location | Reusable ledger-vault establishing shot before/after confrontation |

## 8. Endings Requiring Dedicated Illustration

| Ending ID / proposed visual ID | Current state | Required direction |
|---|---|---|
| `wuxia_preview_grounded` | Generic ending visual and `NO IMAGE`; stale `kind: preview` copy | Easter-egg game-over illustration for “강호의 사원증”: protagonist clings to modern identity until the record closes; clearly terminal, dryly tragic rather than heroic |
| `wuxia_final_epilogue_renderer_contract` | No dedicated ending asset | Final ledger/record closing image that can sit behind or before computed epilogue cards without dictating one route outcome |
| `wuxia_death_rest` | No dedicated ending asset | Final rest: red strokes disperse and the commute/Jianghu boundary fades; solemn, not graphic |

The exact ending visual-ID convention must be confirmed during integration because `artAssetFor()` normalizes the `ending:` prefix. Prefer registering the actual ScenePage visual ID observed in the rebuilt runtime rather than guessing from the ending ID.

## 9. Existing Dedicated Raster Art — Do Not Recreate

These surfaces already resolve to authored raster assets:

- `wuxia_commute_rift` — encounter presentation only; it does not satisfy the opening stage placeholder
- `location:jianghu_roadside`
- `location:jianghu_market_street`
- `wuxia_heuksa_bang_first_fight`
- `wuxia_cheonggi_record_first_fragment`
- `wuxia_seo_harin_rescue`
- `wuxia_cheongryu_apprentice_entry`

Existing candidates for `wuxia_mumyeong_first_confrontation`, `wuxia_boss_first_appearance`, `wuxia_sado_final_battle`, `wuxia_return_modern_commute_scene_resolved`, and `wuxia_settlement_stay_scene_resolved` are commented out in the manifest and no corresponding files exist on `origin/main`. Produce new policy-compliant assets; do not simply uncomment stale mappings.

## 10. Delivery Checklist for Each Asset Batch

- [ ] Exact runtime `visual_id` confirmed from YAML/ScenePage
- [ ] 5:3 WebP, 150 KB or smaller
- [ ] Protagonist face hidden/minimized where present
- [ ] No copied emblem, character, or exact reference composition
- [ ] File added under `web/public/assets/art/`
- [ ] Manifest mapping added
- [ ] Stage-level placeholder ID replaced when applicable
- [ ] Alt text remains accurate after final composition
- [ ] `npm run build` passes
- [ ] Five-viewport Storybook visual QA passes with rebuilt WASM
- [ ] Screenshot inspected at 390 and 1440 widths
- [ ] No important face, text, weapon, or focal object is cropped by the 5:3 container
