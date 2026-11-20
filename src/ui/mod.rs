mod menu;
mod section_view;
mod theme;
mod welcome;
mod widgets;
mod wrap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::Paragraph,
};

use crate::app::{App, Screen};

/// Smallest terminal the layouts are designed for.
const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 20;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        draw_too_small(frame, area);
        return;
    }

    match app.screen() {
        Screen::Welcome => welcome::draw(frame),
        Screen::Menu => menu::draw(frame, app.selected()),
        Screen::Section(section) => section_view::draw(frame, section, app),
    }
}

/// Shown instead of the layouts when the window is too small for them.
/// Quitting still works, because key handling doesn't depend on the screen.
fn draw_too_small(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from("Terminal too small").style(theme::title()),
        Line::from(format!(
            "Resize to at least {MIN_WIDTH}x{MIN_HEIGHT} (now {}x{})",
            area.width, area.height
        ))
        .style(theme::muted()),
        Line::from("q to quit").style(theme::muted()),
    ];

    let [_, middle, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(area);
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), middle);
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;

    #[test]
    fn small_terminal_shows_a_resize_message() {
        let mut app = App::new();
        let mut terminal =
            Terminal::new(TestBackend::new(40, 10)).expect("test backend should initialise");
        terminal
            .draw(|frame| draw(frame, &mut app))
            .expect("drawing to the test backend should succeed");

        let screen = terminal.backend().to_string();
        assert!(screen.contains("Terminal too small"));
        assert!(screen.contains("now 40x10"));
    }
}
