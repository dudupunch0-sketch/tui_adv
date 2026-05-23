# 반복 등장인물 후보

Status: candidate

이 문서는 `차원격리팩` 첫 slice에서 사용할 반복 NPC 후보 3명을 정리한다.

이들은 아직 runtime NPC가 아니다. 현재 목적은 storypack과 encounter situation card를 안정적으로 작성하기 위한 캐릭터 DB 후보다.

## 요약

| id | 이름 | slot | 핵심 기능 | stat 경향 |
|---|---|---|---|---|
| `park_doyoon` | 박도윤 | `infra_interpreter` | 사내망/서버 로그/권한 해석 | 이성 높음, 사회/충동 낮음 |
| `yoon_seoyeon` | 윤서연 | `hr_identity_keeper` | 조직도/근태/존재 기록 해석 | 사회/자아 높음, 신체 낮음 |
| `choi_minjae` | 최민재 | `security_witness` | CCTV/출입통제/건물 봉쇄 해석 | 감각/자아/신체 높음, 사회 낮음 |

## `park_doyoon`

```yaml
id: park_doyoon
status: candidate
source_refs:
  - idea_box/done/2026-05-22-general-corporate-storypacks.md
  - idea_box/done/2026-05-22-semiconductor-sw-storypacks.md
name: 박도윤
department: IT 인프라팀
story_function:
  - infra_interpreter
  - unreliable_helper
one_line: 냉소적이지만 서버 로그를 끝까지 확인하는 IT 인프라 담당자.
core_stats:
  sense: 11
  social: 7
  reason: 16
  self: 10
  impulse: 6
  body: 10
stat_total: 60
tags:
  body: [평균 체격, 오래 앉아 있던 몸]
  appearance: [눈 밑이 어두움, 낮은 목소리, 무표정]
  outfit: [회사 점퍼, 사원증, 낡은 노트북 가방]
  authority: [서버 로그 접근 가능, 사내망 일부 권한, 계정 잠금 요청 가능]
  state: [수면 부족, 냉소적, 책임감 있음]
derived_impressions:
  trust: 보통
  suspicious: 높음
  expertise: 매우 높음
encounter_uses:
  enables:
    - server_log_interpretation
    - channel_route_trace_hint
    - permission_error_reading
  complicates:
    - social_reassurance
    - urgent_physical_escape
    - trusting_other_survivors
secrets:
  public_safe_summary: 격리 전부터 일부 로그가 하루 뒤 시각으로 찍히는 것을 알고 있었다.
first_appearance_candidate: 사내 메신저에서 가장 먼저 로그 파일명을 공유한다.
```

### 사용법

박도윤은 “시스템이 이상하다”는 사실을 가장 빨리 설명할 수 있는 인물이다. 대신 사람을 설득하거나 안심시키는 데 약하다. 그가 주는 정보는 기술적으로 유용하지만, 플레이어가 어떤 사람을 믿어야 하는지에는 도움이 되지 않을 수 있다.

적합한 카드:

- `isolation_channel_mismatched_floor`
- `server_log_other_branch`
- 권한 오류, 지연 패킷, 빌드 로그, 사내망 검색 계열 카드

## `yoon_seoyeon`

```yaml
id: yoon_seoyeon
status: candidate
source_refs:
  - idea_box/done/2026-05-22-general-corporate-storypacks.md
  - idea_box/done/2026-05-22-semiconductor-sw-storypacks.md
name: 윤서연
department: 인사팀
story_function:
  - hr_identity_keeper
  - reluctant_baseline
one_line: 사라진 사람을 시스템상 없는 사람으로 처리하라는 지시를 받은 인사 담당자.
core_stats:
  sense: 10
  social: 15
  reason: 11
  self: 13
  impulse: 6
  body: 5
stat_total: 60
tags:
  body: [마른 체형, 오래 앉아 있던 몸, 피곤한 몸]
  appearance: [단정함, 얇은 미소, 눈 밑이 어두움]
  outfit: [베이지 블라우스, 검은 슬랙스, 사원증]
  authority: [조직도 열람 가능, 근태 기록 접근 가능, 사원 조회 가능]
  state: [원칙주의, 인간적 갈등, 방어적 말투]
derived_impressions:
  trust: 높음
  suspicious: 보통
  expertise: 높음
encounter_uses:
  enables:
    - organization_chart_identity_check
    - missing_employee_record_trace
    - social_cover_story
  complicates:
    - fast_decision_under_pressure
    - physical_escape
    - revealing_private_hr_records
secrets:
  public_safe_summary: 조회되지 않는 직원 몇 명의 근태 기록이 계속 자동 승인되고 있다는 사실을 숨기고 있다.
first_appearance_candidate: 조직도 조회 화면을 공유하며 “이 사람은 애초에 입사한 적이 없는 것으로 나와요”라고 말한다.
```

### 사용법

윤서연은 사람을 기록으로 다루는 시스템과 인간적 기억 사이의 긴장을 담당한다. 그녀는 social/self가 높아 쉽게 무너지지 않지만, 조직의 절차를 완전히 무시하지 못한다.

적합한 카드:

- `org_chart_missing_employee`
- 자동 근태 알림, 직원 조회 실패, 인사기록 모순 계열 카드
- NPC를 기록에서 복원하거나 삭제하는 선택지

## `choi_minjae`

```yaml
id: choi_minjae
status: candidate
source_refs:
  - idea_box/done/2026-05-22-general-corporate-storypacks.md
  - idea_box/done/2026-05-22-semiconductor-sw-storypacks.md
name: 최민재
department: 보안팀 / 방재실
story_function:
  - security_witness
  - physical_anchor
one_line: CCTV보다 자기 눈을 더 믿지만, CCTV가 가끔 먼저 보는 것을 부정하지 못하는 보안 담당자.
core_stats:
  sense: 14
  social: 6
  reason: 9
  self: 12
  impulse: 8
  body: 11
stat_total: 60
tags:
  body: [단단한 체격, 오래 서 있던 몸, 피곤한 몸]
  appearance: [무표정, 낮은 목소리, 시선을 오래 둠]
  outfit: [보안팀 조끼, 검은 근무화, 무전기, 사원증]
  authority: [CCTV 열람 가능, 출입기록 접근 가능, 방재실 연락 가능]
  state: [말수 적음, 현장 감각 좋음, 경계심 높음]
derived_impressions:
  trust: 보통
  suspicious: 낮음
  expertise: 높음
encounter_uses:
  enables:
    - delayed_cctv_detection
    - physical_route_warning
    - lockdown_status_reading
  complicates:
    - gentle_negotiation
    - explaining_abstract_rules
    - trusting_screen_records
secrets:
  public_safe_summary: CCTV가 한 박자 늦는 것이 아니라, 가끔 플레이어보다 먼저 움직인다는 것을 이미 봤다.
first_appearance_candidate: 보안실 모니터 앞에서 플레이어에게 “방금 거기 지나가지 않았습니까?”라고 묻는다.
```

### 사용법

최민재는 공간과 물리적 위험을 현실감 있게 붙잡는 역할이다. 그는 초자연 설명보다 현장 경험을 믿지만, CCTV와 출입기록의 모순을 몸으로 감지한다.

적합한 카드:

- `delayed_cctv_next_action`
- 출입 게이트, 봉쇄 알림, 비상계단/복도 위험 계열 카드
- 동행 시 물리 피해 완화 또는 위험 선택 경고 후보

## 관계 상태 기본값 후보

```yaml
relation_defaults:
  park_doyoon:
    trust: 0
    suspicion: 1
    debt: 0
    contamination: 0
    presence: remote_only
  yoon_seoyeon:
    trust: 1
    suspicion: 0
    debt: 0
    contamination: 0
    presence: remote_only
  choi_minjae:
    trust: 0
    suspicion: 0
    debt: 0
    contamination: 0
    presence: unknown
```

관계값은 runtime 승격 전까지 확정하지 않는다. 이 값들은 인카운터 작성 시 일관성을 잡기 위한 후보일 뿐이다.
