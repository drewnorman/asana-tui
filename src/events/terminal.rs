use crate::state::State;
use anyhow::Result;
use crossterm::{
    event,
    event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers},
};
use std::{sync::mpsc, thread, time::Duration};

/// Specify terminal event poll rate in milliseconds.
///
const TICK_RATE_IN_MS: u64 = 60;

/// Specify different terminal event types.
///
pub enum Event<I> {
    Input(I),
    Tick,
}

/// Specify struct for managing terminal events channel.
///
pub struct Handler {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    _tx: mpsc::Sender<Event<KeyEvent>>,
}

impl Handler {
    /// Return new instance after spawning new input polling thread.
    ///
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();
        thread::spawn(move || loop {
            let tick_rate = Duration::from_millis(TICK_RATE_IN_MS);
            if event::poll(tick_rate).unwrap() {
                if let CrosstermEvent::Key(key) = event::read().unwrap() {
                    tx_clone.send(Event::Input(key)).unwrap();
                }
            }
            tx_clone.send(Event::Tick).unwrap();
        });
        Handler { rx, _tx: tx }
    }

    /// Receive next terminal event and handle it accordingly. Returns result
    /// with value true if should continue or false if exit was requested.
    ///
    pub fn handle_next(&self, state: &mut State) -> Result<bool> {
        match self.rx.recv()? {
            Event::Input(event) => match event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }
                | KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                } => return Ok(false),
                _ => (),
            },
            Event::Tick => {
                state.advance_spinner_index();
            }
        }
        Ok(true)
    }
}
