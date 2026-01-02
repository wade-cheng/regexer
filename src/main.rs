use anyhow::{Context, Result};
use crossterm::event;
use crossterm::event::Event;
use regexer::State;

use std::sync::mpsc::{self, Sender};
use std::thread;

// use crossterm::style::Stylize as _;
//
// for y in 0..40 {
//     for x in 0..150 {
//         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
//             // in this loop we are more efficient by not flushing the buffer.
//             stdout
//                 .queue(cursor::MoveTo(x, y))?
//                 .queue(style::PrintStyledContent("█".magenta()))?;
//         }
//     }
// }
//
// stdout.flush()?;

fn main() -> Result<()> {
    ctrlc::set_handler(regexer::exit_with_cleanup).context("Error setting Ctrl-C handler")?;

    let state = regexer::startup()?;
    let tx = start_logic_thread(state);

    loop {
        if let Ok(e) = event::read() {
            tx.send(e)
                .expect("Could not send an event to the logic thread.");
        }
    }
}

fn start_logic_thread(initial_state: State) -> Sender<Event> {
    let (tx, rx) = mpsc::channel();

    // thead "joined" by exiting the program
    thread::spawn(move || {
        let mut state = initial_state;
        loop {
            let e = rx.recv().expect("Channel shouldn't hang.");
            regexer::handle_event(e);
        }
    });

    tx
}
