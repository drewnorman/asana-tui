use crate::asana::{User, Workspace};
use crate::ui::SPINNER_FRAME_COUNT;
use tui::layout::Rect;

/// Specifying the different menus.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Menu {
    Status,
    Shortcuts,
    TopList,
}

/// Specifying the different shortcuts.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Shortcut {
    MyTasks,
    DueSoon,
    PastDue,
    RecentlyCreated,
    RecentlyEdited,
    RecentlyCompleted,
}

/// Houses data representative of application state.
///
pub struct State {
    user: Option<User>,
    workspaces: Vec<Workspace>,
    active_workspace_gid: Option<String>,
    terminal_size: Rect,
    spinner_index: usize,
    current_menu: Menu,
    current_shortcut: Shortcut,
}

/// Defines default application state.
///
impl Default for State {
    fn default() -> State {
        State {
            user: None,
            workspaces: vec![],
            active_workspace_gid: None,
            terminal_size: Rect::default(),
            spinner_index: 0,
            current_menu: Menu::Shortcuts,
            current_shortcut: Shortcut::MyTasks,
        }
    }
}

impl State {
    /// Returns details for current user.
    ///
    pub fn get_user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    /// Sets details for current user.
    ///
    pub fn set_user(&mut self, user: User) -> &mut Self {
        self.user = Some(user);
        self
    }

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

    /// Sets the active workspace by the given workspace GID.
    ///
    pub fn set_active_workspace(&mut self, workspace_gid: String) -> &mut Self {
        self.active_workspace_gid = Some(workspace_gid);
        self
    }

    /// Sets workspaces available to current user, initializing the active
    /// workspace GID if unset and at least one workspace is available.
    ///
    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) -> &mut Self {
        self.workspaces = workspaces;
        self
    }

    /// Sets the terminal size.
    ///
    pub fn set_terminal_size(&mut self, size: Rect) -> &mut Self {
        self.terminal_size = size;
        self
    }

    /// Advance the spinner index.
    ///
    pub fn advance_spinner_index(&mut self) -> &mut Self {
        self.spinner_index += 1;
        if self.spinner_index >= SPINNER_FRAME_COUNT {
            self.spinner_index = 0;
        }
        self
    }

    /// Return the current spinner index.
    ///
    pub fn get_spinner_index(&self) -> &usize {
        &self.spinner_index
    }

    /// Return the current menu.
    ///
    pub fn current_menu(&self) -> &Menu {
        &self.current_menu
    }

    /// Activate the next menu.
    ///
    pub fn next_menu(&mut self) -> &mut Self {
        match self.current_menu {
            Menu::Status => self.current_menu = Menu::Shortcuts,
            Menu::Shortcuts => self.current_menu = Menu::TopList,
            Menu::TopList => self.current_menu = Menu::Status,
        }
        self
    }

    /// Activate the previous menu.
    ///
    pub fn previous_menu(&mut self) -> &mut Self {
        match self.current_menu {
            Menu::Status => self.current_menu = Menu::TopList,
            Menu::Shortcuts => self.current_menu = Menu::Status,
            Menu::TopList => self.current_menu = Menu::Shortcuts,
        }
        self
    }

    /// Return the current shortcut.
    ///
    pub fn current_shortcut(&self) -> &Shortcut {
        &self.current_shortcut
    }

    /// Activate the next shortcut.
    ///
    pub fn next_shortcut(&mut self) -> &mut Self {
        match self.current_shortcut {
            Shortcut::MyTasks => self.current_shortcut = Shortcut::DueSoon,
            Shortcut::DueSoon => self.current_shortcut = Shortcut::PastDue,
            Shortcut::PastDue => self.current_shortcut = Shortcut::RecentlyCreated,
            Shortcut::RecentlyCreated => self.current_shortcut = Shortcut::RecentlyEdited,
            Shortcut::RecentlyEdited => self.current_shortcut = Shortcut::RecentlyCompleted,
            Shortcut::RecentlyCompleted => self.current_shortcut = Shortcut::MyTasks,
        }
        self
    }

    /// Activate the previous shortcut.
    ///
    pub fn previous_shortcut(&mut self) -> &mut Self {
        match self.current_shortcut {
            Shortcut::MyTasks => self.current_shortcut = Shortcut::RecentlyCompleted,
            Shortcut::RecentlyCompleted => self.current_shortcut = Shortcut::RecentlyEdited,
            Shortcut::RecentlyEdited => self.current_shortcut = Shortcut::RecentlyCreated,
            Shortcut::RecentlyCreated => self.current_shortcut = Shortcut::PastDue,
            Shortcut::PastDue => self.current_shortcut = Shortcut::DueSoon,
            Shortcut::DueSoon => self.current_shortcut = Shortcut::MyTasks,
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::uuid::UUIDv4;
    use fake::{Fake, Faker};
    use uuid::Uuid;

    #[test]
    fn get_user() {
        let user: User = Faker.fake();
        let state = State {
            user: Some(user.to_owned()),
            ..State::default()
        };
        assert_eq!(user, *state.get_user().unwrap());
    }

    #[test]
    fn set_user() {
        let mut state = State::default();
        let user: User = Faker.fake();
        state.set_user(user.to_owned());
        assert_eq!(user, state.user.unwrap());
    }

    #[test]
    fn get_active_workspace() {
        let workspaces = vec![
            Faker.fake::<Workspace>(),
            Faker.fake::<Workspace>(),
            Faker.fake::<Workspace>(),
        ];
        let active_workspace = workspaces[0].to_owned();
        let state = State {
            active_workspace_gid: Some(active_workspace.gid.to_owned()),
            workspaces,
            ..State::default()
        };
        assert_eq!(active_workspace, *state.get_active_workspace().unwrap());
    }

    #[test]
    fn set_active_workspace() {
        let workspace_gid: Uuid = UUIDv4.fake();
        let mut state = State::default();
        state.set_active_workspace(workspace_gid.to_string());
        assert_eq!(
            workspace_gid.to_string(),
            state.active_workspace_gid.unwrap()
        );
    }

    #[test]
    fn set_workspaces() {
        let workspaces = vec![
            Faker.fake::<Workspace>(),
            Faker.fake::<Workspace>(),
            Faker.fake::<Workspace>(),
        ];
        let mut state = State::default();
        state.set_workspaces(workspaces.to_owned());
        assert_eq!(workspaces, state.workspaces);
    }

    #[test]
    fn set_terminal_size() {
        let mut state = State::default();
        let size = Rect::new(Faker.fake(), Faker.fake(), Faker.fake(), Faker.fake());
        state.set_terminal_size(size);
        assert_eq!(size, state.terminal_size);
    }

    #[test]
    fn advance_spinner_index() {
        let mut state = State::default();
        state.advance_spinner_index();
        assert_eq!(state.spinner_index, 1);
        for _ in 0..SPINNER_FRAME_COUNT {
            state.advance_spinner_index();
        }
        assert_eq!(state.spinner_index, 1);
    }

    #[test]
    fn get_spinner_index() {
        let state = State {
            spinner_index: 2,
            ..State::default()
        };
        assert_eq!(*state.get_spinner_index(), 2);
    }

    #[test]
    fn current_menu() {
        let state = State {
            current_menu: Menu::Status,
            ..State::default()
        };
        assert_eq!(*state.current_menu(), Menu::Status);
    }

    #[test]
    fn next_menu() {
        let mut state = State {
            current_menu: Menu::Status,
            ..State::default()
        };
        state.next_menu();
        assert_eq!(state.current_menu, Menu::Shortcuts);
        state.next_menu();
        assert_eq!(state.current_menu, Menu::TopList);
        state.next_menu();
        assert_eq!(state.current_menu, Menu::Status);
    }

    #[test]
    fn previous_menu() {
        let mut state = State {
            current_menu: Menu::Status,
            ..State::default()
        };
        state.previous_menu();
        assert_eq!(state.current_menu, Menu::TopList);
        state.previous_menu();
        assert_eq!(state.current_menu, Menu::Shortcuts);
        state.previous_menu();
        assert_eq!(state.current_menu, Menu::Status);
    }

    #[test]
    fn current_shortcut() {
        let state = State {
            current_shortcut: Shortcut::MyTasks,
            ..State::default()
        };
        assert_eq!(*state.current_shortcut(), Shortcut::MyTasks);
    }

    #[test]
    fn next_shortcut() {
        let mut state = State {
            current_shortcut: Shortcut::MyTasks,
            ..State::default()
        };
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::DueSoon);
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::PastDue);
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyCreated);
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyEdited);
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyCompleted);
        state.next_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::MyTasks);
    }

    #[test]
    fn previous_shortcut() {
        let mut state = State {
            current_shortcut: Shortcut::MyTasks,
            ..State::default()
        };
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyCompleted);
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyEdited);
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::RecentlyCreated);
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::PastDue);
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::DueSoon);
        state.previous_shortcut();
        assert_eq!(state.current_shortcut, Shortcut::MyTasks);
    }
}
