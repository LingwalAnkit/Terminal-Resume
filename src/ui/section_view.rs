use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::Paragraph,
};

use super::{theme, widgets};
use crate::section::Section;

const HINT: &str = "Esc to go back  ·  q to quit";

pub fn draw(frame: &mut Frame, section: Section) {
    let body = widgets::screen(frame, section.title(), HINT);

    // Stage 2 placeholder. Real content for each section is added in stage 3.
    let [_, middle, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(body);

    frame.render_widget(
        Paragraph::new(format!("{} content arrives in stage 3.", section.title()))
            .style(theme::muted())
            .alignment(Alignment::Center),
        middle,
    );
}
