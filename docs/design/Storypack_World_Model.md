# 스토리팩 세계관 모델

## 1. 문서 목적

이 문서는 `tui_adv`를 “회사 아포칼립스 전용 게임”이 아니라, 여러 세계관 storypack을 갈아끼울 수 있는 선택지/생존/인카운터 엔진으로 개발하기 위한 기준을 정한다.

현재 기본 콘텐츠는 여전히 `escape from the office`다. 그러나 앞으로의 개발 기준은 다음처럼 바꾼다.

```text
Generic narrative engine
  -> world/storypack content bundle
  -> Rust GameCore truth
  -> ScenePage / ActionResult
  -> Web Storybook + GlyphFX
  -> SuperLightTUI terminal renderer
```

즉, “회사”는 엔진의 정체성이 아니라 첫 번째 기본 storypack이다. 신규 기능은 office 전용 단어, office 전용 surface, office 전용 resource 해석에 바로 묶지 않고, 최소 두 세계관에서 설명 가능한 형태로 설계한다.

## 2. 현재 고정 결정

- 기본 storypack: `office_isolation_pack` 계열. 기존 문서와 런타임 콘텐츠의 현재 구현체다.
- 첫 비-office 기준 storypack: `wuxia_jianghu_pack`.
- 첫 목표는 런타임 다중 storypack 선택 UI가 아니라, 새 설계/콘텐츠가 office 없이도 성립하는지 검증하는 것이다.
- Rust GameCore / `ScenePage` / WASM JSON boundary는 계속 renderer-neutral truth를 소유한다.
- Web Storybook과 SuperLightTUI는 storypack-specific 문장과 semantic presentation hint를 표시하지만, storypack별 규칙을 renderer에서 재계산하지 않는다.

## 3. Storypack과 world의 관계

이 프로젝트에서 `world`와 `storypack`은 다음처럼 구분한다.

| 개념 | 의미 | 예 |
|---|---|---|
| `world_id` | 큰 장르/세계관 축 | `office_apocalypse`, `wuxia_jianghu` |
| `storypack_id` | world 안에서 실제 사건/덱을 묶는 후보 | `isolation_pack`, `wuxia_jianghu_pack` |
| `surface` | 사건이 드러나는 매체/표면 | 사내 메신저, 객잔 소문, 비급 주석, 문파 패 |
| `pressure_type` | 플레이어를 압박하는 축 | 체력, 정신력, 위험도, 관계, 명예, 내공 불안정 등 |
| `route_hook` | 탈출/정복/진실/히든 같은 큰 목표와 연결되는 고리 | 격리 규칙, 귀환 단서, 문파 소속, 비급 진실 |

`world_id`는 나중에 machine-readable schema로 열 수 있지만, 지금은 docs-first 설계 필드로만 사용한다.

## 4. 엔진 중립 원칙

새 기능을 설계할 때 다음 질문을 통과해야 한다.

1. 이 기능은 회사가 아닌 무협 storypack에서도 의미가 있는가?
2. 특정 surface가 office 전용이라면, 무협에서 대응 surface는 무엇인가?
3. renderer가 이 기능을 표시만 하는가, 아니면 storypack 규칙을 재계산하고 있는가?
4. `ScenePage`에 renderer-specific 또는 world-specific presentation detail을 넣고 있지 않은가?
5. 런타임 save/state key가 영구적으로 `escape-office`에 묶여야 하는가, 아니면 compatibility layer로 남길 것인가?

## 5. 기존 office 요소의 일반화

| 현재 office 표현 | 일반화된 의미 | 무협 대응 예 |
|---|---|---|
| 사내 메신저 | 원격/지연 커뮤니케이션 surface | 전서구, 객잔 전언, 강호 게시판 |
| 회의록 | 합의/기록/왜곡된 증언 surface | 문파 공문, 장문인 명령서, 객잔 증언 |
| 조직도/근태 | 존재/소속/권한 판정 surface | 문파 소속, 제자 명부, 통행 영패 |
| CCTV/출입기록 | 감시/동선/증거 surface | 표국 장부, 객잔 목격담, 경공 흔적 |
| 서버 로그/build log | 규칙/분기/진실 기록 surface | 비급 주석, 진법 기록, 천기록 |
| 사원증/보안권한 | 통과/인증 item | 사원증이 바뀐 문파 패, 통행 영패, 추천서 |
| 사내 방송 | 광역 공지/압박 cue | 강호 게시판, 객잔 소문, 종루 경보 |
| 현실 연결 힌트 | 선택적 ARG/local-only layer | 기본 무협 pack에서는 비활성 또는 별도 local layer |

## 6. Resource와 ability 일반화

기존 core 자원은 바로 버리지 않는다.

| 현재 자원 | 엔진 의미 | office 표시 | 무협 표시 후보 |
|---|---|---|---|
| `health` | 물리적 생존력 | 신체 반응 | 혈기 / 상처 |
| `sanity` | 현실/자아 안정성 | 집중도 / 정신 안정도 | 심마 / 정신 집중 |
| `battery` | 정보/도구 사용 여력 | 단말기 전원 | 휴대폰 잔량 / 전서 비용 / 내공 감각 |
| `hunger` | 장기 생존 압박 | 허기 | 굶주림 / 기력 고갈 |
| `thirst` | 빠른 생존 압박 | 갈증 | 갈증 / 진기 소모 |
| `danger` | 세계/구역 압박 | 격리 위험도 | 추적도 / 살기 / 강호 소문 |

첫 단계에서는 내부 field명을 바꾸지 않는다. world별 display label과 content text로 해석을 분리한다. 필드명 rename은 save/schema migration이 필요한 별도 작업이다.

## 7. Storypack authoring contract

새 storypack 문서는 최소한 다음을 가진다.

```yaml
id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
name: 무협 강호팩
one_line: 한 줄 컨셉
main_surfaces: []
anomaly_types: []
main_phases: []
reusable_npc_slots: []
ending_candidates: []
main_spine_support: 엔진의 큰 루프를 어떻게 검증하는지
runtime_promotion_notes: 첫 runtime slice 후보와 금지선
```

Encounter situation card는 다음 필드를 권장한다.

```yaml
id: wuxia_office_worker_arrival
storypack_id: wuxia_jianghu_pack
world_id: wuxia_jianghu
phase: wuxia_arrival
priority_class: random_pack
location_tags: [inn, starting_room]
surface: inn_room
anomaly_type: world_displacement
pressure_type: [sanity, relation]
choice_shapes:
  - role: safe_observe
  - role: social_probe
  - role: high_risk_route_opening
main_spine_link: office worker가 다른 world로 이동해도 같은 engine loop를 검증하는 이유
```

## 8. 첫 비-office 기준팩: 회사원 차원이동형 무협 강호팩

`docs/content/storypacks/wuxia_jianghu_pack.md`를 첫 기준팩으로 둔다. 이 팩의 플레이어 전제는 “회사에 다니던 직장인이 눈떠보니 무협 세계에 떨어졌다”이다.

이 팩이 필요한 이유:

- office에서 출발한 플레이어가 office-specific surface 없이도 선택지/상태/인카운터/route hook을 이어갈 수 있는지 검증한다.
- 최근 전투 시스템 문서의 “자동 난투 + 상황 개입” 설계를 자연스럽게 시험할 수 있다.
- Web Storybook의 모바일 게임북 UI는 무협 장면에도 잘 맞는다.
- SuperLightTUI는 강호 게시판, 비급 주석, 객잔 전언, 문파 패 같은 텍스트 surface를 terminal-native로 표현하기 좋다.

## 9. 개발 순서 제안

### Step 1 — docs-first world/storypack foundation

현재 문서 변경의 범위다.

- `Storypack_World_Model.md` 추가.
- `wuxia_jianghu_pack.md` 추가.
- `encounter_db/wuxia_jianghu_pack.md` 추가.
- README/Index/Development_Plan/Checklist/AGENTS를 office-only 표현에서 storypack-capable 표현으로 조정.

### Step 2 — machine-readable storypack DB 검토

다음 후보 작업이다.

- docs-only storypack을 YAML/JSON DB로 옮길지 결정한다.
- `world_id`, `storypack_id`, `surface`, `phase`, `route_hook` 필드를 machine-readable로 검증한다.
- 아직 runtime game content로 바로 섞지 않는다.

### Step 3 — first runtime wuxia route slice

별도 구현 작업에서 진행한다.

- 기존 encounter schema로 무협 인카운터 1~3개를 넣는다.
- 가능하면 `ScenePage` schema 변경 없이 진행한다.
- CLI/Rust/Web generated content smoke로 office가 아닌 location/encounter도 표시되는지 검증한다.
- 저장 키와 시작 화면의 `escape-office` 명칭은 compatibility 이슈로 별도 migration plan을 세운다.

## 10. 금지선

- 이번 단계에서 기존 office runtime 콘텐츠를 삭제하거나 약화하지 않는다.
- `escape from the office` 문서/아카이브의 역사적 맥락을 억지로 rename하지 않는다.
- save/localStorage key를 즉시 바꾸지 않는다.
- 무협팩에 실제 역사 인물, 실제 종교/민족/정치 민감 소재를 무리하게 넣지 않는다. 소림/무당/아미산은 공개 자료 기반 분위기 앵커로만 쓰고 세부 역사 재현을 단정하지 않는다.
- renderer에 world별 gameplay truth를 넣지 않는다.

## 11. 현재 상태

- 상태: 설계 문서화 완료.
- 첫 비-office storypack: `wuxia_jianghu_pack`(회사원 차원이동형 무협).
- 런타임 구현: 미착수.
- 다음 추천: docs-first 후보를 바탕으로 machine-readable storypack DB 또는 schema-less wuxia runtime encounter prototype 중 하나를 선택한다.
