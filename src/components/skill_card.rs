use leptos::prelude::*;

use crate::assets::image;
use crate::components::Link;
use crate::models;

#[component]
pub fn SkillCard(skill: models::Skill) -> impl IntoView {
    let year_text = if skill.years() <= 1.0 {
        "year"
    } else {
        "years"
    };
    let experience_text = format!("{} {} of experience", skill.years(), year_text);

    view! {
        <Link
            href=skill.url()
            class="flex gap-3 items-center p-4 w-full border transition-colors duration-150 focus-ring border-line bg-surface-deep hover:border-ink-muted/50"
            external=true
        >
            <img
                src=image(skill.image())
                alt=skill.alt()
                class="object-contain w-10 h-10 shrink-0"
            />
            <div class="min-w-0">
                <h3 class="font-semibold text-ink">{skill.title()}</h3>
                <p class="text-ink-muted">{experience_text}</p>
            </div>
        </Link>
    }
}
