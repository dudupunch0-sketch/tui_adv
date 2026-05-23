---
status: done
created: 2026-05-22
source: user
source_ref: origin/main:idea_box/general_corporate_storypacks.md
related_docs:
  - idea_box/general_corporate_storypacks.md
  - docs/design/Storypack_Encounter_DB.md
  - docs/content/storypacks/isolation_pack.md
  - docs/content/encounter_db/isolation_pack.md
  - docs/content/characters/recurrent_npcs.md
used_by:
  - docs/design/Storypack_Encounter_DB.md
  - docs/content/storypacks/isolation_pack.md
  - docs/content/encounter_db/isolation_pack.md
  - docs/content/characters/recurrent_npcs.md
done_at: 2026-05-22
---

# 일반 대기업 배경 스토리팩 후보

## 원문/요지

사용자는 `idea_box/general_corporate_storypacks.md`에 일반 대기업 본사/계열사 사무직 조직을 배경으로 한 스토리팩 시나리오 초안을 추가했다.

이 아이디어는 특정 반도체/SW 개발센터에 한정된 이전 스토리팩보다 더 넓은 “일반 대기업 오피스 호러” 후보로 볼 수 있다. 주요 무대는 사무실, 회의실, 사내 메신저, 인트라넷, 결재 시스템, 문서관리 시스템, 근태 시스템, 조직도, 보안 게이트, 인사평가, 연봉협상, 사내공지 등이다.

원문 핵심 문장:

```text
비현실적인 일이 벌어졌지만, 회사의 말투는 끝까지 현실적이어야 한다.
회사는 끝까지 평범하게 굴고, 바로 그 평범함 때문에 세계가 무너진다.
```

이 문서는 원문을 나중의 설계/개발 agent가 바로 검토할 수 있도록 구조화한 아이디어 후보 문서다. 아직 확정 세계관이나 구현 요구사항이 아니며, 기존 게임 스토리/런타임 콘텐츠를 즉시 대체하라는 뜻도 아니다.

## 핵심

이 아이디어의 중심은 “회사 자체는 이상하지 않다”는 전제다.

회사는 마법 기업, 비밀 연구소, 초자연 조직이 아니다. 이상한 것은 사건이고, 그 사건이 평범한 회사 시스템을 타고 퍼진다.

핵심 문법:

- 비상상황이 발생해도 회사는 평소처럼 메일을 보내고 결재를 요구한다.
- 사람이 사라져도 회의록은 공유되고, 현실이 뒤틀려도 결재선은 확인된다.
- 사람은 직원/동료이기 전에 기록, 비용, 권한, 참석자, 결재선, 평가등급으로 처리된다.
- 공포는 괴물보다 “현재 상태는 정상으로 확인됩니다” 같은 자동 응답에서 나온다.
- 블랙코미디는 웃긴데 웃고 나면 찝찝해야 한다.

반복 사용 가능한 회사식 문장:

```text
본 메일은 자동 발송되었습니다.
해당 문의는 담당 부서로 이관되었습니다.
회의록 공유드립니다.
첨부파일을 확인해 주세요.
현재 상태는 정상으로 확인됩니다.
해당 직원은 조회되지 않습니다.
사내 규정에 따라 처리 예정입니다.
자세한 사항은 FAQ를 참고해 주시기 바랍니다.
```

## 기대 효과

- 반도체/SW 특화 지식 없이도 회사생활 경험만으로 공감 가능한 스토리팩 재료가 된다.
- 기존 한국어 회사-아포칼립스 선택지 게임의 핵심 톤과 잘 맞는다.
- TUI/fake-terminal UI와 잘 맞는 표면이 많다: 메신저, 회의록, 결재, 문서관리, 회의실 예약, 근태, 조직도, 인사/보상 시스템.
- 각 팩은 독립 에피소드로도, 장기 캠페인으로도 쓸 수 있다.
- 이전 `semiconductor_sw_storypacks` 아이디어를 더 일반화한 버전으로 참고할 수 있다.

## 반복 등장인물 후보

| 이름 | 부서/역할 | 기본 성격 | 스토리 기능 |
|---|---|---|---|
| 박도윤 | IT 인프라팀 | 냉소적이지만 책임감 있음 | 사내망, 서버 로그, 계정 권한, 시스템 오류 분석 |
| 윤서연 | 인사팀 | 원칙주의자이지만 인간적임 | 조직도, 근태, 직원 신원, 연봉협상, 징계 절차 |
| 한지우 | 전략기획팀/PM | 유능하지만 무리수를 둠 | 실패한 프로젝트, 세계선 분기, 정치적 판단 |
| 최민재 | 보안팀/방재실 | 말수 적고 현장 감각 좋음 | 출입통제, CCTV, 건물 봉쇄, 물리적 생존 |
| 이가람 | 총무팀/시설관리 | 사소한 시스템에 밝음 | 회의실, 비품, 층별 구조, 예약 시스템 |
| 송나래 | 커뮤니케이션/홍보 | 말을 예쁘게 포장하지만 눈치 빠름 | 사내공지, 대외 메시지, 여론 관리 |
| 장우혁 | 재무팀/경영관리 | 숫자로 세상을 봄 | 비용, 예산, 연봉 테이블, 협상 시뮬레이션 |
| 권미선 | 미화/용역직원 | 회사의 비공식 구조를 잘 앎 | 야간 사무실, 뒷문, 창고, 직원들이 모르는 공간 |
| 오세림 | 법무팀 | 냉정하고 절차 중심 | 문서 유출, 책임 소재, 노사 분쟁, 합의문 검토 |
| 강해준 | 신입사원 | 상황 파악은 늦지만 관찰력 있음 | 플레이어의 거울, 이상현상을 순수하게 받아들이는 인물 |

주의: 이 인물들은 아직 실제 NPC로 채택된 것이 아니다. 기존 캐릭터 시스템 또는 6스탯 등장인물 생성기 후보와 통합할지 별도 결정이 필요하다.

## 스토리팩 후보 목록

| 번호 | 스토리팩 | 한 줄 컨셉 | 주요 시스템 | 핵심 공포 |
|---:|---|---|---|---|
| 1 | 차원격리팩: 혼자 남은 사무실 | 퇴근 직전 회사 일부 공간이 격리되고, 플레이어는 메신저/인트라넷으로 다른 격리자와 연결된다. | 메신저, 인트라넷, 회의록, 조직도, 출입기록 | 연결되어 있지만 만날 수 없음 |
| 2 | 문서오염팩: 열람하지 마십시오 | 평범한 업무 문서를 열람한 사람들이 회사와 현실을 조금씩 다르게 인식한다. | PDF, 공유폴더, 문서관리, 보안공지, 조직도 | 읽는 순간 현실 인식이 바뀜 |
| 3 | 회의실예약팩: 예약된 회의는 반드시 열린다 | 회의실 예약 조건이 과거/미래/다른 세계선 회의 공간으로 이어진다. | 회의실 예약, 일정표, 회의록, 참석자 명단 | 모든 결정은 어딘가에서 이미 열렸음 |
| 4 | 연봉협상-파업팩: 기준선 아래의 것들 | 연봉 테이블과 보상 시뮬레이션이 사람의 가치, 기억, 소속, 존재를 재계산한다. | 연봉표, 협상안, 근태, 조직도, 익명 채널 | 인간의 가치가 숫자에 종속됨 |

## 스토리팩별 구현 후보

### 1. 차원격리팩: 혼자 남은 사무실

구현 후보:

- 사내 메신저 `#격리자-임시대응방`을 핵심 UI/서사 장치로 둔다.
- 격리자들은 같은 회사에 있는 것 같지만 층수, 시간, 날씨, 배치가 서로 맞지 않는다.
- 플레이어는 직접 만날 수 없는 인물들의 메신저 증언, 회의록, CCTV, 조직도, 로그를 조합한다.
- 마지막 선택은 “나만 탈출”, “함께 구출”, “실패한 세계선 닫기”, “성공한 세계선 수용”, “격리구역 병합” 같은 방향이 될 수 있다.

런타임 후보:

```text
locations: office_after_hours, server_room_loop, hr_meeting_room, security_control_room, endless_meeting_corridor, pantry_supply_loop
flags: isolation_channel_joined, automatic_minutes_seen, mismatched_floor_reported, future_message_seen
items: auto_minutes_fragment, access_log_export, hr_record_unknown_employee, cctv_loop_clip
endings: escape_alone, rescue_channel_members, close_failed_worldlines, accept_success_worldline, merge_isolation_zones
```

### 2. 문서오염팩: 열람하지 마십시오

구현 후보:

- 플레이어가 문서를 직접 열람할지 말지 선택하게 한다.
- 열람하면 단서를 얻지만 오염도가 오른다.
- 읽지 않은 사람은 현실 기준점이 되지만 핵심 단서를 이해하기 어렵다.
- 보안팀의 열람 금지 공지가 문제 문서를 첨부하면서 오염을 확산시키는 블랙코미디를 사용한다.
- 문서가 현실을 설명하는 것이 아니라 편집하는 방향으로 후반을 전개한다.

오염도 후보:

| 단계 | 상태 |
|---:|---|
| 0 | 문서를 열람하지 않음. 현실 기준점 가능 |
| 1 | 문서 내용 일부가 머릿속에 남음 |
| 2 | 조직도, 이름표, 회의실 표기가 다르게 보임 |
| 3 | 존재하지 않는 회의를 기억함 |
| 4 | 다른 사람의 경력과 자신의 기억이 섞임 |
| 5 | 문서가 플레이어를 문서의 일부로 취급함 |

런타임 후보:

```text
items: contaminated_report_pdf, blocked_notice_with_attachment, printed_page_47, document_version_log
flags: report_opened, contamination_level_1, baseline_reader_preserved, deletion_failed, organization_chart_drifted
endings: report_destroyed, report_used, baseline_reality_preserved, efficient_company_accepted, person_saved_by_record_sacrifice
```

### 3. 회의실예약팩: 예약된 회의는 반드시 열린다

구현 후보:

- 회의실 예약 조건을 퍼즐 입력으로 사용한다.
- 안건, 참석자, 회의 시간, 비용센터, 반복회의 여부, 임원 참석 여부에 따라 연결되는 공간이 바뀐다.
- 회의록은 단서이자 저주로 작동한다. 가져오거나 수정하면 현재 현실도 바뀐다.
- 최종 회의실은 플레이어가 누구를 참석자로 초대하느냐에 따라 달라진다.

예약 조건 후보:

| 예약 조건 | 연결되는 공간 |
|---|---|
| 과거 프로젝트명을 안건에 입력 | 해당 프로젝트가 실패한 날의 회의실 |
| 참석자에 퇴사자를 추가 | 퇴사 직전 면담실 |
| 회의 시간을 새벽 3시로 설정 | 야간의 빈 회사 |
| 비용센터를 잘못 입력 | 존재하지 않는 부서의 회의실 |
| 반복회의로 설정 | 무한히 반복되는 주간회의 |
| 임원 참석 필수로 설정 | 미래의 징계위원회 또는 경영회의 |
| 회의 목적을 비워둠 | 아무도 왜 모였는지 모르는 회의실 |

런타임 후보:

```text
locations: failed_project_room, closed_department_room, future_disciplinary_committee, repeated_weekly_sync, executive_room, final_coordination_room
mechanics: reservation_condition_routing, meeting_minutes_collect, attendee_list_choice, minutes_rewrite_risk
endings: close_abnormal_rooms, rewrite_past_meeting, cancel_future_discipline, revive_failed_project, disclose_all_minutes, no_attendee_meeting
```

### 4. 연봉협상-파업팩: 기준선 아래의 것들

구현 후보:

- 사측/노조/현장 직원/인사/재무/익명 채널을 모두 단순 악역이 아닌 이해관계자로 둔다.
- 연봉 테이블과 보상 시뮬레이션이 사람을 재분류하면서 권한, 조직도, 자리배치, 명함, 사원증, 기억이 바뀐다.
- 정치적 선택과 인간적 선택이 충돌하게 만든다.
- “파일 공개”는 협상 구도를 바꾸지만 오염을 확산시키고, “파일 은폐”는 혼란은 줄이나 은폐로 보일 수 있게 한다.

결말 후보:

| 결말 | 요지 |
|---|---|
| 합의안 타결 | 파업은 막지만 일부 조항이 사람들을 영구 재분류한다. |
| 파업 돌입 | 회사 시스템이 파업 참가자를 업무 중단 변수로 처리한다. |
| 사측 승리 | 회사는 안정되지만 기준선 아래 사람들은 기록될 권리를 잃는다. |
| 노조 승리 | 요구안은 관철되지만 보상 모델이 다른 비용을 청구한다. |
| 시스템 공개 | 모두가 진실을 알지만 같은 진실을 보지는 않는다. |
| 기준선 파괴 | 숫자로 사람을 분류하는 기준을 끊지만 보상 기준 없음 상태가 된다. |

런타임 후보:

```text
items: compensation_simulation_xlsx, leaked_negotiation_plan, strike_vote_notice, anonymous_board_capture
flags: pay_model_opened, salary_zero_seen, baseline_below_seen, strike_vote_started, model_disclosed, baseline_destroyed
endings: agreement_reached, strike_begins, management_wins, union_wins, system_disclosed, baseline_destroyed
```

주의: 노동/파업/연봉/평가 소재는 현실적으로 민감하다. 특정 실제 회사, 실제 노조, 실제 인물, 실제 내부자료처럼 보이는 요소를 피하고, 게임 내 허구의 회사 시스템 공포로 처리해야 한다.

## 이전 반도체 SW 스토리팩과의 관계 후보

이 문서는 `semiconductor_sw_storypacks.md`보다 더 일반적인 대기업 배경 버전이다.

가능한 처리 방향:

- 일반 대기업 버전을 상위 세계관/기본팩으로 두고, 반도체 SW 버전은 산업 특화 변형팩으로 둔다.
- 반도체 SW 버전에서 너무 기술적인 소재를 덜어내고, 일반 대기업 버전의 회의/문서/보상 시스템으로 흡수한다.
- 둘을 별개 후보로 유지하고, 1차 구현은 더 보편적인 일반 대기업 버전에서 시작한다.
- 특정 팩만 교차 채택한다. 예: 일반 `차원격리팩` + 반도체 SW의 `빌드/릴리즈` 소재를 결합.

## TUI/fake-terminal 연출 후보

원문에서 특히 잘 맞는 표면:

```text
자동공지: 야근 사유가 부적절합니다.
보안알림: 해당 문서는 악성코드가 아닙니다.
문서관리: 해당 문서는 정상 승인된 업무자료입니다.
회의실 예약 알림: 예약 시간이 초과되었습니다.
자동회의록: 참석자 전원 사망으로 회의가 조기 종료되었습니다.
급여명세서: 이번 달 지급액 — 존재 확인 필요
현재 상태는 정상으로 확인됩니다.
해당 직원은 조회되지 않습니다.
```

UI/EffectCue 후보:

- 메신저 채널 로그가 시간/발신자를 뒤섞음
- 조직도에서 인물이 흐려지거나 소속이 바뀜
- 회의록이 자동 생성되며 선택 결과를 선반영함
- 문서 뷰어의 페이지 번호/버전명이 계속 바뀜
- 회의실 예약 폼의 입력값이 장소 이동 좌표가 됨
- 급여명세서/연봉 테이블의 숫자가 인물 상태를 바꿈
- 권한 오류 메시지가 존재 판정처럼 작동함

## 데이터화 후보

실제 콘텐츠로 승격한다면 원문 전체를 한 번에 구현하지 말고, 다음 단위로 분리한다.

- `storypack` 메타데이터: id, 제목, 톤, 핵심 시스템, 민감 주제
- `characters`: 반복 등장인물 후보, 신뢰/의심/빚/배신 상태
- `locations`: 사무실, 서버실, 인사팀 회의실, 방재실, 회의실 복도, 문서관리 화면 등
- `items`: 회의록, 오염 문서, 예약 기록, 보상 시뮬레이션 파일
- `events`: 자동메일, 메신저 알림, 결재 요청, 근태 경고, 회의 초대
- `flags`: 문서 열람 여부, 오염도, 격리자 채널 신뢰도, 기준선 미만 판정
- `endings`: 팩별 결말 후보
- `effect_cues`: terminal glitch, dashboard rewrite, auto-minutes overwrite, org-chart drift 등

## 주의점

- 이 아이디어는 후보 입력이며, 현재 구현 범위를 자동으로 확장하지 않는다.
- 일반 대기업 소재는 현실 회사와 닮기 쉬우므로 실제 회사명/실제 조직명/실제 사건/개인정보처럼 보이는 요소는 피한다.
- 노동, 파업, 연봉, 평가, 징계, 법무 소재는 특정 집단 조롱이나 현실적 조언처럼 보이지 않도록 조심한다.
- 블랙코미디는 회사 시스템의 비인간성을 찝찝하게 보여주는 방향으로 유지한다.
- 반복 등장인물은 실제 NPC로 승격하기 전에 기존 캐릭터/스탯/관계 시스템과 통합 여부를 결정해야 한다.
- 스토리팩 4개는 범위가 크므로, 설계 문서화 또는 1개 대표팩 선정 후 작은 슬라이스로 검증하는 편이 안전하다.
- 공개 문서와 런타임 콘텐츠에는 실제 사용자의 사적 메모나 민감한 현실 힌트를 넣지 않는다.

## 설계자에게 물어볼 질문

- 이 일반 대기업 스토리팩을 기본 세계관으로 삼을 것인가, 아니면 기존 회사-아포칼립스 세계의 후보 팩으로 둘 것인가?
- `semiconductor_sw_storypacks`와 이 문서 중 어느 쪽을 상위 기준으로 둘 것인가?
- 1차 구현 후보로 어떤 팩이 가장 적절한가?
  - 범용성과 핵심 소개: `차원격리팩`
  - 텍스트/TUI 연출: `문서오염팩`
  - 퍼즐 구조: `회의실예약팩`
  - 후반 대형 캠페인: `연봉협상-파업팩`
- 반복 등장인물 10명을 실제 NPC 후보로 승격할 것인가, 아니면 archetype으로만 유지할 것인가?
- 이전 6스탯 캐릭터 시스템 후보와 연결해 인물 시트를 만들 것인가?
- 팩별 결말을 기존 엔딩 타입(`escape`, `truth`, `conquest`, `hidden`, `failure`)에 매핑할 것인가, 새 타입을 추가할 것인가?
- 메신저/문서/회의록/예약/연봉 테이블을 실제 게임 시스템으로 구현할 것인가, 텍스트 연출과 EffectCue로만 표현할 것인가?
- 노동/평가/징계 같은 민감 소재의 공개 문서/런타임 데이터 안전 경계는 어디까지인가?

## 처리 기록

- 2026-05-22: 원격 `origin/main:idea_box/general_corporate_storypacks.md`에 추가된 원문을 확인하고, 다른 agent가 검토/설계/개발 후보로 사용할 수 있도록 이 구조화된 `idea_box/inbox` 아이디어 문서로 새로 정리했다. 아직 실제 설계 문서, 런타임 콘텐츠, 코드, 데이터에 반영된 것은 아니므로 `open` 상태를 유지한다.
- 2026-05-22: 일반 대기업 배경의 스토리팩/반복 인물/회사식 시스템 표면을 `docs/design/Storypack_Encounter_DB.md`, `docs/content/storypacks/isolation_pack.md`, `docs/content/encounter_db/isolation_pack.md`, `docs/content/characters/recurrent_npcs.md`에 반영했다. 일반 대기업 버전을 상위 기준으로 삼고 차원격리팩을 첫 DB slice로 채택했으므로 `done` 처리한다.
