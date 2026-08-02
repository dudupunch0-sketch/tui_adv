# Combat Wave 3 Step 1c — ScenePage / WASM 관전 boundary

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step1a-v2` (`2a0e460`), `origin/main` = `3bb8ad5`보다 30 커밋 앞
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **287 passed / 0 failed** (2026-08-02 WSL 실측)
- combat 테스트 현황: `crates/escape-core/tests/combat_*.rs` 8파일 / 94 테스트

## 정본 근거

- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6): "화면 상단은 실제 실시간 전투 시뮬레이션, 하단은 전투 로그로 구성한다", "결과의 인과관계는 상단 전투와 하단 로그를 함께 보고 플레이어가 직접 읽는다"
- [07. UI·템포·리스크](https://app.notion.com/p/36f37e69695e81258c60fc669f4d6800): "전체 로그와 상세 수치는 일시정지 또는 전투 종료 뒤 별도로 열람한다"
- [03. 핵심 상태 시스템](https://app.notion.com/p/36f37e69695e81a9a36fcbe1df5b527f): "전투 기록에는 version을 저장한다", 결정성은 같은 simulation version 안에서만 보장
- 저장소 규칙: `docs/dev/Data_Schema.md`의 `ScenePage`는 renderer-ready semantic page이며 CSS class·pixel·Canvas command·DOM selector·image path를 담지 않는다.

## 이 slice가 하는 일

core가 만든 관전 view와 종료 보고서를 **`ScenePage`의 optional 필드 하나로 묶어 renderer 경계 밖으로 내보낼 수 있게** 한다. renderer는 판정·seed·AI·로그 순서를 재계산하지 않고 이 데이터를 표시만 한다.

전투를 시작하는 인카운터 authoring은 아직 없다(encounter schema에 전투를 여는 필드도, 전투 stage kind도 없다). 따라서 이 slice에서 `ScenePage.combat`은 **항상 `None`**이다. `Option`이므로 `None`은 "이 장면에는 전투가 없다"는 정직한 값이며, `skip_serializing_if`로 기존 JSON은 **바이트 단위로 동일**하게 유지된다. 실제 producer는 Wave 3 Step 2(authoring)가 만든다.

## Scope

- P1: `CombatSpectatorView`에 `simulation_version` 추가 (provenance에서 파생)
- P2: `CombatSpectatorPage` 정의 + `ScenePage.combat: Option<CombatSpectatorPage>` additive 추가
- P3: JSON 경계 회귀 테스트 (기존 JSON 무변경 증명 + WASM passthrough)
- P4: 문서 갱신 + 인덱스 단계 순서 조정

## Hard invariants (위반 금지)

1. **판정 재계산 금지, 새 판정 추가 금지.** 이 slice는 구조체 조립과 필드 노출만 한다. RNG 호출 0회.
2. **`scene_page_from_content`에 combat 생산 로직을 넣지 않는다.** `ScenePage.combat`은 이 slice에서 항상 `None`이다. 억지 producer를 만들지 마라.
3. **기존 `ScenePage` JSON을 바꾸지 않는다.** `#[serde(default, skip_serializing_if = "Option::is_none")]`을 반드시 붙여, combat이 `None`일 때 직렬화 결과에 `combat` 키가 **나타나지 않아야** 한다. 이것을 테스트로 고정한다.
4. **renderer 미접촉**: `crates/escape-terminal/`, `web/src/` 를 건드리지 않는다 (Step 1d 소관). `web/src/core/types.ts`도 손대지 않는다 — JSON이 안 바뀌므로 TS 변경이 필요 없다.
5. **`ScenePage`에 renderer-specific 데이터 금지**: CSS class, pixel 좌표, Canvas command, terminal color, DOM selector, image path를 넣지 않는다. 로그 문장도 넣지 않는다 — core는 `template_id`만 준다.
6. **fingerprint 계약 준수**: 노출되는 fingerprint 옆에는 항상 `simulation_version`이 함께 있어야 한다. 인덱스에 기록된 계약("fingerprint를 비교하는 consumer는 `simulation_version`도 함께 비교해야 한다")을 깨지 않는다.
7. **additive-optional**: `SAVE_SCHEMA_VERSION`·`CONTENT_BUNDLE_SCHEMA_VERSION`을 올리지 않는다.
8. **결정론**: `BTreeMap`/정렬 `Vec`만. `HashMap` 순회 의존 금지.
9. **신규 의존성 금지**, `Cargo.toml` 수정 금지.
10. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs` 읽기만. `.claude/worktrees/` 읽지도 쓰지도 않는다.
11. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**` 수정 금지.

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-core/src/combat_spectator.rs` | `simulation_version` 추가, `CombatSpectatorPage` 정의 |
| `crates/escape-core/src/scene_page.rs` | `ScenePage.combat` 필드 추가 (생성 지점 `:383` 한 곳에 `combat: None`) |
| `crates/escape-core/src/lib.rs` | `CombatSpectatorPage` re-export 추가 |
| `crates/escape-core/tests/combat_spectator_wave3.rs` | `simulation_version`·page 회귀 테스트 |
| `crates/escape-core/tests/scene_page_combat_boundary.rs` | **신규** — ScenePage JSON 무변경 + round-trip |
| `crates/escape-wasm/tests/json_contract.rs` | WASM 경계에 `combat` 키가 없음을 고정 (기존 테스트 본문 무수정, 신규 테스트 추가) |
| `docs/dev/Data_Schema.md` | `ScenePage.combat` 계약 기술 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·순서 조정·경계 갱신 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 공개 API 변경

```rust
// combat_spectator.rs

pub struct CombatSpectatorView {
    /// 이 기록을 만든 simulation version. `resolution.execution.provenance`에서 파생한다.
    /// fingerprint를 비교하는 consumer는 이 값도 함께 비교해야 한다 (정본 03).
    pub simulation_version: CombatSimulationVersion,
    pub resolution_fingerprint: String,
    pub tick_millis: u32,
    // ... 기존 필드 유지 ...
    pub fingerprint: String,
}

/// renderer가 표시할 관전 화면 한 장. 정본 13의 "상단 시뮬레이션 / 하단 로그"와
/// 종료 보고서를 renderer-neutral하게 담는다. 배치 비율·색·아이콘은 renderer가 정한다.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorPage {
    pub view: CombatSpectatorView,
    /// 전투가 아직 진행 중이면 `None` (발생하지 않은 항목은 숨긴다).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<CombatConclusionReport>,
}
```

```rust
// scene_page.rs
pub struct ScenePage {
    // ... 기존 필드 전부, 순서 유지 ...
    /// 이 장면에서 관전 중인 전투. 전투가 없으면 `None`이며 JSON에 키가 나타나지 않는다.
    /// renderer는 이 데이터를 표시만 하고 판정·seed·AI·로그 순서를 재계산하지 않는다.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combat: Option<CombatSpectatorPage>,
}
```

`lib.rs`에 `CombatSpectatorPage`를 re-export한다.

### `simulation_version`에 `#[serde(default)]`를 붙이지 않는 이유

`CombatSimulationVersion`은 빈 문자열을 거부하는 newtype이라 의미 있는 `Default`가 없다. `CombatSpectatorView`는 지금까지 저장·노출된 적이 없으므로 이 필드가 없는 기존 JSON이 존재하지 않는다. 따라서 **필수 필드로 둔다.** `Option`으로 감싸 "없을 수도 있는 척"하지 않는다.

## 파생 규칙

- `CombatSpectatorView.simulation_version` = `resolution.execution.provenance.as_ref().simulation_version.clone()`. provenance가 없으면 이미 `MissingProvenance`로 거부되므로 추가 분기가 필요 없다 — **기존 provenance 읽기 지점에서 같이 꺼낸다** (두 번 읽지 마라).
- `CombatSpectatorPage`는 조립만 한다. 파생 계산이 없다.
- `ScenePage.combat`은 `scene_page_from_turn_view`(`scene_page.rs:383`)에서 `None`으로 초기화한다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — `CombatSpectatorView.simulation_version`
provenance 읽기 지점에서 version도 함께 꺼내 view에 담는다.
검증: `cargo fmt --all -- --check`, `cargo test -p escape-core --test combat_spectator_wave3`, `cargo test --workspace --no-fail-fast`.
테스트 추가: view의 `simulation_version`이 입력 manifest의 값과 일치한다 (**red 먼저 확인**).

### WP-2 — `CombatSpectatorPage`와 `ScenePage.combat`
struct 정의, `lib.rs` re-export, `scene_page.rs:383`에 `combat: None`.
검증: 위 + `cargo test --workspace --no-fail-fast` → **287 + WP-1 신규분 유지** (기존 테스트가 하나도 깨지지 않아야 한다. 깨지면 `skip_serializing_if`가 빠졌을 가능성이 높다).

### WP-3 — JSON 경계 회귀 테스트
신규 파일 `crates/escape-core/tests/scene_page_combat_boundary.rs`:
1. `scene_page_from_content`로 만든 `ScenePage`의 `combat`이 `None`이다
2. 그 `ScenePage`를 `serde_json::to_value`로 직렬화한 결과에 **`"combat"` 키가 없다**
3. `combat`을 `Some(...)`으로 채운 `ScenePage`를 직렬화하면 `"combat"` 키가 있고, `combat.view.simulation_version`·`combat.view.fingerprint`가 함께 존재한다 (invariant 6)
4. `Some(...)` 상태의 `ScenePage`가 serde round-trip에서 동일하다
5. `combat` 키가 없는 기존 JSON을 `ScenePage`로 역직렬화하면 `combat`이 `None`이고 에러가 아니다
6. `CombatSpectatorPage.report`가 `None`이면 직렬화 결과에 `"report"` 키가 없다

`crates/escape-wasm/tests/json_contract.rs`에 신규 테스트 1개 추가: `scene_page_json`이 만든 페이지에 `combat` 키가 없다 (현재 producer가 없으므로 노출 사고를 막는 회귀 가드). **기존 테스트 본문은 수정하지 마라.**

### WP-4 — 문서 갱신 + 단계 순서 조정 (생략 금지)
- `docs/dev/Data_Schema.md`
  - `ScenePage` 관련 절에 `combat` 필드를 기술한다: optional, 없으면 키 자체가 없음, renderer는 표시만 함, 로그 문장이 아니라 `template_id`가 온다, fingerprint는 `simulation_version`과 함께만 비교한다.
  - `TurnView`/`ScenePage` 소유 표를 건드릴 필요는 없다.
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave3-step1c-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 1c` 행을 `fable_combat_wave3_step1c_2608021109.md`로 교체한다.
  - **단계 순서를 `1c → Step 2 → 1d`로 조정한다.** 근거를 한 줄로 남긴다: 전투를 시작하는 인카운터 authoring(Step 2)이 없으면 renderer(1d)가 표시할 데이터가 없다. 표에서 Step 2 행을 Step 1d 앞으로 옮긴다.
  - "현재 코드와 정본의 경계"에 1c 확보분(`ScenePage.combat` boundary)을 적고, **아직 없는 것**에 "전투를 시작하는 인카운터 authoring — `ScenePage.combat`의 producer가 없어 현재 항상 `None` → Wave 3 Step 2"를 명시한다.
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 기존 형식대로 한 줄씩 추가한다.
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.** 이미 적힌 수치가 stale해졌으면 갱신한다.
- 각 문서 100KB 이하 유지 (`Data_Schema.md`는 현재 약 33KB).

## 명시적 범위 밖

- 전투를 시작하는 인카운터 authoring, encounter schema의 전투 필드, 전투 stage kind → Wave 3 Step 2
- `web/src/core/types.ts` TS 타입, Web/terminal 렌더러, 상단 65~75% / 하단 25~35% 레이아웃, 색·아이콘 동기화 → Wave 3 Step 1d
- 로그 템플릿 문장 테이블 (renderer 소유)
- 프리셋 저장/재도전, 우선 목표 규칙
- 치유량·명줄·패배 결과 스키마
- 고급 다수전 AI·조기 결착·증원
- 배속·즉시 결과 UI (core enum은 이미 있음)
- 밸런스 확정 수치
- wasm 재빌드, 5뷰포트 QA (JSON이 바뀌지 않으므로 Web 화면 결과가 달라지지 않는다)

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_spectator_wave3
cargo test -p escape-core --test scene_page_combat_boundary
cargo test -p escape-core --test event_stage_branch_surfaces
cargo test -p escape-wasm --test json_contract
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 287 passed + 신규 증가분 / 0 failed. 기존 테스트가 **하나도** 깨지지 않아야 한다.

## 최종 체크리스트

- [ ] `ScenePage.combat`이 `None`일 때 직렬화 JSON에 `combat` 키가 없다 (테스트로 고정)
- [ ] `scene_page_from_content`에 combat producer를 넣지 않았다
- [ ] `CombatSpectatorView.simulation_version`이 provenance에서 한 번만 읽힌다 (중복 읽기 없음)
- [ ] `simulation_version`을 `Option`으로 감싸지 않았다
- [ ] `ScenePage`에 CSS/pixel/Canvas/DOM/image path·로그 문장이 없다
- [ ] `crates/escape-terminal/`·`web/src/` 무변경
- [ ] RNG 호출 0회, `HashMap` 순회 의존 없음
- [ ] WP-1 red 출력 기록
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-4 문서 4개 갱신, 인덱스 단계 순서가 `1c → Step 2 → 1d`로 조정됨
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [ ] 보고서 `fable_combat_wave3_step1c_report.md` 작성
- [ ] **보고서/커밋 메시지에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것** — Write 툴로 쓰고 `git commit -F <파일>`로 넘긴다
