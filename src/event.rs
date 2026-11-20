use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// What a key press means to the app. Screens decide what each action does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Back,
    Up,
    Down,
    Confirm,
    Other,
}

/// Waits up to `timeout` for a key press and maps it to an action.
/// Returns `None` when nothing arrived in time, so the loop can redraw.
pub fn next_action(timeout: Duration) -> Result<Option<Action>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    match event::read()? {
        // Windows also reports key releases. Handle presses only, or each key counts twice.
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Some(map_key(key))),
        _ => Ok(None),
    }
}

fn map_key(key: KeyEvent) -> Action {
    match key.code {
        // Raw mode turns off Ctrl+C's signal, so it has to be handled here.
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Back,
        KeyCode::Up | KeyCode::Char('k') => Action::Up,
        KeyCode::Down | KeyCode::Char('j') => Action::Down,
        KeyCode::Enter => Action::Confirm,
        _ => Action::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new_with_kind(code, modifiers, KeyEventKind::Press)
    }

    #[test]
    fn q_and_ctrl_c_quit() {
        assert_eq!(
            map_key(press(KeyCode::Char('q'), KeyModifiers::NONE)),
            Action::Quit
        );
        assert_eq!(
            map_key(press(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Action::Quit
        );
    }

    #[test]
    fn arrows_and_vim_keys_move() {
        assert_eq!(map_key(press(KeyCode::Up, KeyModifiers::NONE)), Action::Up);
        assert_eq!(
            map_key(press(KeyCode::Char('k'), KeyModifiers::NONE)),
            Action::Up
        );
        assert_eq!(
            map_key(press(KeyCode::Down, KeyModifiers::NONE)),
            Action::Down
        );
        assert_eq!(
            map_key(press(KeyCode::Char('j'), KeyModifiers::NONE)),
            Action::Down
        );
    }

    #[test]
    fn esc_goes_back_and_enter_confirms() {
        assert_eq!(
            map_key(press(KeyCode::Esc, KeyModifiers::NONE)),
            Action::Back
        );
        assert_eq!(
            map_key(press(KeyCode::Enter, KeyModifiers::NONE)),
            Action::Confirm
        );
    }
}
