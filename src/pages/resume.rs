use leptos::prelude::*;

use crate::components::Link;
use crate::skill::Skill;
use crate::IMAGE_DIR;

struct Experience {
    org: &'static str,
    role: &'static str,
    period: &'static str,
    points: &'static [&'static str],
    url: Option<&'static str>,
}

const EXPERIENCE: [Experience; 3] = [
    Experience {
        org: "THE LEGO GROUP",
        role: "LEGO® Brick Editor | Tools Programmer",
        period: "November 2024 – July 2026",
        points: &[
            "Brick Snapping Logic & Tooling",
            "Import Pipeline Redesign",
            "Connection Point Based Gizmo",
        ],
        url: Some("https://www.lego.com/en-us/aboutus/lego-group"),
    },
    Experience {
        org: "STARBREW GAMES",
        role: "Duckside of the Moon | Game Developer",
        period: "June 2023 – June 2024",
        points: &[
            "Dialogue, Quest, Crafting, & Upgrade Systems",
            "Save Game System",
            "Technical Animation",
            "UI Programming",
        ],
        url: Some("https://starbrewgames.com/"),
    },
    Experience {
        org: "PERSONAL PROJECTS",
        role: "Various projects | Game Developer",
        period: "April 2021 – Present",
        points: &[
            "Asset Pipelines & Tools",
            "Procedural Geometry",
            "Online Multiplayer",
            "Shader Programming",
            "Third Person Camera",
        ],
        url: None,
    },
];

struct Education {
    school: &'static str,
    study: &'static str,
    period: &'static str,
    note: Option<&'static str>,
    url: Option<&'static str>,
}

const EDUCATION: [Education; 3] = [
    Education {
        school: "BUAS | BREDA UNIVERSITY OF APPLIED SCIENCES",
        study: "Creative Media and Game Technologies",
        period: "Sep 2020 – Jul 2024",
        note: None,
        url: Some("https://www.buas.nl/en"),
    },
    Education {
        school: "TU DELFT | DELFT UNIVERSITY OF TECHNOLOGY",
        study: "Technical Informatics",
        period: "Sep 2018 – Nov 2019",
        note: None,
        url: Some("https://www.tudelft.nl/"),
    },
    Education {
        school: "ISW GASTHUISLAAN",
        study: "Pre-University Education (VWO)",
        period: "Sep 2012 – Jun 2018",
        note: Some("Socrates (Top 10% of students)"),
        url: Some("https://iswgasthuislaan.nl/"),
    },
];

fn org_name(name: &'static str, url: Option<&'static str>) -> AnyView {
    if let Some(url) = url {
        return view! {
            <Link href=url class="hover:underline focus-ring" external=true>
                {name}
            </Link>
        }
        .into_any();
    }
    view! { <span>{name}</span> }
    .into_any()
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
                        class="inline-flex items-center hover:underline gap-[1.5mm] focus-ring"
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

    let experience = EXPERIENCE
        .iter()
        .map(|e| {
            view! {
                <article class="last:mb-0 mb-[7mm]">
                    <div class="flex flex-wrap gap-x-3 justify-between items-baseline">
                        <h3 class="font-semibold tracking-wide text-[15px]">
                            {org_name(e.org, e.url)}
                        </h3>
                        <span class="text-[12px] text-ink-muted">{e.period}</span>
                    </div>
                    <p class="mt-[1mm] text-[13px] text-background-hover-ink">{e.role}</p>
                    <ul class="list-disc mt-[3mm] space-y-[1.5mm] pl-[5mm] text-[13px] marker:text-ink-muted">
                        {e
                            .points
                            .iter()
                            .copied()
                            .map(|point| view! { <li>{point}</li> })
                            .collect_view()}
                    </ul>
                </article>
            }
        })
        .collect_view();

    let education = EDUCATION
        .iter()
        .map(|d| {
            view! {
                <article class="last:mb-0 mb-[5mm]">
                    <div class="flex flex-wrap gap-x-3 justify-between items-baseline">
                        <h3 class="font-semibold tracking-wide text-[15px]">
                            {org_name(d.school, d.url)}
                        </h3>
                        <span class="text-[12px] text-ink-muted">{d.period}</span>
                    </div>
                    <p class="mt-[1mm] text-[13px] text-ink-muted">{d.study}</p>
                    {d
                        .note
                        .map(|note| {
                            view! {
                                <p class="italic mt-[1mm] text-[12px] text-ink-muted">{note}</p>
                            }
                        })}
                </article>
            }
        })
        .collect_view();

    let personal_details = view! {
        <li>
            <Link
                href="https://jessestolk.com"
                class="link-muted"
                external=true
            >
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
                        <h2 class="mb-[6mm] border-b border-line pb-[2.5mm] label-uppercase">"Experience"</h2>
                        {experience}
                    </section>
                    <p class="mt-[5mm] text-[13px]">
                        <Link
                            href="https://jessestolk.com"
                            class="link-muted"
                            external=true
                        >
                            "See jessestolk.com for more info…"
                        </Link>
                    </p>

                    <section class="mt-auto pt-[4mm]">
                        <h2 class="mb-[6mm] border-b border-line pb-[2.5mm] label-uppercase">"Education"</h2>
                        {education}
                    </section>
                </main>
            </div>
        </div>
    }
}
