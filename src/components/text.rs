use leptos::prelude::*;

use crate::components::fragments::{TextFragments, TextParagraph};

/// A paragraph of rich text.
#[component]
pub fn Text(
    text: TextParagraph,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let TextParagraph { fragments } = text;
    let classes = class.unwrap_or("").to_string();
    view! {
        <p class=classes>
            <TextFragments fragments=fragments />
        </p>
    }
}
