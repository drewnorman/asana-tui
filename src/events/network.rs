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
    ProjectTasks,
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
            Event::ProjectTasks => self.project_tasks().await?,
            Event::MyTasks => self.my_tasks().await?,
        }
        Ok(())
    }

    /// Update state with user details and projects for active workspace.
    ///
    async fn me(&mut self) -> Result<()> {
        info!("Preparing initial application data...");
        info!("Fetching user details and available workspaces...");
        let (user, workspaces) = self.asana.me().await?;
        let mut projects = vec![];
        if !workspaces.is_empty() {
            info!("Fetching projects for active workspace...");
            projects = self.asana.projects(&workspaces[0].gid).await?;
        }
        let mut state = self.state.lock().await;
        state.set_user(user);
        state.set_active_workspace(workspaces[0].gid.to_owned());
        state.set_workspaces(workspaces);
        state.set_projects(projects);
        info!("Loaded initial application data.");
        Ok(())
    }

    /// Update state with tasks for project.
    ///
    async fn project_tasks(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        let project = state.get_project();
        if project.is_none() {
            warn!("Skipping tasks request for unset project.");
            return Ok(());
        }
        info!("Fetching tasks for project '{}'...", &project.unwrap().name);
        let tasks = self.asana.tasks(&project.unwrap().gid).await?;
        info!("Received tasks for project '{}'.", &project.unwrap().name);
        state.set_tasks(tasks);
        Ok(())
    }

    /// Update state with tasks assigned to the user.
    ///
    async fn my_tasks(&mut self) -> Result<()> {
        info!("Fetching incomplete tasks assigned to user...");
        let mut state = self.state.lock().await;
        let my_tasks = self
            .asana
            .my_tasks(
                &state.get_user().unwrap().gid,
                &state.get_active_workspace().unwrap().gid,
            )
            .await?;
        state.set_tasks(my_tasks);
        info!("Received incomplete tasks assigned to user.");
        Ok(())
    }
}
