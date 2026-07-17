# Event Stage Wave 3 구현 보고서

## 범위와 기준

- 계획: fable_eventstage_wave3_step1_2607171715.md
- 설계: docs/design/Event_Stage_Wave3_Design.md
- 구현 baseline: origin/main ca5c960
- 구현 브랜치: codex/eventstage-wave3-implementation
- 목표: 최종전·결산·귀환/정착 후일담·붕괴 게이트 14개를 ordered Event로 전환하고 이구학지 preview 44/44 Encounter에 event.stages를 제공
- 비범위: 새 전투 resolver/HP 규칙, epilogue body 계약·final_epilogue.rs, art asset·artManifest, 새 action/storage/schema, office pack

## 구현 결과

### WP-S1 — core 회귀와 Wave 3 guard

- crates/escape-core/tests/event_stage_wave3.rs를 추가했다.
- 14개 고정 목록, Story → Choice → per-choice ResultStage, illustration 1개/한국어 alt, legacy choice ID와 outcome log, check/branch 금지, 44/44 coverage를 검증한다.
- collapse StoryStage가 active staged Event보다 선점하지 않는지, 기사회생의 health +40·second_wind_used·재발동 차단, 안식 이후 wuxia_death_rest ending supersede를 검증한다.
- black_serpent_aftermath choice 이후 wuxia_final_epilogue_renderer_contract ending supersede를 canonical final_combat_result_battle_victory_seeded 포함 fixture로 고정했다.

### WP-C1/C2/C3 — 14개 authoring

모든 항목은 기존 top-level body, presentation, conditions, choices, action ID, outcome log, flags, clues, destination을 유지하고 event만 추가했다.

1. wuxia_sado_final_phase_1_price_tag
2. wuxia_cheonoe_analysis_thread_phase1_bridge
3. wuxia_sado_final_phase_2_weakpoint_control
4. wuxia_sado_final_phase_3_outside_calculation
5. wuxia_sado_battle_loss_route_bridge
6. wuxia_boss_resolution
7. wuxia_mumyeong_resolution
8. wuxia_seoharin_qingliu_resolution
9. wuxia_seoharin_unsaid_stay
10. wuxia_cheongirok_resolution
11. wuxia_black_serpent_aftermath
12. wuxia_return_modern_commute_scene
13. wuxia_settlement_stay_scene
14. wuxia_collapse_gate

일반 장면은 StoryStage에 기존 body와 presentation.visual_id illustration을 배치하고, manifest에 없는 그림은 placeholder true와 완전한 한국어 alt로 남겼다. ChoiceStage는 이벤트당 하나이며 모든 legacy choice ref가 해당 choice_id_result를 가리킨다. ResultStage는 result_summary와 기존 outcome.log를 포함한다. cheongirok_resolution은 speaker 없는 narration과 document block을 함께 사용한다. collapse_choice 순서는 revive first, rest last다.

## 생성 산출물

- crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
- web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
- WASM은 web/src/core/wasm-pkg에 재생성했으며 저장소 정책상 ignored generated output이다.
- art/epilogue renderer 파일은 변경하지 않았다.

## 자동 검증 결과

- cargo fmt --all -- --check — 통과
- cargo test -p escape-core --test event_stage --test event_stage_wave1 --test event_stage_wave2 --test event_stage_wave3 --test route_parity --test core_contract — 전체 통과 (8 Wave 3 tests 포함)
- cargo test --workspace --no-fail-fast — 통과
- .venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py — 72 passed
- python3 scripts/export_web_data.py --root . --check — 통과
- wuxia preview bundle --check — 두 generated bundle 모두 통과
- git diff --check — 통과
- cd web && npm test — 13 files / 72 tests passed, art-assets 7 mappings passed
- cd web && npx tsc --noEmit — 통과
- cd web && npm run build — 통과
- cd web && wasm-pack build ../crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg — 통과
- Storybook visual QA --require-wasm — 390x844, 414x896, 800x1440, 810x1644, 1440x1000 통과; report는 /tmp/tui_adv_wave3_visual/visual-qa-report.json

## 수동 QA와 보류

자동 five-viewport/WASM gate는 통과했다. 실제 플레이 중 typewriter·ResultStage save/reload·collapse revive 이후 정상 play·rest 이후 restart button·final ending supersede 시각 흐름은 이번 WSL 세션에서 수동 확인하지 못했다. 다음 Fable review에서 수행한다. Notion 원격 runtime ledger 연결은 현재 사용할 수 없어 baseline, 44/44 coverage, collapse/ending precedence, 수동 QA 보류를 idea_box/notion_sources.yml에 pending reverse-sync로 기록한다.

## 다음 단계

- Fable review에서 수동 four-flow acceptance를 확인한다.
- Notion 연결이 가능해지면 13번 runtime status page에 이 보고서와 ca5c960 baseline을 reverse-sync하고 pending 상태를 live_synced로 바꾼다.
- 새 art 제작, epilogue body contract, combat resolver, save/schema migration은 별도 승인 없이는 열지 않는다.
