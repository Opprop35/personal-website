use leptos::prelude::*;

use crate::assets::image;
use crate::components::{Experience, Link, SidebarSection, SkillItem};
use crate::content::resume::{EDUCATION, PROJECTS};
use crate::locale::resume_label;
use crate::models;

fn skill_items(skills: &[models::Skill]) -> impl IntoView {
    skills
        .iter()
        .copied()
        .map(|skill| view! { <SkillItem skill=skill /> })
        .collect_view()
}

// TODO: Create a full data model for resume so that it's easy to edit the content or create multiple versions.
#[component]
pub fn Page() -> impl IntoView {
    document().set_title(&format!("Jesse Stolk - {}", resume_label()));

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
            <Link href="https://jessestolk.com" class="text-link" external=true>
                "jessestolk.com"
            </Link>
        </li>
        <li>
            <a href="mailto:jeus.stolk@gmail.com" class="break-all text-link">
                "jeus.stolk@gmail.com"
            </a>
        </li>
        <li>"Birth Date: 10-Nov-1999"</li>
        <li>"Netherlands, Delft"</li>
    };

    let programming = skill_items(&[
        models::Skill::Rust,
        models::Skill::Cpp,
        models::Skill::CSharp,
    ]);

    let game_engines = skill_items(&[
        models::Skill::Bevy,
        models::Skill::Unreal,
        models::Skill::Godot,
    ]);

    let version_control = skill_items(&[models::Skill::Git, models::Skill::Perforce]);

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
        <div class="flex overflow-x-auto overscroll-x-contain flex-1 min-h-0 print:overflow-visible">
            <article class="flex flex-col mx-auto ring-1 shrink-0 w-[210mm] ring-line print:min-h-[297mm] print:overflow-hidden print:bg-surface print:ring-0">
                <header class="flex items-center gap-[8mm] bg-surface-deep px-[12mm] py-[9mm]">
                    <img
                        src=image("personal-photo.webp")
                        alt="Portrait of Jesse Stolk"
                        class="object-cover h-[38mm] w-[38mm] shrink-0 rounded-[2mm]"
                    />
                    <div class="flex-1 min-w-0">
                        <h1 class="font-bold text-[26px] tracking-[0.06em]">"JESSE STOLK"</h1>
                        <p class="mt-[1.5mm] text-[14px] text-ink-muted">"Software Engineer"</p>
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
                    <aside class="w-[68mm] shrink-0 bg-surface-raised px-[7mm] py-[9mm]">
                        <SidebarSection title="Personal Details" children=personal_details />
                        <SidebarSection title="Programming" children=programming />
                        <SidebarSection title="Game Engines" children=game_engines />
                        <SidebarSection title="Version Control" children=version_control />
                        <SidebarSection title="Languages" children=languages />
                        <SidebarSection title="Hobbies" children=hobbies />
                    </aside>

                    <div class="flex flex-col flex-1 min-w-0 px-[9mm] py-[8mm]">
                        <section class="break-inside-avoid">
                            <h2 class="font-semibold uppercase text-[16px] tracking-[0.16em] text-ink-muted">
                                "Projects"
                            </h2>
                            <div class="mt-[2.5mm] mb-[6mm] divider" />
                            {projects}
                        </section>
                        <p class="mt-[5mm] text-[13px]">
                            <Link href="https://jessestolk.com" class="text-link" external=true>
                                "See jessestolk.com for more info…"
                            </Link>
                        </p>

                        <section class="pt-[6mm] break-inside-avoid">
                            <h2 class="font-semibold uppercase text-[16px] tracking-[0.16em] text-ink-muted">
                                "Education"
                            </h2>
                            <div class="mt-[2.5mm] mb-[6mm] divider" />
                            {education}
                        </section>
                    </div>
                </div>
            </article>
        </div>
    }
}
