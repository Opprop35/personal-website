use crate::components::{ListParagraph, Paragraph, TextFragment, TextParagraph};
use crate::content::project::{
    Article, Chapter, Date, Info, Month, Project, ProjectEndDate, ProjectType, Showcase,
};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Level Editor Plugin",
    slug: "/level-editor-plugin",
    card_thumbnail: "level-editor-thumbnail.webp",
    main_contributions: ListParagraph {
        items: &[
            "Tool Programming", "Tool Design"
        ],
    },
    background: Some("level-editor-background.webp"),
    hero_video: Some("tEBmK5gDUyk"),
    badges: &[],
    info: Info {
        role: "Tool Programmer",
        genres: None,
        project_type: ProjectType::PluginsAndTooling,
        platforms: "PC",
        team_size: "Solo",
        duration: "7 weeks",
        end_date: ProjectEndDate::Ended(Date {
            day: 21,
            month: Month::October,
            year: 2022,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[],
    },
    description: TextParagraph {
        fragments: &[
            TextFragment::Text("An Unreal Engine "),
            TextFragment::StrongText("editor plugin"),
            TextFragment::Text(
            " I made in order to get more familiar with C++. The plugin allows you to draw on a \
            grid and define custom behaviours for this. It was designed with quick iteration \
            and flexibility in mind.",
            ),
        ],
    },
    chapters: &[Chapter {
        heading: "Features",
        articles: &[Article {
            title: "Custom Blueprint Behaviours",
            body: &[            Paragraph::Text(TextParagraph {
                fragments: &[
                    TextFragment::Text(
                    "I defined an interface in C++ that could be implemented through blueprints and \
                    allows you to define your own behaviours when dragging across the grid. By \
                    implementing this interface I created a blueprint that would automatically tile \
                    itself using a free tileset I found online.",
                    )
                ],
            }),
            ],
            showcase: Some(Showcase::Video("-9xbKIwheJ8")),
        }],
    }],
};
