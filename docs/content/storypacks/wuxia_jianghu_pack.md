# 무협 강호팩

Status: candidate

## Record

```yaml
id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
source_refs:
  - user_request:2026-05-29-office-worker-wuxia-isekai
  - https://en.wikipedia.org/api/rest_v1/page/summary/Wuxia
  - https://en.wikipedia.org/api/rest_v1/page/summary/Shaolin_Kung_Fu
  - https://whc.unesco.org/en/list/1305/
  - https://whc.unesco.org/en/list/705/
  - https://whc.unesco.org/en/list/779/
name: 무협 강호팩
one_line: 회사에 다니던 직장인이 퇴근 직전의 기억을 마지막으로, 눈떠보니 객잔과 문파와 강호 규칙이 있는 무협 세계에 떨어진다.
main_surfaces:
  - inn_room
  - jianghu_notice_board
  - courier_letter
  - martial_manual
  - tavern_rumor
  - sect_token
  - duel_record
anomaly_types:
  - world_displacement
  - workplace_memory_mismatch
  - foreigner_without_sect
  - manual_as_onboarding
  - qi_deviation
  - oath_binding
main_phases:
  - office_departure
  - wuxia_arrival
  - jianghu_orientation
  - sect_contact
  - route_commitment
  - resolution_pressure
sensitive_topics:
  - real_history_similarity
  - martial_violence
  - religious_site_reference
reusable_npc_slots:
  - innkeeper_guide
  - shaolin_anchor_monk
  - wudang_anchor_taoist
  - emei_anchor_swordswoman
  - courier_broker
ending_candidates:
  - return_to_office
  - leave_jianghu
  - join_orthodox_sect
  - reveal_displacement_truth
  - break_oath_curse
main_spine_support: office에서 출발한 현대 직장인이 전혀 다른 world로 이동해도 선택지, 자원 압박, NPC 관계, 전투 개입, 기록 surface, 귀환/정착/진실 루트가 성립하는지 검증한다.
runtime_promotion_notes: 첫 구현은 기존 encounter schema만 사용한 schema-less wuxia prototype으로 시작한다. 첫 runtime 후보는 `wuxia_office_worker_arrival`이며, world_id/storypack_id schema는 반복 필요가 확인된 뒤 연다.
```

## 핵심 컨셉

플레이어는 회사에 다니던 평범한 직장인이다. 퇴근 직전 엘리베이터를 탔거나, 야근 중 잠깐 눈을 감았거나, 사내 시스템의 이상한 알림을 눌렀다.

다음 순간 플레이어는 어느 객잔의 이층 방에서 깨어난다. 품속에는 사원증 대신 낯선 나무 패가 있고, 휴대폰은 켜지지 않으며, 객잔 아래층에서는 “소림”, “무당”, “아미산 쪽 검수”, “표국” 같은 말을 당연하게 주고받는다.

이 팩의 핵심은 정통 무협 지식을 많이 요구하는 것이 아니라, 현대 회사원의 사고방식이 강호의 규칙과 부딪히는 데 있다.

## 배경 길이 기준

이 팩은 아직 무협 지식이 많지 않은 상태에서 쓰는 첫 기준팩이므로 배경을 짧게 유지한다.

- 첫 문장 목표: “회사원이 눈떠보니 무협 세계 객잔이다.”
- 깊은 사문 계보, 장대한 왕조사, 복잡한 무공 단계는 당장 만들지 않는다.
- 소림/무당/아미산 같은 이름은 분위기와 방향을 잡는 자료 기반 앵커로만 쓴다.
- 게임 내 고유 문파와 세부 인물은 나중에 충분히 조사한 뒤 별도 storypack/card에서 확장한다.

## 자료 기반 문파/강호 앵커

사용자가 요청한 “공신력 있는 자료 기반” 원칙에 따라, 첫 slice에서는 아래처럼 확인 가능한 공개 자료를 앵커로 둔다. 다만 게임은 역사/종교/무술 재현물이 아니라 fiction storypack이므로, 실제 단체·장소를 세부 설정으로 단정하지 않는다.

| 앵커 | 자료에서 확인한 용도 | 게임 내 사용 원칙 |
|---|---|---|
| 무협 / wuxia | 중국 무술가의 모험을 다루는 중국식 fiction/low fantasy 장르로 요약된다. | 장르 문법 참고. 현대 직장인 시점으로 쉽게 풀어 쓴다. |
| 소림 / Shaolin | Shaolin kung fu는 소림사와 연결된 유명한 중국 무술 전통으로 소개된다. Dengfeng의 역사 기념물은 UNESCO World Heritage 자료가 있다. | “소림 계열 승려/무승” 정도의 낮은 해상도 앵커로 사용한다. |
| 무당산 / Wudang | Wudang Mountains는 도교 사원/수도원 복합체로 알려져 있고 UNESCO World Heritage 등재 자료가 있다. | “무당 계열 도사/검법” 분위기 앵커로 사용한다. |
| 아미산 / Mount Emei | Mount Emei Scenic Area는 UNESCO World Heritage 자료가 있고 중국 불교 성산으로 널리 알려져 있다. | “아미산 쪽 검수/여행자” 같은 지리·분위기 앵커로만 사용한다. |

참조 URL:

- https://en.wikipedia.org/api/rest_v1/page/summary/Wuxia
- https://en.wikipedia.org/api/rest_v1/page/summary/Shaolin_Kung_Fu
- https://whc.unesco.org/en/list/1305/
- https://whc.unesco.org/en/list/705/
- https://whc.unesco.org/en/list/779/

## 톤

초반은 이세계 적응 블랙코미디다.

- 플레이어는 강호 게시판을 사내 공지처럼 읽으려 한다.
- 객잔 숙박비를 법인카드/교통카드/사원증으로 해결하려고 한다.
- “문파가 어디냐”는 질문을 “소속 부서가 어디냐”로 오해한다.
- 비급은 온보딩 문서처럼 보이지만, 읽으면 몸이 먼저 반응한다.

중후반은 강호 규칙과 정체성 압박이다.

- 사원증은 더 이상 통하지 않지만, 낯선 문파 패가 문을 연다.
- 소문과 공고가 플레이어의 신분을 먼저 정의한다.
- 소림/무당/아미산 계열 인물은 귀환, 정착, 맹세, 전투 개입의 서로 다른 해석을 제공한다.
- 최종 선택은 회사로 돌아가는 것, 강호를 떠나는 것, 문파에 들어가는 것, 차원이동의 진실을 밝히는 것 사이에서 갈린다.

## 메인 story spine 연결

| phase | 이 팩에서의 역할 |
|---|---|
| `office_departure` | 마지막 office 기억을 짧게 제시한다. 야근, 엘리베이터, 사내 알림 등. |
| `wuxia_arrival` | 객잔에서 깨어나고, 사원증/휴대폰/회사 언어가 통하지 않음을 확인한다. |
| `jianghu_orientation` | 객잔, 게시판, 표국, 소문을 통해 강호 기본 규칙을 배운다. |
| `sect_contact` | 소림/무당/아미산 계열 인물이나 문파 패를 통해 소속/맹세/수련 압박을 받는다. |
| `route_commitment` | 귀환, 강호 정착, 문파 가입, 차원이동 진실 추적 중 방향이 생긴다. |
| `resolution_pressure` | 현대인의 정체성과 강호에서 얻은 이름 중 무엇을 남길지 선택한다. |

## 주요 surface

### inn_room

- 첫 장면의 안전지대다.
- 회사원의 물건이 무협 세계의 물건으로 바뀌어 있는지 확인한다.
- 휴대폰, 사원증, 가방, 지갑 같은 현대 물건의 실패를 보여준다.

### jianghu_notice_board

- 객잔 벽의 강호 게시판.
- 현상금, 문파 소식, 표국 의뢰, 비무 일정이 붙는다.
- 플레이어는 이를 사내 공지/이슈 보드처럼 해석하려 한다.

### courier_letter

- 전서구, 표국, 심부름꾼이 전달하는 편지.
- 날짜와 발신자가 어긋나며 귀환/정착 루트 단서를 준다.

### martial_manual

- 비급, 검보, 권법 주석.
- 플레이어에게는 온보딩 문서나 운영 매뉴얼처럼 보인다.
- 너무 오래 읽으면 심마/기혈 역류 압박이 생긴다.

### tavern_rumor

- 객잔 소문, 술자리 증언, 점소이의 관찰.
- 강호 평판과 NPC 관계를 움직인다.
- 플레이어는 이를 회사 슬랙/메신저 rumor처럼 읽으려 한다.

### sect_token

- 문파 패, 통행 영패, 추천서.
- office pack의 사원증/보안권한 surface에 대응한다.
- “소속”과 “통과 권한”을 동시에 다룬다.

## 반복 NPC slot

| slot | 후보 인물 | 자료 앵커 | 기능 |
|---|---|---|---|
| `innkeeper_guide` | 노객주 | 객잔/강호 소문 trope | 현대 직장인의 오해를 받아 주는 첫 안내자. |
| `shaolin_anchor_monk` | 행각승 | Shaolin / Dengfeng anchor | 폭력보다 절제와 귀환 가능성을 묻는 인물. |
| `wudang_anchor_taoist` | 떠돌이 도사 | Wudang / Taoist mountain anchor | 기이한 차원이동을 도가적 언어로 해석한다. |
| `emei_anchor_swordswoman` | 아미산 쪽 검수 | Mount Emei anchor | 문파 규칙, 명예, 여성 검수 trope를 낮은 해상도로 제공한다. |
| `courier_broker` | 표국 중개인 | 표국/전서 surface | 편지, 이동, 비용, 귀환 단서를 연결한다. |

## 후보 인카운터 카드

| id | phase | surface | 핵심 상황 | 승격 우선순위 |
|---|---|---|---|---|
| `wuxia_office_worker_arrival` | `wuxia_arrival` | `inn_room` | 회사원이 객잔에서 깨어나고 현대 물건이 통하지 않는다. | 높음 |
| `wuxia_notice_foreigner_without_sect` | `jianghu_orientation` | `jianghu_notice_board` | 강호 게시판에 “출신불명 외지인” 경고가 붙어 있다. | 높음 |
| `wuxia_manual_as_onboarding` | `jianghu_orientation` | `martial_manual` | 비급을 회사 온보딩 문서처럼 읽다가 몸이 먼저 반응한다. | 높음 |
| `wuxia_shaolin_wudang_emei_rumor` | `sect_contact` | `tavern_rumor` | 객잔 소문으로 소림/무당/아미산 계열 선택지의 성격을 짧게 배운다. | 중간 |
| `wuxia_badge_mistaken_for_token` | `sect_contact` | `sect_token` | 사원증이 문파 패로 오해받아 통과 권한과 의심을 동시에 만든다. | 중간 |
| `wuxia_duel_bridge_intervention` | `route_commitment` | `duel_record` | 다리 위 결투에서 자동 난투가 벌어지고 1회 개입 기회가 생긴다. | 중간 |

상세 카드는 `docs/content/encounter_db/wuxia_jianghu_pack.md`에 둔다.

## 기존 시스템과의 대응

| 기존 office 축 | 무협팩 대응 |
|---|---|
| 사내 메신저/공지 | 강호 게시판, 객잔 소문, 전서구 편지 |
| 사원증/보안권한 | 문파 패, 통행 영패, 추천서 |
| 조직도/근태 | 문파 소속, 사제 관계, 표국 장부 |
| CCTV/출입기록 | 객잔 목격담, 표국 이동 기록, 결투 증언 |
| 서버 로그/build log | 비급 주석, 진법 기록, 사부의 편지 |
| 정신력 왜곡 | 심마, 낯선 세계 적응 실패, 기억 불일치 |
| 배터리/정보 비용 | 휴대폰 고갈, 전서 비용, 내공/기혈 소모 |
| 자동 난투 + 개입 | 결투/협공/다리 위 몸싸움의 결정적 순간 선택 |

## Runtime promotion notes

첫 runtime 승격 후보는 다음 3개가 적합하다.

1. `wuxia_office_worker_arrival`
   - 사용자가 정한 “회사 직장인 → 무협 세계 차원이동” 전제를 가장 직접적으로 보여준다.
   - 기존 encounter schema의 title/body/choices만으로 구현 가능하다.
2. `wuxia_notice_foreigner_without_sect`
   - 강호 세계의 소속/평판/문파 질문을 짧게 소개한다.
   - Web Storybook의 story flow와 잘 맞다.
3. `wuxia_duel_bridge_intervention`
   - 최근 전투 시스템 설계의 첫 비-office 검증 후보가 된다.
   - 새 combat schema 없이 “자동 난투 body + 1회 choice”로 시작할 수 있다.

주의:

- 첫 구현에서는 기존 office route와 같은 runtime bundle에 섞기 전에 `world_id`/`storypack_id` gating 전략을 정해야 한다.
- 현재 save/localStorage key의 `escape-office` 명칭은 compatibility 이슈로 남겨 두고, 이름 변경은 별도 migration으로 다룬다.
- 무협팩에는 현실 사무실 final hint를 연결하지 않는다. 현실 연결이 필요하면 world별 local-only layer를 따로 설계한다.
- 실제 소림/무당/아미산 단체나 종교·문화재 설명을 게임 설정처럼 단정하지 않는다. 첫 slice는 낮은 해상도의 장르 앵커만 사용한다.
