use crate::state::State;
use tui::{
    layout::Alignment,
    text::{Span, Spans},
    widgets::Paragraph,
};

/// Define frames for loading indicator.
///
pub const FRAMES: &[&str] = &["▄ ▆ ▇", "▄ ▆ ▇", "▆ ▇ ▄", "▆ ▇ ▄", "▇ ▄ ▆", "▇ ▄ ▆"];

/// Build the spinner widget according to state.
///
pub fn widget(state: &State, container_height: u16) -> Paragraph {
    // Remove a line for each border (top and bottom) as well as the line the
    // widget will be drawn on. Finally divide in half.
    let vertical_line_offset = (container_height - 3) / 2;

    let mut text = vec![Spans::from(vec![Span::raw(
        FRAMES[*state.get_spinner_index()],
    )])];
    for _ in 0..vertical_line_offset {
        text.insert(0, Spans::from(vec![Span::raw("")]));
    }
    Paragraph::new(text).alignment(Alignment::Center)
}
