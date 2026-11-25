use leptos::prelude::*;

use crate::components::text_fragments::TextFragments;
use crate::models;

#[component]
pub fn TextParagraph(
    text: models::TextParagraph,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = class.unwrap_or("").to_string();
    view! {
        <p class=classes>
            <TextFragments fragments=text.fragments />
        </p>
    }
}

#[component]
pub fn ListParagraph(list: models::ListParagraph) -> impl IntoView {
    view! {
        <ul class="pl-5 list-disc marker:text-ink-muted">
            {list.items.iter().copied().map(|item| view! { <li>{item}</li> }).collect_view()}
        </ul>
    }
}

#[component]
pub fn Paragraphs(
    paragraphs: &'static [models::Paragraph],
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = class.unwrap_or("").to_string();
    let blocks = paragraphs
        .iter()
        .map(|paragraph| match paragraph {
            models::Paragraph::Text(text) => {
                let text = *text;
                view! { <TextParagraph text=text /> }.into_any()
            }
            models::Paragraph::List(list) => {
                let list = *list;
                view! { <ListParagraph list=list /> }.into_any()
            }
        })
        .collect_view();
    view! { <div class=classes>{blocks}</div> }
}
