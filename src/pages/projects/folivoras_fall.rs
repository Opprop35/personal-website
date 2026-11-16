use leptos::prelude::*;

use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Fragment, Info, Link, Month, Project};
use crate::skill::Skill;

pub const PAGE: Project = Project {
    title: "Folivora's Fall",
    href: "/FolivorasFall",
    image: "ff-title.jpg",
    contributions: &["Camera System", "Puzzle Design", "Level Blockout"],
    background: Some("ff-background.jpg"),
    hero_video: Some("0EdXbR93AfQ"),
    badges: &[Link {
        image: "available-on-itch-black.svg",
        alt: "Available On Itch",
        url: "https://buas.itch.io/folivorasfall",
    }],
    info: Info {
        role: "Level Designer",
        genres: "Platformer, Puzzle, Stealth",
        project_type: "University Project",
        platforms: "PC",
        team_size: "11",
        duration: "8 weeks",
        release_date: Some(Date {
            day: 7,
            month: Month::July,
            year: 2021,
        }),
        skills: &[Skill::Unreal, Skill::Perforce],
        socials: &[],
    },
    description: &[Fragment::Text(
        "The first big team project I worked on as a student. Folivora's Fall is a stealth \
         platformer set in a haunted mansion where you play as a sloth. During this project I \
         mostly focussed on level design and some implementation.",
    )],
    sections: &[],
    under_construction: false,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Folivora's Fall");
    view! { <ProjectPageView data=&PAGE /> }
}
