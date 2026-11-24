mod components;
mod locale;
mod organization;
mod pages;
mod project;
mod skill;

use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::{
    components::{toast::create_toast, Layout, Toasts},
    pages::{
        home, not_found, princess,
        projects::{
            attic_panic, duck_side_of_the_moon, duckside_of_the_moon_student, folivoras_fall,
            lego_framework, level_editor_plugin, radorian, twin_stick_duckbox,
        },
        resume,
    },
};

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
                <Route path=path!("/lego-framework") view=lego_framework::Page />
                <Route path=path!("/duck-side-of-the-moon") view=duck_side_of_the_moon::Page />
                <Route
                    path=path!("/duckside-of-the-moon-student")
                    view=duckside_of_the_moon_student::Page
                />
                <Route path=path!("/attic-panic") view=attic_panic::Page />
                <Route path=path!("/radorian") view=radorian::Page />
                <Route path=path!("/level-editor-plugin") view=level_editor_plugin::Page />
                <Route path=path!("/twin-stick-duckbox") view=twin_stick_duckbox::Page />
                <Route path=path!("/folivoras-fall") view=folivoras_fall::Page />
                <Route path=path!("/princess") view=princess::Page />
            </Routes>
        </Router>
    };

    view! {
        // TODO: Should Toasts live here inside the body?
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
