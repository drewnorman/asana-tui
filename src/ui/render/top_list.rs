use super::Frame;
use crate::state::{Menu, State};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

const BLOCK_TITLE: &str = "Projects";

/// Render top list widget according to state.
///
pub fn top_list(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default().title(BLOCK_TITLE).borders(Borders::ALL);
    if *state.current_menu() == Menu::TopList {
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
    }
    let items: Vec<Spans> = state
        .get_projects()
        .iter()
        .map(|p| Spans::from(vec![Span::raw(p.name.to_owned())]))
        .collect();
    let list = Paragraph::new(items)
        .style(styling::normal_list_item_style())
        .block(block);
    frame.render_widget(list, size);
}
