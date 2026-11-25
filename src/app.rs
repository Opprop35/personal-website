use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::{prelude::*, tachys::view::iterators::StaticVec};
use leptos_router::{components::*, path, MatchNestedRoutes, StaticSegment};

use crate::{
    components::{toast::create_toast, Layout, Toasts},
    content::projects::ProjectId,
    pages::{home, not_found, princess, project, resume},
};

#[component]
pub fn App() -> impl IntoView {
    let routes = view! {
        <Router>
            <Routes fallback=move || view! { <not_found::Page /> }>
                <Route path=path!("") view=home::Page />
                <Route path=path!("/resume") view=resume::Page />
                <Route path=path!("/princess") view=princess::Page />
                <ProjectRoutes />
            </Routes>
        </Router>
    };

    view! {
        <Layout children=routes />
        <Toasts />
    }
}

#[component(transparent)]
fn ProjectRoutes() -> impl MatchNestedRoutes + Clone {
    let routes: Vec<_> = ProjectId::ALL
        .iter()
        .map(|id| {
            let project = id.project();
            Route(RouteProps {
                path: StaticSegment(project.slug),
                view: move || project::Page(project::PageProps { project }),
                ssr: Default::default(),
            })
        })
        .collect();

    let routes: StaticVec<_> = routes.into();
    routes
}

pub fn install_panic_hook() {
    set_hook(Box::new(move |info: &PanicHookInfo| {
        console_error_panic_hook::hook(info);
        let _ = catch_unwind(AssertUnwindSafe(|| create_toast(info)));
    }));
}
