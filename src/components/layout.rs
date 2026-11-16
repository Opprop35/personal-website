use leptos::prelude::*;

use crate::components::{NavLink, ProjectsMenu};

#[component]
pub fn Layout(children: impl IntoView) -> impl IntoView {
    view! {
        // TODO: split out header and footer into components
        <div class="text-xs md:text-sm lg:text-base 2xl:text-lg bg-background-light text-ink">
            <div class="flex sticky top-0 z-40 flex-row justify-between items-center w-full print:hidden h-fit border-b-1 border-line bg-background/85 backdrop-blur-sm">
                <div class="flex flex-row gap-3 items-center py-1 pl-3 justify-left">
                    <NavLink href="/">
                        <svg
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            role="img"
                            aria-label="Home"
                            class="w-[1rem] md:w-[1.25rem] lg:w-[1.5rem]"
                        >
                            <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
                        </svg>
                    </NavLink>

                    <ProjectsMenu />

                    <NavLink href="/resume">"Resume"</NavLink>
                </div>

                <div class="flex gap-3 items-center py-1 pr-3 justify-right">
                    <NavLink
                        external=true
                        href="https://www.linkedin.com/in/jesse-stolk-b2542a1b7/"
                    >
                        <svg
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            role="img"
                            aria-label="Linked In"
                            class="w-[1rem] md:w-[1.25rem] lg:w-[1.5rem]"
                        >
                            <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
                        </svg>
                    </NavLink>
                </div>
            </div>

            <div class="flex flex-col min-h-screen bg-background-light">{children}</div>

            <div class="py-3 px-4 print:hidden bg-background text-ink-muted">
                <p class="text-center">
                    "Website made using "
                    <a
                        href="https://leptos.dev/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="hover:underline text-background-hover-ink"
                    >
                        "Leptos"
                    </a> " & "
                    <a
                        href="https://tailwindcss.com/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="hover:underline text-background-hover-ink"
                    >
                        "Tailwind"
                    </a>
                </p>
            </div>
        </div>
    }
}
