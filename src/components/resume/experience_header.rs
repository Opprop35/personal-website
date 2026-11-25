use leptos::prelude::*;

use crate::components::Link;
use crate::models;

#[component]
pub fn ExperienceHeader(source: models::ExperienceSource) -> AnyView {
    let name = source.name();
    if let Some(url) = source.url() {
        return view! {
            <Link href=url class="hover:underline focus-ring" external=true>
                {name}
            </Link>
        }
        .into_any();
    }
    view! { <span>{name}</span> }.into_any()
}
