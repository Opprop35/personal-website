use leptos::prelude::*;

use crate::components::Paragraphs;
use crate::models;

const FOOTER: [models::Paragraph; 1] = [models::Paragraph::Text(models::TextParagraph {
    fragments: &[
        models::TextFragment::Text("Website made using "),
        models::TextFragment::skill(models::Skill::Leptos),
        models::TextFragment::Text(" & "),
        models::TextFragment::skill(models::Skill::Tailwind),
    ],
})];

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="py-3 px-4 print:hidden bg-surface-deep text-ink-muted">
            <Paragraphs paragraphs=&FOOTER class="text-center" />
        </footer>
    }
}
