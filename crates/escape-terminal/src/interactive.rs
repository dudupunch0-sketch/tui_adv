use super::*;

pub(crate) fn run_content_play_loop(
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
pub(crate) fn resolve_play_action<'a>(view: &'a TurnView, input: &str) -> Option<&'a ActionView> {
    if let Ok(index) = input.parse::<usize>() {
        return index
            .checked_sub(1)
            .and_then(|offset| view.actions.get(offset));
    }
    find_available_action(view, input)
}
pub(crate) fn input_hint_for_actions(actions: &[ActionView]) -> String {
    format!(
        "입력 안내: {} 또는 action id, q/quit 종료",
        action_number_range(actions)
    )
}
pub(crate) fn invalid_input_hint(actions: &[ActionView]) -> String {
    format!(
        "사용 가능한 번호: {} 또는 action id",
        action_number_range(actions)
    )
}
