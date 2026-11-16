use leptos::prelude::*;

use super::{Fragment, Info, ProjectPageData, ProjectPageView, Skill};

pub const PAGE: ProjectPageData = ProjectPageData {
    title: "LEGO® Brick Editor",
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
        release_date: "17-Jun-2025",
        skills: &[
            Skill {
                image: "ue-logo.svg",
                alt: "Unreal Engine Logo",
            },
            Skill {
                image: "cpp-logo.svg",
                alt: "C++ Logo",
            },
            Skill {
                image: "Git-logo.svg",
                alt: "Git Logo",
            },
            Skill {
                image: "perforce.png",
                alt: "Perforce Logo",
            },
        ],
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
    let _ = document().set_title("Jesse Stolk - LegoFramework");

    view! { <ProjectPageView data=&PAGE /> }
}
