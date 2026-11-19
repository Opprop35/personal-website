mod components;
mod locale;
mod organization;
mod pages;
mod project;
mod skill;

use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::{toast::create_toast, Layout, Toasts};
use crate::pages::{home, not_found, princess, projects, resume};

const IMAGE_DIR: &str = "public/images";

fn main() {
    install_panic_hook();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let routes = view! {
        <Router>
            <Routes fallback=move || view! { <not_found::Page /> }>
                <Route path=path!("") view=home::Page />
                <Route path=path!("/resume") view=resume::Page />
                <Route path=path!("/lego-framework") view=projects::lego_framework::Page />
                <Route
                    path=path!("/duck-side-of-the-moon")
                    view=projects::duck_side_of_the_moon::Page
                />
                <Route
                    path=path!("/duckside-of-the-moon-student")
                    view=projects::duckside_of_the_moon_student::Page
                />
                <Route path=path!("/attic-panic") view=projects::attic_panic::Page />
                <Route path=path!("/radorian") view=projects::radorian::Page />
                <Route path=path!("/level-editor-plugin") view=projects::level_editor_plugin::Page />
                <Route path=path!("/twin-stick-duckbox") view=projects::twin_stick_duckbox::Page />
                <Route path=path!("/folivoras-fall") view=projects::folivoras_fall::Page />
                <Route path=path!("/princess") view=princess::Page />
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
