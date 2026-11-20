use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph},
};

use crate::data::RESUME;

use super::theme;

const HINT: &str = "Press any key to continue  ·  q to quit";

pub fn draw(frame: &mut Frame) {
    let area = frame.area();
    let border = Block::bordered().border_style(theme::muted());
    let inner = border.inner(area);
    frame.render_widget(border, area);

    let [_, name_area, role_area, _, hint_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new(RESUME.name)
            .style(theme::title())
            .alignment(Alignment::Center),
        name_area,
    );
    frame.render_widget(
        Paragraph::new(RESUME.role).alignment(Alignment::Center),
        role_area,
    );
    frame.render_widget(
        Paragraph::new(HINT)
            .style(theme::muted())
            .alignment(Alignment::Center),
        hint_area,
    );
}
