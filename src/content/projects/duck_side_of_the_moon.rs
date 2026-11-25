use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Duck Side of the Moon",
    slug: "duck-side-of-the-moon",
    card_thumbnail: "dsotm-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &[
            "Dialogue System",
            "Save Game System",
            "Quest System",
            "Inventory & Crafting System",
            "Upgrade System",
        ],
    },
    background: Some("dsotm-background.webp"),
    hero_video: Some("jDcsH4LYuCo"),
    badges: &[models::ImageLink {
        image: "steam-banner.svg",
        alt: "Available On Steam",
        url: "https://store.steampowered.com/app/2959770/Duck_Side_of_the_Moon/",
    }],
    info: models::Info {
        role: "Tools & System Programmer",
        genres: Some(&[models::Genre::Cozy, models::Genre::Adventure, models::Genre::Exploration]),
        project_type: models::ProjectType::IndieGame,
        platforms: "PC",
        team_size: "~9",
        duration: "1 Year",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 1,
            month: models::Month::July,
            year: 2024,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Cpp, models::Skill::Perforce],
        socials: &[
            models::ImageLink {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/ducksidegame",
            },
            models::ImageLink {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/starbrewgames/",
            },
            models::ImageLink {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@starbrewgames",
            },
        ],
    },
    description: models::TextParagraph {
        fragments: &[
            models::TextFragment::Text(
                "A remake of the original student project. This time it's a cozy adventure game. I \
            was the main ",
            ),
            models::TextFragment::StrongText("systems & tools programmer"),
            models::TextFragment::Text(
                " for the project and built its core systems using a combination of C++ and \
            Blueprints. I also helped establish the indie company ",
            ),
            models::TextFragment::Link {
                url: "https://starbrewgames.com/",
                label: "Starbrew Games",
            },
            models::TextFragment::Text(" but left the project and company midway through development."),
        ],
    },
    chapters: &[
        models::Chapter {
            heading: "Dialogue",
            articles: &[
                models::Article {
                    title: "Dialogue Trees",
                    body: &[
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[models::TextFragment::Text(
                                "In order to build this game's dialogue system I utilized Unreal Engine's \
                                Instanced UObjects to build a tree of nested dialogue events. These trees \
                                are built up of:",
                            )],
                        }),
                        models::Paragraph::List(models::ListParagraph {
                            items: &[
                                "Flow Control (sequences, selectors, choices, etc.)",
                                "Conditions (HasItems, IsQuestCompleted, etc.)",
                                "Events (text, animation, tags/flags, etc.)",
                            ],
                        }),
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[models::TextFragment::Text(
                                "Flow Control will trigger different child Events based on the Conditions \
                                those Events have. For example, the selector executes only the first event \
                                for which all conditions are true.",
                            )],
                        }),
                    ],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-dialogue-tree.webp",
                        alt: "Dialogue Tree",
                    },
                },
                models::Article {
                    title: "Blueprintable Dialogue Triggers",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "To give Designers even more flexibility when working with the system I allowed \
                            them to create custom events as well. This could simply be done by inheriting \
                            from a TriggerDialogueEvent as you can see in the video.",
                        )],
                    })],
                    showcase: models::Showcase::Video("WeMdvLGzth8"),
                },
                models::Article {
                    title: "Details Panel Customization",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "To make building dialogue trees more user friendly I did some research into \
                            Details Panel Customization. This allowed me to add color coding, create a more \
                            compact layout, and add a special wrap button. In the video to the right you \
                            can see how the tree becomes much more readable thanks to this feature.",
                        )],
                    })],
                    showcase: models::Showcase::Video("GuhgFl8Wt-A"),
                },
                models::Article {
                    title: "Typewriter Text Block",
                    body: &[
                        models::Paragraph::Text(models::TextParagraph {
                            fragments: &[models::TextFragment::Text(
                                "I also implemented a text block that had several custom features such as:",
                            )],
                        }),
                        models::Paragraph::List(models::ListParagraph {
                            items: &[
                                "Being able to print characters one-by-one while keeping consistent \
                                text-wrapping.",
                                "Allowing the creation of animated font materials by packing data in the \
                                vertex color of each character.",
                                "Easily changing text style, print interval, and animation duration when \
                                writing dialogue lines.",
                            ],
                        }),
                    ],
                    showcase: models::Showcase::Video("O8jYwwE2SFA"),
                },
            ],
        },
        models::Chapter {
            heading: "Save Game",
            articles: &[models::Article {
                title: "Save Game System",
                body: &[
                    models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I was also responsible for building the game's save game system. \
                            Implementing features such as:",
                        )],
                    }),
                    models::Paragraph::List(models::ListParagraph {
                        items: &[
                            "Saving Placed & Spawned Actors",
                            "Saving Global Game State",
                            "Managing Save Game Slots",
                        ],
                    }),
                    models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I would've liked to build more robust validation as well but I left the \
                            team before I could take that task on.",
                        )],
                    }),
                ],
                showcase: models::Showcase::Video("f0m2nBT5Vm4"),
            }],
        },
        models::Chapter {
            heading: "Inventory & Crafting",
            articles: &[
                models::Article {
                    title: "Asset Definitions",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I used Unreal Engine's Asset Definitions to customize the look of certain types \
                            of assets in the content browser. For example every Item asset displays the \
                            same icon used in the in-game UI.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-item-assets.webp",
                        alt: "Item Assets",
                    },
                },
                models::Article {
                    title: "Recipe Data Assets",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Here you can see what a recipe data asset looks like. You can see the icons also \
                            show up in here allowing you to see at a glance what ingredients this recipe \
                            requires.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-recipe-asset.webp",
                        alt: "Recipe Data Asset",
                    },
                },
            ],
        },
        models::Chapter {
            heading: "Quest System...",
            articles: &[],
        },
        models::Chapter {
            heading: "Upgrade System...",
            articles: &[],
        },
        models::Chapter {
            heading: "Technical Animation...",
            articles: &[],
        },
        models::Chapter {
            heading: "UI Programming...",
            articles: &[],
        },
        models::Chapter {
            heading: "And more...",
            articles: &[],
        },
    ],
};
