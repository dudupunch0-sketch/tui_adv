# 전투 시스템: 자동 난투 + 상황 개입 설계

## 1. 문서 목적

이 문서는 `idea_box/combat_system.md`의 원본 아이디어를 `escape from the office`의 현재 구조에 맞게 승격한 설계 문서다.

기본 공격 형식, 싸우는 형태, 선택지 taxonomy는 `docs/design/Basic_Combat_Action_Model.md`가 담당한다. 이 문서는 상위 철학과 구현 단계, `Basic_Combat_Action_Model.md`는 이구학지와 office가 공유할 수 있는 action authoring 문법이다.

원본 아이디어의 핵심은 다음 한 줄이다.

> 실시간으로 자동 진행되는 MMA식 난투 속에서, 플레이어가 결정적인 상황에만 개입해 전투의 흐름을 바꾸는 시스템.

이 문서는 그 아이디어를 그대로 “실시간 액션 게임”으로 구현하자는 뜻이 아니다. 현재 프로젝트의 기준은 여전히 다음과 같다.

```text
Rust GameCore
  -> renderer-neutral ScenePage / ActionResult
  -> Web Storybook + GlyphFX primary UX
  -> SuperLightTUI terminal renderer / horror edition
```

따라서 전투 시스템도 renderer가 직접 시뮬레이션하거나, Web/terminal이 서로 다른 전투 규칙을 갖는 방식으로 만들지 않는다. 전투의 truth는 Rust GameCore가 소유하고, renderer는 core가 제공한 장면, 상태, 선택지, 효과 cue를 표시한다.

## 2. 프로젝트 적용 요약

`escape from the office`의 전투는 일반 RPG 전투가 아니라 “회사 괴담 속 몸싸움 상황”이어야 한다.

핵심 적용 방향:

1. 전투는 메인 루프 전체를 대체하지 않는다. 이동, 탐색, 사내망, 단서, 엔딩 루프 안에서 드물게 발생하는 인카운터 계열로 둔다.
2. 전투는 긴 조작전이 아니라 짧은 난투극이다.
3. 약한 상대나 사소한 충돌은 자동으로 처리한다.
4. 정예/보스급 장면에서만 플레이어에게 1~3회의 의미 있는 상황 개입을 준다.
5. HP 숫자 교환보다 거리, 균형, 압박, 호흡, 자세, 시야, 공포, 분노, 무기 제어, 환경 활용을 중심으로 한다.
6. 회사 배경에서는 스테이플러, 복합기, 회의실 의자, 사원증 목걸이, 노트북 가방, 자동문, 케이블, 칸막이, 보안 게이트가 전투 문법의 재료가 된다.
7. 전투 UI는 `몬스터 HP`, `스킬 쿨타임`, `전투 보상창`처럼 보이면 안 된다. `[LOCAL STATUS]`, 상황 문장, 선택지 row, 로그/진단 문법으로 표현한다.

## 3. 한 줄 정의

> 자동으로 진행되는 회사 괴담식 난투 장면에서, 플레이어가 제한된 횟수의 상황 선택으로 흐름을 꺾는 짧은 전투 드라마 시스템.

## 4. 왜 이 프로젝트에 맞는가

현재 게임의 재미는 “읽고 판단하고 감당하는 선택”에 있다. 전투도 피지컬 조작으로 바뀌면 프로젝트의 중심에서 벗어난다.

이 설계가 맞는 이유:

- Web Storybook은 읽기 중심 선택지와 짧은 장면 전환에 강하다.
- SuperLightTUI는 실시간 조작보다 로그, 상태 문장, 선택지 입력에 강하다.
- Rust GameCore는 deterministic action/result를 제공해야 하므로, renderer-local physics나 타이머 기반 난투보다 “core가 해결한 자동 난투 결과 + 선택지”가 안전하다.
- 회사 괴담 톤에서는 멋진 콤보보다 복합기에 밀려 넘어지고, 케이블에 발이 걸리고, 사내 방송 때문에 모두가 멈칫하는 장면이 더 고유하다.

## 5. 비목표

이번 설계는 다음을 열지 않는다.

- Web renderer에서 별도 실시간 전투 시뮬레이션 구현.
- SuperLightTUI와 Web이 서로 다른 전투 판정을 갖는 구조.
- 마우스/반응속도/연타/회피 타이밍 중심 액션 게임.
- 복잡한 스킬 쿨타임, 레벨, 경험치, 장비 DPS 중심 RPG 전투.
- `ScenePage`에 CSS class, pixel coordinate, terminal color object 같은 renderer-specific 정보를 넣는 것.
- 현재 PR에서 런타임 YAML, Rust core, Web UI 코드를 바로 변경하는 것.

이 문서의 범위는 아이디어 승격과 설계 정리다. 실제 구현은 별도 slice에서 진행한다.

## 6. 현재 턴 루프와의 결합 방식

전투는 기존 “한 턴 = 하나의 의미 있는 행동” 루프를 깨지 않는다.

권장 흐름:

```text
일반 위치/인카운터 ScenePage
→ 전투 트리거 발생
→ Rust GameCore가 전투 전 비교와 전투 등급 산정
→ 자동 난투 exchange를 해결하고 장면 문장 생성
→ 개입 예산이 남고 결정적 상황이면 intervention ScenePage 생성
→ 플레이어가 choice:* action id 선택
→ ActionResult로 상태/위치/플래그/로그 갱신
→ 자동 난투 지속 또는 결착
→ 일반 루프로 복귀하거나 엔딩/도주/부상 결과 처리
```

중요한 점:

- renderer는 “지금 어떤 선택지가 떠야 하는지”를 계산하지 않는다.
- renderer는 core가 제공한 `SceneAction.id`를 표시하고 다시 core에 전달한다.
- 표시 번호, 버튼, 숫자키, terminal 입력은 renderer-local이다.
- 자동 난투는 실제 wall-clock timer가 아니라 core가 deterministic하게 해결한 장면 단계로 표현한다.

## 7. 전투 등급과 개입 예산

원본 아이디어의 핵심 규칙인 “전투당 개입 요구는 0~3회”를 유지한다.

| 전투 등급 | 프로젝트식 예 | 개입 요구 | 처리 방향 |
|---|---|---:|---|
| 사소한 충돌 | 넘어지는 의자, 약한 잔상, 방해물 | 0회 | 자동 로그/짧은 body block으로 처리 |
| 평범한 위협 | 격리된 직원 잔상, 보안요원 같은 환영 | 0~1회 | 한 번의 환경/도주/제압 선택지만 제공 |
| 정예 위협 | 일그러진 부장, 보안실 권한 잔상, 서버실 감시체 | 1~2회 | 전세를 바꾸는 개입 장면 중심 |
| 보스급 장면 | 대표실 결재 콘솔의 육체화, 회사 자체의 대리자 | 최대 3회 | 짧은 2~3막 전투 드라마 |

개입 예산은 “선택지가 등장할 수 있는 최대 횟수”다. 예산이 2라고 해서 반드시 2번 개입해야 하는 것은 아니다. 자동 난투가 유리하게 흘러가면 1회 개입 후 결착될 수 있다.

## 8. 전투 전 비교 기준

전투 등급은 단순 레벨이나 공격력으로 정하지 않는다.

플레이어 쪽 입력:

- 현재 `health`, `sanity`, `hunger`, `thirst` 상태.
- `physical`, `composure`, `volition`, `logic`, `empathy`, `interface` 능력치.
- 보유 아이템: 사원증, 손전등, 보조배터리, 노트북 가방, 즉석 무기 후보.
- 보유 단서: 상대의 약점, 공간 규칙, 보안 로그, 회의 패턴.
- 현재 위치: 회의실, 복합기 구역, 비상계단, 보안실, 서버실 앞 등.
- 동료/생존자 hook: 목소리, 방송, 지연 메시지, 구조 대상.
- 위험도와 현재 disaster type.

상대/상황 쪽 입력:

- 위협 유형: 사람, 잔상, 시스템 대리자, 괴현상, 보안 장치.
- 숫자 우위 또는 포위 여부.
- 무기/도구 보유 여부.
- 공간 적응도: 좁은 복도, 회의실, 자동문, 서버실, 비상계단.
- 공포/분노/오작동/프로토콜 집착 같은 행동 태그.
- 보스 패턴 또는 route-critical encounter 여부.

## 9. 핵심 전투 상태 축

HP는 존재할 수 있지만 중심이 아니다. 실제 플레이어-facing 전투 감각은 아래 상태 축으로 만든다.

| 축 | 의미 | 현재 프로젝트와의 연결 |
|---|---|---|
| 거리 | 붙었는지, 떨어졌는지, 도망칠 수 있는지 | 위치/도주 선택지, `physical`, `composure` |
| 균형 | 넘어질 위험, 자세 붕괴 | 부상/체력 피해, 상태 로그 |
| 압박 | 벽, 구석, 다수, 보안 게이트에 몰린 정도 | danger 증가, route 차단 |
| 호흡 | 지침, 반응 저하 | `hunger`, `thirst`, `health` 임계치 |
| 자세 | 공격/방어/회피 가능 상태 | 선택지 eligibility, 실패 피해 |
| 시야 | 상대와 환경을 읽는 정도 | 배터리, 손전등, `logic`, `interface` |
| 공포 | 위축, 도주, 패닉 | `sanity`, `volition`, 글리치/왜곡 |
| 분노 | 무리한 돌진, 판단 저하 | 상대 trigger, 반격/유인 선택지 |
| 무기 제어 | 도구를 제대로 들고 있는지 | 아이템/즉석 무기, 손목 제압 선택지 |
| 동료 안정 | 동료/목소리/생존자 hook이 버티는지 | `empathy`, 생존자 설득 루트 |

첫 구현에서 이 축들을 모두 영구 상태로 추가할 필요는 없다. 처음에는 전투 인카운터 내부의 문장, 태그, 선택지 조건으로만 표현하고, 반복 구현 가치가 생기면 `CombatState` 또는 optional combat metadata를 검토한다.

## 10. 상황 트리거 태그

개입 선택지는 랜덤 팝업이 아니라 상황 태그에서 나온다.

| 태그 | 설명 | 회사 배경 예 |
|---|---|---|
| `balance_break` | 균형이 무너짐 | 회의실 의자에 걸려 휘청임 |
| `wall_near` | 벽/칸막이/문 근처 | 회의실 유리벽, 칸막이, 자동문 |
| `clinch` | 서로 붙잡음 | 사원증 목걸이, 재킷, 가방끈을 붙잡힘 |
| `down` | 누군가 넘어짐 | 복합기 앞 토너 가루에 미끄러짐 |
| `ally_at_risk` | 동료/목소리/생존자 hook 위험 | 방송 채널이 끊기려 함 |
| `weapon_lost` | 도구를 놓침 | 손전등, 스테이플러, 소화기를 떨어뜨림 |
| `environment_access` | 주변 사물을 활용 가능 | 복합기, 의자, 케이블, 자동문 |
| `rage_state` | 상대가 무리하게 돌진 | 부장 잔상이 결재판을 들고 달려듦 |
| `fear_state` | 상대가 위축 | 보안요원 환영이 사내 방송에 멈칫함 |
| `sight_blocked` | 시야가 나쁨 | 정전, 연기, GlyphFX 오염, 비상등 |
| `narrow_space` | 좁은 공간 제약 | 비상계단, 서버실 앞, 복도 |
| `height_edge` | 계단/난간/단차 | 비상계단, 옥상 문턱 |

## 11. 선택지 생성 재료

좋은 전투 선택지는 단순 공격 종류가 아니라 흐름 변화를 만든다.

선택지 재료:

1. 현재 상황 태그.
2. 현재 위치와 환경 사물.
3. 플레이어 능력치.
4. 현재 자원 임계치.
5. 보유 아이템/단서.
6. 동료/생존자 hook.
7. 상대의 감정/오작동 태그.

예:

```text
일그러진 부장이 회의실 벽에 등을 부딪히며 휘청인다.

[1] 어깨로 밀어 회의실 유리에 박는다
    결과: 압박 증가, 균형 추가 감소, 위험도 +1

[2] 손목을 잡아 스테이플러를 떨어뜨린다
    결과: 무기 제어 감소, 체력 피해 감소

[3] 뒤로 빠져 숨을 고른다
    결과: 거리 증가, 다음 실패 피해 감소, 결착 지연
```

이 예시는 Web에서는 문장형 choice row로, terminal에서는 번호 선택지와 상태 로그로 표시할 수 있다. 두 renderer 모두 실행하는 action id는 core가 제공한 `choice:*`다.

## 12. 능력치와 전투 선택지

기존 6스탯을 그대로 사용한다. 전투 때문에 새 능력치 체계를 먼저 만들지 않는다.

| 능력치 | 전투에서의 의미 | 선택지 예 |
|---|---|---|
| `physical` | 밀기, 버티기, 도주, 넘어뜨리기 | `[신체] 자동문 쪽으로 밀어붙인다` |
| `composure` | 침착한 거리 조절, 넘어져도 회복 | `[침착] 손전등을 놓치지 않고 빠진다` |
| `volition` | 공포/압박을 버팀 | `[의지] 사내 방송이 이름을 불러도 멈추지 않는다` |
| `logic` | 공간/패턴/상대 빈틈 파악 | `[논리] 반복되는 돌진 타이밍을 읽는다` |
| `empathy` | 동료/상대의 흔들림을 읽음 | `[공감] 떨리는 목소리를 붙잡아 정신 차리게 한다` |
| `interface` | 보안문/자동문/복합기/사내망을 이용 | `[인터페이스] 자동문 센서를 역이용한다` |

## 13. 무기와 즉석 도구

무기는 DPS 장비가 아니라 상황과 선택지를 바꾸는 재료다.

| 도구 | 강점 | 약점/실수 |
|---|---|---|
| 스테이플러 | 손목/손가락 위협, 짧은 거리 | 놓치기 쉽고 과장되면 우스워짐 |
| 회의실 의자 | 밀기, 거리 만들기 | 좁은 공간에서 걸림 |
| 소화기 | 압박, 시야 차단, 둔기 | 무겁고 호흡 소모 큼 |
| 노트북 가방 | 방어, 밀치기, 중요한 물건 보호 | 끈을 붙잡힐 수 있음 |
| 손전등 | 시야 확보, 눈부심 | 배터리 소모, 떨어뜨리면 불리 |
| 사원증 목걸이 | 문/인증/목줄 코미디 | 붙잡히면 위험 |
| 케이블/멀티탭 | 넘어뜨리기, 묶기 | 본인도 걸릴 수 있음 |

핵심은 “무기를 들면 강해진다”가 아니라 “무기를 들면 전투 언어가 바뀐다”다.

## 14. 동료와 생존자 hook

이 시스템은 `docs/content/Survivor_System_Routes.md`의 생존자 설득/시스템 제압 루트와 연결될 수 있다.

전투에서 동료/생존자 hook이 할 수 있는 역할:

- 방송 채널 너머의 목소리가 위협의 주의를 끈다.
- 동료가 보안문을 붙잡고 버틴다.
- 플레이어가 동료를 끌어내는 선택지를 고른다.
- 설득에 실패하면 목소리가 사내 안내 멘트로 흡수되어 전투 압박이 커진다.
- 유대/공감 기반 선택지가 전투 결과를 완화한다.

주의:

- 공개 문서에는 실제 직원, 실제 부서, 실제 보안 절차를 넣지 않는다.
- 동료는 실제 인물 재현보다 public-safe fiction/NPC/hook으로 다룬다.

## 15. UI/renderer 표현 규칙

전투 UI도 기존 UI 규칙을 따른다.

피해야 할 표현:

- 몬스터 HP bar.
- 스킬 버튼/쿨타임 회전.
- 전투 보상창.
- “공격력 +12” 같은 일반 RPG 장비 표현.
- renderer가 전투 결과를 재계산하는 표시.

권장 표현:

```text
[LOCAL STATUS]
신체 반응: 호흡 거칠음
자세: 회의실 벽 근처에서 흔들림
시야: 비상등 때문에 불안정
상대: 결재판을 든 잔상이 돌진 준비
환경: 의자 2개, 유리벽, 자동문 센서
```

Web Storybook:

- 전투 장면도 portrait board 안의 story flow로 표시한다.
- `data-region="visual"`에는 사무실 난투 visual card/GlyphFX를 표시할 수 있다.
- 선택지는 기존 `choice-row` 문법을 유지한다.
- 움직임/전환은 renderer-local transition/audio readiness 구조 위에 얹을 수 있지만, 판정은 core에서 온다.

SuperLightTUI:

- ASCII/Unicode card와 상태 로그로 전투 상황을 보여준다.
- 숫자 선택, 저장, 종료, 도움말 입력 안정성을 전투에서도 유지한다.
- animation이 없어도 `stable_terms`와 핵심 단서가 읽혀야 한다.

## 16. 데이터와 schema 전략

첫 구현은 schema를 과하게 열지 않는다.

권장 단계:

1. 문서화 단계: 이 문서로 방향과 금지선을 고정한다.
2. schema-less content slice: 기존 `encounters.yaml`의 일반 encounter/choice/result만으로 전투 느낌의 샘플 1개를 만든다.
3. 반복 필요가 확인되면 optional presentation metadata를 추가한다.
   - 예: `presentation.layout: combat_exchange`
   - 예: `presentation.visual_id: office_brawl_meeting_room`
   - 예: `effect_cues`로 GlyphFX/위험 신호 제공
4. 그래도 부족하면 Rust core에 `CombatProfile`, `CombatExchange`, `intervention_budget` 같은 전용 구조를 별도 PR로 검토한다.

첫 slice에서는 `ScenePage` schema를 바꾸지 않는 편이 안전하다. 전투는 먼저 “특정 인카운터 문법”으로 증명하고, 충분히 반복될 때만 공통 시스템으로 승격한다.

## 17. 구현 slice 후보

### PR 1 — schema-less combat encounter prototype

목표:

- 기존 YAML encounter/choice/action/result만 사용해 “자동 난투 + 1회 개입” 장면을 1개 만든다.
- 예: 회의실 또는 복합기 구역에서 `distorted_manager_brawl` encounter.
- Rust core, Web Storybook, SuperLightTUI 모두 기존 action id/display 경로로 표시되는지 검증한다.

구현 기록:

- 2026-05-29 첫 runtime slice로 `supply_closet_auto_brawl`을 추가했다.
- `supply_closet_cache`의 `brace_for_supply_scuffle` 선택이 기존 flag outcome만으로 전투형 인카운터를 연다.
- 전투 해결도 기존 choice/outcome의 resource, flag, clue, log만 사용한다.
- Web Storybook과 SuperLightTUI는 Rust GameCore가 제공한 `ScenePage`/action id를 표시하며 renderer 쪽 gameplay 판정은 추가하지 않았다.

비목표:

- 새 전투 schema.
- 실시간 timer.
- 새 renderer-specific 전투 UI.

### PR 2 — combat presentation metadata

목표:

- 반복되는 전투 장면을 위해 optional semantic presentation hint를 추가한다.
- Web/terminal 모두 fallback 가능한 `visual_id`, `layout`, `effect_cues`만 사용한다.

비목표:

- renderer-specific coordinates/class.
- 전투 결과를 renderer에서 계산.

### PR 3 — Rust combat resolver

목표:

- 여러 전투가 쌓인 뒤에만 core 내부 전용 resolver를 추가한다.
- 전투 전 비교, 등급, 개입 예산, 상황 태그, 자동 exchange를 deterministic하게 계산한다.

비목표:

- physics simulation.
- frame-based action game.

### PR 4 — balancing and content expansion

목표:

- 약한 전투는 자동 처리되고, 정예/보스만 기억에 남는 개입을 주는지 QA한다.
- 전투가 자원 루프, 단서 루트, 생존자/시스템 루트와 충돌하지 않는지 확인한다.

## 18. 완료 기준

전투 시스템을 실제 구현으로 승격할 때의 완료 기준:

- 전투당 개입 요구는 0~3회 범위를 넘지 않는다.
- 약한 전투는 선택지 없이 빠르게 처리된다.
- 모든 전투 action은 core-provided `SceneAction.id`로 실행된다.
- Web과 terminal은 같은 `ScenePage`/`ActionResult` truth를 표시한다.
- 전투 선택지는 공격 버튼이 아니라 상황 판단으로 느껴진다.
- HP 숫자전보다 거리, 균형, 압박, 호흡, 시야, 환경 활용이 드러난다.
- 회사 괴담/블랙코미디 톤을 유지한다.
- 공개 문서와 공개 YAML에 실제 회사/사람/보안 세부정보가 들어가지 않는다.

## 19. 주요 리스크와 대응

### 리스크 1. 플레이어가 구경만 하는 느낌

대응:

- 자동 전투는 짧게 유지한다.
- 개입 선택은 적지만 결과가 즉시 체감되게 한다.
- 정예/보스 장면은 1~3개의 강한 장면으로 구성한다.

### 리스크 2. 선택지가 너무 자주 떠서 피곤함

대응:

- 개입 예산을 전투 전 산정한다.
- 평범한 상대는 0~1회, 보스도 최대 3회를 넘지 않는다.

### 리스크 3. 선택지가 랜덤처럼 느껴짐

대응:

- 선택지 앞에 왜 그 선택지가 가능한지 상황 문장을 둔다.
- 태그, 위치, 아이템, 단서가 선택지 문장에 드러나게 한다.

### 리스크 4. 일반 RPG 전투로 변질됨

대응:

- 무기/도구는 공격력보다 거리, 균형, 압박, 실수 가능성을 바꾼다.
- 전투 보상/레벨업/스킬 쿨타임 문법을 피한다.

### 리스크 5. schema를 너무 빨리 열어 복잡해짐

대응:

- 첫 slice는 기존 encounter schema로 증명한다.
- 반복 패턴이 충분히 확인되기 전까지 `CombatState` 같은 전용 구조를 만들지 않는다.

## 20. 원본 아이디어와의 대응

| 원본 아이디어 | 프로젝트 적용 |
|---|---|
| 실시간 자동 난투 | Rust core가 해결한 자동 exchange 장면 |
| 결정적 상황에만 개입 | `intervention_budget` 0~3회 원칙 |
| MMA식 흐름 싸움 | 거리/균형/압박/호흡/자세 중심 선택지 |
| 무기는 MMA 위에 얹힘 | 회사 도구/즉석 무기는 선택지 언어를 바꿈 |
| 동료 시스템 연계 | 생존자 설득/방송/동료 hook과 연결 |
| 숫자 최소화 UI | `[LOCAL STATUS]`, 문장형 상태, choice row |
| 세계관 확장성 | 회사 아포칼립스, 괴담, 시스템 제압 루트에 맞춰 축소 적용 |

## 21. 현재 상태

- 원본 source: `idea_box/combat_system.md`
- 승격 문서: `docs/design/Combat_System_Auto_Brawl.md`
- 기본 액션 모델: `docs/design/Basic_Combat_Action_Model.md`
- 상태: 설계 문서화 완료, PR 1 schema-less runtime prototype 구현 완료
- 첫 runtime slice: `supply_closet_auto_brawl`
- 유지한 금지선: 새 `CombatState`, 새 combat schema, HP 숫자전, 스킬/쿨타임, renderer gameplay 판정 없음
- 다음 후보: 반복 가치가 확인되면 `presentation` metadata 정리 또는 `isolation_pack` runtime encounter 승격을 별도 PR로 검토
