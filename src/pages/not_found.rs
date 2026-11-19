use leptos::prelude::*;

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - Page Not Found");

    view! {
        <div class="flex flex-col gap-6 justify-center items-center py-32 px-6 text-center">
            <h1 class="font-bold tracking-tight text-[2.25em] text-ink">"Page Not Found"</h1>
            <p class="max-w-md leading-relaxed text-ink-muted">
                "Sorry, but the princess is in another URL!"
            </p>
        </div>
    }
}
