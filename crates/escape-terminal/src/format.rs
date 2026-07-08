use super::*;

pub(crate) fn find_available_action<'a>(
    view: &'a TurnView,
    action_id: &str,
) -> Option<&'a ActionView> {
    view.actions.iter().find(|action| action.id == action_id)
}
pub(crate) fn scene_action_number_range(actions: &[SceneAction]) -> String {
    action_count_range(actions.len())
}
pub(crate) fn action_number_range(actions: &[ActionView]) -> String {
    action_count_range(actions.len())
}
fn action_count_range(count: usize) -> String {
    match count {
        0 => "없음".to_string(),
        1 => "1".to_string(),
        count => format!("1-{count}"),
    }
}
pub(crate) fn action_line(index: usize, id: &str, label: &str, cost: Option<&str>) -> String {
    match cost {
        Some(cost) => format!("{index}. {id} / {label} / {cost}"),
        None => format!("{index}. {id} / {label}"),
    }
}
pub(crate) fn blocked_action_line(id: &str, label: &str, cost: Option<&str>) -> String {
    match cost {
        Some(cost) => format!("- [잠김] {id} / {label} / {cost}"),
        None => format!("- [잠김] {id} / {label}"),
    }
}
pub(crate) fn scene_action_line(index: usize, action: &SceneAction) -> String {
    action_line(
        index,
        &action.id,
        &action.label,
        action.cost_text.as_deref(),
    )
}
pub(crate) fn scene_blocked_action_line(action: &SceneBlockedAction) -> String {
    blocked_action_line(&action.id, &action.label, action.cost_text.as_deref())
}
pub(crate) fn turn_action_line(index: usize, action: &ActionView) -> String {
    action_line(
        index,
        &action.id,
        &action.label,
        action.cost_summary.as_deref(),
    )
}
pub(crate) fn turn_blocked_action_line(action: &BlockedActionView) -> String {
    blocked_action_line(&action.id, &action.label, action.cost_summary.as_deref())
}
