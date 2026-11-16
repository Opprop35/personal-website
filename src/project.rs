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

/// A calendar date with day, month and year.
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

pub enum Fragment {
    Text(&'static str),
    Emph(&'static str),
    Link {
        url: &'static str,
        label: &'static str,
    },
    List(&'static [&'static str]),
}

pub enum Showcase {
    Video(&'static str),
    Image {
        src: &'static str,
        alt: &'static str,
    },
}

pub struct Section {
    pub heading: Option<&'static str>,
    pub title: &'static str,
    pub body: &'static [Fragment],
    pub showcase: Option<Showcase>,
}

pub struct Link {
    pub image: &'static str,
    pub alt: &'static str,
    pub url: &'static str,
}

pub struct Info {
    pub role: &'static str,
    pub genres: &'static str,
    pub project_type: &'static str,
    pub platforms: &'static str,
    pub team_size: &'static str,
    pub duration: &'static str,
    /// `None` when the project is not released yet.
    pub release_date: Option<Date>,
    pub skills: &'static [Skill],
    pub socials: &'static [Link],
}

impl Info {
    pub fn release_date_label(&self) -> String {
        self.release_date
            .map(|date| date.format())
            .unwrap_or_else(|| "Not Released".to_owned())
    }
}

/// A project, shown as a card on the home page and in full detail on its own page.
pub struct Project {
    /// The title shown on both the card and the detail page.
    pub title: &'static str,
    /// The route the detail page is served from.
    pub href: &'static str,
    /// The card thumbnail, a file name within `public/images`.
    pub image: &'static str,
    /// Short bullet points of contributions shown on the card.
    pub contributions: &'static [&'static str],

    /// The detail page background image, a file name within `public/images`.
    pub background: Option<&'static str>,
    /// The detail page hero trailer (a YouTube video id).
    pub hero_video: Option<&'static str>,
    /// Store & competition badges shown on the detail page.
    pub badges: &'static [Link],
    /// The structured project-info block (role, skills, release date, socials, ...).
    pub info: Info,
    /// The description paragraph, shown on both the card and the detail page.
    pub description: &'static [Fragment],
    /// The detailed content sections shown on the detail page.
    pub sections: &'static [Section],
    /// When set, shows an "Under Construction" banner in place of the sections.
    pub under_construction: bool,
}
