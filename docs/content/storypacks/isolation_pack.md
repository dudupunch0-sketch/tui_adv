# 차원격리팩

Status: candidate

## Record

```yaml
id: isolation_pack
status: candidate
source_refs:
  - idea_box/done/2026-05-22-general-corporate-storypacks.md
  - idea_box/done/2026-05-22-semiconductor-sw-storypacks.md
name: 차원격리팩
one_line: 퇴근 직전 회사 일부 공간이 격리되고, 플레이어는 사내 시스템으로 다른 격리자들과 연결된다.
main_surfaces:
  - messenger
  - intranet
  - cctv
  - meeting_minutes
  - organization_chart
  - build_log
anomaly_types:
  - mismatched_floor
  - delayed_time
  - absent_people
  - identity_erasure
  - worldline_branch
main_phases:
  - opening_absence
  - normal_operations
  - isolation_rules
  - route_commitment
sensitive_topics:
  - real_company_similarity
  - workplace_disaster
reusable_npc_slots:
  - infra_interpreter
  - hr_identity_keeper
  - security_witness
ending_candidates:
  - escape_alone
  - rescue_channel_members
  - close_failed_worldlines
  - accept_success_worldline
  - merge_isolation_zones
main_spine_support: 사라진 사람과 정상 작동하는 회사 시스템이라는 핵심 미스터리를 여러 격리 구역의 증언으로 확장한다.
```

## 핵심 컨셉

플레이어는 혼자 회사에 남은 것처럼 보인다. 하지만 사내 메신저, CCTV, 회의록, 조직도, 서버 로그에는 아직 다른 사람들이 남아 있다.

문제는 그들이 같은 회사에 있는 것 같으면서도 서로 다른 층, 서로 다른 시간, 서로 다른 날씨, 서로 다른 회사 규칙을 보고한다는 점이다.

이 팩의 공포는 “연결되어 있지만 만날 수 없음”이다.

## 톤

초반은 블랙코미디다.

- 비상상황인데 임시 대응방 초대가 온다.
- 사람들이 사라졌는데 근태 시스템은 정상 출근 여부를 묻는다.
- CCTV는 생존자를 보여주지만 한 박자 늦다.
- 회의록은 참석자 없이 작성된다.

중후반은 코스믹 호러다.

- 격리자들의 층수와 시간이 서로 맞지 않는다.
- 누군가를 구하려면 다른 세계선의 기록을 닫아야 한다.
- 사내 시스템이 플레이어의 선택을 “브랜치 병합” 또는 “격리 구역 정리”로 처리한다.
- 탈출은 가능하지만, 누가 함께 현실로 넘어오는지는 불확실하다.

## 메인 story spine 연결

| phase | 이 팩에서의 역할 |
|---|---|
| `opening_absence` | 플레이어는 자신만 남은 줄 알지만 사내망에서 다른 격리자 흔적을 본다. |
| `normal_operations` | 회사 시스템은 격리자들의 신고를 정상 업무 티켓, 회의록, 근태 알림으로 처리한다. |
| `isolation_rules` | 층수/시간/CCTV/조직도 불일치가 반복 규칙임을 알게 된다. |
| `route_commitment` | 나만 탈출, 일부 구출, 실패 세계선 폐쇄, 격리 병합 중 방향이 생긴다. |
| `resolution_pressure` | 누구의 기록을 남기고 지울지, 누구를 데리고 나갈지 선택해야 한다. |

## 주요 surface

### messenger

- `#격리자-임시대응방`
- DM 지연
- 발신자 이름이 한 박자 늦게 바뀜
- 같은 메시지가 서로 다른 층수에서 도착

### cctv

- 보안실 모니터가 플레이어보다 한 박자 늦음
- 다른 격리자의 복도가 보이지만 연결되지 않음
- 방금 전 선택과 다른 선택을 한 플레이어가 녹화됨

### meeting_minutes

- 참석자가 없는 회의록
- 발언자가 플레이어 하나로 기록됨
- 아직 열리지 않은 회의의 결론이 먼저 작성됨

### organization_chart

- 특정 직원이 조회되지 않음
- 누군가는 플레이어를 기억하지만 시스템은 그 사람을 삭제함
- 부서명이 화면을 새로고침할 때마다 바뀜

### build_log / system log

- 같은 사건의 성공/실패 로그가 다른 브랜치에 남음
- 세계선이 branch, merge, cherry-pick 같은 회사 시스템 은유로 표현됨
- 기술 용어는 실제 개발 지식이 아니라 윤리적 선택으로 번역된다.

## 반복 NPC slot

| slot | 후보 인물 | 기능 |
|---|---|---|
| `infra_interpreter` | 박도윤 | 메신저 라우팅, 서버 로그, 권한/브랜치 기록 해석 |
| `hr_identity_keeper` | 윤서연 | 조직도, 근태, 사원 조회, 존재 말소 기록 해석 |
| `security_witness` | 최민재 | CCTV, 출입기록, 방재실, 봉쇄 상태 해석 |

## 후보 인카운터 카드

| id | phase | surface | 핵심 상황 | 승격 우선순위 |
|---|---|---|---|---|
| `isolation_channel_mismatched_floor` | `opening_absence` | messenger | 같은 대응방 사람들이 서로 다른 층수와 날씨를 보고한다. | 높음 |
| `org_chart_missing_employee` | `normal_operations` | organization_chart | 윤서연이 기억하는 직원이 조직도에서는 존재하지 않는다. | 높음 |
| `delayed_cctv_next_action` | `isolation_rules` | cctv | 최민재가 CCTV에서 플레이어가 아직 하지 않은 행동을 본다. | 높음 |
| `server_log_other_branch` | `isolation_rules` | build_log | 박도윤이 같은 사건의 다른 결과 로그를 발견한다. | 중간 |
| `automatic_minutes_no_attendees` | `normal_operations` | meeting_minutes | 참석자 없는 회의가 이미 결론을 냈다. | 중간 |
| `pantry_survivor_trace` | `opening_absence` | office_object | 탕비실에는 방금 사람이 있었던 흔적만 남아 있다. | 낮음 |

상세 카드는 `docs/content/encounter_db/isolation_pack.md`에 둔다.

## 결말 후보

| ending_candidate | 의미 |
|---|---|
| `escape_alone` | 플레이어만 격리 공간을 빠져나간다. 다른 격리자는 기록으로만 남는다. |
| `rescue_channel_members` | 조건을 맞춘 일부 격리자를 함께 현실로 끌어낸다. |
| `close_failed_worldlines` | 실패한 격리 구역을 닫아 현재 세계를 안정시킨다. 대가가 필요하다. |
| `accept_success_worldline` | 자신이 있던 세계가 아니라 “성공한 회사 기록” 쪽을 받아들인다. |
| `merge_isolation_zones` | 여러 격리 구역을 병합하지만 회사의 규칙도 함께 현실로 넘어온다. |

## Public-safe 기준

- 특정 실제 회사명, 제품명, 프로젝트명은 쓰지 않는다.
- 실제 장애 대응, 실제 보안 구조, 실제 내부망 주소처럼 보이는 세부사항은 쓰지 않는다.
- “대기업 SW 개발센터” 감각은 유지하되, 구체적 내부 프로세스가 아니라 보편적인 사내 시스템 표면으로 표현한다.
- 현실 연결 힌트는 이 팩 문서에 넣지 않는다.

## Runtime promotion notes

이 팩에서 runtime으로 먼저 승격할 후보는 다음 3개가 적합하다.

1. `isolation_channel_mismatched_floor`
   - 초반 hook으로 강함.
   - 메신저 UI와 잘 맞음.
2. `org_chart_missing_employee`
   - identity erasure를 명확히 보여줌.
   - HR NPC와 연결 가능.
3. `delayed_cctv_next_action`
   - 기존 보안실/CCTV 루트와 연결하기 쉬움.
   - 진실/정복 루트 hook으로 확장 가능.

주의:

- 셋 모두 runtime에 넣을 때는 once-only 또는 플래그 gating이 필요하다.
- hub 위치에 항상 eligible하게 두면 이동을 막을 수 있다.
- 실제 승격은 별도 구현 작업에서 content tests와 route smoke를 함께 작성한다.
