mod app;
mod data;
mod event;
mod section;
mod terminal;
mod ui;

use std::time::Duration;

use anyhow::Result;

use crate::app::App;
use crate::terminal::TerminalGuard;

/// How long to wait for input before redrawing.
const TICK: Duration = Duration::from_millis(250);

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    // The guard restores the terminal when it drops, including on early `?` returns.
    let mut guard = TerminalGuard::enter()?;
    let mut app = App::new();

    while !app.should_quit() {
        guard.terminal().draw(|frame| ui::draw(frame, &mut app))?;
        if let Some(action) = event::next_action(TICK)? {
            app.handle(action);
        }
    }

    Ok(())
}
