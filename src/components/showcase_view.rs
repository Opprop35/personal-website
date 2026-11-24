use leptos::prelude::*;

use crate::project::Showcase;
use crate::IMAGE_DIR;

#[component]
pub fn ShowcaseView(showcase: Showcase) -> impl IntoView {
    match showcase {
        Showcase::Video(code) => view! {
            <div class="relative aspect-video">
                <iframe
                    class="absolute inset-0 w-full h-full ring-1 ring-line rounded-default"
                    src=format!("https://www.youtube-nocookie.com/embed/{code}?rel=0")
                    allow="autoplay"
                    title="Project video"
                ></iframe>
            </div>
        }
        .into_any(),
        Showcase::Image { src, alt } => view! {
            <img
                src=format!("{IMAGE_DIR}/{src}")
                alt=alt
                class="object-cover w-full border border-line rounded-default"
            />
        }
        .into_any(),
    }
}
