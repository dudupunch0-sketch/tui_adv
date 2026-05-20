# 데이터 스키마 설계

## 목적

`escape from the office`의 위치, 아이템, 인카운터, 엔딩, 현실 연결 힌트는 가능한 한 코드가 아니라 데이터 파일로 관리한다.
이 문서는 1차 구현에서 사용할 YAML 스키마를 정의한다.

## 기본 원칙

1. 모든 엔티티는 고유한 `id`를 가진다.
2. id는 소문자 snake_case를 사용한다.
3. 공개 데이터에는 실제 사무실 최종 위치를 넣지 않는다.
4. 모든 참조 id는 데이터 검증 단계에서 확인한다.
5. 데이터 파일은 사람이 직접 작성하고 리뷰할 수 있어야 한다.
6. 저장 파일은 YAML이 아니라 JSON을 사용한다.

## 파일 목록

```text
src/tui_adv/data/locations.yaml
src/tui_adv/data/items.yaml
src/tui_adv/data/encounters.yaml
src/tui_adv/data/endings.yaml
src/tui_adv/data/secrets.example.yaml
private/secrets.local.yaml
```

`private/secrets.local.yaml`은 실제 최종 위치를 담을 수 있으므로 git에 올라가면 안 된다.

## 공통 타입

### ResourceDelta

자원 변화량이다.
양수/음수 방향은 자원 자체의 의미를 따른다.

```yaml
resources:
  health: -5
  sanity: 3
  battery: -8
  hunger: 10
  thirst: -20
```

주의:

- `health`, `sanity`, `battery`는 높을수록 좋다.
- `hunger`, `thirst`는 낮을수록 좋다.
- 따라서 `hunger: -20`은 배고픔 완화, `thirst: 15`는 갈증 악화다.

### Conditions

선택지, 인카운터, 엔딩 발생 조건이다.

```yaml
conditions:
  min_resources:
    battery: 3
    sanity: 40
  max_resources:
    thirst: 79
  min_abilities:
    interface: 4
    logic: 3
  has_items:
    - employee_badge
  missing_items:
    - broken_flashlight
  has_flags:
    - printer_secret_started
  missing_flags:
    - escaped_building
  locations:
    - pantry
  disaster_types:
    - unknown_isolation
  min_danger: 2
  max_danger: 5
```

모든 조건은 AND로 처리한다.
OR 조건이 필요하면 인카운터 또는 선택지를 여러 개로 나눈다.

### Effects

선택 결과로 상태를 바꾸는 구조다.

```yaml
effects:
  resources:
    sanity: -5
    battery: -3
  add_items:
    - crumpled_printout
  remove_items:
    - bottled_water
  add_clues:
    - printer_mentions_real_world
  add_flags:
    - printer_secret_started
  remove_flags:
    - safe_room_available
  move_to: hallway
  danger: 1
  log:
    - "복합기가 출력한 종이를 챙겼다."
```

`danger`는 현재 위험도에 더하는 delta다.

## locations.yaml

### 구조

```yaml
locations:
  - id: dev_desk
    name: 내 자리
    description: >
      당신의 모니터는 아직 켜져 있다. 사내 메신저 창이 깜빡인다.
    tags:
      - office
      - safeish
      - start
    connections:
      - dev_office
    danger: 0
    encounter_tags:
      - desk
      - messenger
```

### 필드

| 필드 | 필수 | 설명 |
|---|---|---|
| id | yes | 위치 고유 id |
| name | yes | 표시 이름 |
| description | yes | 위치 설명 |
| tags | no | 위치 분류 태그 |
| connections | yes | 이동 가능한 위치 id 목록 |
| danger | no | 위치 위험도, 기본 0 |
| encounter_tags | no | 인카운터 매칭용 태그 |

### 1차 위치 id

```yaml
- dev_desk
- dev_office
- hallway
- pantry
- meeting_room
- printer_area
- server_room_front
- emergency_stairs
- security_room
```

## items.yaml

### 구조

```yaml
items:
  - id: bottled_water
    name: 생수
    description: "아직 밀봉된 생수병. 지금은 복지보다 생존에 가깝다."
    type: consumable
    tags:
      - survival
      - drink
    usable: true
    use_effects:
      resources:
        thirst: -30
      remove_items:
        - bottled_water
      log:
        - "생수를 마셨다. 갈증이 가라앉았다."
```

### 필드

| 필드 | 필수 | 설명 |
|---|---|---|
| id | yes | 아이템 고유 id |
| name | yes | 표시 이름 |
| description | yes | 설명 |
| type | yes | `consumable`, `tool`, `key`, `clue`, `route` |
| tags | no | 검색/조건용 태그 |
| usable | no | 직접 사용 가능 여부 |
| use_effects | no | 사용 시 효과 |

### 주요 아이템 id

```yaml
- bottled_water
- coffee
- snack
- cup_noodle
- first_aid_kit
- power_bank
- flashlight
- employee_badge
- security_override_badge
- parking_key_fob
- visitor_badge
- crumpled_printout
- ex_employee_memo
```

## encounters.yaml

### 구조

```yaml
encounters:
  - id: printer_prints_alone
    name: 복합기가 혼자 출력한다
    disaster_types:
      - unknown_isolation
    locations:
      - printer_area
    tags:
      - printer
      - reality_link
    weight: 10
    once: true
    conditions:
      min_danger: 1
    description: >
      복합기는 절전 모드다. 그런데 내부에서 롤러가 돌아가는 소리가 난다.
      잠시 뒤, 아무도 요청하지 않은 출력물이 한 장 나온다.
    presentation:
      visual_frames:
        - printer_idle
        - printer_warmup
      stream_text: true
      ambient_messages:
        - "[printer] 대기열에 없는 작업을 수신했습니다."
      glitch_rules:
        - type: text_corruption
          when:
            min_danger: 2
          intensity: low
    choices:
      - id: read_page
        text: 출력물을 읽는다
        conditions:
          min_resources:
            sanity: 20
        cost:
          resources:
            sanity: -3
        outcome:
          type: fixed
          effects:
            add_clues:
              - printer_page_mentions_you
            add_flags:
              - printer_page_read
            log:
              - "출력물에는 당신의 이름과 아직 하지 않은 선택이 적혀 있었다."

      - id: take_page
        text: 출력물을 챙긴다
        outcome:
          type: fixed
          effects:
            add_items:
              - crumpled_printout
            add_flags:
              - printer_secret_started
            log:
              - "구겨진 출력물을 챙겼다. 종이가 이상하게 따뜻하다."
```

### 필드

| 필드 | 필수 | 설명 |
|---|---|---|
| id | yes | 인카운터 고유 id |
| name | yes | 표시 이름 |
| disaster_types | yes | 발생 재난 타입 목록 |
| locations | yes | 발생 위치 목록 |
| tags | no | 분류 태그 |
| weight | no | 랜덤 가중치, 기본 1 |
| once | no | 한 번만 발생 여부 |
| conditions | no | 발생 조건 |
| description | yes | 이벤트 본문 |
| presentation | no | ASCII 프레임, 스트리밍 텍스트, 글리치 같은 TUI 표현 힌트 |
| choices | yes | 선택지 목록 |

### presentation

`presentation`은 엔진 결과에 영향을 주지 않는 표현 레이어 데이터다.
TUI는 이 필드를 사용해 사내 시스템/로그/글리치 감각을 강화할 수 있다.
1차 구현에서는 모든 필드를 optional로 두고, 미지원 필드는 무시한다.

```yaml
presentation:
  visual_frames:
    - elevator_normal
    - elevator_shadow_1
  stream_text: true
  ambient_messages:
    - "[NETWORK QUALITY: POOR]"
  glitch_rules:
    - type: choice_text_mutation
      when:
        max_sanity: 30
      intensity: low
```

표현 데이터 원칙:

- `visual_frames`는 ASCII asset id만 참조한다.
- `stream_text`는 본문을 한 번에 출력하지 않고 순차 출력할지 나타낸다.
- `ambient_messages`는 사내망/시스템 로그처럼 보이는 주변 문구다.
- `glitch_rules`는 `docs/design/UI_Rules.md`의 허용 범위 안에서만 동작해야 한다.
- 입력 왜곡 같은 강한 개입은 기본 데이터 스키마에 넣지 않고, 별도 scripted encounter로 분리한다.

## Choice

### 필드

| 필드 | 필수 | 설명 |
|---|---|---|
| id | yes | 인카운터 내부 선택지 id |
| text | yes | 표시 텍스트 |
| conditions | no | 선택 가능 조건 |
| cost | no | 선택 즉시 적용 비용 |
| outcome | yes | 결과 |

## Outcome

### fixed

항상 같은 결과다.

```yaml
outcome:
  type: fixed
  effects:
    resources:
      thirst: -30
```

### check

2d6 + 능력치 기반 성공/실패다.

```yaml
outcome:
  type: check
  base_effects:
    resources:
      battery: -2
    log:
      - "휴대폰을 사내망에 붙였다."
  check:
    ability: interface
    difficulty: 10
  success:
    effects:
      add_flags:
        - meeting_pattern_noticed
  failure:
    effects:
      resources:
        sanity: -10
```

### chance

시드 기반 확률 결과다.

```yaml
outcome:
  type: chance
  chance: 0.7
  success:
    effects:
      add_items:
        - flashlight
  failure:
    effects:
      resources:
        health: -5
```

1차 구현은 `fixed`, `check`, `chance` 세 종류만 지원한다.

## endings.yaml

### 구조

```yaml
endings:
  - id: escape_emergency_stairs
    name: 비상계단 탈출
    type: escape
    priority: 60
    conditions:
      has_flags:
        - emergency_stairs_route_open
      locations:
        - emergency_stairs
      min_resources:
        health: 1
        sanity: 1
    text: >
      아래층 표시가 마지막으로 한 번 깜빡였다.
      문을 밀자, 차갑고 평범한 밤공기가 들어왔다.
```

### priority

여러 엔딩 조건이 동시에 만족될 때 높은 priority를 우선한다.
실패 엔딩은 일반적으로 가장 높다.

권장 범위:

- 100: 즉시 실패
- 80: 히든/특수 엔딩
- 70: 진실/정복 엔딩
- 60: 탈출 엔딩
- 10: 메타/중간 엔딩

## secrets.example.yaml

공개 가능한 예시 secret 데이터다.
실제 최종 위치를 넣지 않는다.

```yaml
secrets:
  - id: real_note_001
    title: 첫 번째 현실 연결 힌트
    unlock_flags:
      - printer_secret_started
      - pantry_hint_seen
    public_hint_steps:
      - "커피 냄새가 남아 있는 방."
      - "차가운 문과 종이 냄새 사이의 반복 표식."
      - "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다."
    puzzle_prompt: "복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다."
    placeholder_ip_address: "192.168.0.42"
    final_hint_policy: private_only
    reward_text: >
      이 힌트는 게임 안의 장소만을 말하는 것 같지 않다.
```

`placeholder_ip_address`는 공개 저장소용 더미 값이다. 실제 사무실 IP 주소를 쓸 때는 공개 파일 대신 로컬 secret 파일의 `actual_ip_address`에 둔다.

## private/secrets.local.yaml

비공개 로컬 파일이다.
이 파일은 `.gitignore`로 차단한다.
커밋 가능한 출발점은 `docs/templates/local-secrets.template.yaml`이다.
템플릿은 `template_only: true`, `safety_checked: false`, `TODO` 문장만 포함해야 한다.

```yaml
secrets:
  - id: real_note_001
    final_hint: "로컬 비공개 환경에서만 표시할 최종 힌트 문장"
    actual_ip_address: "10.20.30.40"
    safety_checked: true
    notes: "공용 공간, 위험 없음, 개인 물건 아님."
```

`actual_ip_address` 역시 내부망 정보일 수 있으므로 로컬 비공개 파일에만 둔다. 런타임은 IP 문자열의 숫자를 모두 더한 값을 퍼즐 답으로 계산한다.

공개 예시에는 `final_hint` 필드를 넣지 않는다.

## save JSON schema 초안

저장 파일은 JSON으로 한다.

```json
{
  "version": 1,
  "game_id": "generated-uuid",
  "seed": 12345,
  "turn": 12,
  "location_id": "pantry",
  "danger": 3,
  "disaster_type": "unknown_isolation",
  "player": {
    "health": 82,
    "sanity": 61,
    "battery": 34,
    "hunger": 43,
    "thirst": 58,
    "abilities": {
      "logic": 2,
      "empathy": 2,
      "volition": 2,
      "composure": 2,
      "interface": 4,
      "physical": 2
    }
  },
  "inventory": ["employee_badge", "crumpled_printout"],
  "clues": ["printer_page_mentions_you"],
  "flags": ["printer_secret_started"],
  "seen_encounters": ["printer_prints_alone"],
  "log": ["구겨진 출력물을 챙겼다."]
}
```

## 데이터 검증 규칙

앱 시작 또는 테스트에서 검증한다.

- 모든 id는 고유해야 한다.
- 모든 참조 id는 존재해야 한다.
- location exits는 존재하는 location id만 가리킨다.
- encounter locations는 존재하는 location id만 가리킨다.
- choice add/remove item은 존재하는 item id만 가리킨다.
- ending conditions의 item/flag/location 참조가 유효해야 한다.
- public secret 파일에는 `final_hint`가 없어야 한다.
- private secret 파일은 없어도 정상 실행되어야 한다.

## 1차 구현에서 의도적으로 제외

- 중첩 OR 조건
- 스크립트형 효과
- 외부 네트워크 호출
- 데이터 파일 내부 Python expression
- 실제 회사 위치가 들어간 공개 예시
