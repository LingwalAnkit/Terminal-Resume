use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    text::Line,
    widgets::Paragraph,
};

use super::{theme, widgets};
use crate::data::{NAME, ROLE};
use crate::section::Section;

const HINT: &str = "Up/Down or j/k to move  ·  Enter to open  ·  q to quit";
const MENU_WIDTH: u16 = 24;

pub fn draw(frame: &mut Frame, selected: usize) {
    let body = widgets::screen(frame, "Resume", HINT);

    let [header, _, list] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Length(Section::ALL.len() as u16),
    ])
    .areas(body);

    let header_lines = vec![
        Line::from(NAME).style(theme::title()),
        Line::from(ROLE).style(theme::muted()),
    ];
    frame.render_widget(
        Paragraph::new(header_lines).alignment(Alignment::Center),
        header,
    );

    // Centre the menu column, but keep its items left-aligned so the ">" lines up.
    let [_, column, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(MENU_WIDTH),
        Constraint::Fill(1),
    ])
    .areas(list);

    let items: Vec<Line> = Section::ALL
        .iter()
        .enumerate()
        .map(|(index, section)| {
            if index == selected {
                Line::from(format!("> {}", section.title())).style(theme::selected())
            } else {
                Line::from(format!("  {}", section.title()))
            }
        })
        .collect();
    frame.render_widget(Paragraph::new(items), column);
}
