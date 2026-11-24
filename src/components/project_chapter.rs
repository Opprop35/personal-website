use leptos::prelude::*;

use crate::components::{Paragraphs, ProjectArticle, Section};
use crate::project::Chapter;

#[component]
pub fn ProjectChapter(chapter: &'static Chapter) -> impl IntoView {
    let mut articles: Vec<AnyView> = Vec::new();
    for article in chapter.articles {
        if !articles.is_empty() {
            articles.push(view! { <div class="divider" /> }.into_any());
        }
        articles.push(
            view! {
                <ProjectArticle title=article.title showcase=article.showcase>
                    <Paragraphs
                        paragraphs=article.body
                        class="space-y-4 leading-relaxed text-ink"
                    />
                </ProjectArticle>
            }
            .into_any(),
        );
    }

    if articles.is_empty() {
        return view! { <Section title=chapter.heading /> }.into_any();
    }

    view! {
        <Section title=chapter.heading collapsible=true open=true>
            <div class="flex flex-col gap-10">{articles}</div>
        </Section>
    }
    .into_any()
}
