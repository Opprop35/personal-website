use leptos::prelude::*;

use super::{Fragment, Info, ProjectPageData, ProjectPageView, Section, Showcase, Skill};

pub const PAGE: ProjectPageData = ProjectPageData {
    title: "Level Editor Plugin",
    background: Some("grid-lines.jpg"),
    hero_video: Some("tEBmK5gDUyk"),
    badges: &[],
    info: Info {
        role: "Tool Programmer",
        genres: "Tool",
        project_type: "University Project",
        platforms: "PC",
        team_size: "Solo",
        duration: "7 weeks",
        release_date: "21-Oct-2022",
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[],
    },
    description: &[
        Fragment::Text("An Unreal Engine "),
        Fragment::Emph("editor plugin"),
        Fragment::Text(
            " I made in order to get more familiar with C++. The plugin allows you to draw on a \
             grid and define custom behaviours for this. It was designed with quick iteration \
             and flexibility in mind.",
        ),
    ],
    sections: &[Section {
        heading: None,
        title: "Custom Blueprint Behaviours",
        body: &[Fragment::Text(
            "I defined an interface in C++ that could be implemented through blueprints and \
             allows you to define your own behaviours when dragging across the grid. By \
             implementing this interface I created a blueprint that would automatically tile \
             itself using a free tileset I found online.",
        )],
        showcase: Some(Showcase::Video("-9xbKIwheJ8")),
    }],
    under_construction: false,
};

#[component]
pub fn Page() -> impl IntoView {
    let _ = document().set_title("Jesse Stolk - Level Editor Plugin");

    view! { <ProjectPageView data=&PAGE /> }
}
