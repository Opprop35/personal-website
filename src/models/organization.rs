#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Organization {
    TheLegoGroup,
    StarbrewGames,
    BredaUniversityOfAppliedSciences,
    TuDelft,
    IswGasthuislaan,
}

impl Organization {
    pub const fn title(self) -> &'static str {
        match self {
            Self::TheLegoGroup => "The LEGO Group",
            Self::StarbrewGames => "Starbrew Games",
            Self::BredaUniversityOfAppliedSciences => "Breda University of Applied Sciences",
            Self::TuDelft => "TU Delft",
            Self::IswGasthuislaan => "ISW Gasthuislaan",
        }
    }

    pub const fn url(self) -> &'static str {
        match self {
            Self::TheLegoGroup => "https://www.lego.com/en-us/aboutus/lego-group",
            Self::StarbrewGames => "https://starbrewgames.com/",
            Self::BredaUniversityOfAppliedSciences => "https://www.buas.nl/en",
            Self::TuDelft => "https://www.tudelft.nl/",
            Self::IswGasthuislaan => "https://iswgasthuislaan.nl/",
        }
    }
}
