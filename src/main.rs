mod components;
mod pages;
mod project;
mod skill;

use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::{toast::create_toast, Layout, Toasts};
use crate::pages::{home::Home, projects, resume::Resume};

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
                <Route path=path!("/LegoFramework") view=projects::lego_framework::Page />
                <Route
                    path=path!("/DuckSideOfTheMoon")
                    view=projects::duck_side_of_the_moon::Page
                />
                <Route
                    path=path!("/DucksideOfTheMoonStudent")
                    view=projects::duckside_of_the_moon_student::Page
                />
                <Route path=path!("/AtticPanic") view=projects::attic_panic::Page />
                <Route path=path!("/Radorian") view=projects::radorian::Page />
                <Route path=path!("/LevelEditorPlugin") view=projects::level_editor_plugin::Page />
                <Route path=path!("/TwinStickDuckBox") view=projects::twin_stick_duckbox::Page />
                <Route path=path!("/FolivorasFall") view=projects::folivoras_fall::Page />
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
