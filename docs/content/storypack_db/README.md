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
- `yageunmong_pack`: 야근몽 office-dream 후보 카드 6개.
- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 22개.

`wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`는 preview runtime에 구현되어 서하린 구조/감시/채무, 청류문 수습생/잡역 hook, 장작 마당 첫 겨루기 training hook, route-pressure hook, deferred wounded fallback hook, 정파/백도맹 약상자 채무 opener, 사파/흑천련 탈출로 값 opener, 천기각 이전 이방인 기록/균열 단서, 부상자 피난처 새벽 제안, 무명 첫 목격/첫 대치/카피 무공 공개/정파 무공 간파/중반 재회/보스 첫 등장/무명의 도움 요청 실패 기록/무명 각성/청류문 흔적 조사/현악문 consequence trace와 `cheongryu_outer_courtyard`를 남긴다. `wuxia_qingliu_attack_after_war`는 full flashback이 아니라 현악문/복호금쇄수 흔적 조사로 기존 schema에 landing했고, `wuxia_mumyeong_destroys_orthodox_sect`도 playable 멸문 전투가 아니라 빈 현악문 산문/기록/풍문 trace로 landing했다. legacy office bundle과 legacy `escape-office` save/localStorage key는 유지한다.

`wuxia_mumyeong_reads_orthodox_style`는 preview runtime에 구현됐고, `wuxia_mumyeong_midgame_reunion`는 preview runtime에 구현됐고, `wuxia_boss_first_appearance`는 preview runtime에 구현됐고, `wuxia_mumyeong_request_for_aid`는 preview runtime에 구현됐고, `wuxia_mumyeong_awakening`도 preview runtime에 구현됐고, `wuxia_qingliu_attack_after_war`도 preview runtime에 구현됐고, `wuxia_mumyeong_destroys_orthodox_sect`도 preview runtime에 구현됐다. 다음은 `wuxia_mumyeong_destroys_orthodox_sect_followup` docs-only handoff다.
