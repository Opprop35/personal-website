use leptos::prelude::*;

use crate::{
    components::{Paragraphs, ProjectCard, Section, SkillCard, SmallProjectCard},
    content::projects::ProjectId,
    models,
};

const PROJECTS: [ProjectId; 4] = [
    ProjectId::LegoFramework,
    ProjectId::DuckSideOfTheMoon,
    ProjectId::AtticPanic,
    ProjectId::Radorian,
];

const OLDER_PROJECTS: [ProjectId; 4] = [
    ProjectId::DucksideOfTheMoonStudent,
    ProjectId::LevelEditorPlugin,
    ProjectId::TwinStickDuckbox,
    ProjectId::FolivorasFall,
];

const INTRODUCTION: [models::Paragraph; 3] = [
    models::Paragraph::Text(models::TextParagraph {
        fragments: &[models::TextFragment::Text(
            "My name is Jesse, I'm a software engineer who has been programming for \
             roughly nine years.",
        )],
    }),
    models::Paragraph::Text(models::TextParagraph {
        fragments: &[models::TextFragment::Text(
            "I spent most of my career on games, game tooling, and learning game engine \
             architecture. Having said that, recently I'm interested in exploring other areas of \
             software engineering such as research software engineering and cyber security.",
        )],
    }),
    models::Paragraph::Text(models::TextParagraph {
        fragments: &[
            models::TextFragment::Text("My favourite tools at the moment are "),
            models::TextFragment::skill(models::Skill::Rust),
            models::TextFragment::Text(", "),
            models::TextFragment::skill(models::Skill::Bevy),
            models::TextFragment::Text(", "),
            models::TextFragment::skill(models::Skill::ArchLinux),
            models::TextFragment::Text(", and "),
            models::TextFragment::skill(models::Skill::LazyVim),
            models::TextFragment::Text(
                ", but I'm experienced in various other Languages, Game Engines and IDEs as well.",
            ),
        ],
    }),
];

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Software Engineer");

    let projects = PROJECTS
        .iter()
        .copied()
        .map(|project| {
            view! { <ProjectCard project=project.project() /> }
        })
        .collect_view();

    let older_projects = OLDER_PROJECTS
        .iter()
        .copied()
        .map(|project| {
            view! { <SmallProjectCard project=project.project() /> }
        })
        .collect_view();

    let skills = models::Skill::ALL
        .iter()
        .copied()
        .map(|skill| {
            view! { <SkillCard skill=skill /> }
        })
        .collect_view();

    view! {
        <div class="flex flex-col gap-10">
            <div class="flex flex-col items-stretch md:flex-row bg-surface">
                <iframe
                    class="w-full md:order-last md:w-7/12 md:h-auto h-[72vh] max-h-[55vw] md:max-h-none"
                    src="https://www.youtube-nocookie.com/embed/vqVBLEHiiMU?autoplay=1&mute=1&loop=1&playlist=vqVBLEHiiMU&controls=0&rel=0"
                    allow="autoplay"
                    title="Introduction video"
                    tabindex="-1"
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
