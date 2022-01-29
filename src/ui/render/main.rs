use super::Frame;
use crate::state::{Focus, State, View};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::{Span, Spans},
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
    }
}

fn welcome(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("Welcome", state);
    frame.render_widget(block, size);
}

fn my_tasks(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("My Tasks", state);
    let list = task_list(state).block(block);
    frame.render_widget(list, size);
}

fn recently_modified(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("Recently Modified", state);
    let list = task_list(state).block(block);
    frame.render_widget(list, size);
}

fn recently_completed(frame: &mut Frame, size: Rect, state: &State) {
    let block = view_block("Recently Completed", state);
    let list = task_list(state).block(block);
    frame.render_widget(list, size);
}

fn task_list(state: &State) -> Paragraph {
    let items: Vec<Spans> = state
        .get_tasks()
        .iter()
        .map(|t| Spans::from(vec![Span::raw(t.name.to_owned())]))
        .collect();
    let list = Paragraph::new(items).style(styling::normal_list_item_style());
    list
}

fn view_block<'a>(title: &'a str, state: &State) -> Block<'a> {
    let mut block = Block::default().borders(Borders::ALL);
    if *state.current_focus() == Focus::View {
        block = block.border_style(styling::active_block_border_style());
    }
    block.title(Span::styled(title, styling::active_block_title_style()))
}
