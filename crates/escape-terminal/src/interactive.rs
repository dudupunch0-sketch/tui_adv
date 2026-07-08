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
pub(crate) fn run_content_app_loop(
    content: &ContentIndex,
    mut state: GameState,
) -> Result<(), String> {
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
