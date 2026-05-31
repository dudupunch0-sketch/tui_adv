# Storypack runtime preview mode

Status: 결정 문서 + 첫 runtime preview 구현 완료

## Decision: separate preview mode first

첫 비-office runtime prototype은 기본 office runtime bundle에 바로 섞지 않고, **separate preview mode first**로 진행한다.

핵심 결정:

- `src/tui_adv/data/*.yaml` remains default office runtime content.
- `wuxia_jianghu_pack` enters runtime only through explicit preview bundle or preview flag.
- `escape-office` save/localStorage keys remain unchanged.
- no default bundle mixing: 기본 `content.bundle.json`과 Web generated bundle은 office 기본 콘텐츠를 유지한다.
- renderer는 `ScenePage`만 표시하고, world별 gameplay truth를 Web/SuperLightTUI renderer에서 재계산하지 않는다.

## 왜 gating이 아니라 preview mode인가

`world_id`/`storypack_id` gating을 기본 runtime schema에 즉시 넣으면 다음 문제가 생긴다.

1. 기존 office YAML, Rust content bundle, Web generated bundle, save/localStorage key를 한 번에 건드리게 된다.
2. 첫 무협 prototype은 아직 gameplay schema 확장보다 “기존 encounter schema로 표현 가능한가”를 확인하는 단계다.
3. 기본 번들의 `default_location`, route smoke, Web player start/save UX가 office 전제를 갖고 있으므로, 무협 콘텐츠를 같은 bundle에 넣으면 시작 위치와 encounter-first routing이 쉽게 충돌한다.

따라서 첫 단계는 별도 preview mode다. 이 결정은 gating을 영구히 포기한다는 뜻이 아니다. preview mode로 `wuxia_commute_rift_arrival` 또는 `wuxia_heuksa_bang_first_fight`가 기존 schema에서 잘 작동하는지 확인한 뒤, 다중 storypack 선택 UI/save migration이 필요해질 때 runtime-level gating을 연다.

## Preview mode contract

첫 runtime prototype은 다음 경계 안에서 구현한다.

1. 기본 office runtime은 그대로 둔다.
   - `src/tui_adv/data/*.yaml`은 계속 `escape from the office` 기본 콘텐츠다.
   - `crates/escape-core/fixtures/content/content.bundle.json`와 `web/src/data/generated/content.bundle.json`는 기본 office player artifact다.
2. 무협 prototype은 명시적 preview 입력으로만 열린다.
   - 예: `docs/content/storypack_db/`에서 고른 카드 1개를 별도 preview YAML/JSON fixture로 만들거나, 명시적 preview flag가 있는 export 경로에서만 bundle을 만든다.
   - preview path 이름에는 `wuxia_jianghu_pack` 또는 `storypack-preview`가 들어가야 한다.
3. preview bundle은 최소 metadata를 가진다.
   - `world_id: wuxia_jianghu`
   - `storypack_id: wuxia_jianghu_pack`
   - `runtime_mode: storypack_preview`
   - `default_location` 또는 opening scene이 office 기본 시작점과 구분되어야 한다.
4. renderer-neutral boundary를 유지한다.
   - Rust GameCore가 action eligibility/outcome/ending truth를 소유한다.
   - Web Storybook과 SuperLightTUI는 `ScenePage`와 action id만 표시/전달한다.
   - renderer는 `world_id`를 보고 gameplay branch를 계산하지 않는다.
5. save/key migration은 열지 않는다.
   - `escape-office` save/localStorage keys remain unchanged.
   - preview state가 필요하면 preview-only key 또는 disposable fixture로 제한하고, default player save와 자동 호환시키지 않는다.

## 구현된 첫 prototype

`wuxia_commute_rift_arrival`을 첫 schema-less runtime preview로 구현했다.

Preview source / artifacts:

- source YAML: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml`
- Rust fixture bundle: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- Web generated preview bundle: `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

Runtime metadata:

```yaml
runtime_mode: storypack_preview
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
default_location: wuxia_commute_rift
```

`escape-terminal --scene content`와 `escape-wasm::new_game_json()`은 preview bundle의 `runtime.default_location`을 사용해 `dev_desk`가 아니라 `wuxia_commute_rift`에서 새 게임을 시작한다. `runtime` metadata가 없는 기본 office bundle은 기존처럼 `dev_desk`에서 시작한다.

Preview smoke:

```bash
python scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check

cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_arrival
```

## 첫 prototype 후보

1. `wuxia_commute_rift_arrival` — 구현 완료
   - preview mode smoke에 가장 안전하다.
   - opening scene 자체가 office 기본 시작점과 분리되어야 한다는 contract를 잘 드러낸다.
   - 새 성장 schema 없이 flags/clues/items/logs로 표현 가능하다.
2. `wuxia_heuksa_bang_first_fight`
   - 기존 schema-less combat prototype 경험을 재사용할 수 있다.
   - 다만 전투/부상/구조 hook이 많아서 첫 preview fixture로는 arrival보다 약간 크다.

## 다음 구현 slice 기준

다음 slice는 같은 preview mode에서 `wuxia_heuksa_bang_first_fight`를 추가할지, 또는 preview launcher/UI wiring을 열지 결정하는 것이다. 둘 중 어느 쪽이든 다음을 금지한다.

- 기본 office bundle에 무협 encounter를 직접 추가하지 않는다.
- `ScenePage`에 world별 renderer field를 추가하지 않는다.
- `escape-office` save/localStorage key를 rename하지 않는다.
- 천기록/천외편린 3택 성장 schema를 열지 않는다.
- 실제 회사/통근 경로/사원증 정보 또는 private-only reality hint를 넣지 않는다.

검증은 최소 다음을 포함한다.

```bash
PYTHONPATH=src python -m pytest \
  tests/test_web_data_export.py \
  tests/test_docs_contract.py \
  tests/test_storypack_db.py \
  -q
python scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
cargo test -p escape-core --test content_bundle content_bundle_loads_optional_storypack_preview_runtime_metadata
cargo test -p escape-wasm json_boundary_uses_storypack_preview_default_location
cargo test -p escape-terminal content_tui_smoke_renders_wuxia_storypack_preview_arrival
python -m compileall -q src tests
cargo fmt --check
git diff --check
```
