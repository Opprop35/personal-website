mod components;

use components::Layout;
use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let body = view! {
        <div class="p-8">
            <h1 class="mb-2 text-2xl font-semibold">"Portfolio"</h1>
            <p class="text-neutral-400">
                "This is the content area. The Home page contents will migrate here in a
                later increment."
            </p>
        </div>
    };

    view! { <Layout children=body /> }
}
