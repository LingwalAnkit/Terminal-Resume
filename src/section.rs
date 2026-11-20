/// The resume sections, in menu order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    About,
    Skills,
    Projects,
    Experience,
    Education,
    Contact,
}

impl Section {
    pub const ALL: [Section; 6] = [
        Section::About,
        Section::Skills,
        Section::Projects,
        Section::Experience,
        Section::Education,
        Section::Contact,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::About => "About",
            Self::Skills => "Skills",
            Self::Projects => "Projects",
            Self::Experience => "Experience",
            Self::Education => "Education",
            Self::Contact => "Contact",
        }
    }
}
