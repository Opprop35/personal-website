#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    IndieGame,
    PersonalProject,
    StudentGame,
    PluginsAndTooling,
}

impl ProjectType {
    pub fn label(self) -> &'static str {
        match self {
            Self::IndieGame => "Indie Game",
            Self::PersonalProject => "Personal Project",
            Self::StudentGame => "Student Game",
            Self::PluginsAndTooling => "Plugins & Tooling",
        }
    }
}
