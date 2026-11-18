use leptos::prelude::*;

use crate::components::Fragment;
use crate::pages::projects::ProjectPageView;
use crate::project::{Info, Project};
use crate::skill::Skill;

pub const PAGE: Project = Project {
    title: "Radorian",
    href: "/Radorian",
    card_thumbnail: "radorian-thumbnail.webp",
    main_contributions: &[
        "Third person camera",
        "3D sprite pipeline & tools",
        "Custom shaders",
        "Procedural Geometry",
        "Dynamic gravity",
    ],
    background: None,
    hero_video: Some("tbIsz63srjo"),
    badges: &[],
    info: Info {
        role: "Sole Developer",
        genres: "Atmospheric, Crafting, Third Person",
        project_type: "Personal Project",
        platforms: "PC",
        team_size: "Solo",
        duration: "Too Long",
        release_date: None,
        skills: &[
            Skill::Godot,
            Skill::CSharp,
            Skill::Aseprite,
            Skill::Blender,
            Skill::Git,
        ],
        socials: &[],
    },
    description: &[
        Fragment::Text("A "),
        Fragment::StrongText("personal project"),
        Fragment::Text(
            " I used to work on in my spare time at university. It's a third person game with an \
             experimental art style. I made everything myself from code to art to sound to \
             game design.",
        ),
    ],
    sections: &[],
    under_construction: true,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Radorian");
    view! { <ProjectPageView data=&PAGE /> }
}
