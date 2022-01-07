use super::Frame;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

/// Render footer widget.
///
pub fn footer(frame: &mut Frame, size: Rect) {
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
