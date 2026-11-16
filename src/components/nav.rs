use leptos::prelude::*;

pub const NAV_ITEM: &str =
    "inline-flex items-center rounded-nav p-2 transition-colors duration-150 hover:bg-background-hover hover:text-background-hover-ink";

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button type="button" class=NAV_ITEM data-nav-trigger="">
            {children()}
        </button>
    }
}
