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
    Projects,
    MyTasks,
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
            Event::Me => self.me().await?,
            Event::Projects => self.projects().await?,
            Event::MyTasks => self.my_tasks().await?,
        }
        Ok(())
    }

    /// Update state with user details.
    ///
    async fn me(&mut self) -> Result<()> {
        let (user, workspaces) = self.asana.me().await?;
        let mut state = self.state.lock().await;
        state.set_user(user);
        if !workspaces.is_empty() && state.get_active_workspace().is_none() {
            state.set_active_workspace(workspaces[0].gid.to_owned());
        }
        state.set_workspaces(workspaces);
        Ok(())
    }

    /// Update state with projects.
    ///
    async fn projects(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        let projects = self
            .asana
            .projects(&state.get_active_workspace().unwrap().gid)
            .await?;
        state.set_projects(projects);
        Ok(())
    }

    /// Update state with tasks assigned to the user.
    ///
    async fn my_tasks(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        let my_tasks = self
            .asana
            .my_tasks(
                &state.get_user().unwrap().gid,
                &state.get_active_workspace().unwrap().gid,
            )
            .await?;
        state.set_tasks(my_tasks);
        Ok(())
    }
}
