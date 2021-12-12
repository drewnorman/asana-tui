use crate::asana::Asana;
use crate::state::State;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Specify different network event types.
pub enum Event {
    Me,
}

/// Specify struct for managing state with network events.
pub struct Handler<'a> {
    state: &'a Arc<Mutex<State>>,
    asana: Asana,
}

impl<'a> Handler<'a> {
    /// Return new instance with reference to state.
    pub fn new(state: &'a Arc<Mutex<State>>, access_token: &str) -> Self {
        Handler {
            state,
            asana: Asana::new(String::from(access_token)),
        }
    }

    /// Handle network events by type.
    pub async fn handle(&mut self, event: Event) {
        match event {
            Event::Me => {
                let (user, workspaces) = self.asana.me().await;
                let mut state = self.state.lock().await;
                state.user = Some(user);
                state.workspaces = workspaces;
            }
        }
    }
}
