use crate::event::Action;
use crate::section::Section;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    Menu,
    Section(Section),
}

pub struct App {
    screen: Screen,
    /// Index into `Section::ALL` of the highlighted menu item.
    selected: usize,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Welcome,
            selected: 0,
            should_quit: false,
        }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn handle(&mut self, action: Action) {
        if action == Action::Quit {
            self.should_quit = true;
            return;
        }

        match self.screen {
            // The splash advances on any key.
            Screen::Welcome => self.screen = Screen::Menu,
            Screen::Menu => self.handle_menu(action),
            Screen::Section(_) if action == Action::Back => self.screen = Screen::Menu,
            Screen::Section(_) => {}
        }
    }

    fn handle_menu(&mut self, action: Action) {
        let last = Section::ALL.len() - 1;
        match action {
            // Stops at the first and last item instead of wrapping around.
            Action::Up => self.selected = self.selected.saturating_sub(1),
            Action::Down => self.selected = (self.selected + 1).min(last),
            Action::Confirm => {
                if let Some(&section) = Section::ALL.get(self.selected) {
                    self.screen = Screen::Section(section);
                }
            }
            // Esc does nothing on the top-level menu. Use q to quit.
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_menu() -> App {
        let mut app = App::new();
        app.handle(Action::Other);
        app
    }

    #[test]
    fn any_key_leaves_welcome_screen() {
        let app = at_menu();
        assert_eq!(app.screen(), Screen::Menu);
        assert!(!app.should_quit());
    }

    #[test]
    fn quit_works_from_any_screen() {
        let mut app = App::new();
        app.handle(Action::Quit);
        assert!(app.should_quit());

        let mut app = at_menu();
        app.handle(Action::Quit);
        assert!(app.should_quit());
    }

    #[test]
    fn enter_opens_highlighted_section() {
        let mut app = at_menu();
        app.handle(Action::Down);
        app.handle(Action::Down);
        app.handle(Action::Confirm);
        assert_eq!(app.screen(), Screen::Section(Section::Projects));
    }

    #[test]
    fn esc_returns_to_menu_and_keeps_selection() {
        let mut app = at_menu();
        app.handle(Action::Down);
        app.handle(Action::Confirm);
        app.handle(Action::Back);
        assert_eq!(app.screen(), Screen::Menu);
        assert_eq!(app.selected(), 1);
    }

    #[test]
    fn selection_stops_at_both_ends() {
        let mut app = at_menu();
        app.handle(Action::Up);
        assert_eq!(app.selected(), 0);

        for _ in 0..10 {
            app.handle(Action::Down);
        }
        assert_eq!(app.selected(), Section::ALL.len() - 1);
    }

    #[test]
    fn esc_on_menu_does_nothing() {
        let mut app = at_menu();
        app.handle(Action::Back);
        assert_eq!(app.screen(), Screen::Menu);
    }
}
