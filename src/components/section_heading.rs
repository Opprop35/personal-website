use leptos::prelude::*;

/// A section heading centered between two horizontal rule dividers.
///
/// Pass additional classes (e.g. padding) for the outer container.
#[component]
pub fn SectionHeading(
    title: &'static str,
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class=format!("flex gap-4 items-center {}", class.unwrap_or_default())>
            <span class="flex-1 min-w-0 divider" />
            <h2 class="font-bold text-center uppercase whitespace-nowrap text-[1.25em] tracking-[0.2em] text-ink-muted">
                {title}
            </h2>
            <span class="flex-1 min-w-0 divider" />
        </div>
    }
}
