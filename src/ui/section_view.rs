use ratatui::{
    Frame,
    layout::Margin,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{theme, widgets, wrap::wrap};
use crate::app::App;
use crate::data::RESUME;
use crate::section::Section;

const HINT: &str = "Up/Down or j/k to scroll  ·  Esc to go back  ·  q to quit";
const BULLET: &str = "• ";
const BULLET_INDENT: &str = "  ";
const SIDE_MARGIN: u16 = 2;

pub fn draw(frame: &mut Frame, section: Section, app: &mut App) {
    let body = widgets::screen(frame, section.title(), HINT);
    let area = body.inner(Margin::new(SIDE_MARGIN, 0));

    // Lines are wrapped to the current width up front, so one line is one row.
    let lines = lines_for(section, usize::from(area.width));

    // Stop scrolling once the last line is on screen.
    let max_scroll = lines.len().saturating_sub(usize::from(area.height));
    app.clamp_scroll(u16::try_from(max_scroll).unwrap_or(u16::MAX));

    frame.render_widget(Paragraph::new(lines).scroll((app.scroll(), 0)), area);
}

fn lines_for(section: Section, width: usize) -> Vec<Line<'static>> {
    match section {
        Section::About => about(width),
        Section::Skills => skills(width),
        Section::Projects => projects(width),
        Section::Experience => experience(width),
        Section::Education => education(width),
        Section::Contact => contact(),
    }
}

fn about(width: usize) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(RESUME.name).style(theme::title()),
        Line::from(RESUME.role).style(theme::muted()),
        blank(),
    ];
    lines.extend(wrapped(RESUME.summary, width, "", "", Style::default()));
    lines.push(blank());
    lines.push(heading("Achievements"));
    lines.extend(bullets(RESUME.achievements, width));
    lines
}

fn skills(width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for group in RESUME.skills {
        // The label sits on the first row. Wrapped rows are indented to line up under the items.
        let label = format!("{}: ", group.label);
        let indent = " ".repeat(label.chars().count());
        let items = group.items.join(", ");
        let rows = wrap(
            &items,
            width.saturating_sub(indent.len()).max(1),
            "",
            &indent,
        );

        for (row, text) in rows.into_iter().enumerate() {
            if row == 0 {
                lines.push(Line::from(vec![
                    Span::styled(label.clone(), theme::heading()),
                    Span::raw(text),
                ]));
            } else {
                lines.push(Line::raw(text));
            }
        }
    }
    lines
}

fn projects(width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for (index, project) in RESUME.projects.iter().enumerate() {
        if index > 0 {
            lines.push(blank());
        }
        lines.push(heading(project.name));
        lines.extend(wrapped(
            &project.stack.join(", "),
            width,
            "",
            "",
            theme::muted(),
        ));
        lines.extend(bullets(project.highlights, width));
    }
    lines
}

fn experience(width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for (index, job) in RESUME.experience.iter().enumerate() {
        if index > 0 {
            lines.push(blank());
        }
        lines.push(heading(job.title));
        let meta = format!("{} · {} · {}", job.company, job.location, job.period);
        lines.extend(wrapped(&meta, width, "", "", theme::muted()));
        lines.extend(bullets(job.highlights, width));
    }
    lines
}

fn education(width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for (index, edu) in RESUME.education.iter().enumerate() {
        if index > 0 {
            lines.push(blank());
        }
        lines.push(heading(edu.institution));
        let degree = format!("{} · {}", edu.degree, edu.location);
        lines.extend(wrapped(&degree, width, "", "", theme::muted()));
        let grades = format!("CGPA {} · {}", edu.cgpa, edu.period);
        lines.extend(wrapped(&grades, width, "", "", theme::muted()));
    }
    lines
}

fn contact() -> Vec<Line<'static>> {
    let contact = &RESUME.contact;
    [
        ("Email", contact.email),
        ("Phone", contact.phone),
        ("LinkedIn", contact.linkedin),
        ("GitHub", contact.github),
    ]
    .into_iter()
    .map(|(label, value)| {
        Line::from(vec![
            Span::styled(format!("{label:<10}"), theme::muted()),
            Span::raw(value),
        ])
    })
    .collect()
}

fn heading(text: &str) -> Line<'static> {
    Line::from(text.to_string()).style(theme::heading())
}

fn blank() -> Line<'static> {
    Line::default()
}

fn wrapped(text: &str, width: usize, first: &str, next: &str, style: Style) -> Vec<Line<'static>> {
    wrap(text, width, first, next)
        .into_iter()
        .map(|line| Line::from(line).style(style))
        .collect()
}

fn bullets(items: &[&str], width: usize) -> Vec<Line<'static>> {
    items
        .iter()
        .flat_map(|item| wrapped(item, width, BULLET, BULLET_INDENT, Style::default()))
        .collect()
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;

    fn render(section: Section, width: u16, height: u16, app: &mut App) -> String {
        let mut terminal =
            Terminal::new(TestBackend::new(width, height)).expect("test backend should initialise");
        terminal
            .draw(|frame| draw(frame, section, app))
            .expect("drawing to the test backend should succeed");
        terminal.backend().to_string()
    }

    fn app_on(section_index: usize) -> App {
        let mut app = App::new();
        app.handle(crate::event::Action::Other);
        for _ in 0..section_index {
            app.handle(crate::event::Action::Down);
        }
        app.handle(crate::event::Action::Confirm);
        app
    }

    #[test]
    fn about_shows_name_and_achievements() {
        let screen = render(Section::About, 80, 30, &mut app_on(0));
        assert!(screen.contains(RESUME.name));
        assert!(screen.contains("Achievements"));
    }

    #[test]
    fn projects_show_names_and_stacks() {
        let screen = render(Section::Projects, 80, 40, &mut app_on(2));
        assert!(screen.contains("RPC Framework over QUIC"));
        assert!(screen.contains("Rust, Tokio, Quinn, Rustls"));
    }

    #[test]
    fn contact_shows_email_and_github() {
        let screen = render(Section::Contact, 80, 20, &mut app_on(5));
        assert!(screen.contains(RESUME.contact.email));
        assert!(screen.contains(RESUME.contact.github));
    }

    #[test]
    fn scrolling_stops_at_the_end_of_the_content() {
        let mut app = app_on(0);
        for _ in 0..500 {
            app.handle(crate::event::Action::Down);
        }
        render(Section::About, 60, 10, &mut app);
        assert!(app.scroll() > 0, "a short screen should allow scrolling");
        assert!(
            app.scroll() < 500,
            "scroll should be clamped to the content"
        );
    }
}
