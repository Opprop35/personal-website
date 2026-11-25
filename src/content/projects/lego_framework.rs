use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "LEGO® Framework",
    slug: "lego-framework",
    card_thumbnail: "lego-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &[
            "Brick Snapping Logic & Tooling",
            "Connection Point Based Transform Gizmo",
            "Import Pipeline Redesign",
        ],
    },
    background: Some("lego-background.webp"),
    hero_video: Some("7J7J0zm5xFc"),
    badges: &[],
    info: models::Info {
        role: "Tools Programmer",
        genres: None,
        project_type: models::ProjectType::PluginsAndTooling,
        platforms: "PC",
        team_size: "~5",
        duration: "2 Years",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 1,
            month: models::Month::August,
            year: 2026,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Cpp, models::Skill::Git, models::Skill::Perforce],
        socials: &[],
    },
    description: models::TextParagraph {
        fragments: &[
            models::TextFragment::Text("At "),
            models::TextFragment::organization(models::Organization::TheLegoGroup),
            models::TextFragment::Text(
                " I worked on a general purpose LEGO framework for Unreal Engine. \
            The LEGO® Brick Editor was a key deliverable. I mainly contributed to the Brick \
            building UX, Bug fixes, Automated Tests, and ",
            ),
            models::TextFragment::StrongText("redesigned"),
            models::TextFragment::Text(
                " the framework's import pipeline to be more idiomatic, performant and flexible.",
            ),
        ],
    },
    chapters: &[],
};
