use super::Frame;
use crate::ui::color::*;
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
        .style_error(Style::default().fg(PINK))
        .style_warn(Style::default().fg(YELLOW))
        .style_info(Style::default().fg(BLUE))
        .style_debug(Style::default().fg(GREEN))
        .style_trace(Style::default().fg(PURPLE))
        .output_separator(' ')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(None)
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(log_widget, size);
}
