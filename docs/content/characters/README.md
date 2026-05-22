# 등장인물 DB

Status: 후보 콘텐츠 DB

이 폴더는 storypack과 encounter situation card에서 사용할 반복 등장인물 후보를 정리한다.

등장인물은 현재 runtime NPC 시스템이 아니라, 스토리팩/인카운터 작성용 캐릭터 시트다. 나중에 실제 게임 데이터로 승격할 경우 별도 schema와 validator가 필요하다.

## 기본 원칙

- 6개 core stat만 직접 수치화한다.
- 관계값은 stat이 아니라 별도 상태로 둔다.
- stat은 성격 장식이 아니라 인카운터 기능과 연결한다.
- 캐릭터는 회사 안에서 실제로 있을 법해야 한다.
- 비밀은 공개-safe 수준으로 요약한다.
- 실제 회사/인물/내부 정보처럼 보이는 세부사항을 넣지 않는다.

## 6 core stats

| stat | 한국어 | 의미 | 인카운터 기능 |
|---|---|---|---|
| `sense` | 감각 | 미세한 변화, 표정, 소리, 공간 위화감 감지 | 단서 발견, 거짓말 징후, CCTV/공간 이상 감지 |
| `social` | 사회 | 회사 인간관계와 말의 생태계에서 살아남는 능력 | 설득, 소문, 거짓말, 분위기 완충/조작 |
| `reason` | 이성 | 문서, 숫자, 규칙, 시스템, 절차 이해 | 로그 분석, 회의록 해석, 사내 시스템 퍼즐 |
| `self` | 자아 | 자기 자신을 유지하는 힘 | 오염/공포/존재 말소 저항 |
| `impulse` | 충동 | 순간 행동력, 도주, 기습, 감정적 추진력 | 빠른 구조, 돌발 배신, 무모한 선택 |
| `body` | 신체 | 물리적 능력과 존재감 | 추격/도주, 문 열기/막기, 부상 저항 |

## 수치 규칙

- 기본 총합은 60.
- 초기 캐릭터의 각 stat은 0~20.
- 특별 이유가 없으면 최소 3 이상.
- 21 이상은 runtime 성장, 오염, 회사 시스템 개입 같은 특수 상태에서만 허용한다.

## 관계 상태 후보

관계 상태는 core stat과 분리한다.

- `trust`
- `suspicion`
- `debt`
- `guilt`
- `contamination`
- `alive` / `missing` / `dead`
- `companion` / `remote_only` / `unknown`

## 현재 문서

- `recurrent_npcs.md`: 차원격리팩 첫 slice용 반복 NPC 후보 3명.

## 관련 문서

- `docs/design/Character_Stats_and_Generator.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/content/storypacks/isolation_pack.md`
- `docs/content/encounter_db/isolation_pack.md`
