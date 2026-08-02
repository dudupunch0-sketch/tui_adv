# Combat Wave 3 Step 2a — 인카운터 전투 스키마와 시스템형 producer

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step1a-v2` (PR #179, `origin/main` = `3bb8ad5`보다 35 커밋 앞)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **294 passed / 0 failed** (2026-08-02 WSL 실측)
- PR #179가 머지되면 최신 `origin/main` 위에서 이 slice를 시작하는 것을 권한다.

## 정본 근거

- [01. 전투 루프와 개입 예산](https://app.notion.com/p/36f37e69695e812c92efd2c11edabb66): "전투당 개입 기회 상한은 0~3회다. 상한은 인카운터 중요도·유형이 정하며 승률 예측은 상한을 정하지 않는다." / "**시스템형은 즉시 결과가 가능하다. 혼합형은 필수 선택까지 즉시 진행한 뒤 정지한다. 각본형은 즉시 결과가 불가하다.**"
- [12. 기술 기반 선택지·전투 기회 시스템](https://app.notion.com/p/3a837e69695e81d090b1f0503af26ebe): 하드 오류 목록 — "없는 태그 또는 effect 참조", "성공 또는 실패 effect bundle 누락", "seed 밖 난수 등 결정론 위반", "개입 예산 우회". 경고 목록 — "행동 선택지 4개 초과" 등.
- [04. 선택지 생성 규칙](https://app.notion.com/p/36f37e69695e81a090ebe5f63ab5932e): "시스템형은 공유 효과만 사용한다. 혼합형은 공유 효과와 1~2개의 특수 규칙을 함께 쓴다. 각본형은 커스텀 효과를 추가할 수 있으나 반드시 같은 효과 인터페이스로 들어온다."
- [03. 핵심 상태 시스템](https://app.notion.com/p/36f37e69695e81a9a36fcbe1df5b527f): 재시도 manifest 고정층에 "실제 전투 seed"가 들어간다. RNG namespace 분리.

## 이 slice가 하는 일

Wave 3 Step 1c에서 `ScenePage.combat`을 열었지만 **producer가 없어 항상 `None`**이다. 이 slice가 첫 producer를 만든다.

인카운터가 전투를 열 수 있게 content schema를 확장하고, **시스템형(즉시 결과 가능) 한 종류만** 실제로 구동한다. 혼합형·각본형은 개입 일시정지 흐름이 필요하므로 스키마로만 받고 **명시적 검증 오류로 거부**한다 — 조용히 무시하지 않는다 (정본 12 하드 오류 원칙).

콘텐츠 작성(실제 인카운터 3개 authoring)은 이 slice가 아니다. Step 2b/2c 소관이다.

## Scope

- P1: `EncounterCombatDef` 스키마 + `EncounterDef.combat` additive 필드
- P2: index-time 검증 (조용히 무시 금지)
- P3: 시스템형 producer — `ScenePage.combat`을 실제로 채운다
- P4: 회귀 테스트 + 문서

## Hard invariants (위반 금지)

1. **seed를 authoring 데이터에 두지 않는다.** `EncounterCombatDef`에 seed 필드를 만들지 마라. 실제 전투 seed는 core가 런 상태에서 결정론적으로 파생한다 (아래 파생 규칙). 콘텐츠가 seed를 고정하면 정본 03의 RNG namespace 분리와 재시도 계약이 깨진다.
2. **판정을 renderer로 내보내지 않는다.** producer는 `escape-core` 안에서 돈다. `crates/escape-terminal/`, `web/src/` 를 건드리지 않는다 (Step 1d 소관).
3. **조용히 무시 금지**: 알 수 없는 kind, 예산 초과, 참조 누락, 미지원 kind는 모두 명시적 오류다. 기본값으로 때우지 마라.
4. **개입 예산 상한 0~3** (정본 01). 4 이상은 검증 오류.
5. **밸런스 수치 금지**: 기술 비용·회복률·피해 계수를 코드 상수로 넣지 마라. 전부 authoring 데이터에서 온다.
6. **additive-optional**: `EncounterDef.combat`은 `Option` + `#[serde(default)]`. `CONTENT_BUNDLE_SCHEMA_VERSION`·`SAVE_SCHEMA_VERSION`을 올리지 않는다. 기존 bundle JSON이 그대로 읽혀야 한다.
7. **기존 fixture/generated bundle JSON 수정 금지**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**`. 테스트는 `serde_json`으로 전투를 **주입**한다 (`crates/escape-core/tests/event_stage.rs`가 이미 쓰는 방식).
8. **기존 `ScenePage` JSON 무변경 유지**: 전투가 없는 인카운터에서는 `combat` 키가 나타나지 않아야 한다. Step 1c의 boundary 테스트가 계속 통과해야 한다.
9. **결정론**: `BTreeMap`/정렬 `Vec`만. `HashMap` 순회 의존 금지. 같은 상태 + 같은 bundle → 같은 `ScenePage.combat`.
10. **신규 의존성 금지**, `Cargo.toml` 수정 금지.
11. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs` 읽기만. `.claude/worktrees/` 읽지도 쓰지도 않는다.

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-core/src/content.rs` | `EncounterCombatKind`/`EncounterCombatDef` 정의, `EncounterDef.combat`, raw 파싱, 검증 |
| `crates/escape-core/src/scene_page.rs` | 시스템형 producer |
| `crates/escape-core/src/lib.rs` | 새 타입 re-export |
| `crates/escape-core/tests/encounter_combat_wave3.rs` | **신규** — 스키마·검증·producer 회귀 |
| `docs/dev/Data_Schema.md` | 인카운터 전투 스키마 기술 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·경계 갱신 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 공개 API

```rust
// content.rs

/// 정본 04/01의 인카운터 유형. 즉시 결과 가능 여부가 다르다.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterCombatKind {
    /// 공유 효과만 사용. 즉시 결과 가능.
    Systemic,
    /// 공유 효과 + 1~2개 특수 규칙. 필수 선택까지 즉시 진행 후 정지.
    Mixed,
    /// 커스텀 효과 허용. 즉시 결과 불가.
    Scripted,
}

/// 인카운터가 여는 전투 정의. 기존 combat 타입을 그대로 재사용하며
/// **seed는 담지 않는다** — core가 런 상태에서 파생한다.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncounterCombatDef {
    pub kind: EncounterCombatKind,
    /// 개입 기회 상한. 정본 01 기준 0~3. 인카운터 중요도·유형이 정한다.
    pub intervention_budget: u8,
    /// `CombatSimulationInput`에서 seed를 뺀 나머지.
    pub manifest: CombatManifest,
    pub state: CombatState,
    pub config: CombatSimulationConfig,
    pub participants: Vec<CombatSimulationParticipant>,
    pub roles: Vec<CombatRolePreset>,
    #[serde(default)]
    pub policies: Vec<CombatTargetPolicy>,
    pub attacks: Vec<CombatAttackDefinition>,
    pub defenses: Vec<CombatDefenseProfile>,
    pub effect_catalog: CombatEffectCatalog,
    /// 이번 전투에서 진행할 tick 수. `config.max_ticks` 이하여야 한다.
    pub ticks: u32,
    pub termination: CombatTerminationPolicy,
}

pub struct EncounterDef {
    // ... 기존 필드 유지 ...
    #[serde(default)]
    pub combat: Option<EncounterCombatDef>,
}
```

`RawEncounterDef`에도 `#[serde(default)] combat: Option<EncounterCombatDef>`를 추가하고 `EncounterDef`로 넘긴다.

`lib.rs`에 `EncounterCombatKind`, `EncounterCombatDef`를 re-export한다.

## 파생 규칙

### seed 파생 (invariant 1)
`CombatManifest.actual_seed`가 authoring 데이터에 들어 있으므로, producer는 그것을 그대로 쓰지 않고 **런 상태와 인카운터 id를 섞은 값으로 덮어쓴다.**

- `CombatSimulationInput.seed` = `manifest.derived_seed(CombatRngNamespace::ActualCombat)`을 기반으로, `state.seed`(런 seed)와 `encounter.id`를 함께 해싱한 값
- 구현: `content.rs`/`scene_page.rs`에 이미 있는 FNV 헬퍼와 같은 방식으로 `(run_seed, encounter_id, manifest.fingerprint()?)`를 해싱한다. **새 난수원을 도입하지 마라.**
- 같은 런 seed + 같은 인카운터 + 같은 manifest → 같은 전투 seed. 다른 런 seed → 다른 전투.
- `CombatManifest.actual_seed`도 같은 값으로 맞춰 넣어 manifest와 실제 seed가 어긋나지 않게 한다.

### 시스템형 producer
현재 인카운터가 `combat`을 갖고 `kind == Systemic`이면 `scene_page_from_turn_view`에서:

1. `CombatSimulationInput`을 조립 (위 seed 파생 적용)
2. `execute_combat` → `resolve_combat` → `conclude_combat` → `spectate_combat`
3. `ScenePage.combat = Some(CombatSpectatorPage { view, report: Some(report) })`

파이프라인 중 어느 단계라도 실패하면 **`ScenePage.combat`을 `None`으로 두지 말고** `ContentTurnError`로 전파한다. 조용한 실패는 invariant 3 위반이다.

`kind`가 `Mixed`/`Scripted`면 producer를 돌리지 않는다 — index 검증에서 이미 거부되므로 이 지점에 도달하지 않는다. 방어적으로 도달했다면 오류를 반환한다.

**비용 주의**: 이 producer는 `scene_page_from_content` 호출마다 전투를 재실행한다. 결정론적이라 결과는 같지만 낭비다. 이 slice에서는 그대로 두고, 보고서에 "매 렌더 재실행 비용, 캐싱은 후속 slice" 를 기록한다. 인위적 캐시를 만들지 마라 (상태 저장은 save schema 결정이 필요하다).

### index-time 검증 (정본 12 하드 오류)
`validate_content_bundle` 또는 encounter 검증 경로에 추가한다. 아래는 모두 **오류**다.

1. `intervention_budget > 3` (정본 01 상한)
2. `kind`가 `Mixed` 또는 `Scripted` — 아직 미지원. 오류 메시지에 "Wave 3 Step 2b/2c 소관"임을 남긴다
3. `config.tick_millis == 0`
4. `ticks == 0` 또는 `ticks > config.max_ticks`
5. `attacks`의 `actor_id`가 `state.combatants`에 없음
6. `defenses`의 `combatant_id`가 `state.combatants`에 없음
7. `state.combatants`에 있는데 `defenses`에 없는 전투원 (기존 `combat_resolution.rs`의 검증과 같은 규칙)
8. `participants`의 id 집합이 `state.combatants`의 id 집합과 다름
9. `effect_catalog.validate()` 실패
10. `manifest.validate()` 실패
11. `attacks`의 `effects`가 `effect_catalog`에 없는 effect id 참조 (정본 12 "없는 태그 또는 effect 참조")

새 오류 변형은 기존 `ContentIndexError`/`ContentBundleError` 스타일을 따라 추가한다. 오류 메시지에 인카운터 id를 포함한다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — 스키마
`EncounterCombatKind`, `EncounterCombatDef`, `EncounterDef.combat`, `RawEncounterDef.combat`, `lib.rs` re-export.
검증: `cargo fmt --all -- --check`, `cargo test --workspace --no-fail-fast` → **294 유지** (기존 bundle JSON이 그대로 읽혀야 한다. 줄면 멈추고 보고).

### WP-2 — index-time 검증 (테스트 red 먼저)
위 11개 규칙. 각 규칙마다 테스트를 먼저 쓰고 red를 확인한 뒤 구현한다. red 출력을 보고서에 남긴다.

### WP-3 — 시스템형 producer (테스트 red 먼저)
seed 파생 + 파이프라인 + `ScenePage.combat` 채우기.

### WP-4 — 회귀 테스트 (`crates/escape-core/tests/encounter_combat_wave3.rs` 신규)
office fixture(`crates/escape-core/fixtures/content/content.bundle.json`)를 `serde_json`으로 읽어 한 인카운터에 최소 전투를 **주입**해 테스트한다. **fixture 파일 자체는 수정하지 마라.**

최소 케이스:
1. 전투가 없는 인카운터의 `ScenePage.combat`이 `None`이고 JSON에 `combat` 키가 없다 (Step 1c 계약 유지)
2. 시스템형 전투를 주입하면 `ScenePage.combat`이 `Some`이고 `view.frames`가 비어 있지 않다
3. `report`가 `Some`이고 `duration_millis > 0`, `combatants` 행이 전투원 수와 같다
4. 같은 상태로 두 번 호출하면 `ScenePage.combat`이 **완전히 동일**하다 (결정론)
5. 런 seed가 다르면 전투 seed가 달라진다 — `view.fingerprint`가 다르다
6. authoring의 `manifest.actual_seed`를 바꿔도 실제 전투 seed는 런 상태에서 파생되므로 결과가 authoring seed에 좌우되지 않는다 (invariant 1 증명)
7. `intervention_budget = 4` → 검증 오류
8. `kind = mixed` → 검증 오류, 메시지에 인카운터 id 포함
9. `kind = scripted` → 검증 오류
10. `attacks[0].actor_id`를 없는 id로 바꾸면 검증 오류
11. `effect_catalog`에 없는 effect id를 attack effects에 넣으면 검증 오류
12. `ticks > config.max_ticks` → 검증 오류
13. `combat` 필드가 없는 기존 bundle JSON이 그대로 인덱싱된다 (additive 증명)

### WP-5 — 문서 갱신 (생략 금지)
- `docs/dev/Data_Schema.md`: 인카운터 전투 스키마 절 추가. 필드 표, **seed를 authoring에 두지 않는 이유**, 검증 규칙 목록, 시스템형만 지원한다는 현재 상태.
- `docs/design/Combat_System_Implementation_Plan_Index.md`:
  - `status:` → `wave3-step2a-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 2` 행을 2a/2b/2c로 분할한다. 2a는 이 플랜 파일명, 2b(시스템형 authoring 1개)·2c(혼합형+각본형 authoring과 개입 일시정지 흐름)는 `(플랜 미작성)`.
  - "현재 코드와 정본의 경계"에 producer 확보분을 적고, 아직 없는 것에 "혼합형·각본형 개입 일시정지 흐름", "전투 결과 저장(매 렌더 재실행 중)"을 명시한다.
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 한 줄씩 추가한다.
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.** stale해진 수치는 갱신한다.
- 각 문서 100KB 이하 유지 (`Data_Schema.md` 현재 약 35KB).

## 명시적 범위 밖

- 실제 인카운터 콘텐츠 authoring (시스템형 1개 → Step 2b, 혼합형·각본형 → Step 2c)
- 혼합형·각본형의 개입 일시정지 흐름, 필수 선택, 즉시 결과 정책 → Step 2c
- 개입 기회/대응 제시 (`combat_opportunity.rs`와 encounter의 연결), 행동 선택지 최대 4개 + "개입하지 않는다" → Step 2c
- 전투 결과 캐싱·save 저장 (매 렌더 재실행을 이 slice에서는 허용)
- terminal/Web 관전 렌더러 → Step 1d
- 치유·명줄·패배 결과 스키마
- 고급 다수전 AI, 조기 결착, 증원, 패주, 항복, 대형, 결속, 배경 전투
- 프리셋·재도전·배속 UI
- 밸런스 확정 수치
- wasm 재빌드, 5뷰포트 QA (기존 bundle에 전투가 없으므로 Web 화면 결과가 달라지지 않는다)

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test encounter_combat_wave3
cargo test -p escape-core --test scene_page_combat_boundary
cargo test -p escape-core --test content_bundle
cargo test -p escape-core --test event_stage
cargo test -p escape-wasm --test json_contract
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 294 passed + 신규 증가분 / 0 failed. 기존 테스트가 **하나도** 깨지지 않아야 한다.

## 최종 체크리스트

- [ ] `EncounterCombatDef`에 seed 필드가 없다
- [ ] 전투 seed가 런 상태 + 인카운터 id + manifest fingerprint에서 파생된다 (테스트 6번으로 증명)
- [ ] 새 난수원(`rand` 등)을 도입하지 않았다
- [ ] `Mixed`/`Scripted`가 조용히 무시되지 않고 명시적 오류다
- [ ] `intervention_budget > 3`이 오류다
- [ ] 전투 없는 인카운터의 JSON에 `combat` 키가 없다 (Step 1c 계약 유지)
- [ ] fixture/generated bundle JSON 무변경 (테스트는 주입 방식)
- [ ] `crates/escape-terminal/`·`web/src/` 무변경
- [ ] `HashMap` 순회 의존 없음
- [ ] WP-2·WP-3 red 출력 기록
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-5 문서 4개 갱신, Step 2가 2a/2b/2c로 분할됨
- [ ] 보고서에 "매 렌더 전투 재실행 비용, 캐싱은 후속" 기록
- [ ] `cli_smoke.rs`·`.claude/worktrees/` 무변경
- [ ] 보고서 `fable_combat_wave3_step2a_report.md` 작성
- [ ] **보고서/커밋 메시지에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것** — Write 툴로 쓰고 `git commit -F <파일>`로 넘긴다
