use leptos::prelude::*;

use super::{Fragment, Info, Link, ProjectPageData, ProjectPageView, Skill};

pub const PAGE: ProjectPageData = ProjectPageData {
    title: "Folivora's Fall",
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
        release_date: "07-July-2021",
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
    let _ = document().set_title("Jesse Stolk - Folivora's Fall");

    view! { <ProjectPageView data=&PAGE /> }
}
