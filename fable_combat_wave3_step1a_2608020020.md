# Combat Wave 3 Step 1a — 관전 프레임 어댑터 (core 전용)

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)
승인 필요: 예 (`docs/dev/Combat_System_Goal_Prompt.md`의 "별도 작은 plan으로 설계하고 승인 후 진행")

## Baseline

- 기준 커밋: `f9035d2` (`origin/main`) 위의 `claude/eventstage-closeout-step1` (`4f7b181`, PR #178 OPEN)
- 이 slice는 PR #178과 파일이 겹치지 않는다. #178 머지 후 `origin/main`에서 새 브랜치로 시작하는 것을 권장한다.
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **249 passed / 0 failed** (2026-08-01 WSL 실측)
- combat 테스트 현황: `crates/escape-core/tests/combat_*.rs` 7파일 / 56 테스트

## 정본 근거

- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6) — 이 slice의 1차 정본
- [07. UI·템포·리스크](https://app.notion.com/p/36f37e69695e81258c60fc669f4d6800) — UI 원칙 (13을 따른다고 명시)
- 허브: [전투 시스템](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449). 충돌 시 하위 정본(13)을 우선한다.

정본에서 이 slice가 구현하는 문장:

1. "캐릭터는 체스말 형태로 표시한다. 공용 좌표 이동·진동·전후 움직임·플리커·감광과 기술/상태 이펙트로 자산 비용을 절감한다."
2. "공용 문법은 공격=짧은 전진/복귀, 피격=밀림/진동, 회피=측면 이동, 균형 붕괴=흔들림/기울어짐, 전투불능=흐려짐/표식이다."
3. "전투 중 하단에는 핵심 로그만 출력하되, 모든 공격·이동·판정은 전체 로그에 저장하여 일시정지 또는 전투 종료 뒤 열람할 수 있게 한다."
4. "로그는 자유 생성 문장이 아니라 등록된 사건 태그와 로그 템플릿을 사용한다."
5. "알려지지 않은 적 능력과 숨은 판정은 누설하지 않는다. 같은 seed와 사건이면 로그 순서·내용은 동일하다."
6. "연출은 판정 뒤 표현만 담당하여 seed·판정·AI 순서에 영향하지 않는다."

## Scope

`crates/escape-core/src/combat_spectator.rs` 신규 모듈 1개. `CombatResolutionResult`를 입력으로 받아 renderer가 그대로 그릴 수 있는 **관전 view**를 만든다. 새 판정을 하지 않는다.

- P1: tick별 체스말 프레임 (좌표·facing·활성 여부)
- P2: 공용 연출 cue (공격/피격/회피) — 판정 결과에서 **파생만** 한다
- P3: 템플릿 id 기반 이중 로그 (자유 문장 없음)
- P4: 누설 차단 (숨은 판정 값, 억제 사유, Hidden 효과 id)
- P5: fingerprint 체이닝

## Hard invariants (상속 — 위반 금지)

1. **renderer boundary**: 이 모듈은 `escape-core` 안에서만 산다. `ScenePage`·WASM·terminal·Web을 건드리지 않는다 (Step 1c/1d 소관).
2. **판정 재계산 금지**: seed를 새로 뽑지 않고, RNG를 호출하지 않고, 피해·명중·효과를 다시 계산하지 않는다. 입력 `CombatResolutionResult`에 이미 있는 값만 읽는다.
3. **기존 combat 모듈 수정 금지**: `combat_contract.rs`, `combat_state.rs`, `combat_opportunity.rs`, `combat_simulation.rs`, `combat_execution.rs`, `combat_resolution.rs`, `combat_conclusion.rs`를 수정하지 않는다. 기존 56개 테스트가 한 줄도 바뀌면 안 된다.
4. **additive-only**: `lib.rs`에 `mod`/`pub use` 추가만 한다. 기존 re-export 이름을 바꾸지 않는다.
5. **결정론**: 같은 입력 → 같은 출력. `HashMap` 순회 순서에 의존하지 않는다 (`BTreeMap`/정렬된 `Vec`만 사용). `fingerprint`는 canonical JSON 기반으로 기존 모듈과 같은 방식(FNV-1a, `{:016x}`)을 쓴다.
6. **밸런스 수치 금지**: 확정되지 않은 수치를 코드 상수로 넣지 않는다. cue 판정에 임의 임계값을 만들지 않는다.
7. **신규 의존성 금지**: `Cargo.toml`을 건드리지 않는다.
8. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs`는 읽기만. `.claude/worktrees/`는 읽지도 쓰지도 않는다.
9. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**` 수정 금지.

## 공개 API 계약

```rust
// crates/escape-core/src/combat_spectator.rs

/// 정본 13의 "공용 연출 문법". renderer가 이 종류만 보고 표현을 고른다.
/// 값은 판정 결과에서 파생되며, 이 enum이 애니메이션·CSS·색을 지정하지 않는다.
pub enum CombatSpectatorCue {
    Attack,   // 짧은 전진/복귀
    Hit,      // 밀림/진동
    Evade,    // 측면 이동
}

pub struct CombatSpectatorPiece {
    pub id: String,
    pub side: CombatSide,
    pub position: CombatPosition,
    pub facing: CombatFacing,
    pub active: bool,
    pub cues: Vec<CombatSpectatorCue>, // 정렬 고정: Attack < Hit < Evade, 중복 없음
}

pub struct CombatSpectatorFrame {
    pub tick: u32,
    pub pieces: Vec<CombatSpectatorPiece>, // id 오름차순 정렬 고정
}

/// 등록된 사건 태그 → 템플릿 id. 문장은 renderer가 소유한다.
pub struct CombatSpectatorLogEntry {
    pub tick: u32,
    pub sequence: u32,
    pub template_id: String,
    pub importance: CombatLogImportance,
    pub actor_id: String,
    pub target_id: Option<String>,
    pub value_hundredths: Option<i64>,
    pub effect_id: Option<String>,
}

pub struct CombatSpectatorView {
    pub resolution_fingerprint: String,
    pub tick_millis: u32,
    pub frames: Vec<CombatSpectatorFrame>,
    pub core_log: Vec<CombatSpectatorLogEntry>,
    pub full_log: Vec<CombatSpectatorLogEntry>,
    pub fingerprint: String,
}

pub enum CombatSpectatorError {
    UnknownParticipant(String),
    InvalidTickMillis(u32), // 0이면 화면 시간을 시뮬레이션 시간에 맞출 수 없다
}

pub fn spectate(request: &CombatSpectatorRequest)
    -> Result<CombatSpectatorView, CombatSpectatorError>;

pub struct CombatSpectatorRequest {
    pub resolution: CombatResolutionResult,
    pub participants: Vec<CombatSimulationParticipant>,
    pub catalog: CombatEffectCatalog, // 효과 visibility 조회용
    pub tick_millis: u32,             // 아래 주석 참고
}
```

**`tick_millis` 출처 (2026-08-02 리뷰에서 확정)**: `CombatResolutionResult`도 `CombatExecutionResult`도 입력 `CombatSimulationConfig`를 보관하지 않으므로 tick 길이를 결과에서 유도할 수 없다. 따라서 호출자가 시뮬레이션에 쓴 값을 `CombatSpectatorRequest.tick_millis`로 전달하고, `spectate()`가 그대로 view에 옮긴다. `0`은 `InvalidTickMillis`로 거부한다 — 정본 13의 "시뮬레이션 시간과 화면 시간은 항상 일치한다"를 renderer가 지킬 수 없는 값이기 때문이다. 고정 상수(`0` 등)를 view에 넣지 않는다.

`lib.rs`에는 `pub use combat_spectator::{spectate as spectate_combat, CombatSpectatorCue, CombatSpectatorError, CombatSpectatorFrame, CombatSpectatorLogEntry, CombatSpectatorPiece, CombatSpectatorRequest, CombatSpectatorView};`를 추가한다.

모든 struct/enum에 `Serialize, Deserialize`를 파생하고, `Option`/`Vec` 필드에는 `#[serde(default)]`를 붙인다.

## 파생 규칙 (결정론적, 임의 수치 없음)

### 프레임
- `frames[i].tick` = `resolution.execution.frames[i].tick`
- 좌표 = `resolution.execution.frames[i].positions[id]`. 해당 tick에 id가 없으면 그 조각을 프레임에서 뺀다 (에러 아님).
- `facing`, `side`, `active` = `participants`에서 id로 조회. 조회 실패는 `CombatSpectatorError::UnknownParticipant(id)`.

### cue (같은 tick의 `resolution.frames[i].outcomes`만 본다)
- `Attack`: 그 tick에 `outcome.actor_id == piece.id`인 outcome이 하나 이상
- `Hit`: `outcome.target_id == piece.id && outcome.hit && outcome.damage_hundredths > 0`
- `Evade`: `outcome.target_id == piece.id && outcome.in_range && !outcome.hit`
- 위 세 규칙 외의 cue를 만들지 않는다. cue는 중복 제거하고 enum 선언 순서로 정렬한다.

### 로그 (태그 → 템플릿 id 고정 표)
`resolution.full_log`(`CombatResolutionLogEvent`)와 `resolution.execution.full_log`(`CombatLogEvent`)를 합쳐 `tick` → `sequence` → 실행로그 우선 순서로 정렬한다.

| 원본 태그 | `template_id` |
|---|---|
| `CombatLogTag::MoveIntent` | `combat.log.move_intent` |
| `CombatLogTag::TargetSelection` | `combat.log.target_selection` |
| `CombatResolutionLogTag::Collision` | `combat.log.collision` |
| `CombatResolutionLogTag::DamageApplied` | `combat.log.damage_applied` |
| `CombatResolutionLogTag::EffectApplied` | `combat.log.effect_applied` |

`core_log`는 `full_log`에서 `importance >= CombatLogImportance::Important`인 항목만 남긴 부분집합이다 (기존 `combat_execution.rs`/`combat_resolution.rs`와 같은 규칙). 원래 순서를 유지한다.

### 누설 차단 (정본 5번 문장)
1. `CombatResolutionLogTag::AttackRoll` 이벤트는 관전 로그에 **넣지 않는다.** `roll_percent`는 숨은 판정이다.
2. `CombatResolutionLogTag::EffectSuppressed` 이벤트는 관전 로그에 **넣지 않는다.** 억제 사유는 미확인 적 능력을 누설한다.
3. `EffectApplied`의 `effect_id`가 `catalog`에서 `EffectVisibility::Hidden` 또는 `Conditional`인 정의를 가리키면 `effect_id`를 `None`으로 마스킹하고 `template_id`를 `combat.log.effect_applied_hidden`으로 바꾼다. `catalog`에 없는 id는 마스킹한다 (안전 기본값).
4. `value_hundredths`는 `DamageApplied`에서만 채운다. 나머지는 `None`.

## 명시적 범위 밖

- `BalanceBroken` / `Incapacitated` cue — **의도적 제외.** `CombatResolutionFrame`에 tick별 전투원 상태 스냅샷이 없어(`CombatResolutionState`는 최종 상태 1개) tick 단위로 균형 0 / 생명력 0 시점을 알 수 없다. adapter에서 델타를 재누적하면 `combat_resolution.rs`의 clamp 로직과 갈라질 수 있으므로, **`CombatResolutionFrame`에 per-tick 스냅샷을 추가하는 Wave 2 후속 slice**를 먼저 열고 그 뒤에 이 두 cue를 추가한다. 이 결정 사유를 구현 보고서에도 남긴다.
- 전투 종료 보고서 확장 (전투 시간, 캐릭터별 피해/치유/처치 수) → Step 1b
- `ScenePage` 필드 추가, WASM 노출 → Step 1c
- terminal/Web 렌더러, 상단 65~75% / 하단 25~35% 레이아웃, 색·아이콘 동기화 → Step 1d
- 로그 템플릿 **문장 자체** (renderer 소유). core는 `template_id`만 만든다.
- 프리셋 저장/재도전 유지, 우선 목표 규칙 (정본 13 후반부) → 별도 slice
- 배속·즉시 결과·자동 전투 정책 — 기존 `CombatRunMode`/`CombatPresentationSpeed`로 이미 표현됨. 이 slice에서 손대지 않는다.
- 시스템형/혼합형/각본형 구분 → Wave 3 Step 2
- 치유(healing)·명줄(life thread) — 파이프라인에 아직 개념이 없다. 정본 10번 확인 후 별도 slice.
- 밸런스 확정 수치

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — 타입 정의와 `lib.rs` 배선
`combat_spectator.rs`에 위 공개 API의 타입만 정의하고 `lib.rs`에 배선한다. `spectate`는 `todo!()`가 아니라 컴파일되는 최소 구현(빈 view 반환)으로 둔다.
검증: `cargo fmt --all -- --check`, `cargo test --workspace --no-fail-fast` (249 유지).

### WP-2 — 프레임 파생
좌표·facing·side·active 매핑과 `UnknownParticipant` 에러. cue는 아직 빈 Vec.
검증: 위 + 신규 테스트 파일에서 프레임 관련 케이스 통과.

### WP-3 — cue 파생
Attack/Hit/Evade 3규칙. 중복 제거·정렬 고정.

### WP-4 — 이중 로그와 템플릿 id
합병·정렬·`core_log` 필터·템플릿 표.

### WP-5 — 누설 차단
AttackRoll/EffectSuppressed 제외, Hidden/Conditional 마스킹, `catalog` 미등록 id 마스킹.
**이 WP는 테스트를 먼저 쓰고 red를 확인한 뒤 구현한다.** red 출력을 보고서에 남긴다.

### WP-6 — fingerprint
canonical JSON → FNV-1a `{:016x}`. `resolution_fingerprint`를 체인에 포함한다.

### WP-7 — 회귀 테스트
`crates/escape-core/tests/combat_spectator_wave3.rs` 신규. 최소 8케이스:
1. 같은 입력 두 번 → 동일 `fingerprint`와 동일 view (결정론)
2. 프레임 좌표·facing·side·active가 입력과 일치
3. 미등록 participant → `UnknownParticipant`
4. Attack/Hit/Evade cue가 각 규칙대로 붙고, 규칙 밖 cue가 생기지 않음
5. cue 정렬·중복 제거 고정
6. `core_log ⊆ full_log`이고 `importance >= Important`만 남음, 순서 유지
7. **AttackRoll·EffectSuppressed가 어느 로그에도 없음** (누설 차단)
8. Hidden/Conditional 효과 id가 마스킹되고 `template_id`가 `..._hidden`으로 바뀜, catalog 미등록 id도 마스킹
9. `participants` 입력 순서를 섞어도 view가 동일 (순서 무관)

### WP-8 — 문서 갱신 (마지막 WP, 생략 금지)
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave3-step1a-complete`
  - 단계 순서 표에서 Wave 3 Step 1 행을 **1a/1b/1c/1d로 분할**해 교체한다. 아직 플랜 파일이 없는 1b/1c/1d는 파일명 대신 `(플랜 미작성)`으로 표시한다. 존재하지 않는 `fable_combat_wave3_step1_2607261845.md` / `..._step2_...md` 참조를 제거한다.
  - "현재 코드와 정본의 경계"에 관전 adapter 확보분과 **아직 없는 것**(per-tick 상태 스냅샷, 보고서 확장, ScenePage/WASM/renderer, 프리셋, 치유·명줄)을 적는다.
- `docs/dev/Combat_System_Operating_Guide.md`에 이 slice의 플랜 파일명·구현 파일·테스트 파일과 테스트 개수를 기존 형식대로 추가한다.
- `docs/dev/Combat_System_Goal_Prompt.md`의 완료 목록에 한 줄 추가한다.
- `docs/dev/Development_Plan.md` 10번 항목(combat)에 Wave 3 Step 1a를 반영한다.
- 문서에 수치를 쓸 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.**
- 각 문서 100KB 이하 유지.

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_spectator_wave3
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test combat_conclusion_wave2
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 249 passed 유지 + 신규 테스트 증가분, 기존 combat 56 테스트 무변경. 실패는 숨기지 말고 출력 그대로 보고한다.

## 최종 체크리스트

- [ ] `combat_spectator.rs` 신규, 기존 combat 7모듈 무수정 (`git diff --stat`으로 확인)
- [ ] 기존 combat 테스트 파일 7개 무수정
- [ ] RNG 호출 0회, 판정 재계산 0회 (`grep`으로 `derive_seed`/`roll` 미사용 확인)
- [ ] `HashMap` 순회 의존 없음
- [ ] WP-5 red 출력 기록
- [ ] `cargo test --workspace --no-fail-fast` 249 + 신규분 / 0 failed
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] WP-8 문서 4개 갱신, 존재하지 않는 플랜 파일 참조 제거
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [ ] 보고서 `fable_combat_wave3_step1a_report.md`에 red→green 출력, 실행 명령, 스킵 항목과 사유(특히 BalanceBroken/Incapacitated 제외 사유) 기록
