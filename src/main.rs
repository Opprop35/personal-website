use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
