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
