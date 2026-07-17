use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game,
    new_game_from_content_at, scene_page_from_content, turn_view, turn_view_from_content,
    ActionView, BlockedActionView, BodyBlock, ContentIndex, EffectCue, GameState, SceneAction,
    SceneBlockedAction, SceneContentItem, SceneEffectCue, SceneMode, ScenePage, TurnView,
};
use std::io::{self, Write};
use std::path::PathBuf;

mod bundle;
mod cli;
mod format;
mod glyphfx;
mod headless;
mod interactive;
mod snapshot;

pub(crate) use bundle::*;
pub(crate) use cli::*;
pub(crate) use format::*;
pub(crate) use glyphfx::*;
pub(crate) use headless::*;
pub(crate) use interactive::*;
pub(crate) use snapshot::*;

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
        return Err("--play cannot be combined with smoke modes".to_string());
    }
    if options.play && !options.actions.is_empty() {
        return Err(
            "interactive modes cannot be combined with scripted --action values".to_string(),
        );
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

    let mut state = new_game_from_content_at(options.seed, &content, bundle.start_location_id())
        .map_err(|error| error.to_string())?;
    let mut view = turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
    if options.play {
        return run_content_play_loop(&content, state, view);
    }

    let mut recent_logs = Vec::new();
    if !options.tui_smoke {
        print_turn(&view, &state, &options.scene, options.smoke, true);
    }

    for action_id in &options.actions {
        if options.tui_smoke {
            // Scripted smoke routes predate multi-stage Events. Advance through
            // presentation-only stages (and deterministic intermediate choices)
            // until the requested legacy action becomes available. Interactive
            // play never uses this compatibility path.
            for _ in 0..64 {
                if find_available_action(&view, action_id).is_some() {
                    break;
                }
                let bridge_action_id = if find_available_action(&view, "event:continue").is_some() {
                    Some("event:continue")
                } else if state.active_event_id.is_some() {
                    view.actions.first().map(|action| action.id.as_str())
                } else {
                    None
                };
                let Some(bridge_action_id) = bridge_action_id else {
                    break;
                };
                let result = apply_action_from_content(&state, &content, bridge_action_id)
                    .map_err(|error| error.to_string())?;
                recent_logs.extend(result.logs.iter().cloned());
                state = result.state;
                view =
                    turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
            }
        }
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
        if !options.actions.is_empty() {
            // Keep legacy scripted smoke routes readable after staged conversions:
            // presentation-only story/result stages advance until the next choice
            // or encounter, while interactive play still waits for event:continue.
            for _ in 0..64 {
                if find_available_action(&view, "event:continue").is_none() {
                    break;
                }
                let result = apply_action_from_content(&state, &content, "event:continue")
                    .map_err(|error| error.to_string())?;
                recent_logs.extend(result.logs.iter().cloned());
                state = result.state;
                view =
                    turn_view_from_content(&state, &content).map_err(|error| error.to_string())?;
            }
        }
        let page = scene_page_from_content(&state, &content).map_err(|error| error.to_string())?;
        print_scene_page_snapshot(&page, &recent_logs);
    }

    Ok(())
}
