//! Plain-text output. It is used when the interactive view can't run:
//! output is piped, `--plain` is set, or the terminal fails to start.

use crate::data::RESUME;
use crate::section::Section;

/// Renders the whole resume, or only `section` when one is given.
pub fn render(section: Option<Section>) -> String {
    let sections = match section {
        Some(section) => vec![section],
        None => Section::ALL.to_vec(),
    };

    let mut out = String::new();
    for (index, section) in sections.into_iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        push_section(&mut out, section);
    }
    out
}

fn push_section(out: &mut String, section: Section) {
    push_line(out, &format!("== {} ==", section.title()));

    match section {
        Section::About => {
            push_line(out, RESUME.name);
            push_line(out, RESUME.role);
            push_line(out, "");
            push_line(out, RESUME.summary);
            push_line(out, "");
            push_line(out, "Achievements");
            push_bullets(out, RESUME.achievements);
        }
        Section::Skills => {
            for group in RESUME.skills {
                push_line(out, &format!("{}: {}", group.label, group.items.join(", ")));
            }
        }
        Section::Projects => {
            for project in RESUME.projects {
                push_line(out, project.name);
                push_line(out, &format!("  Stack: {}", project.stack.join(", ")));
                push_bullets(out, project.highlights);
            }
        }
        Section::Experience => {
            for job in RESUME.experience {
                push_line(
                    out,
                    &format!("{} | {}, {}", job.title, job.company, job.location),
                );
                push_line(out, &format!("  {}", job.period));
                push_bullets(out, job.highlights);
            }
        }
        Section::Education => {
            for edu in RESUME.education {
                push_line(out, edu.institution);
                push_line(out, edu.degree);
                push_line(out, edu.location);
                push_line(out, &format!("CGPA {} | {}", edu.cgpa, edu.period));
            }
        }
        Section::Contact => {
            let contact = &RESUME.contact;
            push_line(out, &format!("Email: {}", contact.email));
            push_line(out, &format!("Phone: {}", contact.phone));
            push_line(out, &format!("LinkedIn: {}", contact.linkedin));
            push_line(out, &format!("GitHub: {}", contact.github));
        }
    }
}

fn push_line(out: &mut String, text: &str) {
    out.push_str(text);
    out.push('\n');
}

fn push_bullets(out: &mut String, items: &[&str]) {
    for item in items {
        push_line(out, &format!("  - {item}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_render_includes_every_section() {
        let text = render(None);
        for section in Section::ALL {
            assert!(text.contains(&format!("== {} ==", section.title())));
        }
        assert!(text.contains("RPC Framework over QUIC"));
    }

    #[test]
    fn single_section_render_contains_only_that_section() {
        let text = render(Some(Section::Contact));
        assert!(text.contains(RESUME.contact.email));
        assert!(!text.contains("== Projects =="));
    }
}
