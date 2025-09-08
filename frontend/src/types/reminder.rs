use chrono::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub date_end: DateTime<chrono::Utc>,
}