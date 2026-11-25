use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Radorian",
    slug: "radorian",
    card_thumbnail: "radorian-thumbnail.webp",
    main_contributions: models::ListParagraph {
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
    info: models::Info {
        role: "Sole Developer",
        genres: Some(&[models::Genre::Atmospheric, models::Genre::Crafting, models::Genre::ThirdPerson]),
        project_type: models::ProjectType::PersonalProject,
        platforms: "PC",
        team_size: "Solo",
        duration: "Too Long",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 21,
            month: models::Month::December,
            year: 2024,
        }),
        skills: &[
            models::Skill::Godot,
            models::Skill::CSharp,
            models::Skill::Aseprite,
            models::Skill::Blender,
            models::Skill::Git,
        ],
        socials: &[],
    },
    description: models::TextParagraph {
        fragments: &[
            models::TextFragment::Text("A "),
            models::TextFragment::StrongText("personal project"),
            models::TextFragment::Text(
                " I used to work on in my spare time at university. It's a third person game with an \
            experimental art style. I made everything myself from code to art to sound to \
            game design.",
            ),
        ],
    },
    chapters: &[],
};
