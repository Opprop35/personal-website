use crate::content::project::Project;

pub mod attic_panic;
pub mod duck_side_of_the_moon;
pub mod duckside_of_the_moon_student;
pub mod folivoras_fall;
pub mod lego_framework;
pub mod level_editor_plugin;
pub mod radorian;
pub mod twin_stick_duckbox;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectId {
    LegoFramework,
    DuckSideOfTheMoon,
    DucksideOfTheMoonStudent,
    AtticPanic,
    Radorian,
    LevelEditorPlugin,
    TwinStickDuckbox,
    FolivorasFall,
}

impl ProjectId {
    pub const ALL: [Self; 8] = [
        Self::LegoFramework,
        Self::DuckSideOfTheMoon,
        Self::DucksideOfTheMoonStudent,
        Self::AtticPanic,
        Self::Radorian,
        Self::LevelEditorPlugin,
        Self::TwinStickDuckbox,
        Self::FolivorasFall,
    ];

    pub const fn project(self) -> &'static Project {
        match self {
            Self::LegoFramework => &lego_framework::PROJECT,
            Self::DuckSideOfTheMoon => &duck_side_of_the_moon::PROJECT,
            Self::DucksideOfTheMoonStudent => &duckside_of_the_moon_student::PROJECT,
            Self::AtticPanic => &attic_panic::PROJECT,
            Self::Radorian => &radorian::PROJECT,
            Self::LevelEditorPlugin => &level_editor_plugin::PROJECT,
            Self::TwinStickDuckbox => &twin_stick_duckbox::PROJECT,
            Self::FolivorasFall => &folivoras_fall::PROJECT,
        }
    }
}
