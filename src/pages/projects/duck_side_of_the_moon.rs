use leptos::prelude::*;

use super::{Fragment, Info, Link, ProjectPageData, ProjectPageView, Section, Showcase, Skill};

pub const PAGE: ProjectPageData = ProjectPageData {
    title: "Duck Side of the Moon",
    background: Some("dsotm-cover.jpg"),
    hero_video: Some("V7ev8CYk1ts"),
    badges: &[Link {
        image: "Steam-banner.svg",
        alt: "Available On Steam",
        url: "https://store.steampowered.com/app/2959770/Duck_Side_of_the_Moon/",
    }],
    info: Info {
        role: "Tools & System Programmer",
        genres: "Cozy, Adventure, Exploration",
        project_type: "Indie Project",
        platforms: "PC",
        team_size: "~9",
        duration: "1 Year",
        release_date: "07-May-2026",
        skills: &[Skill::Unreal, Skill::Cpp, Skill::Perforce],
        socials: &[
            Link {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/ducksidegame",
            },
            Link {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/starbrewgames/",
            },
            Link {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@starbrewgames",
            },
        ],
    },
    description: &[
        Fragment::Text(
            "A remake of the original student project. This time it's a cozy adventure game. I \
             was the main ",
        ),
        Fragment::Emph("systems & tools programmer"),
        Fragment::Text(
            " for the project and built its core systems using a combination of C++ and \
             Blueprints. I was also part of establishing the indie company Starbrew Games.",
        ),
    ],
    sections: &[
        Section {
            heading: Some("Dialogue"),
            title: "Dialogue Trees",
            body: &[
                Fragment::Text(
                    "In order to build this game's dialogue system I utilized Unreal Engine's \
                     Instanced UObjects to build a tree of nested dialogue events. These trees \
                     are built up of:",
                ),
                Fragment::List(&[
                    "Flow Control (sequences, selectors, choices, etc.)",
                    "Conditions (HasItems, IsQuestCompleted, etc.)",
                    "Events (text, animation, tags/flags, etc.)",
                ]),
                Fragment::Text(
                    "Flow Control will trigger different child Events based on the Conditions \
                     those Events have. For example, the selector executes only the first event \
                     for which all conditions are true.",
                ),
            ],
            showcase: Some(Showcase::Image {
                src: "dsotm-dialogue-tree.png",
                alt: "Dialogue Tree",
            }),
        },
        Section {
            heading: None,
            title: "Blueprintable Dialogue Triggers",
            body: &[Fragment::Text(
                "To give Designers even more flexibility when working with the system I allowed \
                 them to create custom events as well. This could simply be done by inheriting \
                 from a TriggerDialogueEvent as you can see in the video.",
            )],
            showcase: Some(Showcase::Video("WeMdvLGzth8")),
        },
        Section {
            heading: None,
            title: "Details Panel Customization",
            body: &[Fragment::Text(
                "To make building dialogue trees more user friendly I did some research into \
                 Details Panel Customization. This allowed me to add color coding, create a more \
                 compact layout, and add a special wrap button. In the video to the right you \
                 can see how the tree becomes much more readable thanks to this feature.",
            )],
            showcase: Some(Showcase::Video("GuhgFl8Wt-A")),
        },
        Section {
            heading: None,
            title: "Typewriter Text Block",
            body: &[
                Fragment::Text(
                    "I also implemented a text block that had several custom features such as:",
                ),
                Fragment::List(&[
                    "Being able to print characters one-by-one while keeping consistent \
                     text-wrapping.",
                    "Allowing the creation of animated font materials by packing data in the \
                     vertex color of each character.",
                    "Easily changing text style, print interval, and animation duration when \
                     writing dialogue lines.",
                ]),
            ],
            showcase: Some(Showcase::Video("O8jYwwE2SFA")),
        },
        Section {
            heading: Some("Save Game"),
            title: "Save Game System",
            body: &[
                Fragment::Text(
                    "I was also responsible for building the game's save game system. \
                     Implementing features such as:",
                ),
                Fragment::List(&[
                    "Saving Placed & Spawned Actors",
                    "Saving Global Game State",
                    "Managing Save Game Slots",
                ]),
                Fragment::Text(
                    "I would've liked to build more robust validation as well but I left the \
                     team before I could take that task on.",
                ),
            ],
            showcase: Some(Showcase::Video("f0m2nBT5Vm4")),
        },
        Section {
            heading: Some("Inventory & Crafting System"),
            title: "Asset Definitions",
            body: &[Fragment::Text(
                "I used Unreal Engine's Asset Definitions to customize the look of certain types \
                 of assets in the content browser. For example every Item asset displays the \
                 same icon used in the in-game UI.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-item-assets.png",
                alt: "Item Assets",
            }),
        },
        Section {
            heading: None,
            title: "Recipe Data Assets",
            body: &[Fragment::Text(
                "Here you can see what a recipe data asset looks like. You can see the icons also \
                 show up in here allowing you to see at a glance what ingredients this recipe \
                 requires.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-recipe-asset.png",
                alt: "Recipe Data Asset",
            }),
        },
        Section {
            heading: Some("Quest System..."),
            title: "",
            body: &[],
            showcase: None,
        },
        Section {
            heading: Some("Upgrade System..."),
            title: "",
            body: &[],
            showcase: None,
        },
        Section {
            heading: Some("Technical Animation..."),
            title: "",
            body: &[],
            showcase: None,
        },
        Section {
            heading: Some("UI Programming..."),
            title: "",
            body: &[],
            showcase: None,
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
    let _ = document().set_title("Jesse Stolk - Duck Side Of The Moon");

    view! { <ProjectPageView data=&PAGE /> }
}
