use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Duckside of the Moon (Student)",
    slug: "duckside-of-the-moon-student",
    card_thumbnail: "dsotm-student-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &[
            "Inventory & Crafting System",
            "Mining & Item Pickups",
            "Progression Design",
            "Upgrade & Recipe Balancing",
            "Sound Effects",
        ],
    },
    background: Some("dsotm-student-background.webp"),
    hero_video: Some("FSj0G0oDqdA"),
    badges: &[
        models::ImageLink {
            image: "bafta-shortlist-white.webp",
            alt: "Nominated BAFTA Student Awards",
            url: "https://www.bafta.org/programmes/bafta-student-awards/",
        },
        models::ImageLink {
            image: "dga-nominated-student-badge.webp",
            alt: "Nominated Student Game Dutch Game Awards",
            url: "https://www.dutchgamesindustry.nl/game/duckside-of-the-moon",
        },
        models::ImageLink {
            image: "available-on-itch-black.svg",
            alt: "Available On Itch",
            url: "https://buas.itch.io/ducksidegame",
        },
    ],
    info: models::Info {
        role: "Technical Designer",
        genres: Some(&[models::Genre::Survival, models::Genre::Adventure, models::Genre::Crafting]),
        project_type: models::ProjectType::StudentGame,
        platforms: "PC",
        team_size: "23",
        duration: "4-5 Months",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 28,
            month: models::Month::June,
            year: 2022,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Perforce],
        socials: &[
            models::ImageLink {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/ducksidegame",
            },
            models::ImageLink {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/ducksidegame/",
            },
            models::ImageLink {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@ducksidegame",
            },
        ],
    },
    description: models::TextParagraph {
        fragments: &[
            models::TextFragment::Text("The original student version of Duckside of the Moon. It's a "),
            models::TextFragment::StrongText("survival game"),
            models::TextFragment::Text(
                " set in space where you play as a duck astronaut. I built the crafting system & core \
            gameplay loop for this game and balanced them.",
            ),
        ],
    },
    chapters: &[
        models::Chapter {
            heading: "Inventory & Crafting System",
            articles: &[
                models::Article {
                    title: "Research",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Before creating our game's inventory system I researched other games to figure \
                            out what kind of UX our inventory system should have. My main reference turned \
                            out to be Subnautica since its inventory system is simple and easy to use which \
                            works great for an exploration focused game like ours.",
                        )],
                    })],
                    showcase: models::Showcase::Video("Il9eWYm6Gnk"),
                },
                models::Article {
                    title: "Defining items & recipes",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I looked into several different ways of defining the data for our game's items \
                            and recipes. I decided to use Unreal's Data Tables which would give us a \
                            flexible system for defining new items and recipes.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-items-datatable.webp",
                        alt: "Datatable Image",
                    },
                },
                models::Article {
                    title: "Inventory Backend",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "When implementing the inventory system I made sure it was easy to communicate \
                            with and expand upon. Creating reusable components and using event dispatchers \
                            (observer pattern) to communicate state updates to the UI.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-itemcontainer.webp",
                        alt: "Item Container Blueprint",
                    },
                },
                models::Article {
                    title: "Crafting Backend",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "For the crafting system I used a similar approach. When a recipe is crafted an \
                            event will be emitted. The player or any other actor can subscribe to this event \
                            and trigger any necessary upgrade logic when it is emitted.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-upgraderecipe.webp",
                        alt: "Upgrade Recipe Blueprint",
                    },
                },
                models::Article {
                    title: "Test Level",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I also maintained a test level for the team in order to test inventory, \
                            crafting, and mining related functionality. This was especially helpful to our \
                            UI Designers when testing new features.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-test-level.webp",
                        alt: "Test Level",
                    },
                },
            ],
        },
        models::Chapter {
            heading: "Items & Mining",
            articles: &[
                models::Article {
                    title: "Item Pickups",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Since a core part of our game was about item gathering this process needed to \
                            feel fun and juicy. Item pickups have a physical presence in the world and follow \
                            a curve-based pickup animation when interacted with. Items also react to the \
                            game's gravity zones falling or floating depending on where they are in the \
                            world.",
                        )],
                    })],
                    showcase: models::Showcase::Video("6G7Ye7oFo5g"),
                },
                models::Article {
                    title: "Mining",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "In order to make the mining look and feel right I collaborated with a VFX artist \
                            & Animator. We worked together to make sure the character, VFX, and gameplay \
                            logic were in sync.",
                        )],
                    })],
                    showcase: models::Showcase::Video("7PckBhyEjBQ"),
                },
            ],
        },
        models::Chapter {
            heading: "Progression Design",
            articles: &[
                models::Article {
                    title: "Planning Progression",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[
                            models::TextFragment::Text(
                                "Another responsibility I took on was designing the flow of progression in \
                            our game. I was inspired by ",
                            ),
                            models::TextFragment::Link {
                                url: "https://www.patreon.com/posts/how-my-boss-key-13801754",
                                label: "GMTK's Dungeon Graphs",
                            },
                            models::TextFragment::Text(
                                " and used this method to sketch out a basic progression flow to see how \
                            players might progress within our game. I also worked with other designers \
                            and artists to refine and improve on these ideas.",
                            ),
                        ],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-progression-graph.webp",
                        alt: "Progression Graph",
                    },
                },
                models::Article {
                    title: "Designing the resources",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Together with our concept artist I brainstormed and defined the themes of our \
                            resources (Their names and visuals) based on where you obtain them and what \
                            you'll use them for. This process really helped set the scope of our game and \
                            gave the artists and designers a concrete goal to work towards.",
                        )],
                    })],
                    showcase: models::Showcase::Image {
                        file_name: "dsotm-resource-brainstorming.webp",
                        alt: "Brainstorming Image",
                    },
                },
            ],
        },
        models::Chapter {
            heading: "Balancing & Tweaks",
            articles: &[
                models::Article {
                    title: "Recipes",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Guided by feedback from playtesters I worked towards balancing the game's \
                            recipes. I tackled major hickups in the progression first for example, upgrades \
                            that required more than 2 trips out of the spaceship and back were reduced in \
                            cost in order to make the game not feel too grindy. In the final version \
                            progression felt good to most people, and many simply wanted more content.",
                        )],
                    })],
                    showcase: models::Showcase::Video("2hP9OCdsHEo"),
                },
                models::Article {
                    title: "Gameplay Balancing",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "Additionally I worked on balancing character variables & upgrades. During \
                            playtesting it became clear that players did not have enough time to explore \
                            before their oxygen and food started running out. I iterated on things like food \
                            capacity, oxygen upgrades, and movement speed in order to smoothen out the \
                            experience.",
                        )],
                    })],
                    showcase: models::Showcase::Video("5Apuv2_Lkk0"),
                },
            ],
        },
        models::Chapter {
            heading: "Other contributions",
            articles: &[
                models::Article {
                    title: "Character Movement",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "During the project I also spent time assisting with character movement. For \
                            example making sure the player's velocity is accumulated properly between \
                            movement, boosting, and geysers. I also improved the visibility of the character \
                            model by rotating the character separately from the camera allowing you to view \
                            the character from any angle.",
                        )],
                    })],
                    showcase: models::Showcase::Video("qOdHwQqDw7Y"),
                },
                models::Article {
                    title: "Asteroid Tool",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "I also assisted programmers in tweaking our asteroid tool. In order to make the \
                            resource outcrops fit more naturally on the asteroids, I adjusted their position \
                            based on the asteroid's normal vectors and direction away from its epicenter.",
                        )],
                    })],
                    showcase: models::Showcase::Video("ibE9HOcuVrY"),
                },
                models::Article {
                    title: "Sound Effects",
                    body: &[models::Paragraph::Text(models::TextParagraph {
                        fragments: &[models::TextFragment::Text(
                            "As we didn't have a dedicated sound designer on the team I picked up many of \
                            the sound design tasks. A small selection of these are: Walking, Boosting, \
                            Shooting, Crafting, Open/Close Menu, and Eating.",
                        )],
                    })],
                    showcase: models::Showcase::Video("93oUEzG0bKg"),
                },
            ],
        },
    ],
};
