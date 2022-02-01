use super::welcome::{BANNER, CONTENT};
use super::widgets::spinner;
use super::Frame;
use crate::state::{Focus, State, View};
use crate::ui::widgets::styling;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
};

/// Render main widget according to state.
///
pub fn main(frame: &mut Frame, size: Rect, state: &State) {
    match state.current_view() {
        View::Welcome => {
            welcome(frame, size, state);
        }
        View::MyTasks => {
            my_tasks(frame, size, state);
        }
        View::RecentlyModified => {
            recently_modified(frame, size, state);
        }
        View::RecentlyCompleted => {
            recently_completed(frame, size, state);
        }
        View::ProjectTasks => {
            project_tasks(frame, size, state);
        }
    }
}

fn welcome(frame: &mut Frame, size: Rect, state: &State) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Length(94)].as_ref())
        .margin(2)
        .split(size);

    let block = view_block("Welcome", state);
    frame.render_widget(block, size);

    let mut banner = Text::from(BANNER);
    banner.patch_style(styling::banner_style());
    let banner_widget = Paragraph::new(banner);
    frame.render_widget(banner_widget, rows[0]);

    let mut content = Text::from(CONTENT);
    content.patch_style(styling::normal_text_style());
    let content_widget = Paragraph::new(content);
    frame.render_widget(content_widget, rows[1]);
}

fn my_tasks(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("My Tasks", state);
    let list = task_list(state, size).block(block);
    frame.render_widget(list, size);
}

fn recently_modified(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("Recently Modified", state);
    let list = task_list(state, size).block(block);
    frame.render_widget(list, size);
}

fn recently_completed(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("Recently Completed", state);
    let list = task_list(state, size).block(block);
    frame.render_widget(list, size);
}

fn project_tasks(frame: &mut Frame, size: Rect, state: &State) {
    let title = match state.get_project() {
        Some(project) => &project.name,
        None => "Project",
    };
    let block = view_block(title, state);
    let list = task_list(state, size).block(block);
    frame.render_widget(list, size);
}

fn task_list(state: &State, size: Rect) -> Paragraph {
    if state.get_tasks().is_empty() {
        return spinner::widget(state, size.height);
    }
    let items: Vec<Spans> = state
        .get_tasks()
        .iter()
        .map(|t| Spans::from(vec![Span::raw(t.name.to_owned())]))
        .collect();
    let list = Paragraph::new(items).style(styling::normal_text_style());
    list
}

fn view_block<'a>(title: &'a str, state: &State) -> Block<'a> {
    let mut block = Block::default().borders(Borders::ALL);
    if *state.current_focus() == Focus::View {
        block = block.border_style(styling::active_block_border_style());
    }
    block.title(Span::styled(title, styling::active_block_title_style()))
}
