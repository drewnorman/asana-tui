use tui::style::{Color, Modifier, Style};

const ASANA_PURPLE: Color = Color::Rgb(164, 153, 237);

/// Return the border style for active blocks.
///
pub fn active_block_border_style() -> Style {
    Style::default().fg(ASANA_PURPLE)
}

/// Return the title style for active blocks.
///
pub fn active_block_title_style() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

/// Return the style for current list items.
///
pub fn current_list_item_style() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

/// Return the style for active list items.
///
pub fn active_list_item_style() -> Style {
    current_list_item_style().fg(ASANA_PURPLE)
}

/// Return the style for normal list items.
///
pub fn normal_list_item_style() -> Style {
    Style::default().fg(Color::White)
}
