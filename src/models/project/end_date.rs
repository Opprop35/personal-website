use crate::models::date::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectEndDate {
    #[allow(dead_code)]
    Ongoing,
    Ended(Date),
}

impl ProjectEndDate {
    pub fn label(self) -> String {
        match self {
            Self::Ongoing => "Ongoing".to_owned(),
            Self::Ended(date) => date.format(),
        }
    }
}
