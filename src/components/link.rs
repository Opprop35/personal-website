use leptos::prelude::*;

#[component]
pub fn Link(
    href: &'static str,
    class: &'static str,
    #[prop(optional)] external: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            href=href
            target=move || external.then_some("_blank")
            rel=move || external.then_some("noopener noreferrer")
            class=class
        >
            {children()}
        </a>
    }
}
