use crate::models::project::article::Article;

pub struct Chapter {
    pub heading: &'static str,
    pub articles: &'static [Article],
}
