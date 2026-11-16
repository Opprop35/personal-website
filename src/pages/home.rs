use leptos::prelude::*;

use crate::components::TextSizedImage;
use crate::pages::projects::{self, fragments_view};
use crate::project::Project;
use crate::skill::Skill;
use crate::IMAGE_DIR;

/// The projects shown on the home page, in display order. Each entry is the
/// same `Project` that drives the project's detail page.
const PROJECTS: [&Project; 8] = [
    &projects::lego_framework::PAGE,
    &projects::duck_side_of_the_moon::PAGE,
    &projects::attic_panic::PAGE,
    &projects::radorian::PAGE,
    &projects::duckside_of_the_moon_student::PAGE,
    &projects::level_editor_plugin::PAGE,
    &projects::twin_stick_duckbox::PAGE,
    &projects::folivoras_fall::PAGE,
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
    let description = fragments_view(project.description);

    let contributions = project
        .contributions
        .iter()
        .copied()
        .map(|item| {
            view! { <li>{item}</li> }
        })
        .collect_view();

    let skills = project
        .info
        .skills
        .iter()
        .copied()
        .map(|skill| {
            view! {
                <a
                    href=skill.url()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="transition-transform duration-150 hover:scale-110 ease-nav"
                >
                    <TextSizedImage
                        image=format!("{IMAGE_DIR}/{}", skill.image())
                        alt=skill.alt()
                    />
                </a>
            }
        })
        .collect_view();

    view! {
        <div class="block group">
            <div class="flex flex-col items-center transition-transform duration-300 ease-nav group-hover:scale-[1.02]">
                <a
                    href=project.href
                    class="flex w-fit focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-background-hover-ink"
                >
                    <span class="py-1.5 px-4 max-w-full font-semibold tracking-wide border-t w-fit rounded-t-nav border-x border-line bg-background-hover text-ink">
                        {project.title}
                    </span>
                </a>
                <div class="flex flex-col w-full border transition-colors duration-300 rounded-b-nav border-line bg-background group-hover:border-background-hover-ink/50">
                    <div class="px-4 pt-4 md:px-6 md:pt-6">
                        <div class="flex flex-col gap-5 lg:flex-row lg:gap-8">
                            <a
                                href=project.href
                                class="block lg:w-1/2 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-background-hover-ink"
                            >
                                <img
                                    src=format!("{IMAGE_DIR}/{}", project.image)
                                    alt=format!("{} project image", project.title)
                                    class="object-cover w-full border opacity-90 transition-opacity duration-300 group-hover:opacity-100 rounded-nav border-line"
                                />
                            </a>
                            <div class="flex flex-col gap-6 lg:w-1/2">
                                <div>
                                    <h3 class="mb-2 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                        "Summary"
                                    </h3>
                                    <p class="leading-relaxed text-ink">{description}</p>
                                </div>
                                <a
                                    href=project.href
                                    class="block focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-background-hover-ink"
                                >
                                    <div>
                                        <h3 class="mb-2 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                            "Contributions"
                                        </h3>
                                        <ul class="pl-5 space-y-1 list-disc marker:text-ink-muted">
                                            {contributions}
                                        </ul>
                                    </div>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-wrap gap-3 items-center px-4 pt-4 pb-4 mt-5 w-full border-t md:px-6 md:pb-6 border-line">
                        <span class="flex flex-wrap gap-2 items-center">{skills}</span>
                        <a
                            href=project.href
                            class="inline-flex items-center py-1.5 px-3 ml-auto text-xs font-semibold tracking-wide uppercase transition-colors duration-150 rounded-nav bg-background-hover text-background-hover-ink group-hover:text-ink"
                        >
                            "Read More"
                        </a>
                        <span class="text-ink-muted">{project.info.release_date_label()}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SkillCard(skill: Skill) -> impl IntoView {
    let years_text = if skill.years() < 1.0 {
        "Less than a Year of experience".to_string()
    } else if skill.years() == 1.0 {
        "1 Year of experience".to_string()
    } else {
        format!("{} Years of experience", skill.years())
    };

    view! {
        <a
            href=skill.url()
            target="_blank"
            rel="noopener noreferrer"
            class="flex gap-3 items-center p-4 w-full border transition-colors duration-150 rounded-nav border-line bg-background hover:border-background-hover-ink/50 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-background-hover-ink"
        >
            <img
                src=format!("{IMAGE_DIR}/{}", skill.image())
                alt=skill.alt()
                class="object-contain w-10 h-10 shrink-0"
            />
            <div class="min-w-0">
                <h3 class="font-semibold text-ink">{skill.title()}</h3>
                <p class="text-ink-muted">{years_text}</p>
            </div>
        </a>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    document().set_title("Jesse Stolk - Software Engineer");

    let projects = PROJECTS
        .iter()
        .copied()
        .map(|project| {
            view! { <ProjectCard project=project /> }
        })
        .collect_view();

    let skills = Skill::ALL
        .iter()
        .copied()
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
