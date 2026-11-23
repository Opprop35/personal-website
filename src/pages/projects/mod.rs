use leptos::prelude::*;

use crate::IMAGE_DIR;
use crate::components::{Link, Paragraphs, Section, TextSizedImage};
use crate::project::{Project, SectionBlock, Showcase};

pub mod attic_panic;
pub mod duck_side_of_the_moon;
pub mod duckside_of_the_moon_student;
pub mod folivoras_fall;
pub mod lego_framework;
pub mod level_editor_plugin;
pub mod radorian;
pub mod twin_stick_duckbox;

fn showcase_view(showcase: Showcase) -> impl IntoView {
    match showcase {
        Showcase::Video(code) => view! {
            <div class="relative aspect-video">
                <iframe
                    class="absolute inset-0 w-full h-full border border-line rounded-default"
                    src=format!("https://www.youtube-nocookie.com/embed/{code}")
                    allow="autoplay"
                    title="Project video"
                ></iframe>
            </div>
        }
        .into_any(),
        Showcase::Image { src, alt } => view! {
            <img
                src=format!("{IMAGE_DIR}/{src}")
                alt=alt
                class="object-cover w-full border border-line rounded-default"
            />
        }
        .into_any(),
    }
}

#[component]
fn SectionBody(
    title: &'static str,
    showcase: Option<Showcase>,
    children: Children,
) -> impl IntoView {
    let showcase = showcase.map(showcase_view);
    view! {
        <div class="flex flex-col gap-6 md:flex-row md:gap-8 md:items-start">
            <article class="w-full min-w-0 md:w-5/12">
                <h4 class="pb-1 font-semibold text-ink">{title}</h4>
                <div class="leading-relaxed text-ink">{children()}</div>
            </article>
            <div class="w-full min-w-0 md:w-7/12">{showcase}</div>
        </div>
    }
}

fn section_view(block: &'static SectionBlock) -> impl IntoView {
    match block {
        SectionBlock::Heading(heading) => {
            let heading = *heading;
            view! { <Section title=heading /> }.into_any()
        }
        SectionBlock::Section {
            title,
            body,
            showcase,
        } => {
            let title = *title;
            let body = *body;
            let showcase = *showcase;
            view! {
                <SectionBody title=title showcase=showcase>
                    <Paragraphs paragraphs=body class="space-y-4" />
                </SectionBody>
            }
            .into_any()
        }
    }
}

#[component]
fn InfoRow(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <span class="text-ink-muted">{label}":"</span>
        <span class="min-w-0">{children()}</span>
    }
}

#[component]
pub fn ProjectPageView(data: &'static Project) -> impl IntoView {
    let title = data.title;

    let background_class = if data.background.is_some() {
        "bg-fixed bg-cover bg-center bg-no-repeat"
    } else {
        ""
    };
    let background_style = data
        .background
        .map(|src| format!("background-image:url({IMAGE_DIR}/{src})"))
        .unwrap_or_default();

    let hero_video = data.hero_video.map(|code| {
        view! {
            <div class="relative aspect-video">
                <iframe
                    class="absolute inset-0 w-full h-full border border-line rounded-default"
                    src=format!("https://www.youtube-nocookie.com/embed/{code}")
                    allow="autoplay"
                    title=format!("Trailer for {title}")
                ></iframe>
            </div>
        }
    });

    let badges = data
        .badges
        .iter()
        .map(|badge| {
            view! {
                <Link href=badge.url class="image-link" external=true>
                    <img
                        src=format!("{IMAGE_DIR}/{}", badge.image)
                        alt=badge.alt
                        class="w-auto h-12 md:h-16"
                    />
                </Link>
            }
        })
        .collect_view();

    let info = &data.info;
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
        view! {
            <InfoRow label="Socials">
                <span class="inline-flex gap-2 items-center">{socials}</span>
            </InfoRow>
        }
    });

    let genres_row = info.genres.map(|genres| {
        let genres = genres
            .iter()
            .map(|genre| genre.label())
            .collect::<Vec<_>>()
            .join(", ");
        view! { <InfoRow label="Genres">{genres}</InfoRow> }
    });

    let info_rows = view! {
        <InfoRow label="Role">{info.role}</InfoRow>
        {genres_row}
        <InfoRow label="Type">{info.project_type.label()}</InfoRow>
        <InfoRow label="Skills Applied">
            <span class="inline-flex gap-2 items-center">{skills}</span>
        </InfoRow>
        <InfoRow label="Platforms">{info.platforms}</InfoRow>
        <InfoRow label="Team Size">{info.team_size}</InfoRow>
        <InfoRow label="Duration">{info.duration}</InfoRow>
        <InfoRow label="End Date">{info.end_date.label()}</InfoRow>
        {socials_row}
    };

    let sections = data.sections.iter().map(section_view).collect_view();

    let under_construction = data.under_construction.then(|| {
        view! {
            <div class="flex gap-4 justify-center items-center py-4">
                <img src="public/images/wrench.svg" alt="Wrench" class="w-8 h-8 shrink-0" />
                <span class="font-bold uppercase tracking-[0.2em] text-ink-muted">
                    "Under Construction"
                </span>
                <img src="public/images/hammer.svg" alt="Hammer" class="w-8 h-8 shrink-0" />
            </div>
        }
    });

    let has_more_content = data.under_construction || !data.sections.is_empty();

    view! {
        <div class=background_class style=background_style>
            <div class="flex flex-col py-10 px-4 mx-auto w-full max-w-6xl md:py-14 md:px-8">
                <span class="self-center py-2.5 px-4 max-w-full font-semibold tracking-wide text-center border-t border-x border-line bg-surface-raised text-ink rounded-t-nav">
                    {title}
                </span>
                <div class="flex flex-col gap-8 p-4 bg-gradient-to-b border md:p-8 border-line from-surface-deep/95 to-surface-deep rounded-b-nav">
                    {hero_video}
                    <div class="flex flex-wrap gap-4 justify-center items-center">{badges}</div>
                    <div class="divider" />
                    <div class="flex flex-col gap-8 lg:flex-row lg:items-start">
                        <div class="w-full min-w-0 lg:w-5/12">
                            <h3 class="mb-3 label-uppercase">"Project Info"</h3>
                            <div class="grid grid-cols-1 gap-y-1 sm:gap-y-2 sm:gap-x-5 sm:items-center text-ink sm:grid-cols-[max-content_1fr]">
                                {info_rows}
                            </div>
                        </div>
                        <div class="w-full min-w-0 lg:w-7/12">
                            <h3 class="mb-3 label-uppercase">"Description"</h3>
                            <Paragraphs
                                paragraphs=data.description
                                class="space-y-4 leading-relaxed text-ink"
                            />
                        </div>
                    </div> {has_more_content.then(|| view! { <div class="divider" /> })}
                    <div class="flex flex-col gap-10">{sections}{under_construction}</div>
                </div>
            </div>
        </div>
    }
}
