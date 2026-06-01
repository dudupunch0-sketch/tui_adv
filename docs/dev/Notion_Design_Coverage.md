# Notion Design Coverage

Status: docs-only sync + Mumyeong aid request runtime implemented + failed-aid follow-up handoff selected
Last checked: 2026-06-02T06:02:38+09:00
Runtime status: `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, and `wuxia_mumyeong_request_for_aid` were implemented later as separate `storypack_preview` runtime slices. The next repo step is the docs-only handoff `wuxia_mumyeong_followup_after_failed_aid`. This document remains the Notion coverage ledger. 2026-06-01 default 전환 이후 Web player 기본 storypack은 이구학지이며, 2026-06-02 terminal content scene default도 이구학지 fixture로 맞췄다.
Raw snapshot policy: live Notion markdown/DB exports were used for audit in `/tmp/tui_adv_notion_wuxia_sync`; raw snapshots are not committed.

## Purpose

This document records the live Notion coverage for `wuxia_jianghu_pack` / **이구학지 — 천기록** and maps it into repo canonical design docs before any runtime implementation.

## Source precedence

1. Runtime implementation is gated by repo canonical docs, especially `docs/dev/Development_Plan.md`, `docs/content/storypacks/wuxia_jianghu_pack.md`, and `docs/content/encounter_db/wuxia_jianghu_pack.md`.
2. For Notion upstream, the parent page `무협 스토리팩: 이구학지 — 천기록` is a synopsis and early planning document.
3. Latest detailed Notion standards come from the child management pages and the two Notion DBs. If the parent page body conflicts with a child page or DB row, prefer the child page/DB.
4. Notion 사건 카드 DB rows and 후일담 카드 DB rows are design sources, not runtime-complete proof.
5. Runtime completion is only claimed when preview YAML/Rust/Web generated bundles and tests prove the content exists.
6. Never explain or reveal the true identity of 천기록. “정체 접근” means only sensing the recorder’s presence/gaze or noticing that the record behaves as if someone is writing.

## Checked Notion sources

| source | object | id | last edited | repo use |
|---|---:|---|---:|---|
| 무협 스토리팩: 이구학지 — 천기록 | page | `36f37e69-695e-8198-86c2-d35c00609441` | 2026-05-31T11:31:00Z | parent synopsis + precedence notice |
| 00. 스토리팩 관리 개요 | page | `37137e69-695e-81b6-abaa-dd88aec5ede2` | 2026-05-31T02:35:00Z | operating model, no internal 청류문 antagonism |
| 01. 청류문 | page | `37137e69-695e-8181-bda7-edc1d61da7f0` | 2026-05-31T02:35:00Z | latest 청류문 identity and martial structure |
| 02. 주요 등장인물 | page | `37137e69-695e-81bc-b6d1-f9bdf7f3c429` | 2026-06-01T01:48:00Z | character slots, 서하린 emotional role |
| 서하린 상세 설계 | child page | `37137e69-695e-81ae-8575-c1a005dc73b3` | 2026-05-31T15:21:00Z | checked as linked child of 02; future companion/epilogue design source |
| 03. 세력과 외부 압박 | page | `37137e69-695e-815c-b207-e6c8805fcbd2` | 2026-06-01T01:48:00Z | external pressure, 흑사방/무림맹/current named faction policy |
| 04. 메인 루트 구조 | page | `37137e69-695e-8188-8927-dfb33d4eeec9` | 2026-05-31T08:10:00Z | route candidates and final narrowing policy |
| 05. 사건 카드 운영 규칙 | page | `37137e69-695e-812d-8aca-d97eea39964e` | 2026-05-31T08:10:00Z | event card fields and DB-first event operation |
| 06. 사이드 퀘스트와 미해결 부채 | page | `37137e69-695e-8141-954f-d8a519b80e5e` | 2026-05-31T08:10:00Z | unresolved debt / epilogue principles |
| 07. 천기록 / 천외편린 보상 | page | `37137e69-695e-819e-9d66-d9eef819b02a` | 2026-06-01T01:49:00Z | latest 천기록 policy and 3-choice reward policy |
| 08. 엔딩과 후일담 연결 | page | `37137e69-695e-8177-a228-d7f96d084622` | 2026-05-31T14:51:00Z | epilogue card linking rules |
| 09. 이구학지 사건 카드 DB | data_source | `3a1bab76-6eb9-48de-afad-5b4f65c996c0` | 2026-05-31T08:04:00Z | 26 event-card design source rows |
| 10. 이구학지 후일담 카드 DB | data_source | `85157a3a-6e89-42b5-a6a0-d23e7b2d231f` | 2026-05-31T08:06:00Z | 17 epilogue-card future design rows |
| 99. 통합 체크포인트 | page | `37137e69-695e-81b9-84fc-ca879702a5d1` | 2026-06-01T01:45:00Z | final unresolved-item checkpoint; none open in Notion |

## Latest operating standards reflected in repo docs

### 청류문

- 청류문 is a collapsed orthodox sect: a poor, fallen 정파 문파 after a larger 정·사·마 conflict.
- Tone is a warm underdog home base, not a hostile internal-politics arena.
- Forbidden inside 청류문: internal political infighting, elder-council frustration arcs, persistent protagonist suspicion, disciple power struggles, internal traitors.
- 청류문 problems should come from scarcity and pressure: money, people, herbs, lost manuals, broken training grounds, sick sect master, empty rooms, lost status, 흑사방/무림맹 pressure.
- Martial structure: `청류심법` flows qi without fixing it; `청류안` reads breath/center/flow/openings; `관류` sees flow, `수류` receives it, `역류` cuts against it, `환류` returns/reconstructs it.

### 천기록 / 천외편린

- 천기록 is not a search box.
- It does not answer arbitrary player questions.
- It presents three candidate fragments only when conditions are satisfied; the player chooses one, and the other two are lost for that moment.
- It may look like a personality-like record in mid/late game, but its identity remains unrevealed.
- “정체 접근” is limited to sensing presence, gaze, real-time writing, or remembering choice traces. Do not turn it into a lore explanation.

## Notion 사건 카드 DB 26-row mapping

| # | Notion event id | Notion name | repo mapping | repo status / action |
|---:|---|---|---|---|
| 1 | `wuxia_seoharin_unsaid_stay` | 가지 말라는 말 | none yet | future 서하린 companion / late return-route event source |
| 2 | `wuxia_seoharin_left_meal` | 남겨둔 밥 | none yet | future 서하린 companion event source |
| 3 | `wuxia_seoharin_empty_place` | 비워둔 자리 | none yet | future 서하린 companion / 무명 clue event source |
| 4 | `wuxia_mumyeong_departure_truth_summary` | 무명 이탈의 진실 정리 | none yet | future 무명 truth / 서하린 handoff source |
| 5 | `wuxia_black_serpent_pressures_qingliu` | 흑사방의 청류문 압박 | partial: `wuxia_cheongryu_raid_route_split` pressure background | future side/pressure event; not runtime |
| 6 | `wuxia_mumyeong_copy_style_reveal` | 무명의 카피 무공 공개 | `wuxia_mumyeong_copy_style_reveal` | preview runtime implemented |
| 7 | `wuxia_mumyeong_resolution` | 무명 결산 | none yet | future final-route event source |
| 8 | `wuxia_mumyeong_midgame_reunion` | 무명 중반 재회 | `wuxia_mumyeong_midgame_reunion` | preview runtime implemented |
| 9 | `wuxia_boss_resolution` | 보스 결산 | none yet | future final boss/result event source |
| 10 | `wuxia_mumyeong_first_sighting` | 무명 첫 목격 | `wuxia_mumyeong_first_sighting` | preview runtime implemented as common post-opener midgame bridge |
| 11 | `wuxia_mumyeong_first_confrontation` | 무명 첫 대치 | `wuxia_mumyeong_first_confrontation` | preview runtime implemented |
| 12 | `wuxia_boss_first_appearance` | 보스 첫 등장 | `wuxia_boss_first_appearance` | preview runtime implemented |
| 13 | `wuxia_mumyeong_destroys_orthodox_sect` | 정파 문파 멸문 | none yet | future backstory / consequence source |
| 14 | `wuxia_mumyeong_awakening` | 무명의 각성 | none yet | future rival corruption/growth source |
| 15 | `wuxia_boss_recruits_mumyeong` | 흑사방 보스의 스카웃 | none yet | future backstory/source row |
| 16 | `wuxia_mumyeong_reads_orthodox_style` | 무명의 정파 무공 간파 | `wuxia_mumyeong_reads_orthodox_style` | preview runtime implemented |
| 17 | `wuxia_qingliu_attack_after_war` | 무너져가는 청류문 습격 | none yet | future backstory/route pressure source |
| 18 | `wuxia_mumyeong_request_for_aid` | 무명의 도움 요청 | `wuxia_mumyeong_request_for_aid` | preview runtime implemented |
| 19 | `wuxia_tianjilu_first_fragment` | 천기록 첫 천외편린 | `wuxia_cheonggi_record_first_fragment` | preview runtime implemented as schema-less foreshadow; full 3-choice reward schema still future |
| 20 | `wuxia_seoharin_intervention` | 서하린의 개입 | `wuxia_seo_harin_rescue` | preview runtime implemented as schema-less rescue/protection bridge |
| 21 | `wuxia_prologue_commute_rift` | 출근길의 균열 | `wuxia_commute_rift_arrival` | preview runtime implemented |
| 22 | `wuxia_qingliu_apprentice_entry` | 청류문 임시 수습생 등록 | `wuxia_cheongryu_apprentice_entry` | preview runtime implemented as schema-less apprentice/chore bridge |
| 23 | `wuxia_qingliu_first_arrival` | 청류문 첫 도착 | partial: `wuxia_cheongryu_apprentice_entry` / `cheongryu_outer_courtyard` | courtyard arrival is represented in the implemented apprentice bridge; richer arrival beats remain future |
| 24 | `wuxia_black_serpent_first_trouble` | 흑사방 첫 시비 | `wuxia_heuksa_bang_first_fight` | preview runtime implemented |
| 25 | `wuxia_arrival_market_confusion` | 낯선 장터에 떨어지다 | `wuxia_commute_rift_arrival` | preview runtime implemented |
| 26 | `wuxia_main_qingliu_eye_001` | 청류안 첫 발현 | none yet; related to future 청류안/천외편린 growth | future growth-system/source row; not runtime implemented |

## Notion 후일담 카드 DB 17-row status

The epilogue DB is a future design source. None of these 17 rows is runtime implementation by itself, and this docs-only sync does not add an epilogue renderer/schema.

| # | Notion card id | card name | category | repo status |
|---:|---|---|---|---|
| 1 | `epilogue_seoharin_closed_gate` | 닫힌 산문 | 히든 / 특수 | future 서하린 corruption epilogue source |
| 2 | `epilogue_seoharin_last_bowl` | 마지막 밥그릇 | 동료 | future 서하린 low-relationship epilogue source |
| 3 | `epilogue_seoharin_open_gate` | 닫히지 않은 산문 | 동료 | future 무명 구원 + 서하린 epilogue source |
| 4 | `epilogue_seoharin_empty_place` | 비워둔 자리 | 동료 | future return/absence epilogue source |
| 5 | `epilogue_tianjilu_last_page` | 천기록의 마지막 장 | 기연 / 천외편린 | future 천기록 finale source; no identity reveal |
| 6 | `epilogue_qingliu_future` | 청류문의 후일 | 세력 | future 청류문 survival/rebuild source |
| 7 | `epilogue_seoharin_future` | 서하린의 후일 | 동료 | future baseline 서하린 survival epilogue source |
| 8 | `epilogue_mumyeong_stolen_forms_stopped` | 훔친 초식이 멈춘 날 | 동료 | future 무명 구원 rumor source |
| 9 | `epilogue_mumyeong_end_of_stolen_forms` | 훔친 초식의 끝 | 동료 | future 무명 non-rescue source |
| 10 | `epilogue_boss_alliance_silence` | 무림맹의 침묵 | 세력 | future official-denial source |
| 11 | `epilogue_mumyeong_second_wooden_sword` | 문밖의 두 번째 목검 | 동료 | future 무명 return/trace source |
| 12 | `epilogue_boss_broken_black_serpent` | 부러진 검은 뱀 | 세력 | future boss defeat rumor source |
| 13 | `epilogue_mumyeong_new_shadow` | 흑사방의 새 그림자 | 동료 | future 무명 assimilation source |
| 14 | `epilogue_mumyeong_unsent_apology` | 전하지 못한 사과 | 동료 | future partial-truth source |
| 15 | `epilogue_boss_black_serpent_banner` | 흑사방의 깃발 | 세력 | future boss/unresolved pressure source |
| 16 | `epilogue_mumyeong_black_serpent_new_scale` | 검은 뱀의 새 비늘 | 세력 | future successor/source row |
| 17 | `epilogue_wuxia_southern_market_rumor` | 남쪽 장터의 풍문 | 풍문 | future unresolved-debt source |

## Repo coverage summary

Already reflected before this sync:

- `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, and `wuxia_mumyeong_copy_style_reveal` exist as separate storypack preview runtime content.
- Later epilogue candidates remain designed docs/handoff candidates, not runtime content.
- Default office bundle separation and `escape-office` save/localStorage compatibility boundaries were already documented.

Synced by the 2026-06-01 docs-only pass:

- The Notion child-doc/DB-over-parent precedence rule.
- Latest 청류문 identity, no-internal-villain policy, scarcity-conflict rule, and 청류심법/청류안/관류/수류/역류/환류 structure.
- Latest 천기록 policy: not search, 3 candidate fragments with one selection, personality-like record but no identity reveal.
- 26 Notion event-card rows mapped to repo candidates or future source status.
- 17 Notion epilogue-card rows marked as future design sources, not runtime implementation.
- `서하린의 개입` mapped to repo `wuxia_seo_harin_rescue`; the follow-up runtime slice implemented that mapping in preview mode.

Still not runtime implemented:

- The next follow-up after the three route openers and later repo 후보 cards.
- Most Notion event DB rows beyond the currently previewed early/raid beats.
- All Notion epilogue DB rows.
- Full 천외편린 reward/ability schema, faction route graph, epilogue renderer/schema, companion/relation/debt ledgers, and route/multi-ending implementation.

## Runtime handoff notes

`wuxia_seo_harin_rescue`, mapped from Notion DB row `wuxia_seoharin_intervention` / `서하린의 개입`, `wuxia_cheongryu_apprentice_entry`, mapped from `wuxia_qingliu_apprentice_entry`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, and `wuxia_black_heaven_escape_price` are implemented in the preview runtime bundle.

Route opener implementation: the route-opener selection chose and implemented the righteous opener `wuxia_baekdo_medicine_debt`. It accepts both direct raid route and deferred wounded fallback branches through shared `righteous_route_started` + `cheongryu_rebuild_thread` flags; `baekdo_alliance_debt` and `baekdo_medicine_debt` remain branch flavor hooks.

Route opener follow-up implementation: `route_opener_followup_after_baekdo` rechecked the Notion parent, `03. 세력과 외부 압박`, `04. 메인 루트 구조`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-01. The selected candidate `wuxia_black_heaven_escape_price` is now implemented in the preview runtime. There is no exact Notion 사건 카드 DB row for this candidate; it is derived from the parent 사파 route synopsis, the external-pressure constraints in `03`, the route narrowing in `04`, the unresolved-debt reflection policy in `06`, and the 천기록 non-reveal policy in `07`/`99`. It accepts direct/deferred sapa branches through shared `sapa_route_started` + `dowol_debt` flags, while `black_heaven_deal_marked` and `black_heaven_escape_marker` remain flavor hooks. It still avoids legacy office bundle changes, `escape-office` key changes, 천기록 identity reveal, and new relation/debt/faction/reward/epilogue schemas.

Route opener follow-up after black heaven: `route_opener_followup_after_black_heaven` rechecked the Notion parent, `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`, and the event-card DB row `wuxia_prologue_commute_rift` on 2026-06-02. The selected next candidate is `wuxia_heavenly_archive_previous_outsiders`. There is no exact Notion 사건 카드 DB row for this opener; it is derived from the parent Branch C 천기/귀환 synopsis, `04`'s route narrowing policy, `05`'s event-card field contract, `06`'s unresolved-debt/future-echo boundary, `07`/`99`'s no identity reveal policy, and `wuxia_prologue_commute_rift`'s later return-thread connection. It accepts direct/deferred 천기 branches through shared `cheonggi_return_route_started` + `cheonggi_record_targeted` flags, while `heavenly_archive_contact` and `heavenly_archive_triage_map_seen` remain flavor hooks. It must still avoid legacy office bundle changes, `escape-office` key changes, 천기록 identity reveal, return system, and new relation/debt/faction/reward/epilogue schemas.

Route opener follow-up after black heaven implementation: `wuxia_heavenly_archive_previous_outsiders` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no exact 사건 카드 DB row was claimed, no 천기록 identity reveal or return system was opened, and direct/deferred 천기 branches still share `cheonggi_return_route_started` + `cheonggi_record_targeted` while `heavenly_archive_contact` and `heavenly_archive_triage_map_seen` remain flavor hooks.

Route opener follow-up after heavenly archive: `route_opener_followup_after_heavenly_archive` rechecked the Notion parent, `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`, and the event-card DB row `wuxia_qingliu_attack_after_war` on 2026-06-02. The selected next candidate is `wuxia_wounded_shelter_dawn_offers`. There is no exact Notion 사건 카드 DB row for this deferred-offer card; it is derived from `04`'s route narrowing policy, `05`'s event-card field contract, `06`'s unresolved-debt policy that important deferred choices should not block the main ending, `07`/`99`'s no identity reveal policy, and the repo runtime hook left by `stabilize_wounded_until_dawn`. It accepts only the concrete deferred branch flags `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`, while `survivor_roll_call_complete` and `route_delay_cost_recorded` remain flavor hooks. It was selected over the first post-opener midgame continuity card because the latter would likely need any-of conditions over `righteous_route_opened`/`sapa_route_opened`/`cheonggi_return_route_opened` or an early route graph/faction reputation/ending schema. The next runtime pass must still avoid legacy office bundle changes, `escape-office` key changes, triage/companion death/mass combat, route graph/faction reputation/debt/relation schemas, reward/ability/epilogue schemas, return system, and 천기록 identity reveal.

Route opener follow-up after heavenly archive implementation: `wuxia_wounded_shelter_dawn_offers` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no exact 사건 카드 DB row was claimed, no route graph/faction reputation/debt/relation schema was opened, `survivor_roll_call_complete` and `route_delay_cost_recorded` remain flavor hooks, and the card only reconnects the deferred branch through existing route starter flags.

Post-opener midgame continuity handoff: `route_midgame_continuity_after_wounded_shelter` rechecked the Notion parent, `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`, and the event-card DB rows `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, and `wuxia_mumyeong_midgame_reunion` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_first_sighting`, because the Notion row is explicitly a first midgame sighting after Qingliu entry/Black Serpent pressure and precedes first confrontation and midgame reunion. The handoff selected a common midgame bridge over route-specific fan-out and over a deferred-offer-only bridge. To avoid opening any-of conditions, the next implementation adds a common `route_opener_resolved` flag to every outcome of `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, and `wuxia_heavenly_archive_previous_outsiders`, then requires that flag plus `cheongryu_raid_survived`, `cheongryu_trial_started`, and `first_fragment_seen`. The next runtime pass must still avoid legacy office bundle changes, `escape-office` key changes, route graph/faction reputation/debt/relation schemas, combat schema, reward/ability/epilogue schemas, return system, boss first appearance, first confrontation, and 천기록 identity reveal.

Post-opener midgame continuity implementation: `wuxia_mumyeong_first_sighting` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no any-of condition schema was opened, the three route openers fan in through `route_opener_resolved`, and the event only leaves flags/clues/log/presentation for 무명 존재, 서하린 침묵, and copied Qingliu flow. The next handoff, `wuxia_mumyeong_first_confrontation_after_sighting`, must decide whether `wuxia_mumyeong_first_confrontation` can be implemented with the existing encounter schema before opening combat, route graph, faction reputation, debt/relation, reward/ability, epilogue, return, boss, or 천기록 identity reveal surfaces.

Rival confrontation handoff: `wuxia_mumyeong_first_confrontation_after_sighting` rechecked the Notion event-card DB rows `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_midgame_reunion`, and `wuxia_boss_first_appearance`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_first_confrontation`, because its Notion prerequisite is exactly after first sighting and its stated goal is endurance/analysis rather than victory. `wuxia_mumyeong_midgame_reunion` is deferred because it requires first confrontation and past clues; `wuxia_boss_first_appearance` is deferred because it opens boss-wall pressure. The next runtime pass must express the confrontation with existing encounter hooks only: `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, copied-flow weakness clues, 서하린 silence clues, and `destination_id: cheongryu_outer_courtyard`. It must still avoid legacy office bundle changes, `escape-office` key changes, combat resolver/schema, HP numeric combat, route graph/faction reputation/debt/relation schemas, reward/ability/epilogue schemas, return system, boss first appearance, midgame reunion, and 천기록 identity reveal.

Rival confrontation implementation: `wuxia_mumyeong_first_confrontation` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no combat resolver/schema or HP numeric fight was opened, every outcome leaves `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, and `destination_id: cheongryu_outer_courtyard`, and the event expresses endurance/analysis rather than victory. A follow-up Notion check on 2026-06-02 found `wuxia_mumyeong_copy_style_reveal` as a strong next candidate because its prerequisite is first confrontation and its function is to reveal this run's copied martial style. The next handoff, `wuxia_mumyeong_followup_after_first_confrontation`, must compare that candidate with `wuxia_mumyeong_midgame_reunion` and `wuxia_boss_first_appearance` before opening random copy-style systems, 천외편린 reward schema, boss combat, route graph/faction reputation/debt/relation, epilogue, return, or 천기록 identity reveal surfaces.

Post-confrontation follow-up handoff: `wuxia_mumyeong_followup_after_first_confrontation` rechecked the Notion event-card DB rows `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_midgame_reunion`, and `wuxia_boss_first_appearance`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_copy_style_reveal`, because its Notion prerequisite is exactly after first confrontation and it can reveal the copied-style family through existing flags/clues/log/presentation hooks. `wuxia_mumyeong_midgame_reunion` is deferred because it also needs some past-Mumyeong clues; `wuxia_boss_first_appearance` is deferred because it opens boss-wall/final-logic pressure. The next runtime pass must still avoid seed-based random copy-style tables, combat resolver/schema, HP numeric fights, boss first appearance, midgame reunion, route graph/faction reputation/debt/relation schemas, reward/ability/epilogue/return schemas, and 천기록 identity reveal.

Post-confrontation follow-up implementation: `wuxia_mumyeong_copy_style_reveal` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no seed-based random copy-style table, reward/ability schema, combat resolver/schema, boss first appearance, or midgame reunion was opened, and every outcome only leaves `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard`, plus copied-family/호흡 불일치/겉흐름 복사 clues.

Post-copy-style follow-up handoff: `wuxia_mumyeong_followup_after_copy_style_reveal` rechecked the Notion event-card DB rows `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, and `wuxia_mumyeong_reads_orthodox_style`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_reads_orthodox_style`, because it turns the copy-style reveal clues into a focused orthodox-style trace: 현악문, 복호금쇄수, and 무명의 special eye. `wuxia_mumyeong_midgame_reunion` is deferred until past-Mumyeong clues are stronger; `wuxia_boss_first_appearance` is deferred to preserve boss-wall/final logic pressure; `wuxia_mumyeong_departure_truth_summary` is deferred because it reveals the larger departure truth and salvation route too directly. The next runtime pass must still avoid boss combat, midgame reunion, full Qingliu attack flashback, departure-truth reveal, random copy-style tables, reward/ability, epilogue, return, route graph, relation/debt/faction schemas, or 천기록 identity reveal surfaces.

Post-copy-style follow-up implementation: `wuxia_mumyeong_reads_orthodox_style` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no boss first appearance, midgame reunion, full Qingliu attack flashback, departure-truth reveal, random copy-style table, reward/ability schema, combat resolver/schema, route graph, relation/debt/faction schema, epilogue, return, or 천기록 identity reveal was opened. Every outcome leaves `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard`, plus 현악문/복호금쇄수/무명 시야 변주 clues. The next handoff, `wuxia_mumyeong_followup_after_orthodox_style_trace`, must compare `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, and `wuxia_qingliu_attack_after_war` before opening boss combat, salvation truth reveal, full flashback, reward/ability, epilogue, return, or 천기록 identity reveal surfaces.

Orthodox-style follow-up handoff: `wuxia_mumyeong_followup_after_orthodox_style_trace` rechecked the Notion event-card DB rows `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, and `wuxia_qingliu_attack_after_war`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_midgame_reunion`, because its prerequisites now line up with repo hooks from first confrontation, copy-style reveal, and orthodox-style trace: `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, and `departure_truth_still_incomplete`. `wuxia_mumyeong_departure_truth_summary` is deferred because it is a late truth/salvation handoff; `wuxia_boss_first_appearance` is deferred to preserve boss-wall/final logic pressure; `wuxia_qingliu_attack_after_war` is deferred because it is full flashback/backstory source. The next runtime pass must still avoid boss combat, salvation truth reveal, 서하린에게 진실 전달, full flashback, random copy-style tables, reward/ability, epilogue, return, route graph, relation/debt/faction schemas, or 천기록 identity reveal surfaces.

Orthodox-style follow-up implementation: `wuxia_mumyeong_midgame_reunion` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no departure-truth summary, boss first appearance, full Qingliu attack flashback, boss combat, salvation truth reveal, 서하린에게 진실 전달, random copy-style table, reward/ability schema, route graph, relation/debt/faction schema, epilogue, return, or 천기록 identity reveal was opened. Every outcome leaves `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, and `destination_id: cheongryu_outer_courtyard`, with clue hooks for 서하린의 침묵, 현악문 흔적 공유, 무명의 미완성 진실, and `boss_used_mumyeongs_wound`. The next handoff, `wuxia_mumyeong_followup_after_midgame_reunion`, must compare `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, and `wuxia_qingliu_attack_after_war` before opening truth reveal, boss-wall combat, full flashback, reward/ability, epilogue, return, or 천기록 identity reveal surfaces.

Midgame reunion follow-up handoff: `wuxia_mumyeong_followup_after_midgame_reunion` rechecked the Notion event-card DB rows `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, and `wuxia_qingliu_attack_after_war`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. Notion search also surfaced `wuxia_mumyeong_request_for_aid`, which remains a future backstory bridge because it is not the current repo candidate and would pull the departure-truth thread forward. The selected next runtime candidate is `wuxia_boss_first_appearance`, because its stage is midgame, its prerequisites line up with the current repo hooks from `wuxia_mumyeong_midgame_reunion`, and it can be expressed as an overwhelming boss-wall encounter without a combat resolver. `wuxia_mumyeong_departure_truth_summary` is deferred because it is a late truth/salvation handoff involving 서하린에게 진실 전달; `wuxia_qingliu_attack_after_war` is deferred because it is full flashback/backstory source. The next runtime pass must express the boss as pressure, organization, and weakness-reading through existing encounter hooks only: `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, and `destination_id: cheongryu_outer_courtyard`. It must still avoid boss combat/final boss resolution, departure-truth reveal, full Qingliu attack flashback, 서하린에게 진실 전달, random copy-style tables, reward/ability, epilogue, return, route graph, relation/debt/faction schemas, or 천기록 identity reveal surfaces.

Boss first appearance implementation: `wuxia_boss_first_appearance` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no boss combat/final boss resolution, departure-truth reveal, full Qingliu attack flashback, 서하린에게 진실 전달, random copy-style table, reward/ability schema, route graph, relation/debt/faction schema, epilogue, return, or 천기록 identity reveal was opened. Every outcome leaves `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, and `destination_id: cheongryu_outer_courtyard`, with clue hooks for `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, and `qingliu_cannot_outmuscle_boss_yet`. The next handoff, `wuxia_boss_followup_after_first_appearance`, must compare `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, and `wuxia_boss_resolution` before opening backstory bridge, salvation truth reveal, full flashback, final boss resolution, reward/ability, epilogue, return, or 천기록 identity reveal surfaces.

Boss follow-up handoff: `wuxia_boss_followup_after_first_appearance` rechecked the Notion event-card DB rows `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, and `wuxia_boss_resolution`, plus the Notion operating docs `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, and `99. 통합 체크포인트` on 2026-06-02. The selected next runtime candidate is `wuxia_mumyeong_request_for_aid`, because it bridges the boss-wall clue into a focused record/rumor encounter about Mumyeong trying to save Qingliu and being refused by orthodox sects. `wuxia_mumyeong_departure_truth_summary` is deferred because it is a late truth/salvation handoff involving 서하린에게 진실 전달; `wuxia_qingliu_attack_after_war` is deferred because it is full flashback/backstory source; `wuxia_boss_resolution` is deferred because it is final boss/faction/epilogue resolution. The next runtime pass must still avoid departure-truth reveal, full Qingliu attack flashback, final boss resolution, boss combat, reward/ability, epilogue, return, route graph, relation/debt/faction schemas, or 천기록 identity reveal surfaces.

Mumyeong aid request implementation: `wuxia_mumyeong_request_for_aid` is now implemented in the preview runtime. It preserved the Notion-derived boundaries above: no departure-truth reveal, full Qingliu attack flashback, final boss resolution, boss combat, reward/ability schema, route graph, relation/debt/faction schema, epilogue, return, or 천기록 identity reveal was opened. Every outcome leaves `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, and `destination_id: cheongryu_outer_courtyard`, with clue hooks for `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, and `seoharin_does_not_know_failed_aid`. The next handoff, `wuxia_mumyeong_followup_after_failed_aid`, must compare the follow-up candidates again before opening truth reveal, full flashback, final boss resolution, reward/ability, epilogue, return, or 천기록 identity reveal surfaces.
