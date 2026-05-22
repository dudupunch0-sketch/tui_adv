# 스토리팩 DB

Status: 후보 콘텐츠 DB

이 폴더는 `escape from the office`의 storypack 후보를 정리한다.

스토리팩은 런타임에 바로 들어가는 확정 데이터가 아니라, 여러 인카운터 상황 카드와 NPC 후보를 묶는 콘텐츠 설계 단위다. 확정된 일부만 나중에 `src/tui_adv/data/*.yaml` 또는 Rust content bundle로 승격한다.

## 원칙

- 메인 스토리 spine을 대체하지 않는다.
- storypack은 surface, anomaly palette, NPC slot, route hook을 제공한다.
- 실제 회사명, 실제 조직명, 실제 사람, 실제 내부자료처럼 보이는 세부사항은 쓰지 않는다.
- 모든 storypack은 `source_refs`로 idea_box 또는 기존 문서 출처를 남긴다.
- `status: promoted`는 runtime content로 실제 승격된 뒤에만 사용한다.

## status

| status | 의미 |
|---|---|
| `raw` | 아이디어에서 막 추출한 원재료 |
| `candidate` | DB 형식으로 정리된 후보 |
| `curated` | 톤/안전/구조 검토 통과 |
| `promoted` | 런타임 YAML/content bundle로 승격됨 |
| `merged` | 다른 팩에 흡수됨 |
| `rejected` | 톤/안전/중복/범위 문제로 폐기됨 |

## 현재 후보 목록

| id | 이름 | status | 한 줄 컨셉 | 주 surface | 문서 |
|---|---|---|---|---|---|
| `isolation_pack` | 차원격리팩 | candidate | 회사 일부 공간이 격리되고, 사내 시스템으로 다른 격리자들과 연결된다. | messenger, cctv, meeting_minutes, organization_chart | `isolation_pack.md` |
| `document_contamination_pack` | 문서오염팩 | raw | 평범한 업무 문서를 열람한 사람들이 현실을 다르게 인식한다. | document_viewer, intranet, organization_chart | 추후 분리 |
| `meeting_reservation_pack` | 회의실예약팩 | raw | 회의실 예약 조건이 과거/미래/다른 세계선 회의 공간으로 이어진다. | reservation_panel, meeting_minutes | 추후 분리 |
| `compensation_strike_pack` | 연봉협상-파업팩 | raw | 보상 시뮬레이션과 파업/협상 시스템이 사람의 가치와 존재를 재계산한다. | payroll_sheet, approval_system, intranet | 추후 분리 |

## 우선순위

첫 설계 slice는 `isolation_pack`만 full record로 작성한다.

나머지 팩은 다음 조건을 만족할 때 별도 문서로 승격한다.

1. 해당 팩의 후보 encounter situation card가 최소 6개 있다.
2. public-safe 민감도 검토를 통과할 수 있다.
3. 메인 story spine과 연결되는 route hook이 있다.
4. 기존 runtime encounter와 중복되지 않는 역할이 있다.

## 관련 문서

- `docs/design/Storypack_Encounter_DB.md`
- `docs/content/encounter_db/README.md`
- `docs/content/characters/README.md`
- `idea_box/inbox/2026-05-22-general-corporate-storypacks.md`
- `idea_box/inbox/2026-05-22-semiconductor-sw-storypacks.md`
