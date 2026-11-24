use leptos::prelude::*;

use crate::components::{Paragraph, Paragraphs, TextFragment, TextParagraph};
use crate::skill::Skill;

const FOOTER: [Paragraph; 1] = [
    Paragraph::Text(TextParagraph {
        fragments: &[
            TextFragment::Text("Website made using "),
            TextFragment::skill(Skill::Leptos),
            TextFragment::Text(" & "),
            TextFragment::skill(Skill::Tailwind),
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
