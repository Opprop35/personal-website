use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Level Editor Plugin",
    slug: "level-editor-plugin",
    card_thumbnail: "level-editor-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &["Tool Programming", "Tool Design"],
    },
    background: Some("level-editor-background.webp"),
    hero_video: Some("tEBmK5gDUyk"),
    badges: &[],
    info: models::Info {
        role: "Tool Programmer",
        genres: None,
        project_type: models::ProjectType::PluginsAndTooling,
        platforms: "PC",
        team_size: "Solo",
        duration: "7 weeks",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 21,
            month: models::Month::October,
            year: 2022,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Cpp, models::Skill::Perforce],
        socials: &[],
    },
    description: models::TextParagraph {
        fragments: &[
            models::TextFragment::Text("An Unreal Engine "),
            models::TextFragment::StrongText("editor plugin"),
            models::TextFragment::Text(
                " I made in order to get more familiar with C++. The plugin allows you to draw on a \
            grid and define custom behaviours for this. It was designed with quick iteration \
            and flexibility in mind.",
            ),
        ],
    },
    chapters: &[models::Chapter {
        heading: "Features",
        articles: &[models::Article {
            title: "Custom Blueprint Behaviours",
            body: &[models::Paragraph::Text(models::TextParagraph {
                fragments: &[models::TextFragment::Text(
                    "I defined an interface in C++ that could be implemented through blueprints and \
                    allows you to define your own behaviours when dragging across the grid. By \
                    implementing this interface I created a blueprint that would automatically tile \
                    itself using a free tileset I found online.",
                )],
            })],
            showcase: models::Showcase::Video("-9xbKIwheJ8"),
        }],
    }],
};
