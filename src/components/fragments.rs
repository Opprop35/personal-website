use leptos::prelude::*;

use crate::components::Link;
use crate::organization::Organization;
use crate::skill::Skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFragment {
    Text(&'static str),
    StrongText(&'static str),
    MutedText(&'static str),
    Link {
        url: &'static str,
        label: &'static str,
    },
}

impl TextFragment {
    pub const fn skill(skill: Skill) -> Self {
        TextFragment::Link {
            url: skill.url(),
            label: skill.title(),
        }
    }

    pub const fn organization(organization: Organization) -> Self {
        TextFragment::Link {
            url: organization.url(),
            label: organization.title(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paragraph {
    Text(&'static [TextFragment]),
    List(&'static [&'static str]),
}

#[component]
pub fn TextFragments(fragments: &'static [TextFragment]) -> impl IntoView {
    fragments
        .iter()
        .map(|fragment| match fragment {
            TextFragment::Text(text) => {
                let text = *text;
                view! { <span>{text}</span> }.into_any()
            }
            TextFragment::StrongText(text) => {
                let text = *text;
                view! { <strong class="font-semibold text-background-hover-ink">{text}</strong> }
                    .into_any()
            }
            TextFragment::MutedText(text) => {
                let text = *text;
                view! { <span class="text-ink-muted">{text}</span> }.into_any()
            }
            TextFragment::Link { url, label } => {
                let url = *url;
                let label = *label;
                view! {
                    <Link href=url class="link-muted" external=true>
                        {label}
                    </Link>
                }
                .into_any()
            }
        })
        .collect_view()
}

#[component]
pub fn Paragraphs(
    paragraphs: &'static [Paragraph],
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let classes = class.unwrap_or("").to_string();
    let blocks = paragraphs
        .iter()
        .map(|paragraph| match paragraph {
            Paragraph::Text(fragments) => {
                let fragments = *fragments;
                view! {
                    <p>
                        <TextFragments fragments=fragments />
                    </p>
                }
                .into_any()
            }
            Paragraph::List(items) => {
                let items = *items;
                view! {
                    <ul class="pl-5 list-disc marker:text-ink-muted">
                        {items.iter().copied().map(|item| view! { <li>{item}</li> }).collect_view()}
                    </ul>
                }
                .into_any()
            }
        })
        .collect_view();
    view! { <div class=classes>{blocks}</div> }
}
