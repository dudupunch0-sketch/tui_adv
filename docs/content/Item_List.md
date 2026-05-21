# 아이템 목록

이 문서는 런타임 기본 아이템 데이터 `src/tui_adv/data/items.yaml`의 공개 문서판이다.
실제 구현 기준 현재 아이템 수는 13개다.

## 목록 요약

| id | 이름 | 유형 | 사용 가능 | 태그 |
|---|---|---|---|---|
| `bottled_water` | 생수 | `consumable` | `true` | `survival`, `drink` |
| `coffee` | 커피 | `consumable` | `true` | `survival`, `office` |
| `snack` | 과자 | `consumable` | `true` | `survival`, `food` |
| `cup_noodle` | 컵라면 | `consumable` | `true` | `survival`, `food` |
| `first_aid_kit` | 구급상자 | `consumable` | `true` | `survival`, `medical` |
| `power_bank` | 보조배터리 | `tool` | `true` | `battery`, `tool` |
| `flashlight` | 손전등 | `tool` | `false` | `light` |
| `employee_badge` | 사원증 | `key` | `false` | `access` |
| `security_override_badge` | 보안실 우회권한 | `key` | `false` | `security`, `access`, `conquest` |
| `crumpled_printout` | 구겨진 출력물 | `clue` | `false` | `printer`, `reality_link` |
| `ex_employee_memo` | 퇴사자의 메모 | `clue` | `false` | `truth`, `internal_network` |
| `parking_key_fob` | 지하주차장 키태그 | `key` | `false` | `escape`, `parking`, `access` |
| `visitor_badge` | 임시 방문증 | `key` | `false` | `escape`, `lobby`, `access` |

## 상세

### bottled_water: 생수

아직 밀봉된 생수병. 지금은 복지보다 생존에 가깝다.

- 유형: `consumable`
- 태그: `survival`, `drink`
- 직접 사용: `true`
- 사용 효과: `thirst: -35`
- 사용 로그: 생수를 마셨다. 목 안쪽에서 사무실 먼지 맛이 조금 사라졌다.

### coffee: 커피

식어가는 커피. 회사에서 가장 흔한 각성제이자 가장 약한 위로.

- 유형: `consumable`
- 태그: `survival`, `office`
- 직접 사용: `true`
- 사용 효과: `sanity: +5`, `thirst: +8`
- 사용 로그: 커피를 마셨다. 정신은 조금 돌아왔지만 입안은 더 말랐다.

### snack: 과자

누군가 비상용으로 숨겨둔 과자. 이제 정말 비상용이 되었다.

- 유형: `consumable`
- 태그: `survival`, `food`
- 직접 사용: `true`
- 사용 효과: `hunger: -25`
- 사용 로그: 과자를 먹었다. 부스러기가 키보드보다 바닥에 더 많이 떨어졌다.

### cup_noodle: 컵라면

야근의 오래된 동료. 물을 넣을 수 있다면 아직 희망이 있다.

- 유형: `consumable`
- 태그: `survival`, `food`
- 직접 사용: `true`
- 사용 효과: `hunger: -45`, `thirst: +10`, `sanity: +3`
- 사용 로그: 컵라면을 먹었다. 익숙한 야근 맛이 잠깐 현실감을 붙잡아 주었다.

### first_aid_kit: 구급상자

비상용 구급상자. 누군가 한 번 열어본 흔적이 있다.

- 유형: `consumable`
- 태그: `survival`, `medical`
- 직접 사용: `true`
- 사용 효과: `health: +30`
- 사용 로그: 구급상자를 사용했다. 플라스틱 붕대 냄새가 상처보다 먼저 덮였다.

### power_bank: 보조배터리

충전량이 애매하게 남은 보조배터리. 그래도 어둠 속에서는 애매한 희망도 희망이다.

- 유형: `tool`
- 태그: `battery`, `tool`
- 직접 사용: `true`
- 사용 효과: `battery: +35`
- 사용 로그: 보조배터리를 연결했다. 단말기가 짧게 진동하며 아직 살아 있음을 알렸다.

### flashlight: 손전등

작은 손전등. 회사 지급품은 아니고 누군가의 개인 물건 같다.

- 유형: `tool`
- 태그: `light`
- 직접 사용: `false`

### employee_badge: 사원증

당신의 사원증. 사진 속 당신은 아직 이 상황을 모른다.

- 유형: `key`
- 태그: `access`
- 직접 사용: `false`

### security_override_badge: 보안실 우회권한

보안실 콘솔에서 임시로 발급한 출입 권한. 층수 기록이 맞지 않는 문에만 통한다.

- 유형: `key`
- 태그: `security`, `access`, `conquest`
- 직접 사용: `false`

### crumpled_printout: 구겨진 출력물

복합기에서 나온 구겨진 출력물. 문서 양식은 사내 표준인데, 내용은 아직 일어나지 않은 일을 말한다.

- 유형: `clue`
- 태그: `printer`, `reality_link`
- 직접 사용: `false`

### ex_employee_memo: 퇴사자의 메모

사내망 캐시에 남은 전임자의 메모. 문장은 회사 업무 양식처럼 차분하지만 내용은 격리 프로토콜을 가리킨다.

- 유형: `clue`
- 태그: `truth`, `internal_network`
- 직접 사용: `false`

### parking_key_fob: 지하주차장 키태그

시동이 켜진 차 안쪽에서 발견한 작은 키태그. 차종 로고 대신 사내 자산번호가 붙어 있다.

- 유형: `key`
- 태그: `escape`, `parking`, `access`
- 직접 사용: `false`

### visitor_badge: 임시 방문증

무인 로비 키오스크가 방금 뱉어낸 방문증. 방문 목적은 '퇴근 승인 대기'로 인쇄되어 있다.

- 유형: `key`
- 태그: `escape`, `lobby`, `access`
- 직접 사용: `false`
