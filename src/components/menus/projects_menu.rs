use leptos::prelude::*;

use crate::components::menus::Menu;

#[component]
pub fn ProjectsMenu() -> impl IntoView {
    view! {
        <Menu label="Projects">
            <a class="menu-button" role="menuitem" href="/lego-framework">
                "LEGO Framework"
            </a>
            <a class="menu-button" role="menuitem" href="/duck-side-of-the-moon">
                "Duck Side of the Moon"
            </a>
            <a class="menu-button" role="menuitem" href="/attic-panic">
                "Attic Panic"
            </a>
            <a class="menu-button" role="menuitem" href="/radorian">
                "Radorian"
            </a>
            <a class="menu-button" role="menuitem" href="/duckside-of-the-moon-student">
                "Duckside of the Moon (Student)"
            </a>
            <a class="menu-button" role="menuitem" href="/level-editor-plugin">
                "Level Editor Plugin"
            </a>
            <a class="menu-button" role="menuitem" href="/twin-stick-duckbox">
                "Twin-Stick Duckbox"
            </a>
            <a class="menu-button" role="menuitem" href="/folivoras-fall">
                "Folivora's Fall"
            </a>
        </Menu>
    }
}
