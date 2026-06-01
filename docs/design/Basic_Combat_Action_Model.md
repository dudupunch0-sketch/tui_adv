# 기본 전투 액션 모델

Status: design candidate
Scope: docs-only; runtime YAML/Rust/Web/generated artifact 변경 없음
Primary reference storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**
Compatibility target: default office storypack / `escape from the office`
Related docs:
- `docs/design/Combat_System_Auto_Brawl.md`
- `docs/design/Storypack_World_Model.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/Encounter_List.md`

## 1. 문서 목적

이 문서는 `Combat_System_Auto_Brawl.md`의 “자동 난투 + 상황 개입” 철학을 실제 콘텐츠 작성자가 쓸 수 있는 기본 액션 문법으로 낮춘다.

이번 기준은 **이구학지 — 천기록**이다. 다만 전투 시스템 자체는 무협 전용이 아니어야 한다. 같은 구조가 office 기본팩에서도 스테이플러, 복합기, 케이블, 보안문, 회의실 의자 같은 표면으로 작동해야 한다.

따라서 이 문서의 목표는 다음이다.

1. 전투에서 “공격한다”가 무엇을 뜻하는지 분해한다.
2. 무협의 초식/무공과 office의 즉석 도구가 같은 상태축 위에서 작동하게 한다.
3. 콘텐츠 작성자가 choice id, label, outcome hook을 만들 때 참고할 taxonomy를 제공한다.
4. 지금 당장 새 `CombatState`나 combat resolver를 요구하지 않는 schema-less 설계선을 유지한다.

## 2. 핵심 결론

이 게임의 기본 전투 단위는 `attack command`가 아니라 `exchange`다.

```text
상황 발생
→ 자동 난투 exchange가 진행됨
→ 결정적 틈/opening이 생김
→ 플레이어가 intervention choice를 고름
→ 하나 이상의 전투 흐름 축이 변함
→ 전투가 결착되거나 다음 exchange로 넘어감
```

플레이어가 고르는 것은 “평타 / 방어 / 스킬”이 아니다. 플레이어는 다음 중 하나를 고른다.

- 거리를 바꾼다.
- 균형을 무너뜨리거나 회복한다.
- 압박을 만들거나 빠져나온다.
- 호흡과 자세를 되찾는다.
- 무기를 제어하거나 포기한다.
- 환경을 끌어들인다.
- 동료/관계/단서를 전투 흐름에 끼운다.
- 큰 위험을 감수하고 강행한다.

## 3. 기본 용어

| 용어 | 의미 | 구현상 현재 표현 |
|---|---|---|
| `exchange` | 자동으로 흘러간 짧은 공방/난투 단락 | ScenePage body/log 문장 |
| `opening` | 플레이어가 개입할 수 있는 틈 | body 문장 + choice list |
| `intervention` | 플레이어가 고르는 결정적 선택 | `SceneAction.id` / `choice:*` |
| `flow_axis` | 전투 흐름을 설명하는 상태축 | 현재는 텍스트/flags/clues/log/presentation |
| `consequence` | 선택 후 전투 흐름 변화 | resources/danger/flags/clues/items/destination/log |
| `fallback` | 항상 고를 수 있어야 하는 안전/후퇴/버티기 선택지 | 기존 choice 중 fallback 역할 명시 |

주의: 이 용어들은 지금 당장 runtime schema field가 아니다. 첫 단계에서는 문서/choice label/outcome log/presentation stable term으로 사용한다.

## 4. 전투 흐름 축

전투 선택지는 적어도 하나 이상의 축을 바꿔야 한다.

| 축 | 질문 | 이구학지 표면 | office 표면 | 대표 결과 hook |
|---|---|---|---|---|
| `distance` | 붙었나, 떨어졌나, 도망칠 수 있나? | 장터 골목, 목검 거리, 도포 소매, 담벼락 | 회의실 책상, 복합기 통로, 자동문 | `add_clues`, `danger`, `destination_id` |
| `balance` | 누가 휘청이거나 넘어질 위험에 있나? | 구두가 미끄러짐, 하단이 비었음, 목검이 걸림 | 카트 바퀴, 케이블, 의자, 토너 가루 | `resources.health`, `add_flags` |
| `pressure` | 누가 벽/군중/권력/다수에 몰렸나? | 흑사방 말단, 시장 군중, 청류문 평판 | 보안문, 회의실 유리벽, CCTV | `danger`, `add_clues` |
| `breath` | 숨이 찼나, 다음 행동 여유가 있나? | 구두/정장 때문에 호흡이 흐트러짐 | 좁은 물품창고, 소화기 분말 | `resources.health`, `resources.sanity` |
| `posture` | 공격/방어/도주 가능한 자세인가? | 넥타이/가방을 정리함, 발을 다시 딛음 | 의자 뒤로 물러남, 문틀을 잡음 | `add_flags`, `log` |
| `visibility` | 상대와 환경을 읽을 수 있나? | 청류안 암시, 먼지, 등불, 군중 | 손전등, 비상등, GlyphFX 오염 | `add_clues`, `effect_cues` |
| `weapon_control` | 무기를 쥐고 있나, 빼앗기나, 버리나? | 목검, 죽봉, 단검, 장검, 소매 | 스테이플러, 가방, 소화기, 케이블 | `add_items`, `remove_items`, `add_flags` |
| `environment` | 장소/사물이 전장 흐름을 바꾸나? | 좌판, 담벼락, 약재 포대, 장터 군중 | 복합기, 카트, 자동문, 케이블 | `presentation`, `add_clues`, `danger` |
| `nerve` | 겁/분노/당황이 판단을 흔드나? | 흑사방 위협, 문파 시선, 무명 압박 | 사내 방송, 잔상, 동료 패닉 | `resources.sanity`, `add_flags` |
| `ally_stability` | 동료/구조자가 버티고 있나? | 서하린, 무명, 청류문 제자 | 생존자/방송 목소리/동료 직원 | `add_clues`, `add_flags` |

첫 구현에서는 이 축들을 영구 수치로 저장하지 않는다. 단, choice authoring 단계에서는 “이 선택지가 어떤 축을 바꾸는가?”를 반드시 적는다.

## 5. 기본 액션 taxonomy

### 5.1 `strike` — 때리기 / 찌르기 / 치기

정의: 상대에게 직접 충격을 준다. 하지만 이 게임에서 `strike`의 목적은 HP를 깎는 것이 아니라 거리, 균형, 압박, 무기 제어를 바꾸는 것이다.

좋은 사용 조건:
- 상대가 휘청이거나 시야를 잃었다.
- 무기 든 손/다리/중심이 노출됐다.
- 때리는 행동이 다음 상태 변화를 만든다.

나쁜 사용:
- `attack_enemy` / “공격한다”만 있고 변화축이 없다.
- 피해량 숫자만 남고 전투 장면이 변하지 않는다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 목검 등으로 손등을 쳐 무기를 떨어뜨린다 | `weapon_control`, `balance` | 위험 감소, 단서/flag |
| 이구학지 | 발목을 낮게 차고 바로 물러난다 | `balance`, `distance` | health cost 가능, escape hook |
| office | 스테이플러 든 손목을 내려친다 | `weapon_control` | 부상 위험 감소 |
| office | 노트북 가방으로 한 번 막고 밀친다 | `distance`, `pressure` | item damage flag 가능 |

### 5.2 `shove` — 밀기 / 몰기 / 박기

정의: 상대 또는 자신을 특정 위치로 이동시켜 압박과 균형을 바꾼다.

핵심: 밀기는 단순 공격보다 “어디로 미는가”가 중요하다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 담벼락 쪽으로 밀어 시장 군중의 시야를 끊는다 | `pressure`, `visibility` | danger 조정, crowd clue |
| 이구학지 | 죽봉 끝을 밀어 상대 하단을 비운다 | `balance`, `distance` | 다음 선택지 유리 |
| office | 회의실 유리벽 쪽으로 어깨를 넣어 민다 | `pressure`, `balance` | health cost + advantage flag |
| office | 카트를 캐비닛에 걸어 통로를 막는다 | `distance`, `environment` | `improvised_distance_control` |

### 5.3 `grapple` — 붙잡기 / 클린치 / 묶기

정의: 거리를 없애고 서로의 행동을 제한한다.

장점:
- 강한 무기/긴 무기/큰 동작을 끊는다.
- 도주나 동료 구조 시간을 번다.

위험:
- 실패하면 더 크게 다친다.
- 다수전에서 오래 붙잡으면 불리하다.
- 출근복/가방/사원증 줄/도포 소매가 잡히는 약점이 될 수 있다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 목검을 버리고 손목을 붙잡아 파고든다 | `weapon_control`, `distance` | item 포기, 위험 감소 |
| 이구학지 | 소매를 잡히기 전에 팔꿈치로 끊고 빠진다 | `posture`, `distance` | sanity/health cost |
| office | 사원증 목걸이를 잡히기 전에 끊어낸다 | `weapon_control`, `nerve` | item/flag 변화 |
| office | 가방끈을 일부러 내줘 상대 손을 묶는다 | `weapon_control`, `environment` | item damage + advantage |

### 5.4 `trip_sweep` — 걸기 / 넘어뜨리기 / 미끄러뜨리기

정의: 상대의 하단과 지면 조건을 이용해 균형을 크게 바꾼다.

좋은 조건:
- 상대가 돌진한다.
- 바닥이 미끄럽다.
- 긴 무기/가방/카트/케이블이 발 밑에 있다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 구두를 벗어 던져 미끄러운 돌바닥을 피한다 | `posture`, `balance` | dignity/sanity cost, mobility flag |
| 이구학지 | 죽봉을 낮게 눕혀 돌진을 걸어 넘긴다 | `balance`, `pressure` | danger 감소 가능 |
| office | 케이블을 발목 높이로 당긴다 | `balance`, `environment` | 실패 시 본인도 health cost |
| office | 토너 가루가 깔린 쪽으로 물러나며 유도한다 | `balance`, `visibility` | clue/log 중심 |

### 5.5 `reposition` — 빠지기 / 파고들기 / 돌아서기

정의: 공격이 아니라 위치와 다음 선택지를 바꾸는 행동이다.

이 문법은 fallback choice로 특히 중요하다. 전투에서 항상 멋진 반격을 고르지 않아도 된다. 물러나거나 숨을 고르는 선택지가 있어야 플레이어가 “전투를 읽는다”고 느낀다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 큰길 쪽으로 비틀거리며 물러난다 | `distance`, `pressure` | health small cost, route clue |
| 이구학지 | 넥타이를 풀고 구두를 벗어 움직임을 회복한다 | `posture`, `breath` | sanity small cost, mobility flag |
| office | 선반 사이 거리를 벌려 숨을 고른다 | `distance`, `breath` | sanity 회복/전투 종료 |
| office | 자동문 센서 밖으로 반걸음 빠진다 | `distance`, `environment` | door clue, danger 변화 |

### 5.6 `guard_brace` — 막기 / 버티기 / 충격 흘리기

정의: 피해를 완전히 없애기보다 치명적 결과를 완화한다.

이구학지 기준에서 현대 회사원은 처음부터 멋진 무공 방어를 하지 않는다. 처음에는 가방, 팔, 벽, 숨 고르기, 몸을 낮추기 같은 현실적인 버티기로 시작한다. 나중에 청류안/청류심법을 얻어도 “막았다”보다 “흐름을 읽고 덜 맞았다”가 맞다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 출근 가방을 방패처럼 세워 첫 몽둥이를 흘린다 | `posture`, `weapon_control` | health cost 감소, bag damage |
| 이구학지 | 청류안으로 호흡이 끊기는 박자를 읽고 한 박자 늦게 물러난다 | `visibility`, `posture` | clue/flag, ability schema는 열지 않음 |
| office | 복합기 옆면을 등지고 충격을 분산한다 | `pressure`, `balance` | health cost 완화 |
| office | 회의실 의자를 사이에 끼워 버틴다 | `distance`, `posture` | 전투 지연/escape hook |

### 5.7 `disarm_control` — 무기 제어 / 떨어뜨리기 / 포기하기

정의: 무기는 공격력 수치가 아니라 선택지 언어를 바꾸는 장치다. 따라서 무기 제어는 핵심 액션이다.

형태:
- 상대 무기를 떨어뜨린다.
- 내 무기를 계속 잡는다.
- 내 무기를 일부러 버리고 붙잡는다.
- 긴 무기를 짧게 잡는다.
- 즉석 도구가 부서지기 전에 사용 목적을 바꾼다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 목검을 짧게 잡아 좁은 골목에서 밀어낸다 | `weapon_control`, `distance` | narrow-space 해결 |
| 이구학지 | 검을 버리고 손목으로 파고든다 | `weapon_control`, `distance` | item loss + survival |
| office | 소화기 핀을 뽑아 둔기 대신 시야 차단 도구로 쓴다 | `visibility`, `environment` | clue/flag |
| office | 스테이플러를 휘두르지 않고 상대 손에서 떼어낸다 | `weapon_control` | danger 감소 |

### 5.8 `environment_use` — 환경 활용

정의: 전투를 “둘의 HP 교환”이 아니라 장소 기반 상황극으로 만든다.

좋은 환경 선택지는 다음 중 하나를 해야 한다.

- 도주로를 만든다.
- 시야를 끊는다.
- 균형을 무너뜨린다.
- 상대의 큰 동작을 막는다.
- 동료/구조 대상에게 시간을 준다.
- 다음 story hook을 남긴다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 장터 좌판 천을 당겨 시야를 끊는다 | `visibility`, `pressure` | crowd/danger hook |
| 이구학지 | 약재 포대 뒤로 빠져 돌진 각도를 꺾는다 | `distance`, `balance` | location clue |
| office | 자동문 센서를 역이용해 문틈에 압박을 만든다 | `environment`, `pressure` | interface clue |
| office | 회의실 조명을 꺼서 잔상의 돌진을 늦춘다 | `visibility`, `nerve` | battery/sanity cost |

### 5.9 `read_flow` — 읽기 / 간파 / 단서 기반 개입

정의: 직접 몸을 쓰기보다 상황, 호흡, 규칙, 과거 단서를 읽어 전투 선택지를 바꾼다.

이구학지에서는 청류안/천기록/현대 회사원의 기록 습관이 이 역할을 한다. office에서는 CCTV, 사내망 로그, 회의록, 기계 소리, 사내 방송 패턴이 같은 역할을 한다.

주의: `read_flow`는 전투를 퀴즈로 바꾸면 안 된다. 읽은 뒤에도 결국 거리/균형/압박/무기 제어 중 하나를 바꿔야 한다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 흑사방 말단의 돌진이 왼발에서 늦는 것을 본다 | `visibility`, `balance` | clue + safer reposition |
| 이구학지 | 천기록의 실패 기록을 떠올려 같은 실수를 피한다 | `nerve`, `posture` | sanity cost + flag |
| office | CCTV 지연 화면으로 다음 돌진 방향을 읽는다 | `visibility`, `environment` | battery cost + advantage |
| office | 사내 방송 멘트가 잔상을 멈칫하게 하는 박자를 듣는다 | `nerve`, `visibility` | clue + pressure shift |

### 5.10 `ally_coordination` — 동료 지시 / 보호 / 합동 개입

정의: 동료는 플레이어의 선택을 대신하지 않는다. 동료는 전투 상황을 흔들거나, 플레이어가 고를 수 있는 선택지의 의미를 바꾼다.

이구학지 기준:
- 서하린은 초반에 플레이어를 대신해 무쌍하는 캐릭터가 아니다.
- 그녀는 구조자/감시자/멘토 hook이며, 플레이어가 버티고 해석할 시간을 만들어준다.
- 무명은 rival/future route hook으로, 초반 기본 전투 문법에서는 아직 full companion action으로 쓰지 않는다.

office 기준:
- 생존자/동료/방송 목소리는 전투를 대신 이기지 않는다.
- 경고, 시선 끌기, 문 잡아주기, 패닉 방지, 위기 개입 정도로 시작한다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 서하린이 만든 틈으로 장터 군중 밖으로 빠진다 | `distance`, `ally_stability` | destination/relationship clue |
| 이구학지 | 서하린이 막은 사이 수첩을 숨긴다 | `pressure`, `weapon_control` | suspicion/danger tradeoff |
| office | 방송 너머 목소리에게 자동문을 붙잡아 달라고 한다 | `environment`, `ally_stability` | battery/sanity cost |
| office | 패닉에 빠진 동료 이름을 불러 뒤로 물린다 | `ally_stability`, `nerve` | empathy clue/flag |

### 5.11 `desperate_action` — 무리수 / 강행 / 더러운 버티기

정의: 큰 비용을 내고 흐름을 바꾸는 선택지다.

이 선택지는 반드시 매력과 대가를 동시에 가져야 한다.

좋은 예:
- health를 크게 잃지만 danger를 줄이거나 route hook을 얻는다.
- sanity를 잃지만 중요한 clue를 본다.
- item을 잃지만 도주로를 만든다.
- 평판/의심을 악화시키지만 생존한다.

나쁜 예:
- 그냥 “강공격”이다.
- 비용만 있고 전투 흐름 변화가 없다.

예시:

| 표면 | 선택지 label | 바뀌는 축 | 결과 방향 |
|---|---|---|---|
| 이구학지 | 어깨로 들이받고 넘어지듯 버틴다 | `pressure`, `balance` | health 큰 비용, attention flag |
| 이구학지 | 천기록 문장을 억지로 떠올려 위험한 틈을 본다 | `visibility`, `nerve` | sanity/danger 비용, clue |
| office | 복합기와 함께 넘어지며 통로를 막는다 | `environment`, `pressure` | health 비용, escape route |
| office | 소화기를 터뜨려 모두의 시야를 끊는다 | `visibility`, `breath` | sanity/battery/flag tradeoff |

## 6. 무기/도구 문법

무기는 “강한 공격력”이 아니라 “가능한 개입 형태”를 바꾼다.

| 장비/도구 | 열리는 문법 | 약점/실수 | office 대응 |
|---|---|---|---|
| 맨손 | 붙잡기, 밀기, 태클, 손목 제어 | 리치 부족, 다수전 위험 | 맨손으로 잔상/직원을 밀어냄 |
| 출근 가방 | 막기, 완충, 밀치기, 물건 보호 | 끈을 잡힘, 내용물 파손 | 노트북 가방, 서류가방 |
| 목검/죽봉 | 거리 유지, 손등 치기, 하단 견제 | 좁은 곳에서 걸림, 뺏김 | 밀대, 우산, 청소 도구 |
| 단검/짧은 칼 | 근접 위협, 손목/팔 제어 | 붙잡히면 위험, 과격해 보임 | 커터칼은 공개-safe 주의 필요 |
| 장검/긴 무기 | 거리 압박, 위협 범위 | 클린치/좁은 공간에 약함 | 긴 막대, 표지판, 파이프 |
| 둔기 | 균형 붕괴, 압박 | 느림, 호흡 소모 | 소화기, 의자, 스테이플러 |
| 케이블/끈 | 걸기, 묶기, 지연 | 본인도 걸림 | 멀티탭, 사원증 줄 |
| 문/기계 | 압박, 차단, 시야/소리 변형 | interface/전원 조건 | 자동문, 복합기, 보안 게이트 |

이구학지의 무협 장비가 늘어나도 원칙은 같다. 초식명이나 무공명은 `choice label`의 flavor나 `presentation`의 stable term으로 먼저 쓰고, 별도 ability/reward schema는 별도 slice 전까지 열지 않는다.

## 7. 선택지 작성 규칙

### 7.1 좋은 전투 선택지 조건

전투 선택지는 다음을 만족해야 한다.

1. 상황 문장이 먼저 있어야 한다.
2. 선택지가 어떤 opening에 대한 대응인지 보여야 한다.
3. 선택 후 최소 하나의 flow axis가 바뀌어야 한다.
4. 비용이 있으면 이유가 장면 안에서 보이게 한다.
5. fallback choice가 있어야 한다.
6. 성공/실패보다 “어떤 방식으로 버텼는가”가 log에 남아야 한다.
7. Web/terminal 모두 같은 action id를 실행해야 한다.

### 7.2 나쁜 선택지 패턴

피해야 할 label:

```text
공격한다
방어한다
스킬을 쓴다
필살기를 쓴다
강공격
회피
아이템 사용
```

위 label은 너무 추상적이다. 아래처럼 바꾼다.

| 나쁜 label | 좋은 label |
|---|---|
| 공격한다 | 목검 든 손등을 쳐 무기를 떨어뜨린다 |
| 방어한다 | 출근 가방을 세워 첫 몽둥이를 흘린다 |
| 회피한다 | 담벼락 쪽으로 반걸음 물러나 돌진 각도를 꺾는다 |
| 스킬을 쓴다 | 청류안으로 호흡이 끊기는 박자를 읽는다 |
| 아이템 사용 | 소화기 핀을 뽑아 통로 시야를 끊는다 |

### 7.3 choice id naming

권장:

```text
<verb>_<object>_<purpose>
```

예시:

```text
loosen_tie_and_drop_shoes
swing_commute_bag
hook_cart_to_cabinet
pull_extinguisher_pin
read_left_foot_delay
brace_with_laptop_bag
cut_off_weapon_hand
```

피함:

```text
attack_1
defend
skill_qingliu_01
choice_good
win_fight
```

## 8. outcome hook 작성 기준

현재 schema-less 단계에서는 기존 outcome primitive만 쓴다.

| 의도 | 권장 hook |
|---|---|
| 부상/소모 | `resources.health`, `resources.sanity`, `resources.battery` |
| 위험 상승/완화 | `danger` |
| 전투 흐름 기억 | `add_flags` |
| 학습/단서/전투 감각 | `add_clues` |
| 도구 획득/파손/포기 | `add_items`, `remove_items`, item-related flag |
| 장면 이동 | `destination_id` |
| UI/연출 힌트 | `presentation.visual_id`, `presentation.layout`, `effect_cues` |
| 서사 기록 | `log` |

예시 shape:

```yaml
- id: loosen_tie_and_drop_shoes
  role: combat_reposition
  label: 넥타이를 풀고 구두를 벗어 움직임을 회복한다
  cost:
    sanity: -1
  outcome:
    add_flags:
      - mobility_recovered
    add_clues:
      - shoes_and_suit_are_liability
    log: 체면은 무너졌지만 발이 땅을 잡는다. 무공보다 먼저 몸을 살리는 준비가 필요했다.
```

주의: 위 예시는 설계 문법 설명용이다. 현재 문서 작성만으로 runtime YAML에 추가하지 않는다.

## 9. 기본 전투 템포

전투 하나의 권장 길이:

| 전투 등급 | 자동 exchange | 플레이어 개입 | 사용처 |
|---|---:|---:|---|
| 사소한 충돌 | 1 | 0 | 문장/log로 처리 |
| 평범한 위협 | 1~2 | 0~1 | 이동/탐색 중 짧은 난투 |
| 정예 위협 | 2~3 | 1~2 | 기억에 남는 route beat |
| 보스급 장면 | 3~4 | 최대 3 | 별도 설계 필요 |

이번 문서는 보스전 설계가 아니다. 보스급 장면도 이 action model을 쓸 수는 있지만, 페이즈/후일담/route condition은 별도 문서에서 다룬다.

## 10. 이구학지 기준 예시

### 10.1 흑사방 첫 난투

상황:

```text
흑사방 말단이 길을 막는다. 주인공은 현대 회사원의 몸, 출근복, 구두, 가방 그대로다. 무공은 없다. 맞으면 진짜 아프다.
```

개입 후보:

| choice id | label | taxonomy | 축 | 결과 방향 |
|---|---|---|---|---|
| `run_toward_open_street` | 큰길 쪽으로 비틀거리며 물러난다 | `reposition` | `distance`, `pressure` | health 작은 비용, 도주로 clue |
| `deescalate_with_words` | 말로 시간을 벌며 사원증을 감춘다 | `read_flow` | `nerve`, `pressure` | sanity 비용, 오해 clue, 서하린 hook |
| `swing_commute_bag` | 출근 가방을 방패처럼 휘두른다 | `guard_brace`, `strike` | `weapon_control`, `distance` | health 비용, bag damage flag |
| `loosen_tie_and_drop_shoes` | 넥타이를 풀고 구두를 벗어 움직임을 회복한다 | `reposition` | `posture`, `balance` | mobility flag, dignity/sanity cost |
| `crash_in_with_body` | 어깨로 들이받고 넘어지듯 버틴다 | `desperate_action`, `shove` | `pressure`, `balance` | 큰 health 비용, attention flag |

핵심: 이 전투는 “이긴다”가 아니라 “어떤 방식으로 살아남고 무엇을 배웠는가”를 남긴다.

### 10.2 청류문 수습생 sparring / 잡역 중 작은 충돌

상황:

```text
장작을 옮기던 중 외문 제자가 장난처럼 어깨를 밀어 온다. 무공 대련이 아니라 청류문 생활이 몸에 익는 첫 접촉이다.
```

개입 후보:

| choice id | label | taxonomy | 축 | 결과 방향 |
|---|---|---|---|---|
| `step_back_with_firewood` | 장작을 떨어뜨리지 않고 반걸음 물러난다 | `reposition`, `guard_brace` | `balance`, `posture` | chore/training clue |
| `let_shoulder_turn_with_push` | 밀리는 힘을 거스르지 않고 어깨를 돌린다 | `read_flow`, `guard_brace` | `balance`, `breath` | 청류심법 flavor clue |
| `plant_bare_foot_in_dust` | 흙먼지에 발을 박아 미끄러짐을 멈춘다 | `guard_brace` | `balance`, `posture` | mobility/training flag |
| `ask_harin_what_changed` | 방금 왜 덜 밀렸는지 서하린에게 묻는다 | `ally_coordination`, `read_flow` | `ally_stability`, `visibility` | mentor clue |

핵심: 무협다운 성장감은 “대미지 증가”보다 같은 몸싸움을 다르게 읽기 시작하는 데서 나온다.

## 11. office 기준 대응 예시

### 11.1 물품창고 자동 난투

현재 구현된 `supply_closet_auto_brawl`은 이 문법의 office prototype이다.

| existing choice id | label | taxonomy | 축 |
|---|---|---|---|
| `keep_distance_between_shelves` | 선반 사이 거리를 벌려 숨을 고른다 | `reposition` | `distance`, `breath` |
| `hook_cart_to_cabinet` | 캐비닛 손잡이에 카트를 걸어 거리를 만든다 | `environment_use`, `shove` | `distance`, `balance` |
| `pull_extinguisher_pin` | 소화기 핀을 뽑아 시야를 끊는다 | `environment_use`, `disarm_control` | `visibility`, `pressure` |

이 예시는 office와 이구학지가 같은 전투 문법을 공유할 수 있음을 보여준다. 표면은 다르지만 “거리/균형/시야/환경” 축은 같다.

### 11.2 회의실 잔상과의 충돌

후속 office 후보는 다음처럼 쓸 수 있다.

| choice id | label | taxonomy | 축 | 결과 방향 |
|---|---|---|---|---|
| `brace_with_meeting_chair` | 회의실 의자를 사이에 끼워 돌진을 늦춘다 | `guard_brace`, `environment_use` | `distance`, `pressure` | health 피해 완화 |
| `push_into_projector_glare` | 빔프로젝터 빛 쪽으로 밀어 시야를 흔든다 | `shove`, `environment_use` | `visibility`, `balance` | sanity/battery cost |
| `read_repeating_meeting_pattern` | 반복되는 회의 멘트 박자로 돌진 타이밍을 읽는다 | `read_flow` | `visibility`, `nerve` | clue 기반 유리함 |
| `crawl_under_table_line` | 회의 테이블 아래로 빠져 압박선을 끊는다 | `reposition` | `distance`, `pressure` | dignity/sanity cost |

## 12. 무공/초식 사용 기준

이구학지에서 무공명은 쓸 수 있다. 다만 처음부터 별도 스킬 시스템처럼 쓰면 안 된다.

허용:

```text
[청류안] 상대의 호흡이 끊기는 박자를 읽는다
[수류] 밀려오는 힘을 정면으로 막지 않고 어깨로 흘린다
[관류] 군중 사이 흐름이 비는 방향을 본다
```

금지:

```text
청류검법 1식 사용
내공 10 소모
쿨타임 3턴
공격력 150% 피해
```

원칙:

1. 무공명은 행동을 설명하는 flavor/semantic tag다.
2. 실제 결과는 여전히 기존 outcome hook으로 표현한다.
3. 무공은 HP damage multiplier가 아니라 읽기, 균형, 호흡, 거리, 압박을 바꾸는 방식이다.
4. 청류안/청류심법/천기록은 별도 ability/reward schema가 열리기 전까지 clue/flag/log/presentation으로만 남긴다.

## 13. schema 단계

### Phase 0 — 현재 권장: schema-less action model

사용:
- 기존 encounter/choice/outcome schema
- choice label과 log에 action taxonomy 반영
- optional `presentation.layout` / `visual_id` / `effect_cues`
- flags/clues로 전투 흐름 기억

금지:
- `CombatState`
- 별도 `combat_hp_track`
- skill cooldown
- weapon DPS table
- renderer-side combat calculation

### Phase 1 — 반복 확인 후: semantic presentation metadata

반복 전투가 늘어나면 다음 정도를 검토할 수 있다.

```yaml
presentation:
  layout: combat_intervention
  stable_terms:
    - 거리
    - 균형
    - 목검
    - 시야
  effect_cues:
    - kind: pressure_pulse
      intensity: medium
```

이 단계도 결과 판정은 core/outcome이 소유한다.

### Phase 2 — 충분한 반복 후: resolver 후보

다음 조건이 모두 만족될 때만 검토한다.

- 같은 taxonomy가 여러 storypack에서 반복된다.
- choice authoring만으로 중복이 너무 많다.
- 전투 전 비교, 개입 예산, 상황 태그가 실제로 재사용된다.
- Web/terminal 모두 같은 ScenePage/ActionResult truth를 유지할 수 있다.

그 전까지는 “전투 시스템”이라는 이름으로 큰 engine을 열지 않는다.

## 14. 구현 전 체크리스트

전투 encounter를 runtime 후보로 승격하기 전에 확인한다.

- [ ] fallback choice가 있다.
- [ ] 각 choice가 taxonomy 중 하나 이상에 속한다.
- [ ] 각 choice가 flow axis 하나 이상을 바꾼다.
- [ ] HP 숫자전/스킬 쿨타임/전투 보상창 문법이 없다.
- [ ] Web/terminal renderer가 결과를 재계산하지 않는다.
- [ ] office surface로 바꿔도 같은 축이 성립한다.
- [ ] 이구학지 surface로 바꿔도 office와 너무 동떨어진 전용 시스템이 되지 않는다.
- [ ] 선택지는 `choice:*` action id로 core에 전달 가능하다.
- [ ] 결과는 기존 resources/danger/flags/clues/items/destination/log로 표현 가능하다.
- [ ] 전투당 플레이어 개입은 0~3회 범위 안에 있다.

## 15. 다음 설계/구현 후보

첫 이구학지 비보스 소형 전투 runtime sample은 `wuxia_cheongryu_chore_sparring`로 구현했다. 이 sample은 청류문 장작 마당의 작은 몸싸움을 `presentation.layout: combat_intervention`, 기존 resources/danger/flags/clues/log, stable choice id 4개로 표현하며 새 `CombatState`나 reward/ability schema를 열지 않는다.

다음 후보는 별도 slice로 고른다.

1. office 기본팩에서 회의실/복합기/보안문 기반 combat_intervention encounter를 하나 더 추가한다.
2. `wuxia_heuksa_bang_first_fight`와 `wuxia_cheongryu_chore_sparring`의 choice taxonomy를 docs/runtime source에 더 명시적으로 주석화한다.
3. 반복 샘플이 2~3개가 되면 `presentation.layout: combat_intervention`의 stable term 규칙을 정리한다.
4. 그래도 중복이 많을 때만 semantic presentation metadata 또는 Rust combat resolver를 검토한다.

현재 권장 순서는 다음이다.

```text
Basic_Combat_Action_Model 문서화 완료
→ 이구학지 비보스 소형 전투 `wuxia_cheongryu_chore_sparring` 구현 완료
→ office 대응 전투 후보 1개 설계/구현
→ 두 후보가 같은 taxonomy로 표현되는지 검증
→ 그때도 중복이 많으면 presentation metadata 또는 resolver 검토
```

## 16. 비목표

이번 문서는 다음을 하지 않는다.

- 사도 최종전/보스전 설계.
- full faction route graph.
- relation/debt/faction/companion schema 추가.
- 천외편린 3택 reward/ability schema 추가.
- 기본 office runtime YAML/Rust/Web default generated bundle 변경.
- Web-only combat UI 또는 terminal-only combat UI 설계.
- HP bar, DPS table, 스킬 쿨타임, 경험치 보상창 도입.

## 17. 최종 원칙

기본 전투의 맛은 다음 문장으로 고정한다.

> 이구학지의 무협도, office의 괴담도, 전투의 기본은 같은 몸싸움이다. 표면은 목검과 구두, 소화기와 죽봉으로 바뀌지만, 플레이어가 읽고 바꾸는 것은 거리, 균형, 압박, 호흡, 시야, 무기 제어, 동료의 안정이다.

따라서 새로운 전투 콘텐츠를 만들 때 첫 질문은 “얼마나 피해를 주는가?”가 아니라 다음이어야 한다.

```text
이 선택은 어떤 흐름을 바꾸는가?
그 흐름은 이구학지와 office 양쪽에서 같은 원리로 설명되는가?
```
