use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CliOptions {
    pub(crate) scene: String,
    pub(crate) seed: u64,
    pub(crate) smoke: bool,
    pub(crate) tui_smoke: bool,
    pub(crate) app_smoke: bool,
    pub(crate) play: bool,
    pub(crate) app: bool,
    pub(crate) tick: u64,
    pub(crate) content_bundle: Option<PathBuf>,
    pub(crate) storypack_preview: Option<String>,
    pub(crate) actions: Vec<String>,
}
pub(crate) fn parse_args<I>(args: I) -> Result<CliOptions, String>
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
pub(crate) fn print_help() {
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
