use leptos::prelude::*;

use crate::components::menus::Menu;

#[component]
pub fn ProjectsMenu() -> impl IntoView {
    view! {
        <Menu label="Projects">
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
            <a class="menu-button" role="menuitem" href="/FolivoraFall">
                "Folivora's Fall"
            </a>
        </Menu>
    }
}
