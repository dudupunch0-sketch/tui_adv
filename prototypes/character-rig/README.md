# 전투 캐릭터 리그 — 스타일 프로토타입

전투 관전 표면에 세울 **캐릭터를 에셋 없이 코드로 만드는** 두 가지 방식을,
같은 실제 전투 데이터로 렌더해 비교하기 위한 프로토타입이다. 결정용이며 배포되지 않는다.

배경: [Vyom-26/Wave-Racer](https://github.com/Vyom-26/Wave-Racer) (INK TIDE)가 메시·텍스처·사운드를
전부 런타임 생성으로 해결했고, 그 중 캐릭터에 해당하는 부분이 `rider/riderRig.ts`(형태)와
`rider/riderAnim.ts`(움직임)다. 여기서 차용한 것은 3D가 아니라 **애니메이션 구조**다.

## 핵심 주장

`rig.js`는 렌더러를 모른다. INK TIDE `riderAnim.ts`의 4층 구조 —

```
SIGNALS → STATES → MODIFIERS → IK
```

— 는 차원과 무관하며, 두 렌더러가 **같은 관절 각도**를 소비한다. `rig2d.js`는 캔버스에
실루엣으로 그리고, `rig3d.js`는 같은 숫자를 Three.js 계층의 `rotation.z`에 넣는다.
어느 렌더러도 포즈를 계산하지 않는다.

큐 5종이 포즈 레이어에 그대로 대응한다:

| cue | 레이어 |
|---|---|
| `attack` | STATE (발도·베기, phase로 매개변수화) |
| `evade` | STATE (뒷발 중심 회피) |
| `incapacitated` | STATE (붕괴, 다른 상태를 덮어씀) |
| `hit` | **MODIFIER** — 가산. 공격 중에도 피격될 수 있어야 한다 |
| `balance_broken` | MODIFIER + STATE 혼합 |

`hit`이 상태가 아니라 수정자인 이유: 실제 픽스처에서 두 전투원이 **같은 tick에**
서로 때린다. 상태로 만들면 맞는 순간 공격이 사라진다.

## 파일

| 파일 | 역할 |
|---|---|
| `rig.js` | 스켈레톤(17본), 포즈 상태, 수정자, 2본 IK, 도포 verlet 체인 |
| `rig2d.js` | 스타일 1 — 캔버스 2D 잉크 실루엣 |
| `rig3d.js` | 스타일 2 — Three.js 로우폴리 + 셀 셰이딩 + 인버티드 헐 아웃라인 |
| `index.html` | 비교 시트 |
| `capture.mjs` | Playwright 결정론적 캡처 |
| `combat-frames.json` | **실제** `ScenePage.combat` 출력 |

## 재현

데이터는 지어낸 좌표가 아니라 코어가 실제로 만든 프레임이다.

```bash
cargo run -p escape-core --example dump_combat_spectator -- 2 \
  > prototypes/character-rig/combat-frames.json

cd prototypes/character-rig && npm install
PLAYWRIGHT_CHROMIUM_PATH=/path/to/chrome node capture.mjs
```

시드 고정 + 애니메이션 클럭 없음이므로 같은 명령은 항상 같은 바이트를 낸다.
`PLAYWRIGHT_CHROMIUM_PATH`는 브라우저가 이미 설치된 머신에서 버전별 다운로드를 건너뛰기 위한 것이며,
생략하면 Playwright 기본 해석을 따른다.

## 캡처가 잡아낸 것 (반영 완료)

추론이 아니라 실제 프레임이 잡아낸 결함들이다.

- `taperedPath`가 `ctx.arc()`로 끝을 둥글리려다 **관절마다 완전한 원**을 남겼다 (arc는 캡이 아니라 서브패스를 추가한다)
- 검로 호가 어깨+칼날 반지름으로 2.1rad을 쓸어 프레임을 가로질렀다 — 그 캐릭터의 베기가 아니라 배경 장식으로 읽혔다
- 피격 스파크 9개가 몸통을 덮어 캐릭터에 붙인 파편처럼 보였다
- 3D 도포 원뿔(46 단위)이 다리를 전부 삼켜 **종(鐘) 모양**이 됐다
- 3D 전완이 피부색이라 소매가 맨팔로 읽혔다
- 셀 밴드 사다리가 어두운 로브를 전부 그림자 대역에 넣었다

## 남은 격차 (정직하게)

- **2D 검로가 여전히 크다.** 어깨 중심 호라 인물 높이의 1/3을 차지한다. 손 중심 짧은 호가 맞다.
- **3D는 병합/스키닝이 없다.** 파츠별 메시 계층이라 드로우콜이 INK TIDE의 라이더당 4개 예산을 훨씬 넘는다.
  실제 채용 시 병합 + 정점 스키닝이 필요하고, 이 프로토타입은 그 비용을 계상하지 않았다.
- **3D 도포는 천이 아니다.** 원뿔이 엉덩이 움직임에 기울 뿐, 2D의 verlet 헴 같은 관성이 없다.
- `evade` / `balance_broken` 포즈는 **작성됐지만 캡처되지 않았다.** 이 픽스처는 두 큐를 만들지 않는다.
- 두 스타일 모두 헥스 보드와 합성되지 않았다. 인물만 본 비교다.
- 얼굴 없음은 정책 준수이자 의도된 선택이며, 원거리 표정 연출은 두 스타일 모두 아직 답이 없다.
