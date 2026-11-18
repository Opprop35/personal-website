use leptos::prelude::*;

use crate::components::{Fragment, Fragments, Link};
use crate::organization::Organization;
use crate::skill::Skill;
use crate::IMAGE_DIR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExperienceSource {
    Organization(Organization),
    Personal,
}

impl ExperienceSource {
    fn name(self) -> &'static str {
        match self {
            Self::Organization(organization) => organization.title(),
            Self::Personal => "Personal Projects",
        }
    }

    fn url(self) -> Option<&'static str> {
        match self {
            Self::Organization(organization) => Some(organization.url()),
            Self::Personal => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExperienceData {
    source: ExperienceSource,
    subtitle: &'static str,
    period: &'static str,
    details: Option<&'static [Fragment]>,
}

const PROJECTS: [ExperienceData; 3] = [
    ExperienceData {
        source: ExperienceSource::Organization(Organization::TheLegoGroup),
        subtitle: "LEGO® Brick Editor | Tools Programmer",
        period: "November 2024 – July 2026",
        details: Some(&[
            Fragment::List(&[
                "Brick Snapping Logic & Tooling",
                "Import Pipeline Redesign",
                "Connection Point Based Gizmo",
            ]),
        ]),
    },
    ExperienceData {
        source: ExperienceSource::Organization(Organization::StarbrewGames),
        subtitle: "Duckside of the Moon | Game Developer",
        period: "June 2023 – June 2024",
        details: Some(&[
            Fragment::List(&[
                "Dialogue, Quest, Crafting, & Upgrade Systems",
                "Save Game System",
                "Technical Animation",
                "UI Programming",
            ]),
        ]),
    },
    ExperienceData {
        source: ExperienceSource::Personal,
        subtitle: "Various projects | Game Developer",
        period: "April 2021 – Present",
        details: Some(&[
            Fragment::List(&[
                "Asset Pipelines & Tools",
                "Procedural Geometry",
                "Online Multiplayer",
                "Shader Programming",
                "Third Person Camera",
            ]),
        ]),
    },
];

const SOCRATES_NOTE: [Fragment; 2] = [
    Fragment::Link {
        url: "https://www.socrateshonours.org/lid-worden/toelatingscriteria",
        label: "Socrates",
    },
    Fragment::MutedText(" (Top 10% of students)"),
];

const EDUCATION: [ExperienceData; 3] = [
    ExperienceData {
        source: ExperienceSource::Organization(Organization::BredaUniversityOfAppliedSciences),
        subtitle: "Creative Media and Game Technologies",
        period: "Sep 2020 – Jul 2024",
        details: None,
    },
    ExperienceData {
        source: ExperienceSource::Organization(Organization::TuDelft),
        subtitle: "Computer Science",
        period: "Sep 2018 – Nov 2019",
        details: None,
    },
    ExperienceData {
        source: ExperienceSource::Organization(Organization::IswGasthuislaan),
        subtitle: "Pre-University Education (VWO)",
        period: "Sep 2012 – Jun 2018",
        details: Some(&SOCRATES_NOTE),
    },
];

#[component]
fn ExperienceHeader(source: ExperienceSource) -> AnyView {
    let name = source.name();
    if let Some(url) = source.url() {
        return view! {
            <Link href=url class="hover:underline focus-ring" external=true>
                {name}
            </Link>
        }
        .into_any();
    }
    view! { <span>{name}</span> }.into_any()
}

#[component]
fn Experience(experience: ExperienceData) -> impl IntoView {
    view! {
        <article class="last:mb-0 mb-[7mm]">
            <div class="flex flex-wrap gap-x-3 justify-between items-baseline">
                <h3 class="font-semibold tracking-wide text-[15px] uppercase">
                    <ExperienceHeader source=experience.source />
                </h3>
                <span class="text-[12px] text-ink-muted">{experience.period}</span>
            </div>
            <p class="mt-[1mm] text-[13px] text-ink-muted">{experience.subtitle}</p>
            {experience
                .details
                .map(|details| {
                    view! {
                        <div class="text-[12px] leading-snug">
                            <Fragments fragments=details />
                        </div>
                    }
                })}
        </article>
    }
}

fn skill_items(skills: &[Skill]) -> impl IntoView {
    skills
        .iter()
        .copied()
        .map(|skill| {
            let title = skill.title();
            let url = skill.url();
            let image = format!("{IMAGE_DIR}/{}", skill.image());
            let alt = skill.alt();
            view! {
                <li>
                    <Link
                        href=url
                        class="inline-flex items-center hover:underline focus-ring gap-[1.5mm]"
                        external=true
                    >
                        <img
                            src=image
                            alt=alt
                            class="object-contain h-[1.5em] w-[1.5em] shrink-0"
                        />
                        {title}
                    </Link>
                </li>
            }
        })
        .collect_view()
}

#[component]
fn SidebarSection(title: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <section class="last:mb-0 mb-[7mm]">
            <h2 class="font-semibold uppercase mb-[3mm] text-[11px] tracking-[0.16em] text-background-hover-ink">
                {title}
            </h2>
            <ul class="leading-snug space-y-[2.5mm] text-[13px] text-ink">{children}</ul>
        </section>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    document().set_title("Jesse Stolk - Resume");

    let projects = PROJECTS
        .iter()
        .copied()
        .map(|experience| {
            view! { <Experience experience=experience /> }
        })
        .collect_view();

    let education = EDUCATION
        .iter()
        .copied()
        .map(|experience| {
            view! { <Experience experience=experience /> }
        })
        .collect_view();

    let personal_details = view! {
        <li>
            <Link href="https://jessestolk.com" class="link-muted" external=true>
                "jessestolk.com"
            </Link>
        </li>
        <li>
            <a href="mailto:jeus.stolk@gmail.com" class="break-all hover:underline focus-ring">
                "jeus.stolk@gmail.com"
            </a>
        </li>
        <li>"Birth Date: 10-Nov-1999"</li>
        <li>"Netherlands, Delft"</li>
    };

    let programming = skill_items(&[Skill::Rust, Skill::Cpp, Skill::CSharp]);

    let game_engines = skill_items(&[Skill::Bevy, Skill::Unreal, Skill::Godot]);

    let version_control = skill_items(&[Skill::Git, Skill::Perforce]);

    let languages = view! {
        <li>"English: Fluent"</li>
        <li>"Dutch: Native"</li>
    };

    let hobbies = view! {
        <li>"EGYM"</li>
        <li>"Yoga"</li>
        <li>"Running"</li>
        <li>"Game Development"</li>
        <li>"Board Games"</li>
    };

    view! {
        <div class="flex flex-col flex-1 mx-auto w-full max-w-[210mm] bg-background-light">
            <header class="flex items-center gap-[8mm] bg-background px-[12mm] py-[9mm]">
                <img
                    src="public/images/personal-photo.webp"
                    alt="Portrait of Jesse Stolk"
                    class="object-cover h-[38mm] w-[38mm] shrink-0 rounded-[2mm]"
                />
                <div class="flex-1 min-w-0">
                    <h1 class="font-bold text-[26px] tracking-[0.06em]">"JESSE STOLK"</h1>
                    <p class="mt-[1.5mm] text-[14px] text-background-hover-ink">
                        "Software Engineer"
                    </p>
                </div>
                <button
                    class="inline-flex gap-2 items-center py-2 px-3 print:hidden text-[12px] bg-button"
                    on:click=|_| {
                        window().print().ok();
                    }
                >
                    <svg
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="h-[1.1rem] w-[1.1rem]"
                        role="img"
                        aria-hidden="true"
                    >
                        <path d="M12 16 6 10h4V4h4v6h4l-6 6zm-7 4v-2h14v2H5z" />
                    </svg>
                    "Download PDF"
                </button>
            </header>

            <div class="flex flex-1">
                <aside class="w-[68mm] shrink-0 bg-background-hover px-[7mm] py-[9mm]">
                    <SidebarSection title="Personal Details" children=personal_details />
                    <SidebarSection title="Programming" children=programming />
                    <SidebarSection title="Game Engines" children=game_engines />
                    <SidebarSection title="Version Control" children=version_control />
                    <SidebarSection title="Languages" children=languages />
                    <SidebarSection title="Hobbies" children=hobbies />
                </aside>

                <main class="flex flex-col flex-1 min-w-0 px-[9mm] py-[8mm]">
                    <section>
                        <h2 class="label-uppercase">"Projects"</h2>
                        <div class="mt-[2.5mm] mb-[6mm] divider" />
                        {projects}
                    </section>
                    <p class="mt-[5mm] text-[13px]">
                        <Link href="https://jessestolk.com" class="link-muted" external=true>
                            "See jessestolk.com for more info…"
                        </Link>
                    </p>

                    <section class="mt-auto pt-[4mm]">
                        <h2 class="label-uppercase">"Education"</h2>
                        <div class="mt-[2.5mm] mb-[6mm] divider" />
                        {education}
                    </section>
                </main>
            </div>
        </div>
    }
}
