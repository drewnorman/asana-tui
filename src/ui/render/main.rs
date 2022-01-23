use super::Frame;
use crate::state::{Focus, State};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders},
};

/// Render main widget according to state.
///
pub fn main(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default().borders(Borders::ALL);
    let block_title = state.current_view().title();

    if *state.current_focus() == Focus::View {
        block = block.border_style(styling::active_block_border_style());
    }
    block = block.title(Span::styled(
        block_title,
        styling::active_block_title_style(),
    ));

    frame.render_widget(block, size);
}
