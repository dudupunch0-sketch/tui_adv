# 보상 파이프라인 웨이브 1 플랜 — 획득·도감 (step 1)

- 작성: fable / 2026-07-17 19:03
- 구현: codex
- 기준 커밋: origin/main (Event/Stage 44/44 완료 이후)
- Notion 근거: 19. 보상 획득 매핑 DB(정본), 17. 스킬 DB, 18. 칭호 DB, 14. 아이템 DB, 15. 기연 DB, 09. 사건 카드 DB(청류문 수련기 7카드), 보상 시스템 설계 원칙 v0.1

## 0. 확정된 방향 (기획자 인터뷰 2026-07-17)

1. 구현 순서: **A(보상 파이프라인) → B(사이드·랜덤 스케줄러) → C(다회차 메타)**. 본 플랜은 A.
2. 효과 범위: **획득 + 도감 + 연출까지만**. 스킬·칭호·기연 신규분의 실효과(판정 보너스 등)는 수치 확정 후 별도 슬라이스. 설계 원칙 "보상 카드는 숫자 대신 효과 방향 기술"과 일치.
3. 콘텐츠 갭: 19번 매핑에 획득 경로가 없는 보상(스킬 5, 칭호 6, 아이템 5, 기연 3)과 본문 없는 랜덤 카드 23장은 **이번 웨이브 범위 제외**. 매핑에 등장하는 보상만 정의·구현한다.
4. 문서 간 충돌(후일담 슬롯 수, 랜덤 재사용 규칙 등)은 개별 재검토 예정 — 전부 웨이브 B 영역이므로 본 웨이브를 막지 않는다.

## 1. 핵심 발견 (플랜 전제)

- 19번 매핑 DB의 30건은 **청류문 초반 7개 사건**에 붙는다:
  `first_night_shelter`(4) / `first_breathing_lesson`(4) / `training_first_failure`(4) / `medicine_errand`(4) / `raid_omen`(6) / `gate_patrol_first_trouble`(4) / `seoharin_hides_training_injury`(4)
- 이 7개 사건은 **런타임 encounters.yaml에 존재하지 않는다.** 09 사건 카드 DB에 초안(선택지 4개·결과·선행 조건 완비) 상태로 있으며, 정식 ID는 `wuxia_cheongryu_first_night_shelter` 형식(서하린 카드만 `wuxia_seoharin_hides_training_injury`)이다. **따라서 웨이브 1은 "보상 엔진 + 신규 사건 7개 authoring"이 한 몸이다.**
- ID 정본 규칙: **09 DB의 전체 ID가 정본**. 19번 매핑 DB의 축약형(`first_night_shelter` 등)은 구현 시 전체 ID로 해석한다. Notion 측 축약형 정규화는 후속 역동기화 항목.
- 배치 지점: `wuxia_cheongryu_apprentice_entry`(입문) 이후 ~ `wuxia_cheongryu_raid_route_split`(습격 분기) 이전의 수련기 구간. 기존 `wuxia_cheongryu_chore_sparring`과 병존한다.

## 2. 목표 / 비범위

**목표**
- GameCore에 스킬·칭호·관계(인물/단체 호감도) 상태와 보상 지급 파이프라인 추가.
- 신규 사건 7개를 Event/Stage 형식(ordered content_stream)으로 authoring — 커버리지 44/44 → **51/51**.
- 19번 매핑 30건을 사건 outcome에 반영 (획득 시점 4종 처리 포함).
- 웹: 드로어 스킬·칭호 섹션, 획득 연출, 지연 공개 마스킹.

**비범위**
- 스킬·칭호·기연 신규분의 실효과 수치 (별도 슬라이스).
- 랜덤 등장 확률·슬롯 스케줄러·태그 중복 회피 (웨이브 B). `gate_patrol_first_trouble`의 "낮은 확률" 트리거도 B에서 확률화 — 이번엔 결정론 조건으로 대체.
- 기억 파편·계정 도감·행상인 (웨이브 C). 이번 도감은 **회차(run) 내 드로어 노출**까지만.
- 매핑 미커버 보상 21종, 랜덤 카드 23장, 새 아트 asset, office pack.

## 3. 워크패키지

### WP-R1 — 코어 상태·콘텐츠 모델 확장

- `GameState`에 additive 필드 (전부 `#[serde(default)]`):
  - `skills: Vec<String>` / `titles: Vec<String>`
  - `relationships: BTreeMap<String, i32>` — 키는 매핑 DB 표기 그대로 `relationship_person_seoharin_affection`, `relationship_faction_cheongryu_affection`. 델타 기본 +1(내부 수치, 임시), UI 비노출.
- `OutcomeDef`에 additive 필드: `add_skills: Vec<String>`, `add_titles: Vec<String>`, `relationship_deltas: BTreeMap<String, i32>`.
- 콘텐츠 정의: `skills.yaml`, `titles.yaml` 신설 — 필드: id, name, concept(도감 문구), rarity(보통/희귀/전설), category(메인/사이드/랜덤/히든), reveal_immediate(bool). ContentIndex 로딩·중복 ID 검증은 기존 insights 패턴을 따른다.
- 중복 획득: 이미 보유한 스킬/칭호/기연 재지급 시 **무시 + 로그 1줄**(기억 파편 전환은 웨이브 C). 설계 문서의 "중복 시 기억 파편" 규칙은 C에서 완성됨을 주석으로 명시.
- 판정(roll)·기존 효과 경로에 신규 상태가 개입하지 않음을 보장 (효과 없음 = 해시 계약 자동 유지).

### WP-R2 — 보상 콘텐츠 정의 (매핑 등장분만)

19번 매핑 30건에 등장하는 보상만 yaml에 정의한다:
- 스킬 7종: match_the_pulse, fallen_leaf_flow_step, record_the_gap, turning_blade, guard_the_threshold, two_steps_back, cut_the_presence
- 칭호 5종: not_yet_disciple, guest_of_cheongryu, keeper_of_returning_name, badge_bearer, footprints_of_two_paths
- 기연 5종: first_current_breath, recording_defeat, measure_fidelity, sort_like_documents, read_the_wrist — 보너스 수치 미정이므로 **bonus 0 / 능력치 없음**으로 정의하고 도감 문구만. 기존 구현 3종(+1)은 무변경.
- 위 목록은 이 플랜 작성 시점 조회 결과이며, 구현 시 **19번 DB 행 조회 결과가 정본** — 불일치 시 DB를 따르고 보고서에 기록.
- 아이템 7종: modern_first_aid_pouch, empty_medicine_ledger, cracked_whistle, red_thread_fragment, wet_gate_register, life_talisman, seoharin_handkerchief — 전부 `usable: false`(14번 DB 사용 가능 전원 NO), 사용 효과 없음.
- 이름·설명·등급은 해당 DB 행 텍스트 그대로 (verbatim).

### WP-R3 — 신규 사건 7개 authoring

- 09 DB 카드 초안(사건명·선택지 4개·결과·선행 조건)을 본문으로 확장, Event/Stage 형식: StoryStage(일러스트 1개, placeholder true + 한국어 장면 묘사 alt) → ChoiceStage(4지) → per-choice ResultStage.
- 선행 조건은 카드 그대로 플래그화: shelter(입문 직후) → breathing(shelter 해결) → first_failure(breathing+chore_sparring) → medicine/wrist/raid_omen/gate_patrol 카드 조건 준수. 각 사건은 `*_resolved` 플래그로 1회 소비.
- **encounters.yaml 배치 순서 주의**: first-match 선택이므로 raid_route_split보다 앞에 두되, **메인 도달성 보존** — 신규 사건을 하나도 보지 않아도(전부 스킵 불가능하므로: 조건 충족 시 등장하는 구조상 순차 소비됨) raid_route_split 도달 가능해야 한다. shelter·breathing·first_failure는 카드상 순차 필수 흐름이므로 메인 사이 삽입으로 처리하고, medicine/wrist/gate_patrol은 선택적(조건 미충족 시 건너뜀) 구조를 유지한다.
- `gate_patrol_first_trouble`: "낮은 확률" 대신 결정론 조건(수습생 상태 + raid_route_split 이전 + 전용 플래그)으로 트리거. 웨이브 B에서 확률화 예정을 주석으로 남긴다.
- 획득 시점 4종 처리:
  - **즉시(19건)**: 해당 choice outcome에 add_* 직접 기재.
  - **판정 성공(3건)**: 해당 choice에 check 부여(능력치는 카드 맥락에 맞게 codex 재량, 난이도 보통), success branch outcome에만 지급. content_stream branch 필터는 기존 엔진 그대로.
  - **후속 회수(4건)**: `reward_pending_<보상ID>` 플래그만 세팅. 실제 지급은 후속 사건 웨이브에서. 드로어에는 노출하지 않는다(도감 "미회수" 표시는 C의 계정 도감에서).
  - **히든 발견(2건)**: 매핑의 연계 조건을 플래그 조합으로 표현 (예: raid_omen 두 조사 모두 수행 시 `footprints_of_two_paths`).
- 획득 연출: 19번 DB "획득 연출" 텍스트를 ResultStage의 결과 블록(또는 로그)에 verbatim 반영.

### WP-R4 — 웹 렌더러

- 드로어: 기존 기연/아이템 옆에 **스킬·칭호 섹션** 추가 — 수묵 토큰 내에서 등급(보통/희귀/전설) 시각 위계, 효과 방향 텍스트만 노출(숫자 금지).
- 획득 연출: 보상 획득 시 기존 획득 비트/토스트 패턴 재사용해 종류 아이콘+이름 노출. 전설 등급은 연출 강조 1단계.
- 지연 공개: `reveal_immediate: false`(퀘스트 아이템 3종 + 빈칸 기연류)는 정체 마스킹(예: "정체를 알 수 없는 물건")으로 표기, 공개 조건은 후속.
- 관계 수치는 어디에도 숫자 노출 금지.

### WP-R5 — 검증 게이트·가드 테스트

- 신규 가드 (`reward_pipeline_wave1.rs`):
  - 매핑 30건 커버리지: 각 매핑 ID가 정확히 1개 choice outcome(또는 pending 플래그)에 대응.
  - 기존 44 사건 byte-parity (choices/outcomes 무변경).
  - 메인 도달성: 시드 고정 재생으로 raid_route_split·이후 메인 체인 도달.
  - 중복 지급 무시, 관계 델타 누적, 판정 성공 매핑의 branch 배타성.
  - 신규 51/51 Event 커버리지 (기존 웨이브 가드 갱신).
- 기존 전체 게이트: cargo workspace / pytest / export --check(양 번들) / vitest+아트 게이트 / tsc / build / wasm-pack / 5-viewport QA.
- 수동 QA(리뷰 시 fable): 7개 사건 실화면 플레이, 드로어 스킬·칭호 표시, 지연 공개 마스킹, 히든 양립 획득, save/reload 중 신규 필드 보존.

## 4. 불변식

1. 기존 44개 사건의 choice ID·outcome·route graph 무변경 (byte-parity).
2. roll 해시 계약 `"{seed}:{turn}:{ability}:{difficulty}"` 무변경. 신규 상태는 판정에 무영향.
3. serde 스키마 additive only — 구세이브 로드 시 신규 필드 default.
4. office pack 무변경. 새 action/storage 스키마 금지 (드로어 확장은 기존 렌더 계약 내).
5. Notion DB 텍스트(이름·설명·연출) verbatim — 문장 경계 분리만 허용.

## 5. 리스크·유보

- 19번 DB 30행 전원 "기획자 검수 필요" 상태 — 구현은 초안 기준으로 진행하고, 기획자 검수는 fable 리뷰·실화면 QA와 병행. 검수 중 변경분은 후속 PR.
- 기연 신규분 bonus 0은 임시값 — 수치 확정 슬라이스에서 갱신 (도감 문구에 수치 미표기라 사용자 노출 없음).
- raid_omen 매핑 6건 중 카드 선택지 4개와의 대응(히든 양립 포함)이 1:1이 아닐 수 있음 — codex는 매핑 DB 행 기준으로 outcome을 구성하고, 카드와 불일치 발견 시 보고서에 기록.
- 관계 델타 기본 +1은 내부 임시 수치 — 밸런스 확정 전 UI 비노출로 안전.

## 6. 산출물

- 코어: state.rs/content.rs/turn.rs 확장, skills.yaml/titles.yaml, encounters.yaml(+7), items.yaml(+7), insights.yaml(+6), 가드 테스트.
- 번들 재생성 (양쪽), wasm 재빌드.
- 웹: 드로어 섹션·획득 연출·마스킹.
- 구현 보고서 + idea_box/notion_sources.yml pending reverse-sync 기록 (17/18/19 DB 해당 행 "구현됨" 전환은 fable 리뷰 후 live sync).
