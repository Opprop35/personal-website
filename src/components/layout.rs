use leptos::prelude::*;

use crate::components::{SiteFooter, SiteHeader};

#[component]
pub fn Layout(children: impl IntoView) -> impl IntoView {
    view! {
        <div class="text-xs md:text-sm lg:text-base 2xl:text-lg bg-surface text-ink">
            <SiteHeader />
            <main class="flex flex-col min-h-screen bg-surface">{children}</main>
            <SiteFooter />
        </div>
    }
}
