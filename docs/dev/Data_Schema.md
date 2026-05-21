# 데이터 스키마 설계

## 목적

`escape from the office`의 공개 런타임 콘텐츠는 `src/tui_adv/data/*.yaml`에서 관리한다.
이 문서는 현재 코드(`src/tui_adv/game/content.py`, `encounters.py`, `items.py`, `endings.py`, `secrets.py`)가 실제로 읽는 스키마만 기록한다.

현재 공개 데이터 수:

| 파일 | 루트 키 | 항목 수 |
|---|---|---:|
| `src/tui_adv/data/locations.yaml` | `locations` | 16 |
| `src/tui_adv/data/items.yaml` | `items` | 13 |
| `src/tui_adv/data/encounters.yaml` | `encounters` | 20 |
| `src/tui_adv/data/endings.yaml` | `endings` | 13 |
| `src/tui_adv/data/achievements.yaml` | `achievements` | 11 |
| `src/tui_adv/data/secrets.example.yaml` | `secrets` | 3 |

`private/secrets.local.yaml`은 실제 최종 위치를 담을 수 있으므로 git에 올라가면 안 된다.
공개 예시와 문서에는 실제 회사명, 층, 좌석, 개인명, 내부망 실주소, 최종 보물 위치를 쓰지 않는다.

## 공통 규칙

- 모든 엔티티는 고유한 `id`를 가진다.
- id는 소문자 snake_case를 사용한다.
- 공개 콘텐츠 참조 무결성은 `validate_public_content()`와 `tests/test_content_data.py`에서 검증한다.
- 위치 연결, 인카운터/엔딩 조건의 위치·아이템 참조, 인카운터 결과의 이동 목적지·아이템 참조, public secret의 `final_hint` 누락 정책을 검사한다.
- 자원 이름은 `health`, `sanity`, `battery`, `hunger`, `thirst`만 사용한다.
- `health`, `sanity`, `battery`는 높을수록 좋고, `hunger`, `thirst`는 낮을수록 좋다.

## Conditions

인카운터, 선택지, 엔딩, 업적에서 쓰는 조건 구조다. 모든 조건은 AND로 처리한다.
OR가 필요하면 항목을 나누거나 별도 선택지를 만든다.

```yaml
conditions:
  locations: [pantry]
  disaster_types: [unknown_isolation]
  required_items: [crumpled_printout]
  required_clues: [future_choice_printout]
  required_flags: [printer_secret_started]
  forbidden_flags: [thirst_hallucination_seen]
  min_resources:
    thirst: 60
  max_resources:
    sanity: 80
  min_abilities:
    interface: 4
```

로더는 과거 문서 호환을 위해 `has_items`, `has_clues`, `has_flags`, `missing_flags`도 읽지만, 새 데이터에는 `required_*`/`forbidden_flags`를 사용한다.

## Outcome

선택 결과 또는 판정 성공/실패 결과다.

```yaml
outcome:
  resources:
    sanity: -4
    thirst: -25
  add_items: [bottled_water]
  remove_items: [snack]
  add_clues: [water_dispenser_false_sound]
  add_flags: [thirst_hallucination_seen]
  remove_flags: [temporary_flag]
  destination_id: hallway
  danger: -1
  log: 정수기 전원을 뽑자 물소리가 멈췄다.
```

`health`, `sanity`, `battery`, `hunger`, `thirst`는 `resources` 안에 둘 수도 있고 outcome 최상위에 둘 수도 있다.
현재 공개 YAML은 두 형식을 모두 사용한다.

## locations.yaml

```yaml
locations:
  - id: dev_desk
    name: 내 자리
    description: 당신의 모니터는 아직 켜져 있다.
    connections: [dev_office]
    tags: [office, start, messenger, personal]
    danger: 0
```

필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 위치 id |
| `name` | yes | 표시 이름 |
| `description` | yes | 위치 설명 |
| `connections` | yes | 이동 가능한 위치 id 목록 |
| `tags` | no | 위치 분류 태그 |
| `danger` | no | 위치 위험도, 기본 0 |

## items.yaml

```yaml
items:
  - id: bottled_water
    name: 생수
    description: 아직 밀봉된 생수병.
    type: consumable
    tags: [survival, drink]
    usable: true
    use_effects:
      thirst: -35
    use_log: 생수를 마셨다.
```

필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 아이템 id |
| `name` | yes | 표시 이름 |
| `description` | yes | 설명 |
| `type` | yes | `consumable`, `tool`, `key`, `clue` 등 |
| `tags` | no | 분류 태그 |
| `usable` | no | `use:<item_id>` 행동으로 직접 사용할 수 있는지 |
| `use_effects` | no | 직접 사용 시 자원 변화량 |
| `use_log` | no | 직접 사용 시 로그 |

사용 가능한 아이템은 사용 후 인벤토리에서 제거된다.

## encounters.yaml

```yaml
encounters:
  - id: printer_prints_alone
    title: 복합기가 혼자 출력한다
    body: 꺼져 있던 복합기가 아직 하지 않은 선택을 출력한다.
    conditions:
      locations: [printer_area]
    choices:
      - id: take_printout
        label: 출력물을 챙긴다
        outcome:
          add_items: [crumpled_printout]
          add_flags: [printer_secret_started]
          log: 따뜻한 출력물을 접어 주머니에 넣었다.
```

인카운터 필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 인카운터 id |
| `title` | yes | 표시 제목 |
| `body` | yes | 본문 |
| `conditions` | no | 발생 조건 |
| `choices` | yes | 선택지 목록 |
| `repeatable` | no | 반복 가능 여부, 기본 false |
| `weight` | no | 랜덤 선택 가중치, 기본 1 |

선택지 필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 인카운터 내부 선택지 id |
| `label` | yes | 표시 문구 |
| `conditions` | no | 선택 가능 조건 |
| `cost` | no | 선택 즉시 지불 비용. 좋은 자원은 감소, 압박 자원은 증가한다. |
| `outcome` | yes | 기본 결과 |
| `check` | no | `2d6 + ability >= difficulty` 판정 분기 |

판정 예:

```yaml
check:
  ability: interface
  difficulty: 10
  success:
    add_clues: [delayed_packet_route]
    add_flags: [network_truth_hint]
    log: 숨은 라우팅을 찾았다.
  failure:
    resources:
      sanity: -4
    danger: 1
    log: 패킷이 역으로 단말을 훑고 지나갔다.
```

## endings.yaml

```yaml
endings:
  - id: hidden_reality_hint_001
    name: 첫 번째 현실 연결 힌트
    kind: hidden
    priority: 80
    conditions:
      required_items: [crumpled_printout]
      required_flags: [printer_secret_started, pantry_hint_seen]
      min_resources:
        health: 1
        sanity: 1
    local_hint_id: real_note_001
    text: >
      출력물의 마지막 문장은 더 이상 게임 속 장소만을 말하지 않았다.
```

필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 엔딩 id |
| `name` | yes | 표시 이름 |
| `kind` | yes | `failure`, `escape`, `hidden`, `truth`, `conquest` |
| `priority` | yes | 여러 엔딩 조건이 동시에 맞을 때 높은 값 우선 |
| `conditions` | no | 엔딩 조건 |
| `local_hint_id` | no | 현실 연결 힌트 id |
| `text` | yes | 엔딩 본문 |

체력 0과 정신력 0 실패 엔딩은 YAML이 아니라 `src/tui_adv/game/endings.py`의 `_FAILURE_ENDINGS`에서 즉시 판정한다.

## achievements.yaml

```yaml
achievements:
  - id: truth_protocol_understood
    name: 격리 프로토콜 독해
    description: 회의록과 CCTV 지연 프레임에서 격리 프로토콜의 실체를 읽었다.
    hidden: true
    conditions:
      required_flags: [isolation_protocol_revealed]
```

필드:

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | yes | 업적 id |
| `name` | yes | 표시 이름 |
| `description` | yes | 설명 |
| `hidden` | no | 히든 업적 여부, 기본 false |
| `conditions` | no | 해금 조건 |

## secrets.example.yaml

공개 가능한 현실 연결 힌트 예시 데이터다. 실제 최종 위치는 쓰지 않는다.

```yaml
secrets:
  - id: real_note_001
    title: 첫 번째 현실 연결 힌트
    unlock_flags:
      - printer_secret_started
      - pantry_hint_seen
    public_hint_steps:
      - 커피 냄새가 남아 있는 방을 기억한다.
    puzzle_prompt: 복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.
    placeholder_ip_address: 192.168.0.42
    final_hint_policy: private_only
    reward_text: 이 힌트는 게임 안의 장소만을 말하는 것 같지 않다.
```

`placeholder_ip_address`는 더미 값이다. 내부망 실주소가 필요하면 `private/secrets.local.yaml`의 `actual_ip_address`에만 둔다.

## private/secrets.local.yaml

로컬 전용 파일이다. 공개 저장소에 커밋하지 않는다.

```yaml
secrets:
  - id: real_note_001
    final_hint: "로컬 비공개 환경에서만 표시할 최종 힌트 문장"
    actual_ip_address: "10.20.30.40"
    safety_checked: true
```

런타임은 local secret이 없으면 공개 힌트와 placeholder 퍼즐 답까지만 표시한다.
local secret이 있으면 `final_hint`와 `actual_ip_address` 기반 숫자 합계를 추가로 표시할 수 있다.

## 검증 명령

```bash
PYTHONPATH=src python -m tui_adv --new --seed 123
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123
python -m pytest tests/test_content_data.py tests/test_secrets.py -q
```
