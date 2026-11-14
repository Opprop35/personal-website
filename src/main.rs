mod components;

use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::prelude::*;

use crate::components::{toast::create_toast, Layout, Toasts};

fn main() {
    install_panic_hook();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let body = view! {
        <div class="p-8">
            <h1 class="mb-2 text-2xl font-semibold">"Portfolio"</h1>
            <p class="text-ink-muted">
                "This is the content area. The Home page contents will migrate here in a
                later increment."
            </p>
        </div>
    };

    view! {
        <Toasts />
        <Layout children=body />
    }
}

fn install_panic_hook() {
    set_hook(Box::new(move |info: &PanicHookInfo| {
        console_error_panic_hook::hook(info);
        let _ = catch_unwind(AssertUnwindSafe(|| create_toast(info)));
    }));
}
