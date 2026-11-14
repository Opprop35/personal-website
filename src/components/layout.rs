use leptos::prelude::*;

use crate::components::{NavLink, ProjectsMenu, TextSizedImage};

#[component]
pub fn Layout(children: impl IntoView) -> impl IntoView {
    view! {
        // TODO: split out header and footer into components
        <div class="text-xs md:text-sm lg:text-base 2xl:text-lg bg-surface text-ink">
            <div class="sticky top-0 z-40 flex h-fit flex-row items-center justify-between border-b-2 border-line bg-nav w-full">
                <div class="flex flex-row gap-3 items-center py-1 pl-3 justify-left">
                    <NavLink href="/">
                        <TextSizedImage image="public/images/home.svg" alt="Home" />
                    </NavLink>

                    <ProjectsMenu />

                    <NavLink
                        external=true
                        href="public/Resume_JesseStolk_SoftwareEngineer.pdf"
                    >
                        "Resume"
                    </NavLink>
                </div>

                <div class="flex gap-3 items-center py-1 pr-3 justify-right">
                    <NavLink external=true href="https://www.linkedin.com/in/jesse-stolk-b2542a1b7/">
                        <TextSizedImage image="public/images/linkedin.svg" alt="Linked In" />
                    </NavLink>
                </div>
            </div>

            <div class="flex-col mb-auto min-h-screen bg-surface">{children}</div>

            <div class="py-3 px-4 bg-nav text-ink-muted">
                <p class="text-center">"Website made using Leptos & Tailwind"</p>
            </div>
        </div>
    }
}
