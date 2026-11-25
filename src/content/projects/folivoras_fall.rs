use crate::models;

pub const PROJECT: models::Project = models::Project {
    title: "Folivora's Fall",
    slug: "folivoras-fall",
    card_thumbnail: "ff-thumbnail.webp",
    main_contributions: models::ListParagraph {
        items: &["Camera System", "Puzzle Design", "Level Blockout"],
    },
    background: Some("ff-background.webp"),
    hero_video: Some("0EdXbR93AfQ"),
    badges: &[models::ImageLink {
        image: "available-on-itch-black.svg",
        alt: "Available On Itch",
        url: "https://buas.itch.io/folivorasfall",
    }],
    info: models::Info {
        role: "Level Designer",
        genres: Some(&[models::Genre::Platformer, models::Genre::Puzzle, models::Genre::Stealth]),
        project_type: models::ProjectType::StudentGame,
        platforms: "PC",
        team_size: "11",
        duration: "8 weeks",
        end_date: models::ProjectEndDate::Ended(models::Date {
            day: 7,
            month: models::Month::July,
            year: 2021,
        }),
        skills: &[models::Skill::Unreal, models::Skill::Perforce],
        socials: &[],
    },
    description: models::TextParagraph {
        fragments: &[models::TextFragment::Text(
            "The first big team project I worked on as a student. Folivora's Fall is a stealth \
            platformer set in a haunted mansion where you play as a sloth. During this project I \
            mostly focussed on level design and some implementation.",
        )],
    },
    chapters: &[],
};
