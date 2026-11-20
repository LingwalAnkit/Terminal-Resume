mod menu;
mod section_view;
mod theme;
mod welcome;
mod widgets;

use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen() {
        Screen::Welcome => welcome::draw(frame),
        Screen::Menu => menu::draw(frame, app.selected()),
        Screen::Section(section) => section_view::draw(frame, section),
    }
}
