# Storypack preview UX QA handoff

Date: 2026-06-01
Scope: read-only UX QA snapshot for the currently implemented `wuxia_jianghu_pack` storypack preview.
Audience: future implementation or QA LLM sessions. Start here before changing preview UX, Web generated data loading, or terminal rendering.

## Source state checked

Initial commands requested by the user:

```bash
cd <repo-root>
git status --short --branch
git log --oneline -1
```

Observed at QA time:

```text
## docs/wuxia-notion-design-sync...origin/docs/wuxia-notion-design-sync
dedc983 docs: sync wuxia Notion design coverage
```

Later repo state may differ. Treat this document as a QA finding snapshot, not as the current source of truth for git state.

## QA scope

Checked surfaces:

- storypack preview selection UI
- `wuxia_jianghu_pack` preview entrypoint
- `wuxia_commute_rift_arrival`
- `wuxia_heuksa_bang_first_fight`
- `wuxia_cheonggi_record_first_fragment`
- terminal preview rendering
- Web preview generated data loading
- separation between default office play and preview play

Checked files and runtime surfaces included:

- `crates/escape-terminal/src/main.rs`
- `crates/escape-wasm/src/lib.rs`
- `crates/escape-core/src/scene_page.rs`
- `web/src/main.ts`
- `web/src/core/contentBundles.ts`
- `web/src/core/wasmRuntime.ts`
- `web/src/ui/startScreen.ts`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`

## Commands used for evidence

Use the tmp policy before Rust/Web checks on this repo:

```bash
source ~/.config/tui_adv/tmp-installs.sh
export PATH="$CARGO_HOME/bin:$PATH"
```

Representative QA commands:

```bash
cargo +stable run --locked -q -p escape-terminal -- \
  --scene content \
  --content-bundle crates/escape-core/fixtures/content/content.bundle.json \
  --seed 123 \
  --tui-smoke

cargo +stable run --locked -q -p escape-terminal -- \
  --scene content \
  --storypack-preview wuxia_jianghu_pack \
  --seed 123 \
  --tui-smoke

cargo +stable run --locked -q -p escape-terminal -- \
  --scene content \
  --storypack-preview wuxia_jianghu_pack \
  --seed 123 \
  --action choice:follow_roadside_dust \
  --action move:jianghu_market_street \
  --action choice:run_toward_open_street \
  --tui-smoke
```

Web/WASM direct smoke used during QA:

```bash
node --input-type=module - <<'NODE'
import fs from 'node:fs';
import { initSync, new_game_json, scene_page_json } from './web/src/core/wasm-pkg/escape_wasm.js';
initSync({ module: fs.readFileSync('./web/src/core/wasm-pkg/escape_wasm_bg.wasm') });
for (const [label, path] of [
  ['office', './web/src/data/generated/content.bundle.json'],
  ['wuxia-preview', './web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json'],
]) {
  const bundle = fs.readFileSync(path, 'utf8');
  try {
    const state = JSON.parse(new_game_json(123n, bundle));
    const page = JSON.parse(scene_page_json(JSON.stringify(state), bundle));
    console.log(`${label}: ok location=${state.location_id} title=${page.title} actions=${page.actions.map((a)=>a.id).join(',')}`);
  } catch (error) {
    console.log(`${label}: error ${String(error)}`);
  }
}
NODE
```

Observed result:

```text
office: ok location=dev_desk title=퇴사자의 메신저 actions=choice:check_message,choice:ignore_phone,choice:search_ex_employee
wuxia-preview: error unknown start location: dev_desk
```

Web unit tests could not be run in that session because `vitest` was unavailable:

```text
sh: line 1: vitest: command not found
```

Do not treat that as a product failure by itself. It only means the QA session did not have the Web test dependency environment ready.

## Pass findings

- Historical preview mode was explicit opt-in; current main/default storypack is 이구학지.
  - Terminal `--scene content` now defaults to the built-in `wuxia_jianghu_pack` fixture when no bundle is provided.
  - `--content-bundle` remains the explicit override for legacy office fixtures.
  - Unsupported preview ids fail with an available-id message.
  - Web start screen no longer needs a separate 이구학지 preview launcher because 이구학지가 default storypack이다.
- Legacy office play remains available by explicit bundle path and is not contaminated by wuxia default play.
  - Terminal office content starts at `dev_desk` with `퇴사자의 메신저`.
  - Legacy office generated bundle direct smoke starts at `dev_desk` and works with generated `wasm-pkg`.
  - Source guard: `new-game` and `continue` pass `storypackPreview: null`; preview saves are not written to the office save key.
- Terminal `wuxia_jianghu_pack` default/explicit entrypoint works.
  - Start location: `wuxia_commute_rift`
  - First title: `출근길 균열`
  - Stable terms: `사원증 / 출근복 / 천기록`
- The three-encounter order is coherent:
  1. `wuxia_commute_rift_arrival`
  2. `wuxia_heuksa_bang_first_fight`
  3. `wuxia_cheonggi_record_first_fragment`
- Cheonggi record content does not read like a search box or universal answer machine.
  - Text explicitly says it is not a search/reward box.
  - Actions choose a training direction or delay the choice; there is no free-form query UI.
- First fight content does not read like HP-only combat or musou fantasy.
  - Choices are survival/de-escalation/body-prep oriented.
  - `choice:crash_in_with_body` explicitly says it was not musou.
- Stable terminal action ids observed:
  - `choice:grip_employee_badge`
  - `choice:follow_roadside_dust`
  - `move:wuxia_commute_rift`
  - `move:jianghu_market_street`
  - `choice:run_toward_open_street`
  - `choice:deescalate_with_words`
  - `choice:swing_commute_bag`
  - `choice:loosen_tie_and_drop_shoes`
  - `choice:crash_in_with_body`
  - `choice:choose_guard_basics`
  - `choice:choose_keep_feet_moving`
  - `choice:choose_failure_log`
  - `choice:close_notebook_without_choice`

## Problems and suspected issues

| Severity | 위치 | 재현 | 예상 | 실제 |
|---|---|---|---|---|
| High | Web preview generated data loading / generated `wasm-pkg` | Call generated Web `new_game_json(123n, web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json)` | Runtime should use bundle `default_location: wuxia_commute_rift` and load `출근길 균열` | Fails with `unknown start location: dev_desk`. Office bundle works in the same generated WASM, so the generated `wasm-pkg` is likely stale relative to current Rust `escape-wasm` source. In browser preview this likely appears as a fatal preview load error after the loading page. |
| Medium | Terminal ScenePage body rendering | Run terminal `--storypack-preview wuxia_jianghu_pack --tui-smoke` and scripted fight/cheonggi snapshots | Encounter body should render once with clear speaker attribution | The same body appears once as `speaker: body` and again as a narration/body block. Long Korean text is also clipped/wrapped awkwardly at snapshot width. |
| Low | Terminal preview chrome | Open wuxia preview terminal snapshot | Preview should clearly signal storypack/wuxia mode without implying office content contamination | Top chrome still says `ESCAPE OFFICE // SuperLightTUI HORROR EDITION` and chapter label remains `격리 n턴`. Functionally separate, but UX can read as office/horror mixed with wuxia preview. |

## Must fix before next implementation session

1. Regenerate or otherwise align `web/src/core/wasm-pkg/` with current `crates/escape-wasm` source.
   - Gate: the Web direct smoke should print `wuxia-preview: ok location=wuxia_commute_rift title=출근길 균열`.
   - Also smoke the actual browser Preview 시작 flow when dependencies are available.
2. Remove duplicate encounter body rendering in ScenePage consumers.
   - Prefer one owner: either dialogue entries carry the text, or body blocks do, not both for the same source/text.
   - Add a regression for terminal snapshot or ScenePage JSON so speaker text is not duplicated.
3. Improve terminal Korean body layout/wrapping for preview snapshots.
4. Decide preview chrome policy.
   - Either keep office chrome intentionally and document why, or add clear `STORYPACK PREVIEW / wuxia_jianghu_pack` labeling.

## UX constraints to preserve

- 이구학지는 Web/terminal default storypack이다.
- Legacy office play, continue, save summary, and save key must not be changed by 이구학지 play.
- Preview play must not write office save state.
- Stable action ids are a cross-surface contract for terminal, Web, tests, and future QA automation.
- Cheonggi record is not a search box, chatbot, universal answer machine, or instant reward UI.
- First fight should stay survival-first: retreat, de-escalation, preparation, pain, failure learning. Avoid big-damage, special-move, or guaranteed-victory language.
- Modern anchors such as 사원증, 출근복, and 업무수첩 are useful identity anchors; office runtime content such as `dev_desk` or `퇴사자의 메신저` must never leak into preview runtime.
- Web preview validation must check both generated JSON and generated WASM package behavior. JSON parity alone is insufficient.

## Quick next-session checklist

Before coding, run:

```bash
git status --short --branch
source ~/.config/tui_adv/tmp-installs.sh
export PATH="$CARGO_HOME/bin:$PATH"
```

Then verify the two highest-risk points:

```bash
cargo +stable run --locked -q -p escape-terminal -- \
  --scene content \
  --storypack-preview wuxia_jianghu_pack \
  --seed 123 \
  --tui-smoke
```

and the Web generated WASM direct smoke from this document.

If the Web smoke still reports `unknown start location: dev_desk`, fix generated `wasm-pkg` freshness before doing additional preview UX work.
