use leptos::prelude::*;

/// A block of content labeled with a muted uppercase heading.
#[component]
pub fn Labeled(
    title: &'static str,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let label = view! { <h3 class="mb-2 label-uppercase">{title}</h3> };

    view! {
        <section class=class>{label}{children()}</section>
    }
}
