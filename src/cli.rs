use clap::{ArgGroup, Parser};

use crate::section::Section;

/// A terminal-based resume. Run it with no options to open the interactive view.
#[derive(Debug, Parser)]
#[command(name = "resume-terminal", version)]
#[command(group(
    ArgGroup::new("section")
        .args(["about", "skills", "projects", "experience", "education", "contact"])
        .multiple(false)
))]
pub struct Cli {
    /// Open the About section directly
    #[arg(long)]
    about: bool,

    /// Open the Skills section directly
    #[arg(long)]
    skills: bool,

    /// Open the Projects section directly
    #[arg(long)]
    projects: bool,

    /// Open the Experience section directly
    #[arg(long)]
    experience: bool,

    /// Open the Education section directly
    #[arg(long)]
    education: bool,

    /// Open the Contact section directly
    #[arg(long)]
    contact: bool,

    /// Print the resume as plain text instead of the interactive view
    #[arg(long)]
    plain: bool,
}

impl Cli {
    /// The section named by a flag, if any.
    pub fn section(&self) -> Option<Section> {
        if self.about {
            Some(Section::About)
        } else if self.skills {
            Some(Section::Skills)
        } else if self.projects {
            Some(Section::Projects)
        } else if self.experience {
            Some(Section::Experience)
        } else if self.education {
            Some(Section::Education)
        } else if self.contact {
            Some(Section::Contact)
        } else {
            None
        }
    }

    pub fn plain(&self) -> bool {
        self.plain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("resume-terminal").chain(args.iter().copied()))
    }

    #[test]
    fn no_flags_opens_the_interactive_menu() {
        let cli = parse(&[]).expect("no flags should parse");
        assert_eq!(cli.section(), None);
        assert!(!cli.plain());
    }

    #[test]
    fn section_flag_selects_that_section() {
        let cli = parse(&["--projects"]).expect("--projects should parse");
        assert_eq!(cli.section(), Some(Section::Projects));
    }

    #[test]
    fn two_section_flags_are_rejected() {
        assert!(parse(&["--about", "--skills"]).is_err());
    }

    #[test]
    fn plain_flag_is_read() {
        let cli = parse(&["--plain", "--contact"]).expect("should parse");
        assert!(cli.plain());
        assert_eq!(cli.section(), Some(Section::Contact));
    }
}
