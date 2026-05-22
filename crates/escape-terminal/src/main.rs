use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game,
    new_game_from_content, scene_page_from_content, turn_view, turn_view_from_content, ActionView,
    BlockedActionView, ContentIndex, EffectCue, GameState, SceneAction, SceneBlockedAction,
    SceneEffectCue, SceneMode, ScenePage, TurnView,
};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
struct CliOptions {
    scene: String,
    seed: u64,
    smoke: bool,
    tui_smoke: bool,
    play: bool,
    content_bundle: Option<PathBuf>,
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
    if options.play && (options.smoke || options.tui_smoke) {
        return Err("--play cannot be combined with --smoke or --tui-smoke".to_string());
    }
    if options.play && !options.actions.is_empty() {
        return Err("--play cannot be combined with scripted --action values".to_string());
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
    } else {
        print_turn(&view, &state, &options.scene, options.smoke, false);
    }
    Ok(())
}

fn run_content_scene(options: &CliOptions) -> Result<(), String> {
    let bundle_path = options
        .content_bundle
        .as_ref()
        .ok_or_else(|| "--content-bundle is required with --scene content".to_string())?;
    let json_text = std::fs::read_to_string(bundle_path).map_err(|error| {
        format!(
            "failed to read content bundle '{}': {error}",
            bundle_path.display()
        )
    })?;
    let bundle = load_content_bundle(&json_text).map_err(|error| error.to_string())?;
    let content = index_content_bundle(&bundle).map_err(|error| error.to_string())?;

    let mut state =
        new_game_from_content(options.seed, &content).map_err(|error| error.to_string())?;
    let mut view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
    if options.play {
        return run_content_play_loop(&content, state, view);
    }

    let mut recent_logs = Vec::new();
    if !options.tui_smoke {
        print_turn(&view, &state, &options.scene, options.smoke, true);
    }

    for action_id in &options.actions {
        let action = find_available_action(&view, action_id)
            .ok_or_else(|| format!("action '{action_id}' is not available in current turn"))?;
        let result = apply_action_from_content(&state, &content, action_id)
            .map_err(|error| error.to_string())?;
        if !options.tui_smoke {
            print_execution(&result.action_id, &action.label, &result.logs);
        }
        recent_logs.extend(result.logs.iter().cloned());
        state = result.state;
        view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
        if !options.tui_smoke {
            print_turn(&view, &state, &options.scene, options.smoke, true);
        }
    }

    if options.tui_smoke {
        let page = scene_page_from_content(&state, &content).map_err(|error| error.to_string())?;
        print_scene_page_snapshot(&page, &recent_logs);
    }

    Ok(())
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
            println!("잘못된 입력: {input}");
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
    let mut play = false;
    let mut content_bundle = None;
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
            "--action" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--action requires a value".to_string())?;
                actions.push(value);
            }
            "--smoke" => smoke = true,
            "--tui-smoke" => tui_smoke = true,
            "--play" => play = true,
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
        play,
        content_bundle,
        actions,
    })
}

fn print_help() {
    println!("escape-terminal --scene printer --seed 123 --smoke");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --play");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --smoke --action choice:check_message");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --tui-smoke --action choice:check_message");
    println!();
    println!("Options:");
    println!("  --scene <printer|content>  Run the printer scene or content-backed smoke/play");
    println!("  --content-bundle <path>    JSON content bundle for --scene content");
    println!(
        "  --action <id>              Script one content action; repeat for multi-turn smokes"
    );
    println!("  --seed <n>                 Preserve deterministic seed in core state");
    println!("  --play                     Start an interactive content-backed terminal loop");
    println!("  --smoke                    Print a headless renderer smoke snapshot");
    println!(
        "  --tui-smoke                Print the final TUI-style snapshot after scripted actions"
    );
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

fn render_scene_page_snapshot(page: &ScenePage, logs: &[String]) -> String {
    let mut backend = slt::TestBackend::new(120, 36);
    backend.render(|ui| render_scene_page(ui, page, logs));
    backend.to_string_trimmed()
}

fn render_scene_page(ui: &mut slt::Context, page: &ScenePage, logs: &[String]) {
    let _ = ui.col(|ui| {
        ui.text("ESCAPE OFFICE // SuperLightTUI HORROR EDITION");
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
        ui.text(format!(
            "{} / {} / {}",
            page.visual.kind, page.visual.id, page.visual.alt
        ));
        ui.text(glyphfx_line(&page.effect_cues));

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
        ui.text(block.text.as_str());
    }
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

fn glyphfx_line(effect_cues: &[SceneEffectCue]) -> String {
    if effect_cues.is_empty() {
        return "GlyphFX: terminal-native fallback idle".to_string();
    }
    let cues = effect_cues
        .iter()
        .map(|cue| format!("{}:{} {}", cue.kind, cue.intensity, cue.distortion))
        .collect::<Vec<_>>()
        .join(" | ");
    format!("GlyphFX: {cues}")
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
