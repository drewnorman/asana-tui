use super::Frame;
use crate::state::State;
use tui::{
    layout::Rect,
    widgets::{Block, Borders},
};

/// Render main widget according to state.
///
pub fn main(frame: &mut Frame, size: Rect, _state: &State) {
    let main_widget = Block::default().borders(Borders::ALL);
    frame.render_widget(main_widget, size);
}
