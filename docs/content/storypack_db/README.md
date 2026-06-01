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
- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 7개.

다음 runtime slice는 이미 결정된 `storypack_preview` mode에서 `wuxia_seo_harin_rescue`를 구현하는 것이다. `wuxia_cheongryu_apprentice_entry`는 설계/handoff 완료된 후속 bridge지만, 서하린 구조/감시/채무 hook과 `cheongryu_outer_courtyard`가 preview source에 생긴 뒤에만 구현한다. `wuxia_cheongryu_raid_route_split`는 그 뒤의 later route-pressure 후보로 설계 완료했지만, rescue/apprentice/first-fragment hook이 안정화되기 전에는 구현하지 않는다. `wuxia_cheongryu_raid_wounded_fallback`은 raid split의 `evacuate_the_wounded_first` branch 이후 조건부 deferred follow-up으로만 연다. 기본 office bundle에 후보 storypack을 자동으로 섞지 않으며, `yageunmong_pack`은 별도 preview 후보로만 유지한다.
