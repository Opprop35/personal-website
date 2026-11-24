use crate::components::{ListParagraph, Paragraph, TextParagraph};
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

pub struct Article {
    pub title: &'static str,
    pub body: &'static [Paragraph],
    pub showcase: Option<Showcase>,
}

pub struct Chapter {
    pub heading: &'static str,
    pub articles: &'static [Article],
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

pub struct Project {
    pub title: &'static str,
    pub href: &'static str,
    pub card_thumbnail: &'static str,

    pub description: TextParagraph,
    pub main_contributions: ListParagraph,
    pub info: Info,

    pub hero_video: Option<&'static str>,
    pub badges: &'static [Link],
    pub background: Option<&'static str>,
    pub chapters: &'static [Chapter],
    pub under_construction: bool,
}
