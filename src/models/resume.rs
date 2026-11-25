use crate::models::organization::Organization;
use crate::models::paragraph::Paragraph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperienceSource {
    Organization(Organization),
    Personal,
}

impl ExperienceSource {
    pub fn name(self) -> &'static str {
        match self {
            Self::Organization(organization) => organization.title(),
            Self::Personal => "Personal Projects",
        }
    }

    pub fn url(self) -> Option<&'static str> {
        match self {
            Self::Organization(organization) => Some(organization.url()),
            Self::Personal => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExperienceData {
    pub source: ExperienceSource,
    pub subtitle: &'static str,
    pub period: &'static str,
    pub details: Option<&'static [Paragraph]>,
}
