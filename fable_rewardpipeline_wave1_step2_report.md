# 보상 파이프라인 웨이브 1 구현 보고서

- 기준: `fable_rewardpipeline_wave1_step1_2607171903.md`
- 구현 브랜치: `codex/reward-pipeline-wave1`
- 범위: WP-R1~R5 (획득·회차 도감·연출, 7개 청류문 수련기 사건)

## 구현 내용

- `GameState`에 `skills`, `titles`, `relationships`를 additive serde 필드로 추가했다.
- `OutcomeDef`에 스킬·칭호 지급, 관계 델타 필드를 추가하고 중복 스킬/칭호는 무시한다.
- `skills.yaml` 7종, `titles.yaml` 5종, 신규 기연 5종, 신규 아이템 7종을 정의했다.
- `wuxia_cheongryu_first_night_shelter`, `wuxia_cheongryu_first_breathing_lesson`, `wuxia_cheongryu_training_first_failure`, `wuxia_cheongryu_medicine_errand`, `wuxia_cheongryu_raid_omen`, `wuxia_cheongryu_gate_patrol_first_trouble`, `wuxia_seoharin_hides_training_injury`를 Story → Choice → ResultStage 형식으로 추가했다.
- 결정론적 수련기 사건을 사용해 기존 `wuxia_cheongryu_raid_route_split` 도달성을 보존했다. `gate_patrol_first_trouble` 확률화는 Wave B 유보 사항으로 남겼다.
- ScenePage/WASM 경계에 스킬·칭호 도감과 `reveal_immediate`를 전달하고, Web Storybook 드로어에 보통/희귀/전설 위계·지연 공개 마스킹·획득 비트를 추가했다.
- 기존 default office source는 변경하지 않았고, preview와 generated web bundle만 갱신했다.

## 산출물

- `crates/escape-core/src/{content,state,turn,scene_page}.rs`
- `crates/escape-core/tests/reward_pipeline_wave1.rs`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/{encounters,items,insights,skills,titles}.yaml`
- Rust/Web preview bundle 및 Web generated reward sections
- `web/src/ui/storybook/{render.ts,render.test.ts}`, `web/src/styles/storybook.css`

## 검증

- `cargo test -p escape-core --test content_bundle --test event_stage_wave1 --test event_stage_wave2 --test event_stage_wave3 --test reward_pipeline_wave1 --no-fail-fast`
- `.venv/bin/pytest -q tests/test_web_data_export.py`
- `python3 scripts/export_web_data.py --root . --check`
- `git diff --check`

기획 문서가 참조하는 Notion DB 원문 30행은 현재 repo에 snapshot으로 들어 있지 않아, ID와 획득 시점 규칙은 Wave 1 계획문서 기준으로 authoring했다. 원문 대조 후 이름·문구 변경분은 후속 reverse-sync로 처리한다.
