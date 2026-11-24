use crate::components::{ListParagraph, TextFragment, TextParagraph};
use crate::content::project::{Date, Genre, Info, Month, Project, ProjectEndDate, ProjectType};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Radorian",
    slug: "/radorian",
    card_thumbnail: "radorian-thumbnail.webp",
    main_contributions: ListParagraph {
        items: &[
            "Third person camera",
            "3D sprite pipeline & tools",
            "Custom shaders",
            "Procedural Geometry",
            "Dynamic gravity",
        ],
    },
    background: None,
    hero_video: Some("tbIsz63srjo"),
    badges: &[],
    info: Info {
        role: "Sole Developer",
        genres: Some(&[Genre::Atmospheric, Genre::Crafting, Genre::ThirdPerson]),
        project_type: ProjectType::PersonalProject,
        platforms: "PC",
        team_size: "Solo",
        duration: "Too Long",
        end_date: ProjectEndDate::Ended(Date {
            day: 21,
            month: Month::December,
            year: 2024,
        }),
        skills: &[
            Skill::Godot,
            Skill::CSharp,
            Skill::Aseprite,
            Skill::Blender,
            Skill::Git,
        ],
        socials: &[],
    },
    description: TextParagraph {
        fragments: &[
            TextFragment::Text("A "),
            TextFragment::StrongText("personal project"),
            TextFragment::Text(
            " I used to work on in my spare time at university. It's a third person game with an \
            experimental art style. I made everything myself from code to art to sound to \
            game design.",
            ),
        ],
    },
    chapters: &[],
};
