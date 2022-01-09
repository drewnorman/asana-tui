use super::Frame;
use crate::state::State;
use tui::{
    layout::Rect,
    widgets::{Block, Borders},
};

/// Render main widget according to state.
///
pub fn main(frame: &mut Frame, size: Rect, _state: &State) {
    let block = Block::default().borders(Borders::ALL);
    frame.render_widget(block, size);
}
