use crate::asana::{User, Workspace};
use tui::layout::Rect;

/// Houses data representative of application state.
///
#[derive(Default)]
pub struct State {
    user: Option<User>,
    workspaces: Vec<Workspace>,
    active_workspace_gid: Option<String>,
    terminal_size: Rect,
}

impl State {
    /// Returns a reference to the active workspace or None.
    ///
    pub fn get_active_workspace(&self) -> Option<&Workspace> {
        match &self.active_workspace_gid {
            Some(active_workspace_gid) => self
                .workspaces
                .iter()
                .find(|workspace| active_workspace_gid == &workspace.gid),
            None => None,
        }
    }

    /// Sets details for current user.
    ///
    pub fn set_user(&mut self, user: User) -> &mut Self {
        self.user = Some(user);
        self
    }

    /// Returns details for current user.
    ///
    pub fn get_user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    /// Sets workspaces available to current user, initializing the active
    /// workspace GID if unset and at least one workspace is available.
    ///
    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) -> &mut Self {
        if workspaces.is_empty() {
            return self;
        }
        self.workspaces = workspaces;
        self.active_workspace_gid = Some(self.workspaces[0].gid.to_owned());
        self
    }

    /// Sets the terminal size.
    ///
    pub fn set_terminal_size(&mut self, size: Rect) -> &mut Self {
        self.terminal_size = size;
        self
    }
}
