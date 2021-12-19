use super::*;
use crate::state::State;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

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

    sidebar(frame, columns[0], state);
    main(frame, columns[1], state);
    footer(frame, rows[1]);
}

/// Render sidebar to terminal frame according to state.
///
fn sidebar(frame: &mut Frame, size: Rect, state: &State) {
    let shortcuts_widget = Block::default().title("Shortcuts").borders(Borders::ALL);
    let projects_widget = Block::default().title("Projects").borders(Borders::ALL);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(12),
            Constraint::Min(1),
        ])
        .split(size);

    status(frame, rows[0], state);
    frame.render_widget(shortcuts_widget, rows[1]);
    frame.render_widget(projects_widget, rows[2]);
}

/// Render main content to terminal frame according to state.
///
fn main(frame: &mut Frame, size: Rect, _state: &State) {
    let intro_widget = Block::default().borders(Borders::ALL);
    let log_widget = Block::default().title("Log").borders(Borders::ALL);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(8)])
        .split(size);

    frame.render_widget(intro_widget, rows[0]);
    frame.render_widget(log_widget, rows[1]);
}

/// Render footer to terminal frame.
///
fn footer(frame: &mut Frame, size: Rect) {
    let controls_content = Spans::from(vec![Span::styled(
        "j k h l: navigate, enter: select, esc: cancel, q: quit",
        Style::default().fg(Color::Blue),
    )]);
    let controls_widget = Paragraph::new(controls_content).alignment(Alignment::Left);

    let version_content = Spans::from(vec![Span::styled(
        format!(" {}", env!("CARGO_PKG_VERSION")),
        Style::default().fg(Color::Green),
    )]);
    let version_content_width = version_content.width();
    let version_widget = Paragraph::new(version_content).alignment(Alignment::Right);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(version_content_width.try_into().unwrap()),
        ])
        .split(size);

    frame.render_widget(controls_widget, columns[0]);
    frame.render_widget(version_widget, columns[1]);
}
