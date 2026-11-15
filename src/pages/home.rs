use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="mb-2 text-2xl font-semibold">"Portfolio"</h1>
            <p class="text-ink-muted">
                "This is the content area. The Home page contents will migrate here in a
                later increment."
            </p>
        </div>
    }
}
