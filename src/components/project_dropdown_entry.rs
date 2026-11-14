use leptos::prelude::*;

#[component]
pub fn ProjectDropdownEntry(text: impl Into<String>, page: impl Into<String>) -> impl IntoView {
    let text = text.into();
    let page = page.into();
    let href = format!("/{page}");

    view! {
        <a href=href>
            <li class="p-1 rounded hover:bg-gray-700">
                <p>{text}</p>
            </li>
        </a>
    }
}
