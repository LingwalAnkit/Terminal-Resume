mod app;
mod cli;
mod data;
mod event;
mod fallback;
mod section;
mod terminal;
mod ui;

use std::io::{self, IsTerminal};
use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::cli::Cli;
use crate::section::Section;
use crate::terminal::TerminalGuard;

/// How long to wait for input before redrawing.
const TICK: Duration = Duration::from_millis(250);

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start = cli.section();

    // Piped output, or an explicit --plain, gets plain text instead of the interactive view.
    if cli.plain() || !io::stdout().is_terminal() {
        print!("{}", fallback::render(start));
        return Ok(());
    }

    run(start)
}

fn run(start: Option<Section>) -> Result<()> {
    // The guard restores the terminal when it drops, including on early `?` returns.
    let mut guard = match TerminalGuard::enter() {
        Ok(guard) => guard,
        Err(err) => {
            eprintln!("Interactive view unavailable ({err:#}). Showing plain text instead.");
            print!("{}", fallback::render(start));
            return Ok(());
        }
    };
    let mut app = App::new_at(start);

    while !app.should_quit() {
        guard.terminal().draw(|frame| ui::draw(frame, &mut app))?;
        if let Some(action) = event::next_action(TICK)? {
            app.handle(action);
        }
    }

    Ok(())
}
