use leptos::prelude::*;

use const_format::formatcp;

use crate::components::BACKGROUND_BUTTON;

pub const NAV_ITEM: &str = formatcp!("inline-flex items-center p-2 {}", BACKGROUND_BUTTON);

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button type="button" class=NAV_ITEM data-nav-trigger="">
            {children()}
        </button>
    }
}
