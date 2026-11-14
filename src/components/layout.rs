use crate::components::{ProjectDropdownEntry, TextSizedImage};
use leptos::prelude::*;

#[component]
pub fn Layout(children: impl IntoView) -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = signal(false);

    view! {
        <div class="text-xs md:text-sm lg:text-base 2xl:text-lg bg-neutral-900 text-slate-50">
            <div class="flex sticky top-0 z-40 flex-row justify-between items-center w-full bg-black border-b-2 h-fit border-neutral-700">
                <div class="flex flex-row gap-3 items-center py-1 pl-3 justify-left">
                    <a class="p-2 hover:rounded-full hover:bg-neutral-800" href="/">
                        <TextSizedImage image="public/images/home.svg" alt="Home" />
                    </a>

                    <button
                        type="button"
                        class="relative p-2 rounded-lg hover:bg-neutral-800"
                        on:click=move |_| set_dropdown_open.update(|open| *open = !*open)
                    >
                        <span>"Projects"</span>
                        <div
                            class="absolute left-0 mt-2 w-max text-left bg-gray-800 rounded-md border-2 border-gray-600 top-max"
                            class:visible=move || dropdown_open.get()
                            class:hidden=move || !dropdown_open.get()
                        >
                            <ul class="p-1">
                                <ProjectDropdownEntry text="LEGO Framework" page="LegoFramework" />
                                <ProjectDropdownEntry
                                    text="Duck Side of the Moon"
                                    page="DuckSideOfTheMoon"
                                />
                                <ProjectDropdownEntry text="Attic Panic" page="AtticPanic" />
                                <ProjectDropdownEntry text="Radorian" page="Radorian" />
                                <ProjectDropdownEntry
                                    text="Duckside of the Moon (Student)"
                                    page="DucksideOfTheMoonStudent"
                                />
                                <ProjectDropdownEntry
                                    text="Level Editor Plugin"
                                    page="LevelEditorPlugin"
                                />
                                <ProjectDropdownEntry
                                    text="Twin-Stick Duckbox"
                                    page="TwinStickDuckBox"
                                />
                                <ProjectDropdownEntry text="Folivora's Fall" page="FolivorasFall" />
                            </ul>
                        </div>
                    </button>

                    <a
                        href="public/Resume_JesseStolk_SoftwareEngineer.pdf"
                        target="_blank"
                        class="relative p-2 rounded-lg hover:bg-neutral-800"
                    >
                        <span>"Resume"</span>
                    </a>
                </div>

                <div class="flex gap-3 items-center items-center py-1 pr-3 justify-right">
                    <a
                        class="p-2 hover:rounded-full hover:bg-neutral-800"
                        href="https://www.linkedin.com/in/jesse-stolk-b2542a1b7/"
                        target="_blank"
                    >
                        <TextSizedImage image="public/images/linkedin.svg" alt="Linked In" />
                    </a>
                </div>
            </div>

            <div class="flex-col mb-auto min-h-screen bg-gray-900">{children}</div>

            <div class="py-3 px-4 bg-black text-neutral-500">
                <p class="text-center">"Website made using Leptos & Tailwind"</p>
            </div>
        </div>
    }
}
