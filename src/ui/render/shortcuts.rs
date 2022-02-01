use super::Frame;
use crate::state::{Focus, Menu, State, SHORTCUTS};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

const BLOCK_TITLE: &str = "Shortcuts";

/// Render shortcuts widget according to state.
///
pub fn shortcuts(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default()
        .title(BLOCK_TITLE)
        .borders(Borders::ALL)
        .border_style(styling::normal_block_border_style());

    let mut list_item_style = styling::current_list_item_style();
    if *state.current_focus() == Focus::Menu && *state.current_menu() == Menu::Shortcuts {
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
        list_item_style = styling::active_list_item_style();
    }

    let text: Vec<Spans> = SHORTCUTS
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = match i == *state.current_shortcut_index() {
                true => list_item_style,
                false => styling::normal_text_style(),
            };
            Spans::from(vec![Span::styled(s.to_owned(), style)])
        })
        .collect();

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, size);
}
