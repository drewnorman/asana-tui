use super::Frame;
use crate::state::{CurrentMenu, State};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders},
};

const BLOCK_TITLE: &str = "Shortcuts";

/// Render shortcuts widget according to state.
///
pub fn shortcuts(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default().title(BLOCK_TITLE).borders(Borders::ALL);
    if *state.current_menu() == CurrentMenu::Shortcuts {
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
    }
    frame.render_widget(block, size);
}
