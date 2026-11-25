use crate::models::paragraph::{ListParagraph, TextParagraph};

pub mod article;
pub mod chapter;
pub mod end_date;
pub mod genre;
pub mod image_link;
pub mod info;
pub mod project_type;
pub mod showcase;

pub use article::Article;
pub use chapter::Chapter;
pub use end_date::ProjectEndDate;
pub use genre::Genre;
pub use image_link::ImageLink;
pub use info::Info;
pub use project_type::ProjectType;
pub use showcase::Showcase;

pub struct Project {
    pub title: &'static str,
    pub slug: &'static str,
    pub card_thumbnail: &'static str,

    pub description: TextParagraph,
    pub main_contributions: ListParagraph,
    pub info: Info,

    pub hero_video: Option<&'static str>,
    pub badges: &'static [ImageLink],
    pub background: Option<&'static str>,
    pub chapters: &'static [Chapter],
}

impl Project {
    pub fn path(&self) -> String {
        format!("/{slug}", slug = self.slug)
    }
}
