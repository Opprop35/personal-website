use leptos::prelude::*;

use crate::components::Link;
use crate::skill::Skill;

pub enum Fragment {
    Text(&'static str),
    StrongText(&'static str),
    MutedText(&'static str),
    Link {
        url: &'static str,
        label: &'static str,
    },
    List(&'static [&'static str]),
}

impl Fragment {
    pub const fn skill(skill: Skill) -> Self {
        Fragment::Link {
            url: skill.url(),
            label: skill.title(),
        }
    }
}

#[component]
pub fn Fragments(fragments: &'static [Fragment]) -> impl IntoView {
    fragments
        .iter()
        .map(|fragment| match fragment {
            Fragment::Text(text) => {
                let text = *text;
                view! { <span>{text}</span> }.into_any()
            }
            Fragment::StrongText(text) => {
                let text = *text;
                view! { <strong class="font-semibold text-background-hover-ink">{text}</strong> }
                    .into_any()
            }
            Fragment::MutedText(text) => {
                let text = *text;
                view! { <span class="text-ink-muted">{text}</span> }.into_any()
            }
            Fragment::Link { url, label } => {
                let url = *url;
                let label = *label;
                view! {
                    <Link href=url class="link-muted" external=true>
                        {label}
                    </Link>
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
