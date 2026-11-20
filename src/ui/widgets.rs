use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
};

use super::theme;

/// Draws the bordered frame used by the menu and section screens.
/// Returns the body area above the footer, where the screen draws its content.
/// The footer shows `hint` in muted text.
pub fn screen(frame: &mut Frame, title: &str, hint: &str) -> Rect {
    let area = frame.area();
    let block = Block::bordered()
        .border_style(theme::muted())
        .title(Line::from(format!(" {title} ")).style(theme::title()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [body, footer] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(inner);
    frame.render_widget(
        Paragraph::new(hint)
            .style(theme::muted())
            .alignment(Alignment::Center),
        footer,
    );

    body
}
