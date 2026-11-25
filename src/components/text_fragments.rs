use leptos::prelude::*;

use crate::components::Link;
use crate::models;

#[component]
pub fn TextFragments(fragments: &'static [models::TextFragment]) -> impl IntoView {
    fragments
        .iter()
        .map(|fragment| match fragment {
            models::TextFragment::Text(text) => {
                let text = *text;
                view! { <span>{text}</span> }.into_any()
            }
            models::TextFragment::StrongText(text) => {
                let text = *text;
                view! { <strong class="font-bold text-ink">{text}</strong> }.into_any()
            }
            models::TextFragment::MutedText(text) => {
                let text = *text;
                view! { <span class="text-ink-muted">{text}</span> }.into_any()
            }
            models::TextFragment::Link { url, label } => {
                let url = *url;
                let label = *label;
                view! {
                    <Link href=url class="text-link" external=true>
                        {label}
                    </Link>
                }
                .into_any()
            }
        })
        .collect_view()
}
