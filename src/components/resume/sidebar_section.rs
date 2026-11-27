use leptos::prelude::*;

#[component]
pub fn SidebarSection(
    title: &'static str,
    #[prop(optional)] inline: bool,
    children: impl IntoView,
) -> impl IntoView {
    let list_class = if inline {
        "flex flex-wrap items-center gap-x-[3mm] gap-y-[2mm] text-[13px] text-ink"
    } else {
        "leading-snug space-y-[2.5mm] text-[13px] text-ink"
    };

    view! {
        <section class="last:mb-0 mb-[7mm]">
            <h2 class="font-semibold uppercase mb-[3mm] text-[11px] tracking-[0.16em] text-ink-muted">
                {title}
            </h2>
            <ul class=list_class>{children}</ul>
        </section>
    }
}
