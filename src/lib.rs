use std::process::exit;

use crossterm::{ExecutableCommand as _, cursor, event::Event, terminal};

/// Clean up the terminal and exit with code 0.
pub fn exit_with_cleanup() {
    #[rustfmt::skip]
    std::io::stdout()
        .execute(cursor::MoveTo(0, 0)).unwrap()
        .execute(cursor::Show).unwrap();

    exit(0);
}

/// State of the tui.
pub struct State;

/// Run startup code and return initial state.
pub fn startup() -> std::io::Result<State> {
    std::io::stdout()
        .execute(cursor::Hide)?
        .execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(State)
}

pub fn handle_event(e: Event) {
    // todo!()
}
