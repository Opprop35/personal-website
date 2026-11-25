use leptos::prelude::*;

use crate::models;

#[component]
pub fn SmallProjectCard(project: &'static models::Project) -> impl IntoView {
    view! {
        <a
            href=project.path()
            class="flex flex-wrap gap-y-1 gap-x-3 items-baseline py-3 px-4 w-full border transition-colors duration-150 focus-ring border-line bg-surface-deep hover:border-ink-muted/50"
        >
            <span class="font-semibold text-ink">{project.title}</span>
            <span class="text-ink-muted">{project.info.project_type.label()}</span>
            <span class="ml-auto text-ink-muted">{project.info.end_date.label()}</span>
        </a>
    }
}
