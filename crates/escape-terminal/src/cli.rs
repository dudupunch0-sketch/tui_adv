use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CliOptions {
    pub(crate) scene: String,
    pub(crate) seed: u64,
    pub(crate) smoke: bool,
    pub(crate) tui_smoke: bool,
    pub(crate) play: bool,
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
    let mut play = false;
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
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --smoke --action choice:check_message");
    println!("escape-terminal --scene content --content-bundle <path> --seed 123 --tui-smoke --action choice:check_message");
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
    println!("  --smoke                    Print a headless renderer smoke snapshot");
    println!(
        "  --tui-smoke                Print the final TUI-style snapshot after scripted actions"
    );
}
