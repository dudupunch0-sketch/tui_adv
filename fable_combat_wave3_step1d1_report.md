# Combat Wave 3 Step 1d-1 — 구현 보고서

작성: 2026-08-02 (coding subagent, sonnet, effort medium)
브랜치: `claude/combat-wave3-step1d1`
플랜: `fable_combat_wave3_step1d1_2608021329.md`

## 요약

`crates/escape-terminal`에 terminal(SuperLightTUI) 관전 렌더러를 추가했다. `ScenePage.combat`에 이미 있는 값만 포맷하고(`resolve_combat`/`conclude_combat`/`spectate_combat` 호출 0회), 게이트 플래그·`escape-core`·`cli_smoke.rs`·`web/`·YAML·번들은 무변경으로 유지했다. WP-1~WP-6을 순서대로 커밋 6개로 나눠 구현했다.

## WP별 변경 파일과 내용

### WP-1 — 로그 템플릿 테이블 (커밋 `56de229`)
- 변경: `crates/escape-terminal/src/snapshot.rs`
- `round_hundredths_to_int`(정수만 사용하는 반올림, 부동소수점 없음)와 `combat_log_template_line`(6개 `template_id` 매핑 + 미등록 id fallback)을 추가했다.
- 검증: `cargo fmt --all -- --check` 통과, `cargo test -p escape-terminal` 8 passed(신규) + 61 passed(cli_smoke, 무수정) = 69, `cargo test --workspace --no-fail-fast` 330 passed(322+8) / 0 failed.

### WP-2 — 체스말 보드 (커밋 `073296d`)
- 변경: `crates/escape-terminal/src/snapshot.rs`
- `combat_cue_symbol`(cue 5종 → 문자), `combat_piece_token`, `render_combat_board`(마지막 프레임, 32×16 상한 초과 시 좌표 목록 대체, 빈 프레임/빈 pieces 처리, `BTreeMap` 기반 결정론적 그리드)를 추가했다.
- 검증: `cargo test -p escape-terminal` 13 passed + 61 passed. `cargo fmt --all -- --check` 통과.

### WP-3 — 핵심 로그 (커밋 `6b8afbe`)
- 변경: `crates/escape-terminal/src/snapshot.rs`
- `render_combat_core_log`(`core_log`만 문장화, `full_log`는 개수만, 상한 `COMBAT_CORE_LOG_LIMIT=20` 초과 시 생략 줄 수 명시)를 추가했다.
- 검증: `cargo test -p escape-terminal` 15 passed + 61 passed.

### WP-4 — 보고서 (커밋 `aeb9832`)
- 변경: `crates/escape-terminal/src/snapshot.rs`
- `combat_outcome_label`/`combat_reason_label`/`render_combat_report`(하이라이트 `None`이면 줄 생략, 캐릭터별 상세, 금지 문구 없음, fingerprint는 simulation_version과 같은 줄)를 추가했다.
- 검증: `cargo test -p escape-terminal` 20 passed + 61 passed.

### WP-5 — 통합 + 회귀 (커밋 `5bb1a06`)
- 변경: `crates/escape-terminal/src/snapshot.rs`
- `render_combat_section`을 `render_scene_page_snapshot`의 두 분기(ordered content-stream / classic) 모두에서 `[최근 로그]` 직전에 호출하도록 배선했다. `page.combat`이 `None`이면 `render_combat_section`이 한 줄도 추가하지 않아 스냅샷이 이 slice 이전과 바이트 단위로 동일함을 `scene_snapshot_unchanged_bytes_when_combat_is_none`(+ `combat_section_adds_nothing_when_combat_is_none`)으로 고정했다.
- 검증: `cargo test -p escape-terminal` **24 passed + 61 passed(cli_smoke, 무수정) = 85**, `cargo test --workspace --no-fail-fast` **346 passed(322+24) / 0 failed**, `cargo test -p escape-core --test encounter_combat_wave3` 28 passed(무수정), `git diff --check` 통과.

### WP-6 — 문서 갱신 (커밋 `317a2e5`)
- 변경: `docs/dev/TUI_Layout.md`, `docs/design/Combat_System_Implementation_Plan_Index.md`, `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`
- `TUI_Layout.md`에 "전투 관전(Combat Spectator) 표시 계약" 절을 신설해 cue 대응표·템플릿 대응표·보드 상한/대체 규칙·보고서 규칙·전체 로그 범위 밖 사실을 기록했다.
- `Combat_System_Implementation_Plan_Index.md`: `status: wave3-step1d1-complete`, 단계 표의 미작성 "Wave 3 Step 1d" 행을 완료된 1d-1과 미작성 1d-2로 분할, "현재 코드와 정본의 경계"에 1d-1 확보분 문단과 1d-2/전체 로그 UI를 "아직 없는 것"에 명시.
- `Combat_System_Operating_Guide.md`/`Combat_System_Goal_Prompt.md`: 완료 항목 한 줄씩 추가, "아직 열지 않음"/non-goal 요약 갱신, Goal_Prompt는 다음 goal 문장도 Step 1d-2 기준으로 갱신(Step 1c 시절 문장은 이력으로 보존).
- 문서 크기(전부 100KB 이하): `TUI_Layout.md` 23,014B, `Combat_System_Implementation_Plan_Index.md` 24,021B, `Combat_System_Operating_Guide.md` 24,571B, `Combat_System_Goal_Prompt.md` 14,925B.

## cue 5종 → 텍스트 표식 대응표

`crates/escape-terminal/src/snapshot.rs`의 `combat_cue_symbol` 코드 주석과 `docs/dev/TUI_Layout.md`에 동일하게 기록됨.

| `CombatSpectatorCue` | 정본 연출 의미 | terminal 표식 |
|---|---|---|
| `Attack` | 짧은 전진/복귀 | `>` |
| `Hit` | 밀림/진동 | `<` |
| `Evade` | 측면 이동 | `~` |
| `BalanceBroken` | 흔들림/기울어짐 | `!` |
| `Incapacitated` | 흐려짐/표식 | `x` |

말 토큰 = 진영/생존 문자(`A`/`a`=아군 생존/비활성, `E`/`e`=적 생존/비활성) + cue 표식(Attack→Hit→Evade→BalanceBroken→Incapacitated 순서로 이어붙임). 보드 하단에 항상 범례 줄을 출력한다.

## template_id 6개 → 문장 형식 대응표

| `template_id` | 문장 형식 |
|---|---|
| `combat.log.move_intent` | `{actor} 이동 의도 (목표 {target})` — target 없으면 목표 생략 |
| `combat.log.target_selection` | `{actor} → 목표 지정: {target}` |
| `combat.log.collision` | `{actor} × {target} 충돌` |
| `combat.log.damage_applied` | `{actor} → {target} 피해 {value}` (hundredths 반올림 정수) |
| `combat.log.effect_applied` | `{actor} → {target} 효과 적용 [{effect_id}]` |
| `combat.log.effect_applied_hidden` | `{actor} → {target} 효과 적용 [정체불명]` (effect id 비공개) |
| (미등록 id) | `{actor} → {target} 알 수 없는 사건 [template_id={id}]` — fallback, 조용히 버리지 않음 |

## `page.combat`이 `None`일 때 무변경 증명

1. `render_combat_section`은 `let Some(combat) = &page.combat else { return; };`로 시작한다 — `None`이면 함수가 즉시 반환하고 `lines`에 아무것도 추가하지 않는다.
2. 단위 테스트 `combat_section_adds_nothing_when_combat_is_none`이 `render_combat_section`을 직접 호출해 결과 `lines`가 비어 있음을 고정한다(함수 단위 증명).
3. 단위 테스트 `scene_snapshot_unchanged_bytes_when_combat_is_none`이 전체 `render_scene_page_snapshot` 경로에서 `[전투 판]`/`[전투 로그]`/`[전투 보고서]` 마커가 전혀 없음을 확인하고, 같은 입력을 두 번 렌더링해 바이트 단위로 동일함(결정론)도 함께 고정한다.
4. `render_combat_section` 호출은 `render_scene_page_snapshot`의 두 분기 모두에서 기존 마지막 섹션(`[최근 로그]`) 바로 앞에 삽입됐을 뿐, 그 앞의 기존 라인 생성 로직은 한 글자도 바뀌지 않았다.
5. 간접 증거: `crates/escape-terminal/tests/cli_smoke.rs`(무수정, 항상 `combat: None`인 시나리오만 실행)의 기존 61개 테스트가 이 slice 적용 후에도 그대로 통과한다 — 실제 CLI 출력 회귀가 없음을 통합 수준에서 확인한다.

## 실제 렌더 결과 샘플 (게이트 플래그를 세운 상태)

`wuxia_combat_spectator_preview_bout`(게이트 `combat_spectator_preview_unlocked`, seed 2)를 직접 `GameState.flags`에 넣고 `scene_page_from_content` → `render_scene_page_snapshot`을 호출해 얻은 실제 출력의 일부다(임시 `#[ignore]` 테스트로 `cargo test -p escape-terminal snapshot::tests::__print_real_combat_sample -- --nocapture --ignored`를 실행해 확인한 뒤, 커밋 범위 밖이라 되돌렸다 — 아래는 그 실행에서 그대로 복사한 실제 텍스트다).

```text
[현재 인카운터]
관전용 표준 대련
[이야기]
연무장 한쪽에서 표준 규격의 대련이 시작된다. 두 사람 모두 같은 기수식과 같은 초식만을 익힌 처지라, 겨루기가 어떻게 끝날지는 아직
누구의 것도 아니다.
[일러스트]
[NO IMAGE] 연무장 한쪽에서 표준 규격의 대련 태세를 갖추는 두 수련생
[선택]
1. event:continue / 계속
[전투 판]
tick 10 · 경과 1000ms
y=   0: A><x E><x
표기: A/E=아군/적(생존) a/e=아군/적(비활성) · > 공격 · < 피격 · ~ 회피 · ! 균형붕괴 · x 전투불능
[전투 로그]
전체 로그 80건 (일시정지 또는 전투 종료 후 별도 열람, 이 화면은 개수만 표시)
- wuxia_spectator_bout_ally 이동 의도 (목표 wuxia_spectator_bout_challenger)
- wuxia_spectator_bout_ally → wuxia_spectator_bout_challenger 피해 13
- wuxia_spectator_bout_challenger 이동 의도 (목표 wuxia_spectator_bout_ally)
- wuxia_spectator_bout_challenger → wuxia_spectator_bout_ally 피해 13
  ... (총 20줄 표시) ...
- …(생략 20줄)
[전투 보고서]
시뮬레이션 버전: v1 · 지문: 997fee1a65a46b5a
결과: 양측 전멸
사유: 양측 전멸
전투 시간: 1100ms
생존: 없음
전투불능: wuxia_spectator_bout_ally, wuxia_spectator_bout_challenger
최대 피해를 가한 전투원: wuxia_spectator_bout_ally
최대 피해를 받은 전투원: wuxia_spectator_bout_ally
- wuxia_spectator_bout_ally: 가한 피해 133 · 받은 피해 133 · 처치 1 · 전투불능 예
- wuxia_spectator_bout_challenger: 가한 피해 133 · 받은 피해 133 · 처치 1 · 전투불능 예
[최근 로그]
- 아직 기록된 로그가 없다.
```

관찰: 양측 모두 마지막 tick에 `A><x`/`E><x`(공격+피격+전투불능) cue가 겹쳐 표시됐고, 최대 피해 하이라이트가 동점(양쪽 다 133)이라 `escape-core`의 "동점은 id 최소" 규칙에 따라 `wuxia_spectator_bout_ally`가 양쪽 다 선택됐다(이 renderer는 그 값을 그대로 옮겼을 뿐 재계산하지 않았다).

## 스킵/이탈 항목과 사유

- **`crates/escape-terminal/tests/cli_smoke.rs` 무수정** — 절대 규칙(다른 작업자의 uncommitted 변경). 읽기만 했고 git status에서 여전히 `M`(무단 미스테이지)으로 남아 있다.
- **Web Storybook 관전 렌더러, TS 타입, 게이트 플래그 제거, 5뷰포트 실화면 QA, wasm 재빌드** — 명시적 범위 밖(Step 1d-2 소관), 착수하지 않았다.
- **전체 로그(`full_log`) 열람 UI, 일시정지 흐름** — 명시적 범위 밖. terminal은 개수만 표시한다.
- **배속·즉시 결과·자동 전투 UI, 개입 기회/대응 제시, 프리셋·재도전, `escape-core` 변경, 치유·명줄·패배 결과, 밸런스 수치** — 모두 플랜의 "명시적 범위 밖" 목록대로 손대지 않았다.
- **임시 샘플-출력 테스트(`__print_real_combat_sample`)** — 실제 렌더 샘플을 얻으려고 추가했다가 실행 후 즉시 `git checkout`으로 되돌렸다. 커밋에는 포함되지 않았다(WP-5 커밋은 24개 테스트만 포함).
- **문서 재조직(P1~P5 production 함수 순서 정렬, 테스트 helper를 첫 사용 지점 옆으로 이동)** — 플랜에 명시되지 않았지만 WP당 커밋 1개 요구를 지키기 위해 `snapshot.rs` 내부 코드/테스트 순서를 P1→P5 순으로 정렬했다(로직 변경 없음, 매 정렬 후 fmt+테스트로 재검증).

## 최종 검증

```text
git status --short -uall
 M crates/escape-terminal/tests/cli_smoke.rs   (다른 작업자 변경, 무수정)
?? .claude/worktrees/caveman-repo-sync-8a6b94/  (다른 세션 산출물, 무관)
?? fable_combat_wave3_step1d1_2608021329.md     (이 plan 파일 자체, 예상 변경 파일 표 밖이라 미커밋)

git log --oneline -6
317a2e5 docs(combat): record Wave 3 Step 1d-1 terminal spectator renderer (WP-6)
5bb1a06 feat(escape-terminal): wire combat spectator sections into terminal snapshot (WP-5)
aeb9832 feat(escape-terminal): add combat conclusion report renderer (WP-4)
6b8afbe feat(escape-terminal): add combat core log renderer (WP-3)
073296d feat(escape-terminal): add combat board renderer (WP-2)
56de229 feat(escape-terminal): add combat log template table (WP-1)

cargo fmt --all -- --check   -> 통과 (출력 없음)
cargo test -p escape-terminal -> 24 passed (snapshot 단위 테스트) + 61 passed (cli_smoke, 무수정), 0 failed
cargo test -p escape-core --test encounter_combat_wave3 -> 28 passed, 0 failed (무수정)
cargo test --workspace --no-fail-fast -> 346 passed (322 baseline + 24 신규), 0 failed
git diff --check -> 통과
```

## 최종 체크리스트 자가 점검

- [x] `escape-core` 무변경, combat 함수 호출 0회
- [x] `page.combat`이 `None`일 때 스냅샷 출력이 기존과 동일 (테스트로 고정)
- [x] 로그 문장이 템플릿 테이블에서만 나온다, 알 수 없는 id는 fallback으로 보인다
- [x] 로그 줄 truncation 시 생략 개수를 명시한다
- [x] 하이라이트가 `None`이면 해당 줄이 없다
- [x] 금지 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교) 문구·계산 없음
- [x] fingerprint를 표시하면 `simulation_version`이 함께 있다
- [x] 보드 상한 초과 시 좌표 목록으로 대체하고 사유를 남긴다, panic 없음
- [x] cue 5종 전부 표기 수단이 있고 대응표가 코드 주석과 `TUI_Layout.md`에 있다
- [x] `HashMap` 순회 의존 없음 (`BTreeMap`/정렬된 `Vec` 사용)
- [x] `cli_smoke.rs` 무변경, `web/` 무변경, YAML·번들 무변경, 게이트 플래그 유지
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] `cargo test --workspace --no-fail-fast` 0 failed
- [x] WP-6 문서 4개 갱신
- [x] 보고서 `fable_combat_wave3_step1d1_report.md` 작성
