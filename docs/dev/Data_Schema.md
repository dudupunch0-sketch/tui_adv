# 데이터 스키마 설계

## 목적

`escape from the office`의 공개 런타임 콘텐츠는 `src/tui_adv/data/*.yaml`에서 관리한다.
이 문서는 현재 Python/Textual·legacy TypeScript mirror가 읽는 YAML schema와, 전환 목표인 Rust GameCore/Web Storybook/SuperLightTUI가 공유할 renderer-neutral wire contract를 함께 기록한다.

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

## 2026-05-22 Rust renderer-neutral schema direction

이 문서는 이제 두 층을 함께 기록한다.

1. 현재 Python/Textual 및 legacy TypeScript mirror가 실제로 읽는 YAML authoring schema.
2. 새 목표 구조인 Rust GameCore + Web Storybook + SuperLightTUI terminal renderer가 공유할 renderer-neutral wire contract.

책임 분리는 다음 문서를 따른다.

- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`: crate/renderer 책임과 migration 흐름.
- 이 문서: YAML, generated bundle, `ScenePage`, `ActionResult`, `EffectCue`, WASM JSON boundary의 data shape.

핵심 원칙:

```text
YAML authoring data
  -> renderer-neutral content.bundle.json
  -> Rust GameCore state/action/result
  -> ScenePage semantic page
  -> Web Storybook or SuperLightTUI terminal rendering
```

Renderer는 action eligibility, outcome, ending, achievement를 다시 계산하지 않는다. Renderer는 core가 제공한 action id와 semantic cue만 표시하고, 선택된 action id를 다시 core에 전달한다.

## Authoring schema vs runtime contract

| 층 | 소유 | 용도 | renderer-specific 데이터 허용 |
|---|---|---|---|
| `src/tui_adv/data/*.yaml` | human-authored content | 원본 공개 콘텐츠 | no |
| `content.bundle.json` | exporter output | Rust/Web 공통 runtime input | no |
| `TurnView` | `escape-core` | low-level runner/debug compatibility | no |
| `ScenePage` | `escape-core` | renderer-ready semantic page | no |
| Web visual catalog | `web/src/ui/storybook/*` | `visual_id` -> DOM/CSS/SVG/Canvas/image | yes |
| Terminal visual catalog | `crates/escape-terminal/*` | `visual_id` -> ASCII/Unicode/ANSI/SuperLightTUI cells | yes |

`ScenePage`와 `content.bundle.json`에는 CSS class, pixel coordinate, Canvas command, terminal color object, SuperLightTUI type, DOM selector, image file path를 넣지 않는다.

## Presentation metadata

중요한 장면은 YAML에 optional `presentation` metadata를 가질 수 있다. 이 metadata는 renderer별 구현 세부가 아니라 semantic hint다.

Encounter 예:

```yaml
encounters:
  - id: printer_prints_alone
    title: 복합기가 혼자 출력한다
    body: 꺼져 있던 복합기가 아직 하지 않은 선택을 출력한다.
    presentation:
      visual_id: printer_anomaly
      speaker: 시스템 복합기
      layout: anomaly_object
      effect_cues:
        - kind: glyph_anomaly
          source: copier_output
          intensity: 0.72
          stable_terms: [비상계단, 토너, 접힌 방향]
          distortion: reflow_then_stabilize
    conditions:
      locations: [printer_area]
```

Location 예:

```yaml
locations:
  - id: hallway
    name: 복도
    description: 비상등만 남은 복도.
    presentation:
      visual_id: office_corridor_static
      layout: location_page
```

규칙:

- `presentation`은 optional이다.
- `presentation.visual_id`가 없으면 core는 deterministic fallback id를 만든다.
  - encounter page: `encounter:<encounter_id>`
  - movement/location page: `location:<location_id>`
  - ending page: `ending:<ending_id>`
- `visual_id`는 semantic id다. `web/assets/printer.webp` 같은 path를 넣지 않는다.
- `speaker`는 dialogue/body hint다. Renderer는 이를 말풍선, speaker label, terminal prefix 등으로 해석할 수 있다.
- `layout`은 semantic layout kind다. CSS class나 SuperLightTUI widget name이 아니다.
- `effect_cues[].stable_terms`는 Web/terminal/reduced-motion 모두에서 읽을 수 있어야 한다.
- 공개 YAML에는 실제 회사 위치, 실제 내부망 주소, 실제 사람 이름, 실제 최종 보물 위치, `final_hint`를 넣지 않는다.

초기 visual id 후보:

| id | 의미 | Web 해석 | Terminal 해석 |
|---|---|---|---|
| `opening_messenger` | 퇴사자 메신저/오프닝 text-first scene | 문서/메신저형 card | text-first page + small message frame |
| `printer_anomaly` | 복합기 출력/현실 연결 단서 | Canvas/GlyphFX + visual card | ASCII 복합기 card + cell GlyphFX |
| `office_corridor_static` | 이동/fallback location page | 정적인 복도 visual card | Unicode corridor card |

Unknown visual id는 빈 화면이 아니라 safe placeholder를 표시한다. Placeholder는 `visual.alt` 또는 title을 보여주고, action 선택은 계속 가능해야 한다.

## Renderer-neutral `content.bundle.json`

Rust fixture와 Web runtime은 같은 bundle schema를 사용한다.

권장 artifact paths:

```text
crates/escape-core/fixtures/content/content.bundle.json
web/src/data/generated/content.bundle.json
```

초안 shape:

```json
{
  "schema_version": 1,
  "kind": "tui_adv.content_bundle",
  "source": "src/tui_adv/data/*.yaml",
  "manifest": {
    "schema_version": 1,
    "source": "src/tui_adv/data/*.yaml",
    "counts": {
      "locations": 16,
      "items": 13,
      "encounters": 20,
      "endings": 13,
      "achievements": 11,
      "secrets": 3
    }
  },
  "content": {
    "locations": [],
    "items": [],
    "encounters": [],
    "endings": [],
    "achievements": [],
    "secrets": []
  }
}
```

Bundle 규칙:

- Bundle은 public-safe 데이터만 포함한다.
- `presentation` metadata는 semantic field만 보존한다.
- `final_hint`, `actual_ip_address`, `office_location`, `treasure_location` 같은 private-only field는 bundle, WASM fixture, Web generated data에 들어가면 안 된다.
- Exporter check는 Rust fixture bundle과 Web runtime bundle이 둘 다 최신인지 확인해야 한다.

## Action ID contract

Core가 renderer에 제공하는 action id는 그대로 `apply_action_from_content` 또는 WASM `apply_action_json`에 전달 가능한 값이어야 한다.

| prefix | 예 | 의미 |
|---|---|---|
| `choice:` | `choice:check_message` | 현재 encounter choice 실행 |
| `move:` | `move:dev_office` | 현재 위치에서 인접 위치로 이동 |
| `use:` | `use:bottled_water` | 사용 가능한 inventory item 소비 |

Renderer 규칙:

- 표시 번호(`1`, `2`, `3`)는 renderer-local이다.
- 실제 실행은 항상 `SceneAction.id`로 한다.
- Web button, number key, terminal number key, 이동 단축키는 모두 같은 core action id로 수렴한다.
- Renderer는 조건을 재계산하지 않는다. 비활성 행동은 `blocked_actions`의 reason을 표시한다.
- 알 수 없는 action id나 현재 turn에 없는 stale action id는 사용자-facing error로 처리하고 state를 변경하지 않는다.

## Low-level core contracts

`TurnView`는 현재 runner/debug compatibility용 low-level view다. 새 renderer는 가능하면 `ScenePage`를 사용하지만, migration 중 `TurnView`는 유지할 수 있다.

```json
{
  "location_id": "dev_desk",
  "encounter_id": "ex_employee_messenger",
  "title": "퇴사자의 메신저",
  "body": "꺼진 사내 메신저가 다시 켜졌다.",
  "actions": [
    { "id": "choice:check_message", "label": "메시지를 확인한다" }
  ],
  "blocked_actions": [
    { "id": "choice:trace_network", "label": "라우팅을 추적한다", "reasons": ["필요: 단말기 전원 충분"] }
  ],
  "effect_cues": []
}
```

`ActionResult` 초안:

```json
{
  "action_id": "choice:check_message",
  "source_id": "ex_employee_messenger",
  "state": { "...": "GameState" },
  "logs": ["메시지 마지막 줄만 도착했다."],
  "newly_unlocked_achievements": [],
  "effect_cues": []
}
```

`SaveEnvelope` 초안:

```json
{
  "schema_version": 1,
  "state": {
    "seed": 123,
    "turn": 1,
    "location_id": "dev_desk",
    "danger": 0,
    "player": {
      "health": 100,
      "sanity": 100,
      "battery": 100,
      "hunger": 0,
      "thirst": 0
    },
    "inventory": [],
    "clues": [],
    "flags": [],
    "seen_encounters": [],
    "unlocked_achievements": [],
    "log": []
  }
}
```

Save/load 규칙:

- JSON envelope만 사용하고 pickle/runtime object를 저장하지 않는다.
- unsupported `schema_version`은 traceback이 아니라 사용자-facing error다.
- Web localStorage는 core save JSON을 감싸는 저장 표면이다. Web-only rule state를 추가하지 않는다.

## `ScenePage` JSON contract

`ScenePage`는 Web Storybook과 SuperLightTUI terminal이 공유하는 renderer-ready semantic page다.

```json
{
  "mode": "encounter",
  "title": "퇴사자의 메신저",
  "location": {
    "id": "dev_desk",
    "name": "내 자리",
    "description": "당신의 모니터는 아직 켜져 있다."
  },
  "chapter_label": "격리 1턴",
  "status_summary": {
    "turn": 1,
    "danger": 0,
    "resources": [
      { "id": "health", "label": "신체 반응", "band": "normal", "text": "정상 범위", "value": 100 },
      { "id": "sanity", "label": "집중도", "band": "normal", "text": "안정", "value": 100 },
      { "id": "battery", "label": "단말기 전원", "band": "normal", "text": "100%", "value": 100 }
    ],
    "warnings": []
  },
  "body_blocks": [
    { "kind": "narration", "text": "꺼진 사내 메신저가 다시 켜졌다.", "source_id": "ex_employee_messenger" }
  ],
  "dialogue_entries": [
    { "speaker": "퇴사자", "text": "아직 네가 하지 않은 선택이 있어.", "source_id": "ex_employee_messenger" }
  ],
  "visual": {
    "id": "opening_messenger",
    "kind": "message_panel",
    "alt": "꺼진 사내 메신저 창에 마지막 문장이 떠 있다.",
    "source_id": "ex_employee_messenger"
  },
  "actions": [
    { "id": "choice:check_message", "label": "메시지를 확인한다", "kind": "choice", "cost_text": "단말기 전원 소모" }
  ],
  "blocked_actions": [],
  "history_entries": [],
  "inventory_summary": { "items": [], "overflow_count": 0 },
  "achievement_summary": { "unlocked": [], "newly_unlocked": [] },
  "pressure_cues": [],
  "effect_cues": []
}
```

필드 규칙:

| 필드 | 설명 |
|---|---|
| `mode` | `encounter`, `movement`, `ending` 중 하나 |
| `title` | 현재 page title. Renderer title bar/body title로 사용 |
| `location` | 현재 위치 요약. location page가 아니어도 항상 가능하면 제공 |
| `chapter_label` | 턴/챕터/격리 단계 같은 짧은 label |
| `status_summary` | core가 계산한 resource/status band와 warning |
| `body_blocks` | narrative/document/system text block |
| `dialogue_entries` | speaker가 있는 대화/메신저/시스템 발화 |
| `visual` | semantic visual id와 alt text |
| `actions` | 현재 실행 가능한 action list |
| `blocked_actions` | 보여줄 가치가 있는 비활성 action과 reason |
| `history_entries` | GameState log/action result에서 파생한 최근 기록 |
| `inventory_summary` | renderer가 짧게 표시할 inventory 요약 |
| `achievement_summary` | 기존/신규 업적 요약 |
| `pressure_cues` | 저정신력, 고갈증, 저전원 등 pressure presentation cue |
| `effect_cues` | GlyphFX/anomaly 등 장면 효과 cue |

`body_blocks[].kind` 후보:

- `narration`
- `dialogue`
- `document`
- `system`
- `warning`
- `clue`

`history_entries[].kind` 후보:

- `system`
- `dialogue`
- `action`
- `clue`
- `warning`
- `achievement`

History 규칙:

- `ActionResult.logs`는 다음 page의 `history_entries`에 ordered append된다.
- Opening message는 status/debug text가 아니라 story body 또는 dialogue entry로 표시한다.
- Web drawer와 terminal recent history는 같은 `history_entries`를 사용한다.
- 신규 업적은 `achievement_summary.newly_unlocked`와 history entry 양쪽에 표시할 수 있다.

## `EffectCue` and `PressureCue` contract

`EffectCue`는 core가 발행하는 semantic presentation cue다. Renderer가 encounter title/body를 보고 효과를 추측하지 않는다.

```json
{
  "kind": "glyph_anomaly",
  "source": "copier_output",
  "intensity": 0.72,
  "stable_terms": ["비상계단", "토너", "접힌 방향"],
  "distortion": "reflow_then_stabilize",
  "duration_hint_ms": 1800,
  "fallback_text": "출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다."
}
```

규칙:

- `intensity`는 `0.0`부터 `1.0`까지의 float로 시작한다.
- `stable_terms`는 public-safe clue terms만 담는다.
- Web은 Canvas/GlyphFX로 해석할 수 있다.
- Terminal은 SuperLightTUI cell/ASCII/ANSI effect로 해석할 수 있다.
- Reduced-motion, no-canvas, plain SSH/WSL fallback에서도 `fallback_text`나 final stable terms를 읽을 수 있어야 한다.

`PressureCue` 초안:

```json
{
  "kind": "low_sanity",
  "severity": "warning",
  "message": "집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.",
  "resource_id": "sanity"
}
```

Pressure threshold 판단은 core가 한다. Renderer는 cue의 severity/message를 presentation에 맞게 표시한다.

## WASM JSON boundary contract

`escape-wasm`은 초기에는 typed binding보다 JSON string boundary를 사용한다.

```text
new_game_json(seed: u64, content_bundle_json: &str) -> Result<String, Error>
scene_page_json(state_json: &str, content_bundle_json: &str) -> Result<String, Error>
apply_action_json(state_json: &str, content_bundle_json: &str, action_id: &str) -> Result<String, Error>
save_state_json(state_json: &str) -> Result<String, Error>
load_state_json(save_json: &str) -> Result<String, Error>
```

반환 규칙:

- success는 JSON string이다.
  - `new_game_json`: `GameState`
  - `scene_page_json`: `ScenePage`
  - `apply_action_json`: `ActionResult` 또는 갱신된 `GameState` + result metadata
  - `save_state_json`: `SaveEnvelope`
  - `load_state_json`: `GameState`
- failure는 Web이 error panel에 표시할 수 있는 사용자-facing message를 포함해야 한다.
- schema version mismatch, invalid bundle, unknown action, stale action, malformed save는 panic/traceback이 아니라 정상 error path다.

## Cross-renderer semantic parity

Web과 terminal parity는 rendered string이 아니라 semantic data로 비교한다.

비교 대상:

- final `GameState`
- available `ScenePage.actions[*].id`
- `blocked_actions` reason의 존재 여부
- resource deltas
- flags/clues/items
- ending id/kind
- unlocked achievements
- public reward metadata
- emitted `EffectCue` / `PressureCue`
- `ScenePage.mode`, `title`, `visual.id`, `body_blocks`, `history_entries`

Web/terminal의 색, 줄바꿈, Canvas frame, ASCII art 모양은 같을 필요가 없다. 단, 같은 action id를 실행했을 때 core 결과와 stable clue terms는 같아야 한다.

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
공개 `secrets.example.yaml`에는 `final_hint`, `actual_ip_address`, `office_location`, `treasure_location` 같은 private-only 필드를 넣지 않는다. Python 검증과 브라우저 export guard가 이를 차단한다.

## 브라우저 generated JSON

브라우저 앱은 YAML을 런타임에 직접 읽지 않는다. 다음 명령으로 공개 YAML을 정렬된 JSON으로 변환한다.

```bash
python scripts/export_web_data.py --write
python scripts/export_web_data.py --check
```

생성 위치:

```text
web/src/data/generated/locations.json
web/src/data/generated/items.json
web/src/data/generated/encounters.json
web/src/data/generated/endings.json
web/src/data/generated/achievements.json
web/src/data/generated/secrets.example.json
web/src/data/generated/manifest.json
```

`manifest.json`은 schema_version과 각 루트 키의 항목 수를 담는다. 브라우저 테스트는 이 JSON을 import해서 TypeScript mirror core, secret guard, fake-TUI renderer를 검증한다. 현재 mirror core는 생성 JSON 위에서 대표 terminal 루트 parity, 아이템 사용, 업적 해금, 능력치 판정, 자원 압박 상태를 다룬다.

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
python scripts/export_web_data.py --check
python -m pytest tests/test_content_data.py tests/test_secrets.py tests/test_web_data_export.py -q
cd web && npm test
```
