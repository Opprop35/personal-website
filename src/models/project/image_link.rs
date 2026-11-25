#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageLink {
    pub image: &'static str,
    pub alt: &'static str,
    pub url: &'static str,
}
