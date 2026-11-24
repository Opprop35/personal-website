use leptos::prelude::*;

use crate::components::Link;
use crate::project::Project;

#[component]
pub fn OlderProjectRow(project: &'static Project) -> impl IntoView {
    view! {
        <Link
            href=project.href
            class="flex flex-wrap gap-y-1 gap-x-3 items-baseline py-3 px-4 w-full border transition-colors duration-150 focus-ring border-line bg-surface-deep hover:border-ink-muted/50"
        >
            <span class="font-semibold text-ink">{project.title}</span>
            <span class="text-ink-muted">{project.info.project_type.label()}</span>
            <span class="ml-auto text-ink-muted">{project.info.end_date.label()}</span>
        </Link>
    }
}
