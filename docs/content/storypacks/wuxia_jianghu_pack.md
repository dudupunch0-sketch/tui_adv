# 이구학지 — 천기록

Status: candidate

## Record

```yaml
id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
source_refs:
  - idea_box/done/2026-05-29-notion-wuxia-igu-hakji-cheonggi-record.md
  - notion:36f37e69-695e-8198-86c2-d35c00609441
  - docs/dev/Notion_Design_Coverage.md
  - idea_box/notion_sources.yml
name: 이구학지 — 천기록
previous_placeholder_status: superseded
one_line: 현대 회사원이 본인 몸과 출근복장 그대로 무협 세계의 시장 한복판에 떨어지고, 흑사방 첫 전투와 청류문 수습생 구간을 거쳐 천기록/천외편린 기반의 랜덤 현대지식 성장 구조를 경험한다.
main_surfaces:
  - commute_rift
  - market_street
  - office_items
  - sect_courtyard
  - training_chore
  - cheonggi_record
  - fragment_choice
  - sect_raid
  - faction_negotiation
anomaly_types:
  - world_displacement
  - workplace_memory_mismatch
  - outsider_without_sect
  - first_brawl_defeat
  - notebook_oracle
  - fragment_choice
  - sect_debt
  - faction_pressure
main_phases:
  - commute_rift
  - market_arrival
  - first_brawl
  - rescue_and_investigation
  - cheongryu_apprenticeship
  - cheonggi_record_awakening
  - cheongryu_raid
  - route_commitment
  - resolution_pressure
sensitive_topics:
  - martial_violence
  - fictional_sect_politics
  - return_to_modern_life
reusable_npc_slots:
  - early_rescuer
  - sect_master_guardian
  - archive_keeper
  - righteous_ally
  - sapa_ally
  - cheonggi_record_keeper
  - blood_moon_antagonist
ending_candidates:
  - cheongryu_divine_sword
  - white_path_prison
  - black_night_gentleman
  - debtor_of_all_under_heaven
  - returnee
  - murim_outsider
main_spine_support: office에서 출발한 현대인이 완전히 다른 world에 떨어져도 소속, 평판, 관계, 성장, 전투, 자원, 선택, 엔딩을 같은 engine loop로 설명할 수 있는지 검증한다.
runtime_promotion_notes: `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid` runtime은 separate storypack preview bundle에서 완료했다. `wuxia_boss_followup_after_first_appearance` handoff는 `wuxia_mumyeong_request_for_aid`를 골랐고, 구현 slice는 보스 첫 등장 뒤 무명의 도움 요청 실패 기록을 departure truth reveal/full flashback/final resolution 없이 flags/clues/log/presentation hook으로 landing했다. `wuxia_mumyeong_followup_after_failed_aid` docs-only handoff는 다음 runtime 후보를 `wuxia_mumyeong_awakening`으로 결정했다. 2026-06-01 default 전환 이후 Web player 기본 경로는 이구학지이며, 2026-06-02 terminal content scene default도 이구학지 fixture로 맞췄다. office content는 legacy/parity fixture로 남긴다. legacy office bundle, legacy `escape-office` save/localStorage key, 천외편린 3택 성장 schema, faction/relation/debt ledgers는 아직 열지 않는다.

runtime_history_note: `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid` runtime은 separate storypack preview bundle에서 완료했다. `wuxia_mumyeong_followup_after_failed_aid` handoff는 완료됐고 다음 구현 후보는 `wuxia_mumyeong_awakening`이다.
runtime_compat_note: `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style` runtime은 separate storypack preview bundle에서 완료했고, 이후 `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`도 같은 bundle에 구현했다.
```

## 최신화 기준

이 문서가 현재 repo canonical 무협 storypack이다.

Notion upstream 기준은 2026-06-01 live check 이후 다음 precedence를 따른다.

1. `무협 스토리팩: 이구학지 — 천기록` 상위 문서는 초기 기획/시놉시스 성격을 포함한다.
2. 최신 세부 운영 기준은 Notion 하위 관리 문서 `00`~`08`, `09. 이구학지 사건 카드 DB`, `10. 이구학지 후일담 카드 DB`, `99. 통합 체크포인트`를 우선한다.
3. 사건/후일담/인물의 최신 상태는 긴 본문보다 사건 카드 DB와 후일담 카드 DB를 기준으로 본다.
4. repo runtime 구현은 Notion DB를 직접 따르지 않고, 이 문서와 `docs/content/encounter_db/wuxia_jianghu_pack.md`, `docs/dev/Development_Plan.md`에 반영된 handoff를 먼저 통과한다.
5. Notion DB row는 design source이며 runtime 구현 완료 표시가 아니다.

이전의 짧은 “회사원 객잔 도착 + 소림/무당/아미 저해상도 앵커” 무협 placeholder는 폐기/대체한다. 앞으로 무협팩을 언급할 때는 Notion에서 갱신된 **이구학지 — 천기록** 설정을 기준으로 하되, 상위 문서와 하위 문서/DB가 충돌하면 하위 문서/DB를 우선한다.

중요한 차이:

- 시작 위치는 객잔 방이 아니라 **출근길 균열 이후 시장 한복판**이다.
- 주인공은 빙의자가 아니라 **본인 몸, 출근복장, 사원증, 지갑 일부, 볼펜, 업무수첩**을 가진 이방인이다.
- 스마트폰은 전이 직후 사라진다.
- 초반 핵심 사건은 흑사방 불량배와의 첫 전투, 서하린의 구조, 청류문 수습생 편입이다.
- 성장 축은 검색 가능한 스마트폰이 아니라 업무수첩과 연결된 **천기록**, 그리고 랜덤 현대지식 보상인 **천외편린**이다.
- 실제 소림/무당/아미 같은 현실 단체·장소는 현 단계의 active setting이 아니다. 필요하면 장르 참고 자료로만 낮은 해상도에서 다룬다.

## 핵심 컨셉

주인공은 현대의 평범한 회사원이다. 평소처럼 출근하던 중 엘리베이터 문, 지하철 문, 횡단보도 신호 같은 일상적 경계에서 하얀 균열에 휘말린다.

다시 눈을 뜨면 형광등 대신 종이등, 아스팔트 대신 흙길, 자동차 소리 대신 말발굽 소리가 있다. 사람들은 한복과 무복을 입고 있고, 주인공은 정장 차림 그대로 시장 한복판에 서 있다.

남은 것은 다음뿐이다.

- 정장과 구두
- 목에 건 사원증
- 지갑 일부
- 볼펜
- 업무수첩
- 커피 또는 가방 같은 출근길 소지품

스마트폰은 없다. 이 부재가 중요하다. 이 storypack은 “현대 지식을 검색해서 해결하는 무협”이 아니라, 현대인의 관찰력, 기록 습관, 효율화 사고, 실패 복기 능력이 무협 수련과 충돌하고 융합되는 이야기다.

## 제목 의미

**이구학지**는 다른 시대와 다른 세계의 지식으로, 낯선 세계의 법칙을 다시 배운다는 의미다.

주인공은 무협 세계의 천재가 아니다. 하지만 현대인의 관찰력과 기록 습관, 반복 훈련에 대한 감각, 현대식 사고의 파편을 바탕으로 무림의 수련법을 다르게 해석한다.

```text
기연은 우연히 온다.
선택은 플레이어의 몫이다.
숙련은 몸에 새겨야 한다.
```

## 메인 story spine

| phase | 역할 |
|---|---|
| `commute_rift` | 출근 중 일상적 경계에서 균열이 열리고, 플레이어가 본인 몸 그대로 전이된다. |
| `market_arrival` | 시장 한복판에서 이질감, 스마트폰 부재, 사원증 오해, 현대 물건의 한계를 확인한다. |
| `first_brawl` | 흑사방 말단 불량배와의 튜토리얼성 전투로 이 세계의 위험과 부상 현실감을 보여준다. |
| `rescue_and_investigation` | 서하린이 개입하고, 주인공은 정체불명의 외지인으로 조사받는다. |
| `cheongryu_apprenticeship` | 청류문 수습생/객식/잡역/임시 보호 대상이 되어 빚과 잡일, 기초 수련을 시작한다. |
| `cheonggi_record_awakening` | 업무수첩이 천기록과 연결되고, 천외편린 3택 성장 구조가 열린다. |
| `cheongryu_raid` | 청류문 습격 사건으로 정파/사파/천기·귀환 루트의 중반 분기점이 열린다. |
| `route_commitment` | 백도맹, 흑천련, 천기각 중 어느 축과 손잡을지 선택한다. |
| `resolution_pressure` | 무공은 누구의 것인지, 귀환할지 정착할지, 천기록을 어떻게 다룰지 결말 압박을 만든다. |

## 공용 시스템 대응

| 공용 시스템 | 회사팩 표현 | 이구학지 표현 |
|---|---|---|
| 소속 | 부서, 팀, 임시 대응방 | 청류문 수습생, 문파, 백도맹/흑천련/천기각 |
| 평판 | 사내 평판, 권한 로그 | 무림 명성, 외지인 의심, 흑사방의 표적 |
| 권한 | 결재, 승인, 사원증 | 장문 허가, 문파 보호, 통행/참가 자격 |
| 과업 | 업무, 프로젝트, 장애 대응 | 잡일, 수련, 의뢰, 호송, 문파 재건 |
| 스트레스 | 번아웃, 공포, 현실감 상실 | 심마, 이방인 정체성 압박, 수련 한계 |
| 체력/부상 | 피로, 컨디션 저하 | 기혈, 내상, 외상, 구두/정장으로 인한 불리함 |
| 동료 | 팀원, 사수, 격리자 | 서하린, 사형제, 낭인, 기록관 |
| 성장 | 스킬 습득, 업무 숙련 | 무공 숙련, 천외편린, 복기수련법 |
| 정보 | 문서, 메신저, 서버 로그 | 비급, 소문, 천기록, 천기각 기록 |
| 장비 | 사무도구, 단말기 | 병장기, 약재, 업무수첩, 볼펜, 출근 가방 |
| 거점 | 사무실, 회의실, 서버실 | 청류문, 시장, 객잔, 산문, 천기각 |

## 주요 surface

### commute_rift

출근길의 일상적 경계가 world transition surface가 된다. 엘리베이터, 지하철, 횡단보도 같은 장면을 짧게 쓰되 실제 회사/역/주소는 넣지 않는다.

### market_street

첫 도착 지점이다. 이질감, 군중의 시선, 흑사방 시비, 현대 물건의 무력함을 동시에 보여준다.

### office_items

정장, 구두, 사원증, 지갑, 볼펜, 업무수첩, 커피/가방이 초반 선택지를 만든다. 이 물건들은 무림을 지배하는 치트가 아니라 임시 변수와 오해의 원천이다.

### sect_courtyard

청류문 수습생 구간의 거점이다. 보호, 빚, 잡일, 수련 허가, 문파 소속을 다룬다.

### training_chore

장작 패기, 물 긷기, 연무장 청소, 약초 말리기, 서고 정리 같은 잡일이 수련과 성장의 반복 루프가 된다.

### cheonggi_record

업무수첩이 천기록과 연결된 surface다. 검색창이 아니며, 원하는 질문에 답하지 않는다. 특정 순간에 현대 지식의 조각이 떠오른다.

최신 Notion 정책상 천기록은 초반에는 시스템처럼 보이고 중후반에는 인격처럼 보이는 기록으로 남긴다. 단, 정체는 끝까지 밝히지 않는다. “정체 접근”은 기록자의 존재감, 시선, 실시간으로 적히는 듯한 문장을 감지하는 정도이며, 세계관 설명이나 정체 reveal로 쓰지 않는다.

### fragment_choice

천외편린 3택 보상 화면이다. 세 후보 중 하나만 고를 수 있고, 나머지 두 개는 사라진다. 즉시 강해지는 보상이 아니라 수련 방향/과제/해석이 열린다.

### sect_raid

청류문 습격 사건처럼 route commitment를 강제하는 큰 사건 surface다. 백도맹, 흑천련, 천기각이 서로 다른 명분과 대가를 제시한다.

## 천기록 / 천외편린 규칙

천기록은 업무수첩과 연결된 기연이다. 스마트폰 검색이나 만능 지식창이 아니다.
플레이어가 질문을 입력해 답을 얻는 도구가 아니며, 조건을 만족한 순간 후보 세 개만 제시한다.
선택은 항상 “후보 3개 중 하나”이며, 고르지 않은 두 후보는 그 순간 사라진다.

```text
천기록 발현
↓
현대 지식 후보 세 개 출현
↓
그중 하나만 선택
↓
나머지 두 개는 사라짐
↓
선택한 지식이 수련 과제나 능력 방향으로 전환됨
```

천외편린 후보 계열:

| 계열 | 역할 |
|---|---|
| 현대 무술 | 복싱, 레슬링, 거리 조절, 방어 자세 같은 현대 격투 감각을 무공의 빈틈 해석에 사용한다. |
| 훈련법 | 반복 훈련, 휴식, 회복, 교정, 과훈련 방지 등 장기 성장 효율을 다룬다. |
| 응급 처치 | 출혈, 염좌, 탈수, 부상 악화를 줄이는 생존 지식이다. |
| 생존 전술 | 도주, 은신, 지형 활용, 다수전 회피, 무기 든 상대 대응을 다룬다. |
| 사고법 | 기록, 복기, 원인 분석, 협상, 심리전, 위험관리처럼 회사원 출신 색깔을 살린다. |
| 귀환 단서 | 원래 세계, 전이 원인, 천기록의 정체에 대한 희귀 조각이다. |

천외편린은 큰 전투 후, 심각한 패배 후, 수련 한계에 부딪힌 밤, 중요한 선택 직전, 동료와의 유대가 깊어진 뒤, 천기각 유물 근처, 귀환 단서를 발견한 순간에 발현하기 좋다.

## 주요 세력

### 청류문 최신 운영 기준

청류문은 정·사·마 전쟁 이후 몰락한 정파 문파다. 돈, 인력, 지위, 무공 전승이 부족하지만 남은 사람들은 서로를 버리지 않는다. 이 문파는 플레이어가 소속감을 느껴야 하는 따뜻한 언더독 홈베이스이며, 내부 배신자나 내부 정치질로 고구마를 만드는 장소가 아니다.

금지:

- 내부 정치질
- 장로회의 고구마
- 주인공 지속 의심
- 사형제 권력 다툼
- 문파 내부 배신자/내부 적대자

청류문의 문제는 내부 악인이 아니라 결핍이다: 돈 부족, 인력 부족, 약재 부족, 소실된 비급, 무너진 수련장, 장문인 병상, 떠난 제자의 빈 방, 정파 내 지위 상실. 갈등 원천은 무림맹 정치질, 주변 정파의 조롱과 견제, 흑사방의 위협, 전쟁 후유증, 소실된 무공, 청류문 명예의 추락에서 온다.

무공 구조:

- `청류심법`: 몸 안의 기를 고정하지 않고 흐르게 하는 심법. 상대의 기세와 흐름을 받아내기 위한 기반이다.
- `청류안`: 상대의 호흡, 중심, 기의 흐름, 초식의 빈틈을 보는 눈. 단순 동체시력이 아니라 무공 구조를 읽는 감각이다.
- `관류`: 흐름을 본다.
- `수류`: 흐름을 받아낸다.
- `역류`: 흐름을 거슬러 끊는다.
- `환류`: 흐름을 자기 방식으로 되돌린다.

이 구조 덕분에 주인공의 현대식 관찰/기록/복기와 천외편린은 “남의 무공을 그대로 훔치는 치트”가 아니라, 소실된 청류문 무공의 구조와 응용을 자기 몸에 맞게 복구하는 성장 루프로 설명된다.

| 세력 | 역할 | 주의점 |
|---|---|---|
| 청류문 | 주인공이 처음 보호받고 수습생으로 시작하는 몰락한 문파 | 약하지만 따뜻한 출발 거점. 공짜 보호가 아니라 빚과 의무가 있다. |
| 백도맹 | 정파 연합 | 의와 협을 내세우지만 명문 중심 서열과 정치가 있다. |
| 흑천련 | 사파 연합 | 위험하고 실리적이지만 밑바닥을 보호하는 인물도 있다. |
| 천기각 | 무림 기록을 관리하는 중립 세력 | 천기록의 비밀, 이전 이방인 기록, 귀환 루트와 연결된다. |
| 혈월교 | 메인 적대 세력 | “무공은 모두에게 열려야 한다”는 급진 명분을 가졌지만 방식이 극단적이다. |

## 주요 등장인물 slot

| slot | 후보 인물 | 기능 |
|---|---|---|
| `early_rescuer` | 서하린 | 흑사방 첫 전투 후 개입하는 청류문 외문 제자. 초반 구조자이자 수습생 구간 멘토. |
| `sect_master_guardian` | 청류문 장문인 | 보호와 채무, 잡일/수련 허가 조건을 제시한다. |
| `archive_keeper` | 막노인 | 청류문 폐서고와 몰락의 진실, 선택의 대가를 암시한다. |
| `righteous_ally` | 남궁서윤 | 백도맹/정파 루트의 원칙적 동료. |
| `sapa_ally` | 도월 | 흑천련/사파 루트의 거래와 생존 감각을 제공하는 낭인. |
| `cheonggi_record_keeper` | 연소하 | 천기각 기록관. 천기록의 정체와 귀환 루트 단서를 쥔다. |
| `blood_moon_antagonist` | 유하린 | 혈월교주. 무공 독점에 대한 급진적 반론과 메인 갈등을 만든다. |

## 후보 인카운터 카드

| id | phase | surface | 핵심 상황 | 승격 우선순위 |
|---|---|---|---|---|
| `wuxia_commute_rift_arrival` | `commute_rift` / `market_arrival` | `commute_rift`, `market_street` | 출근 중 균열에 휘말려 시장 한복판에 본인 몸 그대로 떨어진다. | 높음 |
| `wuxia_heuksa_bang_first_fight` | `first_brawl` | `market_street`, `office_items` | 흑사방 말단에게 시비가 걸리고, 대부분 패배하는 튜토리얼성 첫 전투를 겪는다. | 높음 |
| `wuxia_seo_harin_rescue` | `rescue_and_investigation` | `market_street` | 서하린이 개입해 주인공을 구조하지만, 정체불명의 외지인으로 의심한다. | 높음 |
| `wuxia_cheongryu_apprentice_entry` | `cheongryu_apprenticeship` | `sect_courtyard`, `training_chore` | 청류문 수습생/객식/잡역으로 편입되고 보호의 대가를 갚기 시작한다. | 높음 |
| `wuxia_cheonggi_record_first_fragment` | `cheonggi_record_awakening` | `cheonggi_record`, `fragment_choice` | 업무수첩이 천기록과 연결되고 첫 천외편린 3택이 열린다. | 높음 |
| `wuxia_cheongryu_chore_sparring` | `cheongryu_apprenticeship` / `basic_combat_training` | `sect_courtyard`, `training_chore`, `office_items` | 장작 마당 잡일 중 첫 몸싸움식 겨루기로 균형·호흡·발디딤 수련 hook을 남긴다. | 중간 |
| `wuxia_cheongryu_raid_route_split` | `cheongryu_raid` / `route_commitment` | `sect_raid`, `faction_negotiation` | 청류문 습격 후 백도맹/흑천련/천기각 루트 선택 압박이 생긴다. | 중간 |
| `wuxia_cheongryu_raid_wounded_fallback` | `cheongryu_raid` / `route_commitment` | `sect_raid`, `faction_negotiation`, `sect_courtyard` | 부상자 대피 fallback 이후 route 선택을 미룬 대가와 재합류 hook을 다룬다. | 중간 |
| `wuxia_baekdo_medicine_debt` | `route_commitment` | `faction_negotiation`, `sect_courtyard` | direct/deferred 정파 route starter를 받아 백도맹 약상자와 청류문 재건 채무를 첫 정파 opener로 고정한다. | 구현 완료 |
| `wuxia_black_heaven_escape_price` | `route_commitment` | `faction_negotiation`, `sect_courtyard`, `market_street` | direct/deferred 사파 route starter를 받아 흑천련 탈출로와 도월의 표식이 남기는 값을 첫 사파 opener로 고정한다. | 구현 완료 |
| `wuxia_heavenly_archive_previous_outsiders` | `route_commitment` / `cheonggi_return` | `cheonggi_record`, `faction_negotiation`, `sect_courtyard` | direct/deferred 천기·귀환 route starter를 받아 천기각 이전 이방인 기록과 세계 균열 단서를 첫 천기 opener로 고정한다. | 구현 완료 |
| `wuxia_wounded_shelter_dawn_offers` | `route_commitment` | `sect_courtyard`, `faction_negotiation`, `cheonggi_record` | `stabilize_wounded_until_dawn` branch가 남긴 deferred flags를 받아 부상자 피난처 새벽 제안으로 route pressure를 다시 연다. | 구현 완료 |
| `wuxia_mumyeong_first_sighting` | `midgame_rival` | `sect_courtyard`, `market_street`, `training_chore` | route opener 이후 흑사방 쪽에서 청류문식 흐름을 훔쳐 쓰는 그림자를 처음 목격하고, 무명/서하린/카피 무공 thread를 연다. | 구현 완료 |
| `wuxia_mumyeong_first_confrontation` | `midgame_rival` / `rival_confrontation` | `sect_courtyard`, `training_chore`, `faction_negotiation` | 첫 목격 이후 무명을 라이벌로 확정하는 첫 대치다. 이기는 전투가 아니라 버티기/관찰/카피 무공 분석/서하린 침묵 확인으로 구현한다. | 구현 완료 |
| `wuxia_mumyeong_copy_style_reveal` | `midgame_rival` / `copy_style_analysis` | `sect_courtyard`, `cheonggi_record`, `training_chore` | 첫 대치 이후 무명이 덧씌운 카피 무공 계열의 윤곽과 결함을 청류안/천기록 대비로 읽는다. | 구현 완료 |
| `wuxia_mumyeong_reads_orthodox_style` | `midgame_rival` / `orthodox_style_trace` | `sect_courtyard`, `cheonggi_record`, `training_chore` | 카피 무공 공개 뒤 무명이 과거에 읽어낸 정파식 제압술 흔적을 현악문/복호금쇄수 단서로 연결한다. | 구현 완료 |
| `wuxia_mumyeong_midgame_reunion` | `midgame_rival` / `rival_reunion` | `sect_courtyard`, `cheonggi_record`, `training_chore` | 첫 대치/카피 무공/정파식 통제 무공 단서 뒤, 무명과 다시 마주쳐 서하린의 침묵과 과거 상처를 라이벌/거울 관계로 연결한다. | 구현 완료 |
| `wuxia_boss_first_appearance` | `midgame_boss` / `boss_wall_pressure` | `sect_courtyard`, `faction_negotiation`, `training_chore` | 무명 중반 재회 뒤 흑사방 보스가 처음 직접 압박으로 등장하고, 주인공이 흐름을 읽어도 몸이 따라가지 못하는 벽을 각인한다. | 구현 완료 |
| `wuxia_mumyeong_request_for_aid` | `midgame_rival` / `failed_aid_records` | `sect_courtyard`, `faction_negotiation`, `cheonggi_record` | 보스 첫 등장 뒤 무명이 청류문을 살리려 했으나 정파에게 거절당한 기록/소문을 추적해 보스 논리가 먹힌 이유를 준비한다. | 구현 완료 |
| `wuxia_mumyeong_awakening` | `midgame_rival` / `anger_copy_bloom` | `sect_courtyard`, `cheonggi_record`, `faction_negotiation` | 도움 요청 실패와 정파 무공 흔적 이후, 무명의 카피가 재능이 아니라 분노와 상처에서 극단적으로 개화한 순간을 추적한다. | 다음 runtime 후보 |

상세 카드는 `docs/content/encounter_db/wuxia_jianghu_pack.md`에 둔다.

## 루트와 결말 후보

### 정파 루트

주제는 “질서 안에서 개혁할 것인가”다. 주인공은 백도맹과 손잡고 청류문을 재건한다. 하지만 정파는 깨끗하기만 한 질서가 아니며, 명문 문파들은 천기록을 견제하거나 빼앗으려 한다.

결말 후보:

- `cheongryu_divine_sword`: 청류문을 신흥 명문으로 재건하고 천기록을 제한적으로 공개한다.
- `white_path_prison`: 질서를 지키는 데 성공하지만 천기록을 상층부에 빼앗기고, 질서가 항상 정의는 아님을 깨닫는다.

### 사파 루트

주제는 “살아남기 위해 어디까지 내려갈 것인가”다. 청류문이 무너진 뒤 흑천련과 거래하고, 생존/거래/암투/잠입 중심으로 진행한다.

결말 후보:

- `black_night_gentleman`: 잔혹한 세력을 제거하고 밑바닥 사람들을 보호하는 새 사파 질서를 만든다.
- `debtor_of_all_under_heaven`: 살아남기 위해 너무 많은 거래를 하여 적은 줄었지만 빚과 대가가 늘어난다.

### 천기/귀환 루트

주제는 “이 세계는 정말 현실인가”다. 천기각과 함께 천기록의 비밀, 이전 이방인 기록, 세계 균열, 귀환 가능성을 조사한다.

결말 후보:

- `returnee`: 원래 세계 사무실 책상 앞에서 깨어나지만 손의 굳은살과 서랍 속 천기록이 남아 있다.
- `murim_outsider`: 돌아갈 방법을 찾지만 남기로 선택하고, 자신이 이 세계의 새로운 기록자가 된다.

## Runtime promotion notes

첫 runtime 승격은 storypack 선택 UI나 새 성장 schema를 바로 열지 않는다.

현재 상태와 권장 순서:

1. `wuxia_commute_rift_arrival` — preview 구현 완료
   - updated story의 전제를 가장 직접적으로 고정한다.
   - 기존 encounter schema의 title/body/choices/outcome만으로 구현했다.
   - separate preview bundle metadata와 `default_location: wuxia_commute_rift` 경계를 검증했다.
2. `wuxia_heuksa_bang_first_fight` — preview 구현 완료
   - 숫자 HP 전투가 아니라 결과 hook 중심의 튜토리얼성 첫 전투로 구현했다.
   - 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` key는 변경하지 않았다.
3. `wuxia_cheonggi_record_first_fragment` — preview 구현 완료
   - 현재 구현은 첫 난투 뒤 시장에서 뜨는 foreshadow version이다.
   - 새 reward/ability schema 없이 `cheonggi_record_awakened`, `first_fragment_seen`, fragment thread flags/clues/log/presentation만 남긴다.
4. `wuxia_seo_harin_rescue` — preview 구현 완료
   - first fight/first fragment 이후 서하린 구조, 외지인 조사, 청류문 보호·감시 bridge를 연다.
   - `cheongryu_outer_courtyard` destination과 `seo_harin_rescue_resolved`/`taken_under_watch` 공통 hook을 보장한다.
5. `wuxia_cheongryu_apprentice_entry` — preview runtime 구현 완료
   - rescue hook 이후 청류문 수습생/잡역/채무/서고 bridge를 연다.
   - `accept_three_month_trial` fallback과 `cheongryu_apprentice_entry_resolved`/`cheongryu_trial_started` 공통 hook을 남긴다.
6. `wuxia_cheongryu_chore_sparring` — preview runtime 구현 완료
   - apprentice/first fragment 뒤 장작 마당 잡일 중 첫 몸싸움식 겨루기 bridge를 연다.
   - 새 combat schema 없이 `cheongryu_chore_sparring_resolved`, 균형·호흡·발디딤 clues, 서하린 mentor thread를 남긴다.
7. `wuxia_cheongryu_raid_route_split` — preview runtime 구현 완료
   - rescue/apprentice/chore와 first-fragment 공통 hook 뒤에 열린다.
   - 정파/사파/천기·귀환 route pressure를 flags/clues/log로만 남기고, faction route graph나 multi-ending schema는 열지 않았다.
8. `wuxia_cheongryu_raid_wounded_fallback` — preview runtime 구현 완료
   - raid split의 `evacuate_the_wounded_first` fallback 이후에만 열린다.
   - route opener 전 공통 재합류를 만들고, route graph/faction schema 없이 `deferred_route_reopened`와 same route starter flags로 이어 붙였다.
9. `wuxia_baekdo_medicine_debt` — preview runtime 구현 완료
   - 첫 route opener는 정파/백도맹 약상자 채무 축으로 landing했다.
   - direct raid branch의 `baekdo_alliance_debt`와 deferred wounded branch의 `baekdo_medicine_debt`는 eligibility 필수가 아니라 flavor이며, 공통 start condition은 `righteous_route_started` + `cheongryu_rebuild_thread`로 둔다.
   - `accept_medicine_with_written_debt`, `ask_terms_before_opening_gate`, `send_supplies_to_wounded_first`, `compare_banner_to_record_margin` stable choices와 `righteous_route_opened` hook을 남긴다.
   - route graph/faction reputation schema 없이 기존 flags/clues/log/presentation만 사용한다.
10. `wuxia_black_heaven_escape_price` — preview runtime 구현 완료
   - 첫 사파 opener는 흑천련 탈출로와 도월의 표식이 남기는 값/장부를 다룬다.
   - direct raid branch의 `black_heaven_deal_marked`와 deferred wounded branch의 `black_heaven_escape_marker`는 eligibility 필수가 아니라 flavor이며, 공통 start condition은 `sapa_route_started` + `dowol_debt`로 둔다.
   - `accept_dowol_marker_for_safehouse`, `ask_who_collects_the_price`, `keep_cheongryu_names_off_ledger`, `map_exit_before_following_dowol` stable choices와 `sapa_route_opened` hook을 남긴다.
   - route graph/faction reputation/debt ledger/relation schema 없이 기존 flags/clues/log/presentation만 사용한다.
11. `wuxia_heavenly_archive_previous_outsiders` — preview runtime 구현 완료
   - `route_opener_followup_after_black_heaven` handoff에서 천기·귀환 opener로 결정했고, preview runtime으로 구현했다.
   - direct raid branch의 `heavenly_archive_contact`와 deferred wounded branch의 `heavenly_archive_triage_map_seen`는 eligibility 필수가 아니라 flavor이며, 공통 start condition은 `cheonggi_return_route_started` + `cheonggi_record_targeted`로 둔다.
   - `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory` stable choices와 `cheonggi_return_route_opened` hook을 남긴다.
   - 천기록 정체 reveal, return system, route graph/faction reputation/debt ledger/relation schema, reward/ability schema 없이 기존 flags/clues/log/presentation만 사용한다.
12. `wuxia_wounded_shelter_dawn_offers` — preview runtime 구현 완료
   - `route_opener_followup_after_heavenly_archive` handoff에서 deferred-offer card로 결정했고, preview runtime으로 구현했다.
   - start condition은 `cheongryu_raid_wounded_fallback_resolved`, `route_commitment_deferred`, `deferred_route_reopened`, `wounded_shelter_stabilized`이며, `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 flavor hook으로만 둔다.
   - stable choice id는 `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha`다.
   - route graph/faction reputation/debt ledger/relation schema, return system, 천외편린 3택 reward/ability schema는 열지 않는다.

13. `wuxia_mumyeong_first_sighting` — preview runtime 구현 완료
   - `route_midgame_continuity_after_wounded_shelter` handoff에서 common midgame bridge로 결정했고, preview runtime으로 구현했다.
   - 세 route opener outcome에 공통 `route_opener_resolved` flag를 추가하고, start condition은 `route_opener_resolved`, `cheongryu_raid_survived`, `cheongryu_trial_started`, `first_fragment_seen`로 둔다.
   - stable choice id는 `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form`다.
   - `mumyeong_first_sighting_resolved`/`midgame_continuity_started` common hook과 무명 존재/카피 무공/서하린 침묵 clues를 남긴다.
   - 무명 첫 대치/중반 재회, boss first appearance, combat schema, route graph/faction reputation/debt/relation schema, return system, 천외편린 3택 reward/ability schema는 열지 않는다.

14. `wuxia_mumyeong_first_confrontation` — preview runtime 구현 완료
   - `wuxia_mumyeong_first_confrontation_after_sighting` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `cheongryu_raid_survived`, `first_fragment_seen`로 둔다.
   - stable choice id는 `meet_mumyeong_head_on`, `endure_until_copy_flow_breaks`, `watch_seo_harin_hold_back`, `read_mumyeongs_copied_form`, `do_not_provoke_mumyeong`다.
   - 첫 대치는 승리/패배 판정이 아니라 버티기/관찰/분석 encounter로 구현한다.
   - combat resolver/schema, HP 숫자전, boss first appearance, route graph/faction reputation/debt/relation schema, return system, 천외편린 3택 reward/ability schema는 열지 않는다.

15. `wuxia_mumyeong_copy_style_reveal` — preview runtime 구현 완료
   - `wuxia_mumyeong_followup_after_first_confrontation` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `midgame_continuity_started`이며, `copied_flow_weakness_noted`/`cheonggi_copy_contrast_noted` 등은 flavor hook으로만 둔다.
   - stable choice id는 `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder`다.
   - common hook은 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - seed 기반 random copy-style system/table, combat resolver/schema, boss first appearance, 무명 중반 재회, route graph/faction reputation/debt/relation schema, return system, 천외편린 3택 reward/ability schema는 열지 않는다.

16. `wuxia_mumyeong_reads_orthodox_style` — preview runtime 구현 완료
   - `wuxia_mumyeong_followup_after_copy_style_reveal` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `midgame_continuity_started`, `first_fragment_seen`이며, `copied_form_family_seen`/`breath_mismatch_marks_copy` 등은 flavor hook으로만 둔다.
   - stable choice id는 `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation`다.
   - common hook은 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - 현악문/복호금쇄수는 최신 확정명으로 쓰되, 무명 이탈의 진실 전체나 서하린에게 진실 전달은 열지 않는다.

17. `wuxia_mumyeong_midgame_reunion` — preview runtime 구현 완료
   - `wuxia_mumyeong_followup_after_orthodox_style_trace` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`이며, `hyeonakmun_trace_suspected`/`bokho_geumsaesu_name_recorded`/`departure_truth_still_incomplete` 등은 flavor hook으로만 둔다.
   - stable choice id는 `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer`다.
   - common hook은 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - 무명 이탈 진실 전체 reveal, 보스 첫 등장, 청류문 습격 full flashback, 서하린에게 진실 전달, 구원 확정, reward/ability/combat/route graph schema는 열지 않는다.

18. `wuxia_boss_first_appearance` — preview runtime 구현 완료
   - `wuxia_mumyeong_followup_after_midgame_reunion` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `cheongryu_raid_survived`, `midgame_continuity_started`이며, `boss_used_mumyeongs_wound`/`hyeonakmun_trace_shared_without_accusation` 등은 flavor hook으로만 둔다.
   - stable choice id는 `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step`다.
   - common hook은 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - 이 장면은 보스 전투나 최종 결산이 아니라 압도감, 조직력, 약점 읽기, 무명이 따르는 이유를 flags/clues/log/presentation으로 각인한다.
   - 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 서하린에게 진실 전달, 구원 확정, reward/ability/combat/route graph schema는 열지 않는다.

19. `wuxia_mumyeong_request_for_aid` — preview runtime 구현 완료
   - `wuxia_boss_followup_after_first_appearance` handoff에서 다음 runtime 후보로 결정했고, preview runtime으로 구현했다.
   - start condition은 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `mumyeong_mirror_thread_deepened`, `orthodox_style_trace_recorded`, `midgame_continuity_started`이며, `boss_used_mumyeongs_wound`/`hyeonakmun_trace_shared_without_accusation`/`mumyeong_follows_power_that_saw_his_wound` 등은 flavor hook으로만 둔다.
   - stable choice id는 `search_the_rejected_aid_letters`, `follow_old_inn_rumors_about_mumyeong`, `ask_seo_harin_what_help_never_came`, `keep_the_failed_aid_record_unshown`다.
   - common hook은 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `search_the_rejected_aid_letters`는 `rejected_aid_letter_fragment` item을 추가하고 `mumyeong_tried_to_save_qingliu`/`orthodox_refusal_broke_mumyeong` clue를 남긴다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 서하린에게 진실 전달, 보스 최종 결산, reward/ability/combat/route graph schema는 열지 않는다.

20. `wuxia_mumyeong_awakening` — 다음 runtime 후보
   - `wuxia_mumyeong_followup_after_failed_aid` handoff에서 다음 runtime 후보로 결정했다.
   - start condition은 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `midgame_continuity_started`이며, `mumyeong_awakening_resolved`로 반복을 막는다.
   - stable choice id 후보는 `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation`다.
   - common hook 후보는 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_departure_truth_summary`는 후반 truth/서하린 진실 전달/구원 조건 범위라 보류했고, `wuxia_qingliu_attack_after_war`는 full flashback source라 보류했으며, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`는 후반/final consequence 범위라 보류했다.
   - 이번 handoff는 runtime YAML/Rust/Web generated bundle을 변경하지 않는다. 다음 구현 slice에서만 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 추가한다.

주의:

- 무협 콘텐츠는 office legacy content와 섞지 않고 `wuxia_jianghu_pack` bundle 경로로 연다. terminal content scene과 Web default는 이 bundle을 이구학지 main run으로 사용하되, terminal fixture metadata는 `storypack_preview`를 유지한다.
- legacy office save/localStorage key의 `escape-office` 명칭은 compatibility 이슈로 남겨 두고, 이름 변경은 별도 cleanup/migration으로 다룬다.
- 실제 회사명, 실제 통근 경로, 실제 역/건물/사원증 정보는 쓰지 않는다.
- 혈월교/흑천련/백도맹/청류문은 fiction 세력이다. 현실 종교·민족·정치 소재처럼 보이는 세부 묘사는 피한다.
