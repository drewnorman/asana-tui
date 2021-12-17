use tui::backend;

type Frame<'a> = tui::terminal::Frame<'a, backend::CrosstermBackend<std::io::Stdout>>;

pub mod render;
mod spinner;
mod status;

pub const SPINNER_FRAME_COUNT: usize = spinner::FRAMES.len();

use status::status;
