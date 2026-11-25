use leptos::prelude::*;

use crate::components::Paragraphs;
use crate::components::resume::ExperienceHeader;
use crate::models;

#[component]
pub fn Experience(experience: models::ExperienceData) -> impl IntoView {
    view! {
        <article class="last:mb-0 mb-[7mm] break-inside-avoid">
            <div class="flex flex-wrap gap-x-3 justify-between items-baseline">
                <h3 class="font-semibold tracking-wide uppercase text-[15px]">
                    <ExperienceHeader source=experience.source />
                </h3>
                <span class="text-[12px] text-ink-muted">{experience.period}</span>
            </div>
            <p class="mt-[1mm] text-[13px] text-ink-muted">{experience.subtitle}</p>
            {experience
                .details
                .map(|details| {
                    view! {
                        <Paragraphs paragraphs=details class="space-y-3 leading-snug text-[12px]" />
                    }
                })}
        </article>
    }
}
