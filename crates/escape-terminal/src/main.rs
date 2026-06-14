use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game,
    new_game_from_content_at, scene_page_from_content, turn_view, turn_view_from_content,
    ActionView, BlockedActionView, BodyBlock, ContentBundle, ContentIndex, EffectCue, GameState,
    SceneAction, SceneBlockedAction, SceneEffectCue, SceneMode, ScenePage, TurnView,
    DEFAULT_START_LOCATION_ID,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_STORYPACK_ID: &str = "wuxia_jianghu_pack";
const DEFAULT_STORYPACK_BUNDLE_REL: &str =
    "../escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json";

#[derive(Debug, PartialEq, Eq)]
struct CliOptions {
    scene: String,
    seed: u64,
    smoke: bool,
    tui_smoke: bool,
    app_smoke: bool,
    play: bool,
    app: bool,
    tick: u64,
    content_bundle: Option<PathBuf>,
    storypack_preview: Option<String>,
    actions: Vec<String>,
}

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("error: {error}");
        std::process::exit(2);
    }
}

fn run<I>(args: I) -> Result<(), String>
where
    I: IntoIterator<Item = String>,
{
    let options = parse_args(args)?;
    if options.smoke && options.tui_smoke {
        return Err("--smoke and --tui-smoke cannot be combined".to_string());
    }
    if options.app_smoke && (options.smoke || options.tui_smoke) {
        return Err("--app-smoke cannot be combined with --smoke or --tui-smoke".to_string());
    }
    if options.play && (options.smoke || options.tui_smoke || options.app_smoke) {
        return Err("--play cannot be combined with smoke modes".to_string());
    }
    if options.app && (options.smoke || options.tui_smoke || options.app_smoke || options.play) {
        return Err("--app cannot be combined with --play or smoke modes".to_string());
    }
    if (options.play || options.app) && !options.actions.is_empty() {
        return Err(
            "interactive modes cannot be combined with scripted --action values".to_string(),
        );
    }
    if options.tick != 0 && !options.app_smoke {
        return Err("--tick is only supported with --app-smoke".to_string());
    }
    if options.content_bundle.is_some() && options.storypack_preview.is_some() {
        return Err("--content-bundle and --storypack-preview cannot be combined".to_string());
    }

    match options.scene.as_str() {
        "printer" => run_printer_scene(&options),
        "content" => run_content_scene(&options),
        other => Err(format!(
            "unsupported scene '{other}'; available scenes: printer, content"
        )),
    }
}

fn run_printer_scene(options: &CliOptions) -> Result<(), String> {
    if options.content_bundle.is_some() {
        return Err("--content-bundle is only supported with --scene content".to_string());
    }
    if options.storypack_preview.is_some() {
        return Err("--storypack-preview is only supported with --scene content".to_string());
    }
    if !options.actions.is_empty() {
        return Err("--action is only supported with --scene content".to_string());
    }
    if options.play {
        return Err("--play is only supported with --scene content".to_string());
    }

    let state = new_game(options.seed);
    let view = turn_view(&state);
    if options.tui_smoke {
        print_tui_snapshot(&view, &state, &view.location_id, &[]);
    } else if options.app_smoke || options.app {
        return Err("--app and --app-smoke are only supported with --scene content".to_string());
    } else {
        print_turn(&view, &state, &options.scene, options.smoke, false);
    }
    Ok(())
}

fn run_content_scene(options: &CliOptions) -> Result<(), String> {
    let bundle_path = selected_content_bundle_path(options)?;
    let json_text = std::fs::read_to_string(&bundle_path).map_err(|error| {
        format!(
            "failed to read content bundle '{}': {error}",
            bundle_path.display()
        )
    })?;
    let bundle = load_content_bundle(&json_text).map_err(|error| error.to_string())?;
    let content = index_content_bundle(&bundle).map_err(|error| error.to_string())?;

    let mut state = new_game_from_content_at(
        options.seed,
        &content,
        content_bundle_start_location(&bundle),
    )
    .map_err(|error| error.to_string())?;
    let mut view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
    if options.play {
        return run_content_play_loop(&content, state, view);
    }
    if options.app {
        return run_content_app_loop(&content, state);
    }

    let mut recent_logs = Vec::new();
    if !options.tui_smoke && !options.app_smoke {
        print_turn(&view, &state, &options.scene, options.smoke, true);
    }

    for action_id in &options.actions {
        let action = find_available_action(&view, action_id)
            .ok_or_else(|| format!("action '{action_id}' is not available in current turn"))?;
        let result = apply_action_from_content(&state, &content, action_id)
            .map_err(|error| error.to_string())?;
        if !options.tui_smoke && !options.app_smoke {
            print_execution(&result.action_id, &action.label, &result.logs);
        }
        recent_logs.extend(result.logs.iter().cloned());
        state = result.state;
        view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
        if !options.tui_smoke && !options.app_smoke {
            print_turn(&view, &state, &options.scene, options.smoke, true);
        }
    }

    if options.tui_smoke {
        let page = scene_page_from_content(&state, &content).map_err(|error| error.to_string())?;
        print_scene_page_snapshot(&page, &recent_logs);
    } else if options.app_smoke {
        let page = scene_page_from_content(&state, &content).map_err(|error| error.to_string())?;
        print_scene_page_app_smoke(&page, &recent_logs, options.tick);
    }

    Ok(())
}

fn selected_content_bundle_path(options: &CliOptions) -> Result<PathBuf, String> {
    if let Some(bundle_path) = &options.content_bundle {
        return Ok(bundle_path.clone());
    }
    if let Some(storypack_id) = &options.storypack_preview {
        return storypack_preview_bundle_path(storypack_id);
    }
    Ok(default_storypack_bundle_path())
}

fn storypack_preview_bundle_path(storypack_id: &str) -> Result<PathBuf, String> {
    if storypack_id != DEFAULT_STORYPACK_ID {
        return Err(format!(
            "unsupported --storypack-preview '{storypack_id}'; available: {DEFAULT_STORYPACK_ID}"
        ));
    }
    Ok(default_storypack_bundle_path())
}

fn default_storypack_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_STORYPACK_BUNDLE_REL)
}

fn content_bundle_start_location(bundle: &ContentBundle) -> &str {
    bundle
        .runtime
        .as_ref()
        .map(|runtime| runtime.default_location.as_str())
        .filter(|location_id| !location_id.is_empty())
        .unwrap_or(DEFAULT_START_LOCATION_ID)
}

fn find_available_action<'a>(view: &'a TurnView, action_id: &str) -> Option<&'a ActionView> {
    view.actions.iter().find(|action| action.id == action_id)
}

fn run_content_play_loop(
    content: &ContentIndex,
    mut state: GameState,
    mut view: TurnView,
) -> Result<(), String> {
    let mut recent_logs = Vec::new();

    println!("escape-terminal / 직접 플레이");
    println!("입력: 번호 또는 action id, q 종료");

    loop {
        let page = scene_page_from_content(&state, content).map_err(|error| error.to_string())?;
        print_scene_page_snapshot(&page, &recent_logs);
        println!("{}", input_hint_for_actions(&view.actions));
        print!("입력> ");
        io::stdout()
            .flush()
            .map_err(|error| format!("failed to flush prompt: {error}"))?;

        let mut input = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .map_err(|error| format!("failed to read input: {error}"))?;
        if bytes_read == 0 {
            println!("입력이 끝나 게임을 종료한다.");
            break;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            println!("게임을 종료한다.");
            break;
        }
        if input.is_empty() {
            continue;
        }

        let Some(action) = resolve_play_action(&view, input) else {
            println!(
                "잘못된 입력: {input} ({})",
                invalid_input_hint(&view.actions)
            );
            continue;
        };
        let action_id = action.id.clone();
        let action_label = action.label.clone();
        let result = apply_action_from_content(&state, content, &action_id)
            .map_err(|error| error.to_string())?;
        print_play_execution(&action_id, &action_label, &result.logs);
        recent_logs.extend(result.logs.iter().cloned());
        state = result.state;
        view = turn_view_from_content(&state, content).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn run_content_app_loop(content: &ContentIndex, mut state: GameState) -> Result<(), String> {
    let mut recent_logs = Vec::new();
    let mut last_message: Option<String> = None;
    let config = slt::RunConfig::default()
        .tick_rate(Duration::from_millis(16))
        .max_fps(60)
        .title("escape-terminal".to_string());

    slt::run_with(config, |ui| {
        let page = match scene_page_from_content(&state, content) {
            Ok(page) => page,
            Err(error) => {
                ui.text(format!("fatal renderer error: {error}"));
                ui.quit();
                return;
            }
        };

        render_scene_page_app(ui, &page, &recent_logs, ui.tick());
        if let Some(message) = &last_message {
            ui.text(format!("! {message}"));
        }

        if ui.key('q') || ui.key('Q') {
            ui.quit();
            return;
        }
        if ui.key('?') {
            last_message = Some(app_input_hint_for_scene_actions(&page.actions));
            return;
        }

        for number in 1..=9 {
            let key = char::from_digit(number, 10).expect("1..=9 should convert to char");
            if ui.key(key) {
                let Some(action) = page.actions.get(number as usize - 1) else {
                    last_message = Some(format!(
                        "사용 가능한 번호: {}",
                        scene_action_number_range(&page.actions)
                    ));
                    return;
                };
                match apply_action_from_content(&state, content, &action.id) {
                    Ok(result) => {
                        recent_logs.extend(result.logs.iter().cloned());
                        state = result.state;
                        last_message = Some(format!("실행: {}", action.label));
                    }
                    Err(error) => {
                        last_message = Some(error.to_string());
                    }
                }
                return;
            }
        }
    })
    .map_err(|error| format!("failed to run SuperLightTUI app loop: {error}"))
}

fn resolve_play_action<'a>(view: &'a TurnView, input: &str) -> Option<&'a ActionView> {
    if let Ok(index) = input.parse::<usize>() {
        return index
            .checked_sub(1)
            .and_then(|offset| view.actions.get(offset));
    }
    find_available_action(view, input)
}

fn parse_args<I>(args: I) -> Result<CliOptions, String>
where
    I: IntoIterator<Item = String>,
{
    let mut scene = "printer".to_string();
    let mut seed = 123_u64;
    let mut smoke = false;
    let mut tui_smoke = false;
    let mut app_smoke = false;
    let mut play = false;
    let mut app = false;
    let mut tick = 0_u64;
    let mut content_bundle = None;
    let mut storypack_preview = None;
    let mut actions = Vec::new();
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--scene" => {
                scene = iter
                    .next()
                    .ok_or_else(|| "--scene requires a value".to_string())?;
            }
            "--seed" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--seed requires a value".to_string())?;
                seed = value
                    .parse::<u64>()
                    .map_err(|_| format!("--seed must be an unsigned integer, got '{value}'"))?;
            }
            "--content-bundle" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--content-bundle requires a value".to_string())?;
                content_bundle = Some(PathBuf::from(value));
            }
            "--storypack-preview" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--storypack-preview requires a value".to_string())?;
                storypack_preview = Some(value);
            }
            "--action" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--action requires a value".to_string())?;
                actions.push(value);
            }
            "--tick" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--tick requires a value".to_string())?;
                tick = value
                    .parse::<u64>()
                    .map_err(|_| format!("--tick must be an unsigned integer, got '{value}'"))?;
            }
            "--smoke" => smoke = true,
            "--tui-smoke" => tui_smoke = true,
            "--app-smoke" => app_smoke = true,
            "--play" => play = true,
            "--app" => app = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(CliOptions {
        scene,
        seed,
        smoke,
        tui_smoke,
        app_smoke,
        play,
        app,
        tick,
        content_bundle,
        storypack_preview,
        actions,
    })
}

fn print_help() {
    println!("escape-terminal --scene printer --seed 123 --smoke");
    println!("escape-terminal --scene content --seed 123 --play");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --play");
    println!(
        "escape-terminal --scene content --storypack-preview wuxia_jianghu_pack --seed 123 --play"
    );
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --app");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --smoke --action choice:check_message");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --tui-smoke --action choice:check_message");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --app-smoke --tick 7 --action choice:check_message");
    println!();
    println!("Options:");
    println!("  --scene <printer|content>  Run the printer scene or content-backed smoke/play");
    println!(
        "  --content-bundle <path>    Override the default storypack with a JSON content bundle"
    );
    println!(
        "  --storypack-preview <id>  Use a built-in storypack bundle explicitly (wuxia_jianghu_pack)"
    );
    println!(
        "  --action <id>              Script one content action; repeat for multi-turn smokes"
    );
    println!("  --seed <n>                 Preserve deterministic seed in core state");
    println!("  --play                     Start an interactive content-backed terminal loop");
    println!("  --app                      Start the full-screen SuperLightTUI app loop");
    println!("  --smoke                    Print a headless renderer smoke snapshot");
    println!(
        "  --tui-smoke                Print the final TUI-style snapshot after scripted actions"
    );
    println!("  --app-smoke                Print one full-screen app frame with raw-draw GlyphFX");
    println!("  --tick <n>                 Animation tick for --app-smoke raw-draw GlyphFX");
}

fn print_turn(
    view: &TurnView,
    state: &GameState,
    scene: &str,
    smoke: bool,
    include_action_ids: bool,
) {
    println!();
    println!("== Turn {} ==", state.turn);
    println!("escape-terminal / Rust GameCore smoke");
    println!("scene: {scene}");
    println!("seed: {}", state.seed);
    println!(
        "mode: {}",
        if smoke { "headless smoke" } else { "headless" }
    );
    println!("location: {}", view.location_id);
    println!(
        "status: health={} sanity={} battery={} hunger={} thirst={} danger={}",
        state.player.health,
        state.player.sanity,
        state.player.battery,
        state.player.hunger,
        state.player.thirst,
        state.danger
    );
    println!(
        "encounter: {}",
        view.encounter_id.as_deref().unwrap_or("none")
    );
    println!();
    println!("[{}]", view.title);
    println!("{}", view.body);
    println!();
    println!("Effect cues:");
    for cue in &view.effect_cues {
        match cue {
            EffectCue::GlyphAnomaly(details) => {
                println!("- {}", cue.kind_label());
                println!("  source: {}", details.source);
                println!("  intensity: {}", details.intensity);
                println!("  distortion: {}", details.distortion);
                println!("  stable_terms: {}", details.stable_terms.join(", "));
            }
        }
    }
    println!();
    println!("Choices:");
    for (index, action) in view.actions.iter().enumerate() {
        print_action(index + 1, action, include_action_ids);
    }
    print_blocked_actions(&view.blocked_actions, include_action_ids);
}

fn print_action(index: usize, action: &ActionView, include_action_ids: bool) {
    match (&action.cost_summary, include_action_ids) {
        (Some(cost), true) => println!("{index}. {} / {} / {cost}", action.id, action.label),
        (None, true) => println!("{index}. {} / {}", action.id, action.label),
        (Some(cost), false) => println!("{index}. {} / {cost}", action.label),
        (None, false) => println!("{index}. {}", action.label),
    }
}

fn print_blocked_actions(blocked_actions: &[BlockedActionView], include_action_ids: bool) {
    if blocked_actions.is_empty() {
        return;
    }
    println!();
    println!("[잠긴 선택지]");
    for action in blocked_actions {
        print_blocked_action(action, include_action_ids);
    }
}

fn print_blocked_action(action: &BlockedActionView, include_action_ids: bool) {
    match (&action.cost_summary, include_action_ids) {
        (Some(cost), true) => println!("- [잠김] {} / {} / {cost}", action.id, action.label),
        (None, true) => println!("- [잠김] {} / {}", action.id, action.label),
        (Some(cost), false) => println!("- [잠김] {} / {cost}", action.label),
        (None, false) => println!("- [잠김] {}", action.label),
    }
    println!("   이유: {}", action.reasons.join(", "));
}

fn print_tui_snapshot(view: &TurnView, state: &GameState, location_name: &str, logs: &[String]) {
    println!("[SuperLightTUI Snapshot]");
    let snapshot = render_turn_view_snapshot(view, state, location_name, logs);
    if !snapshot.is_empty() {
        println!("{snapshot}");
    }
}

fn print_scene_page_snapshot(page: &ScenePage, logs: &[String]) {
    println!("[SuperLightTUI Snapshot]");
    let snapshot = render_scene_page_snapshot(page, logs);
    if !snapshot.is_empty() {
        println!("{snapshot}");
    }
}

fn print_scene_page_app_smoke(page: &ScenePage, logs: &[String], tick: u64) {
    println!("[SuperLightTUI App Smoke]");
    let snapshot = render_scene_page_app_frame(page, logs, tick);
    if !snapshot.is_empty() {
        println!("{snapshot}");
    }
}

fn render_scene_page_app_frame(page: &ScenePage, logs: &[String], tick: u64) -> String {
    let mut backend = slt::TestBackend::new(120, 40);
    backend.render(|ui| render_scene_page_app(ui, page, logs, tick));
    backend.to_string_trimmed()
}

#[derive(Clone)]
struct RawGlyphFxFrame {
    tick: u64,
    effect_cues: Vec<SceneEffectCue>,
}

fn render_scene_page_app(ui: &mut slt::Context, page: &ScenePage, logs: &[String], tick: u64) {
    let _ = ui.col(|ui| {
        ui.text(scene_page_terminal_title(page));
        ui.text("app loop: full-screen SuperLightTUI frame");
        ui.text(format!("tick: {tick}"));
        ui.text(format!(
            "{} · {} · {} ({})",
            page.chapter_label,
            scene_mode_label(&page.mode),
            page.location.name,
            page.location.id
        ));
        ui.text(format!(
            "진단: 체력 {} · 정신력 {} · 배터리 {} · 허기 {} · 갈증 {} · 위험도 {}",
            resource_value(page, "health"),
            resource_value(page, "sanity"),
            resource_value(page, "battery"),
            resource_value(page, "hunger"),
            resource_value(page, "thirst"),
            page.status_summary.danger
        ));

        ui.text("[STORY PAGE]");
        ui.text(format!("visual: {} / {}", page.visual.id, page.visual.kind));
        ui.text(format!("alt: {}", page.visual.alt));
        ui.container().w(110).h(7).draw_with(
            RawGlyphFxFrame {
                tick,
                effect_cues: page.effect_cues.clone(),
            },
            draw_raw_glyphfx,
        );

        if matches!(page.mode, SceneMode::Encounter) {
            ui.text("[현재 인카운터]");
        } else {
            ui.text("[현재 행동]");
        }
        ui.text(page.title.as_str());
        render_scene_body(ui, page);

        ui.text("[선택지]");
        for (index, action) in page.actions.iter().enumerate() {
            ui.text(scene_action_line(index + 1, action));
        }
        if !page.blocked_actions.is_empty() {
            ui.text("[잠긴 선택지]");
            for action in &page.blocked_actions {
                ui.text(scene_blocked_action_line(action));
                ui.text(format!("   이유: {}", action.reasons.join(", ")));
            }
        }
        ui.text(app_input_hint_for_scene_actions(&page.actions));

        ui.text("[최근 로그]");
        if logs.is_empty() {
            ui.text("- 아직 기록된 로그가 없다.");
        } else {
            for log in logs.iter().rev().take(5).rev() {
                ui.text(format!("- {log}"));
            }
        }
    });
}

fn draw_raw_glyphfx(buf: &mut slt::Buffer, rect: slt::Rect, frame: &RawGlyphFxFrame) {
    let lines = raw_glyphfx_lines(frame);
    for (index, line) in lines.iter().enumerate() {
        if index >= rect.height as usize {
            break;
        }
        buf.set_string(rect.x, rect.y + index as u32, line, slt::Style::new());
    }
}

fn raw_glyphfx_lines(frame: &RawGlyphFxFrame) -> Vec<String> {
    let mut lines = vec![
        "[RAW-DRAW GLYPHFX LAYER]".to_string(),
        format!(
            "raw-draw glyphfx tick={} {}",
            frame.tick,
            glyphfx_tick_wave(frame.tick)
        ),
    ];

    if frame.effect_cues.is_empty() {
        lines.push("raw-draw glyphfx idle · no EffectCue".to_string());
        return lines;
    }

    for cue in &frame.effect_cues {
        lines.push(format!(
            "cue: {} source={} intensity={} distortion={}",
            cue.kind, cue.source, cue.intensity, cue.distortion
        ));
        if !cue.stable_terms.is_empty() {
            lines.push(format!("stable terms: {}", cue.stable_terms.join(" / ")));
        }
        if let Some(fallback) = &cue.fallback_text {
            lines.push(format!("fallback: {fallback}"));
        }
    }
    lines
}

fn glyphfx_tick_wave(tick: u64) -> String {
    const CELLS: [char; 5] = ['·', '░', '▒', '▓', '▒'];
    (0..24)
        .map(|offset| CELLS[((tick as usize) + offset) % CELLS.len()])
        .collect()
}

fn app_input_hint_for_scene_actions(actions: &[SceneAction]) -> String {
    format!(
        "입력: 번호 {} · q 종료 · ? 도움말",
        scene_action_number_range(actions)
    )
}

fn scene_action_number_range(actions: &[SceneAction]) -> String {
    match actions.len() {
        0 => "없음".to_string(),
        1 => "1".to_string(),
        count => format!("1-{count}"),
    }
}

fn render_scene_page_snapshot(page: &ScenePage, logs: &[String]) -> String {
    let mut backend = slt::TestBackend::new(120, 36);
    backend.render(|ui| render_scene_page(ui, page, logs));
    backend.to_string_trimmed()
}

fn render_scene_page(ui: &mut slt::Context, page: &ScenePage, logs: &[String]) {
    let _ = ui.col(|ui| {
        ui.text(scene_page_terminal_title(page));
        ui.text(format!(
            "{} · {}",
            page.chapter_label,
            scene_mode_label(&page.mode)
        ));
        ui.text("[상태]");
        ui.text(format!("턴: {}", page.status_summary.turn));
        ui.text(format!(
            "위치: {} ({})",
            page.location.name, page.location.id
        ));
        ui.text(format!(
            "체력: {}  정신력: {}  배터리: {}  허기: {}  갈증: {}  위험도: {}",
            resource_value(page, "health"),
            resource_value(page, "sanity"),
            resource_value(page, "battery"),
            resource_value(page, "hunger"),
            resource_value(page, "thirst"),
            page.status_summary.danger
        ));
        for warning in &page.status_summary.warnings {
            ui.text(format!("! {warning}"));
        }

        ui.text("[비주얼]");
        for line in scene_visual_card_lines(page) {
            ui.text(line);
        }

        if matches!(page.mode, SceneMode::Encounter) {
            ui.text("[현재 인카운터]");
            ui.text(page.title.as_str());
            render_scene_body(ui, page);
        }

        ui.text("[현재 행동]");
        if !matches!(page.mode, SceneMode::Encounter) {
            ui.text(page.title.as_str());
            render_scene_body(ui, page);
        }
        for (index, action) in page.actions.iter().enumerate() {
            ui.text(scene_action_line(index + 1, action));
        }
        if !page.blocked_actions.is_empty() {
            ui.text("[잠긴 선택지]");
            for action in &page.blocked_actions {
                ui.text(scene_blocked_action_line(action));
                ui.text(format!("   이유: {}", action.reasons.join(", ")));
            }
        }

        ui.text("[최근 로그]");
        if logs.is_empty() {
            ui.text("- 아직 기록된 로그가 없다.");
        } else {
            for log in logs {
                ui.text(format!("- {log}"));
            }
        }
    });
}

fn render_scene_body(ui: &mut slt::Context, page: &ScenePage) {
    for entry in &page.dialogue_entries {
        ui.text(format!("{}: {}", entry.speaker, entry.text));
    }
    for block in &page.body_blocks {
        if let Some(heading) = scene_body_block_heading(block) {
            ui.text(heading);
        }
        let lines = compact_terminal_body_block_lines(block);
        for line in lines {
            for wrapped in wrap_terminal_body_line(line, 76) {
                ui.text(wrapped);
            }
        }
    }
}

fn compact_terminal_body_block_lines(block: &BodyBlock) -> Vec<&str> {
    match block.kind.as_str() {
        "epilogue_result" => block
            .text
            .lines()
            .filter(|line| {
                line.starts_with("final_result_key:")
                    || line.starts_with("result_title:")
                    || line.starts_with("owned_by:")
            })
            .collect(),
        "epilogue_card" => block
            .text
            .lines()
            .filter(|line| line.starts_with("card_id:"))
            .collect(),
        "epilogue_state_audit" => block
            .text
            .lines()
            .filter(|line| {
                line.starts_with("audit_id:")
                    || line.starts_with("source_contract:")
                    || line.starts_with("final_result_key:")
                    || line.starts_with("canonical_state: combat_result")
                    || line.starts_with("canonical_state: boss_resolution_route")
            })
            .collect(),
        "epilogue_suppressed" => block
            .text
            .lines()
            .filter(|line| line.starts_with("card_id:") || line.starts_with("suppressed_by:"))
            .collect(),
        _ => block.text.lines().collect(),
    }
}

fn scene_body_block_heading(block: &BodyBlock) -> Option<&'static str> {
    match block.kind.as_str() {
        "epilogue_result" => Some("[결산 판정]"),
        "epilogue_state_audit" => Some("[결산 상태 감사]"),
        "epilogue_suppressed" => Some("[억제된 후일담 후보]"),
        "epilogue_contract_error" => Some("[후일담 계약 오류]"),
        _ => None,
    }
}

fn wrap_terminal_body_line(line: &str, max_chars: usize) -> Vec<String> {
    let mut rows = Vec::new();
    let mut remaining = line.trim_end();
    while remaining.chars().count() > max_chars {
        let mut last_space_byte = None;
        let mut hard_break_byte = remaining.len();
        for (char_count, (byte_index, ch)) in remaining.char_indices().enumerate() {
            if char_count >= max_chars {
                hard_break_byte = byte_index;
                break;
            }
            if ch.is_whitespace() {
                last_space_byte = Some(byte_index);
            }
        }
        let break_byte = last_space_byte
            .filter(|byte_index| *byte_index > 0)
            .unwrap_or(hard_break_byte);
        rows.push(remaining[..break_byte].trim_end().to_string());
        remaining = remaining[break_byte..].trim_start();
    }
    if rows.is_empty() || !remaining.is_empty() {
        rows.push(remaining.to_string());
    }
    rows
}

fn scene_visual_card_lines(page: &ScenePage) -> Vec<String> {
    let mut lines = vec![
        "╭─ VISUAL CARD ─────────────────────────╮".to_string(),
        format!("│ visual id: {}", page.visual.id),
        format!("│ layout: {}", page.visual.kind),
        format!("│ alt: {}", page.visual.alt),
    ];
    lines.extend(glyphfx_card_lines(&page.effect_cues));
    lines.push("╰────────────────────────────────────────╯".to_string());
    lines
}

fn glyphfx_card_lines(effect_cues: &[SceneEffectCue]) -> Vec<String> {
    if effect_cues.is_empty() {
        return vec!["│ glyphfx signal: idle · terminal-native fallback".to_string()];
    }

    let mut lines = Vec::new();
    for cue in effect_cues {
        let percent = glyphfx_intensity_percent(cue.intensity);
        lines.push(format!(
            "│ glyphfx signal: {} [{}] {}% {}",
            cue.kind,
            glyphfx_meter(percent),
            percent,
            cue.distortion
        ));
        if !cue.stable_terms.is_empty() {
            lines.push(format!("│ stable terms: {}", cue.stable_terms.join(" / ")));
        }
        if let Some(fallback) = &cue.fallback_text {
            lines.push(format!("│ fallback: {fallback}"));
        }
    }
    lines
}

fn scene_page_terminal_title(page: &ScenePage) -> &'static str {
    if is_wuxia_scene_page(page) {
        "이구학지 - 천기록 // SuperLightTUI STORYBOOK"
    } else {
        "ESCAPE OFFICE // SuperLightTUI HORROR EDITION"
    }
}

fn is_wuxia_scene_page(page: &ScenePage) -> bool {
    page.location.id.starts_with("wuxia_")
        || page.visual.id.contains("wuxia")
        || page
            .visual
            .source_id
            .as_deref()
            .is_some_and(|source_id| source_id.contains("wuxia"))
}

fn glyphfx_intensity_percent(intensity: f32) -> u32 {
    (intensity.clamp(0.0, 1.0) * 100.0).round() as u32
}

fn glyphfx_meter(percent: u32) -> String {
    let filled = (percent / 10).min(10) as usize;
    format!("{}{}", "#".repeat(filled), "-".repeat(10 - filled))
}

fn render_turn_view_snapshot(
    view: &TurnView,
    state: &GameState,
    location_name: &str,
    logs: &[String],
) -> String {
    let mut backend = slt::TestBackend::new(120, 32);
    backend.render(|ui| {
        let _ = ui.col(|ui| {
            ui.text("ESCAPE OFFICE // SuperLightTUI HORROR EDITION");
            ui.text("legacy printer scene · TurnView bridge");
            ui.text("[상태]");
            ui.text(format!("턴: {}", state.turn));
            ui.text(format!("위치: {location_name} ({})", state.location_id));
            ui.text(format!(
                "체력: {}  정신력: {}  배터리: {}  위험도: {}",
                state.player.health, state.player.sanity, state.player.battery, state.danger
            ));
            ui.text("[비주얼]");
            ui.text(glyphfx_turn_line(&view.effect_cues));

            if view.encounter_id.is_some() {
                ui.text("[현재 인카운터]");
                ui.text(view.title.as_str());
                ui.text(view.body.as_str());
            }

            ui.text("[현재 행동]");
            if view.encounter_id.is_none() {
                ui.text(view.title.as_str());
                ui.text(view.body.as_str());
            }
            for (index, action) in view.actions.iter().enumerate() {
                ui.text(turn_action_line(index + 1, action));
            }
            if !view.blocked_actions.is_empty() {
                ui.text("[잠긴 선택지]");
                for action in &view.blocked_actions {
                    ui.text(turn_blocked_action_line(action));
                    ui.text(format!("   이유: {}", action.reasons.join(", ")));
                }
            }

            ui.text("[최근 로그]");
            if logs.is_empty() {
                ui.text("- 아직 기록된 로그가 없다.");
            } else {
                for log in logs {
                    ui.text(format!("- {log}"));
                }
            }
        });
    });
    backend.to_string_trimmed()
}

fn scene_mode_label(mode: &SceneMode) -> &'static str {
    match mode {
        SceneMode::Encounter => "인카운터",
        SceneMode::Movement => "이동",
        SceneMode::Ending => "엔딩",
    }
}

fn resource_value(page: &ScenePage, id: &str) -> i32 {
    page.status_summary
        .resources
        .iter()
        .find(|resource| resource.id == id)
        .map(|resource| resource.value)
        .unwrap_or_default()
}

fn input_hint_for_actions(actions: &[ActionView]) -> String {
    format!(
        "입력 안내: {} 또는 action id, q/quit 종료",
        action_number_range(actions)
    )
}

fn invalid_input_hint(actions: &[ActionView]) -> String {
    format!(
        "사용 가능한 번호: {} 또는 action id",
        action_number_range(actions)
    )
}

fn action_number_range(actions: &[ActionView]) -> String {
    match actions.len() {
        0 => "없음".to_string(),
        1 => "1".to_string(),
        count => format!("1-{count}"),
    }
}

fn glyphfx_turn_line(effect_cues: &[EffectCue]) -> String {
    if effect_cues.is_empty() {
        return "GlyphFX: terminal-native fallback idle".to_string();
    }
    let cues = effect_cues
        .iter()
        .map(|cue| match cue {
            EffectCue::GlyphAnomaly(details) => format!(
                "{}:{} {}",
                cue.kind_label(),
                details.intensity,
                details.distortion
            ),
        })
        .collect::<Vec<_>>()
        .join(" | ");
    format!("GlyphFX: {cues}")
}

fn scene_action_line(index: usize, action: &SceneAction) -> String {
    match &action.cost_text {
        Some(cost) => format!("{index}. {} / {} / {cost}", action.id, action.label),
        None => format!("{index}. {} / {}", action.id, action.label),
    }
}

fn scene_blocked_action_line(action: &SceneBlockedAction) -> String {
    match &action.cost_text {
        Some(cost) => format!("- [잠김] {} / {} / {cost}", action.id, action.label),
        None => format!("- [잠김] {} / {}", action.id, action.label),
    }
}

fn turn_action_line(index: usize, action: &ActionView) -> String {
    match &action.cost_summary {
        Some(cost) => format!("{index}. {} / {} / {cost}", action.id, action.label),
        None => format!("{index}. {} / {}", action.id, action.label),
    }
}

fn turn_blocked_action_line(action: &BlockedActionView) -> String {
    match &action.cost_summary {
        Some(cost) => format!("- [잠김] {} / {} / {cost}", action.id, action.label),
        None => format!("- [잠김] {} / {}", action.id, action.label),
    }
}

fn print_play_execution(action_id: &str, label: &str, logs: &[String]) {
    if action_id.starts_with("move:") {
        println!("이동 실행: {label}");
    } else {
        println!("선택 실행: {label}");
    }
    println!("결과:");
    for log in logs {
        println!("- {log}");
    }
}

fn print_execution(action_id: &str, label: &str, logs: &[String]) {
    println!();
    println!("executed: {action_id} / {label}");
    println!("Logs:");
    for log in logs {
        println!("- {log}");
    }
}
