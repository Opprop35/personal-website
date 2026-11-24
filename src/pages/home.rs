use leptos::prelude::*;

use crate::components::{Link, Paragraph, Paragraphs, Section, TextFragment, TextSizedImage};
use crate::pages::projects;
use crate::project::Project;
use crate::skill::Skill;
use crate::IMAGE_DIR;

const PROJECTS: [&Project; 4] = [
    &projects::lego_framework::PROJECT,
    &projects::duck_side_of_the_moon::PROJECT,
    &projects::attic_panic::PROJECT,
    &projects::radorian::PROJECT,
];

const OLDER_PROJECTS: [&Project; 4] = [
    &projects::duckside_of_the_moon_student::PROJECT,
    &projects::level_editor_plugin::PROJECT,
    &projects::twin_stick_duckbox::PROJECT,
    &projects::folivoras_fall::PROJECT,
];

const INTRODUCTION: [Paragraph; 3] = [
    Paragraph::Text(&[TextFragment::Text(
        "My name is Jesse, I'm a software engineer who has been programming for \
         roughly nine years.",
    )]),
    Paragraph::Text(&[TextFragment::Text(
        "I spent most of my career on games, game tooling, and learning game engine \
         architecture. Having said that, recently I'm interested in exploring other areas of \
         software engineering such as research software engineering and cyber security.",
    )]),
    Paragraph::Text(&[
        TextFragment::Text("My favourite tools at the moment are "),
        TextFragment::skill(Skill::Rust),
        TextFragment::Text(", "),
        TextFragment::skill(Skill::Bevy),
        TextFragment::Text(", "),
        TextFragment::skill(Skill::ArchLinux),
        TextFragment::Text(", and "),
        TextFragment::skill(Skill::LazyVim),
        TextFragment::Text(
            ", but I'm experienced in various other Languages, Game Engines and IDEs as well.",
        ),
    ]),
];

#[component]
fn ProjectCard(project: &'static Project) -> impl IntoView {
    let skills = project
        .info
        .skills
        .iter()
        .copied()
        .map(|skill| {
            view! {
                <Link href=skill.url() class="image-link" external=true>
                    <TextSizedImage
                        image=format!("{IMAGE_DIR}/{}", skill.image())
                        alt=skill.alt()
                    />
                </Link>
            }
        })
        .collect_view();

    view! {
        <div class="block">
            <div class="flex flex-col items-center transition-transform duration-300 ease-nav has-[.link-nav:hover]:scale-[1.02]">
                <a tabindex="-1" href=project.href class="flex focus-ring link-nav w-fit">
                    <span class="py-1.5 px-4 max-w-full font-semibold tracking-wide border-t w-fit rounded-t-nav border-x border-line bg-surface-raised text-ink">
                        {project.title}
                    </span>
                </a>
                <div class="flex flex-col w-full border transition-colors duration-300 rounded-b-nav border-line bg-surface-deep has-[.link-nav:hover]:border-ink-muted/50">
                    <div class="px-4 pt-4 md:px-6 md:pt-6">
                        <div class="flex flex-col gap-5 lg:flex-row lg:gap-8">
                            <a href=project.href class="block lg:w-1/2 focus-ring link-nav">
                                <img
                                    src=format!("{IMAGE_DIR}/{}", project.card_thumbnail)
                                    alt=format!("{} project image", project.title)
                                    class="object-cover w-full border opacity-90 transition-opacity duration-300 has-[.link-nav:hover]:opacity-100 rounded-default border-line"
                                />
                            </a>
                            <div class="flex flex-col gap-6 lg:w-1/2">
                                <div>
                                    <h3 class="mb-2 label-uppercase">"Summary"</h3>
                                    <Paragraphs paragraphs=project.description />
                                </div>
                                <div>
                                    <h3 class="mb-2 label-uppercase">"Main Contributions"</h3>
                                    <Paragraphs paragraphs=project.main_contributions />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-wrap gap-3 items-center px-4 pt-4 pb-4 mt-5 w-full border-t md:px-6 md:pb-6 border-line">
                        <span class="flex flex-wrap gap-2 items-center">{skills}</span>
                        <a
                            href=project.href
                            class="inline-flex items-center py-1.5 px-3 ml-auto font-semibold tracking-wide uppercase link-nav text-[0.875em] text-ink bg-button"
                        >
                            "Read More"
                        </a>
                        <span class="text-ink-muted">{project.info.end_date.label()}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SkillCard(skill: Skill) -> impl IntoView {
    let year_text = if skill.years() <= 1.0 {
        "year"
    } else {
        "years"
    };
    let experience_text = format!("{} {} of experience", skill.years(), year_text);

    view! {
        <Link
            href=skill.url()
            class="flex gap-3 items-center p-4 w-full border transition-colors duration-150 focus-ring border-line bg-surface-deep hover:border-ink-muted/50"
            external=true
        >
            <img
                src=format!("{IMAGE_DIR}/{}", skill.image())
                alt=skill.alt()
                class="object-contain w-10 h-10 shrink-0"
            />
            <div class="min-w-0">
                <h3 class="font-semibold text-ink">{skill.title()}</h3>
                <p class="text-ink-muted">{experience_text}</p>
            </div>
        </Link>
    }
}

#[component]
fn OlderProjectRow(project: &'static Project) -> impl IntoView {
    view! {
        <Link
            href=project.href
            class="flex flex-wrap gap-y-1 gap-x-3 items-baseline py-3 px-4 w-full border transition-colors duration-150 focus-ring border-line bg-surface-deep hover:border-ink-muted/50"
        >
            <span class="font-semibold text-ink">{project.title}</span>
            <span class="text-ink-muted">{project.info.project_type.label()}</span>
            <span class="ml-auto text-ink-muted">{project.info.end_date.label()}</span>
        </Link>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Software Engineer");

    let projects = PROJECTS
        .iter()
        .copied()
        .map(|project| {
            view! { <ProjectCard project=project /> }
        })
        .collect_view();

    let older_projects = OLDER_PROJECTS
        .iter()
        .copied()
        .map(|project| {
            view! { <OlderProjectRow project=project /> }
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
        <div class="flex flex-col gap-10 w-full">
            <div class="flex flex-col items-stretch md:flex-row bg-surface">
                <iframe
                    class="w-full md:order-last md:w-7/12 md:h-auto h-[72vh] max-h-[55vw] md:max-h-none"
                    src="https://www.youtube-nocookie.com/embed/vqVBLEHiiMU?autoplay=1&mute=1&loop=1&playlist=vqVBLEHiiMU"
                    allow="autoplay"
                    title="About me"
                ></iframe>
                <div class="py-8 px-6 w-full bg-gradient-to-b md:w-6/12 from-surface to-surface-deep">
                    <h1 class="font-bold tracking-tight text-[2.25em] text-ink">"Jesse Stolk"</h1>
                    <h2 class="mt-1 font-semibold text-[1.25em] text-ink-muted">
                        "Software Engineer"
                    </h2>
                    <div class="my-[1em] divider" />
                    <Paragraphs
                        paragraphs=&INTRODUCTION
                        class="space-y-5 leading-relaxed text-ink"
                    />
                </div>
            </div>

            <Section title="Projects">
                <div class="flex flex-col gap-10 px-4 pb-8 mx-auto w-full max-w-6xl md:px-8 lg:px-0">
                    {projects}
                </div>
            </Section>

            <Section title="Older projects" collapsible=true>
                <div class="flex flex-col gap-3 px-4 pb-8 mx-auto w-full max-w-6xl md:px-8 lg:px-0">
                    {older_projects}
                </div>
            </Section>

            <Section title="Software Skills">
                <div class="grid grid-cols-2 gap-4 px-4 pb-14 mx-auto w-full max-w-6xl sm:grid-cols-3 md:grid-cols-4">
                    {skills}
                </div>
            </Section>
        </div>
    }
}
