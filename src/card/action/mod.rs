use super::action_type::ActionType;

pub struct Action {
    action_type: ActionType,
    filters: Option<Vec<ActionFilter>>,
}
