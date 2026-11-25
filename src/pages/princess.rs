use crate::assets::image;
use leptos::prelude::*;

#[component]
pub fn Page() -> impl IntoView {
    document().set_title("Jesse Stolk - The Princess");

    view! {
        <div class="flex flex-col gap-8 justify-center items-center py-32 px-6 text-center">
            <img
                src=image("princess.svg")
                alt="The princess"
                class="h-[12em] w-auto"
            />
            <h1 class="font-bold tracking-tight text-[2.25em] text-ink">
                "You found the princess!"
            </h1>
        </div>
    }
}
