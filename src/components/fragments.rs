use leptos::prelude::*;

use crate::components::text::Text;
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

/// A paragraph of rich text fragments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextParagraph {
    pub fragments: &'static [TextFragment],
}

/// A paragraph of a bulleted list of short items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListParagraph {
    pub items: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paragraph {
    Text(TextParagraph),
    List(ListParagraph),
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
                view! { <strong class="font-bold text-ink">{text}</strong> }
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
                    <Link href=url class="text-link" external=true>
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
            Paragraph::Text(text) => {
                let text = *text;
                view! { <Text text=text /> }.into_any()
            }
            Paragraph::List(list) => {
                let list = *list;
                view! { <List list=list /> }.into_any()
            }
        })
        .collect_view();
    view! { <div class=classes>{blocks}</div> }
}

/// A bulleted list of short items.
#[component]
pub fn List(list: ListParagraph) -> impl IntoView {
    let ListParagraph { items } = list;
    view! {
        <ul class="pl-5 list-disc marker:text-ink-muted">
            {items.iter().copied().map(|item| view! { <li>{item}</li> }).collect_view()}
        </ul>
    }
}
