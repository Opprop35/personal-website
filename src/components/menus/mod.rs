pub mod projects_menu;

use const_format::formatcp;

use crate::components::BACKGROUND_BUTTON;

const MENU: &str = "rounded-default border border-line bg-background p-1 shadow-lg";

const MENU_ITEM: &str =
    formatcp!("block w-full px-3 py-2 text-left {}", BACKGROUND_BUTTON);
