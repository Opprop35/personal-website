use crate::models;

pub const PROJECTS: [models::ExperienceData; 3] = [
    models::ExperienceData {
        source: models::ExperienceSource::Organization(models::Organization::TheLegoGroup),
        subtitle: "LEGO® Brick Editor | Tools Programmer",
        period: "November 2024 – July 2026",
        details: Some(&[models::Paragraph::List(models::ListParagraph {
            items: &[
                "Brick Snapping Logic & Tooling",
                "Import Pipeline Redesign",
                "Connection Point Based Gizmo",
            ],
        })]),
    },
    models::ExperienceData {
        source: models::ExperienceSource::Organization(models::Organization::StarbrewGames),
        subtitle: "Duckside of the Moon | Game Developer",
        period: "June 2023 – June 2024",
        details: Some(&[models::Paragraph::List(models::ListParagraph {
            items: &[
                "Dialogue, Quest, Crafting, & Upgrade Systems",
                "Save Game System",
                "Technical Animation",
                "UI Programming",
            ],
        })]),
    },
    models::ExperienceData {
        source: models::ExperienceSource::Personal,
        subtitle: "Various projects | Game Developer",
        period: "April 2021 – Present",
        details: Some(&[models::Paragraph::List(models::ListParagraph {
            items: &[
                "Asset Pipelines & Tools",
                "Procedural Geometry",
                "Online Multiplayer",
                "Shader Programming",
                "Third Person Camera",
            ],
        })]),
    },
];

pub const SOCRATES_NOTE: [models::Paragraph; 1] = [models::Paragraph::Text(models::TextParagraph {
    fragments: &[
        models::TextFragment::Link {
            url: "https://www.socrateshonours.org/lid-worden/toelatingscriteria",
            label: "Socrates",
        },
        models::TextFragment::MutedText(" (Top 10% of students)"),
    ],
})];

pub const EDUCATION: [models::ExperienceData; 3] = [
    models::ExperienceData {
        source: models::ExperienceSource::Organization(models::Organization::BredaUniversityOfAppliedSciences),
        subtitle: "Creative Media and Game Technologies",
        period: "Sep 2020 – Jul 2024",
        details: None,
    },
    models::ExperienceData {
        source: models::ExperienceSource::Organization(models::Organization::TuDelft),
        subtitle: "Computer Science",
        period: "Sep 2018 – Nov 2019",
        details: None,
    },
    models::ExperienceData {
        source: models::ExperienceSource::Organization(models::Organization::IswGasthuislaan),
        subtitle: "Pre-University Education (VWO)",
        period: "Sep 2012 – Jun 2018",
        details: Some(&SOCRATES_NOTE),
    },
];
