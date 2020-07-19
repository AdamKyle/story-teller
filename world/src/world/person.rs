use std::collections::HashMap;

use crate::world::actions::{Action, OnAction};
use crate::world::conversation::{Converse};


#[derive(Clone, Debug)]
pub struct Person {
    pub name: String,
    pub actions: Option<HashMap<Action, OnAction>>,
    pub conversation: Converse,
}

impl Person {
    pub fn new(name: String, actions: Option<HashMap<Action, OnAction>>, conversation: Converse) -> Self {
        Person {
            name: name,
            actions: actions,
            conversation: conversation,
        }
    }
}
