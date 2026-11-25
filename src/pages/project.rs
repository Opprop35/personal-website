use leptos::prelude::*;

use crate::assets::image;
use crate::components::{Labeled, Link, ProjectChapter, ProjectInfo, TextParagraph};
use crate::models;

#[component]
pub fn Page(project: &'static models::Project) -> impl IntoView {
    let title = project.title;
    document().set_title(&format!("Jesse Stolk - {}", title));

    let background_class = if project.background.is_some() {
        "bg-fixed bg-cover bg-center bg-no-repeat"
    } else {
        ""
    };
    let background_style = project
        .background
        .map(|src| format!("background-image:url({})", image(src)))
        .unwrap_or_default();

    let hero_video = project.hero_video.map(|code| {
        view! {
            <div class="relative aspect-video">
                <iframe
                    class="absolute inset-0 w-full h-full ring-1 ring-line rounded-default"
                    src=format!("https://www.youtube-nocookie.com/embed/{code}?rel=0")
                    allow="autoplay"
                    title=format!("Hero video for {title}")
                    tabindex="-1"
                ></iframe>
            </div>
        }
    });

    let badges = project
        .badges
        .iter()
        .map(|badge| {
            view! {
                <Link href=badge.url class="image-link" external=true>
                    <img src=image(badge.image) alt=badge.alt class="w-auto h-12 md:h-16" />
                </Link>
            }
        })
        .collect_view();

    let chapters = project
        .chapters
        .iter()
        .map(|chapter| view! { <ProjectChapter chapter=chapter /> })
        .collect_view();

    view! {
        <div class=background_class style=background_style>
            <div class="flex flex-col py-10 px-4 mx-auto w-full max-w-6xl md:py-14 md:px-8">
                <span class="self-center py-2.5 px-4 max-w-full font-semibold tracking-wide text-center border-t border-x border-line bg-surface-raised text-ink rounded-t-nav">
                    {title}
                </span>
                <article class="flex flex-col gap-8 p-4 bg-gradient-to-b border md:p-8 border-line from-surface-deep/95 to-surface-deep rounded-b-nav">
                    {hero_video}
                    <div class="flex flex-wrap gap-4 justify-center items-center">{badges}</div>
                    <div class="divider" />
                    <div class="flex flex-col gap-8 lg:flex-row lg:items-start">
                        <ProjectInfo info=&project.info />
                        <Labeled title="Description" class="w-full min-w-0 lg:w-7/12">
                            <TextParagraph
                                text=project.description
                                class="space-y-4 leading-relaxed text-ink"
                            />
                        </Labeled>
                    </div> <div class="flex flex-col gap-10">{chapters}</div>
                </article>
            </div>
        </div>
    }
}
