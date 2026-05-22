---
type: llm_design_handoff
created: 2026-05-22
source: assistant
related_docs:
  - idea_box/BACKLOG_ORDER.md
  - docs/design/Storypack_Encounter_DB.md
  - docs/content/storypacks/README.md
  - docs/content/storypacks/isolation_pack.md
  - docs/content/encounter_db/README.md
  - docs/content/encounter_db/isolation_pack.md
  - docs/content/characters/README.md
  - docs/content/characters/recurrent_npcs.md
---

# LLM 설계 핸드오프: 스토리팩 / 인카운터 DB / 6스탯 등장인물

이 문서는 다른 LLM/agent가 `idea_box`만 먼저 봤을 때, 어떤 문서를 읽고 어떤 설계를 이어가야 하는지 알려주는 표지판이다.

중요: 이 문서는 새 구현 요구사항이 아니다. 후보 아이디어와 설계 문서의 내비게이션이며, 런타임 코드나 YAML 승격은 사용자가 명시적으로 요청했을 때만 한다.

또한 `idea_box`의 open 항목은 임의 선택 후보가 아니라 backlog다. 처리 순서는 `idea_box/BACKLOG_ORDER.md`에 적힌 Git 최초 반영 순서를 따른다.

## 먼저 읽을 순서

1. `idea_box/BACKLOG_ORDER.md`
   - 현재 open backlog의 처리 순서.
   - 같은 날짜 파일명이 여러 개일 때도 Git 최초 반영 순서로 정렬한다.
   - 가장 낮은 order의 open 항목부터 처리한다.

2. `docs/design/Storypack_Encounter_DB.md`
   - storypack, encounter situation card, 6스탯 NPC DB의 상위 설계.
   - phase, surface, anomaly_type, pressure_type, npc_slot taxonomy.
   - 후보 콘텐츠를 runtime content로 승격하기 전 검토 기준.

3. `docs/content/storypacks/README.md`
   - storypack 후보 목록과 status 규칙.
   - 현재 full record는 `isolation_pack` 하나이며, 나머지는 raw 후보다.

4. `docs/content/storypacks/isolation_pack.md`
   - 첫 full storypack record.
   - 차원격리팩의 핵심 컨셉, surfaces, route hooks, NPC slots, 결말 후보.

5. `docs/content/encounter_db/README.md`
   - encounter situation card DB 운영 규칙.
   - runtime encounter가 아니라 후보 상황 카드라는 경계.

6. `docs/content/encounter_db/isolation_pack.md`
   - 차원격리팩용 후보 encounter situation card 6개.
   - 다른 pack을 설계할 때 복제할 수 있는 카드 밀도와 형식 예시.

7. `docs/content/characters/README.md`
   - 6 core stats의 의미와 수치 규칙.
   - 관계 상태를 stat과 분리하는 원칙.

8. `docs/content/characters/recurrent_npcs.md`
   - 차원격리팩 첫 slice용 반복 NPC 후보 3명.
   - stat_total 60, encounter_uses, complicates, public-safe secret 작성 예시.

## 스토리팩/인카운터 관련 backlog를 처리할 때의 작업 단위

아래 작업 단위는 `BACKLOG_ORDER.md`에서 현재 처리할 open 항목이 storypack, encounter DB, 6스탯 NPC 설계와 관련될 때만 적용한다. 더 앞선 open 항목이 있으면 이 문서의 storypack 후보를 임의로 먼저 고르지 않는다.

### A. raw storypack을 full record로 승격

`BACKLOG_ORDER.md`에서 현재 처리 중인 open 항목이 특정 raw storypack과 연결될 때, `docs/content/storypacks/README.md`의 해당 raw 후보를 별도 문서로 확장한다.

- `document_contamination_pack`: 문서오염팩
- `meeting_reservation_pack`: 회의실예약팩
- `compensation_strike_pack`: 연봉협상-파업팩

작성 위치 예시:

- `docs/content/storypacks/document_contamination_pack.md`
- `docs/content/storypacks/meeting_reservation_pack.md`
- `docs/content/storypacks/compensation_strike_pack.md`

완료 시 함께 갱신할 곳:

- `docs/content/storypacks/README.md`
- 필요하면 `docs/00_Index.md`

### B. storypack별 encounter situation card 작성

새 pack 하나당 최소 6개 후보 카드를 만든다.

작성 위치 예시:

- `docs/content/encounter_db/document_contamination_pack.md`
- `docs/content/encounter_db/meeting_reservation_pack.md`
- `docs/content/encounter_db/compensation_strike_pack.md`

각 카드는 최소한 다음을 가져야 한다.

- `id`
- `status: candidate`
- `storypack_id`
- `phase`
- `priority_class`
- `location_tags`
- `surface`
- `anomaly_type`
- `pressure_type`
- `npc_slots`
- `summary`
- `setup_text`
- `choice_shapes`
- `outcome_hooks`
- `main_spine_link`
- `randomization_notes`
- `promotion_notes`

중요 기준:

- 하나의 카드는 하나의 명확한 상황만 다룬다.
- 최소 하나의 안전한 관찰/무시/이탈 선택지를 둔다.
- 최소 하나의 flag, clue, item, relation, resource, route hint 후보를 둔다.
- `main_spine_link`가 없으면 랜덤 잡음으로 보고 승격하지 않는다.

### C. 6스탯 기반 반복 등장인물 후보 작성

새 storypack에 필요한 반복 NPC를 2~3명씩 만든다.

작성 위치 예시:

- 기존 파일에 추가: `docs/content/characters/recurrent_npcs.md`
- 또는 pack별로 분리: `docs/content/characters/document_contamination_pack.md`

NPC 작성 규칙:

- core stat은 `sense`, `social`, `reason`, `self`, `impulse`, `body` 6개만 사용한다.
- 기본 `stat_total`은 60이다.
- 초기 stat은 0~20이며, 특별한 이유가 없으면 최소 3 이상이다.
- 관계값은 stat이 아니다. `trust`, `suspicion`, `debt`, `contamination`, `alive`, `companion` 같은 별도 상태 후보로 둔다.
- stat은 성격 장식이 아니라 인카운터 기능과 연결한다.
- 실제 회사/실제 인물/실제 내부정보처럼 보이는 세부사항을 쓰지 않는다.

## 다른 LLM에게 주는 기본 프롬프트

다른 LLM/agent에게 이 작업을 넘길 때는 아래 문장을 그대로 줘도 된다.

```text
이 repo의 idea_box/BACKLOG_ORDER.md를 먼저 읽고, 가장 낮은 order의 open 항목부터 처리해라.
idea_box/LLM_DESIGN_HANDOFF.md는 storypack/encounter DB/6스탯 NPC 관련 항목을 처리할 때 참고하는 설계 표지 문서다.
이번 작업이 storypack/encounter 관련이라면 관련 docs/design 및 docs/content 문서를 순서대로 확인해라.
작업은 먼저 후보 설계 문서 작성으로 시작하고, runtime 구현은 사용자가 명시적으로 요청했을 때만 한다.
실제 회사/인물/내부자료처럼 보이는 정보는 쓰지 말고, public-safe fictional corporate apocalypse 톤을 유지하라.
작업 후 관련 README와 docs/00_Index.md만 필요한 만큼 갱신하고, 요청 없이는 runtime YAML/Rust/Web 코드를 건드리지 마라.
작업한 idea entry는 실제 반영/폐기/병합 처리 결과를 기록한 뒤에만 status: done으로 바꿔라.
```

## 하지 말 것

- `src/tui_adv/data/encounters.yaml`, Rust core, Web Storybook, terminal renderer를 자동 수정하지 않는다.
- raw 아이디어를 읽었다는 이유만으로 `done` 처리하지 않는다.
- 실제 사용자의 메모, 사적 노트, 실제 회사 단서, 현실 ARG 힌트를 공개 문서에 옮기지 않는다.
- 모든 raw pack을 한 번에 확정하려고 하지 않는다. 한 번에 하나의 pack만 full slice로 만든다.
- LLM 즉석 생성 runtime 시스템으로 설계를 바꾸지 않는다. 현재 방향은 후보 DB를 사람이/agent가 문서로 축적하고 검토 후 승격하는 방식이다.

## 완료 기준

한 설계 slice가 끝났다고 보려면 다음이 있어야 한다.

- storypack full record 1개.
- 해당 storypack의 encounter situation card 최소 6개.
- 6스탯 기반 반복 NPC 후보 2~3명.
- 관련 README 또는 index 갱신.
- 후보와 runtime 승격 범위가 분리되어 있다는 설명.
- public-safe 검토 메모.

## 현재 기준 slice

현재 기준 예시는 `isolation_pack`이다.

- storypack: `docs/content/storypacks/isolation_pack.md`
- encounter cards: `docs/content/encounter_db/isolation_pack.md`
- recurrent NPCs: `docs/content/characters/recurrent_npcs.md`

새 설계자는 이 세 문서의 밀도와 형식을 참고하되, 내용을 그대로 복제하지 말고 새 pack의 surface/anomaly/pressure에 맞게 작성한다.
