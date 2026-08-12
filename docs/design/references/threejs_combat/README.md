# Three.js 전투 구현 레퍼런스 팩

Status: private-study implementation reference, 2026-08-12
Canonical decisions: [Three.js 전투 비주얼 아키텍처](../../ThreeJS_Combat_Visual_Architecture.md)

이 폴더는 정본을 대신하지 않는다. 정본이 무엇을 만들지 결정하고, 이 팩은 구현자가
어떤 화면과 기법을 참고할지 고정한다. 둘이 충돌하면 정본이 우선한다.

이 GitHub 저장소는 public이므로 사용자 제공 이미지 원본은 commit하지 않는다. 이 머신의
영구 로컬 원본은 /home/dudu/work/references/tui-adv-threejs-combat/images/에 보존한다.
아래 해시와 파일명이 일치하는 원본만 구현 참고에 사용한다.

## 구현자 read order

1. [Three.js 전투 비주얼 아키텍처](../../ThreeJS_Combat_Visual_Architecture.md)
2. 이 문서의 이미지별 관찰 목표
3. 필요한 WP에 해당하는 외부 저장소와 아래 기준 commit
4. 현재 ScenePage.combat schema와 실제 fixture

## 화면 레퍼런스

### 1. 보드 구성과 전술 가독성

로컬 원본: /home/dudu/work/references/tui-adv-threejs-combat/images/autobattler-board-composition.png

- 참고: 고정 시점에서 보드 전체가 한눈에 읽히는 구도, compact unit scale, 셀과 진영
  표식, HP bar, 전투 영역과 HUD의 계층.
- 그대로 따르지 않음: 상점·벤치·재화 UI, 캐릭터 IP, 정확한 색·타일·장식.
- 구현 질문: 7×6·12명 fixture에서 머리·몸통·무기·진영 ring이 서로 합쳐지지 않는가.

### 2. authored 2D 감정 초상

로컬 원본: /home/dudu/work/references/tui-adv-threejs-combat/images/authored-anime-portrait.png

- 참고: 주요 인물의 얼굴·머리카락·의상 디테일, 대화 장면에서 감정을 맡길 수 있는
  초상 품질.
- 역할: 보드 얼굴을 확대해 감정을 전달하는 대신 Storybook HUD/대화 surface가 사용한다.
- 그대로 따르지 않음: 캐릭터 디자인, 모자·장신구·색 조합.

### 3. 애니풍 3D 전신 비율

로컬 원본: /home/dudu/work/references/tui-adv-threejs-combat/images/anime-3d-fullbody-proportion.png

- 참고: 2D 애니 디자인을 3D silhouette로 번역하는 방식, 단순한 얼굴과 큰 눈,
  머리·의상·소품으로 정체성을 만드는 방향.
- 정본 보정: 실제 보드 모델은 이 예시보다 4~4.5등신 semi-SD로 압축하고, 얼굴을
  gameplay semantic으로 사용하지 않는다.

### 4. semi-SD 3D 포즈와 의상 덩어리

로컬 원본: /home/dudu/work/references/tui-adv-threejs-combat/images/semi-sd-3d-pose.png

- 참고: 고정 카메라에서 읽히는 큰 포즈, 의상과 신발의 색 덩어리, 무릎을 꿇은
  silhouette, compact proportion.
- 구현 질문: idle, attack, hit, evade, incapacitated가 얼굴 없이도 포즈로 구별되는가.

## 코드 레퍼런스

### Wave-Racer

- URL: https://github.com/Vyom-26/Wave-Racer
- 기준 commit: 588653b0e25c4fe9efd03026eae00bffd7b2995b
- 분석에 사용한 로컬 clone: /home/dudu/work/references/Wave-Racer
- 먼저 볼 파일:
  - ARCHITECTURE.md: subsystem 경계, frame-loop allocation, 성능 계측 관점
  - src/core/types.ts: renderer-neutral subsystem contract
  - src/rider/riderAnim.ts: signal/state/modifier/IK 분리 사례
  - src/render/renderer.ts, src/render/textures.ts: renderer resource 소유와 공유
  - harness/: deterministic capture와 probe 방식
- 가져올 교훈: fixed-step truth와 render interpolation 분리, shared resource, capture 기반
  시각 검증, 프레임 루프 allocation 억제, 실제 draw/resource 측정.
- 주의: 수상 레이싱의 chase camera, water pipeline, 전체 postprocess stack은 우리 고정
  전투판 요구가 아니다. 구조와 측정법을 선택적으로 사용한다.

### LinearAbiltyCastingThreeJS

- URL: https://github.com/achrefelouafi/LinearAbiltyCastingThreeJS
- 기준 commit: f9ba4f91bfa1506b98f5f3cf801b80a975d7dd1a
- 분석에 사용한 로컬 clone: /home/dudu/work/references/LinearAbiltyCastingThreeJS
- 먼저 볼 파일:
  - src/abilities/AbilityManager.js: effect lifecycle와 동시 효과 관리
  - src/utils/ObjectPool.js: 재사용 pool
  - ribbon/beam/impact 관련 geometry와 material: 궤적 primitive
  - src/postprocessing/PostProcessing.js: 품질 실험 후보와 비용 구조
- 가져올 교훈: IDLE/TRAVEL/IMPACT/FADE/DONE lifecycle, ribbon·burst·decal 조합,
  object pooling, GPU resource disposal, 동시 효과 상한.
- 우리 계약으로 바꿀 부분: Math.random(), wall-clock, pointer/camera 의존을 제거하고
  ScenePage.combat의 ordered tick/cue와 canonical visual seed를 사용한다.

## 적용 규칙

- private-study 프로젝트이므로 코드·셰이더·구조·에셋을 학습과 프로토타입에 직접
  적용할 수 있다.
- 적용할 때 최소한 source URL + commit + 가져온 범위 + 변경한 파일을 구현 보고서에
  남긴다. 현재 사용을 막기 위한 license gate가 아니라 미래의 교체·추적을 위한 기록이다.
- screenshot이나 reference repo의 고유 IP를 게임 세계관의 최종 캐릭터·로고·문구로
  확정하지 않는다. 화면 문법과 기술 구현을 참고하고 이구학지 고유 디자인으로 교체한다.
- 공개·배포로 전환할 때만 provenance 목록을 기준으로 권리·유사성 감사를 다시 연다.

## 이미지 무결성

| 파일 | 크기 | SHA-256 |
| --- | --- | --- |
| autobattler-board-composition.png | 1920×1080 | 23062e668b716c3c3a445da6a0d032c33b9a7e39ef9b7fe96a2c4014eff92b29 |
| authored-anime-portrait.png | 1042×1509 | 24cd52a3fda9a41464dd53fcfd063b94d08ca52813f462980cc49ab85af8448d |
| anime-3d-fullbody-proportion.png | 640×1242 | f67072d36dc8c520d3f234ee3b97a4e6a79f66615385cf4ed086917c68efba17 |
| semi-sd-3d-pose.png | 291×422 | fd51ab4a726a1c3909a38f9cae1b364f989d4a41a5268a471626feb99608f489 |
