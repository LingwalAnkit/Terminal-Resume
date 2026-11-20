use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    cursor::Show,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Owns the terminal in raw mode on the alternate screen.
/// Dropping it restores the terminal, so errors and early returns leave it usable.
pub struct TerminalGuard {
    terminal: Tui,
}

impl TerminalGuard {
    pub fn enter() -> Result<Self> {
        install_panic_hook();

        enable_raw_mode().context("failed to enable raw mode")?;

        let mut stdout = io::stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen) {
            let _ = restore();
            return Err(err).context("failed to enter alternate screen");
        }

        match Terminal::new(CrosstermBackend::new(stdout)) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(err) => {
                let _ = restore();
                Err(err).context("failed to create terminal")
            }
        }
    }

    pub fn terminal(&mut self) -> &mut Tui {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = restore();
    }
}

/// Puts the terminal back the way it was. Safe to call more than once.
pub fn restore() -> io::Result<()> {
    let raw_result = disable_raw_mode();
    let screen_result = execute!(io::stdout(), LeaveAlternateScreen, Show);
    raw_result.and(screen_result)
}

/// Restores the terminal before a panic message is printed.
/// Otherwise the message would be written onto the alternate screen and lost.
fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore();
        original(info);
    }));
}
