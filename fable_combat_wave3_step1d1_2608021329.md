# Combat Wave 3 Step 1d-1 — terminal 관전 렌더러

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step1d1` (`origin/main` = `da8e289`, PR #181 머지 직후)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **322 passed / 0 failed** (2026-08-02 WSL 실측)
- Step 2b가 `ScenePage.combat`을 실제로 채우는 인카운터를 제공한다 (`wuxia_combat_spectator_preview_bout`, 게이트 `combat_spectator_preview_unlocked`).

## Step 1d 분할

Step 1d를 둘로 쪼갠다.

- **1d-1 (이 플랜)**: terminal(SuperLightTUI) 관전 렌더러. Web·QA 게이트가 없어 독립 검증이 쉽다. **게이트 플래그는 그대로 둔다.**
- **1d-2 (후속)**: Web Storybook 관전 렌더러 + TS 타입 + 게이트 플래그 제거 + 5뷰포트 실화면 QA + wasm 재빌드.

게이트 제거를 1d-2에 두는 이유: 게이트를 풀면 Web 기본 플레이 경로에 노출되므로, Web 렌더러가 준비된 슬라이스에서 함께 풀어야 한다.

## 정본 근거

- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6)
  - "화면 상단은 실제 실시간 전투 시뮬레이션, 하단은 전투 로그로 구성한다."
  - "캐릭터는 체스말 형태로 표시한다."
  - "공용 문법은 공격=짧은 전진/복귀, 피격=밀림/진동, 회피=측면 이동, 균형 붕괴=흔들림/기울어짐, 전투불능=흐려짐/표식이다."
  - "전투 중 하단에는 핵심 로그만 출력하되, 모든 공격·이동·판정은 전체 로그에 저장하여 일시정지 또는 전투 종료 뒤 열람할 수 있게 한다."
  - "**로그는 자유 생성 문장이 아니라 등록된 사건 태그와 로그 템플릿을 사용한다.**"
  - "시간 조작은 금지한다. 순간 정지와 슬로모션을 사용하지 않으며 … 연출은 판정 뒤 표현만 담당하여 seed·판정·AI 순서에 영향하지 않는다."
  - 보고서: "승패, 전투 시간, 생존/전투불능, 최대 피해 가한 캐릭터, 최대 피해 받은 캐릭터 … 발생하지 않은 항목은 숨긴다." 금지: 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교
- [07. UI·템포·리스크](https://app.notion.com/p/36f37e69695e81258c60fc669f4d6800): "전체 로그와 상세 수치는 일시정지 또는 전투 종료 뒤 별도로 열람한다. 텍스트는 보조 정보다."

## Scope

`crates/escape-terminal`가 `ScenePage.combat`을 텍스트로 표시한다. 판정·집계를 하지 않는다.

- P1: 로그 템플릿 테이블 (renderer 소유)
- P2: 체스말 보드 (최종 프레임)
- P3: 핵심 로그 섹션
- P4: 전투 종료 보고서 섹션
- P5: 스냅샷 통합 + 단위 테스트

## Hard invariants (위반 금지)

1. **판정·집계 재계산 금지.** renderer는 `ScenePage.combat`에 이미 있는 값만 포맷한다. 피해 합산, 승패 판정, cue 파생, 로그 필터링을 다시 하지 마라. `escape-core`의 combat 함수(`resolve_combat`, `conclude_combat`, `spectate_combat`)를 **호출하지 마라.**
2. **`escape-core` 수정 금지.** 이 slice는 renderer 전용이다. core에 필요한 데이터가 없으면 **멈추고 보고하라** — 임시로 renderer에서 계산해 채우지 마라.
3. **로그 문장은 템플릿 테이블에서만.** core가 주는 `template_id`를 알려진 테이블로 매핑한다. 자유 생성 문장을 만들지 마라. **알 수 없는 `template_id`는 조용히 버리지 말고** 안전한 fallback 줄(예: `template_id` 자체를 노출)로 표시한다 — 조용한 유실은 로그 계약 위반이다.
4. **누설 금지 유지.** core가 이미 `AttackRoll`·`EffectSuppressed`를 제외하고 Hidden 효과 id를 마스킹했다. renderer가 `combat.view` 밖의 원본 데이터를 뒤져 그 값을 복원하지 마라. `_hidden` 템플릿은 "무언가 걸렸다" 수준으로만 표시한다.
5. **시간 조작 금지.** 스냅샷은 정적 텍스트다. 애니메이션·지연·재생 순서 조작을 넣지 마라.
6. **panic 금지.** 좌표 범위, 빈 프레임, 빈 로그, 전투원 0명 같은 경계에서 `unwrap`/인덱싱으로 죽지 마라. 보드 폭이 상한을 넘으면 좌표 목록으로 대체한다 (아래 파생 규칙).
7. **결정론**: 같은 `ScenePage`는 같은 텍스트를 만든다. `HashMap` 순회 의존 금지.
8. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs`를 **수정하지 마라.** 읽기만. 새 테스트는 `snapshot.rs` 안 `#[cfg(test)] mod tests`에 둔다 (renderer 함수가 `pub(crate)`라 외부 통합 테스트에서 호출할 수 없다).
9. **`.claude/worktrees/`** 읽지도 쓰지도 마라.
10. **게이트 플래그를 제거하지 마라.** 1d-2 소관이다.
11. **콘텐츠·번들 불변**: YAML, fixture, `web/src/data/generated/**` 수정 금지. `web/` 전체를 건드리지 마라.
12. **신규 의존성 금지**, `Cargo.toml` 수정 금지.

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-terminal/src/snapshot.rs` | 템플릿 테이블·보드·로그·보고서 렌더 + 단위 테스트 |
| `docs/dev/TUI_Layout.md` | terminal 관전 표시 계약 기술 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·경계 갱신 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 파생 규칙

### 로그 템플릿 테이블 (P1)

`template_id` → 한국어 문장 형식. 6개를 모두 다룬다.

| `template_id` | 문장 형식 (actor/target/value 치환) |
|---|---|
| `combat.log.move_intent` | 이동 의도 |
| `combat.log.target_selection` | 목표 지정 |
| `combat.log.collision` | 충돌 |
| `combat.log.damage_applied` | 피해 (`value_hundredths`를 정수 반올림으로 표시 — 정본 11 §8 "UI는 정수 반올림으로 표시한다") |
| `combat.log.effect_applied` | 효과 적용 (effect id 표시) |
| `combat.log.effect_applied_hidden` | 효과 적용, 정체 불명 (effect id 표시 금지) |

문장 톤은 기존 terminal 스냅샷 표기와 맞춘다. 알 수 없는 id는 fallback 줄.

### 체스말 보드 (P2)

`combat.view.frames`의 **마지막 프레임**을 보드로 그린다. 정적 스냅샷이므로 결착 시점이 가장 정보량이 많다.

- 보드 범위는 그 프레임 pieces의 `position` min/max에서 계산한다.
- 폭 `x_span`과 높이 `y_span`이 각각 상한(폭 32, 높이 16)을 넘으면 **보드 대신 좌표 목록**을 출력하고 그 사유를 한 줄 남긴다. 스케일 축소를 하지 마라 (좌표 단위 의미가 정본에 확정되지 않았다).
- 말 표기: ally / enemy를 구분하는 문자, `active == false`는 별도 표기.
- cue는 말 옆 표식으로 붙인다. 5종 전부 표기 수단을 준다: Attack / Hit / Evade / BalanceBroken / Incapacitated. 정본의 연출 의미(전진·밀림·측면·기울어짐·흐려짐)를 텍스트 표식으로 대응시키고, 대응표를 코드 주석과 `TUI_Layout.md`에 남긴다.
- 보드 위에 `tick` 번호와 `tick_millis` 기반 경과 시간을 표시한다 (시뮬레이션 시간 = 화면 시간).
- pieces가 비면 보드를 그리지 않고 그 사실을 한 줄로 남긴다.

### 핵심 로그 섹션 (P3)

- `combat.view.core_log`만 문장화한다 (정본: 하단에는 핵심 로그만).
- `combat.view.full_log`는 문장화하지 않고 **개수만** 표시한다. 정본 07의 "전체 로그는 일시정지 또는 전투 종료 뒤 별도로 열람" — terminal 스냅샷에서 전체 로그 열람 UI는 이 slice 범위 밖이며, 존재와 개수만 알린다.
- 로그 줄 수 상한을 두고, 잘렸으면 **몇 줄이 생략됐는지 명시한다.** 조용한 truncation 금지.

### 보고서 섹션 (P4)

`combat.report`가 `Some`일 때만 그린다 (`None`이면 전투 진행 중 — 섹션 자체를 그리지 않는다).

- 승패(`outcome`), 사유(`reason`), 전투 시간(`duration_millis`), 생존/전투불능(`survivor_ids`/`defeated_ids`)
- 최대 피해 가한/받은 캐릭터: `top_damage_dealt_id`/`top_damage_taken_id`가 `None`이면 **그 줄을 아예 출력하지 않는다** (정본: 발생하지 않은 항목은 숨긴다)
- 캐릭터별 상세: `combatants`의 입힌 피해·받은 피해·처치 수·전투불능. 수치는 hundredths를 정수 반올림으로 표시
- **금지**: 전략 평가, 핵심 전환점, 자동 원인 분석, 전략 조언, 종합 MVP, 이전 전투 비교. 이런 문구·계산을 만들지 마라
- `simulation_version`을 표시한다. fingerprint를 표시할 경우 반드시 version과 같은 줄/인접에 둔다 (인덱스에 기록된 계약)

### 스냅샷 통합 (P5)

`render_scene_page_snapshot`(`snapshot.rs:3`)에서 `page.combat`이 `Some`일 때 위 섹션들을 넣는다. 위치는 정본 13의 "상단 전투 / 하단 로그"를 텍스트 순서로 반영한다: 보드 → 핵심 로그 → 보고서. 기존 섹션(제목·본문·선택지·로그) 순서와 내용은 바꾸지 마라. `page.combat`이 `None`이면 출력이 **바이트 단위로 기존과 동일**해야 한다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — 템플릿 테이블
6개 template id 매핑 + 알 수 없는 id fallback. 단위 테스트로 6개 전부와 fallback을 고정한다.
검증: `cargo fmt --all -- --check`, `cargo test -p escape-terminal`, `cargo test --workspace --no-fail-fast` → **322 유지**.

### WP-2 — 보드
최종 프레임 렌더, 상한 초과 시 좌표 목록 대체, cue 5종 표식, 빈 pieces 처리.
단위 테스트: 정상 보드, 상한 초과 대체, cue 5종 표기, 빈 프레임.

### WP-3 — 핵심 로그
`core_log` 문장화, `full_log` 개수 표시, 줄 수 상한과 생략 개수 명시.

### WP-4 — 보고서
위 규칙. 단위 테스트: 하이라이트가 `None`이면 줄이 없음, `Some`이면 있음, 캐릭터별 행 수, 금지 문구 미포함.

### WP-5 — 통합 + 회귀
`render_scene_page_snapshot` 통합. **`page.combat`이 `None`일 때 출력이 기존과 동일함을 테스트로 고정한다** (기존 스냅샷 회귀 방지).
검증: `cargo test -p escape-terminal` 전체(기존 `cli_smoke.rs` 61개 포함) 통과. 깨지면 `cli_smoke.rs`를 고치지 말고 **멈추고 출력 그대로 보고하라.**

### WP-6 — 문서 갱신 (생략 금지)
- `docs/dev/TUI_Layout.md`: terminal 관전 표시 계약 절 추가. 보드 표기·cue 대응표·로그 템플릿·보고서 항목·보드 상한 초과 시 대체 동작·금지 항목
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave3-step1d1-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 1d` 행을 1d-1(이 플랜)과 `(플랜 미작성) — 1d-2`로 분할
  - "현재 코드와 정본의 경계"에 terminal 렌더러 확보분을 적고, 아직 없는 것에 "Web 관전 렌더러·게이트 제거·5뷰포트 QA → 1d-2", "전체 로그 열람 UI"를 명시
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 한 줄씩 추가
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.** stale 수치는 갱신
- 각 문서 100KB 이하 유지

## 명시적 범위 밖

- Web Storybook 관전 렌더러, TS 타입, 게이트 플래그 제거, 5뷰포트 QA, wasm 재빌드 → 1d-2
- 전체 로그 열람 UI, 일시정지 흐름
- 배속·즉시 결과·자동 전투 UI (core enum은 이미 있음)
- 개입 기회/대응 제시 → Step 2c
- 프리셋·재도전
- `escape-core` 변경
- 치유·명줄·패배 결과
- 밸런스 수치

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-terminal
cargo test -p escape-core --test encounter_combat_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 322 passed + 신규 증가분 / 0 failed. 기존 테스트가 **하나도** 깨지지 않아야 한다.

## 최종 체크리스트

- [ ] `escape-core` 무변경, combat 함수 호출 0회
- [ ] `page.combat`이 `None`일 때 스냅샷 출력이 기존과 동일 (테스트로 고정)
- [ ] 로그 문장이 템플릿 테이블에서만 나온다, 알 수 없는 id는 fallback으로 보인다 (조용히 버리지 않음)
- [ ] 로그 줄 truncation 시 생략 개수를 명시한다
- [ ] 하이라이트가 `None`이면 해당 줄이 없다
- [ ] 금지 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교) 문구·계산 없음
- [ ] fingerprint를 표시하면 `simulation_version`이 함께 있다
- [ ] 보드 상한 초과 시 좌표 목록으로 대체하고 사유를 남긴다, panic 없음
- [ ] cue 5종 전부 표기 수단이 있고 대응표가 코드 주석과 `TUI_Layout.md`에 있다
- [ ] `HashMap` 순회 의존 없음
- [ ] `cli_smoke.rs` 무변경, `web/` 무변경, YAML·번들 무변경, 게이트 플래그 유지
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-6 문서 4개 갱신
- [ ] 보고서 `fable_combat_wave3_step1d1_report.md` 작성
- [ ] **보고서/커밋 메시지/python을 셸 heredoc으로 넘기지 말 것** — Write 툴로 파일에 쓰고 `git commit -F`, `python3 <파일>`로 실행 (이 세션에서 3회 유실 사고)
