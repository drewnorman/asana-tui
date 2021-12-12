use crate::config::Config;
use crate::events::network::{Event as NetworkEvent, Handler as NetworkEventHandler};
use crate::events::terminal::{Event as TerminalEvent, Events as TerminalEvents};
use crate::state::State;
use anyhow::{anyhow, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

/// Oversees event processing, state management, and terminal output.
///
pub struct App {
    access_token: String,
    terminal: Option<Terminal<CrosstermBackend<std::io::Stdout>>>,
    state: Arc<Mutex<State>>,
}

impl App {
    /// Start a new application according to the given configuration. Returns
    /// the result of the application execution.
    ///
    pub async fn start(config: Config) -> Result<()> {
        let access_token = config
            .access_token
            .ok_or(anyhow!("Failed to retrieve access token"))?;

        let mut app = App {
            access_token,
            terminal: None,
            state: Arc::new(Mutex::new(State::new())),
        };

        app.start_network()?;
        app.start_ui().await?;

        Ok(())
    }

    /// Start a separate thread for asynchronous state mutations.
    ///
    fn start_network(&self) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<NetworkEvent>();
        let cloned_state = Arc::clone(&self.state);
        let access_token = self.access_token.to_owned();
        std::thread::spawn(move || {
            tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let mut network_event_handler =
                        NetworkEventHandler::new(&cloned_state, &access_token);
                    while let Ok(network_event) = rx.recv() {
                        network_event_handler.handle(network_event).await;
                    }
                })
        });

        if let Err(e) = tx.send(NetworkEvent::Me) {
            println!("Error from dispatch {}", e);
            // TODO: Handle error
        };
        Ok(())
    }

    /// Begin the terminal event poll on a separate thread before starting the
    /// render loop on the main thread. Return the result following an exit
    /// request or unrecoverable error.
    ///
    async fn start_ui(&mut self) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        terminal.hide_cursor()?;
        self.terminal = Some(terminal);

        let terminal_events = TerminalEvents::new();
        loop {
            let state = self.state.lock().await;
            self.terminal
                .as_mut()
                .unwrap()
                .draw(|frame| crate::render::render(frame, &state))?;
            match terminal_events.next()? {
                TerminalEvent::Input(event) => match event {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    }
                    | KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    } => break,
                    _ => (),
                },
                TerminalEvent::Tick => {}
            }
        }

        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }
}
