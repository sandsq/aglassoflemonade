use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Thought {
    pub id: usize,
    pub content: String,
    pub date: NaiveDate,
    pub author: String,
}
