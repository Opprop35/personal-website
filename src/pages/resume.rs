use leptos::prelude::*;

struct Experience {
    org: &'static str,
    role: &'static str,
    period: &'static str,
    points: &'static [&'static str],
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
    },
];

struct Education {
    school: &'static str,
    study: &'static str,
    period: &'static str,
    note: Option<&'static str>,
}

const EDUCATION: [Education; 3] = [
    Education {
        school: " BUAS | BREDA UNIVERSITY OF APPLIED SCIENCES",
        study: "Creative Media and Game Technologies",
        period: "Sep 2020 – Jul 2024",
        note: None,
    },
    Education {
        school: "TU DELFT | DELFT UNIVERSITY OF TECHNOLOGY",
        study: "Technical Informatics",
        period: "Sep 2018 – Nov 2019",
        note: None,
    },
    Education {
        school: "ISW GASTHUISLAAN",
        study: "Pre-University Education (VWO)",
        period: "Sep 2012 – Jun 2018",
        note: Some("Socrates (Top 10% of students)"),
    },
];

const SECTION_TITLE: &str = "mb-[6mm] border-b border-line pb-[2.5mm] text-[12px] font-semibold tracking-[0.16em] uppercase text-ink-muted";

#[component]
fn SidebarSection(title: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <section class="mb-[7mm] last:mb-0">
            <h2 class="mb-[3mm] text-[11px] font-semibold tracking-[0.16em] uppercase text-background-hover-ink">
                {title}
            </h2>
            <ul class="space-y-[2.5mm] text-[13px] leading-snug text-ink">{children}</ul>
        </section>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    let experience = EXPERIENCE
        .iter()
        .map(|e| {
            view! {
                <article class="mb-[7mm] last:mb-0">
                    <div class="flex flex-wrap items-baseline justify-between gap-x-3">
                        <h3 class="text-[15px] font-semibold tracking-wide">{e.org}</h3>
                        <span class="text-[12px] text-ink-muted">{e.period}</span>
                    </div>
                    <p class="mt-[1mm] text-[13px] text-background-hover-ink">{e.role}</p>
                    <ul class="mt-[3mm] list-disc space-y-[1.5mm] pl-[5mm] text-[13px] marker:text-ink-muted">
                        {e.points.iter().copied().map(|point| view! { <li>{point}</li> }).collect_view()}
                    </ul>
                </article>
            }
        })
        .collect_view();

    let education = EDUCATION
        .iter()
        .map(|d| {
            view! {
                <article class="mb-[5mm] last:mb-0">
                    <div class="flex flex-wrap items-baseline justify-between gap-x-3">
                        <h3 class="text-[15px] font-semibold tracking-wide">{d.school}</h3>
                        <span class="text-[12px] text-ink-muted">{d.period}</span>
                    </div>
                    <p class="mt-[1mm] text-[13px] text-ink-muted">{d.study}</p>
                    {d.note.map(|note| view! {
                        <p class="mt-[1mm] text-[12px] italic text-ink-muted">{note}</p>
                    })}
                </article>
            }
        })
        .collect_view();

    let personal_details = view! {
        <li>
            <a
                href="https://jessestolk.com"
                target="_blank"
                rel="noopener noreferrer"
                class="text-background-hover-ink hover:underline"
            >
                "jessestolk.com"
            </a>
        </li>
        <li>
            <a
                href="mailto:jeus.stolk@gmail.com"
                class="break-all hover:underline"
            >
                "jeus.stolk@gmail.com"
            </a>
        </li>
        <li>"Birth Date: 10-Nov-1999"</li>
        <li>"Netherlands, Delft"</li>
    };

    let programming = view! {
        <li>"Rust"</li>
        <li>"C++"</li>
        <li>"C#"</li>
    };

    let game_engines = view! {
        <li>"Bevy Engine"</li>
        <li>"Unreal Engine"</li>
        <li>"Godot Engine"</li>
    };

    let version_control = view! {
        <li>"Git"</li>
        <li>"Perforce"</li>
    };

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
        <div class="mx-auto flex w-full max-w-[210mm] flex-1 flex-col bg-background-light">
            <header class="flex items-center gap-[8mm] bg-background px-[12mm] py-[9mm]">
                <img
                    src="public/resume.jpg"
                    alt="Portrait of Jesse Stolk"
                    class="h-[38mm] w-[38mm] shrink-0 rounded-[2mm] object-cover"
                />
                <div class="min-w-0 flex-1">
                    <h1 class="text-[26px] font-bold tracking-[0.06em]">"JESSE STOLK"</h1>
                    <p class="mt-[1.5mm] text-[14px] text-background-hover-ink">
                        "Software Engineer"
                    </p>
                </div>
                    <button class="print:hidden inline-flex items-center gap-2 rounded-nav px-3 py-2 text-[12px] transition-colors duration-150 hover:bg-background-hover hover:text-background-hover-ink" on:click=|_| { window().print().ok(); }>
                        <svg viewBox="0 0 24 24" fill="currentColor" class="h-[1.1rem] w-[1.1rem]" role="img" aria-hidden="true">
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

                <main class="flex min-w-0 flex-1 flex-col px-[9mm] py-[8mm]">
                    <section>
                        <h2 class=SECTION_TITLE>"Experience"</h2>
                        {experience}
                    </section>
                    <p class="mt-[5mm] text-[13px]">
                        <a
                            href="https://jessestolk.com"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-background-hover-ink hover:underline"
                        >
                            "See jessestolk.com for more info…"
                        </a>
                    </p>

                    <section class="mt-auto pt-[4mm]">
                        <h2 class=SECTION_TITLE>"Education"</h2>
                        {education}
                    </section>
                </main>
            </div>
        </div>
    }
}
