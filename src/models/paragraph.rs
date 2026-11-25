use crate::models::text::TextFragment;

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

/// A paragraph of either rich text or a bulleted list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paragraph {
    Text(TextParagraph),
    List(ListParagraph),
}
