use crate::state::State;
use tui::{
    backend::Backend,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

type Frame<'a, B> = tui::terminal::Frame<'a, B>;

/// Render to terminal frame according to state.
///
pub fn render<B: Backend>(frame: &mut Frame<B>, state: &State) {
    render_status(frame, state);
}

/// Render status block to terminal frame according to state.
///
fn render_status<B: Backend>(frame: &mut Frame<B>, state: &State) {
    let frame_size = frame.size();
    let size = tui::layout::Rect::new(
        frame_size.x,
        frame_size.y,
        frame_size.width,
        frame_size.height / 8,
    );
    let block = Block::default().title("Status").borders(Borders::ALL);

    if state.get_user().is_none() || state.get_active_workspace().is_none() {
        frame.render_widget(block, size);
        return;
    }

    let user = state.get_user().as_ref().unwrap();
    let workspace = state.get_active_workspace().unwrap();

    let text = vec![
        Spans::from(vec![Span::raw(format!(
            "User: {} <{}>",
            &user.name, &user.email
        ))]),
        Spans::from(vec![Span::raw("Workspace: "), Span::raw(&workspace.name)]),
    ];

    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, size);
}
