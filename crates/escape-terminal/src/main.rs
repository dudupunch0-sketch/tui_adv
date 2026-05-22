use escape_core::{new_game, turn_view, EffectCue};

#[derive(Debug, PartialEq, Eq)]
struct CliOptions {
    scene: String,
    seed: u64,
    smoke: bool,
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

    if options.scene != "printer" {
        return Err(format!(
            "unsupported scene '{}'; only 'printer' is available in this smoke executable",
            options.scene
        ));
    }

    let state = new_game(options.seed);
    let view = turn_view(&state);

    print_turn(&view, state.seed, options.smoke);
    Ok(())
}

fn parse_args<I>(args: I) -> Result<CliOptions, String>
where
    I: IntoIterator<Item = String>,
{
    let mut scene = "printer".to_string();
    let mut seed = 123_u64;
    let mut smoke = false;
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
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(CliOptions { scene, seed, smoke })
}

fn print_help() {
    println!("escape-terminal --scene printer --seed 123 --smoke");
    println!();
    println!("Options:");
    println!("  --scene <printer>  Run the printer anomaly scene smoke");
    println!("  --seed <n>         Preserve deterministic seed in core state");
    println!("  --smoke            Print a headless renderer smoke snapshot");
}

fn print_turn(view: &escape_core::TurnView, seed: u64, smoke: bool) {
    println!("escape-terminal / Rust GameCore smoke");
    println!("seed: {seed}");
    println!(
        "mode: {}",
        if smoke { "headless smoke" } else { "headless" }
    );
    println!("location: {}", view.location_id);
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
        match &action.cost_summary {
            Some(cost) => println!("{}. {} / {}", index + 1, action.label, cost),
            None => println!("{}. {}", index + 1, action.label),
        }
    }
}
