use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game,
    new_game_from_content, turn_view, turn_view_from_content, ActionView, EffectCue, GameState,
    TurnView,
};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
struct CliOptions {
    scene: String,
    seed: u64,
    smoke: bool,
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

    let state = new_game(options.seed);
    let view = turn_view(&state);
    print_turn(&view, &state, &options.scene, options.smoke, false);
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
    print_turn(&view, &state, &options.scene, options.smoke, true);

    for action_id in &options.actions {
        let action = find_available_action(&view, action_id)
            .ok_or_else(|| format!("action '{action_id}' is not available in current turn"))?;
        let result = apply_action_from_content(&state, &content, action_id)
            .map_err(|error| error.to_string())?;
        print_execution(&result.action_id, &action.label, &result.logs);
        state = result.state;
        view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
        print_turn(&view, &state, &options.scene, options.smoke, true);
    }

    Ok(())
}

fn find_available_action<'a>(view: &'a TurnView, action_id: &str) -> Option<&'a ActionView> {
    view.actions.iter().find(|action| action.id == action_id)
}

fn parse_args<I>(args: I) -> Result<CliOptions, String>
where
    I: IntoIterator<Item = String>,
{
    let mut scene = "printer".to_string();
    let mut seed = 123_u64;
    let mut smoke = false;
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
        content_bundle,
        actions,
    })
}

fn print_help() {
    println!("escape-terminal --scene printer --seed 123 --smoke");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --smoke --action choice:check_message");
    println!();
    println!("Options:");
    println!("  --scene <printer|content>  Run the printer scene or content-backed smoke");
    println!("  --content-bundle <path>    JSON content bundle for --scene content");
    println!(
        "  --action <id>              Script one content action; repeat for multi-turn smokes"
    );
    println!("  --seed <n>                 Preserve deterministic seed in core state");
    println!("  --smoke                    Print a headless renderer smoke snapshot");
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
        "status: health={} sanity={} battery={}",
        state.player.health, state.player.sanity, state.player.battery
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
}

fn print_action(index: usize, action: &ActionView, include_action_ids: bool) {
    match (&action.cost_summary, include_action_ids) {
        (Some(cost), true) => println!("{index}. {} / {} / {cost}", action.id, action.label),
        (None, true) => println!("{index}. {} / {}", action.id, action.label),
        (Some(cost), false) => println!("{index}. {} / {cost}", action.label),
        (None, false) => println!("{index}. {}", action.label),
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
