use leptos::prelude::*;

use crate::components::TextSizedImage;
use crate::pages::projects;
use crate::pages::projects::Skill as Icon;

/// A fragment of a project summary. `Emph` renders in the accent color,
/// mirroring the orange highlight from the old Blazor site.
enum Segment {
    Text(&'static str),
    Emph(&'static str),
    Break,
}

struct Project {
    href: &'static str,
    image: &'static str,
    title: &'static str,
    date: &'static str,
    summary: &'static [Segment],
    contributions: &'static [&'static str],
    /// The project page's skills, the single source of truth shared with the
    /// project detail page.
    icons: &'static [Icon],
}

struct Skill {
    image: &'static str,
    title: &'static str,
    years: u8,
}

const IMG: &str = "public/images";

const PROJECTS: [Project; 8] = [
    Project {
        href: "/LegoFramework",
        image: "lego-brick-editor.png",
        title: "LEGO® Brick Editor",
        date: "17-Jun-2025",
        summary: &[
            Segment::Text(
                "At The LEGO Group I worked on a general purpose LEGO framework for Unreal Engine. \
                 The LEGO® Brick Editor was a key deliverable. ",
            ),
            Segment::Text(
                "I mainly contributed to the Brick building UX, Bug fixes, Automated Tests, and ",
            ),
            Segment::Emph("redesigned"),
            Segment::Text(
                " the framework's import pipeline to be more idiomatic, performant and flexible.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Brick Snapping Logic & Tooling",
            "Connection Point Based Transform Gizmo",
            "Import Pipeline Redesign",
        ],
        icons: projects::lego_framework::PAGE.info.skills,
    },
    Project {
        href: "/DuckSideOfTheMoon",
        image: "dsotm-cover.jpg",
        title: "Duck Side of the Moon",
        date: "07-May-2026",
        summary: &[
            Segment::Text(
                "A remake of the original student project. This time it's a cozy adventure game.",
            ),
            Segment::Break,
            Segment::Text("I was the main "),
            Segment::Emph("systems & tools programmer"),
            Segment::Text(
                " for the project and built its core systems using a combination of C++ and \
                 Blueprints. I was also part of establishing the indie company Starbrew Games.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Save Game System",
            "Quest System",
            "Dialogue System",
            "Inventory & Crafting System",
            "Upgrade System",
            "Technical Animation",
            "UI Programming",
            "Various Gameplay Elements",
        ],
        icons: projects::duck_side_of_the_moon::PAGE.info.skills,
    },
    Project {
        href: "/AtticPanic",
        image: "attic-panic-header.jpg",
        title: "Attic Panic",
        date: "12-Jun-2023",
        summary: &[
            Segment::Text("A year long student project. Attic Panic is an "),
            Segment::Emph("online multiplayer"),
            Segment::Text(
                " top-down shooter with rogue-like elements. I mainly worked on the upgrade system, \
                 individual upgrades, and replication of upgrades between server & clients.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Upgrade System",
            "Upgrade Design & Implementation",
            "Upgrade & Gameplay Balancing",
            "Sound Effects & Audio",
        ],
        icons: projects::attic_panic::PAGE.info.skills,
    },
    Project {
        href: "/Radorian",
        image: "radorian-screenshot.jpg",
        title: "Radorian",
        date: "Not Released",
        summary: &[
            Segment::Text("A "),
            Segment::Emph("personal project"),
            Segment::Text(
                " I work on intermittently in my spare time. It's a third person game with an \
                 experimental art style. I'm making everything myself from code to art to sound to \
                 game design.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Third person camera",
            "3D sprite pipeline & tools",
            "Custom shaders",
            "Procedural Geometry",
            "Dynamic gravity",
            "And everything else",
        ],
        icons: projects::radorian::PAGE.info.skills,
    },
    Project {
        href: "/DucksideOfTheMoonStudent",
        image: "dsotm-ingame-ice.jpg",
        title: "Duckside of the Moon (Student)",
        date: "28-Jun-2022",
        summary: &[
            Segment::Text("The original student version of Duckside of the Moon. It's a "),
            Segment::Emph("survival game"),
            Segment::Text(
                " set in space where you play as a duck astronaut. I built the crafting system & \
                 core gameplay loop for this game and balanced them.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Inventory & Crafting System",
            "Mining & Item Pickups",
            "Progression Design",
            "Upgrade & Recipe Balancing",
            "Sound Effects",
        ],
        icons: projects::duckside_of_the_moon_student::PAGE.info.skills,
    },
    Project {
        href: "/LevelEditorPlugin",
        image: "level-editor.jpg",
        title: "Level Editor Plugin",
        date: "21-Oct-2022",
        summary: &[
            Segment::Text("An Unreal Engine "),
            Segment::Emph("editor plugin"),
            Segment::Text(
                " I made in order to get more familiar with C++. The plugin allows you to draw on a \
                 grid and define custom behaviours for this. It was designed with quick iteration \
                 and flexibility in mind.",
            ),
            Segment::Break,
        ],
        contributions: &["Tool Programming", "Tool Design"],
        icons: projects::level_editor_plugin::PAGE.info.skills,
    },
    Project {
        href: "/TwinStickDuckBox",
        image: "tsj-banner.png",
        title: "Twin-Stick Duckbox",
        date: "03-Nov-2021",
        summary: &[
            Segment::Text(
                "A simple project in which we were tasked to design, implement, and polish an \
                 additional enemy and weapon. I also spent time developing a spawn event system \
                 and various other tools for my team.",
            ),
            Segment::Break,
        ],
        contributions: &[
            "Enemy Spawn Event System",
            "Chomper Enemy",
            "Gauss Weapon",
            "Debug & Cheat Menu",
        ],
        icons: projects::twin_stick_duckbox::PAGE.info.skills,
    },
    Project {
        href: "/FolivorasFall",
        image: "ff-title.jpg",
        title: "Folivora's Fall",
        date: "07-Jul-2021",
        summary: &[
            Segment::Text(
                "The first big team project I worked on as a student. Folivora's Fall is a stealth \
                 platformer set in a haunted mansion where you play as a sloth. During this project \
                 I mostly focussed on level design and some implementation.",
            ),
            Segment::Break,
        ],
        contributions: &["Camera System", "Puzzle Design", "Level Blockout"],
        icons: projects::folivoras_fall::PAGE.info.skills,
    },
];

const SKILLS: [Skill; 10] = [
    Skill {
        image: "rust-logo.svg",
        title: "Rust",
        years: 1,
    },
    Skill {
        image: "cpp-logo.svg",
        title: "C++",
        years: 4,
    },
    Skill {
        image: "c-sharp.svg",
        title: "C#",
        years: 4,
    },
    Skill {
        image: "nvim.svg",
        title: "LazyVim",
        years: 1,
    },
    Skill {
        image: "Git-logo.svg",
        title: "Git",
        years: 8,
    },
    Skill {
        image: "perforce.png",
        title: "Perforce",
        years: 5,
    },
    Skill {
        image: "arch-linux.svg",
        title: "Arch Linux",
        years: 1,
    },
    Skill {
        image: "bevy-logo.svg",
        title: "Bevy",
        years: 1,
    },
    Skill {
        image: "ue-logo.svg",
        title: "Unreal",
        years: 6,
    },
    Skill {
        image: "Godot-logo.svg",
        title: "Godot",
        years: 4,
    },
];

#[component]
fn SectionHeading(title: &'static str) -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center py-6 px-4 md:px-8 lg:py-8">
            <span class="flex-1 min-w-0 h-px bg-line" />
            <h2 class="text-base font-bold text-center uppercase whitespace-nowrap md:text-xl tracking-[0.2em] text-ink-muted">
                {title}
            </h2>
            <span class="flex-1 min-w-0 h-px bg-line" />
        </div>
    }
}

#[component]
fn ProjectCard(project: &'static Project) -> impl IntoView {
    let summary = project
        .summary
        .iter()
        .map(|segment| match segment {
            Segment::Text(text) => {
                let text = *text;
                view! { <span>{text}</span> }
                .into_any()
            }
            Segment::Emph(text) => {
                let text = *text;
                view! { <strong class="font-semibold text-background-hover-ink">{text}</strong> }
                .into_any()
            }
            Segment::Break => view! { <br /> }
            .into_any(),
        })
        .collect_view();

    let contributions = project
        .contributions
        .iter()
        .copied()
        .map(|item| {
            view! { <li>{item}</li> }
        })
        .collect_view();

    let icons = project
        .icons
        .iter()
        .map(|icon| {
            view! { <TextSizedImage image=format!("{IMG}/{}", icon.image) alt=icon.alt /> }
        })
        .collect_view();

    view! {
        <a
            href=project.href
            class="block group focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-background-hover-ink"
        >
            <div class="flex flex-col items-center transition-transform duration-300 ease-nav group-hover:scale-[1.02]">
                <span class="py-1.5 px-4 max-w-full font-semibold tracking-wide border-t w-fit rounded-t-nav border-x border-line bg-background-hover text-ink">
                    {project.title}
                </span>
                <div class="flex flex-col w-full border transition-colors duration-300 rounded-b-nav border-line bg-background group-hover:border-background-hover-ink/50">
                    <div class="p-4 md:p-6">
                        <div class="flex flex-col gap-5 lg:flex-row lg:gap-8">
                            <div class="lg:w-1/2">
                                <img
                                    src=format!("{IMG}/{}", project.image)
                                    alt="{project.title} project image"
                                    class="object-cover w-full border opacity-90 transition-opacity duration-300 group-hover:opacity-100 rounded-nav border-line"
                                />
                            </div>
                            <div class="flex flex-col gap-6 lg:w-1/2">
                                <div>
                                    <h3 class="mb-2 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                        "Summary"
                                    </h3>
                                    <p class="leading-relaxed text-ink">{summary}</p>
                                </div>
                                <div>
                                    <h3 class="mb-2 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                        "Contributions"
                                    </h3>
                                    <ul class="pl-5 space-y-1 list-disc marker:text-ink-muted">
                                        {contributions}
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="flex flex-wrap gap-3 items-center pt-4 mt-5 border-t border-line">
                            <span class="flex flex-wrap gap-2 items-center">{icons}</span>
                            <span class="inline-flex items-center py-1.5 px-3 ml-auto text-xs font-semibold tracking-wide uppercase transition-colors duration-150 rounded-nav bg-background-hover text-background-hover-ink group-hover:text-ink">
                                "Read More"
                            </span>
                            <span class="text-ink-muted">{project.date}</span>
                        </div>
                    </div>
                </div>
            </div>
        </a>
    }
}

#[component]
fn SkillCard(skill: &'static Skill) -> impl IntoView {
    view! {
        <div class="flex gap-3 items-center p-4 border transition-colors duration-150 rounded-nav border-line bg-background hover:border-background-hover-ink/50">
            <img
                src=format!("{IMG}/{}", skill.image)
                alt="{skill.title} logo"
                class="object-contain w-10 h-10 shrink-0"
            />
            <div class="min-w-0">
                <h3 class="font-semibold text-ink">{skill.title}</h3>
                <p class="text-ink-muted">
                    {if skill.years == 1 {
                        "1 Year of experience".to_string()
                    } else {
                        format!("{} Years of experience", skill.years)
                    }}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let _ = document().set_title("Jesse Stolk - Software Engineer");

    let projects = PROJECTS
        .iter()
        .map(|project| {
            view! { <ProjectCard project=project /> }
        })
        .collect_view();

    let skills = SKILLS
        .iter()
        .map(|skill| {
            view! { <SkillCard skill=skill /> }
        })
        .collect_view();

    view! {
        <div class="flex flex-col w-full">
            <div class="flex flex-col items-stretch md:flex-row bg-background-light">
                <iframe
                    class="w-full md:order-last md:w-7/12 md:h-auto h-[72vh] max-h-[55vw] md:max-h-none"
                    src="https://www.youtube.com/embed/vqVBLEHiiMU?autoplay=1&loop=1"
                    allow="autoplay"
                    title="About me"
                ></iframe>
                <div class="py-8 px-6 w-full bg-gradient-to-b md:w-6/12 from-background-light to-background">
                    <h1 class="text-3xl font-bold tracking-tight md:text-4xl text-ink">
                        "Jesse Stolk"
                    </h1>
                    <p class="mt-1 text-lg font-semibold md:text-xl text-background-hover-ink">
                        "Software Engineer"
                    </p>
                    <div class="my-6 w-full h-px bg-line" />
                    <p class="leading-relaxed text-ink">
                        "My name is Jesse. I'm a software developer and I enjoy designing systems for simulating and modelling complex interactions. I've been programming for approximately nine years."
                        <br /> <br />
                        "I spent most of my career on games, game tooling, and learning game engine architecture. Having said that, recently I'm interested in software for research and scientific purposes. I've always enjoyed various sciences and feel like contributing to scientific research fits my values better than contributing to the commercial world."
                        <br /> <br />
                        "My favourite tools at the moment are Rust, Bevy Engine, Linux, and LazyVim, but I'm experienced in various other Languages, Game Engines and IDEs as well."
                    </p>
                </div>
            </div>

            <div class="flex flex-col">
                <SectionHeading title="Projects" />
                <div class="flex flex-col gap-10 px-4 pt-4 pb-8 mx-auto w-full max-w-6xl md:px-8 lg:px-0">
                    {projects}
                </div>
            </div>

            <div class="flex flex-col">
                <SectionHeading title="Software Skills" />
                <div class="grid grid-cols-1 gap-4 px-4 pb-14 mx-auto w-full max-w-6xl sm:grid-cols-2 lg:grid-cols-5 lg:px-0">
                    {skills}
                </div>
            </div>
        </div>
    }
}
