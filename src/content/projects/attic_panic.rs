use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Attic Panic",
    slug: "attic-panic",
    card_thumbnail: "attic-panic-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &[
            "Upgrade System",
            "Upgrade Design & Implementation",
            "Upgrade & Gameplay Balancing",
            "Sound Effects & Audio",
        ],
    },
    background: Some("attic-panic-background.webp"),
    hero_video: Some("NjRhr-QN3SE"),
    badges: &[models::ImageLink {
        image: "steam-banner.svg",
        alt: "Available On Steam",
        url: "https://store.steampowered.com/app/2168760/Attic_Panic/",
    }],
    info: models::Info {
        role: "Technical Designer",
        genres: Some(&[models::Genre::Multiplayer, models::Genre::Arcade, models::Genre::Roguelike]),
        project_type: models::ProjectType::StudentGame,
        platforms: "PC",
        team_size: "~24",
        duration: "8 Months",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 12,
            month: models::Month::June,
            year: 2023,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Cpp, models::Skill::Perforce],
        socials: &[
            models::ImageLink {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/AtticPanicGame",
            },
            models::ImageLink {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/atticpanicgame/",
            },
            models::ImageLink {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@atticpanic",
            },
        ],
    },
    description: models::TextParagraph {
        fragments: &[
                models::TextFragment::Text("A year long student project. Attic Panic is an "),
                models::TextFragment::StrongText("online multiplayer"),
                models::TextFragment::Text(
                " top-down shooter with rogue-like elements. I mainly worked on the upgrade system, \
                individual upgrades, and replication of upgrades between server & clients.",
                ),
        ],
    },
    chapters: &[
        models::Chapter {
            heading: "Upgrades",
            articles: &[
                models::Article {
                    title: "Upgrade Data",
                    body: &[
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[
                                models::TextFragment::Text(
                                "While working on the game I redesigned & refactored the upgrade system \
                                in order to create a smoother data-driven experience for designers. \
                                This way the upgrades could be balanced & tweaked more easily \
                                throughout development. I defined Data Assets in C++ which can be \
                                created in the content browser of Unreal and were automatically \
                                detected and used by the game's systems. I also created a short video \
                                for the team explaining how to create these.",
                                ),
                            ],
                        }),
                    ],
                    showcase: models::Showcase::Video("i1D-GloaSK4"),
                },
                models::Article {
                    title: "Upgrade Functionality & Replication",
                    body: &[
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[
                                models::TextFragment::Text(
                                "I worked on a lot of upgrades for the game and made sure those upgrades \
                                would replicate properly between the different players.",
                                )
                            ],
                        }),
                    ],   showcase: models::Showcase::Video("UryKkAPVYIQ"),
                },
                models::Article {
                    title: "Upgrade VFX Implementation",
                    body: &[
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[
                                models::TextFragment::Text(
                                "I worked closely with one of the VFX artists to implement some of the VFX. \
                                Here you can see such an example we worked on the mine laying animation, \
                                explosion effect, and Disappear effect.",
                                )
                            ],
                        }),
                    ],   showcase: models::Showcase::Video("59EkYuxYBVM"),
                },
            ],
        },
        models::Chapter {
            heading: "And more...",
            articles: &[],
        },
    ],
};
