use super::Frame;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_logger::TuiLoggerWidget;

/// Render log widget according to state.
///
pub fn log(frame: &mut Frame, size: Rect) {
    let log_widget: TuiLoggerWidget = TuiLoggerWidget::default()
        .block(Block::default().title("Log").borders(Borders::ALL))
        .style_error(Style::default().fg(Color::Red))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_info(Style::default().fg(Color::Cyan))
        .style_debug(Style::default().fg(Color::Green))
        .style_trace(Style::default().fg(Color::Magenta))
        .output_separator(' ')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(None)
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(log_widget, size);
}
