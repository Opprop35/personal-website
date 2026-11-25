use leptos::prelude::*;

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button type="button" class="nav-button" data-nav-trigger="">
            {children()}
        </button>
    }
}
