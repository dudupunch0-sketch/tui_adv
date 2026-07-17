# Event Stage Wave 2 구현 보고서

## 범위와 기준

- 계획: `fable_eventstage_wave2_step1_2607171454.md`
- 계획 baseline: `origin/main` `fb718ed`; 실제 구현 시작 기준은 최신 `origin/main` `73c7c76`
- 목표: 이구학지 midgame 16개 사건을 staged content stream으로 전환해 Wave 1의 14/44에서 30/44로 확장
- 비범위: `wuxia_sado_*`, `wuxia_cheonoe_analysis_thread_phase1_bridge`, 모든 resolution/aftermath/return/settlement, `wuxia_collapse_gate`, 신규 art asset/manifest, office pack

## 구현 결과

### WP-S1 — index-time validation hardening

- `branch`가 `result`가 아닌 stage의 block에 있으면 bundle indexing을 실패시킨다.
- ResultStage의 branch 값은 기존처럼 `success`/`failure`만 허용한다.
- stage의 `next_stage_id`와 choice reference의 `next_stage_id`가 같은 event 안의 stage를 가리키는지 index-time에 검증한다.
- 기존 branch 없는 legacy bundle과 Wave 1 cursor/branch 동작은 유지한다.

### WP-C1/C2/C3 — 16개 사건 전환

전환한 사건은 다음과 같다.

1. `wuxia_mumyeong_first_confrontation`
2. `wuxia_mumyeong_copy_style_reveal`
3. `wuxia_mumyeong_reads_orthodox_style`
4. `wuxia_mumyeong_midgame_reunion`
5. `wuxia_boss_first_appearance`
6. `wuxia_mumyeong_request_for_aid`
7. `wuxia_mumyeong_awakening`
8. `wuxia_qingliu_attack_after_war`
9. `wuxia_mumyeong_destroys_orthodox_sect`
10. `wuxia_boss_recruits_mumyeong`
11. `wuxia_mumyeong_departure_truth_summary`
12. `wuxia_cheonggi_record_writing_sense`
13. `wuxia_cheonoe_pyeonrin_first_reward`
14. `wuxia_cheonoe_pyeonrin_second_reward`
15. `wuxia_seoharin_empty_place`
16. `wuxia_seoharin_left_meal`

각 사건은 authored Story → Choice → choice별 ResultStage 순서를 제공한다. 기존 choice ID와 action prefix, outcome log, flags, clues, checks, destination, Korean body를 유지하고, 기존 visual ID를 사용한 illustration block과 Korean alt를 추가했다. manifest에 매핑되지 않은 장면은 `placeholder: true`로 표시했다.

Wave 2의 16개 choice에는 check가 없어 success/failure branch fixture를 억지로 추가하지 않았다. `wuxia_cheonggi_record_writing_sense`의 기록이 쓰고 반응하는 문장은 `document` block으로 표현했고, 일반 narration에 `speaker: 천기록`을 설정하지 않았다. Web renderer의 기존 ruled/ink document styling을 그대로 사용해 별도 UI 기능은 추가하지 않았다.

## 검증 결과

- `cargo fmt --all`
- `cargo test -p escape-core --test event_stage --test event_stage_wave1 --test event_stage_wave2 --test route_parity` — 11 + 3 + 3 + 23 passed
- `cargo test --workspace` — 전체 통과
- `.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py` — 통과
- `python3 scripts/export_web_data.py --check` — web data up to date
- 두 wuxia preview bundle `--check` — up to date
- `git diff --check` — 통과
- `cd web && npm test && npx tsc --noEmit && npm run build` — 통과 (npm test에 art-asset gate 포함)
- `wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg` — 통과
- `npm run qa:storybook:visual -- --require-wasm` — 390×844, 414×896, 800×1440, 810×1644, 1440×1000 통과
- Wave 2 semantic guard — 16개 사건 staged entry/action ID/illustration/outcome log 보존 통과
- document guard — document block 존재 및 `천기록` narration speaker 부재 통과

## 수동 QA 상태

- [x] 자동 테스트로 ordered stream, direct ResultStage cursor, document block을 확인
- [x] 5 viewport visual/WASM QA와 document-block unit guard 통과
- [ ] 390px `mumyeong_first_confrontation → copy_style_reveal` 실제 플레이 및 typewriter 확인
- [ ] fragment reward 하나 선택 후 ResultStage 서사와 resolved flag 재진입 차단 확인
- [ ] `wuxia_cheonggi_record_writing_sense` document block 실화면 구분 확인
- [ ] Wave 2 ResultStage 안에서 save/reload cursor 복원 확인

Playwright Chrome 설치가 WSL 환경에서 완료되지 않아 위 네 가지 motion/screenshot 기반 수동 항목은 이번 closeout에서 실행하지 못했다. 다음 review에서 수행한다.

## 생성 산출물과 후속

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- 다음 구현 대상은 Wave 3의 최종전/결산/붕괴 게이트 별도 설계이며, art asset 병렬 트랙과 office pack은 계속 분리한다.
