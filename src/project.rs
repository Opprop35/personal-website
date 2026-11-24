use crate::components::Paragraph;
use crate::skill::Skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn code(self) -> &'static str {
        match self {
            Self::January => "Jan",
            Self::February => "Feb",
            Self::March => "Mar",
            Self::April => "Apr",
            Self::May => "May",
            Self::June => "Jun",
            Self::July => "Jul",
            Self::August => "Aug",
            Self::September => "Sep",
            Self::October => "Oct",
            Self::November => "Nov",
            Self::December => "Dec",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

impl Date {
    pub fn format(self) -> String {
        format!(
            "{day:02}-{month}-{year}",
            day = self.day,
            month = self.month.code(),
            year = self.year
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Genre {
    Action,
    Adventure,
    Arcade,
    Atmospheric,
    Cozy,
    Crafting,
    Exploration,
    Multiplayer,
    Platformer,
    Puzzle,
    Roguelike,
    Stealth,
    Survival,
    ThirdPerson,
    TwinStickShooter,
}

impl Genre {
    pub fn label(self) -> &'static str {
        match self {
            Self::Action => "Action",
            Self::Adventure => "Adventure",
            Self::Arcade => "Arcade",
            Self::Atmospheric => "Atmospheric",
            Self::Cozy => "Cozy",
            Self::Crafting => "Crafting",
            Self::Exploration => "Exploration",
            Self::Multiplayer => "Multiplayer",
            Self::Platformer => "Platformer",
            Self::Puzzle => "Puzzle",
            Self::Roguelike => "Roguelike",
            Self::Stealth => "Stealth",
            Self::Survival => "Survival",
            Self::ThirdPerson => "Third Person",
            Self::TwinStickShooter => "Twin-stick Shooter",
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectEndDate {
    #[allow(dead_code)]
    Ongoing,
    Ended(Date),
}

impl ProjectEndDate {
    pub fn label(self) -> String {
        match self {
            Self::Ongoing => "Ongoing".to_owned(),
            Self::Ended(date) => date.format(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Showcase {
    Video(&'static str),
    Image {
        src: &'static str,
        alt: &'static str,
    },
}

pub enum SectionBlock {
    Heading(&'static str),
    Section {
        title: &'static str,
        body: &'static [Paragraph],
        showcase: Option<Showcase>,
    },
}

pub struct Link {
    pub image: &'static str,
    pub alt: &'static str,
    pub url: &'static str,
}

pub struct Info {
    pub role: &'static str,
    pub genres: Option<&'static [Genre]>,
    pub project_type: ProjectType,
    pub platforms: &'static str,
    pub team_size: &'static str,
    pub duration: &'static str,
    pub end_date: ProjectEndDate,
    pub skills: &'static [Skill],
    pub socials: &'static [Link],
}

/// A project, shown as a card on the home page and in full detail on its own page.
pub struct Project {
    /// The title shown on both the card and the detail page.
    pub title: &'static str,
    /// The route the detail page is served from.
    pub href: &'static str,
    /// The card thumbnail, a file name within `public/images`.
    pub card_thumbnail: &'static str,
    /// Short bullet points of contributions shown on the card.
    pub main_contributions: &'static [Paragraph],

    /// The detail page background image, a file name within `public/images`.
    pub background: Option<&'static str>,
    /// The detail page hero trailer (a YouTube video id).
    pub hero_video: Option<&'static str>,
    /// Store & competition badges shown on the detail page.
    pub badges: &'static [Link],
    /// The structured project-info block (role, skills, end date, socials, ...).
    pub info: Info,
    /// The description paragraphs, shown on both the card and the detail page.
    pub description: &'static [Paragraph],
    /// The detailed content sections shown on the detail page.
    pub sections: &'static [SectionBlock],
    /// When set, shows an "Under Construction" banner in place of the sections.
    pub under_construction: bool,
}
