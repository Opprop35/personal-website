use leptos::prelude::*;

use crate::components::Fragment;
use crate::organization::Organization;
use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Info, Month, Project, ProjectEndDate, ProjectType};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "LEGO® Framework",
    href: "/LegoFramework",
    card_thumbnail: "lego-thumbnail.webp",
    main_contributions: &[
        "Brick Snapping Logic & Tooling",
        "Connection Point Based Transform Gizmo",
        "Import Pipeline Redesign",
    ],
    background: Some("lego-background.webp"),
    hero_video: Some("7J7J0zm5xFc"),
    badges: &[],
    info: Info {
        role: "Tools Programmer",
        genres: None,
        project_type: ProjectType::PluginsAndTooling,
        platforms: "PC",
        team_size: "~5",
        duration: "2 Years",
        end_date: ProjectEndDate::Ended(Date {
            day: 1,
            month: Month::August,
            year: 2026,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Git, Skill::Perforce],
        socials: &[],
    },
    description: &[
        Fragment::Text("At "),
        Fragment::organization(Organization::TheLegoGroup),
        Fragment::Text(
            " I worked on a general purpose LEGO framework for Unreal Engine. \
             The LEGO® Brick Editor was a key deliverable. I mainly contributed to the Brick \
             building UX, Bug fixes, Automated Tests, and ",
        ),
        Fragment::StrongText("redesigned"),
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
    view! { <ProjectPageView data=&PROJECT /> }
}
