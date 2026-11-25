use leptos::prelude::*;

use crate::assets::image;
use crate::components::{Labeled, Link, ListParagraph, TextParagraph, TextSizedImage};
use crate::models;

#[component]
pub fn ProjectCard(project: &'static models::Project) -> impl IntoView {
    let skills = project
        .info
        .skills
        .iter()
        .copied()
        .map(|skill| {
            view! {
                <Link href=skill.url() class="image-link" external=true>
                    <TextSizedImage image=image(skill.image()) alt=skill.alt() />
                </Link>
            }
        })
        .collect_view();

    view! {
        <article class="flex flex-col items-center transition-transform duration-300 ease-nav has-[.link-nav:hover]:scale-[1.02]">
            <header>
                <a tabindex="-1" href=project.path() class="flex focus-ring link-nav w-fit">
                    <h1 class="py-1.5 px-4 max-w-full font-semibold tracking-wide border-t w-fit rounded-t-nav border-x border-line bg-surface-raised text-ink">
                        {project.title}
                    </h1>
                </a>
            </header>
            <div class="flex flex-col w-full border transition-colors duration-300 rounded-b-nav border-line bg-surface-deep has-[.link-nav:hover]:border-ink-muted/50">
                <div class="px-4 pt-4 md:px-6 md:pt-6">
                    <div class="flex flex-col gap-5 lg:flex-row lg:gap-8">
                        <a href=project.path() class="block lg:w-1/2 focus-ring link-nav">
                            <img
                                src=image(project.card_thumbnail)
                                alt=format!("{} project image", project.title)
                                class="object-cover w-full border opacity-90 transition-opacity duration-300 has-[.link-nav:hover]:opacity-100 rounded-default border-line"
                            />
                        </a>
                        <div class="flex flex-col gap-6 lg:w-1/2">
                            <Labeled title="Summary">
                                <TextParagraph text=project.description />
                            </Labeled>
                            <Labeled title="Main Contributions">
                                <ListParagraph list=project.main_contributions />
                            </Labeled>
                        </div>
                    </div>
                </div>
                <footer class="flex flex-wrap gap-3 items-center px-4 pt-4 pb-4 mt-5 w-full border-t md:px-6 md:pb-6 border-line">
                    <span class="flex flex-wrap gap-2 items-center">{skills}</span>
                    <a
                        href=project.path()
                        class="inline-flex items-center py-1.5 px-3 ml-auto font-semibold tracking-wide uppercase link-nav text-[0.875em] text-ink bg-button"
                    >
                        "Read More"
                    </a>
                    <span class="text-ink-muted">{project.info.end_date.label()}</span>
                </footer>
            </div>
        </article>
    }
}
