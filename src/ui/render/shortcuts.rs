use super::Frame;
use crate::state::{Focus, Menu, Shortcut, State};
use crate::ui::widgets::styling;
use tui::{
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

const BLOCK_TITLE: &str = "Shortcuts";

/// Render shortcuts widget according to state.
///
pub fn shortcuts(frame: &mut Frame, size: Rect, state: &State) {
    let mut block = Block::default().title(BLOCK_TITLE).borders(Borders::ALL);

    let mut my_tasks_style = Style::default();
    let mut due_soon_style = Style::default();
    let mut past_due_style = Style::default();
    let mut recently_created_style = Style::default();
    let mut recently_edited_style = Style::default();
    let mut recently_completed_style = Style::default();

    let mut list_item_style = styling::current_list_item_style();
    if *state.current_focus() == Focus::Menu && *state.current_menu() == Menu::Shortcuts {
        block = block
            .border_style(styling::active_block_border_style())
            .title(Span::styled(
                BLOCK_TITLE,
                styling::active_block_title_style(),
            ));
        list_item_style = styling::active_list_item_style();
    }

    match state.current_shortcut() {
        Shortcut::MyTasks => {
            my_tasks_style = list_item_style;
        }
        Shortcut::DueSoon => {
            due_soon_style = list_item_style;
        }
        Shortcut::PastDue => {
            past_due_style = list_item_style;
        }
        Shortcut::RecentlyCreated => {
            recently_created_style = list_item_style;
        }
        Shortcut::RecentlyEdited => {
            recently_edited_style = list_item_style;
        }
        Shortcut::RecentlyCompleted => {
            recently_completed_style = list_item_style;
        }
    };

    let text = vec![
        Spans::from(vec![Span::styled("My Tasks", my_tasks_style)]),
        Spans::from(vec![Span::styled("Due Soon", due_soon_style)]),
        Spans::from(vec![Span::styled("Past Due", past_due_style)]),
        Spans::from(vec![Span::styled(
            "Recently Created",
            recently_created_style,
        )]),
        Spans::from(vec![Span::styled("Recently Edited", recently_edited_style)]),
        Spans::from(vec![Span::styled(
            "Recently Completed",
            recently_completed_style,
        )]),
    ];
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, size);
}
