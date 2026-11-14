use leptos::prelude::*;

use crate::components::{
    menus::{MENU, MENU_ITEM},
    NavButton,
};

#[component]
pub fn ProjectsMenu() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div
            class="relative"
            on:mouseenter=move |_| open.set(true)
            on:mouseleave=move |_| open.set(false)
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
                        "absolute left-0 top-full z-50 min-w-56 transition-opacity duration-200 {MENU} {state}",
                    )
                }
                role="menu"
            >
                <a class=MENU_ITEM role="menuitem" href="/LegoFramework">
                    "LEGO Framework"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/DuckSideOfTheMoon">
                    "Duck Side of the Moon"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/AtticPanic">
                    "Attic Panic"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/Radorian">
                    "Radorian"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/DucksideOfTheMoonStudent">
                    "Duckside of the Moon (Student)"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/LevelEditorPlugin">
                    "Level Editor Plugin"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/TwinStickDuckBox">
                    "Twin-Stick Duckbox"
                </a>
                <a class=MENU_ITEM role="menuitem" href="/FolivorasFall">
                    "Folivora's Fall"
                </a>
            </div>
        </div>
    }
}
