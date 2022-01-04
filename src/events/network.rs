use crate::asana::Asana;
use crate::state::State;
use anyhow::Result;
use log::*;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Specify different network event types.
///
#[derive(Debug)]
pub enum Event {
    Me,
}

/// Specify struct for managing state with network events.
///
pub struct Handler<'a> {
    state: &'a Arc<Mutex<State>>,
    asana: &'a mut Asana,
}

impl<'a> Handler<'a> {
    /// Return new instance with reference to state.
    ///
    pub fn new(state: &'a Arc<Mutex<State>>, asana: &'a mut Asana) -> Self {
        Handler { state, asana }
    }

    /// Handle network events by type.
    ///
    pub async fn handle(&mut self, event: Event) -> Result<()> {
        debug!("Processing network event '{:?}'...", event);
        match event {
            Event::Me => {
                let (user, workspaces) = self.asana.me().await?;
                let mut state = self.state.lock().await;
                state.set_user(user);
                if !workspaces.is_empty() && state.get_active_workspace().is_none() {
                    state.set_active_workspace(workspaces[0].gid.to_owned());
                }
                state.set_workspaces(workspaces);
            }
        }
        Ok(())
    }
}
