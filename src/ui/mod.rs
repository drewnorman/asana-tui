use tui::backend;

type Frame<'a> = tui::terminal::Frame<'a, backend::CrosstermBackend<std::io::Stdout>>;

pub mod render;
mod status;

use status::status;
