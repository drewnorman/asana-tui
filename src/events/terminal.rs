use crossterm::{
    event,
    event::{Event as CrosstermEvent, KeyEvent},
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
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    _tx: mpsc::Sender<Event<KeyEvent>>,
}

impl Events {
    /// Return new instance after spawning new input polling thread.
    ///
    pub fn new() -> Events {
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
        Events { rx, _tx: tx }
    }

    /// Wait for a value on the events channel.
    ///
    pub fn next(&self) -> Result<Event<crossterm::event::KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
