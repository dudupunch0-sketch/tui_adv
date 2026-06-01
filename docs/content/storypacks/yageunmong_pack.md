# 야근몽

Status: candidate

## Record

```yaml
id: yageunmong_pack
world_id: office_dream
status: candidate
source_refs:
  - idea_box/done/2026-05-29-notion-office-yageunmong.md
  - idea_box/done/2026-05-29-notion-storypack-system.md
  - notion:36f37e69-695e-8113-9dd5-ddc56a633226
reference_status: live Notion markdown fetched 2026-05-31; whitespace-normalized snapshot matched; idea_box entries archived done
name: 야근몽
one_line: 회사에서 잠깐 엎드려 잠든 주인공이 자각몽 상태의 회사 악몽에 갇히고, 업무 완료가 아니라 꿈에서 깨어나는 것을 목표로 한다.
main_surfaces:
  - messenger
  - approval_system
  - meeting_minutes
  - reservation_panel
  - organization_chart
  - document_viewer
  - office_object
  - security_gate
anomaly_types:
  - space_loop
  - delayed_time
  - future_record
  - document_contamination
  - permission_denied_as_existence
  - identity_erasure
main_phases:
  - late_night_sleep
  - lucid_dream_awareness
  - nightmare_office_loop
  - reality_anchor_collection
  - clockout_gate_refusal
  - wake_resolution
sensitive_topics:
  - workplace_burnout
  - real_company_similarity
  - mental_health_recovery
reusable_npc_slots:
  - pm_worldline_mediator
  - hr_identity_keeper
  - facility_pathfinder
  - newcomer_mirror
ending_candidates:
  - wake_up
  - endless_overtime_loop
  - rescue_coworkers
  - clockout_declaration
main_spine_support: 같은 office surface를 사용하되 목표를 업무 처리에서 자각몽 탈출로 바꾸어, 공용 시스템의 소속/권한/관계/성장/엔딩 축이 회사 악몽 storypack에서도 성립하는지 검증한다.
runtime_promotion_notes: 기본 office isolation runtime을 대체하지 않는다. 첫 구현은 별도 storypack preview 또는 명시적 flag 경로에서 기존 encounter schema만 쓰는 docs-first 후보로 둔다.
```

## 현재 반영 범위

이 문서는 Notion-origin `회사 스토리팩: 야근몽` 아이디어를 repo의 storypack 후보 문서로 승격한 것이다. 2026-05-31에 Notion API로 원본 Markdown을 다시 가져와 `idea_box` 로컬 스냅샷과 공백 정규화 기준으로 일치함을 확인했고, 이 문서와 후보 DB가 원문의 핵심 축을 반영하므로 관련 idea entry를 `done` 처리했다. runtime 구현은 아직 future work다.

## 핵심 컨셉

주인공은 야근 중 책상에 엎드려 잠깐 눈을 붙인다. 다시 눈을 뜨면 여전히 회사지만, 시간과 공간과 사람의 반응이 현실과 어긋나 있다. 주인공은 곧 이곳이 꿈이라는 사실을 자각하지만, 꿈인 것을 안다고 바로 깰 수는 없다.

핵심 목표는 업무를 끝내는 것이 아니다.

```text
업무 완료가 아니라 꿈에서 깨어나기
```

악몽은 계속 “이것만 끝내면 퇴근할 수 있다”고 속인다. 플레이어는 업무를 완벽히 처리하는 대신, 꿈의 규칙을 알아차리고 거부해야 한다.

## legacy office storypack과의 관계

- `isolation_pack` / `office_apocalypse`는 legacy office runtime/parity 콘텐츠와 가장 가까운 후보로 유지한다.
- `yageunmong_pack` / `office_dream`은 같은 회사 surface를 쓰는 별도 office-family 후보이며, Web/default 이구학지 runtime이나 legacy office runtime을 자동으로 rewrite하지 않는다.
- 현실 회사명, 실제 직원, 실제 위치, private reality hint는 넣지 않는다.
- 자각몽/번아웃/회복 서사는 장식이 아니라 선택지와 엔딩 목표를 바꾸는 storypack premise다.

## 메인 story spine

| phase | 역할 |
|---|---|
| `late_night_sleep` | 야근 중 책상에 엎드리고, 회사 악몽으로 진입한다. |
| `lucid_dream_awareness` | 시간이 멈추고, 문서/메신저/공간이 반복되며, 플레이어가 자각몽임을 인식한다. |
| `nightmare_office_loop` | 상사/동료/업무가 사람이라기보다 압박과 죄책감의 형상으로 나타난다. |
| `reality_anchor_collection` | 식은 커피 냄새, 손목시계 초침, 현실 알람음 같은 현실 앵커를 모은다. |
| `clockout_gate_refusal` | 퇴근 게이트는 업무 완료나 승인으로 열리지 않고, 꿈의 핵심 규칙 거부로 열린다. |
| `wake_resolution` | 업무를 완벽히 끝내는 대신, 나가야 하므로 나간다는 선언으로 깨어난다. |

## 공용 시스템 대응

| 공용 시스템 | isolation/office apocalypse 표현 | 야근몽 표현 | 이구학지 표현 |
|---|---|---|---|
| 소속 | 부서, 임시 대응방 | 악몽화된 팀/회의실/책임 범위 | 청류문 수습생, 문파 |
| 평판 | 사내 평판, 권한 로그 | 책임도, 신뢰도, 죄책감 압박 | 무림 명성, 외지인 의심 |
| 권한 | 결재, 사원증, 출입 권한 | 승인받아야 나갈 수 있다는 꿈의 함정 | 장문 허가, 통행/참가 자격 |
| 관계 | 격리자, 동료, NPC 협력자 | 악몽화된 상사/동료와 구출 가능한 동료 | 서하린, 사형제, 기록관 |
| 성장 | 단서, 아이템, 업적 | 각성편린, 현실 앵커, 자각도 상승 | 천외편린, 무공 숙련 |
| 전투 | 회피, 설득, 시스템 제압 | 논쟁, 버티기, 모순 지적, 규칙 거부 | 난투, 도주, 현대식 몸 사용 |
| 엔딩 | 탈출/정복/진실/히든 현실 연결 | 각성, 무한 야근, 동료 구출, 퇴근 선언 | 귀환, 정착, 세력 루트 |

## 주요 surface

### messenger

죽은 프로젝트나 끝난 회의 이름으로 메신저 알림이 온다. 모든 알림에 답하면 꿈의 루프가 강해진다.

### approval_system

승인받아야 퇴근할 수 있다는 꿈의 규칙을 표면화한다. 최종적으로는 승인을 받는 것이 아니라 승인 규칙 자체를 거부해야 한다.

### meeting_minutes / reservation_panel

회의실과 회의록이 반복된다. 참석자가 없어도 회의는 끝나지 않고, 결론은 항상 “다시 검토”로 돌아간다.

### document_viewer

보고서, 액션 아이템, 업무 문서가 주인공의 책임 범위를 오염시킨다. 문서를 끝내는 선택과 문서의 모순을 보는 선택을 분리한다.

### office_object

식은 커피, 손목시계, 노트북 덮개, 책상 위 알람 같은 현실 앵커가 깨어나는 route hook이 된다.

### security_gate

퇴근 게이트의 surface다. 물리적 문이 아니라 “나가려면 허가받아야 한다”는 악몽 규칙을 거부할 때 열린다.

## 각성편린 / 현실 앵커 규칙

각성편린은 무협팩의 천외편린과 대응되는 storypack별 성장 장치다. 세 후보 중 하나만 고르는 3택 구조는 공유할 수 있지만, 첫 runtime slice에서는 새 성장 schema를 열지 않고 flag/clue/log/presentation text로만 표현한다.

예시 후보:

- `이건 꿈이다`: 현재 공간의 모순을 하나 파악한다.
- `이 일은 내 일이 아니다`: 책임 전가 공격에 저항한다.
- `문을 닫고 나간다`: 퇴근 게이트 탐색 가능성이 열린다.

현실 앵커는 깨어나는 루트를 여는 단서다. 실제 사무실 위치나 private clue가 아니라 보편적이고 public-safe한 감각 단서로 둔다.

## 후보 인카운터 카드

| id | phase | surface | 핵심 상황 | 승격 우선순위 |
|---|---|---|---|---|
| `yageunmong_late_night_desk_awake` | `late_night_sleep` / `lucid_dream_awareness` | `office_object`, `messenger` | 야근 중 엎드린 뒤, 멈춘 시간과 퇴근 미승인 알림으로 자각몽 진입을 보여준다. | 높음 |
| `yageunmong_unapproved_meeting_room_loop` | `nightmare_office_loop` | `reservation_panel`, `meeting_minutes` | 회의실 문과 회의록이 같은 검토 루프로 돌아간다. | 높음 |
| `yageunmong_manager_approval_trap` | `nightmare_office_loop` / `clockout_gate_refusal` | `approval_system`, `organization_chart` | 승인하지 않는 팀장이 권한/책임을 꿈의 잠금으로 만든다. | 높음 |
| `yageunmong_reality_anchor_pantry` | `reality_anchor_collection` | `office_object`, `document_viewer` | 식은 커피와 현실 알람음 같은 앵커를 발견한다. | 중간 |
| `yageunmong_awakening_fragment_choice` | `reality_anchor_collection` | `document_viewer`, `approval_system` | 각성편린 3택 후보가 떠오른다. | 중간 |
| `yageunmong_clockout_gate_self` | `clockout_gate_refusal` / `wake_resolution` | `security_gate`, `approval_system` | 최종 적인 퇴근을 잊은 나 자신과 마주하고, 퇴근 선언으로 깨어난다. | 중간 |

상세 카드는 `docs/content/encounter_db/yageunmong_pack.md`에 둔다.

## Runtime promotion notes

첫 runtime 승격은 다음 원칙을 지킨다.

1. 기본 `isolation_pack` / office runtime을 야근몽으로 자동 대체하지 않는다.
2. 별도 storypack preview 또는 명시적 flag 경로에서만 연다.
3. `ScenePage`, action id, 기존 encounter/choice/outcome schema를 우선 사용한다.
4. 각성편린 3택은 매력적이지만 새 reward/ability schema로 즉시 확장하지 않는다.
5. 정신건강/번아웃 소재는 조롱하지 않고, 회복 서사의 목표가 “일을 더 잘함”이 아니라 “꿈의 규칙을 거부하고 깨어남”임을 유지한다.
