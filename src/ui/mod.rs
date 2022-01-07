use tui::backend;

type Frame<'a> = tui::terminal::Frame<'a, backend::CrosstermBackend<std::io::Stdout>>;

mod render;
mod widgets;

pub const SPINNER_FRAME_COUNT: usize = widgets::spinner::FRAMES.len();

pub use render::render;
