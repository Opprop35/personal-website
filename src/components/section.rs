use leptos::prelude::*;

#[component]
pub fn Section(
    title: &'static str,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] open: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(open);

    let chevron = if collapsible {
        Some(
            view! {
                <button
                    type="button"
                    tabindex="-1"
                    aria-hidden="true"
                    on:click=move |_| open.set(!open.get())
                    class="p-1 -m-1 cursor-pointer"
                >
                    <svg
                        viewBox="0 0 16 16"
                        class=move || {
                            format!(
                                "h-4 w-4 shrink-0 transition-[transform] duration-300 ease-in-out [transform-origin:center] {}",
                                if open.get() {
                                    "[transform:rotate(90deg)]"
                                } else {
                                    "[transform:rotate(0deg)]"
                                },
                            )
                        }
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <path d="M6 4l4 4-4 4" />
                    </svg>
                </button>
            }
            .into_any(),
        )
    } else {
        None
    };

    let heading = view! {
        <span class="font-bold uppercase whitespace-nowrap text-[1.25em] tracking-[0.2em]">
            {title}
        </span>
    };

    let heading = if collapsible {
        view! {
            <button
                type="button"
                aria-expanded=move || open.get().to_string()
                on:click=move |_| open.set(!open.get())
                class="py-1 px-2 cursor-pointer select-none focus-ring"
            >
                {heading}
            </button>
        }
        .into_any()
    } else {
        heading.into_any()
    };

    let line = view! {
        <span class="block grid gap-4 items-center w-full transition-colors duration-300 grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] text-ink-muted ease-nav has-[button:hover]:text-ink">
            <span class="flex gap-4 items-center min-w-0">
                <span class="flex-1 min-w-0 divider" />
                {chevron}
            </span>
            {heading}
            <span class="min-w-0 divider" />
        </span>
    };

    let body = children.map(|children|
        {
            let padded = view! { <div class="pt-10">{children()}</div> }.into_any();
            if collapsible {
                view! {
                    <div class=move || {
                        format!(
                            "grid transition-[grid-template-rows] duration-400 ease-nav {}",
                            if open.get() { "grid-rows-[1fr]" } else { "grid-rows-[0fr]" },
                        )
                    }>
                        <div class=move || {
                            format!(
                                "overflow-hidden transition-[opacity,visibility] duration-400 ease-nav {}",
                                if open.get() {
                                    "opacity-100 visible"
                                } else {
                                    "opacity-0 invisible"
                                },
                            )
                        }>{padded}</div>
                    </div>
                }
                    .into_any()
            } else {
                padded
            }
        });

    view! { <section class="flex flex-col">{line} {body}</section> }
}
