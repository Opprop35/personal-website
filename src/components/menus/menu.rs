use leptos::prelude::*;

use crate::components::NavButton;

const MENU: &str = "rounded-default border border-line bg-surface-deep p-1 shadow-lg";

#[component]
pub fn Menu(label: &'static str, children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    let wrapper = NodeRef::new();

    view! {
        <div
            class="relative"
            node_ref=wrapper
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
            on:focusin=move |_| open.set(true)
            on:focusout=move |_| open.set(false)
        >
            <NavButton>{label}</NavButton>
            <div
                class=move || {
                    let state = if open.get() {
                        "visible opacity-100"
                    } else {
                        "invisible opacity-0"
                    };
                    format!(
                        "absolute left-0 top-full z-50 min-w-56 transition-[opacity,visibility] duration-200 {MENU} {state}"
                    )
                }
                role="menu"
            >
                {children()}
            </div>
        </div>
    }
}
