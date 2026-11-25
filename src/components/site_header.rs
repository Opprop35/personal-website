use leptos::prelude::*;

use crate::components::{PersonalLinks, SiteNavigation};

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <header class="flex sticky top-0 z-40 flex-row justify-between items-center w-full text-base print:hidden h-fit border-b-1 border-line bg-surface-deep/85 backdrop-blur-sm">
            <SiteNavigation />
            <PersonalLinks />
        </header>
    }
}
