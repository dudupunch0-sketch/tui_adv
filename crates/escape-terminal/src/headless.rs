use super::*;

pub(crate) fn print_turn(
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
pub(crate) fn print_action(index: usize, action: &ActionView, include_action_ids: bool) {
    if include_action_ids {
        println!("{}", turn_action_line(index, action));
        return;
    }
    match &action.cost_summary {
        Some(cost) => println!("{index}. {} / {cost}", action.label),
        None => println!("{index}. {}", action.label),
    }
}
pub(crate) fn print_blocked_actions(
    blocked_actions: &[BlockedActionView],
    include_action_ids: bool,
) {
    if blocked_actions.is_empty() {
        return;
    }
    println!();
    println!("[잠긴 선택지]");
    for action in blocked_actions {
        print_blocked_action(action, include_action_ids);
    }
}
pub(crate) fn print_blocked_action(action: &BlockedActionView, include_action_ids: bool) {
    if include_action_ids {
        println!("{}", turn_blocked_action_line(action));
    } else {
        match &action.cost_summary {
            Some(cost) => println!("- [잠김] {} / {cost}", action.label),
            None => println!("- [잠김] {}", action.label),
        }
    }
    println!("   이유: {}", action.reasons.join(", "));
}
pub(crate) fn print_tui_snapshot(
    view: &TurnView,
    state: &GameState,
    location_name: &str,
    logs: &[String],
) {
    println!("[Terminal Snapshot]");
    let snapshot = render_turn_view_snapshot(view, state, location_name, logs);
    if !snapshot.is_empty() {
        println!("{snapshot}");
    }
}
pub(crate) fn print_scene_page_snapshot(page: &ScenePage, logs: &[String]) {
    println!("[Terminal Snapshot]");
    let snapshot = render_scene_page_snapshot(page, logs);
    if !snapshot.is_empty() {
        println!("{snapshot}");
    }
}
pub(crate) fn print_play_execution(action_id: &str, label: &str, logs: &[String]) {
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
pub(crate) fn print_execution(action_id: &str, label: &str, logs: &[String]) {
    println!();
    println!("executed: {action_id} / {label}");
    println!("Logs:");
    for log in logs {
        println!("- {log}");
    }
}
