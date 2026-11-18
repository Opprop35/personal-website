use leptos::prelude::*;

use crate::components::Fragment;
use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Info, Month, Project, ProjectEndDate, ProjectType, Section, Showcase};
use crate::skill::Skill;

pub const PROJECT: Project = Project {
    title: "Level Editor Plugin",
    href: "/LevelEditorPlugin",
    card_thumbnail: "level-editor-thumbnail.webp",
    main_contributions: &["Tool Programming", "Tool Design"],
    background: Some("level-editor-background.webp"),
    hero_video: Some("tEBmK5gDUyk"),
    badges: &[],
    info: Info {
        role: "Tool Programmer",
        genres: None,
        project_type: ProjectType::PluginsAndTooling,
        platforms: "PC",
        team_size: "Solo",
        duration: "7 weeks",
        end_date: ProjectEndDate::Ended(Date {
            day: 21,
            month: Month::October,
            year: 2022,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[],
    },
    description: &[
        Fragment::Text("An Unreal Engine "),
        Fragment::StrongText("editor plugin"),
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
    document().set_title("Jesse Stolk - Level Editor Plugin");
    view! { <ProjectPageView data=&PROJECT /> }
}
