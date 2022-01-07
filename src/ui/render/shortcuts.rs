use super::Frame;
use crate::state::State;
use tui::{
    layout::Rect,
    widgets::{Block, Borders},
};

/// Render shortcuts widget according to state.
///
pub fn shortcuts(frame: &mut Frame, size: Rect, _state: &State) {
    let shortcuts_widget = Block::default().title("Shortcuts").borders(Borders::ALL);
    frame.render_widget(shortcuts_widget, size);
}
