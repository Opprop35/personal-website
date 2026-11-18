use leptos::prelude::*;

use crate::components::{Link, Paragraph, Paragraphs, ProjectsMenu, TextFragment};

// TODO: Leptos and Tailwind can be skills as well.
const FOOTER: [Paragraph; 1] = [Paragraph::Text(&[
    TextFragment::Text("Website made using "),
    TextFragment::Link {
        url: "https://leptos.dev/",
        label: "Leptos",
    },
    TextFragment::Text(" & "),
    TextFragment::Link {
        url: "https://tailwindcss.com/",
        label: "Tailwind",
    },
])];

#[component]
pub fn Layout(children: impl IntoView) -> impl IntoView {
    view! {
        // TODO: split out header and footer into components
        <div class="text-xs md:text-sm lg:text-base 2xl:text-lg bg-background-light text-ink">
            <div class="flex sticky top-0 z-40 flex-row justify-between items-center w-full print:hidden h-fit border-b-1 border-line bg-background/85 backdrop-blur-sm">
                <div class="flex flex-row items-center pl-3 gap-[0.75em] py-[0.25em] justify-left">
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
                        "Resume"
                    </Link>
                </div>

                <div class="flex items-center pr-3 gap-[0.75em] py-[0.25em] justify-right">
                    <Link href="https://github.com/Opprop35" class="nav-button" external=true>
                        <svg
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            role="img"
                            aria-label="GitHub"
                            class="w-[1.4em]"
                        >
                            <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
                        </svg>
                    </Link>
                    <Link
                        href="https://www.linkedin.com/in/jesse-stolk-b2542a1b7/"
                        class="nav-button"
                        external=true
                    >
                        <svg
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            role="img"
                            aria-label="Linked In"
                            class="w-[1.4em]"
                        >
                            <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
                        </svg>
                    </Link>
                </div>
            </div>

            <div class="flex flex-col min-h-screen bg-background-light">{children}</div>

            <div class="py-3 px-4 print:hidden bg-background text-ink-muted">
                <Paragraphs paragraphs=&FOOTER class="text-center" />
            </div>
        </div>
    }
}
