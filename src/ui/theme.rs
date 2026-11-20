use ratatui::style::{Color, Modifier, Style};

// Restrained palette. Text uses the terminal's own foreground and background
// so the app respects the user's theme.
const ACCENT: Color = Color::Cyan;
const MUTED: Color = Color::DarkGray;

pub fn title() -> Style {
    Style::new().fg(ACCENT).add_modifier(Modifier::BOLD)
}

pub fn heading() -> Style {
    Style::new().add_modifier(Modifier::BOLD)
}

pub fn muted() -> Style {
    Style::new().fg(MUTED)
}

/// Highlight for the selected menu item. Reversing the colours works on any theme.
pub fn selected() -> Style {
    Style::new().add_modifier(Modifier::REVERSED | Modifier::BOLD)
}
