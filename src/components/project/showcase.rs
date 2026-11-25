use leptos::prelude::*;

use crate::assets::image;
use crate::models;

#[component]
pub fn Showcase(showcase: models::Showcase) -> impl IntoView {
    match showcase {
        models::Showcase::Video(code) => view! {
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
        models::Showcase::Image { file_name, alt } => view! {
            <img
                src=image(file_name)
                alt=alt
                class="object-cover w-full border border-line rounded-default"
            />
        }
        .into_any(),
    }
}
