use leptos::prelude::*;

/// Style for a link that encapsulates an image: the image smoothly scales up
/// on hover to give clear interactive feedback without disturbing the layout.
pub const IMAGE_LINK: &str =
    "transition-transform duration-150 hover:scale-110 ease-nav focus-ring";

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
