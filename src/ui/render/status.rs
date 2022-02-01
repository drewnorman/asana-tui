use super::widgets::spinner;
use super::Frame;
use crate::state::{Focus, Menu, State};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

const BLOCK_TITLE: &str = "Status";

/// Render status widget according to state.
///
pub fn status(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default()
        .title(BLOCK_TITLE)
        .borders(Borders::ALL)
        .border_style(styling::normal_block_border_style());

    if *state.current_focus() == Focus::Menu && *state.current_menu() == Menu::Status {
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
    }

    if state.get_user().is_none() || state.get_active_workspace().is_none() {
        frame.render_widget(spinner::widget(state, size.height).block(block), size);
        return;
    }

    let user = state.get_user().unwrap();
    let workspace = state.get_active_workspace().unwrap();
    let text = vec![
        Spans::from(vec![Span::styled(
            format!("User: {}", &user.name),
            styling::normal_text_style(),
        )]),
        Spans::from(vec![Span::styled(
            format!("Email: {}", &user.email),
            styling::normal_text_style(),
        )]),
        Spans::from(vec![Span::styled(
            format!("Workspace: {}", &workspace.name),
            styling::normal_text_style(),
        )]),
    ];
    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, size);
}
