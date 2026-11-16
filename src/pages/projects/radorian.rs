use leptos::prelude::*;

use super::{Fragment, Info, ProjectPageData, ProjectPageView, Skill};

pub const PAGE: ProjectPageData = ProjectPageData {
    title: "Radorian",
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
        release_date: "Not Released",
        skills: &[
            Skill {
                image: "Godot-logo.svg",
                alt: "Godot Logo",
            },
            Skill {
                image: "c-sharp.svg",
                alt: "C Sharp Logo",
            },
            Skill {
                image: "aseprite.svg",
                alt: "Aseprite Logo",
            },
            Skill {
                image: "blender.png",
                alt: "Blender Logo",
            },
            Skill {
                image: "Git-logo.svg",
                alt: "Git Logo",
            },
        ],
        socials: &[],
    },
    description: &[
        Fragment::Text("A "),
        Fragment::Emph("personal project"),
        Fragment::Text(
            " I work on intermittently in my spare time. It's a third person game with an \
             experimental art style. I'm making everything myself from code to art to sound to \
             game design.",
        ),
    ],
    sections: &[],
    under_construction: true,
};

#[component]
pub fn Page() -> impl IntoView {
    let _ = document().set_title("Jesse Stolk - Radorian");

    view! { <ProjectPageView data=&PAGE /> }
}
