use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct TaskUpdateDto {
    pub status: Option<String>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
}