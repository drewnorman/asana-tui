use super::Frame;
use crate::state::State;
use tui::{
    layout::Rect,
    widgets::{Block, Borders},
};

/// Render top list widget according to state.
///
pub fn top_list(frame: &mut Frame, size: Rect, _state: &State) {
    let top_list_widget = Block::default().title("Projects").borders(Borders::ALL);
    frame.render_widget(top_list_widget, size);
}
