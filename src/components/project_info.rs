use leptos::prelude::*;

use crate::components::{Labeled, Link, TextSizedImage};
use crate::project::Info;
use crate::IMAGE_DIR;

fn info_row(label: &'static str, value: impl IntoView) -> impl IntoView {
    view! {
        <span class="text-ink-muted">{label}":"</span>
        <span class="min-w-0">{value}</span>
    }
}

#[component]
pub fn ProjectInfo(info: &'static Info) -> impl IntoView {
    let skills = info
        .skills
        .iter()
        .map(|skill| {
            view! {
                <Link href=skill.url() class="image-link" external=true>
                    <TextSizedImage
                        image=format!("{IMAGE_DIR}/{}", skill.image())
                        alt=skill.alt()
                    />
                </Link>
            }
        })
        .collect_view();
    let socials = info
        .socials
        .iter()
        .map(|social| {
            view! {
                <Link href=social.url class="image-link" external=true>
                    <TextSizedImage image=format!("{IMAGE_DIR}/{}", social.image) alt=social.alt />
                </Link>
            }
        })
        .collect_view();
    let socials_row = (!info.socials.is_empty()).then(|| {
        info_row(
            "Socials",
            view! { <span class="inline-flex gap-2 items-center">{socials}</span> },
        )
        .into_any()
    });

    let genres_row = info.genres.map(|genres| {
        let genres = genres
            .iter()
            .map(|genre| genre.label())
            .collect::<Vec<_>>()
            .join(", ");
        info_row("Genres", genres).into_any()
    });

    let info_rows = view! {
        {info_row("Role", info.role)}
        {genres_row}
        {info_row("Type", info.project_type.label())}
        {info_row("Skills Applied", view! { <span class="inline-flex gap-2 items-center">{skills}</span> })}
        {info_row("Platforms", info.platforms)}
        {info_row("Team Size", info.team_size)}
        {info_row("Duration", info.duration)}
        {info_row("End Date", info.end_date.label())}
        {socials_row}
    };

    view! {
        <Labeled title="Project Info" class="w-full min-w-0 lg:w-5/12">
            <div class="grid gap-y-1 gap-x-5 items-center sm:gap-y-2 grid-cols-[max-content_1fr] text-ink">
                {info_rows}
            </div>
        </Labeled>
    }
}
