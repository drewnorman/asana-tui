use crate::app::NetworkEventSender;
use crate::asana::{Project, Task, User, Workspace};
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
    RecentlyModified,
    RecentlyCompleted,
    ProjectTasks,
}

/// Specifying the different shortcuts.
///
pub const SHORTCUTS: [&str; 3] = ["My Tasks", "Recently Modified", "Recently Completed"];

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
    current_shortcut_index: usize,
    current_top_list_item: usize,
    view_stack: Vec<View>,
    tasks: Vec<Task>,
    projects: Vec<Project>,
    project: Option<Project>,
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
            current_shortcut_index: 0,
            current_top_list_item: 0,
            view_stack: vec![View::Welcome],
            tasks: vec![],
            projects: vec![],
            project: None,
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
    pub fn current_shortcut_index(&self) -> &usize {
        &self.current_shortcut_index
    }

    /// Activate the next shortcut.
    ///
    pub fn next_shortcut_index(&mut self) -> &mut Self {
        self.current_shortcut_index += 1;
        if self.current_shortcut_index >= SHORTCUTS.len() {
            self.current_shortcut_index = 0;
        }
        self
    }

    /// Activate the previous shortcut.
    ///
    pub fn previous_shortcut_index(&mut self) -> &mut Self {
        if self.current_shortcut_index > 0 {
            self.current_shortcut_index -= 1;
        } else {
            self.current_shortcut_index = SHORTCUTS.len() - 1;
        }
        self
    }

    /// Select the current shortcut.
    ///
    pub fn select_current_shortcut_index(&mut self) -> &mut Self {
        self.view_stack.clear();
        match SHORTCUTS[self.current_shortcut_index] {
            "My Tasks" => {
                self.tasks.clear();
                self.dispatch(NetworkEvent::MyTasks);
                self.view_stack.push(View::MyTasks);
            }
            "Recently Modified" => {
                self.tasks.clear();
                self.view_stack.push(View::RecentlyModified);
            }
            "Recently Completed" => {
                self.tasks.clear();
                self.view_stack.push(View::RecentlyCompleted);
            }
            _ => (),
        }
        self.focus_view();
        self
    }

    /// Activate the next top list item.
    ///
    pub fn next_top_list_item(&mut self) -> &mut Self {
        self.current_top_list_item += 1;
        if self.current_top_list_item >= self.projects.len() {
            self.current_top_list_item = 0;
        }
        self
    }

    /// Activate the previous top list item.
    ///
    pub fn previous_top_list_item(&mut self) -> &mut Self {
        if self.current_top_list_item > 0 {
            self.current_top_list_item -= 1;
        } else {
            self.current_top_list_item = self.projects.len() - 1;
        }
        self
    }

    /// Return the current top list item.
    ///
    pub fn current_top_list_item(&self) -> &usize {
        &self.current_top_list_item
    }

    /// Select the current top list item.
    ///
    pub fn select_current_top_list_item(&mut self) -> &mut Self {
        if self.projects.is_empty() {
            return self;
        }
        self.project = Some(self.projects[self.current_top_list_item].to_owned());
        self.view_stack.clear();
        self.tasks.clear();
        self.dispatch(NetworkEvent::ProjectTasks);
        self.view_stack.push(View::ProjectTasks);
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

    /// Return the list of projects.
    ///
    pub fn get_projects(&self) -> &Vec<Project> {
        &self.projects
    }

    /// Set the list of projects.
    ///
    pub fn set_projects(&mut self, projects: Vec<Project>) -> &mut Self {
        self.projects = projects;
        self
    }

    /// Return the current project.
    ///
    pub fn get_project(&self) -> Option<&Project> {
        self.project.as_ref()
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
    fn current_shortcut_index() {
        let state = State {
            current_shortcut_index: 0,
            ..State::default()
        };
        assert_eq!(*state.current_shortcut_index(), 0);
    }

    #[test]
    fn next_shortcut_index() {
        let mut state = State {
            current_shortcut_index: 0,
            ..State::default()
        };
        state.next_shortcut_index();
        assert_eq!(state.current_shortcut_index, 1);
        state.next_shortcut_index();
        assert_eq!(state.current_shortcut_index, 2);
        state.next_shortcut_index();
        assert_eq!(state.current_shortcut_index, 0);
    }

    #[test]
    fn previous_shortcut_index() {
        let mut state = State {
            current_shortcut_index: 0,
            ..State::default()
        };
        state.previous_shortcut_index();
        assert_eq!(state.current_shortcut_index, 2);
        state.previous_shortcut_index();
        assert_eq!(state.current_shortcut_index, 1);
        state.previous_shortcut_index();
        assert_eq!(state.current_shortcut_index, 0);
    }

    #[test]
    fn select_current_shortcut_index() {
        let mut state = State {
            current_shortcut_index: 0,
            current_focus: Focus::Menu,
            ..State::default()
        };
        state.select_current_shortcut_index();
        assert_eq!(*state.view_stack.last().unwrap(), View::MyTasks);
        assert_eq!(state.current_focus, Focus::View);
        state.current_shortcut_index = 1;
        state.select_current_shortcut_index();
        assert_eq!(*state.view_stack.last().unwrap(), View::RecentlyModified);
        assert_eq!(state.current_focus, Focus::View);
    }

    #[test]
    fn current_top_list_item() {
        let state = State {
            current_top_list_item: 2,
            ..State::default()
        };
        assert_eq!(*state.current_top_list_item(), 2);
    }

    #[test]
    fn next_top_list_item() {
        let projects = vec![Faker.fake::<Project>(), Faker.fake::<Project>()];
        let mut state = State {
            current_top_list_item: 0,
            projects,
            ..State::default()
        };
        state.next_top_list_item();
        assert_eq!(state.current_top_list_item, 1);
        state.next_top_list_item();
        assert_eq!(state.current_top_list_item, 0);
    }

    #[test]
    fn previous_top_list_item() {
        let projects = vec![Faker.fake::<Project>(), Faker.fake::<Project>()];
        let mut state = State {
            current_top_list_item: 0,
            projects,
            ..State::default()
        };
        state.previous_top_list_item();
        assert_eq!(state.current_top_list_item, 1);
        state.previous_top_list_item();
        assert_eq!(state.current_top_list_item, 0);
    }

    #[test]
    fn current_view() {
        let mut state = State {
            view_stack: vec![View::MyTasks],
            ..State::default()
        };
        assert_eq!(*state.current_view(), View::MyTasks);
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

    #[test]
    fn get_projects() {
        let projects = vec![
            Faker.fake::<Project>(),
            Faker.fake::<Project>(),
            Faker.fake::<Project>(),
        ];
        let state = State {
            projects: projects.to_owned(),
            ..State::default()
        };
        assert_eq!(projects, *state.get_projects());
    }

    #[test]
    fn set_projects() {
        let mut state = State::default();
        let projects = vec![
            Faker.fake::<Project>(),
            Faker.fake::<Project>(),
            Faker.fake::<Project>(),
        ];
        state.set_projects(projects.to_owned());
        assert_eq!(projects, state.projects);
    }
}
