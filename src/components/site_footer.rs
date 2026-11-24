use leptos::prelude::*;

use crate::components::{Paragraph, Paragraphs, TextFragment, TextParagraph};

// TODO: Leptos and Tailwind can be skills as well.
const FOOTER: [Paragraph; 1] = [
    Paragraph::Text(TextParagraph {
        fragments: &[
            TextFragment::Text("Website made using "),
            TextFragment::Link {
                url: "https://leptos.dev/",
                label: "Leptos",
            },
            TextFragment::Text(" & "),
            TextFragment::Link {
                url: "https://tailwindcss.com/",
                label: "Tailwind",
            },
        ],
    }),
];

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="py-3 px-4 print:hidden bg-surface-deep text-ink-muted">
            <Paragraphs paragraphs=&FOOTER class="text-center" />
        </footer>
    }
}
