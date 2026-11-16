use leptos::prelude::*;

use crate::components::TextSizedImage;
use crate::project::{Fragment, ProjectPageData, Section, Showcase};

pub mod attic_panic;
pub mod duck_side_of_the_moon;
pub mod duckside_of_the_moon_student;
pub mod folivoras_fall;
pub mod lego_framework;
pub mod level_editor_plugin;
pub mod radorian;
pub mod twin_stick_duckbox;

const IMG: &str = "public/images";

fn heading_view(heading: &'static str) -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center">
            <span class="flex-1 min-w-0 h-px bg-line" />
            <h3 class="text-base font-bold text-center uppercase whitespace-nowrap md:text-xl tracking-[0.2em] text-ink-muted">
                {heading}
            </h3>
            <span class="flex-1 min-w-0 h-px bg-line" />
        </div>
    }
}

fn fragments_view(fragments: &'static [Fragment]) -> impl IntoView {
    fragments
        .iter()
        .map(|fragment| match fragment {
            Fragment::Text(text) => {
                let text = *text;
                view! { <span>{text}</span> }.into_any()
            }
            Fragment::Emph(text) => {
                let text = *text;
                view! { <strong class="font-semibold text-background-hover-ink">{text}</strong> }
                .into_any()
            }
            Fragment::Link { url, label } => {
                let url = *url;
                let label = *label;
                view! {
                    <a
                        href=url
                        target="_blank"
                        rel="noopener noreferrer"
                        class="underline text-background-hover-ink decoration-background-hover-ink/50"
                    >
                        {label}
                    </a>
                }
                .into_any()
            }
            Fragment::List(items) => {
                let items = *items;
                view! {
                    <ul class="pl-5 my-3 space-y-1 list-disc marker:text-ink-muted">
                        {items.iter().copied().map(|item| view! { <li>{item}</li> }).collect_view()}
                    </ul>
                }
                .into_any()
            }
        })
        .collect_view()
}

fn showcase_view(showcase: &'static Showcase) -> impl IntoView {
    match showcase {
        Showcase::Video(code) => {
            let code = *code;
            view! {
                <div class="relative aspect-video">
                    <iframe
                        class="absolute inset-0 w-full h-full border border-line rounded-nav"
                        src=format!("https://www.youtube.com/embed/{code}")
                        allow="autoplay"
                        title="Project video"
                    ></iframe>
                </div>
            }
            .into_any()
        }
        Showcase::Image { src, alt } => {
            let src = *src;
            let alt = *alt;
            view! {
                <img
                    src=format!("{IMG}/{src}")
                    alt=alt
                    class="object-cover w-full border border-line rounded-nav"
                />
            }
            .into_any()
        }
    }
}

fn section_view(section: &'static Section) -> impl IntoView {
    let heading = section.heading.map(|heading| heading_view(heading));

    let details = if section.title.is_empty() && section.body.is_empty() {
        None
    } else {
        let body = fragments_view(section.body);
        let showcase = section
            .showcase
            .as_ref()
            .map(|showcase| showcase_view(showcase));
        Some(view! {
            <div class="flex flex-col gap-6 min-w-0 md:flex-row md:gap-8 md:items-start">
                <div class="w-full md:w-5/12">
                    <h4 class="pb-1 font-semibold text-ink">{section.title}</h4>
                    <div class="leading-relaxed text-ink">{body}</div>
                </div>
                {showcase}
            </div>
        })
    };

    view! { <section class="flex flex-col gap-6">{heading} {details}</section> }
}

#[component]
fn InfoRow(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0.5 sm:flex-row sm:gap-x-3 sm:items-baseline">
            <span class="shrink-0 text-ink-muted">{label}</span>
            <span class="min-w-0">{children()}</span>
        </div>
    }
}

#[component]
pub fn ProjectPageView(data: &'static ProjectPageData) -> impl IntoView {
    let title = data.title;

    let background_class = if data.background.is_some() {
        "bg-fixed bg-cover bg-center bg-no-repeat"
    } else {
        ""
    };
    let background_style = data
        .background
        .map(|src| format!("background-image:url({IMG}/{src})"))
        .unwrap_or_default();

    let hero_video = data.hero_video.map(|code| {
        view! {
            <div class="relative aspect-video">
                <iframe
                    class="absolute inset-0 w-full h-full border border-line rounded-nav"
                    src=format!("https://www.youtube.com/embed/{code}")
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
                <a
                    href=badge.url
                    target="_blank"
                    rel="noopener noreferrer"
                    class="transition-transform duration-150 hover:scale-105 ease-nav"
                >
                    <img
                        src=format!("{IMG}/{}", badge.image)
                        alt=badge.alt
                        class="w-auto h-12 md:h-16"
                    />
                </a>
            }
        })
        .collect_view();

    let info = &data.info;
    let skills = info
        .skills
        .iter()
        .map(|skill| {
            view! {
                <a
                    href=skill.url()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="transition-transform duration-150 hover:scale-110 ease-nav"
                >
                    <TextSizedImage image=format!("{IMG}/{}", skill.image()) alt=skill.alt() />
                </a>
            }
        })
        .collect_view();
    let socials = info
        .socials
        .iter()
        .map(|social| {
            view! {
                <a
                    href=social.url
                    target="_blank"
                    rel="noopener noreferrer"
                    class="transition-transform duration-150 hover:scale-110 ease-nav"
                >
                    <TextSizedImage image=format!("{IMG}/{}", social.image) alt=social.alt />
                </a>
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

    let info_rows = view! {
        <InfoRow label="Role">{info.role}</InfoRow>
        <InfoRow label="Genres">{info.genres}</InfoRow>
        <InfoRow label="Type">{info.project_type}</InfoRow>
        <InfoRow label="Skills Applied">
            <span class="inline-flex gap-2 items-center">{skills}</span>
        </InfoRow>
        <InfoRow label="Platforms">{info.platforms}</InfoRow>
        <InfoRow label="Team Size">{info.team_size}</InfoRow>
        <InfoRow label="Duration">{info.duration}</InfoRow>
        <InfoRow label="Release Date">{info.release_date_label()}</InfoRow>
        {socials_row}
    };

    let description = fragments_view(data.description);

    let sections = data
        .sections
        .iter()
        .map(|section| section_view(section))
        .collect_view();

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
                <span class="self-center py-2.5 px-4 max-w-full font-semibold tracking-wide text-center border-t border-x border-line bg-background-hover text-ink rounded-t-nav">
                    {title}
                </span>
                <div class="flex flex-col gap-8 p-4 bg-gradient-to-b border-b md:p-8 border-x border-line from-background/95 to-background rounded-b-nav">
                    {hero_video}
                    <div class="flex flex-wrap gap-4 justify-center items-center">{badges}</div>
                    <div class="h-px bg-line" />
                    <div class="flex flex-col gap-8 lg:flex-row lg:items-start">
                        <div class="w-full min-w-0 lg:w-5/12">
                            <h3 class="mb-3 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                "Project Info"
                            </h3>
                            <div class="grid gap-y-2 text-ink">{info_rows}</div>
                        </div>
                        <div class="w-full min-w-0 lg:w-7/12">
                            <h3 class="mb-3 text-xs font-semibold uppercase tracking-[0.16em] text-ink-muted">
                                "Description"
                            </h3>
                            {description}
                        </div>
                    </div> {has_more_content.then(|| view! { <div class="h-px bg-line" /> })}
                    <div class="flex flex-col gap-10">{sections}{under_construction}</div>
                </div>
            </div>
        </div>
    }
}
