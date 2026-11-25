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
