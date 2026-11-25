use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Twin-Stick Duckbox",
    slug: "twin-stick-duckbox",
    card_thumbnail: "tsj-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &[
            "Enemy Spawn Event System",
            "Chomper Enemy",
            "Gauss Weapon",
            "Debug & Cheat Menu",
        ],
    },
    background: Some("tsj-background.webp"),
    hero_video: Some("OAiE_f08wU0"),
    badges: &[models::ImageLink {
        image: "available-on-itch-black.svg",
        alt: "Available On Itch",
        url: "https://jcrombag.itch.io/twin-stick-duckbox",
    }],
    info: models::Info {
        role: "Technical Designer",
        genres: Some(&[models::Genre::Action, models::Genre::Arcade, models::Genre::TwinStickShooter]),
        project_type: models::ProjectType::StudentGame,
        platforms: "PC",
        team_size: "5",
        duration: "7 weeks",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 3,
            month: models::Month::November,
            year: 2021,
        }),
        skills: &[models::Skill::Godot, models::Skill::Perforce],
        socials: &[],
    },
    description: models::TextParagraph {
        fragments: &[models::TextFragment::Text(
            "A simple project in which we were tasked to design, implement, and polish an additional \
            enemy and weapon. I also spent time developing a spawn event system and various other \
            tools for my team.",
        )],
    },
    chapters: &[models::Chapter {
        heading: "Features",
        articles: &[
            models::Article {
                title: "Gauss Gun",
                body: &[models::Paragraph::Text(models::TextParagraph {
                    fragments: &[models::TextFragment::Text(
                        "The Gauss Gun shoots a big laser that deals continuous damage at a certain \
                        interval. It shoots a hidden projectile that uses the same base logic as other \
                        projectiles (so that it can be affected by the black hole and other base \
                        gameplay mechanics). Every damage interval the projectile reconstructs a \
                        collision polygon that deals damage to the enemies.",
                    )],
                })],
                showcase: models::Showcase::Video("VTDdvvMzu94"),
            },
            models::Article {
                title: "Chomper Enemy",
                body: &[models::Paragraph::Text(models::TextParagraph {
                    fragments: &[models::TextFragment::Text(
                        "The Chomper will eat your bullets if you simply attack it from the front. If it \
                        eats three it will regurgitate them into a powerful laser which uses the same \
                        logic as the Gauss Gun.",
                    )],
                })],
                showcase: models::Showcase::Video("dfV3BEkgjfw"),
            },
            models::Article {
                title: "Debug/Cheat Menu",
                body: &[models::Paragraph::Text(models::TextParagraph {
                    fragments: &[models::TextFragment::Text(
                        "A simple cheat menu that allows you to do various things like change your \
                        weapon, spawn enemies, and toggle godmode.",
                    )],
                })],
                showcase: models::Showcase::Video("andzCulJ4nI"),
            },
            models::Article {
                title: "Spawn Event System",
                body: &[models::Paragraph::Text(models::TextParagraph {
                    fragments: &[models::TextFragment::Text(
                        "I developed a system which could create more interesting spawning behaviour for \
                        enemies and control/limit the amount of enemies on screen. In the video below \
                        you can see it starts with spawning swarmers in random positions, then it \
                        spawns a lot of them from the corner, then a black hole, then a chomper etc. \
                        These behaviours were completely data driven and could be tweaked in various \
                        ways with preconditions, spawn interval, event weight, etc.",
                    )],
                })],
                showcase: models::Showcase::Video("OAiE_f08wU0"),
            },
        ],
    }],
};
