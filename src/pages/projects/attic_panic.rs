use leptos::prelude::*;

use crate::components::Fragment;
use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Info, Link, Month, Project, Section, Showcase};
use crate::skill::Skill;

pub const PAGE: Project = Project {
    title: "Attic Panic",
    href: "/AtticPanic",
    card_thumbnail: "attic-panic-thumbnail.webp",
    main_contributions: &[
        "Upgrade System",
        "Upgrade Design & Implementation",
        "Upgrade & Gameplay Balancing",
        "Sound Effects & Audio",
    ],
    background: Some("attic-panic-background.webp"),
    hero_video: Some("NjRhr-QN3SE"),
    badges: &[Link {
        image: "Steam-banner.svg",
        alt: "Available On Steam",
        url: "https://store.steampowered.com/app/2168760/Attic_Panic/",
    }],
    info: Info {
        role: "Technical Designer",
        genres: "Multiplayer, Arcade, Roguelike",
        project_type: "University Project",
        platforms: "PC",
        team_size: "~24",
        duration: "8 Months",
        release_date: Some(Date {
            day: 12,
            month: Month::June,
            year: 2023,
        }),
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[
            Link {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/AtticPanicGame",
            },
            Link {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/atticpanicgame/",
            },
            Link {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@atticpanic",
            },
        ],
    },
    description: &[
        Fragment::Text("A year long student project. Attic Panic is an "),
        Fragment::StrongText("online multiplayer"),
        Fragment::Text(
            " top-down shooter with rogue-like elements. I mainly worked on the upgrade system, \
             individual upgrades, and replication of upgrades between server & clients.",
        ),
    ],
    sections: &[
        Section {
            heading: Some("Upgrades"),
            title: "Upgrade Data",
            body: &[Fragment::Text(
                "While working on the game I redesigned & refactored the upgrade system in order \
                 to create a smoother data-driven experience for designers. This way the upgrades \
                 could be balanced & tweaked more easily throughout development. I defined Data \
                 Assets in C++ which can be created in the content browser of Unreal and were \
                 automatically detected and used by the game's systems. I also created a short \
                 video for the team explaining how to create these.",
            )],
            showcase: Some(Showcase::Video("i1D-GloaSK4")),
        },
        Section {
            heading: None,
            title: "Upgrade Functionality & Replication",
            body: &[Fragment::Text(
                "I worked on a lot of upgrades for the game and made sure those upgrades \
                     would replicate properly between the different players.",
            )],
            showcase: Some(Showcase::Video("UryKkAPVYIQ")),
        },
        Section {
            heading: None,
            title: "Upgrade VFX Implementation",
            body: &[Fragment::Text(
                "I worked closely with one of the VFX artists to implement some of the VFX. Here \
                 you can see such an example we worked on the mine laying animation, explosion \
                 effect, and Disappear effect.",
            )],
            showcase: Some(Showcase::Video("59EkYuxYBVM")),
        },
        Section {
            heading: Some("And more..."),
            title: "",
            body: &[],
            showcase: None,
        },
    ],
    under_construction: false,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Attic Panic");
    view! { <ProjectPageView data=&PAGE /> }
}
