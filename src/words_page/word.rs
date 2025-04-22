use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Word {
    pub id: usize,
    pub word: String,
    pub sounds_good: bool,
    pub looks_good: bool,
    pub means_good: bool,
    pub overall_good: bool,
    pub comment: String,
    pub entry_date: NaiveDate,
    pub author: String,
}

impl Word {
    pub fn new() -> Self {
        Self {
            id: 9999,
            word: "".to_string(),
            sounds_good: false,
            looks_good: false,
            means_good: false,
            overall_good: false,
            comment: String::new(),
            entry_date: NaiveDate::parse_from_str("2011-11-11", "%Y-%m-%d").unwrap(),
            author: String::new(),
        }
    }
}
