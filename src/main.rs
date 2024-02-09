use std::{io, sync::mpsc, thread, time::{Duration, Instant}, process::Termination};
use chrono::{Utc, DateTime};
use crossterm::{event, terminal::enable_raw_mode};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tui::backend::CrosstermBackend;


const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home, 
    Pets,
}

impl From<MenuItem> for usize {
    fn from (input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Pets => 1,
        }
    }
}






fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend);
    terminal.clear()?;

    let (tx, rs) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    let handle_input = thread::spawn(move || {
        let mut last_tick = Instant::now();

        // input handler
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }

        }
    });


    let render_tui = thread::spawn(move || {
        loop {
            terminal.draw
        }
    });
}
