use crate::components::{ListParagraph, Paragraph, TextFragment, TextParagraph};
use crate::content::project::{
    Article, Chapter, Date, Genre, Info, Link, Month, Project, ProjectEndDate, ProjectType,
    Showcase,
};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Twin-Stick Duckbox",
    slug: "/twin-stick-duckbox",
    card_thumbnail: "tsj-thumbnail.webp",
    main_contributions: ListParagraph {
        items: &[
            "Enemy Spawn Event System",
            "Chomper Enemy",
            "Gauss Weapon",
            "Debug & Cheat Menu",
        ],
    },
    background: Some("tsj-background.webp"),
    hero_video: Some("OAiE_f08wU0"),
    badges: &[Link {
        image: "available-on-itch-black.svg",
        alt: "Available On Itch",
        url: "https://jcrombag.itch.io/twin-stick-duckbox",
    }],
    info: Info {
        role: "Technical Designer",
        genres: Some(&[Genre::Action, Genre::Arcade, Genre::TwinStickShooter]),
        project_type: ProjectType::StudentGame,
        platforms: "PC",
        team_size: "5",
        duration: "7 weeks",
        end_date: ProjectEndDate::Ended(Date {
            day: 3,
            month: Month::November,
            year: 2021,
        }),
        skills: &[Skill::Godot, Skill::Perforce],
        socials: &[],
    },
    description: TextParagraph {
        fragments: &[
            TextFragment::Text(
            "A simple project in which we were tasked to design, implement, and polish an additional \
            enemy and weapon. I also spent time developing a spawn event system and various other \
            tools for my team.",
            )
        ],
    },
    chapters: &[Chapter {
        heading: "Features",
        articles: &[
            Article {
                title: "Gauss Gun",
                body: &[
                    Paragraph::Text(TextParagraph {
                        fragments: &[
                        TextFragment::Text(
                        "The Gauss Gun shoots a big laser that deals continuous damage at a certain \
                        interval. It shoots a hidden projectile that uses the same base logic as other \
                        projectiles (so that it can be affected by the black hole and other base \
                        gameplay mechanics). Every damage interval the projectile reconstructs a \
                        collision polygon that deals damage to the enemies.",
                        )
                        ],
                    }),
                ],
                showcase: Some(Showcase::Video("VTDdvvMzu94")),
            },
            Article {
                title: "Chomper Enemy",
                body: &[
                    Paragraph::Text(TextParagraph {
                        fragments: &[
                        TextFragment::Text(
                        "The Chomper will eat your bullets if you simply attack it from the front. If it \
                        eats three it will regurgitate them into a powerful laser which uses the same \
                        logic as the Gauss Gun.",
                        )
                        ],
                    }),
                ],
                showcase: Some(Showcase::Video("dfV3BEkgjfw")),
            },
            Article {
                title: "Debug/Cheat Menu",
                body: &[
                    Paragraph::Text(TextParagraph {
                        fragments: &[
                        TextFragment::Text(
                        "A simple cheat menu that allows you to do various things like change your \
                        weapon, spawn enemies, and toggle godmode.",
                        )
                        ],
                    }),
                ],
                showcase: Some(Showcase::Video("andzCulJ4nI")),
            },
            Article {
                title: "Spawn Event System",
                body: &[
                    Paragraph::Text(TextParagraph {
                        fragments: &[
                        TextFragment::Text(
                        "I developed a system which could create more interesting spawning behaviour for \
                        enemies and control/limit the amount of enemies on screen. In the video below \
                        you can see it starts with spawning swarmers in random positions, then it \
                        spawns a lot of them from the corner, then a black hole, then a chomper etc. \
                        These behaviours were completely data driven and could be tweaked in various \
                        ways with preconditions, spawn interval, event weight, etc.",
                        )
                        ],
                    }),
                ],
                showcase: Some(Showcase::Video("OAiE_f08wU0")),
            },
        ],
    }],
    under_construction: false,
};
