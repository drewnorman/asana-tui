use crate::ui::color::*;
use tui::style::{Color, Modifier, Style};

/// Return the border style for active blocks.
///
pub fn active_block_border_style() -> Style {
    Style::default().fg(PURPLE)
}

/// Return the border style for normal blocks.
///
pub fn normal_block_border_style() -> Style {
    Style::default().fg(Color::White)
}

/// Return the title style for active blocks.
///
pub fn active_block_title_style() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

/// Return the style for current list items.
///
pub fn current_list_item_style() -> Style {
    Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

/// Return the style for active list items.
///
pub fn active_list_item_style() -> Style {
    current_list_item_style().fg(PURPLE)
}

/// Return the style for normal text.
///
pub fn normal_text_style() -> Style {
    Style::default().fg(Color::White)
}

/// Return the style for the banner.
///
pub fn banner_style() -> Style {
    Style::default().fg(PINK)
}
