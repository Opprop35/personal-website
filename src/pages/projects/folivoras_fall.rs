use leptos::prelude::*;

use crate::components::{ListParagraph, TextFragment, TextParagraph};
use crate::pages::projects::ProjectPage;
use crate::project::{Date, Genre, Info, Link, Month, Project, ProjectEndDate, ProjectType};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Folivora's Fall",
    href: "/folivoras-fall",
    card_thumbnail: "ff-thumbnail.webp",
    main_contributions: ListParagraph {
        items: &[
            "Camera System", "Puzzle Design", "Level Blockout",
        ],
    },
    background: Some("ff-background.webp"),
    hero_video: Some("0EdXbR93AfQ"),
    badges: &[Link {
        image: "available-on-itch-black.svg",
        alt: "Available On Itch",
        url: "https://buas.itch.io/folivorasfall",
    }],
    info: Info {
        role: "Level Designer",
        genres: Some(&[Genre::Platformer, Genre::Puzzle, Genre::Stealth]),
        project_type: ProjectType::StudentGame,
        platforms: "PC",
        team_size: "11",
        duration: "8 weeks",
        end_date: ProjectEndDate::Ended(Date {
            day: 7,
            month: Month::July,
            year: 2021,
        }),
        skills: &[Skill::Unreal, Skill::Perforce],
        socials: &[],
    },
    description: TextParagraph {
        fragments: &[
            TextFragment::Text(
            "The first big team project I worked on as a student. Folivora's Fall is a stealth \
            platformer set in a haunted mansion where you play as a sloth. During this project I \
            mostly focussed on level design and some implementation.",
            )
        ],
    },
    chapters: &[],
    under_construction: false,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Folivora's Fall");
    view! { <ProjectPage data=&PROJECT /> }
}
