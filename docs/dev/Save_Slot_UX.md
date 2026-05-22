# 저장 슬롯 이름 변경 UX 후보

이 문서는 현재 Textual 저장 슬롯 시작 화면에 “이름 변경” 기능을 추가할 때의 UX 후보를 정리한다.
현재 구현은 저장 슬롯 불러오기와 삭제까지 지원하며, 이름 변경은 아직 구현하지 않는다.
다음 실시간 UI/UX 점검에서 화면을 보며 조정할 수 있도록 최소 변경안을 기준으로 둔다.

## 현재 구현 계약

현재 Textual/TUI 저장 슬롯 흐름은 다음이다.

- `--tui --save <path>` 또는 `--tui-smoke --save <path>`는 저장 경로의 같은 디렉터리에서 최근 `*.json` 저장 파일을 찾는다.
- 시작 화면은 최근 저장 슬롯을 최대 5개까지 보여준다.
- 숫자 키는 저장 파일 불러오기다.
- `n`은 새 게임 시작이다.
- `d`는 삭제 모드 전환이고, 삭제 모드에서 숫자 키는 해당 저장 파일 삭제다.
- 목록 문구는 현재 `파일명 — 턴 N / 위치명` 또는 `파일명 — 읽기 실패` 형식이다.
- save JSON은 최상위 `schema_version`과 중첩된 `state` 객체를 가진다.
- `state` 안에는 `seed`, `turn`, `location_id`, `disaster_type`, `danger`, `player`, `inventory`, `clues`, `flags`, `seen_encounters`, `unlocked_achievements`, `log`가 저장된다.

## UX 목표

1. 플레이어가 `autosave.json` 같은 파일명 대신 기억하기 쉬운 별명을 볼 수 있어야 한다.
2. 기존 저장 파일과 깨진 저장 파일 처리를 망가뜨리지 않는다.
3. 삭제 모드와 숫자 불러오기 흐름을 혼동시키지 않는다.
4. 키보드만으로 빠르게 변경 가능해야 한다.
5. 다음 UI/UX 실시간 조정에서 문구와 위치를 쉽게 바꿀 수 있어야 한다.

## 권장안: save JSON 안에 선택적 `slot_name` 추가

파일명 자체를 바꾸기보다 save JSON 최상위에 선택적 표시 이름을 저장한다.
`slot_name`은 `state` 바깥의 metadata로 두어 게임 상태 로더와 저장 슬롯 표시 로직의 책임을 분리한다.

```json
{
  "schema_version": 1,
  "slot_name": "서버실 앞에서 멈춤",
  "state": {
    "seed": 123,
    "turn": 8,
    "location_id": "server_room_front",
    "disaster_type": "unknown_isolation",
    "danger": 2,
    "player": {
      "health": 100,
      "sanity": 100,
      "battery": 80,
      "hunger": 0,
      "thirst": 0,
      "abilities": {
        "logic": 2,
        "empathy": 2,
        "will": 2,
        "composure": 2,
        "interface": 2,
        "physical": 2
      }
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

호환 규칙:

- `slot_name`이 없으면 기존처럼 파일명을 표시한다.
- `slot_name`이 비어 있거나 공백뿐이면 파일명을 표시한다.
- 깨진 JSON은 읽기 실패 슬롯으로 유지하고 이름 변경 대상에서 제외한다.
- 실제 저장 경로는 바꾸지 않는다.
- 오래된 save는 로드 후 다시 저장할 때도 이름 없이 정상 동작해야 한다.

이 방식이 좋은 이유:

- 파일 이동/충돌/확장자 문제를 만들지 않는다.
- 기존 `--load saves/foo.json` 계약을 유지한다.
- 브라우저 localStorage 저장 이름 설계와도 나중에 맞추기 쉽다.

## 대안과 보류 이유

| 방식 | 장점 | 보류 이유 |
|---|---|---|
| 파일명 직접 변경 | 외부 파일 관리자에서도 이름이 보인다. | CLI `--load` 경로가 바뀌고, 중복/금지 문자/확장자 처리 UX가 커진다. |
| sidecar index 파일 | 슬롯 메타데이터를 save와 분리할 수 있다. | 동기화 실패와 파일 삭제 후 고아 메타데이터가 생긴다. 현재 규모에는 과하다. |
| 디렉터리별 슬롯 관리 | 장기적으로 확장성이 있다. | 현재 저장 시스템보다 큰 개편이라 다음 UI/UX polish 범위를 넘는다. |

## 시작 화면 키 흐름 후보

기본 모드:

```text
[시작]
숫자: 저장 파일 불러오기 / n: 새 게임 / d: 삭제 모드 / r: 이름 변경 모드

[저장 파일 목록]
1. 서버실 앞에서 멈춤 — 턴 8 / 서버실 앞
2. autosave.json — 턴 3 / 탕비실
3. broken.json — 읽기 실패
```

이름 변경 모드:

```text
[시작]
숫자: 이름 변경할 저장 파일 선택 / n: 새 게임 / Esc: 취소
```

슬롯 선택 후 입력 프롬프트:

```text
[저장 이름 변경]
선택한 슬롯: 서버실 앞에서 멈춤
새 이름 입력 후 Enter. 빈 값은 취소.
```

완료 로그:

```text
저장 슬롯 이름을 `서버실 앞에서 멈춤`에서 `정복 루트 직전`으로 바꿨다.
```

## 입력 규칙

- 시작 화면에서만 `r`을 이름 변경 모드로 쓴다.
- 플레이 중 `r`은 예약하지 않는다. 실시간 UI/UX 점검에서 필요해질 때 다시 결정한다.
- 이름 변경 모드에서 숫자 키는 슬롯 선택이다.
- `Esc` 또는 빈 입력은 취소다.
- 이름은 앞뒤 공백을 제거한다.
- 줄바꿈, 탭, 제어 문자는 공백 하나로 접는다.
- 표시 폭 기준 24~32자 사이에서 자르는 후보를 실시간 화면에서 확인한다.
- 중복 이름은 허용한다. 실제 식별자는 파일 경로다.
- 읽기 실패 슬롯은 이름 변경할 수 없다. 먼저 삭제하거나 파일을 고쳐야 한다.

## 패널 문구 후보

| 상태 | 문구 |
|---|---|
| 기본 | `숫자: 저장 파일 불러오기 / n: 새 게임 / d: 삭제 모드 / r: 이름 변경 모드` |
| 삭제 모드 | `숫자: 저장 파일 삭제 / n: 새 게임 / Esc: 취소` |
| 이름 변경 모드 | `숫자: 이름 변경할 저장 파일 선택 / n: 새 게임 / Esc: 취소` |
| 입력 중 | `새 이름 입력 후 Enter. 빈 값은 취소.` |
| 실패 | `읽기 실패 슬롯은 이름을 바꿀 수 없다.` |

## 구현 메모

최소 구현 slice는 다음 파일에 국한한다.

- `src/tui_adv/game/save.py`
  - 선택적 최상위 `slot_name` metadata 읽기/쓰기 또는 metadata helper 추가
  - 기존 `schema_version` + `state` 구조는 그대로 유지
- `src/tui_adv/tui/app.py`
  - `SaveSlot`에 `slot_name: str | None` 추가
  - `_format_save_slot_panel()`에서 `slot_name or path.name` 표시
  - 시작 화면 모드에 rename 상태 추가
- `src/tui_adv/main.py`
  - smoke 검증용 옵션은 필요할 때만 추가한다.
  - 예: `--rename-save-slot 1 --slot-name "정복 루트 직전"`
- `tests/test_tui_app.py`, `tests/test_save.py`, `tests/test_qa_smoke_script.py`
  - 기존 저장 파일 호환
  - 이름 표시 fallback
  - 읽기 실패 슬롯 rename 차단
  - 삭제 모드와 rename 모드 문구 충돌 방지

## 실시간 UI/UX 점검 때 볼 것

- `[저장 파일 목록]`에서 이름, 턴, 위치의 시각적 우선순위가 맞는가?
- `d` 삭제 모드와 `r` 이름 변경 모드가 한눈에 구분되는가?
- 긴 한글 이름이 패널 폭을 깨지 않는가?
- 이름 변경 완료/취소 피드백이 로그와 시작 패널 중 어디에 보이는 것이 자연스러운가?
- 새 게임과 저장 파일 불러오기가 rename flow 때문에 느려지지 않는가?

## 완료 기준 후보

- 기존 이름 없는 save가 그대로 불러와진다.
- 이름 있는 save는 목록에서 `slot_name`을 먼저 보여준다.
- 이름 변경 후 같은 파일 경로로 계속 저장/로드된다.
- 삭제 모드와 이름 변경 모드의 숫자 입력 의미가 섞이지 않는다.
- `--tui-smoke` 또는 Textual `run_test()`로 시작 화면 문구를 검증한다.
