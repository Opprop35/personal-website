use leptos::prelude::*;

#[component]
pub fn TextSizedImage(image: impl Into<String>, alt: impl Into<String>) -> impl IntoView {
    let image = image.into();
    let alt = alt.into();

    view! {
        <div class="w-[1.4em]">
            <img class="w-full object-contain" src=image alt=alt />
        </div>
    }
}
