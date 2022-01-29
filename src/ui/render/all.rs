use super::*;
use crate::state::State;
use tui::layout::{Constraint, Direction, Layout, Rect};

/// Render all to terminal frame according to state.
///
pub fn all(frame: &mut Frame, state: &State) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(frame.size());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(rows[0]);

    left(frame, columns[0], state);
    right(frame, columns[1], state);
    footer(frame, rows[1]);
}

/// Render left widgets to terminal frame according to state.
///
fn left(frame: &mut Frame, size: Rect, state: &State) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(size);

    status(frame, rows[0], state);
    shortcuts(frame, rows[1], state);
    top_list(frame, rows[2], state);
}

/// Render right widgets to terminal frame according to state.
///
fn right(frame: &mut Frame, size: Rect, state: &State) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(8)])
        .split(size);

    main(frame, rows[0], state);
    log(frame, rows[1]);
}
