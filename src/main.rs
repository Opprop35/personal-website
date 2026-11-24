mod app;
mod assets;
mod components;
mod content;
mod locale;
mod organization;
mod pages;
mod skill;

use leptos::mount::mount_to_body;

use crate::app::install_panic_hook;

fn main() {
    install_panic_hook();
    mount_to_body(app::App);
}
