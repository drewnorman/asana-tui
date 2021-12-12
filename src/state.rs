use crate::asana::{User, Workspace};

pub struct State {
    pub user: Option<User>,
    pub workspaces: Vec<Workspace>,
}

impl State {
    pub fn new() -> State {
        return State {
            user: None,
            workspaces: vec![],
        };
    }
}
