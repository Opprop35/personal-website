use leptos::prelude::*;

use crate::components::Labeled;
use crate::components::project::Showcase;
use crate::models;

#[component]
pub fn ProjectArticle(
    title: &'static str,
    showcase: models::Showcase,
    children: Children,
) -> impl IntoView {
    view! {
        <article class="flex flex-col gap-6 md:flex-row md:gap-8 md:items-start">
            <Labeled title=title class="w-full min-w-0 md:w-5/12">
                {children()}
            </Labeled>
            <figure class="w-full min-w-0 md:w-7/12">
                <Showcase showcase=showcase />
            </figure>
        </article>
    }
}
