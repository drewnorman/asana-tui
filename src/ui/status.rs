use super::Frame;
use crate::state::State;
use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

/// Render status widget according to state.
///
pub fn status(frame: &mut Frame, size: Rect, state: &State) {
    let block = Block::default().title("Status").borders(Borders::ALL);

    if state.get_user().is_none() || state.get_active_workspace().is_none() {
        frame.render_widget(block, size);
        return;
    }

    let user = state.get_user().unwrap();
    let workspace = state.get_active_workspace().unwrap();

    let text = vec![
        Spans::from(vec![Span::raw(format!(
            "User: {} <{}>",
            &user.name, &user.email
        ))]),
        Spans::from(vec![Span::raw("Workspace: "), Span::raw(&workspace.name)]),
    ];

    frame.render_widget(Paragraph::new(text).block(block), size);
}
