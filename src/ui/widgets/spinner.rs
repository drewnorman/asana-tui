use crate::state::State;
use tui::{
    layout::Alignment,
    text::{Span, Spans},
    widgets::Paragraph,
};

/// Define frames for loading indicator.
///
pub const FRAMES: &[&str] = &["▁ ▂ ▃", "▁ ▂ ▃", "▂ ▃ ▁", "▂ ▃ ▁", "▃ ▁ ▂", "▃ ▁ ▂"];

/// Build the spinner widget according to state.
///
pub fn widget(state: &State) -> Paragraph {
    let text = vec![Spans::from(vec![Span::raw(
        FRAMES[*state.get_spinner_index()],
    )])];
    Paragraph::new(text).alignment(Alignment::Center)
}
