use super::widgets::spinner;
use super::Frame;
use crate::state::{Focus, Menu, State};
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
    let list_item_style;
    if *state.current_focus() == Focus::Menu && *state.current_menu() == Menu::TopList {
        list_item_style = styling::active_list_item_style();
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
    } else {
        list_item_style = styling::current_list_item_style();
    }

    if state.get_projects().len() == 0 {
        frame.render_widget(spinner::widget(state, size.height).block(block), size);
        return;
    }

    let items: Vec<Spans> = state
        .get_projects()
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let span;
            if i == *state.current_top_list_item() {
                span = Span::styled(p.name.to_owned(), list_item_style);
            } else {
                span = Span::raw(p.name.to_owned());
            }
            Spans::from(vec![span])
        })
        .collect();
    let list = Paragraph::new(items)
        .style(styling::normal_list_item_style())
        .block(block);
    frame.render_widget(list, size);
}
