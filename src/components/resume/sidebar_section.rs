use leptos::prelude::*;

#[component]
pub fn SidebarSection(title: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <section class="last:mb-0 mb-[7mm]">
            <h2 class="font-semibold uppercase mb-[3mm] text-[11px] tracking-[0.16em] text-ink-muted">
                {title}
            </h2>
            <ul class="leading-snug space-y-[2.5mm] text-[13px] text-ink">{children}</ul>
        </section>
    }
}
