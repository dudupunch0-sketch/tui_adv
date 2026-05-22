# 인카운터 상황 DB

Status: 후보 콘텐츠 DB

이 폴더는 storypack 아이디어를 runtime encounter로 바로 넣기 전에 “상황 카드” 형태로 정리한다.

상황 카드는 아직 `src/tui_adv/data/encounters.yaml`의 확정 인카운터가 아니다. 카드의 목적은 좋은 랜덤 인카운터 후보를 축적하고, 안전/톤/구조 검토 후 일부만 승격하는 것이다.

## 왜 runtime encounter와 분리하는가

- 큰 스토리팩 아이디어를 작고 검토 가능한 단위로 쪼개기 위해서.
- LLM/agent가 오프라인으로 후보를 만들더라도 public-safe 검토를 거치기 위해서.
- 랜덤 인카운터가 메인 스토리 spine을 방해하지 않는지 확인하기 위해서.
- 아직 수치/조건/문장이 확정되지 않은 후보를 runtime YAML에 섞지 않기 위해서.

## 카드 필수 필드

```yaml
id:
status:
storypack_id:
phase:
priority_class:
location_tags:
surface:
anomaly_type:
pressure_type:
npc_slots:
candidate_characters:
summary:
setup_text:
choice_shapes:
outcome_hooks:
main_spine_link:
randomization_notes:
promotion_notes:
```

## 작성 규칙

1. 한 카드는 하나의 상황만 다룬다.
2. 최소 하나의 safe/fallback 선택지를 둔다.
3. 최소 하나의 flag/clue/item/resource/relation hook을 둔다.
4. `main_spine_link`가 없으면 승격하지 않는다.
5. 실제 회사/개인/내부 정보처럼 보이는 세부사항을 넣지 않는다.
6. hub 위치에서 이동을 막을 수 있으면 반드시 `randomization_notes`에 적는다.
7. runtime 승격 전에는 문장보다 구조와 기능을 먼저 검토한다.

## 현재 문서

- `isolation_pack.md`: 차원격리팩 첫 slice 후보 카드 6개.

## 승격 후보 선정 기준

runtime 승격 우선순위는 다음 순서로 본다.

1. 메인 story spine을 가장 잘 보강하는가?
2. 기존 runtime content와 겹치지 않는가?
3. TUI/fake-terminal surface가 분명한가?
4. 선택지가 실제로 다른 결과를 만드는가?
5. public-safe인가?
6. 기존 route 이동을 막지 않게 gating할 수 있는가?

## 관련 문서

- `docs/design/Storypack_Encounter_DB.md`
- `docs/content/storypacks/README.md`
- `docs/content/characters/README.md`
- `docs/design/Game_Loop.md`
- `docs/content/Encounter_List.md`
