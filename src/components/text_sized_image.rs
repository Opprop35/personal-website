use leptos::prelude::*;

#[component]
pub fn TextSizedImage(image: impl Into<String>, alt: impl Into<String>) -> impl IntoView {
    let image = image.into();
    let alt = alt.into();

    view! {
        <div class="w-[1rem] md:w-[1.25rem] lg:w-[1.5rem]">
            <img class="w-full object-fit" src=image alt=alt />
        </div>
    }
}
