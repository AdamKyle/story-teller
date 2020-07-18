/// Acceptable Actions a player can take.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Look, Explore, NONE, Talk
}

/// Do something on action
///
/// Actions are what the player can do.
///
/// the on_action is what happens when the action is succesffull.
/// the action is only successful if the dc_check passes.
///
/// If the Action does not have a dc_check then we should just do the on_action.
#[derive(Clone, Debug)]
pub struct OnAction {
    pub on_action: String,
    pub dc_check: Option<i32>,
}

impl OnAction {

    /// Creates a new on action struct
    pub fn new(on_action: String, dc_check: Option<i32>) -> Self {
        OnAction {
            on_action: on_action,
            dc_check: dc_check,
        }
    }

    /// Do the action.
    pub fn do_action(&mut self) {
        println!("{}", self.on_action);
    }
}
