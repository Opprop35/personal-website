use leptos::prelude::*;

use crate::components::{Labeled, Link, ProjectChapter, ProjectInfo, Text};
use crate::project::Project;
use crate::IMAGE_DIR;

pub mod attic_panic;
pub mod duck_side_of_the_moon;
pub mod duckside_of_the_moon_student;
pub mod folivoras_fall;
pub mod lego_framework;
pub mod level_editor_plugin;
pub mod radorian;
pub mod twin_stick_duckbox;

#[component]
pub fn ProjectPage(data: &'static Project) -> impl IntoView {
    let title = data.title;

    let background_class = if data.background.is_some() {
        "bg-fixed bg-cover bg-center bg-no-repeat"
    } else {
        ""
    };
    let background_style = data
        .background
        .map(|src| format!("background-image:url({IMAGE_DIR}/{src})"))
        .unwrap_or_default();

    let hero_video = data.hero_video.map(|code| {
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

    let badges = data
        .badges
        .iter()
        .map(|badge| {
            view! {
                <Link href=badge.url class="image-link" external=true>
                    <img
                        src=format!("{IMAGE_DIR}/{}", badge.image)
                        alt=badge.alt
                        class="w-auto h-12 md:h-16"
                    />
                </Link>
            }
        })
        .collect_view();

    let chapters = data
        .chapters
        .iter()
        .map(|chapter| view! { <ProjectChapter chapter=chapter /> })
        .collect_view();

    let under_construction = data.under_construction.then(|| {
        view! {
            <div class="flex gap-4 justify-center items-center py-4">
                <img src="public/images/wrench.svg" alt="Wrench" class="w-8 h-8 shrink-0" />
                <span class="font-bold uppercase tracking-[0.2em] text-ink-muted">
                    "Under Construction"
                </span>
                <img src="public/images/hammer.svg" alt="Hammer" class="w-8 h-8 shrink-0" />
            </div>
        }
    });

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
                        <ProjectInfo info=&data.info />
                        <Labeled title="Description" class="w-full min-w-0 lg:w-7/12">
                            <Text
                                text=data.description
                                class="space-y-4 leading-relaxed text-ink"
                            />
                        </Labeled>
                    </div> <div class="flex flex-col gap-10">{chapters}{under_construction}</div>
                </article>
            </div>
        </div>
    }
}
