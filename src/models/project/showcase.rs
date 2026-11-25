#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Showcase {
    Video(&'static str),
    Image {
        file_name: &'static str,
        alt: &'static str,
    },
}
