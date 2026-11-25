use crate::models::organization::Organization;
use crate::models::skill::Skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFragment {
    Text(&'static str),
    StrongText(&'static str),
    MutedText(&'static str),
    Link {
        url: &'static str,
        label: &'static str,
    },
}

impl TextFragment {
    pub const fn skill(skill: Skill) -> Self {
        TextFragment::Link {
            url: skill.url(),
            label: skill.title(),
        }
    }

    pub const fn organization(organization: Organization) -> Self {
        TextFragment::Link {
            url: organization.url(),
            label: organization.title(),
        }
    }
}
