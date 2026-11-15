mod components;
mod pages;

use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::{toast::create_toast, Layout, Toasts};
use crate::pages::{home::Home, resume::Resume};

fn main() {
    install_panic_hook();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let routes = view! {
        <Router>
            <Routes fallback=move || "Page not found.">
                <Route path=path!("") view=Home />
                <Route path=path!("/resume") view=Resume />
            </Routes>
        </Router>
    };

    view! {
        <Toasts />
        <Layout children=routes />
    }
}

fn install_panic_hook() {
    set_hook(Box::new(move |info: &PanicHookInfo| {
        console_error_panic_hook::hook(info);
        let _ = catch_unwind(AssertUnwindSafe(|| create_toast(info)));
    }));
}
