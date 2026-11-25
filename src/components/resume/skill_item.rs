use leptos::prelude::*;

use crate::assets::image;
use crate::components::{Link, TextSizedImage};
use crate::models;

#[component]
pub fn SkillItem(skill: models::Skill) -> impl IntoView {
    view! {
        <li>
            <Link
                href=skill.url()
                class="inline-flex items-center hover:underline focus-ring gap-[1.5mm]"
                external=true
            >
                <TextSizedImage image=image(skill.image()) alt=skill.alt() />
                {skill.title()}
            </Link>
        </li>
    }
}
