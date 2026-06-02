# Machine-readable storypack DB

Status: 후보 콘텐츠 검증 DB

이 폴더는 사람용 storypack/encounter 문서에서 검증 가능한 최소 필드를 JSON으로 옮긴다. 아직 runtime game content가 아니며, `src/tui_adv/data/*.yaml`, Rust content bundle, Web generated content와 섞이지 않는다.

## 파일

- `storypacks.json`: storypack record 목록. `world_id`, `main_surfaces`, `main_phases`, NPC slot, 결말 후보, runtime 승격 메모를 담는다.
- `encounter_situations.json`: runtime 승격 전 encounter situation card 목록. 각 카드는 `storypack_id`, `world_id`, `phase`, `priority_class`, `surface`, `anomaly_type`, fallback choice, outcome hook, `main_spine_link`를 가진다.

## 검증

Python helper:

```python
from tui_adv.game.storypack_db import load_storypack_db, validate_storypack_db

errors = validate_storypack_db(repo_root)
db = load_storypack_db(repo_root)
```

테스트:

```bash
PYTHONPATH=src python -m pytest tests/test_storypack_db.py -q
```

검증 범위:

1. card의 `storypack_id`가 존재한다.
2. card의 `world_id`가 storypack의 `world_id`와 일치한다.
3. `status`, `priority_class`, `surface`, `anomaly_type`, `pressure_type`, `npc_slots`가 허용 taxonomy 안에 있다.
4. 최소 하나의 `safe_*`/fallback 선택지가 있다.
5. 최소 하나의 outcome hook이 있다.
6. `main_spine_link`가 비어 있지 않다.
7. 공개 DB에 private-only field 이름을 넣지 않는다.

## 현재 범위

- `isolation_pack`: office apocalypse 후보 카드 6개.
- `yageunmong_pack`: 야근몽 office-dream 후보 카드 12개. 2026-06-02 Notion 원문 재확인 뒤 이구학지식 route-pressure/동료 구출/후일담 handoff 구조로 확장했다.
- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 31개.

`wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`는 preview runtime에 구현되어 서하린 구조/감시/채무, 청류문 수습생/잡역 hook, 장작 마당 첫 겨루기 training hook, route-pressure hook, deferred wounded fallback hook, 정파/백도맹 약상자 채무 opener, 사파/흑천련 탈출로 값 opener, 천기각 이전 이방인 기록/균열 단서, 부상자 피난처 새벽 제안, 무명 첫 목격/첫 대치/카피 무공 공개/정파 무공 간파/중반 재회/보스 첫 등장/무명의 도움 요청 실패 기록/무명 각성/청류문 흔적 조사/현악문 consequence trace/보스 recruitment trace/무명 이탈 진실 sealed summary/서하린 empty-place bridge/서하린 left-meal belonging bridge/사도 가격표 장부 phase/사도 약점 장악 phase/사도 계산식 밖 phase/보스 결산 route seed bridge/무명 결산 route seed bridge와 `cheongryu_outer_courtyard`, `black_serpent_ledger_vault`를 남긴다. `wuxia_qingliu_attack_after_war`는 full flashback이 아니라 현악문/복호금쇄수 흔적 조사로 기존 schema에 landing했고, `wuxia_mumyeong_destroys_orthodox_sect`도 playable 멸문 전투가 아니라 빈 현악문 산문/기록/풍문 trace로 landing했으며, `wuxia_boss_recruits_mumyeong`는 구원이나 최종 결산이 아니라 보스 스카웃 흔적만 landing했고, `wuxia_mumyeong_departure_truth_summary`는 서하린에게 진실을 전달하지 않는 sealed summary로 landing했으며, `wuxia_seoharin_empty_place`는 truth delivery 없이 `seoharin_axis_opened`/`empty_place_remembered`를 남기는 late bridge로 landing했고, `wuxia_seoharin_left_meal`은 귀환/정착 최종 분기를 열지 않는 daily-care bridge로 landing했고, `wuxia_sado_final_phase_1_price_tag`는 combat resolver 없이 `network_handling`/`evidence_state`/`pressure_state`/`item_logs` 씨앗만 남겼으며, `wuxia_sado_final_phase_2_weakpoint_control`은 combat resolver 없이 `seoharin_axis`/`qingliu_rebuild`/`mumyeong_salvation`/`successor_route`/`own_flow_choice`/`cheongirok_state`/`player_method` 씨앗만 남기고, `wuxia_sado_final_phase_3_outside_calculation`은 combat resolver 없이 `combat_result`/`boss_resolution_route` 후보와 final-state 씨앗만 남겼으며, `wuxia_boss_resolution`은 final epilogue renderer 없이 보스 결산 route와 후속 epilogue candidate seed만 남겼고, `wuxia_mumyeong_resolution`은 final epilogue renderer 없이 무명 결산 route와 후속 epilogue candidate seed만 남긴다. legacy office bundle과 legacy `escape-office` save/localStorage key는 유지한다.

`wuxia_mumyeong_reads_orthodox_style`는 preview runtime에 구현됐고, `wuxia_mumyeong_midgame_reunion`는 preview runtime에 구현됐고, `wuxia_boss_first_appearance`는 preview runtime에 구현됐고, `wuxia_mumyeong_request_for_aid`는 preview runtime에 구현됐고, `wuxia_mumyeong_awakening`도 preview runtime에 구현됐고, `wuxia_qingliu_attack_after_war`도 preview runtime에 구현됐고, `wuxia_mumyeong_destroys_orthodox_sect`도 preview runtime에 구현됐고, `wuxia_boss_recruits_mumyeong`도 preview runtime에 구현됐고, `wuxia_mumyeong_departure_truth_summary`도 preview runtime에 구현됐고, `wuxia_seoharin_empty_place`도 preview runtime에 구현됐고, `wuxia_seoharin_left_meal`도 preview runtime에 구현됐고, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`도 preview runtime에 구현됐다. 다음 handoff 후보는 `wuxia_seoharin_qingliu_resolution`이다.
