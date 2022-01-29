use crate::app::NetworkEventSender;
use crate::asana::{Task, User, Workspace};
use crate::events::network::Event as NetworkEvent;
use crate::ui::SPINNER_FRAME_COUNT;
use log::*;
use tui::layout::Rect;

/// Specifying the different foci.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Focus {
    Menu,
    View,
}

/// Specifying the different menus.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Menu {
    Status,
    Shortcuts,
    TopList,
}

/// Specifying the different views.
///
#[derive(Debug, PartialEq, Eq)]
pub enum View {
    Welcome,
    MyTasks,
    DueSoon,
    PastDue,
    RecentlyCreated,
    RecentlyEdited,
    RecentlyCompleted,
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
    net_sender: Option<NetworkEventSender>,
    user: Option<User>,
    workspaces: Vec<Workspace>,
    active_workspace_gid: Option<String>,
    terminal_size: Rect,
    spinner_index: usize,
    current_focus: Focus,
    current_menu: Menu,
    current_shortcut: Shortcut,
    view_stack: Vec<View>,
    tasks: Vec<Task>,
}

/// Defines default application state.
///
impl Default for State {
    fn default() -> State {
        State {
            net_sender: None,
            user: None,
            workspaces: vec![],
            active_workspace_gid: None,
            terminal_size: Rect::default(),
            spinner_index: 0,
            current_focus: Focus::Menu,
            current_menu: Menu::Shortcuts,
            current_shortcut: Shortcut::MyTasks,
            view_stack: vec![View::Welcome],
            tasks: vec![],
        }
    }
}

impl State {
    pub fn new(net_sender: NetworkEventSender) -> Self {
        State {
            net_sender: Some(net_sender),
            ..State::default()
        }
    }

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

    /// Return the current focus.
    ///
    pub fn current_focus(&self) -> &Focus {
        &self.current_focus
    }

    /// Change focus to the current menu.
    ///
    pub fn focus_menu(&mut self) -> &mut Self {
        self.current_focus = Focus::Menu;
        self
    }

    /// Change focus to the current view.
    ///
    pub fn focus_view(&mut self) -> &mut Self {
        self.current_focus = Focus::View;
        self
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

    /// Select the current shortcut.
    ///
    pub fn select_current_shortcut(&mut self) -> &mut Self {
        self.view_stack.clear();
        match self.current_shortcut {
            Shortcut::MyTasks => {
                self.tasks.clear();
                self.dispatch(NetworkEvent::MyTasks);
                self.view_stack.push(View::MyTasks);
            }
            Shortcut::DueSoon => {
                self.view_stack.push(View::DueSoon);
            }
            Shortcut::PastDue => {
                self.view_stack.push(View::PastDue);
            }
            Shortcut::RecentlyCreated => {
                self.view_stack.push(View::RecentlyCreated);
            }
            Shortcut::RecentlyEdited => {
                self.view_stack.push(View::RecentlyEdited);
            }
            Shortcut::RecentlyCompleted => {
                self.view_stack.push(View::RecentlyCompleted);
            }
        }
        self.focus_view();
        self
    }

    /// Return the current view.
    ///
    pub fn current_view(&self) -> &View {
        self.view_stack.last().unwrap()
    }

    /// Return the list of tasks.
    ///
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Set the list of tasks.
    ///
    pub fn set_tasks(&mut self, tasks: Vec<Task>) -> &mut Self {
        self.tasks = tasks;
        self
    }

    /// Dispatches an asynchronous network event.
    ///
    fn dispatch(&self, event: NetworkEvent) {
        if let Some(net_sender) = &self.net_sender {
            if let Err(err) = net_sender.send(event) {
                error!("Recieved error from network dispatch: {}", err);
            }
        }
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
    fn current_focus() {
        let mut state = State {
            current_focus: Focus::Menu,
            ..State::default()
        };
        assert_eq!(*state.current_focus(), Focus::Menu);
        state.current_focus = Focus::View;
        assert_eq!(*state.current_focus(), Focus::View);
    }

    #[test]
    fn focus_menu() {
        let mut state = State {
            current_focus: Focus::View,
            ..State::default()
        };
        state.focus_menu();
        assert_eq!(state.current_focus, Focus::Menu);
    }

    #[test]
    fn focus_view() {
        let mut state = State {
            current_focus: Focus::Menu,
            ..State::default()
        };
        state.focus_view();
        assert_eq!(state.current_focus, Focus::View);
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

    #[test]
    fn select_current_shortcut() {
        let mut state = State {
            current_shortcut: Shortcut::MyTasks,
            current_focus: Focus::Menu,
            ..State::default()
        };
        state.select_current_shortcut();
        assert_eq!(*state.view_stack.last().unwrap(), View::MyTasks);
        assert_eq!(state.current_focus, Focus::View);
        state.current_shortcut = Shortcut::PastDue;
        state.select_current_shortcut();
        assert_eq!(*state.view_stack.last().unwrap(), View::PastDue);
        assert_eq!(state.current_focus, Focus::View);
    }

    #[test]
    fn current_view() {
        let mut state = State {
            view_stack: vec![View::DueSoon],
            ..State::default()
        };
        assert_eq!(*state.current_view(), View::DueSoon);
        state.view_stack = vec![View::RecentlyCompleted];
        assert_eq!(*state.current_view(), View::RecentlyCompleted);
    }

    #[test]
    fn get_tasks() {
        let tasks = vec![
            Faker.fake::<Task>(),
            Faker.fake::<Task>(),
            Faker.fake::<Task>(),
        ];
        let state = State {
            tasks: tasks.to_owned(),
            ..State::default()
        };
        assert_eq!(tasks, *state.get_tasks());
    }

    #[test]
    fn set_tasks() {
        let mut state = State::default();
        let tasks = vec![
            Faker.fake::<Task>(),
            Faker.fake::<Task>(),
            Faker.fake::<Task>(),
        ];
        state.set_tasks(tasks.to_owned());
        assert_eq!(tasks, state.tasks);
    }
}
