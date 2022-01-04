use crate::asana::Asana;
use crate::config::Config;
use crate::events::network::{Event as NetworkEvent, Handler as NetworkEventHandler};
use crate::events::terminal::Handler as TerminalEventHandler;
use crate::state::State;
use anyhow::{anyhow, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::*;
use std::io::{self, stdout};
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tui_logger::{init_logger, set_default_level};

type NetworkEventSender = std::sync::mpsc::Sender<NetworkEvent>;
type NetworkEventReceiver = std::sync::mpsc::Receiver<NetworkEvent>;

/// Oversees event processing, state management, and terminal output.
///
pub struct App {
    access_token: String,
    state: Arc<Mutex<State>>,
}

impl App {
    /// Start a new application according to the given configuration. Returns
    /// the result of the application execution.
    ///
    pub async fn start(config: Config) -> Result<()> {
        init_logger(LevelFilter::Info).unwrap();
        set_default_level(LevelFilter::Trace);

        info!("Starting application...");
        let mut app = App {
            access_token: config
                .access_token
                .ok_or(anyhow!("Failed to retrieve access token"))?,
            state: Arc::new(Mutex::new(State::default())),
        };

        let (tx, rx) = std::sync::mpsc::channel::<NetworkEvent>();
        app.start_network(rx)?;
        app.start_ui(tx).await?;

        info!("Exiting application...");
        Ok(())
    }

    /// Start a separate thread for asynchronous state mutations.
    ///
    fn start_network(&self, net_receiver: NetworkEventReceiver) -> Result<()> {
        debug!("Creating new thread for asynchronous networking...");
        let cloned_state = Arc::clone(&self.state);
        let access_token = self.access_token.to_owned();
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let mut asana = Asana::new(&access_token);
                    let mut network_event_handler =
                        NetworkEventHandler::new(&cloned_state, &mut asana);
                    while let Ok(network_event) = net_receiver.recv() {
                        match network_event_handler.handle(network_event).await {
                            Ok(_) => (),
                            Err(e) => error!("Failed to handle network event: {}", e),
                        }
                    }
                })
        });
        Ok(())
    }

    /// Begin the terminal event poll on a separate thread before starting the
    /// render loop on the main thread. Return the result following an exit
    /// request or unrecoverable error.
    ///
    async fn start_ui(&mut self, net_sender: NetworkEventSender) -> Result<()> {
        debug!("Starting user interface on main thread...");
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        terminal.hide_cursor()?;

        net_sender.send(NetworkEvent::Me)?;

        let terminal_event_handler = TerminalEventHandler::new();
        loop {
            let mut state = self.state.lock().await;
            if let Ok(size) = terminal.backend().size() {
                state.set_terminal_size(size);
            };
            terminal.draw(|frame| crate::ui::render::all(frame, &state))?;
            if !terminal_event_handler.handle_next(&mut state)? {
                debug!("Received application exit request.");
                break;
            }
        }

        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }
}
