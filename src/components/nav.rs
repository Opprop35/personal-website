use leptos::prelude::*;

const NAV_ITEM: &str =
    "inline-flex items-center rounded-nav p-2 transition-colors duration-150 hover:bg-background-hover hover:text-background-hover-ink";

#[component]
pub fn NavLink(
    href: &'static str,
    #[prop(optional)] external: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            href=href
            target=move || external.then_some("_blank")
            rel=move || external.then_some("noopener noreferrer")
            class=NAV_ITEM
        >
            {children()}
        </a>
    }
}

#[component]
pub fn NavButton(children: Children) -> impl IntoView {
    view! {
        <button type="button" class=NAV_ITEM data-nav-trigger="">
            {children()}
        </button>
    }
}
