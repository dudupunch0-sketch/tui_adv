use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game,
    new_game_from_content, turn_view, turn_view_from_content, ActionView, BlockedActionView,
    ContentIndex, EffectCue, GameState, TurnView,
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
        let location_name = display_location_name(&content, &state);
        print_tui_snapshot(&view, &state, &location_name, &recent_logs);
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
        let location_name = display_location_name(content, &state);
        print_tui_snapshot(&view, &state, &location_name, &recent_logs);
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

fn display_location_name(content: &ContentIndex, state: &GameState) -> String {
    content
        .location(&state.location_id)
        .map(|location| location.name.clone())
        .unwrap_or_else(|| state.location_id.clone())
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
        "status: health={} sanity={} battery={} danger={}",
        state.player.health, state.player.sanity, state.player.battery, state.danger
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
    println!("[TUI Snapshot]");
    println!();
    println!("[상태]");
    println!("턴: {}", state.turn);
    println!("위치: {location_name} ({})", state.location_id);
    println!(
        "체력: {}  정신력: {}  배터리: {}  위험도: {}",
        state.player.health, state.player.sanity, state.player.battery, state.danger
    );
    println!();

    if view.encounter_id.is_some() {
        println!("[현재 인카운터]");
        println!("{}", view.title);
        println!("{}", view.body);
        println!();
    }

    println!("[현재 행동]");
    if view.encounter_id.is_none() {
        println!("{}", view.title);
        println!("{}", view.body);
    }
    for (index, action) in view.actions.iter().enumerate() {
        print_action(index + 1, action, true);
    }
    print_blocked_actions(&view.blocked_actions, true);
    println!();

    println!("[최근 로그]");
    if logs.is_empty() {
        println!("- 아직 기록된 로그가 없다.");
    } else {
        for log in logs {
            println!("- {log}");
        }
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
