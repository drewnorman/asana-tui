use crate::state::State;
use tui::{
    backend::Backend,
    style::{Color, Style},
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

    let (name, email) = match &state.user.as_ref() {
        Some(user) => (user.name.as_str(), user.email.as_str()),
        None => ("", ""),
    };

    let spans = Spans::from(vec![
        Span::styled(name, Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled(format!("<{}>", email,), Style::default().fg(Color::Green)),
    ]);
    let paragraph = Paragraph::new(spans).block(block);
    frame.render_widget(paragraph, size);
}
