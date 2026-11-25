use crate::models::project::showcase::Showcase;
use crate::models::paragraph::Paragraph;

pub struct Article {
    pub title: &'static str,
    pub body: &'static [Paragraph],
    pub showcase: Showcase,
}
