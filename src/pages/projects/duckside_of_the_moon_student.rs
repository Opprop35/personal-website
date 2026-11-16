use leptos::prelude::*;

use crate::pages::projects::ProjectPageView;
use crate::project::{Date, Fragment, Info, Link, Month, Project, Section, Showcase};
use crate::skill::Skill;

pub const PAGE: Project = Project {
    title: "Duckside of the Moon (Student)",
    href: "/DucksideOfTheMoonStudent",
    image: "dsotm-ingame-ice.jpg",
    contributions: &[
        "Inventory & Crafting System",
        "Mining & Item Pickups",
        "Progression Design",
        "Upgrade & Recipe Balancing",
        "Sound Effects",
    ],
    background: Some("dsotm-ship.jpg"),
    hero_video: Some("FSj0G0oDqdA"),
    badges: &[
        Link {
            image: "bafta-shortlist-white.png",
            alt: "Nominated BAFTA Student Awards",
            url: "https://www.bafta.org/programmes/bafta-student-awards/",
        },
        Link {
            image: "dga-nominated-student-badge.png",
            alt: "Nominated Student Game Dutch Game Awards",
            url: "https://www.dutchgamesindustry.nl/game/duckside-of-the-moon",
        },
        Link {
            image: "available-on-itch-black.svg",
            alt: "Available On Itch",
            url: "https://buas.itch.io/ducksidegame",
        },
    ],
    info: Info {
        role: "Technical Designer",
        genres: "Survival, Adventure, Crafting",
        project_type: "University Project",
        platforms: "PC",
        team_size: "23",
        duration: "4-5 Months",
        release_date: Some(Date {
            day: 28,
            month: Month::June,
            year: 2022,
        }),
        skills: &[Skill::Unreal, Skill::Perforce],
        socials: &[
            Link {
                image: "twitter.svg",
                alt: "Twitter",
                url: "https://www.twitter.com/ducksidegame",
            },
            Link {
                image: "instagram.svg",
                alt: "Instagram",
                url: "https://www.instagram.com/ducksidegame/",
            },
            Link {
                image: "tiktok.svg",
                alt: "TikTok",
                url: "https://www.tiktok.com/@ducksidegame",
            },
        ],
    },
    description: &[
        Fragment::Text("The original student version of Duckside of the Moon. It's a "),
        Fragment::Emph("survival game"),
        Fragment::Text(
            " set in space where you play as a duck astronaut. I built the crafting system & core \
             gameplay loop for this game and balanced them.",
        ),
    ],
    sections: &[
        Section {
            heading: Some("Inventory & Crafting System"),
            title: "Research",
            body: &[Fragment::Text(
                "Before creating our game's inventory system I researched other games to figure \
                 out what kind of UX our inventory system should have. My main reference turned \
                 out to be Subnautica since its inventory system is simple and easy to use which \
                 works great for an exploration focused game like ours.",
            )],
            showcase: Some(Showcase::Video("Il9eWYm6Gnk")),
        },
        Section {
            heading: None,
            title: "Defining items & recipes",
            body: &[Fragment::Text(
                "I looked into several different ways of defining the data for our game's items \
                 and recipes. I decided to use Unreal's Data Tables which would give us a \
                 flexible system for defining new items and recipes.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-items-datatable.png",
                alt: "Datatable Image",
            }),
        },
        Section {
            heading: None,
            title: "Inventory Backend",
            body: &[Fragment::Text(
                "When implementing the inventory system I made sure it was easy to communicate \
                 with and expand upon. Creating reusable components and using event dispatchers \
                 (observer pattern) to communicate state updates to the UI.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-itemcontainer.png",
                alt: "Item Container Blueprint",
            }),
        },
        Section {
            heading: None,
            title: "Crafting Backend",
            body: &[Fragment::Text(
                "For the crafting system I used a similar approach. When a recipe is crafted an \
                 event will be emitted. The player or any other actor can subscribe to this event \
                 and trigger any necessary upgrade logic when it is emitted.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-upgraderecipe.png",
                alt: "Upgrade Recipe Blueprint",
            }),
        },
        Section {
            heading: None,
            title: "Test Level",
            body: &[Fragment::Text(
                "I also maintained a test level for the team in order to test inventory, \
                 crafting, and mining related functionality. This was especially helpful to our \
                 UI Designers when testing new features.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-test-level.jpg",
                alt: "Test Level",
            }),
        },
        Section {
            heading: Some("Items & Mining"),
            title: "Item Pickups",
            body: &[Fragment::Text(
                "Since a core part of our game was about item gathering this process needed to \
                 feel fun and juicy. Item pickups have a physical presence in the world and follow \
                 a curve-based pickup animation when interacted with. Items also react to the \
                 game's gravity zones falling or floating depending on where they are in the \
                 world.",
            )],
            showcase: Some(Showcase::Video("6G7Ye7oFo5g")),
        },
        Section {
            heading: None,
            title: "Mining",
            body: &[Fragment::Text(
                "In order to make the mining look and feel right I collaborated with a VFX artist \
                 & Animator. We worked together to make sure the character, VFX, and gameplay \
                 logic were in sync.",
            )],
            showcase: Some(Showcase::Video("7PckBhyEjBQ")),
        },
        Section {
            heading: Some("Progression Design"),
            title: "Planning Progression",
            body: &[
                Fragment::Text(
                    "Another responsibility I took on was designing the flow of progression in \
                     our game. I was inspired by ",
                ),
                Fragment::Link {
                    url: "https://www.patreon.com/posts/how-my-boss-key-13801754",
                    label: "GMTK's Dungeon Graphs",
                },
                Fragment::Text(
                    " and used this method to sketch out a basic progression flow to see how \
                     players might progress within our game. I also worked with other designers \
                     and artists to refine and improve on these ideas.",
                ),
            ],
            showcase: Some(Showcase::Image {
                src: "dsotm-progression-graph.png",
                alt: "Progression Graph",
            }),
        },
        Section {
            heading: None,
            title: "Designing the resources",
            body: &[Fragment::Text(
                "Together with our concept artist I brainstormed and defined the themes of our \
                 resources (Their names and visuals) based on where you obtain them and what \
                 you'll use them for. This process really helped set the scope of our game and \
                 gave the artists and designers a concrete goal to work towards.",
            )],
            showcase: Some(Showcase::Image {
                src: "dsotm-resource-brainstorming.png",
                alt: "Brainstorming Image",
            }),
        },
        Section {
            heading: Some("Balancing & Tweaks"),
            title: "Recipes",
            body: &[Fragment::Text(
                "Guided by feedback from playtesters I worked towards balancing the game's \
                 recipes. I tackled major hickups in the progression first for example, upgrades \
                 that required more than 2 trips out of the spaceship and back were reduced in \
                 cost in order to make the game not feel too grindy. In the final version \
                 progression felt good to most people, and many simply wanted more content.",
            )],
            showcase: Some(Showcase::Video("2hP9OCdsHEo")),
        },
        Section {
            heading: None,
            title: "Gameplay Balancing",
            body: &[Fragment::Text(
                "Additionally I worked on balancing character variables & upgrades. During \
                 playtesting it became clear that players did not have enough time to explore \
                 before their oxygen and food started running out. I iterated on things like food \
                 capacity, oxygen upgrades, and movement speed in order to smoothen out the \
                 experience.",
            )],
            showcase: Some(Showcase::Video("5Apuv2_Lkk0")),
        },
        Section {
            heading: Some("Other contributions"),
            title: "Character Movement",
            body: &[Fragment::Text(
                "During the project I also spent time assisting with character movement. For \
                 example making sure the player's velocity is accumulated properly between \
                 movement, boosting, and geysers. I also improved the visibility of the character \
                 model by rotating the character separately from the camera allowing you to view \
                 the character from any angle.",
            )],
            showcase: Some(Showcase::Video("qOdHwQqDw7Y")),
        },
        Section {
            heading: None,
            title: "Asteroid Tool",
            body: &[Fragment::Text(
                "I also assisted programmers in tweaking our asteroid tool. In order to make the \
                 resource outcrops fit more naturally on the asteroids, I adjusted their position \
                 based on the asteroid's normal vectors and direction away from its epicenter.",
            )],
            showcase: Some(Showcase::Video("ibE9HOcuVrY")),
        },
        Section {
            heading: None,
            title: "Sound Effects",
            body: &[Fragment::Text(
                "As we didn't have a dedicated sound designer on the team I picked up many of \
                     the sound design tasks. A small selection of these are: Walking, Boosting, \
                     Shooting, Crafting, Open/Close Menu, and Eating.",
            )],
            showcase: Some(Showcase::Video("93oUEzG0bKg")),
        },
    ],
    under_construction: false,
};

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Duckside Of The Moon (Student)");
    view! { <ProjectPageView data=&PAGE /> }
}
