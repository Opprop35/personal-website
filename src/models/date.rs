use crate::models::month::Month;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

impl Date {
    pub fn format(self) -> String {
        format!(
            "{day:02}-{month}-{year}",
            day = self.day,
            month = self.month.code(),
            year = self.year
        )
    }
}
