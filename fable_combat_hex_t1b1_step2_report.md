# T1-b1 — 좌표계 교체 (Rust 측: core + terminal) — 완료 보고

plan: `fable_combat_hex_t1b1_step1_2608071921.md`
baseline: `240ab6b` (이 worktree의 HEAD 시작점)
head: `d6c4ed7`

## 1. 시작 baseline 확인

작업 시작 전 `cargo test --workspace --no-fail-fast`를 직접 실행해 **392 passed / 0 failed**를
확인했다(합산 검증: `14+10+10+32+12+23+10+21+8+9+32+29+11+3+3+3+8+4+23+5+24+61+37 = 392`).
계획서 §2가 요구한 조건과 정확히 일치해 그대로 진행했다.

## 2. WP별 커밋

| WP | 커밋 | 한 줄 요약 |
|---|---|---|
| (사전) | `ab8f16b` | 계획서 파일을 이 worktree/브랜치로 반입 (main 체크아웃에만 있고 여기 없었다) |
| WP1+WP2+WP3 (합침) | `d4ae660` | `CombatPosition`/`CombatFacing` 삭제 후 `HexCoord`로 교체 + 타일 단위 이동(`line()` 기반) + 사거리·충돌 판정을 육각 거리로 전환 |
| WP4 | `33200c9` | terminal 보드 렌더가 `(q, r)`을 읽도록 전환, 접근성 대체 표기 라벨도 `(q=, r=)`로 명시 |
| WP5 | `1c972fa` | `CURRENT_SIMULATION_VERSION` v2→v3, 저작 좌표/버전 이관, 번들 2종 재생성 |
| WP6 | `d6c4ed7` | §6 표의 7개 테스트 전부 추가 |

### WP1+WP2+WP3를 합친 이유

계획서 §6은 WP1을 "타입 교체와 컴파일 오류 해소만, 이동 로직은 기존 의미 유지 가능"으로
분리했다. 실제로 시도해보니 `CombatPosition::distance_squared`/`in_range`/`overlaps`가
`HexCoord`에 대응 메서드가 전혀 없어서, "기존 의미 유지"를 하려면 `q`/`r`을 마치 여전히
직교 `x`/`y`인 것처럼 취급하는 코드를 먼저 쓰고 WP2/WP3에서 곧바로 버려야 했다. 이건
진짜 중간 상태가 아니라 순수한 되돌림 작업이라고 판단해, 세 WP를 하나의 커밋으로
묶었다 — 커밋 메시지에 사유를 명시했다.

WP4(터미널)는 `escape-core`가 이미 컴파일되면 독립적으로 진행 가능해 분리 유지했다.
WP5(버전 bump)와 WP6(신규 테스트)도 계획서 그대로 분리했다.

## 3. 검증 명령과 실제 출력

```
$ cargo fmt --all -- --check
(출력 없음, exit 0)

$ cargo build --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.59s

$ cargo test -p escape-core --no-fail-fast
test result 합산: passed 277, failed 0 (21개 test binary + lib unittest, 전부 "0 failed")

$ cargo test -p escape-terminal
test result: ok. 24 passed; 0 failed ... (unittests)
test result: ok. 61 passed; 0 failed ... (cli_smoke.rs)

$ cargo test -p escape-core --test encounter_combat_wave3
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --workspace --no-fail-fast
test result 합산: passed 399, failed 0 (전체 27개 test binary/unittest/doc-test, 전부 "0 failed")

$ git diff --check
(출력 없음, exit 0)

$ grep -rn '"v2"' crates/ src/tui_adv/storypack-previews/
crates/escape-core/tests/encounter_combat_wave3.rs:312:        bundle_with_combat(|combat| combat["manifest"]["simulation_version"] = json!("v2"));
crates/escape-core/tests/encounter_combat_wave3.rs:316:    assert!(message.contains("v2"));
```

399 = 392(baseline) + 7(WP6 신규 테스트). **감소 없음, 신규 테스트 수만큼만 증가** — 계획서
§7이 기대한 그대로다.

번들 재생성 후 `python -m pytest tests/test_web_data_export.py`:

```
$ python3 -m pytest tests/test_web_data_export.py -q
............                                                             [100%]
12 passed in 4.24s
```

### `grep '"v2"'`가 비어 있지 않은 것에 대해 (계획과의 명시적 차이)

계획 §7/§11 체크리스트는 이 grep이 비어 있어야 한다고 적었다. 실제로는 WP6에서 추가한
`v2_authoring_is_rejected_after_the_bump` 테스트가 "v2"라는 문자열을 JSON 리터럴로
직접 구성해서 **T0의 index-time 강제 검증이 v2를 실제로 거부하는지**를 증명한다. 이건
"bump를 빠뜨린 저작이 남아 있다"는 grep이 원래 잡으려는 문제와는 다른 것이다 — 오히려
이 grep이 원래 존재하는 이유(누락된 bump를 잡는 기구가 실제로 동작하는지)를 직접
테스트로 고정한 것이다. `v9`를 쓰는 기존 `unsupported_simulation_version_is_rejected_at_index_time`
테스트만으로는 "말이 안 되는 값을 잡는다"는 것만 증명하지 "실제로 있었던 유효한 옛
버전을 잡는다"는 걸 증명하지 못해서, 이 슬라이스가 정확히 검증해야 하는 지점(T0가
v2→v3 bump를 실제로 지키는가)을 위해 의도적으로 추가했다. 우회하지 않고 여기 명시적으로
적는다.

## 4. §4-6의 두 테스트 — 값이 그대로인지 명시적 확인

**둘 다 값이 그대로였다.**

- `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`: 통과, 첫 명중
  피해 1333 hundredths 그대로.
- 조기 결착 8 tick 계열: 좌표 교체 **전**(디버그 출력으로 직접 측정)과 **후** 두 번 모두
  같은 인카운터(seed 4)를 직접 실행해 다음 값을 나란히 확인했다 — 어느 한쪽도 손으로
  기대값을 맞추지 않았다.

  | 항목 | 교체 전 | 교체 후 |
  |---|---|---|
  | `frames.len()` | 8 | 8 |
  | `decisive_tick` | `Some(8)` | `Some(8)` |
  | `outcome`/`reason` | `MutualDefeat`/`BothSidesDefeated` | `MutualDefeat`/`BothSidesDefeated` |
  | 명중 횟수 | 16 | 16 |
  | 명중당 피해 | 전부 1333 | 전부 1333 |
  | tick별 명중 | 1~8 tick 각 2회 | 1~8 tick 각 2회 |

  (fingerprint 문자열 자체는 값이 다르다 — position/facing 필드명과 simulation_version
  문자열이 직렬화 표현에 실리므로 당연하다. 판정 결과·타이밍은 완전히 동일하다.)

  이 측정은 임시 디버그 테스트(`eprintln!`)로 직접 실행한 뒤 커밋 없이 되돌렸고, 최종적으로
  같은 성질을 `authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap`
  (WP6, `d6c4ed7`)로 회귀 테스트에 고정했다.

메시지의 지시대로 — **둘 다 값이 바뀌지 않았으므로 계획의 사전 검산(§4-6)이 맞았다는
뜻이고, 기대값을 새 결과에 맞춰 고쳐 쓴 곳은 없다.**

## 5. `r ≠ 0`이라 거리 의미가 실제로 달라진 픽스처

**기존 픽스처 중에는 없다.** 저장소 전체에서 `HexCoord { q: .., r: .. }`를 직접 구성하는
모든 지점을 grep했고(`combat_hex.rs` 자신, `combat_hex_t1a.rs`의 범용 헬퍼 `c(q, r)` 제외),
이 슬라이스가 옮긴 기존 픽스처(테스트 파일 5종 + 저작 YAML 1건)는 **전부 `r = 0`**이었다
— 즉 옛 `y = 0`을 기계적으로 옮긴 것뿐이라 거동에 변화가 없다.

`r ≠ 0`을 쓰는 유일한 곳은 이 슬라이스에서 **새로** 작성한 테스트
`attack_range_is_measured_in_hex_distance`(`combat_resolution_wave2.rs`,
`(0,0)`↔`(3,-3)`)다. 이건 "이관 과정에서 의미가 바뀐 기존 픽스처"가 아니라, 육각 거리가
좌표를 단순 직교로 잘못 재해석한 값과 실제로 다르다는 것(3 vs ~4.24)을 **증명하기 위해**
의도적으로 만든 신규 커버리지다.

## 6. `cd web && npm test` 결과 (고치지 않음)

```
$ cd web && npm install && npm test
> escape-office-web@0.1.0 test
> npm run qa:art-assets && vitest run

art-assets: validated 7 in-scope mappings (title_hero excluded)

 Test Files  16 passed (16)
      Tests  165 passed (165)
```

**전부 통과.** 계획서 §9의 예측대로 — 전투가 `combat_spectator_preview_unlocked` 게이트
뒤에 있고, web 테스트가 손으로 만든 TS 픽스처를 쓰기 때문에 Rust 쪽 `{q,r}` 전환이 web
쪽 `{x,y}`(`CombatPoint`, `web/src/core/types.ts`)와 지금 실제로 불일치해도 아무 web
테스트도 이 불일치를 건드리지 않는다. T1-b2가 이 결과를 보고 계획을 쓰면 된다 — 여기서는
고치지 않았다.

## 7. 계획과 다르게 구현한 부분과 사유

1. **WP1/WP2/WP3를 한 커밋으로 합쳤다.** §2에서 상세.
2. **`grep '"v2"'`가 최종적으로 비어 있지 않다.** §3에서 상세 — WP6의
   `v2_authoring_is_rejected_after_the_bump` 테스트가 의도적으로 "v2" 리터럴을 담고
   있어서다.
3. **retreat(후퇴) 이동의 구현 방식이 계획 문구를 문자 그대로 따르지 않는다.** §4-3은
   "line()이 준 경로를 따라 최대 speed_per_tick 타일만큼 전진/후퇴한다"고만 적었다.
   `line(from, to)`는 두 끝점 *사이*만 보간하고 끝점 너머로 외삽하지 않으므로, "후퇴"에
   그대로 재사용할 수 없었다. `target`을 `actor` 기준으로 반사(reflect)한 가상의 점을
   만들고 그 점까지의 `line()`을 후퇴 경로로 쓰는 방식을 택했다 — 여전히 "line() 기반"이고
   결과적으로 옛 유클리드 `step_away`와 같은 방향·같은 거리를 만들어내지만, 계획 문서가
   명시하지 않은 설계 결정이므로 여기 남긴다. `combat_simulation.rs`의 `reflect_through`
   함수 주석에도 근거를 적었다.
4. **`CombatSimulationError`에 `HexMath(HexError)` 변종을 새로 추가했다.** `line()`이
   `Result`를 반환하므로 `?`로 전파할 대상이 필요했다. 계획서가 이 시그니처를 미리
   확정하지 않아서(§4-3은 "Result다, unwrap하지 말고 기존 CombatSimulationError로
   전파하라"고만 했다) 이렇게 채웠다.
5. **`CombatSimulationError::InvalidRange`가 이제 아무 데서도 생성되지 않는다.** 옛
   `CombatPosition::in_range`가 유일한 생성 지점이었는데 그 메서드 자체가 사라졌다.
   variant를 지우는 것은 계획이 요구하지 않은 별개의 API 축소라 그대로 남겨뒀다 —
   `CombatResolutionError::Simulation(CombatSimulationError)`도 같은 이유로 이제 생성
   지점이 없지만 그대로 뒀다.
6. **`combat_simulation_wave2.rs`의 기존 테스트 2개(`config_geometry_and_facing_validate`,
   `range_overlap_boundaries`)를 삭제하지 않고 새 API에 맞춰 다시 썼다.** 원래 테스트가
   호출하던 `CombatPosition` 메서드들이 사라졌으므로 그대로 둘 수 없었고, WP6의 전용
   facing 테스트가 이 두 테스트의 모든 성질을 덮지는 않아서(예: config 오류 케이스) 삭제
   대신 재작성을 택했다.

## 8. 최종 체크리스트 (계획 §11)

- [x] `CombatPosition`·`CombatFacing`이 트리에서 사라졌다 — `grep -rn "CombatPosition\|CombatFacing"`이
      `combat_hex.rs`의 설명 주석 한 줄(코드 아님, `HexCoord`를 대체한다는 문서 문장)과
      계획/보고서 문서들 외에는 아무 데도 나오지 않는다.
- [x] facing이 6방향 중 하나로 제한되고, 영벡터는 여전히 거부된다 —
      `facing_must_be_one_of_the_six_neighbor_directions`,
      `facing_zero_vector_is_still_rejected`가 고정한다.
- [x] `speed_per_tick`이 타일 수로 동작한다 — `speed_per_tick_moves_that_many_tiles`.
- [x] 사거리가 육각 거리로 측정된다 — `attack_range_is_measured_in_hex_distance`.
- [ ] `grep -rn '"v2"' crates/ src/tui_adv/storypack-previews/`가 비어 있다 — **비어 있지
      않다.** §3/§7-2에서 사유를 명시했다(의도적인 신규 회귀 테스트 리터럴이며, 미처리
      저작이 아니다).
- [x] 번들 2종이 **재생성**으로 갱신됐다 (손편집 흔적 없음) — `scripts/export_web_data.py
      --write` → `--check`만 사용, diff가 좌표/버전 필드만 바꾼 최소 diff임을 직접 확인했다.
- [x] §4-6의 두 테스트 값이 변하지 않았다 — §4에서 상세.
- [x] `combat_hex.rs` 무변경 — `git diff 240ab6b..HEAD -- crates/escape-core/src/combat_hex.rs`가
      빈 diff. `web/**` 무변경 — 유일한 예외는 재생성된 미리보기 번들 JSON 1개
      (`web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`)이며,
      이건 계획 §3의 소유 파일 목록이 "번들 2종"으로 명시적으로 허용한 대상이다(손편집
      아님, 스크립트 재생성). web의 손으로 짠 TS/테스트 코드는 전혀 건드리지 않았다.
- [x] 점유 미강제가 의도임이 주석에 남았다 — `combat_simulation.rs`의 `step_toward`/
      `step_away` 앞 블록 주석.
- [x] `cargo fmt --all -- --check` 통과, `git diff --check` 통과.
- [x] `cargo test --workspace --no-fail-fast` 0 failed, 392에서 감소 없음 — 399 (392 + 7).

## 9. 다음 슬라이스(T1-b2)를 위한 메모

- Rust는 육각(`{q,r}`), web은 아직 직교(`{x,y}`, `web/src/core/types.ts`의 `CombatPoint`)다.
  이건 계획대로 남겨둔 의도된 불일치다.
- `web && npm test`는 전부 통과하지만, 이건 web 테스트가 이 불일치를 검증하지 않기
  때문이다(§6) — "web이 멀쩡하다"는 뜻이 아니라 "아직 아무도 체크하지 않는다"는 뜻이다.
- T1-c(점유 강제)를 시작하기 전에 `step_toward`/`step_away`의 주석(점유 미강제가 왜
  의도인지)을 참고하면 된다.
