use std::panic::{catch_unwind, set_hook, AssertUnwindSafe, PanicHookInfo};

use leptos::{prelude::*, tachys::view::iterators::StaticVec};
use leptos_router::{components::*, hooks::use_location, path, MatchNestedRoutes, StaticSegment};

use crate::{
    components::{toast::create_toast, Layout, Toasts},
    content::projects::ProjectId,
    pages::{home, not_found, princess, project, resume},
    viewport,
};

#[component]
pub fn App() -> impl IntoView {
    let routes = view! {
        <Router>
            <SiteRoutes />
        </Router>
    };

    view! {
        // TODO: Should Toasts live here inside the body?
        <Toasts />
        <Layout children=routes />
    }
}

#[component]
fn SiteRoutes() -> impl IntoView {
    let location = use_location();
    ImmediateEffect::new_scoped(move || {
        viewport::apply_viewport(&location.pathname.get());
    });

    view! {
        <Routes fallback=move || view! { <not_found::Page /> }>
            <Route path=path!("") view=home::Page />
            <Route path=path!("/resume") view=resume::Page />
            <Route path=path!("/princess") view=princess::Page />
            <ProjectRoutes />
        </Routes>
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
