use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::{menus::MENU, NavButton};

#[component]
pub fn ProjectsMenu() -> impl IntoView {
    let open = RwSignal::new(false);
    let wrapper = NodeRef::new();

    view! {
        <div
            class="relative"
            node_ref=wrapper
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
            on:focusin=move |_| open.set(true)
            on:focusout=move |ev: web_sys::FocusEvent| {
                let el = wrapper.get();
                let focus_inside = ev
                    .related_target()
                    .as_ref()
                    .and_then(|related| related.dyn_ref::<web_sys::Node>())
                    .and_then(|node| {
                        el.as_ref()
                            .map(|el| el.unchecked_ref::<web_sys::Node>())
                            .filter(|el| el.contains(Some(node)))
                    })
                    .is_some();
                if !focus_inside {
                    open.set(false);
                }
            }
        >
            <NavButton>"Projects"</NavButton>
            <div
                class=move || {
                    let state = if open.get() {
                        "visible opacity-100"
                    } else {
                        "invisible opacity-0"
                    };
                    format!(
                        "absolute left-0 top-full z-50 min-w-56 transition-[opacity,visibility] duration-200 {MENU} {state}",
                    )
                }
                role="menu"
            >
                <a class="menu-button" role="menuitem" href="/LegoFramework">
                    "LEGO Framework"
                </a>
                <a class="menu-button" role="menuitem" href="/DuckSideOfTheMoon">
                    "Duck Side of the Moon"
                </a>
                <a class="menu-button" role="menuitem" href="/AtticPanic">
                    "Attic Panic"
                </a>
                <a class="menu-button" role="menuitem" href="/Radorian">
                    "Radorian"
                </a>
                <a class="menu-button" role="menuitem" href="/DucksideOfTheMoonStudent">
                    "Duckside of the Moon (Student)"
                </a>
                <a class="menu-button" role="menuitem" href="/LevelEditorPlugin">
                    "Level Editor Plugin"
                </a>
                <a class="menu-button" role="menuitem" href="/TwinStickDuckBox">
                    "Twin-Stick Duckbox"
                </a>
                <a class="menu-button" role="menuitem" href="/FolivorasFall">
                    "Folivora's Fall"
                </a>
            </div>
        </div>
    }
}
