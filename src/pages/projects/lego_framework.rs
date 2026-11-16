use leptos::prelude::*;

use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Fragment, Info, Month, Project};
use crate::skill::Skill;

pub const PAGE: Project = Project {
    title: "LEGO® Brick Editor",
    href: "/LegoFramework",
    image: "lego-brick-editor.png",
    contributions: &[
        "Brick Snapping Logic & Tooling",
        "Connection Point Based Transform Gizmo",
        "Import Pipeline Redesign",
    ],
    background: Some("lego-fortnite.jpeg"),
    hero_video: Some("7J7J0zm5xFc"),
    badges: &[],
    info: Info {
        role: "Tools Programmer",
        genres: "None",
        project_type: "Plugins & Tooling",
        platforms: "PC",
        team_size: "~5",
        duration: "2 Years",
        release_date: Some(Date {
            day: 17,
            month: Month::June,
            year: 2025,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Git, Skill::Perforce],
        socials: &[],
    },
    description: &[
        Fragment::Text(
            "At The LEGO Group I worked on a general purpose LEGO framework for Unreal Engine. \
             The LEGO® Brick Editor was a key deliverable. I mainly contributed to the Brick \
             building UX, Bug fixes, Automated Tests, and ",
        ),
        Fragment::Emph("redesigned"),
        Fragment::Text(
            " the framework's import pipeline to be more idiomatic, performant and flexible.",
        ),
    ],
    sections: &[],
    under_construction: true,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - LegoFramework");
    view! { <ProjectPageView data=&PAGE /> }
}
