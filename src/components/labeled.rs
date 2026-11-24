use leptos::prelude::*;

/// The HTML element a [`Labeled`] block renders as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabeledElement {
    /// A thematic grouping of content, e.g. the "Summary" block on a project card.
    Section,
    /// A self-contained piece of content, e.g. a feature block on a project page.
    Article,
}

/// A block of content labeled with a muted uppercase heading.
#[component]
pub fn Labeled(
    title: &'static str,
    element: LabeledElement,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let label = view! { <h3 class="mb-2 label-uppercase">{title}</h3> };

    match element {
        LabeledElement::Section => view! {
            <section class=class>{label}{children()}</section>
        }
        .into_any(),
        LabeledElement::Article => view! {
            <article class=class>{label}{children()}</article>
        }
        .into_any(),
    }
}
