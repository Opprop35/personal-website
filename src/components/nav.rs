use leptos::prelude::*;

pub const NAV_ITEM: &str = "inline-flex items-center p-2 bg-button";

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button type="button" class=NAV_ITEM data-nav-trigger="">
            {children()}
        </button>
    }
}
