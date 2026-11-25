use crate::components::{ListParagraph, Paragraph, TextFragment, TextParagraph};
use crate::content::project::{
    Article, Chapter, Date, Genre, Info, Link, Month, Project, ProjectEndDate, ProjectType,
    Showcase,
};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Attic Panic",
    slug: "attic-panic",
    card_thumbnail: "attic-panic-thumbnail.webp",
    main_contributions: ListParagraph {
        items: &[
            "Upgrade System",
            "Upgrade Design & Implementation",
            "Upgrade & Gameplay Balancing",
            "Sound Effects & Audio",
        ],
    },
    background: Some("attic-panic-background.webp"),
    hero_video: Some("NjRhr-QN3SE"),
    badges: &[Link {
        image: "Steam-banner.svg",
        alt: "Available On Steam",
        url: "https://store.steampowered.com/app/2168760/Attic_Panic/",
    }],
    info: Info {
        role: "Technical Designer",
        genres: Some(&[Genre::Multiplayer, Genre::Arcade, Genre::Roguelike]),
        project_type: ProjectType::StudentGame,
        platforms: "PC",
        team_size: "~24",
        duration: "8 Months",
        end_date: ProjectEndDate::Ended(Date {
            day: 12,
            month: Month::June,
            year: 2023,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[
            Link {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/AtticPanicGame",
            },
            Link {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/atticpanicgame/",
            },
            Link {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@atticpanic",
            },
        ],
    },
    description: TextParagraph {
        fragments: &[
                TextFragment::Text("A year long student project. Attic Panic is an "),
                TextFragment::StrongText("online multiplayer"),
                TextFragment::Text(
                " top-down shooter with rogue-like elements. I mainly worked on the upgrade system, \
                individual upgrades, and replication of upgrades between server & clients.",
                ),
        ],
    },
    chapters: &[
        Chapter {
            heading: "Upgrades",
            articles: &[
                Article {
                    title: "Upgrade Data",
                    body: &[
                        Paragraph::Text(TextParagraph {
                            fragments: &[
                                TextFragment::Text(
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
                    showcase: Some(Showcase::Video("i1D-GloaSK4")),
                },
                Article {
                    title: "Upgrade Functionality & Replication",
                    body: &[
                        Paragraph::Text(TextParagraph {
                            fragments: &[
                                TextFragment::Text(
                                "I worked on a lot of upgrades for the game and made sure those upgrades \
                                would replicate properly between the different players.",
                                )
                            ],
                        }),
                    ],   showcase: Some(Showcase::Video("UryKkAPVYIQ")),
                },
                Article {
                    title: "Upgrade VFX Implementation",
                    body: &[
                        Paragraph::Text(TextParagraph {
                            fragments: &[
                                TextFragment::Text(
                                "I worked closely with one of the VFX artists to implement some of the VFX. \
                                Here you can see such an example we worked on the mine laying animation, \
                                explosion effect, and Disappear effect.",
                                )
                            ],
                        }),
                    ],   showcase: Some(Showcase::Video("59EkYuxYBVM")),
                },
            ],
        },
        Chapter {
            heading: "And more...",
            articles: &[],
        },
    ],
};
