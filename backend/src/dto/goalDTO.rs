use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoalDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub r#type: String,
    pub date_start: String,
    pub date_end: String,
}