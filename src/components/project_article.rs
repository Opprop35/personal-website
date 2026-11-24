use leptos::prelude::*;

use crate::components::{Labeled, ShowcaseView};
use crate::content::project::Showcase;

#[component]
pub fn ProjectArticle(
    title: &'static str,
    showcase: Option<Showcase>,
    children: Children,
) -> impl IntoView {
    let showcase = showcase.map(|showcase| {
        view! {
            <ShowcaseView showcase=showcase />
        }
        .into_any()
    });
    view! {
        <article class="flex flex-col gap-6 md:flex-row md:gap-8 md:items-start">
            <Labeled title=title class="w-full min-w-0 md:w-5/12">
                {children()}
            </Labeled>
            <figure class="w-full min-w-0 md:w-7/12">{showcase}</figure>
        </article>
    }
}
