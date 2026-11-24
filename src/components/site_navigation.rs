use leptos::prelude::*;

use crate::components::{Link, ProjectsMenu};
use crate::locale::resume_label;

#[component]
pub fn SiteNavigation() -> impl IntoView {
    view! {
        <nav class="flex flex-row items-center pl-3 gap-[0.75em] py-[0.25em]">
            <Link href="/" class="nav-button">
                <svg
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    role="img"
                    aria-label="Home"
                    class="w-[1.4em]"
                >
                    <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
                </svg>
            </Link>

            <ProjectsMenu />

            <Link href="/resume" class="nav-button">
                {resume_label()}
            </Link>
        </nav>
    }
}
